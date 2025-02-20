use async_trait::async_trait;
use neo4rs::Graph;

use models::{
    data::data_error::DataError,
    mysql_impl::{self, relational_layer::RelationalLayer},
    neo4j_impl::{self, graph_layer::GraphLayer}
};

use super::migrate::{Migrate, MigrationResult};

pub struct KingdomMigration {
    table_name: String,
    mysql_conn_pool: mysql::Pool,
    neo4j_graph: Graph,
}

impl KingdomMigration {
    pub fn new(table_name: &str, mysql_conn_pool: mysql::Pool, neo4j_graph: Graph) -> Self {
        Self {
            table_name: String::from(table_name),
            mysql_conn_pool,
            neo4j_graph,
        }
    }
}

#[async_trait]
impl Migrate for KingdomMigration {
    async fn migrate(&self) -> Result<MigrationResult, DataError> {
        let mut result = MigrationResult::new(&self.table_name);
        let relational = RelationalLayer::new(self.mysql_conn_pool.clone());
        let graph = GraphLayer::new(self.neo4j_graph.clone());

        let neo4j_model = neo4j_impl::kingdom::KingdomModel::new(graph);
        let mysql_model = mysql_impl::kingdom::KingdomModel::new(relational);

        let kingdoms = mysql_model.fetch().await?;
        let mut affected = 0;
        let mut ignored = 0;

        for kingdom in kingdoms {
            let insert_res = neo4j_model.create(kingdom.clone()).await.map(|_| ());

            match insert_res {
                Ok(_) => {
                    println!("Kingdom: {}, inserted correctly!", kingdom.kingdom);
                    affected += 1;
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Kingdom {} already exists. Will be ignored.", kingdom.kingdom);
                    ignored += 1;
                },
                Err(e) => return Err(DataError::QueryError(
                    format!("Error inserting kingdom {}: {}", kingdom.kingdom, e)
                )),
            };
        }

        println!("Affected: {}, Ignored: {}", affected, ignored);
        result.set_affected(affected);
        result.set_ignored(ignored);

        Ok(result)
    }
}

