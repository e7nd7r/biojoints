use crate::{
    data::data_error::DataError,
    records::specie::Specie
};

use super::{graph_layer::GraphOps, query::QueryBuilder};

pub struct SpecieModel<Conn> where Conn: GraphOps {
    conn: Conn,
}

impl <Conn: GraphOps> SpecieModel<Conn> {
    pub fn new(conn: Conn) -> Self {
        Self {
            conn,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<Specie>, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (s:Specie) RETURN s")
            .build();

        let records = self.conn.clone().fetch_all(query).await?;

        Ok(records)
    }

    pub async fn count(&self, specie_code: &str) -> Result<i32, DataError> {
        let query = QueryBuilder::new()
            .query("MATCH (s:Specie) RETURN COUNT(s) as count")
            .param("specie_code", specie_code)
            .build();

        let count:i32 = self.conn.clone().execute_fetch(query).await?;

        Ok(count)
    }

    pub async fn create_dist_nodes(&self, specie: Specie) -> Result<(), DataError> {
        let distribution = specie.parse_distribution()?;

        for code in distribution {
            let query = QueryBuilder::new()
                .query("
                    MATCH (s: Specie) WHERE s.specie_code = $specie_code
                    MATCH (st: State) WHERE st.code = $code
                    MERGE (s)-[:LOCATED_IN]->(st)
                    RETURN s, st
                ")
                .param("specie_code", &specie.specie_code)
                .param("code", &code)
                .build();

            self.conn.clone().execute(query).await
                .map_err(|err| DataError::NotInsertedEntity(format!("Entity was not inserted: {}", err)))?;
        }

        Ok(())
    }

    pub async fn create(&self, record: Specie) -> Result<Specie, DataError> {
        let query = QueryBuilder::new()
            .query("
                MATCH (g:Genus)
                WHERE g.genus = $genus
                CREATE (s:Specie { genus: $genus, name: $specie, subgenus: $subgenus, specie_code: $specie_code })
                CREATE (s)-[:BELONGS_TO]->(g)
                RETURN s
            ")
            .param("genus", &record.genus)
            .param("specie", &record.specie_name)
            .param("subgenus", &record.subgenus)
            .param("specie_code", &record.specie_code)
            .build();

        let record: Specie = self.conn.clone().execute_fetch(query).await?;

        Ok(record)
    }
}

