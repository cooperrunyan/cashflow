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
    pub plaid_env: String,
    pub plaid_client_id: String,
    pub plaid_secret: String,
    pub plaid_version: String,
    pub localhost: String,
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

    debug!("Environment loaded");
    trace!("");
    trace!("  port:       '{}'", env.port);
    trace!("");
    trace!("  hash_key:   '{}'", env.hash_key);
    trace!("  hash_salt:  '{}'", env.hash_salt);
    trace!("");
    trace!("  jwt_key:    '{}'", env.jwt_key);
    trace!("  jwt_exp:    '{}'", env.jwt_exp);
    trace!("");
    trace!("  twilio_acc_id:       '{}'", env.twilio_acc_id);
    trace!("  twilio_auth_tkn:     '{}'", env.twilio_auth_tkn);
    trace!("  twilio_phone:        '{}'", env.twilio_phone);
    trace!("");
    trace!("  db_url: '{}'", env.db_url);
    trace!("");

    env
}
