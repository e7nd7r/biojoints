use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Option<Uuid>,
    pub class: String,
    pub order: String,
    pub subclass: String,
    pub superorder: String,
}

