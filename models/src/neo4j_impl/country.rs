use std::result;
use neo4rs::{query, Graph, Node};

use crate::{
    data::{crud::{Count, Create, Exists},
    data_error::DataError},
    records::country::Country
};

impl From<Node> for Country {
    fn from(node: Node) -> Self {
        let name: String = node.get("name").unwrap();
        let code: String = node.get("code").unwrap();

        Self {
            name: name.clone(),
            code: code.clone()
        }
    }
}

impl Exists<Graph> for Country {}

impl Count<Graph> for Country {
    async fn count(&self, conn: Graph) -> result::Result<i32, DataError> {
        let query = query("MATCH (n:Country {name: $name}) RETURN COUNT(n) as count")
            .param("name", self.name.clone());

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

impl Create<Graph> for Country {
    async fn create(&self, conn: Graph) -> result::Result<Country, DataError> {
        if self.exists(conn.clone()).await? {
            return Err(DataError::AlreadyExist(format!("{} already exists", self.name)));
        }

        let query = query("
            CREATE (n:Country {name: $name, code: $code})
            RETURN n
        ")
            .param("name", self.name.clone())   
            .param("code", self.code.clone());

        let mut result = conn.execute(query).await.unwrap();

        match result.next().await {
            Ok(Some(row)) => {
                let node: Node = row.get("n").unwrap();

                Ok(Country::from(node))
            },
            Err(err) => Err(DataError::NotInsertedEntity(format!("Entity was not inserted! | {}", err))),
            _ => Err(DataError::NotInsertedEntity("Entity was not inserted!".to_string()))
        }
    }
}
