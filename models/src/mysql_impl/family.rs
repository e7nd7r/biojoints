use std::future::Future;

use mysql::{prelude::*, Value};

use crate::{
    data::{crud::Fetch, data_error::DataError, query_builder::QueryBuilder},
    records::family::{Family, FamilyRecord},
};

impl Fetch<mysql::Pool> for Family {
    async fn fetch(conn_pool: mysql::Pool, query_builder: &dyn QueryBuilder) -> Result<Vec<Self>, DataError> {
        let mut conn = conn_pool.get_conn().expect("Error connection to db");

        let (sql, raw_params) = query_builder.build();

        let mysql_params: Vec<(String, Value)>  = raw_params.into_iter()
            .map(|(k, v)| (k.to_owned(), Value::from(v)))
            .collect();

        let results = conn.exec(sql, mysql_params)
            .unwrap()
            .iter()
            .map(|val: &FamilyRecord| Family::from(val.clone()))
            .collect();

        Ok(results)
    }
}

