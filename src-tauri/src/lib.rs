use serde::Deserialize;

// Global DB pool
// static DB_POOL: OnceLock<Pool<Sqlite>> = OnceLock::new();

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug, Deserialize)]
struct ExpenseInput {
    title: String,
    desc: String,
    category: String,
    amount: f64,
}

#[tauri::command]
fn add_expense(expense: ExpenseInput) {
    println!("Expense created: {:?}", expense);
    const result = await db.execute(
      "INSERT into todos (id, title, status) VALUES ($1, $2, $3)",
      [todos.id, todos.title, todos.status],
    );
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, add_expense])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
