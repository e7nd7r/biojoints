use async_trait::async_trait;

use super::storage_object::{ObjectMetadata, StorageObject};

pub enum StorageError {
    BuildError,
    GetError,
    ListError,
    DownloadError,
    UploadError,
}

#[async_trait]
pub trait StorageProvider {
    async fn get(&self, bucket: &str, path: &str) -> Result<StorageObject, StorageError>;
    async fn list(&self, bucket: &str) -> Result<Vec<ObjectMetadata>, StorageError>;
    async fn download(&self, bucket: &str, path: &str) -> Result<Vec<u8>, StorageError>;
    async fn upload(&self, bucket: &str, path: &str, data: &[u8]) -> Result<ObjectMetadata, StorageError>;
}

