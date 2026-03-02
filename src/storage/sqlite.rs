use sqlx::migrate::Migrator;
use sqlx::sqlite::SqlitePoolOptions;
use std::path::Path;
use tracing::info;

use crate::storage::Result;

pub type SqlitePool = sqlx::SqlitePool;

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    info!("Creating SQLite connection pool: {}", database_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    info!("Running migrations...");
    MIGRATOR.run(&pool).await?;
    info!("Migrations completed");

    Ok(pool)
}

pub async fn create_pool_from_path<P: AsRef<Path>>(path: P) -> Result<SqlitePool> {
    let path = path.as_ref();
    let database_url = format!("sqlite:{}?mode=rwc", path.display());
    create_pool(&database_url).await
}

pub async fn health_check(pool: &SqlitePool) -> Result<bool> {
    let result: Option<(i32,)> = sqlx::query_as("SELECT 1").fetch_optional(pool).await?;

    Ok(result.is_some())
}
