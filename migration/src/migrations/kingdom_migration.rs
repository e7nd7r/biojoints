use async_trait::async_trait;
use neo4rs::Graph;

use models::{data::{crud::{Create, Fetch}, data_error::DataError}, mysql_impl::queries::FetchKingdomBuilder, records::kingdom::Kingdom};
use super::migrate::{Migrate, MigrationResult};

pub struct KingdomMigration {
    description: String,
    mysql_conn_pool: mysql::Pool,
    neo4j_graph: Graph,
}

impl KingdomMigration {
    pub fn new(desc: &str, mysql_conn_pool: mysql::Pool, neo4j_graph: Graph) -> Self {
        Self {
            description: String::from(desc),
            mysql_conn_pool,
            neo4j_graph
        }
    }
}

#[async_trait]
impl Migrate for KingdomMigration {
    async fn migrate(self: &Self) -> Result<MigrationResult, DataError> {
        let result = MigrationResult {};

        let query_builder = FetchKingdomBuilder{}; 
        let kingdoms = Kingdom::fetch(self.mysql_conn_pool.clone(), &query_builder).await?;

        for kingdom in kingdoms {
            let insert_res = kingdom.create(self.neo4j_graph.clone()).await;

            match insert_res {
                Ok(node) => {
                    println!("Kingdom: {}, inserted correctly!", node.kingdom);
                    Ok(())
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Kingdom {} already exists. Will be ignored.", kingdom.kingdom);
                    Ok(())
                },
                _ => Err(DataError::QueryError("Error inserting kingdom".to_string())),
            }?;
        }

        Ok(result)
    }
}

