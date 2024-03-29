use dotenv::dotenv;
use serde::Deserialize;
use eyre::WrapErr;
use color_eyre::Result;
use tracing_subscriber::EnvFilter;
use tracing;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub database_url: String,
}

impl Config {
    #[tracing::instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
        let mut config = config::Config::new();
        config.merge(config::Environment::default())?;
        config.try_into().context("config from env")
    }
}