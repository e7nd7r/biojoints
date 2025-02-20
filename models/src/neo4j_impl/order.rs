use crate::{
    data::data_error::DataError,
    records::order::Order
};

use super::{graph_layer::GraphOps, query::QueryBuilder};

pub struct OrderModel<Conn> where Conn: GraphOps {
    conn: Conn,
}

impl <Conn: GraphOps> OrderModel<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self {
            conn,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<Order>, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (o:Order) RETURN o")
            .build();

        let records = self.conn.clone().fetch_all(query).await?;

        Ok(records)
    }

    pub async fn count(&self, order: &str) -> Result<i32, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (o:Order {order: $order }) RETURN COUNT(o) as count")
            .param("order", order)
            .build();

        let count:i32 = self.conn.clone().execute_fetch(query).await?;

        Ok(count)
    }

    pub async fn create(&self, record: Order) -> Result<Order, DataError> {
        let query = QueryBuilder::new()
            .query("
                MATCH (c:Class)
                WHERE c.class = $class
                CREATE (o:Order { class: $class, order: $order, subclass: $subclass, superorder: $superorder })
                CREATE (o)-[:BELONGS_TO]->(c)
                RETURN o
            ")
            .param("class", &record.class)
            .param("order", &record.order)
            .param("subclass", &record.subclass)
            .param("superorder", &record.superorder)
            .build();

        let record: Order = self.conn.clone().execute_fetch(query).await?;

        Ok(record)
    }
}

