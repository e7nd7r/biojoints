use std::sync::Arc;
use async_trait::async_trait;
use neo4rs::Graph;

use models::{
    data::{crud::{Create, Fetch},
        data_error::DataError, query_builder}, mysql_impl::queries::FetchSpecieBuilder, neo4j_impl::specie::SpecieOps, records::specie::Specie
};

use super::migrate::{Migrate, MigrationResult};

pub struct SpecieMigration {
    description: String,
    mysql_conn_pool: mysql::Pool,
    neo4j_graph: Graph,
}

impl SpecieMigration {
    pub fn new(desc: &str, mysql_conn_pool: mysql::Pool, neo4j_graph: Graph) -> Self {
        Self {
            description: String::from(desc),
            mysql_conn_pool,
            neo4j_graph,
        }
    }
}

#[async_trait]
impl Migrate for SpecieMigration {
    async fn migrate(self: &Self) -> Result<MigrationResult, DataError> {
        let result = MigrationResult {};
        let query_builder = FetchSpecieBuilder{};
        let species = Specie::fetch(self.mysql_conn_pool.clone(), &query_builder).await?;

        for specie in species {
            let result = specie.create(self.neo4j_graph.clone()).await;

            match result {
                Ok(_) => {
                    println!("Specie: {}, inserted correctly!", specie.specie_name);
                    Ok(())
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Specie {} already exists. Will be ignored.", specie.specie_name);
                    Ok(())
                },
                other => other,
            }?;

            let result = specie.create_dist_nodes(self.neo4j_graph.clone()).await;

            match result {
                Ok(_) => {
                    println!("Specie: {}, state nodes inserted correctly!", specie.specie_name);
                    Ok(())
                },
                other => other,
            }?;
        }

        Ok(result)
    }
}

