use std::sync::Arc;
use async_trait::async_trait;
use neo4rs::Graph;

use models::{
    data::{crud::{Create, Fetch},
        data_error::DataError},
    records::specie::Specie, neo4j_impl::specie::SpecieOps
};

use super::migrate::{Migrate, MigrationResult};

pub struct SpecieMigration {
    description: String,
    mysql_conn_pool: Arc<mysql::Pool>,
    neo4j_graph: Graph,
}

impl SpecieMigration {
    pub fn new(desc: &str, mysql_conn_pool: Arc<mysql::Pool>, neo4j_graph: Graph) -> Self {
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
        println!("{}", self.description);

        let result = MigrationResult {};
        let mysql_conn_pool = self.mysql_conn_pool.clone();
        let neo4j_graph = self.neo4j_graph.clone();

        let species = Specie::fetch(mysql_conn_pool.clone())?;

        for specie in species {
            let result = specie.create(neo4j_graph.clone()).await;

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

            let result = specie.create_dist_nodes(neo4j_graph.clone()).await;

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
