#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::image::Image;
use tauri::menu::{CheckMenuItemBuilder, IconMenuItemBuilder, MenuBuilder, SubmenuBuilder};
use tauri::{App, Manager};

/**
 * 创建系统菜单
 */
pub fn create_menu(app: &App) -> Result<(), tauri::Error> {
    let file_menu = SubmenuBuilder::new(app, "File")
        .text("open", "Open")
        .text("quit", "Quit")
        .build()?;

    let lang_str = "en";
    let check_sub_item_1 = CheckMenuItemBuilder::new("English")
        .id("en")
        .checked(lang_str == "en")
        .build(app)?;

    let check_sub_item_2 = CheckMenuItemBuilder::new("Chinese")
        .id("en")
        .checked(lang_str == "en")
        .enabled(false)
        .build(app)?;

    // 从路径加载图标
    let icon_image = Image::from_bytes(include_bytes!("../../icons/icon.png")).unwrap();

    let icon_item = IconMenuItemBuilder::new("icon")
        .icon(icon_image)
        .build(app)?;

    let other_item = SubmenuBuilder::new(app, "language")
        .item(&check_sub_item_1)
        .item(&check_sub_item_2)
        .build()?;

    let menu = MenuBuilder::new(app)
        .items(&[&file_menu, &other_item, &icon_item])
        .build()?;

    app.set_menu(menu)?;

    // 添加菜单事件监听器
    app.on_menu_event(move |app, event| {
        match event.id().as_ref() {
            "open" => {
                // 获取主窗口并显示
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {}
        }
    });

    Ok(())
}
