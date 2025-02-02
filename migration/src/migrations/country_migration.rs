use async_trait::async_trait;
use neo4rs::Graph;

use models::{
    data::crud::Create,
    data::data_error::DataError,
    records::country::Country
};
use super::migrate::{Migrate, MigrationResult};

pub struct CountryMigration {
    description: String,
    neo4j_graph: Graph,
}

impl CountryMigration {
    pub fn new(desc: &str, neo4j_graph: Graph) -> Self {
        Self {
            description: String::from(desc),
            neo4j_graph
        }
    }
}

#[async_trait]
impl Migrate for CountryMigration {
    async fn migrate(self: &Self) -> Result<MigrationResult, DataError> {
        println!("{}", self.description);

        let result = MigrationResult {};
        let neo4j_graph = self.neo4j_graph.clone();
        
        let countries = [
            Country::from((String::from("Mexico"), String::from("mx")))
        ];

        for country in countries {
            let result = country.create(neo4j_graph.clone()).await;

            match result {
                Ok(_) => {
                    println!("Country: {}, inserted correctly!", country.name);
                    Ok(())
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Country {} already exists. Will be ignored.", country.name);
                    Ok(())
                },
                other => other,
            }?;
        }

        Ok(result)
    }
}
