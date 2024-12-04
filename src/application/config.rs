use std::env;
use std::error::Error;

pub struct AppConfig {
    pub app_addr: String,
    pub mongo_username: String,
    pub mongo_password: String,
}

impl AppConfig {
    pub fn new(addr: String, username: String, password: String) -> Self {
        AppConfig {
            app_addr: addr,
            mongo_username: username,
            mongo_password: password,
        }
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let mongo_username = env::var("MONGO_USERNAME")
        .expect("MONGO_USERNAME is not set");
        let mongo_password = env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD is not set");
        let app_addr = env::var("APP_PORT").unwrap_or_else(|_| "[::]:8080".to_string());
        Ok(AppConfig::new(app_addr, mongo_username, mongo_password))
    }
}