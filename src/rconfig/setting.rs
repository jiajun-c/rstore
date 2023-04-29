use config::{ConfigError, Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub db: Db,
}

#[derive(Debug, Deserialize)]
pub struct Db {
    pub url: String
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: i32
}

impl Settings {
    pub fn new(filepath: String) -> Result<Self, ConfigError> {
        let s = Config::builder()
        .add_source(File::with_name(&filepath)).build()?;
        s.try_deserialize()
    }
}