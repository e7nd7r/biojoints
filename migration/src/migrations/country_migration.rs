use async_trait::async_trait;

use models::{
    data::data_error::DataError, records::country::Country
};

use crate::service::service_bundle::ServiceBundle;

use super::migrate::{Migrate, MigrationResult};

pub struct CountryMigration {
    table_name: String,
    service_bundle: ServiceBundle,
}

impl CountryMigration {
    pub fn new(desc: &str, service_bundle: ServiceBundle) -> Self {
        Self {
            table_name: String::from(desc),
            service_bundle,
        }
    }
}

#[async_trait]
impl Migrate for CountryMigration {
    async fn migrate(self: &Self) -> Result<MigrationResult, DataError> {
        let mut result = MigrationResult::new(&self.table_name);

        let countries = [
            Country::from((String::from("Mexico"), String::from("mx")))
        ];

        let neo4j_model = self.service_bundle.neo4j_model_provider.country.clone();

        let mut affected = 0;
        let mut ignored = 0;

        for country in countries {
            let insert_res = neo4j_model.create(country.clone()).await.map(|_| ());

            match insert_res {
                Ok(_) => {
                    println!("Country: {}, inserted correctly!", country.name);
                    affected += 1;
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Country {} already exists. Will be ignored.", country.name);
                    ignored += 1;
                },
                _ => Err(DataError::QueryError("Error inserting country".to_string()))?,
            };
        }

        println!("Affected: {}, Ignored: {}", affected, ignored);

        result.set_affected(affected);
        result.set_ignored(ignored);

        Ok(result)
    }
}

