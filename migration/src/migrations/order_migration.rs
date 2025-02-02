use std::sync::Arc;
use async_trait::async_trait;
use neo4rs::Graph;

use models::{
    data::{crud::{Create, Fetch},
    data_error::DataError},
    records::order::Order
};

use super::migrate::{Migrate, MigrationResult};

pub struct OrderMigration {
    description: String,
    mysql_conn_pool: Arc<mysql::Pool>,
    neo4j_graph: Graph,
}

impl OrderMigration {
    pub fn new(desc: &str, mysql_conn_pool: Arc<mysql::Pool>, neo4j_graph: Graph) -> Self {
        Self {
            description: String::from(desc),
            mysql_conn_pool,
            neo4j_graph,
        }
    }   
}

#[async_trait]
impl Migrate for OrderMigration {
    async fn migrate(self: &Self) -> Result<MigrationResult, DataError> {
        println!("{}", self.description);

        let result = MigrationResult {};
        let mysql_conn_pool = self.mysql_conn_pool.clone();
        let neo4j_graph = self.neo4j_graph.clone();

        let orders = Order::fetch(mysql_conn_pool.clone())?;

        for order in orders {
            let result = order.create(neo4j_graph.clone()).await;

            match result {
                Ok(_) => {
                    println!("Class: {}, inserted correctly!", order.class);
                    Ok(())
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Class {} already exists. Will be ignored.", order.class);
                    Ok(())
                },
                other => other,
            }?;
        }

        Ok(result)
    }
}
