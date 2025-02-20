use async_trait::async_trait;
use neo4rs::Graph;

use models::{
    data::data_error::DataError,
    mysql_impl::{self, relational_layer::RelationalLayer},
    neo4j_impl::{self, graph_layer::GraphLayer}
};

use super::migrate::{Migrate, MigrationResult};

pub struct SpecieMigration {
    table_name: String,
    mysql_conn_pool: mysql::Pool,
    neo4j_graph: Graph,
}

impl SpecieMigration {
    pub fn new(table_name: &str, mysql_conn_pool: mysql::Pool, neo4j_graph: Graph) -> Self {
        Self {
            table_name: String::from(table_name),
            mysql_conn_pool,
            neo4j_graph,
        }
    }
}

#[async_trait]
impl Migrate for SpecieMigration {
    async fn migrate(&self) -> Result<MigrationResult, DataError> {
        let mut result = MigrationResult::new(&self.table_name);
        let relational = RelationalLayer::new(self.mysql_conn_pool.clone());
        let graph = GraphLayer::new(self.neo4j_graph.clone());

        let neo4j_model = neo4j_impl::specie::SpecieModel::new(graph);
        let mysql_model = mysql_impl::specie::SpecieModel::new(relational);

        let species = mysql_model.fetch().await?;
        let mut affected = 0;
        let mut ignored = 0;

        for specie in species {
            let insert_res = neo4j_model.create(specie.clone()).await.map(|_| ());

            match insert_res {
                Ok(_) => {
                    println!("Specie: {}, inserted correctly!", specie.specie_name);

                    // After successful specie creation, create distribution nodes
                    match neo4j_model.create_dist_nodes(specie.clone()).await {
                        Ok(_) => {
                            println!("Specie: {}, state nodes inserted correctly!", specie.specie_name);
                            affected += 1;
                        },
                        Err(e) => return Err(DataError::QueryError(
                            format!("Error creating distribution nodes for specie {}: {}", specie.specie_name, e)
                        )),
                    }
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Specie {} already exists. Will be ignored.", specie.specie_name);
                    ignored += 1;
                },
                Err(e) => return Err(DataError::QueryError(
                    format!("Error inserting specie {}: {}", specie.specie_name, e)
                )),
            };
        }

        println!("Affected: {}, Ignored: {}", affected, ignored);

        result.set_affected(affected);
        result.set_ignored(ignored);

        Ok(result)
    }
}


