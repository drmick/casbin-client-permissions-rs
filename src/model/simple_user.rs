use crate::error::AppError;
use crate::types::AccountID;
use jsonwebtoken::DecodingKey;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleUser {
    pub account_id: AccountID,
    pub name: String,
    pub role: String,
}

impl SimpleUser {
    pub fn guest() -> SimpleUser {
        Self {
            account_id: Default::default(),
            name: "Guest".to_string(),
            role: "Guest".to_string(),
        }
    }

    pub fn from_token(token: &str, secret: &str) -> Result<SimpleUser, AppError> {
        let token_data = jsonwebtoken::decode::<SimpleUser>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}
