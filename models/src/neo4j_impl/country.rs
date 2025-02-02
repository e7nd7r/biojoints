use std::result;
use neo4rs::{query, Graph};

use crate::{
    data::{crud::{Count, Create, Exists},
    data_error::DataError},
    records::country::Country
};

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
            _ => Err(DataError::QueryError(format!("Unexpectely return no row."))),
        }    
    }
}

impl Create<Graph> for Country {
    async fn create(&self, conn: Graph) -> result::Result<(), DataError> {
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
            Ok(_) => return Ok(()),
            Err(err) => Err(DataError::NotInsertedEntity(format!("Entity was not inserted! | {}", err)))
        }
    }
}
