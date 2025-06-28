use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tauri::State;

use crate::database;
use crate::Params;
use crate::ResponseStruct;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Category {
    id: Option<i64>,
    category_name: String,
    category_desc: String,
    created_at: Option<String>,
}

const DEFAULT_CATEGORY_ID: i64 = 1;

#[tauri::command]
pub async fn get_category(
    params: Params,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Category>>, String> {
    let page = params.page.unwrap_or(1);
    let search = params.search.unwrap_or_default();

    if page < 1 {
        return Ok(ResponseStruct {
            message: "Page number must be greater than 0".to_string(),
            success: false,
            data: None,
        });
    }

    let limit = 10;
    let offset = (page - 1) * limit;

    let query = "
        SELECT id, category_name, description, created_at 
        FROM category 
        WHERE category_name LIKE ?
        ORDER BY category_name
        LIMIT ? 
        OFFSET ?";

    match sqlx::query_as(&query)
        .bind(format!("%{}%", &search))
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
    {
        Ok(categories) => {
            let message = if categories.is_empty() {
                "No categories found"
            } else {
                "Categories fetched successfully"
            };

            Ok(ResponseStruct::success(message, categories))
        }

        Err(e) => Ok(ResponseStruct::error(&format!("Database error: {}", e))),
    }
}

#[tauri::command]
pub async fn add_category(
    category: Category,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Category>>, String> {
    let query = "
        INSERT INTO category (category_name, category_desc, created_at)
        VALUES (?, ?, DATETIME('now'))";

    match sqlx::query(&query)
        .bind(&category.category_name)
        .bind(&category.category_desc)
        .execute(&state.db)
        .await
    {
        Ok(_) => Ok(ResponseStruct::success(
            "Category Added Successfully",
            vec![category],
        )),
        Err(e) => Ok(ResponseStruct::error(&format!("Database error: {}", e))),
    }
}

#[tauri::command]
pub async fn pre_delete_category(
    id: i64,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Category>>, String> {
    // Check if trying to delete default category
    if id == 1 {
        return Ok(ResponseStruct::error("Cannot delete default category"));
    }

    // Check if category exists
    let if_category_exists_query = "SELECT id FROM category WHERE id = ?";
    match sqlx::query_scalar::<_, i64>(&if_category_exists_query)
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => {}
        Err(sqlx::Error::RowNotFound) => {
            return Ok(ResponseStruct::error("Category not found"));
        }
        Err(e) => {
            return Ok(ResponseStruct::error(&format!("Database error: {}", e)));
        }
    };

    // Count how many transactions use this category
    let category_usage_count =
        "SELECT COUNT(*) as count FROM transactions WHERE transaction_category_id = ?";
    match sqlx::query_scalar::<_, i64>(&category_usage_count)
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(count) => {
            if count > 0 {
                Ok(ResponseStruct::success(
                    &format!(
                        "This category is being used in {} transactions. Do you want to proceed?",
                        count
                    ),
                    vec![],
                ))
            } else {
                Ok(ResponseStruct::success(
                    "Category can be safely deleted.",
                    vec![],
                ))
            }
        }
        Err(e) => Ok(ResponseStruct::error(&format!("Database error: {}", e))),
    }
}

#[tauri::command]
pub async fn delete_category(
    id: i64,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Category>>, String> {
    if id == 1 {
        return Ok(ResponseStruct::error("Cannot delete default category"));
    }

    let mut tx = match state.db.begin().await {
        Ok(t) => t,
        Err(e) => {
            return Ok(ResponseStruct::error(&format!(
                "Failed to start transaction: {}",
                e
            )))
        }
    };

    let if_category_exists_query = "SELECT id FROM category WHERE id = ?";
    match sqlx::query_scalar::<_, i64>(&if_category_exists_query)
        .bind(id)
        .fetch_one(&mut *tx)
        .await
    {
        Ok(_) => {}
        Err(sqlx::Error::RowNotFound) => {
            return Ok(ResponseStruct::error("Category not found"));
        }
        Err(e) => {
            return Ok(ResponseStruct::error(&format!("Database error: {}", e)));
        }
    };

    let update_transactions_query = "UPDATE transactions 
        SET transaction_category_id = ? 
        WHERE transaction_category_id = ?";
    match sqlx::query(&update_transactions_query)
        .bind(DEFAULT_CATEGORY_ID)
        .bind(id)
        .execute(&mut *tx)
        .await
    {
        Ok(_) => {}
        Err(e) => {
            return Ok(ResponseStruct::error(&format!(
                "Failed to update transactions: {}",
                e
            )))
        }
    };

    let delete_query = "DELETE FROM category WHERE id = ?";
    match sqlx::query(&delete_query).bind(id).execute(&mut *tx).await {
        Ok(_) => {}
        Err(e) => {
            return Ok(ResponseStruct::error(&format!(
                "Failed to delete category: {}",
                e
            )))
        }
    };

    match tx.commit().await {
        Ok(_) => {}
        Err(e) => {
            return Ok(ResponseStruct::error(&format!(
                "Failed to commit transaction: {}",
                e
            )));
        }
    }

    Ok(ResponseStruct::success(
        "Category deleted successfully",
        vec![],
    ))
}
