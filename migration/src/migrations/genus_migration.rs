use neo4rs::Graph;
use async_trait::async_trait;

use models::{
    data::data_error::DataError,
    mysql_impl::{self, relational_layer::RelationalLayer},
    neo4j_impl::{self, graph_layer::GraphLayer}
};

use super::migrate::MigrationResult;

pub struct GenusMigration {
    table_name: String,
    mysql_conn_pool: mysql::Pool,
    neo4j_graph: Graph,
}

use super::migrate::Migrate;

impl GenusMigration {
    pub fn new(table_name: &str, mysql_conn_pool: mysql::Pool, neo4j_graph: Graph) -> Self {
        Self {
            table_name: String::from(table_name),
            mysql_conn_pool,
            neo4j_graph,
        }
    }
}

#[async_trait]
impl Migrate for GenusMigration {
    async fn migrate(&self) -> Result<MigrationResult, DataError> {
        let mut result = MigrationResult::new(&self.table_name);
        let relational = RelationalLayer::new(self.mysql_conn_pool.clone());
        let graph = GraphLayer::new(self.neo4j_graph.clone());

        let neo4j_model = neo4j_impl::genus::GenusModel::new(graph);
        let mysql_model = mysql_impl::genus::GenusModel::new(relational);

        let genuses = mysql_model.fetch().await?;
        let mut affected = 0;
        let mut ignored = 0;

        for genus in genuses {
            let insert_res = neo4j_model.create(genus.clone()).await.map(|_| ());

            match insert_res {
                Ok(_) => {
                    println!("Genus: {}, inserted correctly!", genus.genus);
                    affected += 1;
                },
                Err(DataError::AlreadyExist(_)) => {
                    println!("Genus {} already exists. Will be ignored.", genus.genus);
                    ignored += 1;
                },
                Err(e) => Err(DataError::QueryError(format!("Error inserting genus {}: {}", genus.genus, e)))?,
            };
        }

        println!("Affected: {}, Ignored: {}", affected, ignored);
        result.set_affected(affected);
        result.set_ignored(ignored);

        Ok(result)
    }
}

