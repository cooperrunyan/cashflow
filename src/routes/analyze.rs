use std::vec;

use actix_web::{
    post,
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};

use crate::{
    auth::lock,
    formatters::PhoneNumber,
    prisma::{applicant, connection, institution, member, OrderTimespan, PrismaClient, ProductSku},
};

#[derive(serde::Serialize, serde::Deserialize)]
struct RequestBody {
    name: String,
    phone_number: String,
    products: Vec<Product>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Product {
    sku: ProductSku,
    timespan: OrderTimespan,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CreatedOrder {
    id: String,
    product: ProductSku,
    timespan: OrderTimespan,
    price: f64,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SuccessResponse {
    applicant: String,
    phone_number: String,
    orders: Vec<CreatedOrder>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ErrorResponse {
    message: String,
    error: String,
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
        return HttpResponse::BadRequest().json(ErrorResponse {
            message: "Must select at least 1 product".to_string(),
            error: "bad_input".to_string(),
        });
    }

    let phone_number = match PhoneNumber::check(body.phone_number) {
        Ok(r) => r,
        Err(res) => return res,
    };

    let requester = match client
        .member()
        .find_unique(member::id::equals(user.user_id))
        .with(member::institution::fetch())
        .exec()
        .await
    {
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: e.to_string(),
                message: "Error connecting to database".to_string(),
            })
        }

        Ok(r) => match r {
            None => {
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "none".to_string(),
                    message: "Member not found".to_string(),
                })
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
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: e.to_string(),
                message: "Failed creating connection record".to_string(),
            })
        }

        Ok(connection) => connection,
    };

    let applicant = match client
        .applicant()
        .create(
            body.name,
            phone_number.clone(),
            member::id::equals(requester.id.clone()),
            institution::id::equals(requester.institution_id.clone()),
            connection::id::equals(connection.id.clone()),
            vec![],
        )
        .exec()
        .await
    {
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Failed creating applicant record".to_string(),
                error: e.to_string(),
            })
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
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    message: "Failed creating order".to_string(),
                    error: e.to_string(),
                });
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

    HttpResponse::Created().json(SuccessResponse {
        applicant: applicant.name,
        phone_number,
        orders: created_orders,
    })
}
