mod config;
mod downloader;

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
    let downloader = downloader::S3Downloader::new().await?;

    // Create download tasks for each artifact
    let download_tasks = config.artifact.iter().map(|(name, artifact)| {
        let downloader = &downloader;
        async move {
            tracing::info!("Downloading artifact: {}", name);
            match downloader
                .download_file(&artifact.bucket, &artifact.key, &artifact.dest)
                .await
            {
                Ok(_) => tracing::info!("Successfully downloaded: {}", name),
                Err(e) => tracing::error!("Failed to download {}: {}", name, e),
            }
        }
    });

    // Execute all downloads in parallel
    join_all(download_tasks).await;

    Ok(())
}
