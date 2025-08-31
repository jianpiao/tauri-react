use image::ImageFormat;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    App, Error, Manager,
};

/**
 * 创建系统托盘
 */
pub fn create_tray(app: &App) -> Result<TrayIcon, Error> {
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let show = MenuItem::with_id(app, "show", "显示", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "隐藏", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &hide, &quit])?;

    let tray = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(
            |app: &tauri::AppHandle<_>, event| match event.id().as_ref() {
                "quit" => {
                    println!("退出应用程序");
                    app.exit(0);
                }
                "show" => {
                    println!("显示主窗口");
                    show_main_window(app)
                }
                "hide" => {
                    println!("隐藏主窗口");
                    hide_main_window(app)
                }
                _ => {
                    println!("未知菜单项: {}", event.id().as_ref());
                }
            },
        )
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                println!("left click pressed and released");
                // 在这个例子中，当点击托盘图标时，将展示并聚焦于主窗口
                let app1 = tray.app_handle();
                show_main_window(app1)
            }
            _ => {
                // 处理其他事件
            }
        })
        .build(app)?;

    tray.set_tooltip(Some("Tauri App"))?;
    // 使用自定义图标
    let icon_bytes = include_bytes!("../../icons/icon.png");
    let img = image::load_from_memory_with_format(icon_bytes, ImageFormat::Png)
        .map_err(|e| tauri::Error::Anyhow(anyhow::anyhow!("Failed to load icon: {}", e)))?
        .to_rgba8();
    let (width, height) = img.dimensions();
    let icon = tauri::image::Image::new_owned(img.into_raw(), width, height);
    tray.set_icon(Some(icon))?;
    Ok(tray)
}

/**
 * 显示主窗口
 */
fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

/**
 * 隐藏主窗口
 */
fn hide_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}
