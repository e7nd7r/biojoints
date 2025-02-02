use std::sync::Arc;
use async_trait::async_trait;
use neo4rs::Graph;

use models::{data::{crud::{Create, Fetch}, data_error::DataError}, records::kingdom::Kingdom};
use super::migrate::{Migrate, MigrationResult};

pub struct KingdomMigration {
    description: String,
    mysql_conn_pool: Arc<mysql::Pool>,
    neo4j_graph: Graph,
}

impl KingdomMigration {
    pub fn new(desc: &str, mysql_conn_pool: Arc<mysql::Pool>, neo4j_graph: Graph) -> Self {
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
        println!("{}", self.description);

        let result = MigrationResult {};
        let mysql_conn_pool = self.mysql_conn_pool.clone();
        let neo4j_graph = self.neo4j_graph.clone();
        
        let kingdoms = Kingdom::fetch(mysql_conn_pool.clone())?;

        for kingdom in kingdoms {
            let result = kingdom.create(neo4j_graph.clone()).await;

            match result {
                Ok(_) => {
                    println!("Kingdom: {}, inserted correctly!", kingdom.kingdom);
                    Ok(())
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Kingdom {} already exists. Will be ignored.", kingdom.kingdom);
                    Ok(())
                },
                other => other,
            }?;
        }

        Ok(result)
    }
}
