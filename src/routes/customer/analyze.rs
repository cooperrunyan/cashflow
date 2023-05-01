use std::vec;

use actix_web::{
    post,
    web::{Data, Json},
    HttpRequest, Responder,
};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    auth::lock,
    io::{
        input::phone_number,
        output::{error, success},
        Status,
    },
    prisma::{
        applicant, connection, member, organization, OrderTimespan, PrismaClient, ProductSku,
    },
};

#[derive(Serialize, Deserialize)]
struct RequestBody {
    name: String,
    phone_number: String,
    products: Vec<Product>,
}

#[derive(Serialize, Deserialize)]
struct Product {
    sku: ProductSku,
    timespan: OrderTimespan,
}

#[derive(Serialize)]
struct CreatedOrder {
    id: String,
    product: ProductSku,
    timespan: OrderTimespan,
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
    client: Data<PrismaClient>,
    body: Json<RequestBody>,
    req: HttpRequest,
) -> impl Responder {
    let user = match lock(req) {
        Err(res) => return res,
        Ok(user) => user,
    };

    let body = body.into_inner();

    if body.products.len() <= 0 {
        return error::new(Status::BadInput, "At least 1 product must be selected").finish();
    }

    let phone_number = match phone_number::check(body.phone_number) {
        Ok(r) => r,
        Err(res) => return res,
    };

    let requester = match client
        .member()
        .find_unique(member::id::equals(user.user_id))
        .with(member::organization::fetch())
        .exec()
        .await
    {
        Err(e) => {
            return error::new(Status::InternalServerError, e).finish();
        }

        Ok(r) => match r {
            None => {
                return error::new(
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
            return error::new(
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
            member::id::equals(requester.id.clone()),
            organization::id::equals(requester.organization_id.clone()),
            connection::id::equals(connection.id.clone()),
            vec![],
        )
        .exec()
        .await
    {
        Err(e) => {
            return error::new(
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
                applicant::id::equals(applicant.clone().id),
                member::id::equals(requester.clone().id),
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
                return error::new(
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

    // Send text msg to phone number

    success::new(Status::OrderedProduts, "Successfully ordered")
        .data(json!(SuccessResponse {
            applicant: applicant.name,
            phone_number,
            orders: created_orders,
        }))
        .finish()
}
