use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct Phylum {
    pub id : Option<Uuid>,
    pub kingdom: String,
    pub phylum: String,
    pub subkingdom: String,
}

