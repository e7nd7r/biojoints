use crate::{
    data::data_error::DataError,
    records::family::Family,
};

use super::{query::QueryBuilder, relational_layer::RelationalOps};

pub struct FamilyModel<Conn> {
    conn: Conn,
}

impl <Conn: RelationalOps> FamilyModel<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self {
            conn,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<Family>, DataError> {
        let query = QueryBuilder::new()
            .query("SELECT _Order, Family, Suborder, Superfamily FROM _family")
            .build();

        let records = self.conn.clone().fetch_all(query).await?;

        Ok(records)
    }
}

