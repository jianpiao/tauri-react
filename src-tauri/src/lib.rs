// 关于 Tauri 命令的更多信息，请访问 https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod system_tray;
use system_tray::create_tray;
mod system_menu;
use system_menu::create_menu;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // 创建托盘图标
            let _tray = create_tray(app)?;
            // 创建系统菜单
            create_menu(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
