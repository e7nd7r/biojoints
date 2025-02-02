use std::{result, sync::Arc};
use mysql::prelude::*;

use crate::{
    data::{crud::Fetch, data_error::DataError},
    records::order::{Order, OrderRecord},
};

impl Fetch<Arc<mysql::Pool>> for Order {
    fn fetch(conn_pool: Arc<mysql::Pool>) -> result::Result<Vec<Self>, DataError> {
        let conn = conn_pool.get_conn().expect("Error connection to db");

        // Query the table and collect the results
        let results = conn.unwrap()
            .query("SELECT _Class, _Order, SubClass, Superorder FROM _order")
            .unwrap()
            .iter()
            .map(|val: &OrderRecord| Order::from(val.clone()))
            .collect();

        Ok(results)
    }
}
