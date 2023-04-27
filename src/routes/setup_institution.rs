use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::prisma::{institution, member, PrismaClient, Role};

#[derive(serde::Serialize, serde::Deserialize)]
struct RequestBody {
    name: String,
    admin_name: String,
    admin_email: String,
    admin_password: String,
}

#[derive(serde::Serialize)]
struct SuccessResponse {
    message: String,
    institution: String,
    owner: String,
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    message: String,
}

#[post("/setup_institution")]
async fn setup_institution(client: Data<PrismaClient>, body: Json<RequestBody>) -> impl Responder {
    let body = body.into_inner();

    let institution = client.institution().create(body.name, vec![]).exec().await;

    if institution.is_err() {
        let error = institution.unwrap_err();
        return HttpResponse::InternalServerError().json(ErrorResponse {
            message: error.to_string(),
        });
    }

    let institution = institution.unwrap();

    let institution_id = institution.id.clone();

    let owner = client
        .member()
        .create(
            body.admin_name,
            body.admin_email,
            body.admin_password,
            institution::id::equals(institution_id),
            vec![member::role::set(Role::Owner)],
        )
        .exec()
        .await;

    if owner.is_err() {
        let error = owner.unwrap_err();
        return HttpResponse::InternalServerError().json(ErrorResponse {
            message: error.to_string(),
        });
    }

    let owner = owner.unwrap();

    HttpResponse::Ok().json(SuccessResponse {
        message: "Institution ready".to_string(),
        institution: format!("{:?}", institution),
        owner: format!("{:?}", owner),
    })
}
