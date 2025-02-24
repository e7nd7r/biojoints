use async_trait::async_trait;

use models::{data::data_error::DataError, mysql_impl::{self, relational_layer::RelationalLayer}, neo4j_impl::{self, graph_layer::GraphLayer}};

use crate::service::service_bundle::ServiceBundle;

use super::migrate::{Migrate, MigrationResult};

pub struct FamilyMigration {
    table_name: String,
    service_bundle: ServiceBundle,
}

impl FamilyMigration {
    pub fn new(table_name: &str, service_bundle: ServiceBundle) -> Self {
        Self {
            table_name: String::from(table_name),
            service_bundle,
        }
    }
}

#[async_trait]
impl Migrate for FamilyMigration {
    async fn migrate(&self) -> Result<MigrationResult, DataError> {
        let mut result = MigrationResult::new(&self.table_name);
        let graph = self.service_bundle.graph.clone();
        let mysql_pool = self.service_bundle.mysql_pool.clone();
        let relational = RelationalLayer::new(mysql_pool);
        let graph = GraphLayer::new(graph);

        let neo4j_model = neo4j_impl::family::FamilyModel::new(graph);
        let mysql_model = mysql_impl::family::FamilyModel::new(relational);

        let families = mysql_model.fetch().await?;
        let mut affected = 0;
        let mut ignored = 0;

        for family in families {
            let insert_res = neo4j_model.create(family.clone()).await.map(|_| ());

            match insert_res {
                Ok(_) => {
                    println!("Family: {}, inserted correctly!", family.family);
                    affected += 1;
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Family {} already exists. Will be ignored.", family.family);
                    ignored += 1;
                },
                _ => Err(DataError::QueryError("Error inserting family".to_string()))?,
            };
        }

        println!("Affected: {}, Ignored: {}", affected, ignored);
        result.set_affected(affected);
        result.set_ignored(ignored);

        Ok(result)
    }
}
