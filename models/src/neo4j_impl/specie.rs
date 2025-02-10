use std::future::Future;

use chrono::NaiveDate;
use neo4rs::{query, Graph, Node};

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

impl From<Node> for Specie {
    fn from(node: Node) -> Self {
        let genus: String = node.get("genus").unwrap();
        let specie_code: String = node.get("specie_code").unwrap();
        let common_name: String = node.get("common_name").unwrap();
        let distribution: String = node.get("distribution").unwrap();
        let specie_author: String = node.get("specie_author").unwrap();
        let specie_name: String = node.get("specie_name").unwrap();
        let changed_by: String = node.get("changed_by").unwrap();
        let changed_date: String = node.get("changed_date").unwrap();
        let record_date: String = node.get("record_date").unwrap();
        let subgenus: String = node.get("subgenus").unwrap();
        let subspecie_author: String = node.get("subspecie_author").unwrap();
        let subspecie: String = node.get("subspecie").unwrap();
        let valid_specie_code: String = node.get("valid_specie_code").unwrap();
        let published: bool = node.get("published").unwrap();

        Self {
            genus: genus.clone(),
            specie_code: specie_code.clone(),
            common_name: common_name.clone(),
            distribution: distribution.clone(),
            specie_author: specie_author.clone(),
            specie_name: specie_name.clone(),
            changed_by: changed_by.clone(),
            changed_date: NaiveDate::parse_from_str(&changed_date, "%Y-%m-%d").unwrap(),
            record_date: NaiveDate::parse_from_str(&record_date, "%Y-%m-%d").unwrap(),
            subgenus: subgenus.clone(),
            subspecie_author: subspecie_author.clone(),
            subspecie: subspecie.clone(),
            valid_specie_code: valid_specie_code.clone(),
            published: published.clone()
        }
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
            _ => Err(DataError::QueryError("Unexpectely return no row.".to_string())),
        }
    }
}

impl Create<Graph> for Specie {
    async fn create(&self, conn: Graph) -> Result<Specie, DataError> {
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
            Ok(Some(row)) => {
                let node: Node = row.get("s").unwrap();

                Ok(Specie::from(node))
            },
            Err(err) => Err(DataError::NotInsertedEntity(format!("Entity was not inserted! | {}", err))),
            _ => Err(DataError::NotInsertedEntity("Entity was not inserted!".to_string()))
        }
    }
}

