use async_trait::async_trait;

use models::{
    data::data_error::DataError,
    mysql_impl::{self, relational_layer::RelationalLayer},
    neo4j_impl::{self, graph_layer::GraphLayer}
};

use crate::service::service_bundle::ServiceBundle;

use super::migrate::MigrationResult;

pub struct GenusMigration {
    table_name: String,
    service_bundle: ServiceBundle,
}

use super::migrate::Migrate;

impl GenusMigration {
    pub fn new(table_name: &str, service_bundle: ServiceBundle) -> Self {
        Self {
            table_name: String::from(table_name),
            service_bundle
        }
    }
}

#[async_trait]
impl Migrate for GenusMigration {
    async fn migrate(&self) -> Result<MigrationResult, DataError> {
        let mut result = MigrationResult::new(&self.table_name);
        let graph = self.service_bundle.graph.clone();
        let mysql_pool = self.service_bundle.mysql_pool.clone();

        let relational = RelationalLayer::new(mysql_pool);
        let graph = GraphLayer::new(graph);

        let neo4j_model = neo4j_impl::genus::GenusModel::new(graph);
        let mysql_model = mysql_impl::genus::GenusModel::new(relational);

        let genuses = mysql_model.fetch().await?;
        let mut affected = 0;
        let mut ignored = 0;

        for genus in genuses {
            let insert_res = neo4j_model.create(genus.clone()).await.map(|_| ());

            match insert_res {
                Ok(_) => {
                    println!("Genus: {}, inserted correctly!", genus.genus);
                    affected += 1;
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Genus {} already exists. Will be ignored.", genus.genus);
                    ignored += 1;
                },
                Err(e) => Err(DataError::QueryError(format!("Error inserting genus {}: {}", genus.genus, e)))?,
            };
        }

        println!("Affected: {}, Ignored: {}", affected, ignored);
        result.set_affected(affected);
        result.set_ignored(ignored);

        Ok(result)
    }
}

