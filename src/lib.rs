#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;

pub async fn get_database_connection() -> Result<DatabaseConnection> {
    let db_url = get_database_url()?;

    let db_max_connections = get_database_max_connections()?;
    let db_min_connections = get_database_min_connections()?;

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(db_max_connections)
        .min_connections(db_min_connections);
    let db = Database::connect(opt).await?;
    Ok(db)
}

fn get_database_url() -> Result<String> {
    Ok(env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is not set")?)
}

fn get_database_min_connections() -> Result<u32> {
    match env::var("DATABASE_MIN_CONNECTIONS") {
        Ok(val) => Ok(val
            .parse::<u32>()
            .map_err(|_| "DATABASE_MAX_CONNECTIONS must be a number")?),
        Err(err) => match err {
            env::VarError::NotPresent => Ok(0),
            env::VarError::NotUnicode(_) => Err("DATABASE_MAX_CONNECTIONS is not unicode".into()),
        },
    }
}

fn get_database_max_connections() -> Result<u32> {
    match env::var("DATABASE_MIN_CONNECTIONS") {
        Ok(val) => Ok(val
            .parse::<u32>()
            .map_err(|_| "DATABASE_MAX_CONNECTIONS must be a number")?),
        Err(err) => match err {
            env::VarError::NotPresent => Ok(100),
            env::VarError::NotUnicode(_) => Err("DATABASE_MAX_CONNECTIONS is not unicode".into()),
        },
    }
}
