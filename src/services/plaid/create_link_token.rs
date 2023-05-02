use std::result;

use crate::ENV;

use super::PLAID;

use chrono::{TimeZone, Utc};
use prisma::PrismaClient;

use serde_json::json;

lazy_static! {
    static ref WEHBOOK: String = format!("{}/webhook/link", &ENV.localhost);
}

pub async fn create_link_token(
    client: &PrismaClient,
    id: &String,
) -> result::Result<String, String> {
    trace!("Webhook endpoint: {}", *WEHBOOK);

    let body = json!({
      "client_name": "Cashflow",
      "language": "en",
      "country_codes": &["US"],
      "products": &["auth", "assets"],
      "webhook": *WEHBOOK,
      "user": json!( {
          "client_user_id": id,
      })
    });

    let (tkn, exp) = match PLAID
        .send(PLAID.post("/link/token/create").json(&body))
        .await
    {
        Err(e) => {
            return Err(e);
        }
        Ok(res) => {
            debug!("Link token successful");

            let link_token = res.get("link_token").unwrap();
            let expiration = res.get("expiration").unwrap().as_str().unwrap();

            (
                link_token.to_string(),
                Utc.datetime_from_str(expiration, "%Y-%m-%dT%H:%M:%SZ")
                    .unwrap(),
            )
        }
    };

    debug!("Creating link token");

    let link = format!(
        "http://localhost:8000/link/{}",
        tkn.replace('"', "").as_str()
    );

    debug!("Created link token");
    trace!("  token: {}", tkn);
    trace!("  link: {}", link);
    trace!("  expiration: {}", exp);

    match client
        .applicant()
        .update(
            prisma::applicant::id::equals(id.to_string()),
            vec![
                prisma::applicant::link::set(Some(link.to_string())),
                prisma::applicant::link_ready::set(true),
                prisma::applicant::link_ready_at::set(Some(Utc::now().into())),
                prisma::applicant::link_exp::set(Some(exp.into())),
            ],
        )
        .exec()
        .await
    {
        Err(e) => return Err(e.to_string()),
        Ok(_) => Ok(link.to_owned()),
    }
}
