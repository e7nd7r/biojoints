use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MysqlConfig {
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub db_user: String,
    pub db_pass: String,
}

#[derive(Deserialize, Clone)]
pub struct Neo4jConfig {
    pub db_host: String,
    pub db_user: String,
    pub db_pass: String,
}

#[derive(Deserialize, Clone)]
pub struct MigrationConfig {
    pub neo4j: Neo4jConfig,
    pub mysql: MysqlConfig,
}

impl MigrationConfig {
    pub fn from_toml() -> Result<Self, std::io::Error> {
        let home = std::env::var("HOME").map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        let toml_path = format!("{}/.config/biojoints/config.toml", home);
        println!("Loading config from {}", toml_path);
        let toml = std::fs::read_to_string(toml_path)?;
        let api_config: MigrationConfig = toml::from_str(&toml).unwrap();

        Ok(api_config)
    }
}

