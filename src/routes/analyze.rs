use std::vec;

use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::prisma::{
    applicant, connection, institution, member, OrderTimespan, PrismaClient, ProductSku,
};

#[derive(serde::Serialize, serde::Deserialize)]
struct RequestBody {
    name: String,
    phone_number: String,
    member: String,
    products: Vec<Product>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Product {
    sku: ProductSku,
    timespan: OrderTimespan,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SuccessResponse {
    applicant: String,
    phone_number: String,
    orders: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ErrorResponse {
    message: String,
    error: String,
}

#[post("/analyze")]
async fn analyze(client: Data<PrismaClient>, body: Json<RequestBody>) -> impl Responder {
    if body.products.len() <= 0 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            message: "Must select at least 1 product".to_string(),
            error: "Bad input".to_string(),
        });
    }

    let body = body.into_inner();

    let requester = client
        .member()
        .find_unique(member::id::equals(body.member))
        .with(member::institution::fetch())
        .exec()
        .await;

    if requester.is_err() {
        let error = requester.unwrap_err();
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: error.to_string(),
            message: "Member not found".to_string(),
        });
    }

    let requester = requester.unwrap().unwrap();

    let connection = client
        .connection()
        .create("Registering applicant".to_string(), vec![])
        .exec()
        .await;

    if connection.is_err() {
        let error = connection.unwrap_err();
        return HttpResponse::InternalServerError().json(ErrorResponse {
            message: "Failed creating connection record".to_string(),
            error: error.to_string(),
        });
    }

    let connection = connection.unwrap();

    let applicant = client
        .applicant()
        .create(
            body.name,
            body.phone_number,
            member::id::equals(requester.id.clone()),
            institution::id::equals(requester.institution_id.clone()),
            connection::id::equals(connection.id.clone()),
            vec![],
        )
        .exec()
        .await;

    if applicant.is_err() {
        let error = applicant.unwrap_err();
        return HttpResponse::InternalServerError().json(ErrorResponse {
            message: "Failed creating applicant record".to_string(),
            error: error.to_string(),
        });
    }

    let applicant = applicant.unwrap();

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

    for order in orders.iter().clone() {
        if order.is_err() {
            let error = order.as_ref().unwrap_err();
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Failed creating order".to_string(),
                error: error.to_string(),
            });
        }
    }

    // Send text msg to phone number

    HttpResponse::Ok().json(SuccessResponse {
        applicant: applicant.name,
        phone_number: applicant.phone_number,
        orders: orders
            .iter()
            .map(|order| order.as_ref().unwrap().id.clone())
            .collect(),
    })
}
