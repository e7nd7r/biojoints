use std::convert::From;
use chrono::NaiveDate;

use crate::data::data_error::DataError;

use super::state::State;

pub struct Specie {
    pub genus: String,
    pub specie_code: String,
    pub common_name: String,
    pub distribution: String,
    pub specie_author: String,
    pub specie_name: String,
    pub changed_by: String,
    pub changed_date: NaiveDate,
    pub record_date: NaiveDate,
    pub subgenus: String,
    pub subspecie_author: String,
    pub subspecie: String,
    pub valid_specie_code: String,
    pub published: bool,
}

pub type SpecieRecord = (
    String, // Genus
    String, // Specie Code
    String, // Common Name
    String, // Distribution
    String, // Specie Author
    String, // Specie Name
    String, // Changed By
    NaiveDate, // Changed Date
    NaiveDate, // Record Date
    String, // Subgenus
    String, // Subspecie Author
    String, // Subspecie
    String, // Valid Specie Code
    bool, // published
);

impl Specie {
    pub fn dist_to_vec(&self) -> Result<Vec<String>, DataError> {
        self.distribution
            .split(",")
            .map(|code| code.trim())
            .filter(|code| !code.is_empty())
            .filter(|code| code.eq(&"m"))
            .map(|code| State::nonsdt_to_code(&code.to_lowercase()))
            .collect()
    }
}

impl From<SpecieRecord> for Specie {
    fn from(value: SpecieRecord) -> Self {
        let (
            genus,
            specie_code,
            common_name,
            distribution,
            specie_author,
            specie_name,
            changed_by,
            changed_date,
            record_date,
            subgenus,
            subspecie_author,
            subspecie,
            valid_specie_code,
            published,
        ) = value;

        Self {
            genus,
            specie_code,
            common_name,
            distribution,
            specie_author,
            specie_name,
            changed_by,
            changed_date: changed_date,
            record_date: record_date,
            subgenus,
            subspecie_author,
            subspecie,
            valid_specie_code,
            published
        }
    }
}