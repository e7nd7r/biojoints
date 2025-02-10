use async_trait::async_trait;
use neo4rs::Graph;

use models::{
    data::crud::Create,
    data::data_error::DataError,
    records::state::State,
};
use super::migrate::{Migrate, MigrationResult};

pub struct StateMigration {
    description: String,
    neo4j_graph: Graph,
}

impl StateMigration {
    pub fn new(desc: &str, neo4j_graph: Graph) -> Self {
        Self {
            description: String::from(desc),
            neo4j_graph
        }
    }
}

#[async_trait]
impl Migrate for StateMigration {
    async fn migrate(self: &Self) -> Result<MigrationResult, DataError> {
        let result = MigrationResult {};
        let neo4j_graph = self.neo4j_graph.clone();

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
            let result = state.create(neo4j_graph.clone()).await;

            match result {
                Ok(_) => {
                    println!("State: {}, inserted correctly!", state.name);
                    Ok(())
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("State {} already exists. Will be ignored.", state.name);
                    Ok(())
                },
                other => other,
            }?;
        }

        Ok(result)
    }
}

