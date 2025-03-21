// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! User storage is only responsible for storing/fetching encrypted items:
//! - No cache&concurrency is handled at this level: better let the higher level
//!   components do that since they have a better idea of what should be cached.
//! - No encryption/serialization: simplify code (no need for type dispatching)
//!   and testing (no need to build valid certificates objects.

use std::path::Path;

use libparsec_types::prelude::*;

use crate::platform::user::PlatformUserStorage;

#[derive(Debug)]
pub struct UserStorage {
    platform: PlatformUserStorage,
}

impl UserStorage {
    pub async fn start(data_base_dir: &Path, device: &LocalDevice) -> anyhow::Result<Self> {
        // `maybe_populate_certificate_storage` needs to start a `UserStorage`,
        // leading to a recursive call which is not supported for async functions.
        // Hence `no_populate_start` which breaks the recursion.
        //
        // Also note we don't try to return the `UserStorage` that has been
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
    #[cfg(feature = "expose-test-methods")]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        self.platform.debug_dump().await
    }
}

pub async fn user_storage_non_speculative_init(
    data_base_dir: &Path,
    device: &LocalDevice,
) -> anyhow::Result<()> {
    // 1) Open & initialize the database

    #[cfg(feature = "test-with-testbed")]
    crate::testbed::mark_as_populated_user_storage(data_base_dir, device).await;

    let mut storage = PlatformUserStorage::no_populate_start(data_base_dir, device).await?;

    // 2) Populate the database with the user manifest

    let timestamp = device.now();
    let manifest = LocalUserManifest::new(
        device.device_id,
        timestamp,
        Some(device.user_realm_id),
        false,
    );
    let encrypted = manifest.dump_and_encrypt(&device.local_symkey);

    storage
        .update_user_manifest(&encrypted, manifest.need_sync, manifest.base.version)
        .await?;

    // 4) All done ! Don't forget to close the database before exiting ;-)

    storage.stop().await?;

    Ok(())
}

#[cfg(test)]
#[path = "../tests/unit/user.rs"]
mod test;
