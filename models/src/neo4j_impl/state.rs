use std::result;
use neo4rs::{query, Graph};

use crate::{
    data::{crud::{Count, Create, Exists},
    data_error::DataError},
    records::state::State
};

impl Exists<Graph> for State {}

impl Count<Graph> for State {
    async fn count(&self, conn: Graph) -> result::Result<i32, DataError> {
        let query = query("MATCH (n:State {code: $code}) RETURN COUNT(n) as count")
            .param("code", self.code.clone());
        
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

impl Create<Graph> for State {
    async fn create(&self, conn: Graph) -> result::Result<(), DataError> {
        if self.exists(conn.clone()).await? {
            return Err(DataError::AlreadyExist(format!("{} already exists", self.name)));
        }

        let query = query("
            MATCH (c: Country)
            WHERE c.code = $country_code
            CREATE (s:State {country_code: $country_code, name: $name, code: $code})
            CREATE (s)-[:BELONGS_TO]->(c)
            RETURN s
        ")
            .param("country_code", self.country_code.clone())
            .param("name", self.name.clone())   
            .param("code", self.code.clone());

        let mut result = conn.execute(query).await.unwrap();
        
        match result.next().await {
            Ok(_) => return Ok(()),
            Err(err) => Err(DataError::NotInsertedEntity(format!("Entity was not inserted! | {}", err)))
        }
    }
}
