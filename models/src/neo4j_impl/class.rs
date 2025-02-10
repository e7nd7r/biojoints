use neo4rs::{query, Graph, Node};

use crate::{
    data::{
        crud::{Count, Create, Exists},
        data_error::DataError
    },
    records::class::Class
};

impl From<Node> for Class {
    fn from(node: Node) -> Self {
        let class: String = node.get("class").unwrap();
        let phylum: String = node.get("phylum").unwrap();
        let subphylum: String = node.get("subphylum").unwrap();

        Self {
            class: class.clone(),
            phylum: phylum.clone(),
            subphylum: subphylum.clone()
        }
    }
}

impl Exists<Graph> for Class {}

impl Count<Graph> for Class {
    async fn count(&self,  conn: Graph) -> Result<i32, crate::data::data_error::DataError> {
        let query = query("MATCH (n:Class {class: $class}) RETURN COUNT(n) as count")
            .param("class", self.class.clone());

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

impl Create<Graph> for Class {
    async fn create(&self, conn: Graph) -> Result<Class, DataError> {
        if self.exists(conn.clone()).await? {
            return Err(DataError::AlreadyExist(format!("{} already exists", self.phylum)));
        }

        let query = query("
            MATCH (p:Phylum)
            WHERE p.phylum = $phylum
            CREATE (c:Class { phylum: $phylum, class: $class, subphylum: $subphylum })
            CREATE (c)-[:BELONGS_TO]->(p)
            RETURN c
        ")
            .param("phylum", self.phylum.clone())
            .param("class", self.class.clone())   
            .param("subphylum", self.subphylum.clone());

        let mut result = conn.execute(query).await.unwrap();

        if let Ok(Some(row)) = result.next().await {
            let node:Node = row.get("c").unwrap(); 
            return Ok(Class::from(node));
        }

        Err(DataError::NotInsertedEntity(format!("Entity was not inserted!")))
    }
}
