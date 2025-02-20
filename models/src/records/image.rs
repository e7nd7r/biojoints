use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct Image {
    pub id: Option<i32>,
    pub file_name: String,
    pub notes: String,
    pub specie_code: String,
    pub author_id: i32,
    pub valid_specie_code: String,
}

