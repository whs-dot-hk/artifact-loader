use nix::unistd::{self, Group, User};
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub async fn set_file_permissions(
    path: impl AsRef<Path>,
    file_mode: u32,
    file_owner: Option<&str>,
    file_group: Option<&str>,
) -> anyhow::Result<()> {
    let path = path.as_ref();

    // Set file permissions
    tokio::fs::set_permissions(path, Permissions::from_mode(file_mode)).await?;

    // Get user and group IDs
    let uid = if let Some(owner) = file_owner {
        match User::from_name(owner)? {
            Some(user) => Some(user.uid),
            None => {
                tracing::warn!(
                    "Specified user '{}' not found, skipping user ownership",
                    owner
                );
                None
            }
        }
    } else {
        None
    };

    let gid = if let Some(group) = file_group {
        match Group::from_name(group)? {
            Some(group) => Some(group.gid),
            None => {
                tracing::warn!(
                    "Specified group '{}' not found, skipping group ownership",
                    group
                );
                None
            }
        }
    } else {
        None
    };

    // Set ownership if either uid or gid is specified
    if uid.is_some() || gid.is_some() {
        unistd::chown(path, uid, gid)?;
    }

    Ok(())
}
