use dotenvy::dotenv;
use std::env;

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    dotenv().ok();
    let server_address = env::var("SERVER_ADDRESS")?;
    let db_url = env::var("DATABASE_URL")?;
    Ok(Config {server_address, db_url})
}

pub struct Config {
    pub server_address: String,
    pub db_url: String,
}