// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

mod filesystem;
mod history;
mod inode;
mod mount;

#[cfg(test)]
pub(crate) use filesystem::LOOKUP_HOOK;

pub use mount::Mountpoint;
pub use mount::clean_base_mountpoint_dir;

/// Static value that should be initialized during `Filesystem::init`.
/// Used during offline tests to determine the expected error value that depends on the ABI version.
#[cfg(test)]
pub(crate) static FUSE_ABI_VERSION: std::sync::OnceLock<fuser::Version> =
    std::sync::OnceLock::new();
