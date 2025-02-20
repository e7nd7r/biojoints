use crate::{
    data::data_error::DataError,
    records::family::Family
};

use super::{graph_layer::GraphOps, query::QueryBuilder};

pub struct FamilyModel<Conn> {
    conn: Conn,
}

impl <Conn: GraphOps> FamilyModel<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self {
            conn,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<Family>, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (f:Family) RETURN f")
            .build();

        let records = self.conn.clone().fetch_all(query).await?;

        Ok(records)
    }

    pub async fn count(&self, family: &str) -> Result<i32, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (f:Family {family: $family }) RETURN COUNT(f) as count")
            .param("family", family)
            .build();

        let count:i32 = self.conn.clone().execute_fetch(query).await?;

        Ok(count)
    }

    pub async fn create(&self, record: Family) -> Result<Family, DataError> {
        let query = QueryBuilder::new()
            .query("
                MATCH (o:Order)
                WHERE o.order = $order
                CREATE (f:Family { order: $order, family: $family, suborder: $suborder, superfamily: $superfamily })
                CREATE (f)-[:BELONGS_TO]->(o)
                RETURN f
            ")
            .param("order", &record.order)
            .param("family", &record.family)
            .param("suborder", &record.suborder)
            .param("superfamily", &record.superfamily)
            .build();

        let record: Family = self.conn.clone().execute_fetch(query).await?;

        Ok(record)
    }
}

