# Artifact Loader

A utility for downloading artifacts from S3 with hash verification and permission management.

## Config
```toml
[artifact.file1]
type = "s3"
bucket = "my-bucket"
object_key = "my-key"
dest = "my-dest"
# Required: SHA256 hash of the file for verification
hash = "sha256-hash-here"
# Optional: Set file permissions (as decimal, e.g. 755 for rwxr-xr-x)
file_mode = 755
# Optional: Set file owner and group
file_owner = "myuser"
file_group = "mygroup"

[artifact.file2]
type = "s3"
bucket = "my-bucket"
object_key = "my-key2"
dest = "my-dest2"
hash = "sha256-hash-here"
```

## Features
* Download artifacts from S3 to local with hash verification
* Smart download strategy:
  1. If file exists locally, verify its SHA256 hash
  2. Only download from S3 if:
     - File doesn't exist locally, OR
     - Local file's hash doesn't match expected hash
* File permission management (extracted as reusable utility):
  - Set Unix file modes (e.g., 755 for rwxr-xr-x)
  - Set file owner and group permissions
* Run as a oneshot systemd service
* Download artifacts in parallel, retry on failure
* Simple decimal permission format with automatic conversion to Unix modes
* Built with Rust using clap, anyhow, tokio, serde, etc.

## Design Patterns

### S3 Downloader
The S3 downloader follows a simple, efficient pattern:
1. First check if file exists locally
2. If it exists, calculate its SHA256 hash
3. If hash matches expected hash, skip download
4. Only download from S3 when:
   - File doesn't exist locally
   - Local file's hash doesn't match expected hash
5. Permission handling is separate from download logic for reusability

This design ensures:
- Minimal S3 API calls (no checking remote hash first)
- No duplicate downloads
- Efficient streaming of files to disk
- Clear separation between download and permission logic
