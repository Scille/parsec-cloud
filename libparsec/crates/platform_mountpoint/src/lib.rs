// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(target_os = "linux")]
mod unix;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub(crate) use unix as platform;
#[cfg(target_os = "windows")]
pub(crate) use windows as platform;

// TODO: UNIX implementation is linux-only for the moment
#[cfg(target_os = "macos")]
pub(crate) mod platform {
    use std::sync::Arc;

    use libparsec_client::WorkspaceOps;
    use libparsec_types::prelude::*;

    #[derive(Debug)]
    pub struct Mountpoint {}

    impl Mountpoint {
        pub fn path(&self) -> &std::path::Path {
            todo!()
        }

        pub fn to_os_path(&self, _parsec_path: &FsPath) -> std::path::PathBuf {
            todo!()
        }

        pub async fn mount(_ops: Arc<WorkspaceOps>) -> anyhow::Result<Self> {
            todo!()
        }

        pub async fn unmount(self) -> anyhow::Result<()> {
            todo!()
        }
    }
}

// TODO: implement for Windows & macOS !
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
#[cfg(test)]
pub(crate) use platform::LOOKUP_HOOK;

pub use platform::Mountpoint;

// TODO: implement for macOS !
#[cfg(not(target_os = "macos"))]
#[cfg(test)]
#[path = "../tests/unit/operations/mod.rs"]
#[allow(clippy::unwrap_used)]
mod operations;
