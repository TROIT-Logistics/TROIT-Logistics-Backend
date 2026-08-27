use crate::errors::AppError;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;
use tracing::info;

pub async fn init_db_pool(database_url: &str) -> Result<PgPool, AppError> {
    info!("Initializing PostgreSQL database connection pool...");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(600))
        .connect(database_url)
        .await
        .map_err(|e| {
            tracing::error!("Failed to connect to PostgreSQL: {}", e);
            AppError::InternalServerError(format!("Database connection failed: {}", e))
        })?;

    info!("Database connection established. Running pending migrations...");

    // Automatically run SQLx migrations located in ./migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute SQLx database migrations: {}", e);
            AppError::InternalServerError(format!("Database migration failed: {}", e))
        })?;

    info!("Database migrations executed successfully.");

    Ok(pool)
}
