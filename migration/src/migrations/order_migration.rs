use async_trait::async_trait;
use neo4rs::Graph;

use models::{
    data::{crud::{Create, Fetch},
    data_error::DataError}, mysql_impl::queries::FetchOrderBuilder, records::order::Order
};

use super::migrate::{Migrate, MigrationResult};

pub struct OrderMigration {
    description: String,
    mysql_conn_pool: mysql::Pool,
    neo4j_graph: Graph,
}

impl OrderMigration {
    pub fn new(desc: &str, mysql_conn_pool: mysql::Pool, neo4j_graph: Graph) -> Self {
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
        let result = MigrationResult {};
        let query_builder = FetchOrderBuilder{};

        let orders = Order::fetch(self.mysql_conn_pool.clone(), &query_builder).await?;

        for order in orders {
            let insert_result = order.create(self.neo4j_graph.clone()).await;

            match insert_result {
                Ok(_) => {
                    println!("Class: {}, inserted correctly!", order.class);
                    Ok(())
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Class {} already exists. Will be ignored.", order.class);
                    Ok(())
                },
                _ => {
                    println!("Class: {}, failed to insert!", order.class);
                    Err(DataError::QueryError("Failed to insert class".to_string()))
                }
            }?;
        }

        Ok(result)
    }
}

