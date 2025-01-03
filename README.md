# Config
```toml
[artifact.file1]
type = "s3"
bucket = "my-bucket"
key = "my-key"
dest = "my-dest"

[artifact.file2]
type = "s3"
bucket = "my-bucket"
key = "my-key2"
dest = "my-dest2"
```

# Features
* Download artifacts from s3 to local
* Run as a oneshot systemd service
* Download artifacts in parallel, retry on failure
* Use clap, anyhow, tokio, serde, etc

