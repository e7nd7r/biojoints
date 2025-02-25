use crate::{
    data::data_error::DataError,
    records::genus::Genus
};

use super::{graph_layer::GraphOps, query::QueryBuilder};

#[derive(Clone)]
pub struct GenusModel<Conn> where Conn: GraphOps {
    conn: Conn,
}

impl <Conn: GraphOps> GenusModel<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self {
            conn,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<Genus>, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (g:Genus) RETURN g")
            .build();

        let records = self.conn.clone().fetch_all(query).await?;

        Ok(records)
    }

    pub async fn count(&self, genus: &str) -> Result<i32, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (g:Genus {genus: $genus}) RETURN COUNT(g) as count")
            .param("genus", genus)
            .build();

        let count:i32 = self.conn.clone().execute_fetch(query).await?;

        Ok(count)
    }

    pub async fn create(&self, record: Genus) -> Result<Genus, DataError> {
        let query = QueryBuilder::new()
            .query("
                MATCH (f:Family)
                WHERE f.family = $family
                CREATE (g:Genus { family: $family, genus: $genus, subfamily: $subfamily, tribe: $tribe })
                CREATE (g)-[:BELONGS_TO]->(f)
                RETURN f
            ")
            .param("family", &record.family)
            .param("genus", &record.genus)
            .param("subfamily", &record.subfamily)
            .param("tribe", &record.tribe)
            .build();

        let record: Genus = self.conn.clone().execute_fetch(query).await?;

        Ok(record)
    }
}

