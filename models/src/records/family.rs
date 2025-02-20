use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct Family {
    pub id: Option<uuid::Uuid>,
    pub order: String,
    pub family: String,
    pub suborder: String,
    pub superfamily: String,
}

