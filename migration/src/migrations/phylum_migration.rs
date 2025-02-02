use std::sync::Arc;
use async_trait::async_trait;
use neo4rs::Graph;

use models::{data::{crud::{Create, Fetch}, data_error::DataError}, records::phylum::Phylum};

use super::migrate::{Migrate, MigrationResult};

pub struct PhylumMigration {
    description: String,
    mysql_conn_pool: Arc<mysql::Pool>,
    neo4j_graph: Graph,
}

impl PhylumMigration {
    pub fn new(desc: &str, mysql_conn_pool: Arc<mysql::Pool>, neo4j_graph: Graph) -> Self {
        Self {
            description: String::from(desc),
            mysql_conn_pool,
            neo4j_graph,
        }
    }   
}

#[async_trait]
impl Migrate for PhylumMigration {
    async fn migrate(self: &Self) -> Result<MigrationResult, DataError> {
        println!("{}", self.description);

        let result = MigrationResult {};

        let mysql_conn_pool = self.mysql_conn_pool.clone();
        let neo4j_graph = self.neo4j_graph.clone();

        let phylums = Phylum::fetch(mysql_conn_pool.clone())?;

        for phylum in phylums {
            let result = phylum.create(neo4j_graph.clone()).await;

            match result {
                Ok(_) => {
                    println!("Phylum: {}, inserted correctly!", phylum.phylum);
                    Ok(())
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Phylum {} already exists. Will be ignored.", phylum.phylum);
                    Ok(())
                },
                other => other,
            }?;
        }

        Ok(result)
    }
}
