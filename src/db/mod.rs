use sea_orm::{Database, DatabaseConnection};
use crate::config::Config;
use crate::errors::Result;

pub async fn connect(config: &Config) -> Result<DatabaseConnection> {
    let db = Database::connect(&config.database_url).await?;
    Ok(db)
}
