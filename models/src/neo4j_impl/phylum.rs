use neo4rs::{query, Graph};

use crate::{
    data::{
        crud::{Count, Create, Exists},
        data_error::DataError
    },
    records::phylum::Phylum
};

impl Exists<Graph> for Phylum {}

impl Count<Graph> for Phylum {
    async fn count(&self,  conn: Graph) -> Result<i32, crate::data::data_error::DataError> {
        let query = query("MATCH (n:Phylum {phylum: $phylum}) RETURN COUNT(n) as count")
            .param("phylum", self.phylum.clone());
        
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

impl Create<Graph> for Phylum {
    async fn create(&self, conn: Graph) -> Result<(), DataError> {
        if self.exists(conn.clone()).await? {
            return Err(DataError::AlreadyExist(format!("{} already exists", self.phylum)));
        }
        
        let query = query("
            MATCH (k:Kingdom)
            WHERE k.kingdom = $kingdom
            CREATE (p:Phylum { kingdom: $kingdom, phylum: $phylum, subkingdom: $subkingdom })
            CREATE (p)-[:BELONGS_TO]->(k)
            RETURN p
        ")
            .param("kingdom", self.kingdom.clone())
            .param("phylum", self.phylum.clone())   
            .param("subkingdom", self.subkingdom.clone());

        let mut result = conn.execute(query).await.unwrap();
    
        if let Ok(Some(_)) = result.next().await {
            return Ok(());
        }

        Err(DataError::NotInsertedEntity(format!("Entity was not inserted!")))
    }
}
