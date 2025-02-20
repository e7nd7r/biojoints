use serde::de::DeserializeOwned;

use crate::data::data_error::DataError;

use super::query::Query;

#[async_trait::async_trait]
pub trait GraphOps : Clone + Send {
    async fn fetch_all<T>(self, query: Query) -> Result<Vec<T>, DataError>
    where T: DeserializeOwned + Send;

    async fn execute_fetch<T>(self, query: Query) -> Result<T, DataError>
    where T: DeserializeOwned + Send; 

    async fn execute(self, query: Query) -> Result<(), DataError>;
}

#[derive(Clone)]
pub struct GraphLayer {
    inner: neo4rs::Graph,
}

impl GraphLayer {
    pub fn new(graph: neo4rs::Graph) -> Self {
        Self {
            inner: graph,
        }
    }
}

#[async_trait::async_trait]
impl GraphOps for GraphLayer {
   async fn fetch_all<T>(self, query: Query) -> Result<Vec<T>, DataError> 
   where T: DeserializeOwned + Send {
        let query = neo4rs::query(query.query())
            .params(query.params());

        let mut result = self.inner.execute(query).await.map_err(|err| DataError::QueryError(err.to_string()))?;

        let mut records = Vec::new();

        while let Ok(Some(row)) = result.next().await {
            let record = row.to().map_err(|err| DataError::UnexpectedResult(format!("Unexpected error {}", err)))?;

            records.push(record);
        }

        Ok(records)
   }


   async fn execute_fetch<T>(self, query: Query) -> Result<T, DataError>
   where T: DeserializeOwned + Send {
        let query = neo4rs::query(query.query())
            .params(query.params());

        let mut result = self.inner.execute(query).await.map_err(|err| DataError::QueryError(err.to_string()))?;

        let row_opt = result.next().await.map_err(|err| DataError::QueryError(err.to_string()))?; 
        let row = row_opt.ok_or(DataError::UnexpectedResult("Unexpectely return no row.".to_owned()))?;

        let result = row.to().map_err(|err| DataError::UnexpectedResult(format!("Unexpected error {}", err)))?;

        Ok(result)
   }

   async fn execute(self, query: Query) -> Result<(), DataError> {
        let query = neo4rs::query(query.query())
            .params(query.params());

        let _ = self.inner.execute(query).await.map_err(|err| DataError::QueryError(err.to_string()))?;

        Ok(())
   }
}

