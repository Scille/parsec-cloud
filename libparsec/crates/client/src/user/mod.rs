// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod merge;
mod store;
mod sync;

pub use store::*;
pub use sync::*;

use std::sync::Arc;

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_types::prelude::*;

use self::store::{UserForUpdateLocalWorkspacesUpdater, UserStore};
use crate::{certif::CertifOps, event_bus::EventBus, ClientConfig};

#[derive(Debug)]
pub struct UserOps {
    device: Arc<LocalDevice>,
    store: UserStore,
    cmds: Arc<AuthenticatedCmds>,
    certificates_ops: Arc<CertifOps>,
    event_bus: EventBus,
}

// For readability, we define the public interface here and let the actual
// implementation in separated submodules
impl UserOps {
    pub(crate) async fn start(
        config: &ClientConfig,
        device: Arc<LocalDevice>,
        cmds: Arc<AuthenticatedCmds>,
        certificates_ops: Arc<CertifOps>,
        event_bus: EventBus,
    ) -> Result<Self, anyhow::Error> {
        let store = UserStore::start(&config.data_base_dir, device.clone()).await?;
        Ok(Self {
            device,
            store,
            cmds,
            certificates_ops,
            event_bus,
        })
    }

    pub(crate) async fn stop(&self) -> anyhow::Result<()> {
        self.store.stop().await
    }

    pub(crate) fn get_user_manifest(&self) -> Arc<LocalUserManifest> {
        self.store.get_user_manifest()
    }

    /// As its name implies, local cache is never synchronized. Hence this method that
    /// won't update the `need_sync` and `updated` fields of the local user manifest,
    /// nor will trigger a synchronization events.
    pub(crate) async fn for_update_local_workspaces(&self) -> UserForUpdateLocalWorkspacesUpdater {
        self.store.for_update_local_workspaces().await
    }

    /// For test purpose: currently user manifest doesn't need to be synced, hence
    /// this method that forces the need for sync.
    #[cfg(test)]
    pub(crate) async fn test_bump_updated_and_need_sync(&self) -> Result<(), UserStoreUpdateError> {
        let (updater, mut user_manifest) = self.store.for_update().await;
        {
            // `Arc::make_mut` clones user manifest before we modify it
            let user_manifest = Arc::make_mut(&mut user_manifest);
            user_manifest.need_sync = true;
            user_manifest.updated = self.device.now();
        }
        updater.set_user_manifest(user_manifest).await
    }

    /// Note user realm synchronization is much simpler than what is done for workspace:
    /// given the user realm is not meant to be shared and has no name, we never
    /// use `RealmNameCertificate` and `RealmKeyRotationCertificate`. Hence we only
    /// have to make sure the realm is created, and then update the vlob containing
    /// the user manifest.
    #[allow(unused)]
    pub(crate) async fn sync(&self) -> Result<(), UserSyncError> {
        sync::sync(self).await
    }

    pub fn realm_id(&self) -> VlobID {
        self.device.user_realm_id
    }
}

#[cfg(test)]
#[path = "../../tests/unit/user/mod.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
