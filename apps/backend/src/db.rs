use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::Path;

pub async fn init_db(database_url: &str) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    // Create the database file if it doesn't exist
    if !database_url.starts_with("sqlite::memory:") {
        let path_str = database_url.trim_start_matches("sqlite://");
        let path = Path::new(path_str);
        if let Some(parent) = path.parent()
            && !parent.exists()
        {
            std::fs::create_dir_all(parent)?;
        }

        if !path.exists() {
            std::fs::File::create(path)?;
        }
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn migrate(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::migrate!().run(pool).await?;
    Ok(())
}
