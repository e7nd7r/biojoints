use async_trait::async_trait;
 
use models::data::data_error::DataError;

#[derive(Debug)]
pub struct MigrationResult {
    pub table_name: String,
    rows_affected: usize,
    rows_ignored: usize,
}

impl MigrationResult {
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_owned(),
            rows_affected: 0,
            rows_ignored: 0,
        }
    }

    pub fn set_affected(&mut self, affected: usize) {
        self.rows_affected = affected;
    }

    pub fn set_ignored(&mut self, ignored: usize) {
        self.rows_ignored = ignored;
    }

    pub fn affected(&self) -> usize {
        self.rows_affected
    }

    pub fn ignored(&self) -> usize {
        self.rows_ignored
    }
}

#[async_trait]
pub trait Migrate {
    async fn migrate(&self) -> Result<MigrationResult, DataError>;
}

