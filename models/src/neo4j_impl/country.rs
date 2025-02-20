use crate::{
    data::data_error::DataError,
    records::country::Country
};

use super::{graph_layer::GraphOps, query::QueryBuilder};

pub struct CountryModel<Conn> {
    conn: Conn,
}

impl <Conn: GraphOps> CountryModel<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self {
            conn,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<Country>, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (n:Country) RETURN n")
            .build();

        let records = self.conn.clone().fetch_all(query).await?;

        Ok(records)
    }

    pub async fn count(&self, name: &str) -> Result<i32, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (n:Country {name: $name }) RETURN COUNT(n) as count")
            .param("name", name)
            .build();

        let count:i32 = self.conn.clone().execute_fetch(query).await?;

        Ok(count)
    }

    pub async fn create(&self, record: Country) -> Result<Country, DataError> {
        let query = QueryBuilder::new()
            .query("
                CREATE (n:Country {name: $name, code: $code})
                RETURN n
            ")
            .param("name", &record.name)
            .param("code", &record.code)
            .build();

        let record: Country = self.conn.clone().execute_fetch(query).await?;

        Ok(record)
    }
}

