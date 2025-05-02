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

    Ok(pool)
}
