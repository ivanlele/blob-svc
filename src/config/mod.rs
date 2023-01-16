use config::Config;

use serde::{Deserialize};

#[derive(Deserialize)]
pub struct ServiceConfig {
    pub db_url: String,
    pub port: u16,
}

pub fn get_config() -> ServiceConfig {
    let raw_config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()
        .unwrap();
    
    raw_config.try_deserialize::<ServiceConfig>().unwrap()
}