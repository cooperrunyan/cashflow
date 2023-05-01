use dotenv::dotenv;
use serde::Deserialize;

use crate::middleware::init_logger;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub hash_key: String,
    pub hash_salt: String,
    pub jwt_key: String,
    pub jwt_expiration: i64,
    pub database_url: String,
    pub port: u16,
    pub log_level: String,
}

lazy_static! {
    pub static ref ENV: Config = get_config();
}

fn get_config() -> Config {
    dotenv().expect("Dotenv result failed");

    init_logger();

    let env = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}", error),
    };

    info!(
        "Launching Cashflow Server with log level set to {}",
        env.log_level
    );

    debug!("Environment:");
    debug!("");
    debug!("  hash_key:   '{}'", env.hash_key);
    debug!("  hash_salt:  '{}'", env.hash_salt);
    debug!("  jwt_key:    '{}'", env.jwt_key);
    debug!("  jwt_exp:    '{}'", env.jwt_expiration);
    debug!("  port:       '{}'", env.port);
    debug!("  db_url:     '{}'", env.database_url);
    debug!("");

    env
}
