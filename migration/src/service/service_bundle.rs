use mysql::*;

use std::{error::Error, fmt::{self, Display, Formatter}, sync::Arc};
use neo4rs::Graph;

use super::config::MigrationConfig;

#[derive(Debug)]
pub enum ServiceBuilderError {
    ConfigNotSet,
    GraphNotSet,
    Neo4jConnectionError(String),
}

impl Display for ServiceBuilderError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::ConfigNotSet => write!(f, "Config not set"),
            Self::GraphNotSet => write!(f, "Graph not set"),
            Self::Neo4jConnectionError(e) => write!(f, "Neo4j connection error: {}", e),
        }
    }
}

impl Error for ServiceBuilderError {}

#[derive(Clone)]
pub struct ServiceBuilder {
    config: Option<MigrationConfig>,
    graph: Option<Graph>,
    mysql_pool: Option<mysql::Pool>,
}

#[derive(Clone)]
pub struct ServiceBundle {
    pub config: Arc<MigrationConfig>,
    pub graph: Graph,
    pub mysql_pool: mysql::Pool,
}

impl ServiceBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            graph: None,
            mysql_pool: None,
        }
    }

    pub fn with_config(mut self) -> Self {
        let config = MigrationConfig::from_toml().unwrap();
        self.config = Some(config);
        self
    }

    pub async fn use_graph(mut self) -> Result<Self, ServiceBuilderError> {
        let config = self.config.as_ref()
            .ok_or(ServiceBuilderError::ConfigNotSet)?;

        let graph = Graph::new(
            &config.neo4j.db_host,
            &config.neo4j.db_user,
            &config.neo4j.db_pass
        )
        .await
        .map_err(|e| ServiceBuilderError::Neo4jConnectionError(e.to_string()))?;

        self.graph = Some(graph);

        Ok(self)
    }

    pub async fn use_mysql(mut self) -> Result<Self, ServiceBuilderError> {
        let config = self.config.as_ref()
            .ok_or(ServiceBuilderError::ConfigNotSet)?;

        let mysql_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.mysql.db_user,
            config.mysql.db_pass,
            config.mysql.db_host,
            config.mysql.db_port,
            config.mysql.db_name
        );

        let opts = Opts::from_url(&mysql_url).unwrap();
        let pool = Pool::new(opts)
            .map_err(|e| ServiceBuilderError::Neo4jConnectionError(e.to_string()))?;

        self.mysql_pool = Some(pool);

        Ok(self)
    }

    pub fn build(self) -> Result<ServiceBundle, ServiceBuilderError> {
        let config = self.config
            .ok_or(ServiceBuilderError::ConfigNotSet)?;

        let graph = self.graph
            .ok_or(ServiceBuilderError::GraphNotSet)?;

        let mysql_pool = self.mysql_pool
            .ok_or(ServiceBuilderError::GraphNotSet)?;

        Ok(ServiceBundle {
            config: Arc::new(config),
            graph,
            mysql_pool,
        })
    }
}

impl Default for ServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

