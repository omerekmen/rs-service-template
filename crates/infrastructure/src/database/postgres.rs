use core::time;

use shared::config::database::DatabaseConfig;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn create_postgres_pool(config: DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .min_connections(config.min_connections)
        .max_connections(config.max_connections)
        .acquire_timeout(time::Duration::from_secs(3))
        .max_lifetime(time::Duration::from_secs(config.max_lifetime_seconds))
        .idle_timeout(time::Duration::from_secs(config.idle_timeout_seconds))
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                // PostgreSQL doesn't support parameterized SET commands
                // Use format! to create the query string
                let app_name = env!("CARGO_PKG_NAME");
                let query = format!("SET application_name = '{}'", app_name);
                sqlx::query(&query)
                    .execute(&mut *conn)
                    .await?;
                Ok(())
            })
        })
        .connect(&config.connection_string)
        .await;

    let pool = match pool {
        Ok(pool) => {
            tracing::info!(
                "PostgreSQL pool established (min={}, max={}).",
                config.min_connections,
                config.max_connections
            );
            pool
        }
        Err(e) => {
            tracing::error!("Failed to connect to PostgreSQL: {}", e);
            return Err(e);
        }
    };

    if config.run_migrations {
        tracing::info!("Running database migrations...");
        if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
            tracing::error!("Migration error: {}", e);
            return Err(e.into());
        }
        tracing::info!("Database migrations complete.");
    }

    Ok(pool)
}
