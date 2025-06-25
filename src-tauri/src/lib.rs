mod database;

use tauri::{Manager, State};

#[derive(serde::Deserialize, Debug)]
struct CategoryRequest {
    category_name: String,
    desc: String,
}

#[tauri::command]
async fn add_category(
    category: CategoryRequest,
    state: State<'_, database::AppState>,
) -> Result<String, String> {
    println!("Adding category: {:?}", category);

    let result = sqlx::query(
        "INSERT INTO category (category_name, desc, created_at) VALUES (?, ?, DATETIME('now'))",
    )
    .bind(&category.category_name)
    .bind(&category.desc)
    .execute(&state.db) // Use the database pool from app state
    .await
    .map_err(|e| format!("Database insert error: {}", e))?;

    println!(
        "Category added successfully, rows affected: {}",
        result.rows_affected()
    );
    Ok(format!(
        "Category '{}' added successfully",
        category.category_name
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![add_category])
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
