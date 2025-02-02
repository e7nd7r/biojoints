use std::{result, sync::Arc};
use mysql::prelude::*;

use crate::{
    data::{crud::Fetch, data_error::DataError},
    records::class::{Class, ClassRecord},
};

impl Fetch<Arc<mysql::Pool>> for Class {
    fn fetch(conn_pool: Arc<mysql::Pool>) -> result::Result<Vec<Self>, DataError> {
        let conn = conn_pool.get_conn().expect("Error connection to db");

        // Query the table and collect the results
        let results = conn.unwrap()
            .query("SELECT Phylum, _Class, Subphylum FROM _class")
            .unwrap()
            .iter()
            .map(|val: &ClassRecord| Class::from(val.clone()))
            .collect();

        Ok(results)
    }
}
