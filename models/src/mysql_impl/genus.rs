use std::{result, sync::Arc};
use mysql::prelude::*;

use crate::{
    data::{crud::Fetch, data_error::DataError},
    records::genus::{Genus, GenusRecord},
};

impl Fetch<Arc<mysql::Pool>> for Genus {
    fn fetch(conn_pool: Arc<mysql::Pool>) -> result::Result<Vec<Self>, DataError> {
        let conn = conn_pool.get_conn().expect("Error connection to db");

        // Query the table and collect the results
        let results = conn.unwrap()
            .query("SELECT Family, Genus, Subfamily, Tribe FROM _genus")
            .unwrap()
            .iter()
            .map(|val: &GenusRecord| Genus::from(val.clone()))
            .collect();

        Ok(results)
    }
}
