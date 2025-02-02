use std::{result, sync::Arc};
use mysql::prelude::*;

use crate::{data::{crud::Fetch, data_error::DataError}, records::kingdom::Kingdom};

/// Using Mutex for now since parallel queries is not
/// that important for now. 
impl Fetch<Arc<mysql::Pool>> for Kingdom {
    fn fetch(conn_pool: Arc<mysql::Pool>) -> result::Result<Vec<Self>, DataError> {
        let conn = conn_pool.get_conn().expect("Error connection to db");

        // Query the table and collect the results
        let results = conn.unwrap()
            .query("SELECT Kingdom, Superkingdom FROM _kingdom")
            .unwrap()
            .iter()
            .map(|val: &(String, String)| Kingdom::from(val))
            .collect();

        Ok(results)
    }
}

