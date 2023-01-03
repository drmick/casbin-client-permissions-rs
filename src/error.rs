use actix_web::error::BlockingError;
use derive_more::{Display, Error};
use failure::Fail;
use paperclip::actix::api_v2_errors;
use serde_json::{Map as JsonMap, Value as JsonValue};

#[derive(Display, Debug, Error)]
pub struct FatalError {
    pub message: String,
}

#[api_v2_errors(code = 400, code = 404, code = 401, code = 500, code = 422)]
#[derive(Debug, Fail)]
pub enum AppError {
    #[fail(display = "Argon2 Error {}", _0)]
    Argon2Error(argon2::Error),
    #[fail(display = "sqlx error Error ")]
    SqlxError(sqlx::Error),
    #[fail(display = "Blocking Error")]
    BlockingError(BlockingError),
    #[fail(display = "Jwt Encode Errror: {}", _0)]
    JwtError(jsonwebtoken::errors::Error),
    #[fail(display = "Validation Error {}", _0)]
    ValidationErrors(validator::ValidationErrors),
    #[fail(display = "Fatal Error {}", _0)]
    SomeFatalError(FatalError),
    #[fail(display = "Bad request")]
    HTTPBadRequest(String),
}

impl From<BlockingError> for AppError {
    fn from(e: BlockingError) -> Self {
        AppError::BlockingError(e)
    }
}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            AppError::Argon2Error(ref err) => internal_server_error_response(err),

            AppError::SqlxError(ref err) => match err {
                sqlx::Error::RowNotFound => not_found_response(),
                _ => internal_server_error_response(self),
            },

            AppError::JwtError(ref err) => unauthorized_response(&format!("{err:?}")),
            AppError::BlockingError(ref err) => internal_server_error_response(err),
            AppError::SomeFatalError(ref err) => internal_server_error_response(err),
            AppError::HTTPBadRequest(message) => internal_server_error_response(message),
            AppError::ValidationErrors(ref errs) => {
                unprocessable_entity_response(&validation_errs_to_json(errs))
            }
        }
    }
}

impl From<argon2::Error> for AppError {
    fn from(error: argon2::Error) -> Self {
        AppError::Argon2Error(error)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        AppError::JwtError(error)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(errors: sqlx::Error) -> Self {
        AppError::SqlxError(errors)
    }
}
impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        AppError::ValidationErrors(errors)
    }
}

impl From<FatalError> for AppError {
    fn from(errors: FatalError) -> Self {
        AppError::SomeFatalError(errors)
    }
}

fn validation_errs_to_json(errors: &validator::ValidationErrors) -> JsonValue {
    let mut err_map = JsonMap::new();

    for (field, errors) in errors.clone().field_errors().iter() {
        let errors: Vec<JsonValue> = errors.iter().map(|error| json!(error.message)).collect();
        err_map.insert(field.to_string(), json!(errors));
    }
    json!({ "errors": err_map })
}

fn unprocessable_entity_response(json: &serde_json::Value) -> actix_web::HttpResponse {
    actix_web::HttpResponse::build(actix_web::http::StatusCode::UNPROCESSABLE_ENTITY).json(json)
}

fn internal_server_error_response<T>(err: T) -> actix_web::HttpResponse
where
    T: std::fmt::Debug,
{
    log::error!("{err:?}");
    actix_web::HttpResponse::InternalServerError().json(json!({"error": "Internal Server Error"}))
}

fn not_found_response() -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotFound().json(json!({"error": "Not found"}))
}

fn unauthorized_response(message: &str) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Unauthorized().json(json!({ "error": message }))
}
