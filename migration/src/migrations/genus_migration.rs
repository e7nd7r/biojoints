use async_trait::async_trait;

use models::data::data_error::DataError;
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

        let neo4j_model = self.service_bundle.neo4j_model_provider.genus.clone();
        let mysql_model = self.service_bundle.mysql_model_provider.genus.clone();

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

