use async_trait::async_trait;
use neo4rs::Graph;

use models::{data::{crud::{Create, Fetch}, data_error::DataError}, mysql_impl::queries::FetchPhylumBuilder, records::phylum::Phylum};

use super::migrate::{Migrate, MigrationResult};

pub struct PhylumMigration {
    description: String,
    mysql_conn_pool: mysql::Pool,
    neo4j_graph: Graph,
}

impl PhylumMigration {
    pub fn new(desc: &str, mysql_conn_pool: mysql::Pool, neo4j_graph: Graph) -> Self {
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
        let result = MigrationResult {};

        let query_builder = FetchPhylumBuilder{};
        let phylums = Phylum::fetch(self.mysql_conn_pool.clone(), &query_builder).await?;

        for phylum in phylums {
            let insert_res = phylum.create(self.neo4j_graph.clone()).await;

            match insert_res {
                Ok(node) => {
                    println!("Phylum: {}, inserted correctly!", node.phylum);
                    Ok(())
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Phylum {} already exists. Will be ignored.", phylum.phylum);
                    Ok(())
                },
                _ => {
                    println!("Phylum: {}, failed to insert!", phylum.phylum);
                    Err(DataError::QueryError("Failed to insert phylum".to_string()))
                }
            }?;
        }

        Ok(result)
    }
}

