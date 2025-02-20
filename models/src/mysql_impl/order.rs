use crate::{
    data::data_error::DataError,
    records::order::Order,
};

use super::{query::QueryBuilder, relational_layer::RelationalOps};

pub struct OrderModel<Conn> where Conn: RelationalOps {
    conn: Conn,
}

impl <Conn: RelationalOps> OrderModel<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self {
            conn,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<Order>, DataError> {
        let query = QueryBuilder::new()
            .query("SELECT _Class, _Order, SubClass, Superorder FROM _order")
            .build();

        let records = self.conn.clone().fetch_all(query).await?;

        Ok(records)
    }
}

