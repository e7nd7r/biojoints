use api::ApiService;
use clap::{Parser, Subcommand};
use graph::GraphService;

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
    let cli = BioGraphCli::parse();

    match cli.command {
        Some(BioGraphCommand::Api) => {
            let service = ApiService::new();
            service.run();
        },
        Some(BioGraphCommand::Graph) => {
            let service = GraphService::new();
            service.run();
        }
        Some(BioGraphCommand::Migration) => {
            println!("Running Migration")
        }
        _ => {}
    }
}

