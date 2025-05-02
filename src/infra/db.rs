use sqlx::{
    Pool, Postgres, migrate,
    postgres::{PgConnectOptions, PgPoolOptions},
};

use crate::config::config::Config;

pub async fn setup_database(config: &Config) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .min_connections(config.database_min_connections)
        .connect(&config.database_url)
        .await?;

    println!("Connected to database");
    println!("Running migrations...");
    migrate!("./migrations").run(&pool).await?;
    println!("Migrations completed");
    // Create connection options
    // let connect_options = PgPoolOptions::connect(&config.database_url)
    //     .map_err(|e| {
    //         tracing::error!("Failed to parse database URL: {}", e);
    //         e
    //     })?
    //     .charset(&config.database_charset)
    //     .clone();

    // let pool = PgPoolOptions::new()
    //     .max_connections(config.database_max_connections)
    //     .min_connections(config.database_min_connections)
    //     .connect_with(connect_options)
    //     .await?;

    Ok(pool)
}
