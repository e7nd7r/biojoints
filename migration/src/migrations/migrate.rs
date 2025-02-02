use async_trait::async_trait;
 
use models::data::data_error::DataError;

#[derive(Debug)]
pub struct MigrationResult {
    
}

#[async_trait]
pub trait Migrate {
    async fn migrate(self: &Self) -> Result<MigrationResult, DataError>;
}
