use std::result;
use neo4rs::{query, Graph, Node};

use crate::{
    data::{
        crud::{Count, Create, Exists, Fetch},
        data_error::DataError, query_builder::QueryBuilder
    },
    records::kingdom::Kingdom
};

impl From<Node> for Kingdom {
    fn from(node: Node) -> Self {
        let kingdom: String = node.get("kingdom").unwrap();
        let superkingdom: String = node.get("superkingdom").unwrap();

        Self {
            kingdom: kingdom.clone(),
            superkingdom: superkingdom.clone()
        }
    }
}

impl Fetch<Graph> for Kingdom {
    async fn fetch(conn: Graph, query_builder: &dyn QueryBuilder) -> result::Result<Vec<Self>, DataError> {
        let (query_str, params) = query_builder.build();

        let query = query(&query_str).params(params);
        let mut result = conn.execute(query).await.unwrap();

        let mut kingdoms = Vec::new();

        while let Ok(Some(row)) = result.next().await {
            let node:Node = row.get("n").unwrap();
            let kingdom = Kingdom::from(node);

            kingdoms.push(kingdom);
        }

        Ok(kingdoms)
    }
}

impl Exists<Graph> for Kingdom {}

impl Count<Graph> for Kingdom {
    async fn count(&self, conn: Graph) -> result::Result<i32, DataError> {
        let query = query("MATCH (n:Kingdom {kingdom: $kingdom}) RETURN COUNT(n) as count")
            .param("kingdom", self.kingdom.clone());

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

impl Create<Graph> for Kingdom {
    async fn create(&self, conn: Graph) -> result::Result<(), DataError> {
        if self.exists(conn.clone()).await? {
            return Err(DataError::AlreadyExist(format!("{} already exists", self.kingdom)));
        }

        let query = query("
CREATE (n:Kingdom {kingdom: $kingdom, superkingdom: $superkingdom})
RETURN n
")
            .param("kingdom", self.kingdom.clone())   
            .param("superkingdom", self.superkingdom.clone());

        let mut result = conn.execute(query).await.unwrap();

        if let Ok(Some(_)) = result.next().await {
            return Ok(());
        }

        Err(DataError::NotInsertedEntity(format!("Entity was not inserted!")))
    }
}
