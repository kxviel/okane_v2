use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tauri::State;

use crate::database;
use crate::Params;
use crate::ResponseStruct;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Transaction {
    id: i64,
    transaction_category_id: Option<i64>,
    account_id: Option<i64>,
    amount: Option<f64>,
    transaction_name: Option<String>,
    transaction_desc: Option<String>,
    transaction_date: Option<String>,
    transaction_type: Option<String>,
    merchant_name: Option<String>,
    transaction_location: Option<String>,
    currency: Option<String>,
    payment_method: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[tauri::command]
pub async fn get_transaction(
    params: Params,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Transaction>>, String> {
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
        SELECT id, transaction_category_id, account_id, amount, transaction_name, 
               transaction_desc, transaction_date, transaction_type, merchant_name,
               transaction_location, currency, payment_method, created_at, updated_at
        FROM transactions 
        WHERE transaction_name LIKE ? OR merchant_name LIKE ? OR transaction_desc LIKE ?
        ORDER BY transaction_date DESC, created_at DESC
        LIMIT ? 
        OFFSET ?";

    match sqlx::query_as(&query)
        .bind(format!("%{}%", &search))
        .bind(format!("%{}%", &search))
        .bind(format!("%{}%", &search))
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
    {
        Ok(transactions) => {
            let message = if transactions.is_empty() {
                "No transactions found"
            } else {
                "Transactions fetched successfully"
            };

            Ok(ResponseStruct::success(message, transactions))
        }

        Err(e) => Ok(ResponseStruct::error(&format!("Database error: {}", e))),
    }
}

#[tauri::command]
pub async fn add_transaction(
    transaction: Transaction,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Transaction>>, String> {
    // Validate required fields
    if transaction.transaction_category_id.is_none() {
        return Ok(ResponseStruct::error("Category is required"));
    }

    if transaction.account_id.is_none() {
        return Ok(ResponseStruct::error("Account is required"));
    }

    if transaction.amount.is_none() {
        return Ok(ResponseStruct::error("Amount is required"));
    }

    if transaction.transaction_name.is_none()
        || transaction
            .transaction_name
            .as_ref()
            .unwrap()
            .trim()
            .is_empty()
    {
        return Ok(ResponseStruct::error("Transaction name is required"));
    }

    if transaction.transaction_date.is_none() {
        return Ok(ResponseStruct::error("Transaction date is required"));
    }

    let query = "
        INSERT INTO transactions (transaction_category_id, account_id, amount, transaction_name, 
                                transaction_desc, transaction_date, transaction_type, merchant_name,
                                transaction_location, currency, payment_method, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)";

    match sqlx::query(&query)
        .bind(&transaction.transaction_category_id)
        .bind(&transaction.account_id)
        .bind(&transaction.amount)
        .bind(&transaction.transaction_name)
        .bind(&transaction.transaction_desc)
        .bind(&transaction.transaction_date)
        .bind(&transaction.transaction_type)
        .bind(&transaction.merchant_name)
        .bind(&transaction.transaction_location)
        .bind(&transaction.currency)
        .bind(&transaction.payment_method)
        .execute(&state.db)
        .await
    {
        Ok(_) => Ok(ResponseStruct::success(
            "Transaction Added Successfully",
            vec![transaction],
        )),
        Err(e) => Ok(ResponseStruct::error(&format!("Database error: {}", e))),
    }
}

#[tauri::command]
pub async fn update_transaction(
    request: Transaction,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Transaction>>, String> {
    let transaction_exists_query = "SELECT id FROM transactions WHERE id = ?";
    match sqlx::query_scalar::<_, i64>(&transaction_exists_query)
        .bind(request.id)
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => {}
        Err(sqlx::Error::RowNotFound) => {
            return Ok(ResponseStruct::error("Transaction not found"));
        }
        Err(e) => {
            return Ok(ResponseStruct::error(&format!("Database error: {}", e)));
        }
    };

    let mut update_query = sqlx::QueryBuilder::new("UPDATE transactions SET ");
    let mut fields = update_query.separated(", ");
    let mut is_dirty = false;

    if let Some(category_id) = &request.transaction_category_id {
        fields
            .push("transaction_category_id = ")
            .push_bind(category_id);
        is_dirty = true;
    }

    if let Some(account_id) = &request.account_id {
        fields.push("account_id = ").push_bind(account_id);
        is_dirty = true;
    }

    if let Some(amount) = &request.amount {
        fields.push("amount = ").push_bind(amount);
        is_dirty = true;
    }

    if let Some(transaction_name) = &request.transaction_name {
        if transaction_name.trim().is_empty() {
            return Ok(ResponseStruct::error("Transaction name cannot be empty"));
        }
        fields
            .push("transaction_name = ")
            .push_bind(transaction_name);
        is_dirty = true;
    }

    if let Some(transaction_desc) = &request.transaction_desc {
        fields
            .push("transaction_desc = ")
            .push_bind(transaction_desc);
        is_dirty = true;
    }

    if let Some(transaction_date) = &request.transaction_date {
        fields
            .push("transaction_date = ")
            .push_bind(transaction_date);
        is_dirty = true;
    }

    if let Some(transaction_type) = &request.transaction_type {
        fields
            .push("transaction_type = ")
            .push_bind(transaction_type);
        is_dirty = true;
    }

    if let Some(merchant_name) = &request.merchant_name {
        fields.push("merchant_name = ").push_bind(merchant_name);
        is_dirty = true;
    }

    if let Some(transaction_location) = &request.transaction_location {
        fields
            .push("transaction_location = ")
            .push_bind(transaction_location);
        is_dirty = true;
    }

    if let Some(currency) = &request.currency {
        if currency != "EUR" {
            return Ok(ResponseStruct::error("Only EUR currency is supported"));
        }
        fields.push("currency = ").push_bind(currency);
        is_dirty = true;
    }

    if let Some(payment_method) = &request.payment_method {
        fields.push("payment_method = ").push_bind(payment_method);
        is_dirty = true;
    }

    if !is_dirty {
        return Ok(ResponseStruct::error(
            "At least one field must be provided for update",
        ));
    }

    fields.push("updated_at = CURRENT_TIMESTAMP");

    update_query.push(" WHERE id = ").push_bind(request.id);
    match update_query.build().execute(&state.db).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                Ok(ResponseStruct::error("No changes were made"))
            } else {
                Ok(ResponseStruct::success(
                    "Transaction updated successfully",
                    vec![],
                ))
            }
        }
        Err(e) => Ok(ResponseStruct::error(&format!(
            "Failed to update transaction: {}",
            e
        ))),
    }
}

#[tauri::command]
pub async fn delete_transaction(
    id: i64,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Transaction>>, String> {
    let transaction_exists_query = "SELECT id FROM transactions WHERE id = ?";
    match sqlx::query_scalar::<_, i64>(&transaction_exists_query)
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => {}
        Err(sqlx::Error::RowNotFound) => {
            return Ok(ResponseStruct::error("Transaction not found"));
        }
        Err(e) => {
            return Ok(ResponseStruct::error(&format!("Database error: {}", e)));
        }
    };

    let delete_query = "DELETE FROM transactions WHERE id = ?";
    match sqlx::query(&delete_query).bind(id).execute(&state.db).await {
        Ok(_) => Ok(ResponseStruct::success(
            "Transaction deleted successfully",
            vec![],
        )),
        Err(e) => Ok(ResponseStruct::error(&format!(
            "Failed to delete transaction: {}",
            e
        ))),
    }
}

#[tauri::command]
pub async fn get_transactions_by_category(
    category_id: i64,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Transaction>>, String> {
    let query = "
        SELECT id, transaction_category_id, account_id, amount, transaction_name, 
               transaction_desc, transaction_date, transaction_type, merchant_name,
               transaction_location, currency, payment_method, created_at, updated_at
        FROM transactions 
        WHERE transaction_category_id = ?
        ORDER BY transaction_date DESC";

    match sqlx::query_as(&query)
        .bind(category_id)
        .fetch_all(&state.db)
        .await
    {
        Ok(transactions) => Ok(ResponseStruct::success(
            "Transactions fetched successfully",
            transactions,
        )),
        Err(e) => Ok(ResponseStruct::error(&format!("Database error: {}", e))),
    }
}

#[tauri::command]
pub async fn get_transactions_by_account(
    account_id: i64,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Transaction>>, String> {
    let query = "
        SELECT id, transaction_category_id, account_id, amount, transaction_name, 
               transaction_desc, transaction_date, transaction_type, merchant_name,
               transaction_location, currency, payment_method, created_at, updated_at
        FROM transactions 
        WHERE account_id = ?
        ORDER BY transaction_date DESC";

    match sqlx::query_as(&query)
        .bind(account_id)
        .fetch_all(&state.db)
        .await
    {
        Ok(transactions) => Ok(ResponseStruct::success(
            "Transactions fetched successfully",
            transactions,
        )),
        Err(e) => Ok(ResponseStruct::error(&format!("Database error: {}", e))),
    }
}

#[tauri::command]
pub async fn get_transactions_by_date_range(
    start_date: String,
    end_date: String,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Transaction>>, String> {
    let query = "
        SELECT id, transaction_category_id, account_id, amount, transaction_name, 
               transaction_desc, transaction_date, transaction_type, merchant_name,
               transaction_location, currency, payment_method, created_at, updated_at
        FROM transactions 
        WHERE transaction_date BETWEEN ? AND ?
        ORDER BY transaction_date DESC";

    match sqlx::query_as(&query)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&state.db)
        .await
    {
        Ok(transactions) => Ok(ResponseStruct::success(
            "Transactions fetched successfully",
            transactions,
        )),
        Err(e) => Ok(ResponseStruct::error(&format!("Database error: {}", e))),
    }
}
