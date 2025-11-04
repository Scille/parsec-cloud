// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(target_os = "macos")]
use std::os::macos::fs::MetadataExt;

#[cfg(target_os = "linux")]
use std::os::linux::fs::MetadataExt;

use std::{io::ErrorKind, path::Path, process::Command, sync::Arc, thread::JoinHandle};

use libparsec_client::MountpointMountStrategy;
use libparsec_types::prelude::*;

#[derive(Debug)]
pub struct Mountpoint {
    unmounter: fuser::SessionUnmounter,
    path: std::path::PathBuf,
    session_loop: Option<JoinHandle<std::io::Result<()>>>,
}

impl Mountpoint {
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub fn to_os_path(&self, parsec_path: &FsPath) -> std::path::PathBuf {
        let mut path = self.path.to_owned();
        path.extend(parsec_path.parts().iter().map(|part| part.as_ref()));
        path
    }

    pub async fn mount(
        ops: Arc<libparsec_client::workspace::WorkspaceOps>,
    ) -> anyhow::Result<Self> {
        let (workspace_name, self_role) = ops.get_current_name_and_self_role();
        let is_read_only = !self_role.can_write();
        let filesystem = super::filesystem::Filesystem::new(
            ops.clone(),
            tokio::runtime::Handle::current(),
            is_read_only,
        );
        Self::do_mount(
            &ops.config().mountpoint_mount_strategy,
            filesystem,
            workspace_name,
            is_read_only,
        )
        .await
    }

    pub async fn mount_history(
        ops: Arc<libparsec_client::workspace_history::WorkspaceHistoryOps>,
        mountpoint_name_hint: EntryName,
    ) -> anyhow::Result<Self> {
        let filesystem =
            super::history::Filesystem::new(ops.clone(), tokio::runtime::Handle::current());
        Self::do_mount(
            &ops.config().mountpoint_mount_strategy,
            filesystem,
            mountpoint_name_hint,
            true,
        )
        .await
    }

    async fn do_mount<FS: fuser::Filesystem + Send + 'static>(
        mount_strategy: &MountpointMountStrategy,
        filesystem: FS,
        workspace_name: EntryName,
        is_read_only: bool,
    ) -> anyhow::Result<Self> {
        let mountpoint_base_dir = match mount_strategy {
            MountpointMountStrategy::Directory { base_dir } => base_dir.clone(),
            MountpointMountStrategy::DriveLetter => {
                return Err(anyhow::anyhow!("Mount strategy not supported !"))
            }
            MountpointMountStrategy::Disabled => return Err(anyhow::anyhow!("Mount disabled !")),
        };

        // Mount operation consist of blocking code, so run it in a thread
        tokio::task::spawn_blocking(move || {
            let (mountpoint_path, initial_st_dev) =
                create_suitable_mountpoint_dir(&mountpoint_base_dir, &workspace_name)
                    .context("cannot create mountpoint dir")?;

            let options = [
                fuser::MountOption::FSName("parsec".into()),
                #[cfg(target_os = "macos")]
                fuser::MountOption::CUSTOM(format!("volname={workspace_name}")),
                fuser::MountOption::DefaultPermissions,
                fuser::MountOption::NoSuid,
                fuser::MountOption::Async,
                #[cfg(not(skip_fuse_atime_option))]
                fuser::MountOption::Atime,
                #[cfg(skip_fuse_atime_option)]
                fuser::MountOption::NoAtime,
                fuser::MountOption::Exec,
                // TODO: Should detect and re-mount when the workspace switched between read-only and read-write
                if is_read_only {
                    fuser::MountOption::RO
                } else {
                    fuser::MountOption::RW
                },
                fuser::MountOption::NoDev,
            ];

            let mut session = fuser::Session::new(filesystem, &mountpoint_path, &options)
                .map_err(|err| anyhow::anyhow!("cannot mount: {}", err))?;

            let unmounter = session.unmount_callable();

            let session_loop = std::thread::spawn(move || session.run());

            // Poll the FS to check if the mountpoint has appeared
            // (`st_dev` is the device number of the filesystem, hence it will change after unmounting)
            // Notes:
            // - We only wait for a limited amount of time to avoid ending up in a deadlock
            //   given this part is more of a best-effort mechanism to try to have the mountpoint ready
            //   when our function returns.
            // - Root folder of the workspace is guaranteed to be available in local, so we can
            //   consider the poll operation always resolves fast (i.e. no manifest need to be
            //   fetched from the server).

            for _ in 0..100 {
                println!("polling for the start...");
                if let Ok(new_st_dev) =
                    std::fs::metadata(&mountpoint_path).map(|stat| stat.st_dev())
                {
                    if new_st_dev != initial_st_dev {
                        break;
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(30));
            }
            println!("mountpoint is ready !");

            Ok(Mountpoint {
                unmounter,
                path: mountpoint_path,
                session_loop: Some(session_loop),
            })
        })
        .await
        .context("cannot run mount task")?
    }

    pub async fn unmount(mut self) -> anyhow::Result<()> {
        // Unmount operation consist of blocking code, so run it in a thread
        tokio::task::spawn_blocking(move || {
            // `SessionUnmounter::unmount` is idempotent
            self.unmounter
                .unmount()
                .map_err(|err| anyhow::anyhow!("cannot umount: {}", err))?;

            // Session loop automatically stops once the unmount is called
            if let Some(session_loop) = self.session_loop.take() {
                session_loop
                    .join()
                    .map_err(|err| anyhow::anyhow!("cannot join session loop thread: {:?}", err))?
                    .map_err(|err| anyhow::anyhow!("unexpected error in session loop: {}", err))?;
            }

            // We do the same as WinFSP: remove the directory.
            // (and given this is more of a nice to have feature, we ignore any error)
            let _ = std::fs::remove_dir(&self.path);

            Ok(())
        })
        .await
        .context("cannot run unmount task")?
    }
}

impl Drop for Mountpoint {
    fn drop(&mut self) {
        if self.session_loop.is_some() {
            // `unmount` hasn't been called, the two reasons for that are:
            // - a panic occurred
            // - WorkspaceOps is stopped from libparsec high level API (in that case,
            //   all the workspace's mountpoints are dropped as a fast-close mechanism)
            //
            // In both cases, we must unmount the filesystem in best effort (i.e. we
            // cannot do anything if errors occur).

            // `SessionUnmounter::unmount` is idempotent
            let _ = self.unmounter.unmount();

            // We do the same as WinFSP: remove the directory.
            let _ = std::fs::remove_dir(&self.path);
        }
    }
}

/// Cleanup mountpoint base directory
///
/// This routine ensures the base mountpoint directory does not contain
/// regular empty directories or artifacts from previously mounted workspaces.
///
/// To prevent data loss, non-empty directories are left untouched.
///
/// Essentially, the following is applied to entries in the base mountpoint dir:
/// - Regular file --> skip
/// - Regular empty directory --> remove
/// - Regular non-empty directory --> skip
/// - Directory of a valid mountpoint (empty or not) --> skip
/// - Directory of an invalid mountpoint (artifact) --> unmount + remove
///
/// A directory is considered an artifact when their metadata (stat) cannot be
/// obtained. This is typically the case when Parsec app crashes and workspaces
/// are not properly unmounted.
pub async fn clean_base_mountpoint_dir(
    mountpoint_base_path: std::path::PathBuf,
) -> anyhow::Result<(), libparsec_types::anyhow::Error> {
    log::trace!("Starting clean_base_mountpoint_dir");

    // Check if path is not empty
    if mountpoint_base_path.components().next().is_none() {
        log::error!(
            "Base home dir cleanup, invalid path: {}",
            mountpoint_base_path.display()
        );
        return Ok(());
    }

    tokio::task::spawn_blocking(move || {
        // Obtain metadata for base mountpoint dir and iterate over its entries.
        //
        // Note that if metadata or read fail on base mountpoint dir there is no
        // much we can do so errors are propagated.
        let base_metadata = std::fs::metadata(&mountpoint_base_path)?;
        for entry in std::fs::read_dir(&mountpoint_base_path)?
            .flatten()
        {
            let entry_path = entry.path();
            log::debug!("Base home dir cleanup, processing: {}", entry_path.display());

            let ws_metadata = match std::fs::metadata(&entry_path) {
                Ok(ws_metadata) => ws_metadata,
                Err(_) => {
                    // If dir metadata cannot be obtained, it is most likely
                    // an artifact from a previously mounted workspace
                    // (e.g. Parsec crashed without properly unmounting).
                    // The only way to remove it is to manually force unmount.
                    log::debug!("Base home dir cleanup, unmounting: {}", entry_path.display());
                    if !try_unmount_path(&entry_path) {
                        log::warn!("Failed to unmount path {}, skipping ...", entry_path.display());
                        continue;
                    }

                    // Now that is unmounted, it should be safe to obtain metadata
                    match std::fs::metadata(&entry_path){
                        Ok(ws_metadata) => ws_metadata,
                        Err(err) =>
                        {
                            // Ouch... still an error, let's just skip this directory.
                            log::warn!(
                                "Base home dir cleanup, failed to get metadata after unmount: {} ({err})",
                                entry_path.display()
                            );
                            continue;
                        }
                    }
                }
            };

            // Skip if not a directory
            // NOTE: this filter cannot be applied during the for entry iterator (above)
            //       because is_dir depends on obtaining entry metadata which, as seen
            //       above, will fail on artifacts (and therefore skip them!)
            if !entry_path.is_dir(){
                continue;
            }

            // Only regular directories should be considered (that is, directories that do
            // not correspond to a valid workspace mountpoint!). For that, their device
            // must be equal to the base mountpoint directory.
            if ws_metadata.st_dev() == base_metadata.st_dev() {
                if std::fs::read_dir(&entry_path)?.next().is_none() {
                    // Empty directory, safe to remove it!
                    if let Err(err) = std::fs::remove_dir(&entry_path) {
                        log::error!(
                            "Base home dir cleanup, could not remove empty directory: {} ({err})",
                            entry_path.display(),
                        );
                    }
                } else {
                    // Non-empty directory likely means it was created by the user.
                    // Let's skip it to prevent data loss.
                    log::warn!(
                        "Base home dir cleanup, skipping non-empty directory: {}",
                        entry_path.display(),
                    );
                }
            }
        }
        Ok(())
    })
    .await
    .context("cannot run clean_base_mountpoint_dir task")?
}

/// Find a suitable path to mount the workspace.
///
/// The checks performed here are not atomic (and the mount operation is not
/// itself atomic anyway), hence there are still some edge-cases where the
/// mountpoint can crash due to concurrent changes on the mountpoint path.
fn create_suitable_mountpoint_dir(
    base_mountpoint_path: &std::path::Path,
    workspace_name: &EntryName,
) -> anyhow::Result<(std::path::PathBuf, u64)> {
    log::trace!("Starting create_suitable_mountpoint_dir");
    // In case of hard crash, it's possible the FUSE mountpoint is still mounted
    // but points to nothing. In such case any FS operation on it will fail.
    // For this reason, we cannot consider FS errors as rare & unexpected and let
    // them bubble up. Instead we accept them and try to mount somewhere else.
    macro_rules! ok_or_continue {
        ($outcome:expr) => {
            if let Ok(outcome) = $outcome {
                outcome
            } else {
                continue;
            }
        };
    }

    //`EntryName` format is fully compatible with UNIX path format, so we should always
    // be able to convert workspace name into a single component relative path.
    assert!(
        // `Some(...)` means the path is a relative one with a single component, (while
        // `None` would have meant the path is an absolute one with a single component).
        Path::new(workspace_name.as_ref()).parent() == Some(Path::new("")),
        "Workspace name `{workspace_name:?}` cannot form a valid path item"
    );

    for attempt in 1.. {
        let mountpoint_path = if attempt == 1 {
            base_mountpoint_path.join(workspace_name.as_ref())
        } else {
            base_mountpoint_path.join(format!("{workspace_name} ({attempt})"))
        };

        // On POSIX systems, mounting target must exists
        ok_or_continue!(std::fs::create_dir_all(&mountpoint_path));

        let base_st_dev = ok_or_continue!(std::fs::metadata(base_mountpoint_path)).st_dev();
        let initial_st_dev = ok_or_continue!(std::fs::metadata(&mountpoint_path)).st_dev();

        if initial_st_dev != base_st_dev {
            // mountpoint_path seems to already have a mountpoint on it,
            // hence find another place to setup our own mountpoint
            continue;
        }

        if ok_or_continue!(std::fs::read_dir(&mountpoint_path))
            .next()
            .is_some()
        {
            // mountpoint_path not empty, cannot mount there
            continue;
        }

        return Ok((mountpoint_path, initial_st_dev));
    }

    unreachable!()
}

fn try_unmount_path(path: &Path) -> bool {
    try_unmount_path_using_command("fusermount", &["-u"], path)
        || try_unmount_path_using_command("umount", &[], path)
}

fn try_unmount_path_using_command(command: &str, prefix_args: &[&str], path: &Path) -> bool {
    let display_path = path.display();
    log::debug!("Try unmount {display_path} using `{command}`");
    match Command::new(command).args(prefix_args).arg(path).status() {
        Ok(status) => {
            if status.success() {
                log::debug!("Unmount of {display_path} successful using `{command}`");
                return true;
            } else {
                log::warn!("Bad exit status when trying to unmount {display_path} with `{command}`: {status}")
            }
        }
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                log::debug!(
                    "Failed to unmount {display_path} because `{command}` was not found ({err})"
                );
            } else {
                log::warn!("Failed to unmount {display_path} using `{command}` with error: {err}");
            }
        }
    }
    false
}
