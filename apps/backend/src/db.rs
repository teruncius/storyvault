use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::Path;

pub async fn init_db(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // Create the database file if it doesn't exist
    if !database_url.starts_with("sqlite::memory:") {
        let path_str = database_url.trim_start_matches("sqlite://");
        let path = Path::new(path_str);
        if let Some(parent) = path.parent()
            && !parent.exists()
        {
            std::fs::create_dir_all(parent).unwrap_or_else(|e| {
                eprintln!("Failed to create database directory: {}", e);
            });
        }

        if !path.exists() {
            std::fs::File::create(path).unwrap_or_else(|e| {
                eprintln!("Failed to create database file: {}", e);
                std::process::exit(1);
            });
        }
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS events (
            event_id TEXT PRIMARY KEY,
            topic TEXT NOT NULL,
            payload TEXT NOT NULL,
            created_at DATETIME NOT NULL
        );
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
