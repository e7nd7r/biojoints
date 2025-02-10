use chrono::NaiveDate;
use mysql::*;
use mysql::prelude::*;

use crate::{
    data::{crud::Fetch, data_error::DataError, query_builder::QueryBuilder},
    records::specie::Specie,
};

impl FromRow for Specie {
    fn from_row_opt(row: Row) -> std::result::Result<Self, FromRowError>
    where Self: Sized {
        let changed_date_opt: Option<Option<String>> = row.get("SppRecChangedDate");
        let changed_date_str = changed_date_opt.unwrap().unwrap_or("1970-01-01".to_owned());
        let changed_date = NaiveDate::parse_from_str(&changed_date_str, "%Y-%m-%d").unwrap_or_default();

        let record_date_opt: Option<Option<String>> = row.get("SppRecordDateStr");
        let record_date_str = record_date_opt.unwrap().unwrap_or("1970-01-01".to_owned());
        let record_date = NaiveDate::parse_from_str(&record_date_str, "%Y-%m-%d").unwrap_or_default();

        Ok(Self {
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
        })
    }
}

impl Fetch<mysql::Pool> for Specie {
    async fn fetch(conn_pool: mysql::Pool, query_builder: &dyn QueryBuilder) -> Result<Vec<Self>, DataError> {
        let mut conn = conn_pool.get_conn().expect("Error connection to db");

        // Query the table and collect the results
        let (sql, raw_params) = query_builder.build();

        let mysql_params: Vec<(String, Value)>  = raw_params.into_iter()
            .map(|(k, v)| (k.to_owned(), Value::from(v)))
            .collect();

        let results: Vec<Specie> = conn.exec(sql, mysql_params)
            .unwrap();

        Ok(results)
    }
}

