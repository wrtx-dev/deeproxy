use std::sync::Arc;
use std::time::Duration;
use tauri::menu::{Menu, MenuItem};
use tokio::sync::Mutex;

use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent};
mod proxy;

#[cfg(target_os = "macos")]
mod macos;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            proxy::start_server,
            is_running,
            proxy::stop,
            proxy::restart,
        ])
        .manage(Arc::new(Mutex::new(None)) as proxy::SharedServerState)
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .setup(|app| {
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let stop_item = MenuItem::with_id(app, "stop", "停止", true, None::<&str>)?;
            let start_item = MenuItem::with_id(app, "start", "启动", true, None::<&str>)?;
            let main_item = MenuItem::with_id(app, "main", "设置", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&main_item, &start_item, &stop_item, &quit_item])?;
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => {
                        println!("quit menu item was clicked");
                        let napp = app.app_handle().clone();
                        tauri::async_runtime::spawn(async move {
                            let app = napp.clone();
                            proxy::stop_server(napp).await.unwrap();
                            app.exit(0);
                        });
                    }
                    "stop" => {
                        let napp = app.app_handle().clone();
                        tauri::async_runtime::spawn(async move {
                            proxy::stop_server(napp.app_handle().clone()).await.unwrap();
                        });

                        println!("stop menu item was clicked")
                    }
                    "start" => {
                        let napp = app.app_handle().clone();
                        // 使用应用的async_runtime而不是创建新runtime
                        tauri::async_runtime::spawn(async move {
                            if let Err(e) = proxy::start_api_server(napp.app_handle().clone()).await
                            {
                                eprintln!("Failed to start server: {}", e);
                            }
                        });
                    }
                    "main" => {
                        let win = app.app_handle().get_webview_window("main").unwrap();
                        if let Ok(flag) = win.is_visible() {
                            if flag {
                                win.hide().unwrap();
                            } else {
                                println!("show main window");
                                win.show().expect("failed to show window");
                                let _ = win.set_focus().expect("failed to set focus");
                            }
                        }
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id());
                    }
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let handle = tray.app_handle();
                        let win = handle.get_webview_window("main").unwrap();
                        if win.is_visible().unwrap() {
                            if (win.is_focused().unwrap()) {
                                win.hide().unwrap();
                            } else {
                                win.set_focus().unwrap();
                            }
                        } else {
                            win.show().expect("failed to show window");
                            let _ = win.set_focus().expect("failed to set focus");
                        }
                    }
                    _ => {}
                })
                .build(app)?;
            #[cfg(target_os = "macos")]
            macos::set_activation_policy(macos::ActivationPolicy::Accessory);
            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("deeproxy")
                .hidden_title(true)
                .resizable(false)
                .visible(false)
                .inner_size(400.0, 375.0);
            // #[cfg(target_os = "macos")]
            // let win_builder = win_builder.title_bar_style(TitleBarStyle::Overlay);
            let _window = win_builder.build().unwrap();
            let napp = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(Duration::from_secs(2)).await;
                let status = proxy::start_api_server(napp).await;
                if let Err(_) = status {
                    _window.show().unwrap();
                    _window.set_focus().unwrap();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn is_running(handle: tauri::AppHandle) -> Result<bool, String> {
    let server_state_arc = handle.state::<proxy::SharedServerState>();
    let server_state = server_state_arc.lock().await;
    Ok(server_state.is_some())
}
