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

    Ok(pool)
}

pub async fn migrate(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::migrate!().run(pool).await?;
    Ok(())
}
