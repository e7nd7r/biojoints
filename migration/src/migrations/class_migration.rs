use async_trait::async_trait;
use neo4rs::Graph;

use models::{data::{crud::{Create, Fetch}, data_error::DataError}, mysql_impl::queries::FetchClassBuilder, records::class::Class};
use super::migrate::{Migrate, MigrationResult};

pub struct ClassMigration {
    description: String,
    mysql_conn_pool: mysql::Pool,
    neo4j_graph: Graph,
}

impl ClassMigration {
    pub fn new(desc: &str, mysql_conn_pool: mysql::Pool, neo4j_graph: Graph) -> Self {
        Self {
            description: String::from(desc),
            mysql_conn_pool,
            neo4j_graph,
        }
    }
}

#[async_trait]
impl Migrate for ClassMigration {
    async fn migrate(self: &Self) -> Result<MigrationResult, DataError> {
        let result = MigrationResult {};
        let query_builder = FetchClassBuilder{};

        let classes = Class::fetch(self.mysql_conn_pool.clone(), &query_builder).await?;

        for class in classes {
            let insert_res = class.create(self.neo4j_graph.clone()).await;

            match insert_res {
                Ok(node) => {
                    println!("Class: {}, inserted correctly!", node.class);
                    Ok(())
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Class {} already exists. Will be ignored.", class.class);
                    Ok(())
                },
                _ => {
                    println!("Class: {}, failed to insert!", class.class);
                    Err(DataError::QueryError("Failed to insert class".to_string()))
                }
            }?;
        }

        Ok(result)
    }
}

