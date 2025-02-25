use async_trait::async_trait;

use models::data::data_error::DataError;

use crate::service::service_bundle::ServiceBundle;

use super::migrate::{Migrate, MigrationResult};

pub struct SpecieMigration {
    table_name: String,
    service_bundle: ServiceBundle,
}

impl SpecieMigration {
    pub fn new(table_name: &str, service_bundle: ServiceBundle) -> Self {
        Self {
            table_name: String::from(table_name),
            service_bundle,
        }
    }
}

#[async_trait]
impl Migrate for SpecieMigration {
    async fn migrate(&self) -> Result<MigrationResult, DataError> {
        let mut result = MigrationResult::new(&self.table_name);

        let neo4j_model = self.service_bundle.neo4j_model_provider.specie.clone();
        let mysql_model = self.service_bundle.mysql_model_provider.specie.clone();

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


