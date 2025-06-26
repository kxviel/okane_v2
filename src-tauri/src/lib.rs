mod database;

use serde::{Deserialize, Serialize};
use sqlx::Row;
use tauri::{Manager, State};

#[derive(Debug, Serialize, Deserialize)]
struct Params {
    page: Option<i64>,
    search: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    id: Option<i64>,
    category_name: String,
    desc: String,
    created_at: Option<String>,
}

#[tauri::command]
async fn get_category(
    params: Params,
    state: State<'_, database::AppState>,
) -> Result<Vec<Category>, String> {
    let page = params.page.unwrap_or(1);
    let search = params.search.unwrap_or_default();

    let limit = 10;
    let offset = (page - 1) * limit;

    let query = "
        SELECT * FROM category 
        WHERE category_name LIKE ?
        LIMIT ? 
        OFFSET ?";

    let rows = sqlx::query(&query)
        .bind(format!("%{}%", &search))
        .bind(&limit)
        .bind(&offset)
        .fetch_all(&state.db)
        .await
        .map_err(|e| format!("Database query error: {}", e))?;

    let mut categories = Vec::new();

    for row in rows {
        let category = Category {
            id: Some(row.get::<i64, _>("id")),
            category_name: row.get::<String, _>("category_name"),
            desc: row.get::<String, _>("desc"),
            created_at: Some(row.get::<String, _>("created_at")),
        };
        categories.push(category);
    }

    Ok(categories)
}

#[tauri::command]
async fn add_category(
    category: Category,
    state: State<'_, database::AppState>,
) -> Result<String, String> {
    let query = "
        INSERT INTO category (category_name, desc, created_at)
        VALUES (?, ?, DATETIME('now'))";

    sqlx::query(&query)
        .bind(&category.category_name)
        .bind(&category.desc)
        .execute(&state.db)
        .await
        .map_err(|e| format!("Database insert error: {}", e))?;

    Ok("Category added successfully".to_string())
}

#[tauri::command]
async fn delete_category(id: i64, state: State<'_, database::AppState>) -> Result<String, String> {
    let query = "DELETE FROM category WHERE id = ?";

    sqlx::query(&query)
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|e| format!("Database delete error: {}", e))?;

    Ok("Category deleted successfully".to_string())
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
