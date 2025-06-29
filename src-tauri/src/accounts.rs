use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tauri::State;

use crate::database;
use crate::Params;
use crate::ResponseStruct;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Account {
    id: i64,
    account_name: Option<String>,
    account_type: Option<String>,
    balance: Option<f64>,
    currency: Option<String>,
    is_active: Option<bool>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

const DEFAULT_ACCOUNT_ID: i64 = 1;

#[tauri::command]
pub async fn get_account(
    params: Params,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Account>>, String> {
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
        SELECT id, account_name, account_type, balance, currency, is_active, created_at, updated_at
        FROM accounts 
        WHERE account_name LIKE ? OR account_type LIKE ?
        ORDER BY account_name
        LIMIT ? 
        OFFSET ?";

    match sqlx::query_as(&query)
        .bind(format!("%{}%", &search))
        .bind(format!("%{}%", &search))
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
    {
        Ok(accounts) => {
            let message = if accounts.is_empty() {
                "No accounts found"
            } else {
                "Accounts fetched successfully"
            };

            Ok(ResponseStruct::success(message, accounts))
        }

        Err(e) => Ok(ResponseStruct::error(&format!("Database error: {}", e))),
    }
}

#[tauri::command]
pub async fn add_account(
    account: Account,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Account>>, String> {
    let query = "
        INSERT INTO accounts (account_name, account_type, balance, currency, is_active, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)";

    match sqlx::query(&query)
        .bind(&account.account_name)
        .bind(&account.account_type)
        .bind(&account.balance)
        .bind(&account.currency)
        .bind(&account.is_active)
        .execute(&state.db)
        .await
    {
        Ok(_) => Ok(ResponseStruct::success(
            "Account Added Successfully",
            vec![account],
        )),
        Err(e) => Ok(ResponseStruct::error(&format!("Database error: {}", e))),
    }
}

#[tauri::command]
pub async fn update_account(
    request: Account,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Account>>, String> {
    if request.id == 1 {
        return Ok(ResponseStruct::error("Cannot edit default account"));
    }

    let account_exists_query = "SELECT id FROM accounts WHERE id = ?";
    match sqlx::query_scalar::<_, i64>(&account_exists_query)
        .bind(request.id)
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => {}
        Err(sqlx::Error::RowNotFound) => {
            return Ok(ResponseStruct::error("Account not found"));
        }
        Err(e) => {
            return Ok(ResponseStruct::error(&format!("Database error: {}", e)));
        }
    };

    let mut update_query = sqlx::QueryBuilder::new("UPDATE accounts SET ");
    let mut fields = update_query.separated(", ");
    let mut is_dirty = false;

    if let Some(account_name) = &request.account_name {
        if account_name.trim().is_empty() {
            return Ok(ResponseStruct::error("Account name cannot be empty"));
        }

        fields.push("account_name = ").push_bind(account_name);
        is_dirty = true;
    }

    if let Some(account_type) = &request.account_type {
        if account_type.trim().is_empty() {
            return Ok(ResponseStruct::error("Account type cannot be empty"));
        }

        fields.push("account_type = ").push_bind(account_type);
        is_dirty = true;
    }

    if let Some(balance) = &request.balance {
        fields.push("balance = ").push_bind(balance);
        is_dirty = true;
    }

    if let Some(currency) = &request.currency {
        if currency.trim().is_empty() {
            return Ok(ResponseStruct::error("Currency cannot be empty"));
        }

        fields.push("currency = ").push_bind(currency);
        is_dirty = true;
    }

    if let Some(is_active) = &request.is_active {
        fields.push("is_active = ").push_bind(is_active);
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
                    "Account updated successfully",
                    vec![],
                ))
            }
        }
        Err(e) => Ok(ResponseStruct::error(&format!(
            "Failed to update account: {}",
            e
        ))),
    }
}

#[tauri::command]
pub async fn pre_delete_account(
    id: i64,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Account>>, String> {
    // Check if trying to delete default account
    if id == 1 {
        return Ok(ResponseStruct::error("Cannot delete default account"));
    }

    // Check if account exists
    let account_exists_query = "SELECT id FROM accounts WHERE id = ?";
    match sqlx::query_scalar::<_, i64>(&account_exists_query)
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => {}
        Err(sqlx::Error::RowNotFound) => {
            return Ok(ResponseStruct::error("Account not found"));
        }
        Err(e) => {
            return Ok(ResponseStruct::error(&format!("Database error: {}", e)));
        }
    };

    // Count how many transactions use this account
    let account_usage_count =
        "SELECT COUNT(*) as count FROM transactions WHERE transaction_account_id = ?";
    match sqlx::query_scalar::<_, i64>(&account_usage_count)
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(count) => {
            if count > 0 {
                Ok(ResponseStruct::success(
                    &format!(
                        "This account is being used in {} transactions. Do you want to proceed?",
                        count
                    ),
                    vec![],
                ))
            } else {
                Ok(ResponseStruct::success(
                    "Account can be safely deleted.",
                    vec![],
                ))
            }
        }
        Err(e) => Ok(ResponseStruct::error(&format!("Database error: {}", e))),
    }
}

#[tauri::command]
pub async fn delete_account(
    id: i64,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Account>>, String> {
    if id == 1 {
        return Ok(ResponseStruct::error("Cannot delete default account"));
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

    let account_exists_query = "SELECT id FROM accounts WHERE id = ?";
    match sqlx::query_scalar::<_, i64>(&account_exists_query)
        .bind(id)
        .fetch_one(&mut *tx)
        .await
    {
        Ok(_) => {}
        Err(sqlx::Error::RowNotFound) => {
            return Ok(ResponseStruct::error("Account not found"));
        }
        Err(e) => {
            return Ok(ResponseStruct::error(&format!("Database error: {}", e)));
        }
    };

    let update_transactions_query = "UPDATE transactions 
        SET transaction_account_id = ? 
        WHERE transaction_account_id = ?";
    match sqlx::query(&update_transactions_query)
        .bind(DEFAULT_ACCOUNT_ID)
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

    let delete_query = "DELETE FROM accounts WHERE id = ?";
    match sqlx::query(&delete_query).bind(id).execute(&mut *tx).await {
        Ok(_) => {}
        Err(e) => {
            return Ok(ResponseStruct::error(&format!(
                "Failed to delete account: {}",
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
        "Account deleted successfully",
        vec![],
    ))
}

#[tauri::command]
pub async fn get_active_accounts(
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Account>>, String> {
    let query = "
        SELECT id, account_name, account_type, balance, currency, is_active, created_at, updated_at
        FROM accounts 
        WHERE is_active = TRUE
        ORDER BY account_name";

    match sqlx::query_as(&query).fetch_all(&state.db).await {
        Ok(accounts) => {
            let message = if accounts.is_empty() {
                "No active accounts found"
            } else {
                "Active accounts fetched successfully"
            };

            Ok(ResponseStruct::success(message, accounts))
        }

        Err(e) => Ok(ResponseStruct::error(&format!("Database error: {}", e))),
    }
}

#[tauri::command]
pub async fn update_account_balance(
    id: i64,
    new_balance: f64,
    state: State<'_, database::AppState>,
) -> Result<ResponseStruct<Vec<Account>>, String> {
    let account_exists_query = "SELECT id FROM accounts WHERE id = ?";
    match sqlx::query_scalar::<_, i64>(&account_exists_query)
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => {}
        Err(sqlx::Error::RowNotFound) => {
            return Ok(ResponseStruct::error("Account not found"));
        }
        Err(e) => {
            return Ok(ResponseStruct::error(&format!("Database error: {}", e)));
        }
    };

    let update_query =
        "UPDATE accounts SET balance = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?";
    match sqlx::query(&update_query)
        .bind(new_balance)
        .bind(id)
        .execute(&state.db)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                Ok(ResponseStruct::error("No changes were made"))
            } else {
                Ok(ResponseStruct::success(
                    "Account balance updated successfully",
                    vec![],
                ))
            }
        }
        Err(e) => Ok(ResponseStruct::error(&format!(
            "Failed to update account balance: {}",
            e
        ))),
    }
}
