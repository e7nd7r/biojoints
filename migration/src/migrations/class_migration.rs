use std::sync::Arc;
use async_trait::async_trait;
use neo4rs::Graph;

use models::{data::{crud::{Create, Fetch}, data_error::DataError, query_builder}, mysql_impl::queries::FetchClassBuilder, records::class::Class};
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
            let result = class.create(self.neo4j_graph.clone()).await;

            match result {
                Ok(_) => {
                    println!("Class: {}, inserted correctly!", class.class);
                    Ok(())
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Class {} already exists. Will be ignored.", class.class);
                    Ok(())
                },
                other => other,
            }?;
        }

        Ok(result)
    }
}
