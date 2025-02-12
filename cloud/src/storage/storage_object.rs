use chrono::{DateTime, Utc};

pub struct ObjectMetadata {
    pub key: String,
    pub size: i64,
    pub last_modified: DateTime<Utc>,
}

pub struct StorageObject {
    pub uri: Option<String>,
    pub key: String,
    pub size: i64,
    pub last_modified: DateTime<Utc>,
    pub version_id: String,
}

