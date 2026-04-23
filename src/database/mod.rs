mod models;

use deadpool_postgres::{Pool, Runtime};
use tokio_postgres::NoTls;

#[derive(Debug, serde::Deserialize)]
struct DatabaseConfig {
    pg: deadpool_postgres::Config,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

pub async fn create_pool() -> Result<Pool, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let cfg = DatabaseConfig::from_env().expect("Ошибка в создании конфигурации БД");
    Ok(cfg
        .pg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Ошибка в создании pool"))
}
