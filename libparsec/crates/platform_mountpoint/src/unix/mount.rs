// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{os::linux::fs::MetadataExt, sync::Arc, thread::JoinHandle};

use libparsec_client::WorkspaceOps;
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
        // Mount operation consist of blocking code, so run it in a thread
        tokio::task::spawn_blocking(move || {
            let mountpoint_base_dir = &ops.config().mountpoint_base_dir;

            let (mountpoint_path, initial_st_dev) =
                create_suitable_mountpoint_dir(mountpoint_base_dir, &ops)
                    .context("cannot create mountpoint dir")?;

            let filesystem =
                super::filesystem::Filesystem::new(ops, tokio::runtime::Handle::current());
            let options = [
                fuser::MountOption::FSName("parsec".into()),
                fuser::MountOption::DefaultPermissions,
                fuser::MountOption::NoSuid,
                fuser::MountOption::Async,
                fuser::MountOption::Atime,
                fuser::MountOption::Exec,
                fuser::MountOption::RW,
                fuser::MountOption::NoDev,
            ];

            let mut session = fuser::Session::new(filesystem, &mountpoint_path, &options)
                .map_err(|err| anyhow::anyhow!("cannot mount: {}", err))?;

            let unmounter = session.unmount_callable();

            let session_loop = std::thread::spawn(move || session.run());

            // Poll the FS to check if the mountpoint has appeared
            // (`st_dev` is the device number of the filesystem, hence it will change after unmounting)
            // Note we only wait for a limited amount of time to avoid ending up in a deadlock
            // given this part is more of a best-effort mechanism to try to have the mountpoint ready
            // when our function returns.
            for _ in 0..100 {
                if let Ok(new_st_dev) =
                    std::fs::metadata(&mountpoint_path).map(|stat| stat.st_dev())
                {
                    if new_st_dev != initial_st_dev {
                        break;
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(30));
            }

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

// Find a suitable path where to mount the workspace. The check we are doing
// here are not atomic (and the mount operation is not itself atomic anyway),
// hence there is still edge-cases where the mount can crash due to concurrent
// changes on the mountpoint path.
fn create_suitable_mountpoint_dir(
    base_mountpoint_path: &std::path::Path,
    ops: &WorkspaceOps,
) -> anyhow::Result<(std::path::PathBuf, u64)> {
    let workspace_name = ops.name();

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

    for tentative in 1.. {
        let mountpoint_path = if tentative == 1 {
            base_mountpoint_path.join(workspace_name.as_ref())
        } else {
            base_mountpoint_path.join(format!("{} ({})", workspace_name.as_ref(), tentative))
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
