mod watcher_state;
use tauri::async_runtime::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_blec::init())
        .invoke_handler(tauri::generate_handler![watcher_state::update_blocklist])
        .setup(|app| {
            app.manage(Mutex::new(Vec::<shock_clock_utils::Block>::new()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
