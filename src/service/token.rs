use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::error::AppError;
use crate::model::account::Account;
use crate::types::{AccountID, Role};

#[derive(Debug, Clone)]
pub struct TokenService {
    pub secret: String,
    pub access_token_lifetime_ms: i64,
    pub refresh_token_lifetime_ms: i64,
}
#[derive(Serialize)]
struct AccessTokenClaim<'a> {
    account_id: AccountID,
    name: &'a str,
    role: Role,
    exp: i64,
}

#[derive(Serialize)]
struct RefreshTokenClaim {
    account_id: AccountID,
    exp: i64,
}

impl TokenService {
    pub fn generate_access_token(&self, account: &Account, role: Role) -> Result<String, AppError> {
        let expiration = Utc::now() + Duration::milliseconds(self.access_token_lifetime_ms);
        let claim = AccessTokenClaim {
            account_id: account.id,
            name: &account.email,
            role,
            exp: expiration.timestamp_millis(),
        };
        let token = encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;
        Ok(token)
    }

    pub fn generate_refresh_token(&self, account: &Account) -> Result<String, AppError> {
        let expiration = Utc::now() + Duration::milliseconds(self.refresh_token_lifetime_ms);
        let claim = RefreshTokenClaim {
            account_id: account.id,
            exp: expiration.timestamp_millis(),
        };
        let token = encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;
        Ok(token)
    }
}
