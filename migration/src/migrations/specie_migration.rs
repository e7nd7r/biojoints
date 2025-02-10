use async_trait::async_trait;
use neo4rs::Graph;

use models::{
    data::{
        crud::{Create, Fetch},
        data_error::DataError
    }, mysql_impl::queries::FetchSpecieBuilder, neo4j_impl::specie::SpecieOps, records::specie::Specie
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
            let insert_res = specie.create(self.neo4j_graph.clone()).await;

            match insert_res {
                Ok(node) => {
                    println!("Specie: {}, inserted correctly!", node.specie_name);
                    Ok(())
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Specie {} already exists. Will be ignored.", specie.specie_name);
                    Ok(())
                },
                _ => Err(DataError::QueryError("Error inserting specie".to_string())),
            }?;

            let insert_res = specie.create_dist_nodes(self.neo4j_graph.clone()).await;

            match insert_res {
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

