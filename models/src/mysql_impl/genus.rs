use crate::{
    data::data_error::DataError,
    records::genus::Genus,
};

use super::{query::QueryBuilder, relational_layer::RelationalOps};

pub struct GenusModel<Conn> where Conn: RelationalOps {
    conn: Conn,
}

impl <Conn: RelationalOps> GenusModel<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self {
            conn,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<Genus>, DataError> {
        let query = QueryBuilder::new()
            .query("SELECT Family, Genus, Subfamily FROM _genus")
            .build();

        let records = self.conn.clone().fetch_all(query).await?;

        Ok(records)
    }
}

