use async_trait::async_trait;
use neo4rs::Graph;

use models::{
    data::data_error::DataError, neo4j_impl::{graph_layer::GraphLayer, state::StateModel}, records::state::State
};
use super::migrate::{Migrate, MigrationResult};

pub struct StateMigration {
    table_name: String,
    neo4j_graph: Graph,
}

impl StateMigration {
    pub fn new(desc: &str, neo4j_graph: Graph) -> Self {
        Self {
            table_name: String::from(desc),
            neo4j_graph
        }
    }
}

#[async_trait]
impl Migrate for StateMigration {
    async fn migrate(self: &Self) -> Result<MigrationResult, DataError> {
        let mut result = MigrationResult::new(&self.table_name);
        let mut affected = 0;
        let mut ignored = 0;

        let graph_layer = GraphLayer::new(self.neo4j_graph.clone());
        let model = StateModel::new(graph_layer);

        let states = [
            State::from((String::from("mx"), String::from("Aguascalientes"), String::from("agu"))),
            State::from((String::from("mx"), String::from("Baja California"), String::from("bcn"))),
            State::from((String::from("mx"), String::from("Baja California Sur"), String::from("bcs"))),
            State::from((String::from("mx"), String::from("Campeche"), String::from("cam"))),
            State::from((String::from("mx"), String::from("Chiapas"), String::from("chp"))),
            State::from((String::from("mx"), String::from("Chihuahua"), String::from("chh"))),
            State::from((String::from("mx"), String::from("Coahuila"), String::from("coa"))),
            State::from((String::from("mx"), String::from("Colima"), String::from("col"))),
            State::from((String::from("mx"), String::from("Mexico City"), String::from("cmx"))),
            State::from((String::from("mx"), String::from("Durango"), String::from("dur"))),
            State::from((String::from("mx"), String::from("Guanajuato"), String::from("gua"))),
            State::from((String::from("mx"), String::from("Guerrero"), String::from("gro"))),
            State::from((String::from("mx"), String::from("Hidalgo"), String::from("hid"))),
            State::from((String::from("mx"), String::from("Jalisco"), String::from("jal"))),
            State::from((String::from("mx"), String::from("México"), String::from("mex"))),
            State::from((String::from("mx"), String::from("Michoacán"), String::from("mic"))),
            State::from((String::from("mx"), String::from("Morelos"), String::from("mor"))),
            State::from((String::from("mx"), String::from("Nayarit"), String::from("nay"))),
            State::from((String::from("mx"), String::from("Nuevo León"), String::from("nle"))),
            State::from((String::from("mx"), String::from("Oaxaca"), String::from("oax"))),
            State::from((String::from("mx"), String::from("Puebla"), String::from("pue"))),
            State::from((String::from("mx"), String::from("Querétaro"), String::from("que"))),
            State::from((String::from("mx"), String::from("Quintana Roo"), String::from("roo"))),
            State::from((String::from("mx"), String::from("San Luis Potosí"), String::from("slp"))),
            State::from((String::from("mx"), String::from("Sinaloa"), String::from("sin"))),
            State::from((String::from("mx"), String::from("Sonora"), String::from("son"))),
            State::from((String::from("mx"), String::from("Tabasco"), String::from("tab"))),
            State::from((String::from("mx"), String::from("Tamaulipas"), String::from("tam"))),
            State::from((String::from("mx"), String::from("Tlaxcala"), String::from("tla"))),
            State::from((String::from("mx"), String::from("Veracruz"), String::from("ver"))),
            State::from((String::from("mx"), String::from("Yucatán"), String::from("yuc"))),
            State::from((String::from("mx"), String::from("Zacatecas"), String::from("zac"))),
        ];

        for state in states {
            let insert_res = model.create(state.clone()).await;

            match insert_res {
                Ok(_) => {
                    println!("State: {}, inserted correctly!", state.name);
                    affected += 1;
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("State {} already exists. Will be ignored.", state.name);
                    ignored += 1;
                },
                _ => {
                    println!("State: {}, failed to insert!", state.name);
                    return Err(DataError::QueryError("Failed to insert state".to_string()));
                },
            }
        }

        println!("Affected: {}, Ignored: {}", affected, ignored);

        result.set_affected(affected);
        result.set_ignored(ignored);

        Ok(result)
    }
}

