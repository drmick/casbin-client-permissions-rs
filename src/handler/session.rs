use crate::error::AppError;
use crate::types::Role;
use crate::{AccountService, TokenService};
use actix_web::web::Json;
use paperclip::actix::api_v2_operation;
use paperclip::actix::web;
use paperclip::actix::Apiv2Schema;
#[derive(Deserialize, Apiv2Schema)]
pub struct CreateSessionPayload {
    login: String,
}

#[derive(Serialize, Apiv2Schema)]
pub struct CreateSessionResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Deserialize, Debug, Apiv2Schema)]
pub struct CreateQuery {
    role: Role,
}

#[api_v2_operation]
pub async fn create(
    payload: Json<CreateSessionPayload>,
    query: web::Query<CreateQuery>,
    account_service: web::Data<AccountService>,
    token_service: web::Data<TokenService>,
) -> Result<Json<CreateSessionResponse>, AppError> {
    let account = account_service
        .find_account_by_login(&payload.login)
        .await?;
    let access_token = token_service.generate_access_token(&account, query.0.role)?;
    let refresh_token = token_service.generate_refresh_token(&account)?;
    let response = CreateSessionResponse {
        access_token,
        refresh_token,
    };
    Ok(Json(response))
}
