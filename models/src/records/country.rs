use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct Country {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub code: String,
}

impl From<(String, String)> for Country {
    fn from((name, code): (String, String)) -> Self {
        Self {
            id: None,
            name,
            code,
        }
    }
}

