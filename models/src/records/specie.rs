use chrono::{NaiveDate, NaiveDateTime};
use mysql_common::prelude::*;
use serde::{Deserialize, Serialize};

use crate::data::data_error::DataError;

use super::state::State;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct Specie {
    pub id: uuid::Uuid,
    pub genus: String,
    pub specie_code: String,
    pub common_name: String,
    pub distribution: String,
    pub specie_author: String,
    pub specie_name: String,
    pub changed_by: String,
    pub changed_date: Option<NaiveDateTime>,
    pub record_date: Option<NaiveDate>,
    pub subgenus: String,
    pub subspecie_author: String,
    pub subspecie: String,
    pub valid_specie_code: String,
    pub published: bool,
}

impl Specie {
    /// Converts the distribution string into a vector of standardized state codes
    pub fn parse_distribution(&self) -> Result<Vec<String>, DataError> {
        self.distribution
            .split(',')
            .map(str::trim)
            .filter(|code| !code.is_empty())
            .map(|code| State::nonsdt_to_code(&code.to_lowercase()))
            .collect()
    }
}
