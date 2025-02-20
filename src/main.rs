use api::ApiService;
use clap::{Parser, Subcommand};
use graph::GraphService;
use migration::MigrationService;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct BioGraphCli {
    #[command(subcommand)]
    command: Option<BioGraphCommand>
}

#[derive(Subcommand)]
enum BioGraphCommand {
    Api,
    Graph,
    Migration,
}

fn main() {
    let _cli = BioGraphCli::parse();

    match Some(BioGraphCommand::Api) {
        Some(BioGraphCommand::Api) => {
            let service = ApiService::new();
            service.run();
        },
        Some(BioGraphCommand::Graph) => {
            let service = GraphService::new();
            service.run();
        }
        Some(BioGraphCommand::Migration) => {
            let service = MigrationService::new();
            service.run();
        }
        _ => {}
    }
}

