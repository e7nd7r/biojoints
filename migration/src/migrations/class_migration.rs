use async_trait::async_trait;
use models::data::data_error::DataError;

use crate::service::service_bundle::ServiceBundle;

use super::migrate::{Migrate, MigrationResult};

pub struct ClassMigration {
    table_name: String,
    service_bundle: ServiceBundle,
}

impl ClassMigration {
    pub fn new(desc: &str, service_bundle: ServiceBundle) -> Self {
        Self {
            table_name: String::from(desc),
            service_bundle,
        }
    }
}

#[async_trait]
impl Migrate for ClassMigration {
    async fn migrate(self: &Self) -> Result<MigrationResult, DataError> {
        let mut result = MigrationResult::new(&self.table_name);

        let neo4j_model = self.service_bundle.neo4j_model_provider.class.clone();
        let mysql_model = self.service_bundle.mysql_model_provider.class.clone();

        let classes = mysql_model.fetch().await?;
        let mut affected = 0;
        let mut ignored = 0;

        for class in classes {
            let insert_res = neo4j_model.create(class.clone()).await.map(|_| ());

            match insert_res {
                Ok(_) => {
                    println!("Class: {}, inserted correctly!", class.class);
                    affected += 1;
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Class {} already exists. Will be ignored.", class.class);
                    ignored += 1;
                },
                _ => Err(DataError::QueryError("Error inserting class".to_string()))?,
            };
        }

        println!("Affected: {}, Ignored: {}", affected, ignored);

        result.set_affected(affected);
        result.set_ignored(ignored);

        Ok(result)
    }
}

