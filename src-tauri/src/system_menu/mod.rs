#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::image::Image;
use tauri::menu::{MenuBuilder, CheckMenuItemBuilder, SubmenuBuilder, IconMenuItemBuilder};
use tauri::App;

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

    Ok(())
}
