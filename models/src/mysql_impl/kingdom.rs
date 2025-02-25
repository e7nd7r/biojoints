use crate::{data::data_error::DataError, records::kingdom::Kingdom};
use super::{query::QueryBuilder, relational_layer::RelationalOps};

#[derive(Clone)]
pub struct KingdomModel<Conn> where Conn: RelationalOps {
    conn: Conn,
}

impl<Conn: RelationalOps> KingdomModel<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }

    pub async fn fetch(&self) -> Result<Vec<Kingdom>, DataError> {
        let query = QueryBuilder::new()
            .query("SELECT Kingdom, Subkingdom FROM _kingdom")
            .build();

        let records = self.conn.clone().fetch_all(query).await?;

        Ok(records)
    }
}

