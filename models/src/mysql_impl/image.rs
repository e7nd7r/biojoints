use std::{result, sync::Arc};
use mysql::prelude::*;

use crate::{
    data::{crud::Fetch, data_error::DataError},
    records::image::{Image, ImageRecord},
};

impl Fetch<Arc<mysql::Pool>> for Image {
    fn fetch(conn_pool: Arc<mysql::Pool>) -> result::Result<Vec<Self>, DataError> {
        let conn = conn_pool.get_conn().expect("Error connection to db");

        // Query the table and collect the results
        let results = conn.unwrap()
            .query("
                SELECT 
                    _file, notes, SpeciesCode, author_id, ValidSpCode
                FROM _Image
            ")
            .unwrap()
            .iter()
            .map(|row: &ImageRecord| Image::from(row.clone()))
            .collect();

        Ok(results)
    }
}
