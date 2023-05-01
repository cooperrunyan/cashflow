use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};

use crate::{
    auth,
    io::{
        output::{error, success},
        Status,
    },
    prisma::{member, organization, PrismaClient, Role},
};

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
async fn signup(client: Data<PrismaClient>, body: Json<RequestBody>) -> impl Responder {
    let body = body.into_inner();

    let org = match client
        .organization()
        .create(body.org_name, vec![])
        .exec()
        .await
    {
        Err(e) => {
            return error::new(
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
            organization::id::equals(org.id.clone()),
            vec![member::role::set(Role::Owner)],
        )
        .exec()
        .await
    {
        Err(e) => {
            return error::new(Status::DataNotFound, format!("Error finding member. {e}")).finish()
        }
        Ok(owner) => owner,
    };

    success::new(Status::CreatedOrganization, "Done")
        .cookie(auth::jwt::gen_cookie(owner.id, owner.email))
        .finish()
}
