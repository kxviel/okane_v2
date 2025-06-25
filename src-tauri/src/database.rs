use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use tauri::{App, Manager};

pub struct AppState {
    pub db: Pool<Sqlite>,
}

pub async fn db_connection(app: &App) -> Pool<Sqlite> {
    let mut path = app.path().app_data_dir().expect("failed to get data_dir");

    match std::fs::create_dir_all(path.clone()) {
        Ok(_) => {}
        Err(err) => {
            panic!("error creating directory {}", err);
        }
    };

    path.push("okane.sqlite");

    let db_url = format!(
        "sqlite:{}",
        path.to_str().expect("path should be something")
    );

    // Only create database if it doesn't exist
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url)
            .await
            .expect("failed to create database");
    }

    let db = SqlitePoolOptions::new()
        .connect(&db_url) // Use the full URL, not just the path
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./src/migrations/").run(&db).await.unwrap();

    db
}
