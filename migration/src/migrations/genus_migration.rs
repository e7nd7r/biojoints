use std::sync::Arc;
use async_trait::async_trait;
use neo4rs::Graph;

use models::{
    data::{crud::{Create, Fetch},
    data_error::DataError},
    records::genus::Genus
};

use super::migrate::{Migrate, MigrationResult};

pub struct GenusMigration {
    description: String,
    mysql_conn_pool: Arc<mysql::Pool>,
    neo4j_graph: Graph,
}

impl GenusMigration {
    pub fn new(desc: &str, mysql_conn_pool: Arc<mysql::Pool>, neo4j_graph: Graph) -> Self {
        Self {
            description: String::from(desc),
            mysql_conn_pool,
            neo4j_graph,
        }
    }   
}

#[async_trait]
impl Migrate for GenusMigration {
    async fn migrate(self: &Self) -> Result<MigrationResult, DataError> {
        println!("{}", self.description);

        let result = MigrationResult {};
        let mysql_conn_pool = self.mysql_conn_pool.clone();
        let neo4j_graph = self.neo4j_graph.clone();

        let genuses = Genus::fetch(mysql_conn_pool.clone())?;

        for genus in genuses {
            let result = genus.create(neo4j_graph.clone()).await;

            match result {
                Ok(_) => {
                    println!("Genus: {}, inserted correctly!", genus.genus);
                    Ok(())
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Genus {} already exists. Will be ignored.", genus.genus);
                    Ok(())
                },
                other => other,
            }?;
        }

        Ok(result)
    }
}
