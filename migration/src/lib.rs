mod migrations;

use migrations::{
    class_migration::ClassMigration,
    country_migration::CountryMigration,
    family_migration::FamilyMigration,
    genus_migration::GenusMigration,
    kingdom_migration::KingdomMigration,
    migrate::Migrate,
    order_migration::OrderMigration,
    phylum_migration::PhylumMigration,
    specie_migration::SpecieMigration,
    state_migration::StateMigration,
};

use mysql::*;
use neo4rs::*;

pub struct MigrationService {
}

impl MigrationService {
    pub async fn run() {
        let url = "<URL here>";
        let pool = Pool::new(url).expect("Error creating the pool");

        let graph = Graph::new(
            "<neo4j url here>",
            "<user>", 
            "<password>").await.unwrap();

        let kingdom_migration = KingdomMigration::new(
            "Kingdom Migration", pool.clone(), graph.clone()
        );

        let phylum_migration = PhylumMigration::new(
            "Phylum Migration", pool.clone(), graph.clone()
        );

        let class_migration = ClassMigration::new(
            "Class Migration", pool.clone(), graph.clone()
        );

        let order_migration = OrderMigration::new(
            "Order Migration", pool.clone(), graph.clone()
        );

        let family_migration = FamilyMigration::new(
            "Family Migration", pool.clone(), graph.clone()
        );

        let genus_migration = GenusMigration::new(
            "Genus Migration", pool.clone(), graph.clone()
        );

        let specie_migration = SpecieMigration::new(
            "Specie Migration", pool.clone(), graph.clone()
        );

        let country_migration = CountryMigration::new(
            "Country Migration", graph.clone()
        );

        let state_migration = StateMigration::new(
            "State Migration", graph.clone(),
        );

        let migrations: Vec<Box<dyn Migrate>> = vec![
            Box::new(kingdom_migration),
            Box::new(phylum_migration),
            Box::new(class_migration),
            Box::new(order_migration),
            Box::new(family_migration),
            Box::new(genus_migration),
            Box::new(country_migration),
            Box::new(state_migration),
            Box::new(specie_migration),
        ];

        for migration in migrations {
            match migration.migrate().await {
                Ok(res) => println!("{:?}", res),
                Err(err) => panic!("{:?}", err)
            }
        }
    }
}

