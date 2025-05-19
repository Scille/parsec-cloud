// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use std::fs::{File, OpenOptions};
use std::os::unix::io::AsRawFd;
use std::path::Path;

use super::{TryLockDeviceForUseError, get_lock_file_path};

// Note exclusive lock gets automatically released on file close (i.e. on drop).
#[derive(Debug)]
pub struct InUseDeviceLockGuard(#[allow(unused)] File);

pub fn try_lock_device_for_use(
    config_dir: &Path,
    device_id: DeviceID,
) -> Result<InUseDeviceLockGuard, TryLockDeviceForUseError> {
    let path = get_lock_file_path(config_dir, device_id);

    let fd = open_lock_file(&path).map_err(|err| TryLockDeviceForUseError::Internal(err.into()))?;

    match try_flock_exclusive(&fd) {
        Ok(()) => Ok(InUseDeviceLockGuard(fd)),
        Err(err) if error_is_would_block(&err) => Err(TryLockDeviceForUseError::AlreadyInUse),
        Err(err) => Err(TryLockDeviceForUseError::Internal(err.into())),
    }
}

pub async fn lock_device_for_use(
    config_dir: &Path,
    device_id: DeviceID,
) -> Result<InUseDeviceLockGuard, anyhow::Error> {
    let path = get_lock_file_path(config_dir, device_id);

    let fd = tokio::task::spawn_blocking(move || -> Result<File, anyhow::Error> {
        let fd = open_lock_file(&path).map_err(|err| anyhow::anyhow!(err))?;

        flock_exclusive(&fd).map_err(|err| anyhow::anyhow!(err))?;

        Ok(fd)
    })
    .await??;

    Ok(InUseDeviceLockGuard(fd))
}

fn open_lock_file(path: &Path) -> std::io::Result<File> {
    // Ensure the path to the parent exists
    //
    // Note parent creation and child opening are not one atomic operation, hence
    // it is theoretically possible to have the parent removed before the child open...
    // We take the simple approach here and don't do anything to protect against this
    // here given how unlikely it is to occur.

    let parent_path = path.parent().expect("always in a sub directory");
    std::fs::create_dir_all(parent_path)?;

    // Open the file (creating it if it doesn't already exist)

    let mut opts = OpenOptions::new();
    opts.read(true).write(true).create(true);
    opts.open(path)
}

fn flock(file: &File, flag: libc::c_int) -> std::io::Result<()> {
    // SAFETY: Calling libc flock, where it's provided with a valid file descriptor
    // The lock will be released automatically on file descriptor close (i.e. when
    // the associated `File` is dropped).
    let ret = unsafe { libc::flock(file.as_raw_fd(), flag) };
    if ret < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn flock_exclusive(file: &File) -> std::io::Result<()> {
    flock(file, libc::LOCK_EX)
}

fn try_flock_exclusive(file: &File) -> std::io::Result<()> {
    flock(file, libc::LOCK_EX | libc::LOCK_NB)
}

fn error_is_would_block(err: &std::io::Error) -> bool {
    err.raw_os_error() == Some(libc::EWOULDBLOCK)
}
