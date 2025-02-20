use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct Genus {
    pub id: Option<Uuid>,
    pub family: String,
    pub genus: String,
    pub subfamily: String,
    pub tribe: String
}

