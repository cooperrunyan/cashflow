use actix_web::{
    cookie::Cookie,
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::{
    auth::{self, JwtPayload},
    prisma::{institution, member, PrismaClient, Role},
};

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
            auth::hash(body.admin_password.as_str()),
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

    HttpResponse::Ok()
        .cookie(
            Cookie::build(
                "jwt",
                auth::create_jwt(JwtPayload::new(owner.id, owner.email)).unwrap(),
            )
            // .domain("http://localhost:8000")
            // .path("/")
            // .secure(true)
            // .http_only(true)
            .finish(),
        )
        .json(SuccessResponse {
            message: "Done".to_string(),
        })
}
