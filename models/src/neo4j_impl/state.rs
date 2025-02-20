use std::result;

use crate::{
    data::data_error::DataError,
    records::state::State
};

use super::{graph_layer::GraphOps, query::QueryBuilder};

pub struct StateModel<Conn> where Conn: GraphOps {
    conn: Conn,
}

impl <Conn: GraphOps> StateModel<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self {
            conn,
        }
    }

    pub async fn fetch(&self) -> result::Result<Vec<State>, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (s:State) RETURN s")
            .build();

        let records = self.conn.clone().fetch_all(query).await?;

        Ok(records)
    }

    pub async fn count(&self, state: &str) -> result::Result<i32, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (s:State {state: $state }) RETURN COUNT(s) as count")
            .param("state", state)
            .build();

        let count:i32 = self.conn.clone().execute_fetch(query).await?;

        Ok(count)
    }

    pub async fn create(&self, record: State) -> result::Result<State, DataError> {
        if self.count(&record.name).await? > 0 {
            return Err(DataError::AlreadyExist(format!("{} already exists", record.name)));
        }

        let query = QueryBuilder::new()
            .query("
                MATCH (c:Country)
                WHERE c.code = $country_code
                CREATE (s:State { country_code: $country_code, name: $name, code: $code })
                CREATE (s)-[:BELONGS_TO]->(c)
                RETURN s
            ")
            .param("country_code", &record.country_code)
            .param("name", &record.name)
            .param("code", &record.code)
            .build();

        let record = self.conn.clone().execute_fetch(query).await?;

        Ok(record)
    }

}

