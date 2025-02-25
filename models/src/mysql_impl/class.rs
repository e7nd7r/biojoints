use crate::{data::data_error::DataError, records::class::Class};
use super::{query::QueryBuilder, relational_layer::RelationalOps};

#[derive(Clone)]
pub struct ClassModel<Conn> where Conn: RelationalOps {
    conn: Conn,
}

impl <Conn: RelationalOps> ClassModel<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self {
            conn,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<Class>, DataError> {
        let query = QueryBuilder::new()
            .query("SELECT _Class, Class, Subclass, Superclass FROM _class")
            .build();

        let records = self.conn.clone().fetch_all(query).await?;

        Ok(records)
    }
}

