use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct Class {
    pub id: Option<Uuid>,
    pub phylum: String,
    pub class: String,
    pub subphylum: String,
}

