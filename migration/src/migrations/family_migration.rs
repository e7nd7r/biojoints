use async_trait::async_trait;
use neo4rs::Graph;

use models::{
    data::{
        crud::{Fetch, Create},
    data_error::DataError}, mysql_impl::queries::FetchFamilyBuilder, records::family::Family
};

use super::migrate::{Migrate, MigrationResult};

pub struct FamilyMigration {
    description: String,
    mysql_conn_pool: mysql::Pool,
    neo4j_graph: Graph,
}

impl FamilyMigration {
    pub fn new(desc: &str, mysql_conn_pool: mysql::Pool, neo4j_graph: Graph) -> Self {
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
        let result = MigrationResult {};

        let query_builder = FetchFamilyBuilder{};

        let families = Family::fetch(self.mysql_conn_pool.clone(), &query_builder).await?;

        for family in families {
            let insert_res = family.create(self.neo4j_graph.clone()).await;

            match insert_res {
                Ok(node) => {
                    println!("Family: {}, inserted correctly!", family.family);
                    Ok(())
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Family {} already exists. Will be ignored.", family.family);
                    Ok(())
                },
                _ => {
                    println!("Family: {}, failed to insert!", family.family);
                    Err(DataError::QueryError("Failed to insert family".to_string()))
                }
            }?;
        }

        Ok(result)
    }
}
