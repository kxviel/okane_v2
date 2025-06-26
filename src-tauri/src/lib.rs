mod category;
mod database;

use category::{add_category, delete_category, get_category};
use serde::{Deserialize, Serialize};

use tauri::Manager;

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    page: Option<i64>,
    search: Option<String>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_category,
            add_category,
            delete_category
        ])
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let db = database::db_connection(&app).await;
                app.manage(database::AppState { db });
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
