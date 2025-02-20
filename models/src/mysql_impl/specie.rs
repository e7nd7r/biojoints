use crate::{
    data::data_error::DataError,
    records::specie::Specie,
};

use super::{query::QueryBuilder, relational_layer::RelationalOps};

pub struct SpecieModel<Conn> where Conn: RelationalOps {
    conn: Conn,
}

impl <Conn: RelationalOps> SpecieModel<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self {
            conn,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<Specie>, DataError> {
        let query = QueryBuilder::new()
            .query("
                SELECT 
                Genus, SpeciesCode, CommonName, Distribution,
                SpeciesAuthor, SpeciesName, SppRecChangedBy, SppRecChangedDate,
                CAST(SppRecordDate AS CHAR) AS SppRecordDateStr, Subgenus, SubspAuthor, Subspecies,
                ValidSpCode, Published 
                FROM _specie
            ")
            .build();

        let records = self.conn.clone().fetch_all(query).await?;

        Ok(records)
    }
}

