use std::collections::HashMap;
use std::sync::Arc;

use crate::error::AppError;
use crate::model::simple_user::SimpleUser;
use crate::{AccountService, CasbinService};
use actix_web::http::header::HeaderValue;
use actix_web::web::Json;
use actix_web::HttpRequest;
use paperclip::actix::api_v2_operation;
use paperclip::actix::web;

#[api_v2_operation]
pub async fn permissions(
    request: HttpRequest,
    casbin_service: web::Data<Arc<CasbinService>>,
    account_service: web::Data<AccountService>,
) -> Result<Json<HashMap<String, Vec<String>>>, AppError> {
    let auth_header: Option<&HeaderValue> = request.headers().get("Authorization");
    let current_user = match auth_header {
        None => SimpleUser::guest(),
        Some(header) => account_service.get_current_user(header)?,
    };
    let permissions = casbin_service.get_permissions_for_role(&current_user.role);
    Ok(Json(permissions))
}
