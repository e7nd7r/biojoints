use std::{process, sync::Arc};

use actix_web::rt::spawn;
use neo4rs::Graph;

use reqwest::Url;
use tracing_subscriber::{
    prelude::*,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    filter::LevelFilter
};
use super::config::ApiConfig;

#[derive(Clone)]
pub struct ServiceBundle {
    pub config: Arc<ApiConfig>,
    pub graph: Graph,
}

impl ServiceBundle {
    pub async fn new(config: ApiConfig) -> Result<Self, std::io::Error> {
        let graph = Graph::new(
            &config.neo4j.db_host,
            &config.neo4j.db_user,
            &config.neo4j.db_pass
        ).await.unwrap();

        let bundle = Self {
            config: Arc::new(config),
            graph,
        };

        Ok(bundle)
    }

    pub fn subscribe_logger(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config.trace.enabled {
            return Ok(());
        }

        let registry = tracing_subscriber::registry();

        let level = match self.config.trace.level.as_str() {
            "error" => LevelFilter::ERROR,
            "warn" => LevelFilter::WARN,
            "info" => LevelFilter::INFO,
            "debug" => LevelFilter::DEBUG,
            "trace" => LevelFilter::TRACE,
            _ => LevelFilter::INFO,
        };

        if !self.config.trace.loki.enabled {
            registry
                .with(tracing_subscriber::fmt::layer().with_filter(level))
                .init();

            return Ok(());
        }

        let loki_url = format!(
            "http://{}:{}/loki/api/v1/push",
            self.config.trace.loki.host,
            self.config.trace.loki.port
        );

        let (layer, task) = tracing_loki::builder()
            .label("host", "mine")?
            .extra_field("pid", format!("{}", process::id()))?
            .build_url(Url::parse(&loki_url).unwrap())?;

        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().with_filter(level))
            .with(layer.with_filter(level))
            .init();

        spawn(task);

        Ok(())
    }
}


