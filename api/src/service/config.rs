use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MysqlConfig {
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_pass: String,
    pub db_name: String,
}

#[derive(Deserialize, Clone)]
pub struct Neo4jConfig {
    pub db_host: String,
    pub db_user: String,
    pub db_pass: String,
}

#[derive(Deserialize, Clone)]
pub struct LokiConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Clone)]
pub struct TraceConfig {
    pub enabled: bool,
    pub level:String,
    pub loki: LokiConfig,
}

#[derive(Deserialize, Clone)]
pub struct ApiConfig {
    pub mysql: MysqlConfig,
    pub neo4j: Neo4jConfig,
    pub trace: TraceConfig,
}

impl ApiConfig {
    pub fn from_toml() -> Result<Self, std::io::Error> {
        let home = std::env::var("HOME").map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        let toml_path = format!("{}/.config/biojoints/config.toml", home);
        println!("Loading config from {}", toml_path);
        let toml = std::fs::read_to_string(toml_path)?;
        let api_config: ApiConfig = toml::from_str(&toml).unwrap();

        Ok(api_config)
    }
}

