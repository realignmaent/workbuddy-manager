use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};
use tauri_plugin_clipboard_manager::ClipboardExt;

#[derive(Default)]
struct AppState {
    backend_child: Arc<Mutex<Option<Child>>>,
}

pub fn run() {
    let state = AppState::default();

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(state)
        .setup(|app| {
            // 1. 创建托盘菜单项
            let show_i = MenuItem::with_id(app, "show", "打开控制台", true, None::<&str>)?;
            let copy_api_i = MenuItem::with_id(
                app,
                "copy_api",
                "复制 API 地址 (/v1)",
                true,
                None::<&str>,
            )?;
            let open_browser_i = MenuItem::with_id(
                app,
                "open_browser",
                "在默认浏览器中打开",
                true,
                None::<&str>,
            )?;
            let sep = MenuItem::separator(app)?;
            let quit_i = MenuItem::with_id(app, "quit", "完全退出", true, None::<&str>)?;

            let menu = Menu::with_items(
                app,
                &[
                    &show_i,
                    &copy_api_i,
                    &open_browser_i,
                    &sep,
                    &quit_i,
                ],
            )?;

            // 2. 构建系统托盘
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("WorkBuddy Manager")
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "copy_api" => {
                        let _ = app.clipboard().write_text("http://127.0.0.1:7864/v1".to_string());
                    }
                    "open_browser" => {
                        #[cfg(target_os = "windows")]
                        {
                            let _ = Command::new("cmd")
                                .args(["/c", "start", "http://127.0.0.1:7864"])
                                .spawn();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // 3. 启动后台网关服务
            start_backend(app);

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // 点击窗口右上角叉号时隐藏到系统托盘，防止误停服务
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::ExitRequested { .. } = event {
                stop_backend(app_handle);
            }
        });
}

fn start_backend(app: &tauri::App) {
    let app_handle = app.handle();
    let state = app_handle.state::<AppState>();

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        // 优先尝试寻找打包好的后端可执行文件，若无则使用系统 Python
        let mut cmd = Command::new("python");
        cmd.args(["-m", "uvicorn", "server.main:app", "--host", "127.0.0.1", "--port", "7864"])
            .creation_flags(CREATE_NO_WINDOW);

        if let Ok(child) = cmd.spawn() {
            if let Ok(mut lock) = state.backend_child.lock() {
                *lock = Some(child);
            }
        }
    }
}

fn stop_backend(app_handle: &tauri::AppHandle) {
    let state = app_handle.state::<AppState>();
    if let Ok(mut lock) = state.backend_child.lock() {
        if let Some(mut child) = lock.take() {
            let _ = child.kill();
        }
    }
}
