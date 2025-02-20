use crate::records::phylum::Phylum;

use super::{query::QueryBuilder, relational_layer::RelationalOps};

pub struct PhylumModel<Conn> where Conn: RelationalOps {
    conn: Conn,
}

impl <Conn: RelationalOps> PhylumModel<Conn>{
    pub fn new(conn: Conn) -> Self {
        Self {
            conn,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<Phylum>, crate::data::data_error::DataError> {
        let query = QueryBuilder::new()
            .query("SELECT Kingdom, Phylum, Subkingdom FROM _phylum")
            .build();

        let records = self.conn.clone().fetch_all(query).await?;

        Ok(records)
    }
}

