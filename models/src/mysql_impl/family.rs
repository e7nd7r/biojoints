use std::{result, sync::Arc};
use mysql::prelude::*;

use crate::{
    data::{crud::Fetch, data_error::DataError},
    records::family::{Family, FamilyRecord},
};

impl Fetch<Arc<mysql::Pool>> for Family {
    fn fetch(conn_pool: Arc<mysql::Pool>) -> result::Result<Vec<Self>, DataError> {
        let conn = conn_pool.get_conn().expect("Error connection to db");

        // Query the table and collect the results
        let results = conn.unwrap()
            .query("SELECT _Order, Family, Suborder, Superfamily FROM _family")
            .unwrap()
            .iter()
            .map(|val: &FamilyRecord| Family::from(val.clone()))
            .collect();

        Ok(results)
    }
}
