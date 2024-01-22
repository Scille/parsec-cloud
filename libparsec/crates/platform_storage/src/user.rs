// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! User storage is only responsible for storing/fetching encrypted items:
//! - No cache&concurrency is handled at this level: better let the higher level
//!   components do that since they have a better idea of what should be cached.
//! - No encryption/serialization: simplify code (no need for type dispatching)
//!   and testing (no need to build valid certificates objects.

use std::path::Path;

use libparsec_types::prelude::*;

use crate::platform::user::PlatformUserStorage;

// Re-expose `user_storage_non_speculative_init`
pub use crate::platform::user::user_storage_non_speculative_init;

#[derive(Debug)]
pub struct UserStorage {
    platform: PlatformUserStorage,
}

impl UserStorage {
    pub async fn start(data_base_dir: &Path, device: &LocalDevice) -> anyhow::Result<Self> {
        // `maybe_populate_certificate_storage` needs to start a `CertificatesStorage`,
        // leading to a recursive call which is not supported for async functions.
        // Hence `no_populate_start` which breaks the recursion.
        //
        // Also note we don't try to return the `CertificatesStorage` that has been
        // used during the populate as it would change the internal state of the
        // storage (typically caches) depending of if populate has been needed or not.

        #[cfg(feature = "test-with-testbed")]
        crate::testbed::maybe_populate_user_storage(data_base_dir, device).await;

        Self::no_populate_start(data_base_dir, device).await
    }

    pub(crate) async fn no_populate_start(
        data_base_dir: &Path,
        device: &LocalDevice,
    ) -> anyhow::Result<Self> {
        let platform = PlatformUserStorage::no_populate_start(data_base_dir, device).await?;
        Ok(Self { platform })
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        self.platform.stop().await
    }

    pub async fn get_realm_checkpoint(&mut self) -> anyhow::Result<IndexInt> {
        self.platform.get_realm_checkpoint().await
    }

    pub async fn update_realm_checkpoint(
        &mut self,
        new_checkpoint: IndexInt,
        remote_user_manifest_version: Option<VersionInt>,
    ) -> anyhow::Result<()> {
        self.platform
            .update_realm_checkpoint(new_checkpoint, remote_user_manifest_version)
            .await
    }

    pub async fn get_user_manifest(&mut self) -> anyhow::Result<Option<Vec<u8>>> {
        self.platform.get_user_manifest().await
    }

    pub async fn update_user_manifest(
        &mut self,
        encrypted: &[u8],
        need_sync: bool,
        base_version: VersionInt,
    ) -> anyhow::Result<()> {
        self.platform
            .update_user_manifest(encrypted, need_sync, base_version)
            .await
    }

    /// Only used for debugging tests
    #[allow(unused)]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        self.platform.debug_dump().await
    }
}

#[cfg(test)]
#[path = "../tests/unit/user.rs"]
mod test;
