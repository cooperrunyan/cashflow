use dotenv::dotenv;
use serde::Deserialize;

use crate::middleware::init_logger;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub hash_key: String,
    pub hash_salt: String,
    pub jwt_key: String,
    pub jwt_exp: i64,
    pub db_url: String,
    pub port: u16,
    pub log_level: String,
    pub twilio_auth_tkn: String,
    pub twilio_acc_id: String,
    pub twilio_phone: String,
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
    debug!("  port:       '{}'", env.port);
    debug!("");
    debug!("  hash_key:   '{}'", env.hash_key);
    debug!("  hash_salt:  '{}'", env.hash_salt);
    debug!("");
    debug!("  jwt_key:    '{}'", env.jwt_key);
    debug!("  jwt_exp:    '{}'", env.jwt_exp);
    debug!("");
    debug!("  twilio_acc_id:       '{}'", env.twilio_acc_id);
    debug!("  twilio_auth_tkn:     '{}'", env.twilio_auth_tkn);
    debug!("  twilio_phone:        '{}'", env.twilio_phone);
    debug!("");
    debug!("  db_url: '{}'", env.db_url);
    debug!("");

    env
}
