use std::result;
use uuid::Uuid;

use crate::{
    data::data_error::DataError,
    records::kingdom::Kingdom
};

use super::{graph_layer::GraphOps, query::QueryBuilder};

pub struct KingdomModel<Conn> where Conn: GraphOps {
    conn: Conn,
}

impl <Conn: GraphOps> KingdomModel<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self {
            conn
        }
    }

    pub async fn fetch(&self) -> result::Result<Vec<Kingdom>, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (n:Kingdom) RETURN n")
            .build();

        let result:Vec<Kingdom> = self.conn.clone().fetch_all(query).await?;

        Ok(result)
    }

    pub async fn count(&self, kingdom: &str) -> result::Result<i32, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (n:Kingdom {kingdom: $kingdom}) RETURN COUNT(n) as count")
            .param("kingdom", kingdom)
            .build();

        let result: i32 = self.conn.clone()
            .fetch_all(query).await?
            .first()
            .cloned()
            .unwrap();

        Ok(result)
    }

    pub async fn create(&self, record: Kingdom) -> result::Result<Kingdom, DataError> {
        if self.count(&record.kingdom).await? > 0 {
            return Err(DataError::AlreadyExist(format!("{} already exists", record.kingdom)));
        }

        let id = Uuid::new_v4();

        let query = QueryBuilder::new()
            .query("
                CREATE (n:Kingdom {id: $id, kingdom: $kingdom, superkingdom: $superkingdom})
                RETURN n
            ")
            .param("id", &id.to_string())
            .param("kingdom", &record.kingdom)
            .param("superkingdom", &record.superkingdom)
            .build();

        let kingdom = self.conn.clone().execute_fetch(query).await?;

        Ok(kingdom)

    }
}

