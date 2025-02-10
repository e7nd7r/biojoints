use std::convert::From;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Kingdom {
    pub id: Option<Uuid>,
    pub kingdom: String,
    pub superkingdom: String,
}

impl From<&(String, String)> for Kingdom {
    fn from(value: &(String, String)) -> Self {
        let (kingdom, superkingdom) = value;

        Self {
            id: None,
            kingdom: kingdom.clone(),
            superkingdom: superkingdom.clone()
        }
    }
}

