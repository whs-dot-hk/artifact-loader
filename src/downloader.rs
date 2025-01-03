use aws_sdk_s3::Client;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct S3Downloader {
    client: Client,
}

impl S3Downloader {
    pub async fn new() -> anyhow::Result<Self> {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let client = Client::new(&config);
        Ok(Self { client })
    }

    pub async fn download_file(
        &self,
        bucket: &str,
        key: &str,
        dest: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        let dest = dest.as_ref();

        // Create parent directories if they don't exist
        if let Some(parent) = dest.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Get the object from S3
        let resp = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;

        // Create the destination file
        let mut file = File::create(dest).await?;
        let mut stream = resp.body;

        // Write the data to the file
        while let Some(bytes) = stream.try_next().await? {
            file.write_all(&bytes).await?;
        }
        file.flush().await?;

        Ok(())
    }
}
