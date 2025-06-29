mod accounts;
mod categories;
mod database;
mod transaction;

use accounts::{
    add_account, delete_account, get_account, get_active_accounts, pre_delete_account,
    update_account, update_account_balance,
};
use categories::{
    add_category, delete_category, get_category, pre_delete_category, update_category,
};
use transaction::{
    add_transaction, delete_transaction, get_transaction, get_transactions_by_account,
    get_transactions_by_category, get_transactions_by_date_range, update_transaction,
};

use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    page: Option<i64>,
    search: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseStruct<T> {
    message: String,
    success: bool,
    data: Option<T>,
}

impl<T> ResponseStruct<T> {
    pub fn success(message: &str, data: T) -> Self {
        Self {
            message: message.to_string(),
            success: true,
            data: Some(data),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            message: message.to_string(),
            success: false,
            data: None,
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            add_category,
            delete_category,
            get_category,
            pre_delete_category,
            update_category,
            add_account,
            delete_account,
            get_account,
            get_active_accounts,
            pre_delete_account,
            update_account,
            update_account_balance,
            get_transaction,
            add_transaction,
            update_transaction,
            delete_transaction,
            get_transactions_by_category,
            get_transactions_by_account,
            get_transactions_by_date_range
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
