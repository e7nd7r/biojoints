use crate::{
    data::data_error::DataError,
    records::phylum::Phylum
};

use super::{graph_layer::GraphOps, query::QueryBuilder};

#[derive(Clone)]
pub struct PhylumModel<Conn> where Conn: GraphOps {
    conn: Conn,
}

impl <Conn: GraphOps> PhylumModel<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self {
            conn,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<Phylum>, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (p:Phylum) RETURN p")
            .build();

        let records = self.conn.clone().fetch_all(query).await?;

        Ok(records)
    }

    pub async fn count(&self, phylum: &str) -> Result<i32, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (p:Phylum {phylym: $phylum}) RETURN COUNT(p) as count")
            .param("phylum", phylum)
            .build();

        let count:i32 = self.conn.clone().execute_fetch(query).await?;

        Ok(count)
    }

    pub async fn create(&self, record: Phylum) -> Result<Phylum, DataError> {
        if self.count(&record.phylum).await? > 0 {
            return Err(DataError::AlreadyExist(format!("{} already exists", record.phylum)));
        }

        let query = QueryBuilder::new()
            .query("
                MATCH (k:Kingdom)
                WHERE k.kingdom = $kingdom
                CREATE (p:Phylum { kingdom: $kingdom, phylum: $phylum, subkingdom: $subkingdom })
                CREATE (p)-[:BELONGS_TO]->(k)
                RETURN p
            ")
            .param("kingdom", &record.kingdom)
            .param("phylum", &record.phylum)
            .param("subkingdom", &record.subkingdom)
            .build();

        let record: Phylum = self.conn.clone().execute_fetch(query).await?;

        Ok(record)
    }

}

