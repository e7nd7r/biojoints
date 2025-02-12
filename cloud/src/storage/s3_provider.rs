use std::time::Duration;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{config::SharedCredentialsProvider, presigning::PresigningConfig, primitives::ByteStream, Client};
use aws_smithy_types_convert::date_time::DateTimeExt;
use chrono::{DateTime, Utc};

use super::{provider::{StorageError, StorageProvider}, storage_object::{ObjectMetadata, StorageObject}};

pub struct S3Provider {
    client: Client,
}

fn last_modified_to_chrono_utc(date: &aws_smithy_types::DateTime) -> DateTime<Utc> {
    date.to_chrono_utc().unwrap_or_else(|_| chrono::Utc::now())
}

#[async_trait::async_trait]
impl StorageProvider for S3Provider {
    async fn get(&self, bucket: &str, key: &str) -> Result<StorageObject, StorageError> {
        let expires_in = PresigningConfig::expires_in(Duration::from_secs(3600))
            .map_err(|_| StorageError::GetError)?;

        let presigned = self.client
            .get_object()
            .bucket(bucket)
            .key(key)
            .presigned(expires_in)
            .await
            .map_err(|_| StorageError::GetError)?;

        let result = self.client
            .head_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|_| StorageError::GetError)?;

        let last_modified = result.last_modified().map_or_else(
            chrono::Utc::now,
            last_modified_to_chrono_utc,
        );

        // TODO: Return public URI if acl is public.
        let object = StorageObject {
            uri: Some(presigned.uri().to_string()),
            key: key.to_owned(),
            size: result.content_length().unwrap_or_default(),
            last_modified,
            version_id: result.version_id().unwrap_or_default().to_owned(),
        };

        Ok(object)
    }

    async fn list(&self, bucket: &str) -> Result<Vec<ObjectMetadata>, StorageError> {
        let mut res = self.client.list_objects_v2()
            .bucket(bucket.to_owned())
            .into_paginator()
            .send();

        let mut result = Vec::new();

        while let Some(page) = res.next().await {
            let page = page.map_err(|_| StorageError::ListError)?;

            for object in page.contents() {
                let last_modified = object.last_modified().map_or_else(
                    chrono::Utc::now,
                    last_modified_to_chrono_utc,
                );

                let metadata = ObjectMetadata {
                    key: object.key().unwrap().to_owned(),
                    size: object.size().unwrap_or_default(),
                    last_modified,
                };

                result.push(metadata);
            }
        }

        Ok(result)
    }

    async fn download(&self, bucket: &str, path: &str) -> Result<Vec<u8>, StorageError> {
        let result = self.client
            .get_object()
            .bucket(bucket)
            .key(path)
            .send()
            .await
            .map_err(|_| StorageError::DownloadError)?;

        let data: Vec<u8> = result.body
            .collect().await
            .map_err(|_| StorageError::DownloadError)
            .map(|c| c.into_bytes().to_vec())?;

        Ok(data)
    }

    async fn upload(&self, bucket: &str, key: &str, data: &[u8]) -> Result<ObjectMetadata, StorageError> {
        let stream = ByteStream::from(data.to_vec());

        let result = self.client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(stream)
            .send()
            .await
            .map_err(|_| StorageError::UploadError)?;

        let metadata = ObjectMetadata {
            key: key.to_owned(),
            size: result.size().unwrap_or_default(),
            last_modified: chrono::Utc::now(),
        };

        Ok(metadata)
    }
}

pub struct S3Builder {
    pub endpoint: Option<String>,
    pub region: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
}

impl S3Builder {
    pub fn new() -> S3Builder {
        S3Builder {
            endpoint: None,
            region: None,
            access_key: None,
            secret_key: None, 
        }
    }

    pub async fn region(mut self, region: &str) -> S3Builder {
        self.region = Some(region.to_string());

        self
    }

    pub async fn access_key(mut self, access_key: &str) -> S3Builder {
        self.access_key = Some(access_key.to_string());

        self
    }
    pub async fn secret_key(mut self, secret_key: &str) -> S3Builder {
        self.secret_key = Some(secret_key.to_string());

        self
    }

    async fn with_keys(&self) -> Result<aws_config::SdkConfig, StorageError> {
        let region = self.region.as_ref().ok_or(StorageError::BuildError)?.to_owned();
        let secret_key = self.secret_key.as_ref().ok_or(StorageError::BuildError)?.to_owned();
        let access_key = self.access_key.as_ref().ok_or(StorageError::BuildError)?.to_owned();

        let credential_provider = SharedCredentialsProvider::new(
            aws_sdk_s3::config::Credentials::new(
                access_key,
                secret_key,
                None,
                None,
                "static-provider",
            )
        );

        let mut config_builder = aws_config::load_defaults(BehaviorVersion::latest())
            .await
            .to_builder()
            .region(Region::new(region))
            .credentials_provider(credential_provider);

        // Use default endpoint if not specified
        if let Some(endpoint) = &self.endpoint {
            config_builder = config_builder.endpoint_url(endpoint)
        }

        Ok(config_builder.build())
    }

    pub async fn build(&self) -> Result<S3Provider, StorageError> { 
        let config = match self.with_keys().await {
            Ok(config) => config,
            Err(_) => aws_config::load_from_env().await,
        };

        let client = Client::new(&config);

        let provider = S3Provider {
            client,
        };

        Ok(provider)
    }
}

impl Default for S3Builder {
    fn default() -> Self {
        Self::new()
    }
}

