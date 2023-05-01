use std::vec;

use actix_web::{
    post,
    web::{Data, Json},
    HttpRequest, Responder,
};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{auth, parsers};
use crate::{response::*, twilio};

#[derive(Serialize, Deserialize)]
struct RequestBody {
    name: String,
    phone_number: String,
    products: Vec<Product>,
}

#[derive(Serialize, Deserialize)]
struct Product {
    sku: prisma::ProductSku,
    timespan: prisma::OrderTimespan,
}

#[derive(Serialize)]
struct CreatedOrder {
    id: String,
    product: prisma::ProductSku,
    timespan: prisma::OrderTimespan,
    price: f64,
}

#[derive(Serialize)]
struct SuccessResponse {
    applicant: String,
    phone_number: String,
    orders: Vec<CreatedOrder>,
}

#[post("/analyze")]
async fn analyze(
    client: Data<prisma::PrismaClient>,
    body: Json<RequestBody>,
    req: HttpRequest,
) -> impl Responder {
    let user = match auth::lock(req) {
        Err(res) => return res,
        Ok(user) => user,
    };

    let body = body.into_inner();

    if body.products.len() <= 0 {
        return error(Status::BadInput, "At least 1 product must be selected").finish();
    }

    let phone_number = match parsers::check_phone_number(body.phone_number) {
        Ok(r) => r,
        Err(res) => return res,
    };

    let requester = match client
        .member()
        .find_unique(prisma::member::id::equals(user.user_id))
        .with(prisma::member::organization::fetch())
        .exec()
        .await
    {
        Err(e) => {
            return error(Status::InternalServerError, e).finish();
        }

        Ok(r) => match r {
            None => {
                return error(
                    Status::DataNotFound,
                    format!("Could not find member with matching email"),
                )
                .finish();
            }

            Some(requester) => requester,
        },
    };

    let connection = match client
        .connection()
        .create("Registering applicant".to_string(), vec![])
        .exec()
        .await
    {
        Err(e) => {
            return error(
                Status::FailedToCreateData,
                format!("Could not create connection record. {e}"),
            )
            .finish();
        }

        Ok(c) => c,
    };

    let applicant = match client
        .applicant()
        .create(
            body.name,
            phone_number.clone(),
            prisma::member::id::equals(requester.id.clone()),
            prisma::organization::id::equals(requester.organization_id.clone()),
            prisma::connection::id::equals(connection.id.clone()),
            vec![],
        )
        .exec()
        .await
    {
        Err(e) => {
            return error(
                Status::FailedToCreateData,
                format!("Could not create applicant record. {e}"),
            )
            .finish()
        }
        Ok(applicant) => applicant,
    };

    let orders = futures::future::join_all(body.products.iter().map(|product| async {
        client
            .order()
            .create(
                prisma::applicant::id::equals(applicant.clone().id),
                prisma::member::id::equals(requester.clone().id),
                product.sku,
                10.00,
                product.timespan,
                "Processing order request".to_string(),
                vec![],
            )
            .exec()
            .await
    }))
    .await;

    let mut created_orders: Vec<CreatedOrder> = vec![];

    for order in orders.iter() {
        match order {
            Err(e) => {
                return error(
                    Status::FailedToCreateData,
                    format!("Failed creating order. {e}"),
                )
                .finish();
            }
            Ok(o) => created_orders.append(&mut vec![CreatedOrder {
                id: o.to_owned().id,
                product: o.product,
                price: o.price,
                timespan: o.timespan,
            }]),
        };
    }

    twilio::sms(&phone_number, "Hey").await;

    success(Status::OrderedProduts, "Successfully ordered")
        .data(json!(SuccessResponse {
            applicant: applicant.name,
            phone_number,
            orders: created_orders,
        }))
        .finish()
}
