use crate::{
    data::data_error::DataError,
    records::class::Class
};

use super::{graph_layer::GraphOps, query::QueryBuilder};

#[derive(Clone)]
pub struct ClassModel<Conn> where Conn: GraphOps {
    conn: Conn,
}

impl <Conn: GraphOps> ClassModel<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self {
            conn,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<Class>, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (c:Class) RETURN c")
            .build();

        let records = self.conn.clone().fetch_all(query).await?;

        Ok(records)
    }

    pub async fn count(&self) -> Result<i32, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (c:Class) RETURN COUNT(c) as count")
            .build();

        let count:i32 = self.conn.clone().execute_fetch(query).await?;

        Ok(count)
    }

    pub async fn create(&self, record: Class) -> Result<Class, DataError> {
        let query = QueryBuilder::new()
            .query("
                MATCH (p:Phylum)
                WHERE p.phylum = $phylum
                CREATE (c:Class { phylum: $phylum, class: $class, subphylum: $subphylum })
                CREATE (c)-[:BELONGS_TO]->(p)
                RETURN c
            ")
            .param("phylum", &record.phylum)
            .param("class", &record.class)
            .param("subphylum", &record.subphylum)
            .build();

        let record = self.conn.clone().execute_fetch(query).await?;

        Ok(record)
    }
}

