use neo4rs::{query, Graph, Node};

use crate::{
    data::{
        crud::{Count, Create, Exists},
        data_error::DataError
    },
    records::order::Order
};

impl From<Node> for Order {
    fn from(node: Node) -> Self {
        let class: String = node.get("class").unwrap();
        let order: String = node.get("order").unwrap();
        let subclass: String = node.get("subclass").unwrap();
        let superorder: String = node.get("superorder").unwrap();

        Self {
            class: class.clone(),
            order: order.clone(),
            subclass: subclass.clone(),
            superorder: superorder.clone()
        }
    }
}

impl Exists<Graph> for Order {}

impl Count<Graph> for Order {
    async fn count(&self,  conn: Graph) -> Result<i32, crate::data::data_error::DataError> {
        let query = query("MATCH (n:Order {order: $order}) RETURN COUNT(n) as count")
            .param("order", self.order.clone());

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

impl Create<Graph> for Order {
    async fn create(&self, conn: Graph) -> Result<Order, DataError> {
        if self.exists(conn.clone()).await? {
            return Err(DataError::AlreadyExist(format!("{} already exists", self.order)));
        }

        let query = query("
            MATCH (c:Class)
            WHERE c.class = $class
            CREATE (o:Order { class: $class, order: $order, subclass: $subclass, superorder: $superorder })
            CREATE (o)-[:BELONGS_TO]->(c)
            RETURN o
        ")
            .param("class", self.class.clone())
            .param("order", self.order.clone())   
            .param("subclass", self.subclass.clone())
            .param("superorder", self.superorder.clone());

        let mut result = conn.execute(query).await.unwrap();

        if let Ok(Some(row)) = result.next().await {
            let node:Node = row.get("o").unwrap();

            return Ok(Order::from(node));
        }

        Err(DataError::NotInsertedEntity(format!("Entity was not inserted!")))
    }
}

