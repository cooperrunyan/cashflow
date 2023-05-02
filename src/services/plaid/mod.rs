use std::{collections::HashMap, result, time::Duration};

use reqwest::{Client, Method, RequestBuilder, Url};
use serde_json::Value;

use crate::env::ENV;

mod create_link_token;
pub use create_link_token::*;

lazy_static! {
    pub static ref PLAID: PlaidClient =
        PlaidClient::new(&ENV.plaid_env, &ENV.plaid_client_id, &ENV.plaid_secret,);
}

pub struct PlaidClient {
    env: Url,
    acc_id: String,
    secret: String,
    client: Client,
}

impl PlaidClient {
    pub fn new(env: impl ToString, acc_id: impl ToString, secret: impl ToString) -> Self {
        let c = Self {
            env: Url::parse(format!("https://{}.plaid.com", env.to_string()).as_str()).unwrap(),
            acc_id: acc_id.to_string(),
            secret: secret.to_string(),
            client: Client::builder()
                .connect_timeout(Duration::from_secs(30))
                .build()
                .unwrap(),
        };

        debug!("Initialized Plaid client");

        c
    }

    pub async fn send(
        &self,
        req: RequestBuilder,
    ) -> result::Result<HashMap<String, Value>, String> {
        match self.client.execute(req.build().unwrap()).await {
            Err(e) => Err(format!("{:?}", e)),
            Ok(res) => match res.json::<HashMap<String, Value>>().await {
                Err(e) => Err(format!("{:?}", e)),
                Ok(v) => match v.get("error_message") {
                    None => Ok(v),
                    Some(e) => Err(e.to_string().replace('"', "\'")),
                },
            },
        }
    }

    fn req(&self, method: Method, path: &str) -> RequestBuilder {
        self.client
            .request(method, self.env.join(path).unwrap())
            .header("Accept", "application/json")
            .header("Plaid-client-id", &self.acc_id)
            .header("Plaid-secret", &self.secret)
    }

    pub fn get(&self, path: &str) -> RequestBuilder {
        self.req(Method::GET, path)
    }

    pub fn post(&self, path: &str) -> RequestBuilder {
        self.req(Method::POST, path)
    }
}
