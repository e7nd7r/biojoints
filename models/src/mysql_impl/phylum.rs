use std::sync::Arc;
use mysql::prelude::*;

use crate::{data::crud::Fetch, records::phylum::{Phylum, PhylumRecord}};

impl Fetch<Arc<mysql::Pool>> for Phylum {
    fn fetch(conn_pool: Arc<mysql::Pool>) -> Result<Vec<Self>, crate::data::data_error::DataError> {
        let conn = conn_pool.get_conn().expect("Error connection to db");

        let results = conn.unwrap()
            .query("SELECT Kingdom, Phylum, Subkingdom FROM _phylum")
            .unwrap()
            .iter()
            .map(|val: &PhylumRecord| Phylum::from(val))
            .collect();

        Ok(results)
    }
}
