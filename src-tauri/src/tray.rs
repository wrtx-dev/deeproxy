use tauri::Manager;

use crate::proxy;
#[tauri::command]
pub async fn tray_quit(handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let app = handle.clone();
        proxy::stop_server(handle).await.unwrap();
        app.exit(0);
    });
}

#[tauri::command]
pub async fn tray_start_server(handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        if let Err(e) = proxy::start_api_server(handle).await {
            eprintln!("Failed to start server: {}", e);
        }
    });
}

#[tauri::command]
pub async fn tray_stop_server(handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        proxy::stop_server(handle).await.unwrap();
    });
}

#[tauri::command]
pub async fn tray_restart_server(handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        proxy::stop_server(handle.clone()).await.unwrap();
        proxy::start_api_server(handle).await.unwrap();
    });
}

#[tauri::command]
pub async fn tray_show_setup(handle: tauri::AppHandle) {
    let win = handle.get_webview_window("main").unwrap();
    if win.is_visible().unwrap() {
        #[cfg(target_os = "macos")]
        {
            if win.is_focused().unwrap() {
                win.hide().unwrap();
            } else {
                win.set_focus().unwrap();
            }
        }
        #[cfg(not(target_os = "macos"))]
        win.hide().unwrap();
    } else {
        win.show().expect("failed to show window");
        let _ = win.set_focus().expect("failed to set focus");
    }
}
