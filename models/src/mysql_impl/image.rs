use mysql::{prelude::Queryable, Value};

use crate::{
    data::{crud::Fetch, data_error::DataError, query_builder::QueryBuilder},
    records::image::{Image, ImageRecord},
};

impl Fetch<mysql::Pool> for Image {
    async fn fetch(conn_pool: mysql::Pool, query_builder: &dyn QueryBuilder) -> Result<Vec<Self>, DataError> {
        let mut conn = conn_pool.get_conn().expect("Error connection to db");

        // Query the table and collect the results
        let (sql, raw_params) = query_builder.build();
        let mysql_params: Vec<(String, Value)> = raw_params.into_iter().map(|(k, v)| (k, Value::from(v))).collect();

        let results = conn.exec(sql, mysql_params)
            .unwrap()
            .iter()
            .map(|row: &ImageRecord| Image::from(row.clone()))
            .collect();

        Ok(results)
    }
}

