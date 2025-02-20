use mysql::{prelude::{FromRow, Queryable}, Params};

use crate::data::data_error::DataError;

use super::query::Query;

#[derive(Clone)]
pub struct RelationalLayer {
    inner: mysql::Pool,
}

impl RelationalLayer {
    pub fn new(pool: mysql::Pool) -> Self {
        Self {
            inner: pool,
        }
    }
}

#[async_trait::async_trait]
pub trait RelationalOps : Send + Clone {
    async fn fetch_all<T>(&self, query: Query) -> Result<Vec<T>, DataError>
    where T: FromRow + Send;

    async fn execute(&self, query: Query) -> Result<u64, DataError>;
}

#[async_trait::async_trait]
impl RelationalOps for RelationalLayer {
    async fn fetch_all<T>(&self, query: Query) -> Result<Vec<T>, DataError>
    where T: FromRow + Send {
        let mut conn = self.inner.get_conn().map_err(|err| DataError::UnknownError(format!("Error fetching connection | {}", err)))?;

        let stmt = conn.prep(query.sql())
            .map_err(|err| DataError::QueryError(format!("Error preparing query | {}", err)))?; 

        let params = if query.params().is_empty() {
            Params::Empty
        } else {
            Params::from(query.params().into_iter().collect::<Vec<_>>())
        };

        let results = conn.exec(&stmt, params)
            .map_err(|err| {
                match err {
                    mysql::Error::MySqlError(e) => DataError::QueryError(format!("Query Execution Error | {}", e)),
                    _ => DataError::UnknownError(format!("Entity not found! | {}", err)),
                }
            })?;

        Ok(results)
    }

    async fn execute(&self, query: Query) -> Result<u64, DataError> {
        let mut conn = self.inner.get_conn()
            .map_err(|err| DataError::UnknownError(format!("Error fetching connection | {}", err)))?;

        let stmt = conn.prep(query.sql())
            .map_err(|err| DataError::QueryError(format!("Error preparing query | {}", err)))?;

        let params = if query.params().is_empty() {
            Params::Empty
        } else {
            Params::from(query.params().into_iter().collect::<Vec<_>>())
        };

        conn.exec_drop(stmt, params)
            .map_err(|err| match err {
                mysql::Error::MySqlError(e) => DataError::QueryError(format!("Query Execution Error | {}", e)),
                _ => DataError::UnknownError(format!("Error executing query | {}", err)),
            })?;

        Ok(conn.affected_rows())

    }
}
 
