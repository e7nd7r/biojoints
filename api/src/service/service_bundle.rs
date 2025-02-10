use std::sync::Arc;

use mysql::Pool;
use neo4rs::Graph;

use super::config::ApiConfig;

#[derive(Clone)]
pub struct ServiceBundle {
    pub config: Arc<ApiConfig>,
    pub graph: Graph,
    pub pool: Pool,
}

impl ServiceBundle {
    pub async fn new(config: ApiConfig) -> Result<Self, std::io::Error> {
        let mysql_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.mysql.db_user,
            config.mysql.db_pass,
            config.mysql.db_host,
            config.mysql.db_port,
            config.mysql.db_name
        );

        let pool = Pool::new(mysql_url.as_ref()).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let graph = Graph::new(
            &config.neo4j.db_host,
            &config.neo4j.db_user,
            &config.neo4j.db_pass
        ).await.unwrap();

        let bundle = Self {
            config: Arc::new(config),
            graph,
            pool,
        };

        Ok(bundle)
    }
}

