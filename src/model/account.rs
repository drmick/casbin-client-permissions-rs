use crate::types::AccountID;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Account {
    pub id: AccountID,
    pub email: String,
}
