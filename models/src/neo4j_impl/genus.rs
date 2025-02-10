use neo4rs::{query, Graph, Node};

use crate::{
    data::{
        crud::{Count, Create, Exists},
        data_error::DataError
    },
    records::genus::Genus
};

impl From<Node> for Genus {
    fn from(node: Node) -> Self {
        let family: String = node.get("family").unwrap();
        let genus: String = node.get("genus").unwrap();
        let subfamily: String = node.get("subfamily").unwrap();
        let tribe: String = node.get("tribe").unwrap();

        Self {
            family: family.clone(),
            genus: genus.clone(),
            subfamily: subfamily.clone(),
            tribe: tribe.clone()
        }
    }
}

impl Exists<Graph> for Genus {}

impl Count<Graph> for Genus {
    async fn count(&self,  conn: Graph) -> Result<i32, crate::data::data_error::DataError> {
        let query = query("MATCH (n:Genus {genus: $genus}) RETURN COUNT(n) as count")
            .param("genus", self.genus.clone());

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

impl Create<Graph> for Genus {
    async fn create(&self, conn: Graph) -> Result<Genus, DataError> {
        if self.exists(conn.clone()).await? {
            return Err(DataError::AlreadyExist(format!("{} already exists", self.genus)));
        }

        let query = query("
            MATCH (f:Family)
            WHERE f.family = $family
            CREATE (g:Genus { family: $family, genus: $genus, subfamily: $subfamily, tribe: $tribe })
            CREATE (g)-[:BELONGS_TO]->(f)
            RETURN f
        ")
            .param("family", self.family.clone())
            .param("genus", self.genus.clone())   
            .param("subfamily", self.subfamily.clone())
            .param("tribe", self.tribe.clone());

        let mut result = conn.execute(query).await.unwrap();

        if let Ok(Some(row)) = result.next().await {
            let node:Node = row.get("g").unwrap();

            return Ok(Genus::from(node));
        }

        Err(DataError::NotInsertedEntity(format!("Entity was not inserted!")))
    }
}

