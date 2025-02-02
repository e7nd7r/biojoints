use std::sync::Arc;
use async_trait::async_trait;
use neo4rs::Graph;

use models::{
    data::{crud::{Create, Fetch},
    data_error::DataError},
    records::family::Family
};

use super::migrate::{Migrate, MigrationResult};

pub struct FamilyMigration {
    description: String,
    mysql_conn_pool: Arc<mysql::Pool>,
    neo4j_graph: Graph,
}

impl FamilyMigration {
    pub fn new(desc: &str, mysql_conn_pool: Arc<mysql::Pool>, neo4j_graph: Graph) -> Self {
        Self {
            description: String::from(desc),
            mysql_conn_pool,
            neo4j_graph,
        }
    }   
}

#[async_trait]
impl Migrate for FamilyMigration {
    async fn migrate(self: &Self) -> Result<MigrationResult, DataError> {
        println!("{}", self.description);

        let result = MigrationResult {};
        let mysql_conn_pool = self.mysql_conn_pool.clone();
        let neo4j_graph = self.neo4j_graph.clone();

        let families = Family::fetch(mysql_conn_pool.clone())?;

        for family in families {
            let result = family.create(neo4j_graph.clone()).await;

            match result {
                Ok(_) => {
                    println!("Family: {}, inserted correctly!", family.family);
                    Ok(())
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Family {} already exists. Will be ignored.", family.family);
                    Ok(())
                },
                other => other,
            }?;
        }

        Ok(result)
    }
}
