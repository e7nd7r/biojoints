use neo4rs::{query, Graph, Node};

use crate::{
    data::{
        crud::{Count, Create, Exists},
        data_error::DataError
    },
    records::family::Family
};

impl From<Node> for Family {
    fn from(node: Node) -> Self {
        let order: String = node.get("order").unwrap();
        let family: String = node.get("family").unwrap();
        let suborder: String = node.get("suborder").unwrap();
        let superfamily: String = node.get("superfamily").unwrap();

        Self {
            order: order.clone(),
            family: family.clone(),
            suborder: suborder.clone(),
            superfamily: superfamily.clone()
        }
    }
}

impl Exists<Graph> for Family {}

impl Count<Graph> for Family {
    async fn count(&self,  conn: Graph) -> Result<i32, crate::data::data_error::DataError> {
        let query = query("MATCH (n:Family {family: $family}) RETURN COUNT(n) as count")
            .param("family", self.family.clone());

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

impl Create<Graph> for Family {
    async fn create(&self, conn: Graph) -> Result<Family, DataError> {
        if self.exists(conn.clone()).await? {
            return Err(DataError::AlreadyExist(format!("{} already exists", self.order)));
        }

        let query = query("
            MATCH (o:Order)
            WHERE o.order = $order
            CREATE (f:Family { order: $order, family: $family, suborder: $suborder, superfamily: $superfamily })
            CREATE (f)-[:BELONGS_TO]->(o)
            RETURN f
        ")
            .param("order", self.order.clone())
            .param("family", self.family.clone())   
            .param("suborder", self.suborder.clone())
            .param("superfamily", self.superfamily.clone());

        let mut result = conn.execute(query).await.unwrap();

        if let Ok(Some(row)) = result.next().await {
            let node: Node = row.get("f").unwrap();

            return Ok(Family::from(node));
        }

        Err(DataError::NotInsertedEntity(format!("Entity was not inserted!")))
    }
}
