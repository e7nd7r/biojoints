use std::future::Future;

use neo4rs::{query, Graph};

use crate::{
    data::{
        crud::{Count, Create, Exists},
        data_error::DataError
    },
    records::specie::Specie
};

pub trait SpecieOps {
    fn create_dist_nodes(&self, conn: Graph) -> impl Future<Output = Result<(), DataError>> + Send;
}

impl SpecieOps for Specie {
    async fn create_dist_nodes(&self, conn: Graph) -> Result<(), DataError> {
        let codes = self.dist_to_vec()?;

        for code in codes {
            let query = query("
MATCH (s: Specie) WHERE s.specie_code = $specie_code
MATCH (st: State) WHERE st.code = $code
MERGE (s)-[:LOCATED_IN]->(st)
RETURN s, st
")
                .param("specie_code", self.specie_code.clone())
                .param("code", code);

            let mut result = conn.execute(query).await.unwrap();

            match result.next().await {
                Ok(_) => return Ok(()),
                Err(err) => Err(DataError::NotInsertedEntity(format!("Entity was not inserted! | {}", err)))
            }?
        }

        Ok(())
    }
}

impl Exists<Graph> for Specie {}

impl Count<Graph> for Specie {
    async fn count(&self,  conn: Graph) -> Result<i32, crate::data::data_error::DataError> {
        let query = query("MATCH (n:Specie {specie_code: $specie_code}) RETURN COUNT(n) as count")
            .param("specie_code", self.specie_code.clone());

        let result = conn
            .execute(query)
            .await
            .unwrap()
            .next()
        .await;

        match result {
            Ok(Some(row)) => {
                let count:i32 = row.get("count").unwrap();
                Ok(count)
            },
            Err(err) => Err(DataError::QueryError(format!("${err}"))),
            _ => Err(DataError::QueryError(format!("Unexpectely return no row."))),
        }
    }
}

impl Create<Graph> for Specie {
    async fn create(&self, conn: Graph) -> Result<(), DataError> {
        if self.exists(conn.clone()).await? {
            return Err(DataError::AlreadyExist(format!("{} already exists", self.genus)));
        }

        let query = query("
MATCH (g:Genus)
WHERE g.genus = $genus
CREATE (s:Specie {
genus: $genus,
specie_code: $specie_code,
common_name: $common_name,
distribution: $distribution,
specie_author: $specie_author,
specie_name: $specie_name,
changed_by: $changed_by,
changed_date: date($changed_date),
record_date: date($record_date),
subgenus: $subgenus,
subspecie_author: $subspecie_author,
subspecie: $subspecie,
valid_specie_code: $valid_specie_code,
published: $published
})
CREATE (s)-[:BELONGS_TO]->(g)
RETURN s
")
            .param("genus", self.genus.clone())
            .param("specie_code", self.specie_code.clone())   
            .param("common_name", self.common_name.clone())
            .param("distribution", self.distribution.clone())
            .param("specie_author", self.specie_author.clone())
            .param("specie_name", self.specie_name.clone())
            .param("changed_by", self.changed_by.clone())
            .param("changed_date", self.changed_date)
            .param("record_date", self.record_date)
            .param("subgenus", self.subgenus.clone())
            .param("subspecie_author", self.subspecie_author.clone())
            .param("subspecie", self.subspecie.clone())
            .param("valid_specie_code", self.valid_specie_code.clone())
            .param("published", self.published);

        let mut result = conn.execute(query).await.unwrap();

        match result.next().await {
            Ok(_) => return Ok(()),
            Err(err) => Err(DataError::NotInsertedEntity(format!("Entity was not inserted! | {}", err)))
        }
    }
}
