use aws_sdk_s3::Client;
use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct S3Downloader {}

impl S3Downloader {
    pub fn new() -> Self {
        Self {}
    }

    async fn calculate_hash(path: impl AsRef<Path>) -> anyhow::Result<String> {
        let mut file = File::open(path).await?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192]; // 8KB buffer for efficient reading

        loop {
            let n = file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        Ok(hex::encode(hasher.finalize()))
    }

    pub async fn download_file(
        &self,
        bucket: &str,
        key: &str,
        dest: impl AsRef<Path>,
        expected_hash: &str,
    ) -> anyhow::Result<bool> {
        let dest = dest.as_ref();

        // If file exists, check its hash
        if dest.exists() {
            match Self::calculate_hash(dest).await {
                Ok(local_hash) => {
                    if local_hash == expected_hash {
                        tracing::info!("Local file hash matches. Skipping download.");
                        return Ok(false);
                    }
                    tracing::info!("Local file hash differs. Will download from S3.");
                }
                Err(e) => {
                    tracing::warn!("Failed to calculate local file hash: {}. Will download.", e);
                }
            }
        }

        // Create parent directories if they don't exist
        if let Some(parent) = dest.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Create S3 client
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let client = Client::new(&config);

        // Download the file
        let resp = client.get_object().bucket(bucket).key(key).send().await?;

        // Stream the file to disk
        let mut stream = resp.body;
        let mut file = File::create(dest).await?;

        while let Some(bytes) = stream.try_next().await? {
            file.write_all(&bytes).await?;
        }
        file.flush().await?;

        // Calculate hash after download
        let downloaded_hash = Self::calculate_hash(dest).await?;
        if downloaded_hash != expected_hash {
            return Err(anyhow::anyhow!(
                "Hash mismatch after download. Expected: {}, Got: {}",
                expected_hash,
                downloaded_hash
            ));
        }

        Ok(true)
    }
}
