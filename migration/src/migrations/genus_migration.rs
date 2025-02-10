use async_trait::async_trait;
use neo4rs::Graph;

use models::{
    data::{crud::{Create, Fetch},
        data_error::DataError}, mysql_impl::queries::FetchGenusBuilder, records::genus::Genus,
};

use super::migrate::{Migrate, MigrationResult};

pub struct GenusMigration {
    description: String,
    mysql_conn_pool: mysql::Pool,
    neo4j_graph: Graph,
}

impl GenusMigration {
    pub fn new(desc: &str, mysql_conn_pool: mysql::Pool, neo4j_graph: Graph) -> Self {
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

        let query_builder = FetchGenusBuilder{};
        let genuses = Genus::fetch(self.mysql_conn_pool.clone(), &query_builder).await?;

        for genus in genuses {
            let insert_res = genus.create(self.neo4j_graph.clone()).await;

            match insert_res {
                Ok(_) => {
                    println!("Genus: {}, inserted correctly!", genus.genus);
                    Ok(())
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Genus {} already exists. Will be ignored.", genus.genus);
                    Ok(())
                },
                _ => {
                    println!("Genus: {}, failed to insert!", genus.genus);
                    Err(DataError::QueryError("Failed to insert genus".to_string()))
                }
            }?;
        }

        Ok(result)
    }
}

