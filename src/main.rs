mod config;
mod downloader;
mod utils;

use crate::utils::set_file_permissions;
use clap::Parser;
use futures::future::join_all;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let args = Args::parse();

    // Load configuration
    let config = config::Config::from_file(args.config)?;

    // Initialize S3 downloader
    let downloader = downloader::S3Downloader::new();

    // Create download tasks for each artifact
    let download_tasks = config.artifact.iter().map(|(name, artifact)| {
        let downloader = &downloader;
        async move {
            tracing::info!("Downloading artifact: {}", name);

            // Download file
            match downloader
                .download_file(
                    &artifact.bucket,
                    &artifact.object_key,
                    &artifact.dest,
                    &artifact.hash,
                    artifact.region.as_deref(),
                )
                .await
            {
                Ok(downloaded) => {
                    if downloaded {
                        // Set file permissions after successful download
                        if let Err(e) = set_file_permissions(
                            &artifact.dest,
                            artifact.file_mode,
                            artifact.file_owner.as_deref(),
                            artifact.file_group.as_deref(),
                        )
                        .await
                        {
                            tracing::error!("Failed to set permissions for {}: {}", name, e);
                            return;
                        }
                    }
                    tracing::info!("Successfully processed: {}", name);
                }
                Err(e) => tracing::error!("Failed to download {}: {}", name, e),
            }
        }
    });

    // Execute all downloads in parallel
    join_all(download_tasks).await;

    Ok(())
}
