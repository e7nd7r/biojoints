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
    pub fn new() -> Self {
        Self {}
    }

    async fn inner_run(&self) {
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

        println!("\n=== Starting Migration Process ===\n");

        let mut total_affected = 0;
        let mut total_ignored = 0;
        let mut successful_migrations = 0;
        let total_migrations = migrations.len();

        for migration in migrations {
            match migration.migrate().await {
                Ok(res) => {
                    println!("✓ {}", res.table_name);
                    println!("  Rows affected: {}", res.affected());
                    println!("  Rows ignored: {}\n", res.ignored());

                    total_affected += res.affected();
                    total_ignored += res.ignored();
                    successful_migrations += 1;
                },
                Err(err) => {
                    println!("✗ Migration failed: {:?}\n", err);
                    panic!("Migration process aborted due to error: {:?}", err);
                }
            }
        }

        println!("=== Migration Summary ===");
        println!("Successful migrations: {}/{}", successful_migrations, total_migrations);
        println!("Total rows affected: {}", total_affected);
        println!("Total rows ignored: {}", total_ignored);
        println!("========================\n");
    }

    pub fn run(&self) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(self.inner_run());
    }
}

impl Default for MigrationService {
    fn default() -> Self {
        Self::new()
    }
}

