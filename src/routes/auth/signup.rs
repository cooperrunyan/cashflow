use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};

use crate::auth;
use crate::response::*;

#[derive(serde::Serialize, serde::Deserialize)]
struct RequestBody {
    org_name: String,
    user_name: String,
    user_email: String,
    user_password: String,
}

#[derive(serde::Serialize)]
struct SuccessResponse {
    message: String,
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    message: String,
}

#[post("/signup")]
async fn signup(client: Data<prisma::PrismaClient>, body: Json<RequestBody>) -> impl Responder {
    let body = body.into_inner();

    let org = match client
        .organization()
        .create(body.org_name, vec![])
        .exec()
        .await
    {
        Err(e) => {
            return error(
                Status::FailedToCreateData,
                format!("Error creating org. {e}"),
            )
            .finish()
        }
        Ok(org) => org,
    };

    let owner = match client
        .member()
        .create(
            body.user_name,
            body.user_email,
            auth::hash(body.user_password),
            prisma::organization::id::equals(org.id.clone()),
            vec![prisma::member::role::set(prisma::Role::Owner)],
        )
        .exec()
        .await
    {
        Err(e) => {
            return error(Status::DataNotFound, format!("Error finding member. {e}")).finish()
        }
        Ok(owner) => owner,
    };

    success(Status::CreatedOrganization, "Done")
        .cookie(auth::jwt::gen_cookie(owner.id, owner.email))
        .finish()
}
