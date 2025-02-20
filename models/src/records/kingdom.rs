use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct Kingdom {
    pub id: Option<Uuid>,
    pub kingdom: String,
    pub superkingdom: String,
}

