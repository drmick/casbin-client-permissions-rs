use actix_web::http::header::HeaderValue;
use sqlx::{query_as, Pool, Postgres};

use crate::error::AppError;
use crate::model::account::Account;
use crate::model::simple_user::SimpleUser;

#[derive(Debug, Clone)]
pub struct AccountService {
    pub secret: String,
    pub pool: Pool<Postgres>,
}

impl AccountService {
    pub async fn find_account_by_login(&self, login: &str) -> Result<Account, AppError> {
        Ok(query_as!(
            Account,
            r#"select t.id, t.email from accounts t where t.email = $1"#,
            login
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub fn get_current_user(&self, header: &HeaderValue) -> Result<SimpleUser, AppError> {
        let header = header.to_str().unwrap_or_default();
        if !header.starts_with("bearer") && !header.starts_with("Bearer") {
            return Err(AppError::HTTPBadRequest("Invalid header".to_string()));
        }
        let raw_token = header[6..header.len()].trim();
        SimpleUser::from_token(raw_token, &self.secret)
    }
}
