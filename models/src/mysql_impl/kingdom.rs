use mysql::{prelude::Queryable, Params};

use crate::{data::{crud::Fetch, data_error::DataError, query_builder::QueryBuilder}, records::kingdom::Kingdom};

/// Using Mutex for now since parallel queries is not
/// that important for now. 
impl Fetch<mysql::Pool> for Kingdom {
    async fn fetch(conn_pool: mysql::Pool, query_builder: &dyn QueryBuilder) -> Result<Vec<Self>, DataError> {
        let mut conn = conn_pool.get_conn().expect("Error connection to db");

        let (sql, _) = query_builder.build();

        let stmt = conn.prep(sql).expect("Incorrect query");

        // Query the table and collect the results
        let results = conn.exec(&stmt, Params::Empty)
            .unwrap()
            .iter()
            .map(|val: &(String, String)| Kingdom::from(val))
            .collect();

        Ok(results)
    }
}

