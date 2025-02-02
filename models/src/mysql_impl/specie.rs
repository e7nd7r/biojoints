use std::{result, sync::Arc};
use chrono::NaiveDate;
use mysql::*;
use mysql::prelude::*;

use crate::{
    data::{crud::Fetch, data_error::DataError},
    records::specie::Specie,
};

impl From<Row> for Specie {
    fn from(row: Row) -> Self {
        let changed_date_opt: Option<Option<String>> = row.get("SppRecChangedDate");
        let changed_date_str = changed_date_opt.unwrap().unwrap_or("1970-01-01".to_owned());
        let changed_date = NaiveDate::parse_from_str(&changed_date_str, "%Y-%m-%d").unwrap_or_default();

        let record_date_opt: Option<Option<String>> = row.get("SppRecordDateStr");
        let record_date_str = record_date_opt.unwrap().unwrap_or("1970-01-01".to_owned());
        let record_date = NaiveDate::parse_from_str(&record_date_str, "%Y-%m-%d").unwrap_or_default();

        Self {
            genus: row.get("Genus").unwrap_or_default(),
            specie_code: row.get("SpecieCode").unwrap_or_default(),
            common_name: row.get("CommonName").unwrap_or_default(),
            distribution: row.get("Distribution").unwrap_or_default(),
            specie_author: row.get("SpecieAuthor").unwrap_or_default(),
            specie_name: row.get("SpeciesName").unwrap_or_default(),
            changed_by: row.get("SppRecChangedBy").unwrap_or_default(),
            changed_date,
            record_date,
            subgenus: row.get("Subgenus").unwrap_or_default(),
            subspecie_author: row.get("SubspAuthor").unwrap_or_default(),
            subspecie: row.get("Subspecies").unwrap_or_default(),
            valid_specie_code: row.get("ValidSpCode").unwrap_or_default(),
            published: row.get("Published").unwrap_or_default(),
        }
    }
}

impl Fetch<Arc<mysql::Pool>> for Specie {
    fn fetch(conn_pool: Arc<mysql::Pool>) -> result::Result<Vec<Self>, DataError> {
        let conn = conn_pool.get_conn().expect("Error connection to db");

        // Query the table and collect the results
        let results:Vec<Row> = conn.unwrap()
            .query("
                SELECT 
                    Genus, SpeciesCode, CommonName, Distribution,
                    SpeciesAuthor, SpeciesName, SppRecChangedBy, SppRecChangedDate,
                    CAST(SppRecordDate AS CHAR) AS SppRecordDateStr, Subgenus, SubspAuthor, Subspecies,
                    ValidSpCode, Published 
                FROM _specie
            ")
            .unwrap();

        let results = results
                .iter()
                .map(|row| Specie::from(row.clone()))
                .collect();

        Ok(results)
    }
}
