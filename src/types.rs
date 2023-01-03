use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Apiv2Schema)]
pub enum Role {
    User,
    Spec,
}

pub type AccountID = i64;
