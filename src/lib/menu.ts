import { defaultWindowIcon } from "@tauri-apps/api/app";
import { invoke } from "@tauri-apps/api/core";
import { Menu } from "@tauri-apps/api/menu";
import { TrayIcon } from "@tauri-apps/api/tray";

export const createStartMenu = async () => {
    return await Menu.new(
        {
            items: [
                {
                    id: "setup",
                    text: "设置",
                    action: async () => {
                        invoke("tray_show_setup");
                    },
                },
                {
                    id: "startup",
                    text: "启动",
                    action: async () => {
                        invoke("tray_start_server");
                    },
                },
                {
                    id: "exit",
                    text: "退出",
                    action: async () => {
                        invoke("tray_quit");
                    },
                },
            ],
        },
    );
};

export const createStopMenu = async () => {
    return await Menu.new(
        {
            items: [
                {
                    id: "setup",
                    text: "设置",
                    action: async () => {
                        invoke("tray_show_setup");
                    },
                },
                {
                    id: "restart",
                    text: "重启",
                    action: async () => {
                        invoke("tray_restart_server");
                    },
                },
                {
                    id: "stop",
                    text: "停止",
                    action: async () => {
                        invoke("tray_stop_server");
                    },
                },
                {
                    id: "exit",
                    text: "退出",
                    action: async () => {
                        invoke("tray_quit");
                    },
                },
            ],
        },
    );
};

export const createTray = async (menu?: Menu) => {
    return await TrayIcon.new({
        icon: await defaultWindowIcon() ?? undefined,
        menu: menu,
        showMenuOnLeftClick: false,
        action: (event) => {
            switch (event.type) {
                case "Click":
                    if (event.buttonState === "Up" && event.button === "Left") {
                        invoke("tray_show_setup");
                    }
                    break;
                default:
                    break;
            }
        },
    });
};
