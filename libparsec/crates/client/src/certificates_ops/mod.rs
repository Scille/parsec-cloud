// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod add;
mod encrypt;
mod list;
mod poll;
mod realm_creation;
mod store;
mod validate_manifest;
mod validate_message;

pub use add::{AddCertificateError, InvalidCertificateError, MaybeRedactedSwitch};
pub use list::{DeviceInfo, GetUserDeviceError, UserInfo, WorkspaceUserAccessInfo};
pub use poll::PollServerError;
pub use realm_creation::EnsureRealmsCreatedError;
pub use store::{GetCertificateError, UpTo};
pub use validate_manifest::{InvalidManifestError, ValidateManifestError};
pub use validate_message::{InvalidMessageError, ValidateMessageError};

use std::{collections::HashMap, sync::Arc};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_types::prelude::*;

use crate::{event_bus::EventBus, ClientConfig};

#[derive(Debug)]
pub struct CertificatesOps {
    #[allow(unused)]
    config: Arc<ClientConfig>,
    device: Arc<LocalDevice>,
    event_bus: EventBus,
    cmds: Arc<AuthenticatedCmds>,
    store: store::CertificatesStore,
}

// For readability, we define the public interface here and let the actual
// implementation in separated submodules
impl CertificatesOps {
    /*
     * Crate-only interface (used by client, opses and monitors)
     */

    pub(crate) async fn start(
        config: Arc<ClientConfig>,
        device: Arc<LocalDevice>,
        event_bus: EventBus,
        cmds: Arc<AuthenticatedCmds>,
    ) -> anyhow::Result<Self> {
        let store = store::CertificatesStore::start(&config.data_base_dir, device.clone()).await?;
        Ok(Self {
            config,
            device,
            event_bus,
            cmds,
            store,
        })
    }

    /// Stop the underlying storage (and flush whatever data is not yet on disk)
    ///
    /// Once stopped, it can still theoretically be used (i.e. `stop` doesn't
    /// consume `self`), but will do nothing but return stopped error.
    pub(crate) async fn stop(&self) {
        self.store.stop().await;
    }

    // For readability, we define the public interface here and let the actual
    // implementation in separated submodules

    pub(crate) async fn poll_server_for_new_certificates(
        &self,
        latest_known_index: Option<IndexInt>,
    ) -> Result<IndexInt, PollServerError> {
        poll::poll_server_for_new_certificates(self, latest_known_index).await
    }

    pub(crate) async fn validate_message(
        &self,
        certificate_index: IndexInt,
        index: IndexInt,
        sender: &DeviceID,
        timestamp: DateTime,
        encrypted: &[u8],
    ) -> Result<MessageContent, ValidateMessageError> {
        validate_message::validate_message(
            self,
            certificate_index,
            index,
            sender,
            timestamp,
            encrypted,
        )
        .await
    }

    pub(crate) async fn validate_user_manifest(
        &self,
        certificate_index: IndexInt,
        author: &DeviceID,
        version: VersionInt,
        timestamp: DateTime,
        encrypted: &[u8],
    ) -> Result<UserManifest, ValidateManifestError> {
        validate_manifest::validate_user_manifest(
            self,
            certificate_index,
            author,
            version,
            timestamp,
            encrypted,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn validate_workspace_manifest(
        &self,
        realm_id: VlobID,
        realm_key: &SecretKey,
        certificate_index: IndexInt,
        author: &DeviceID,
        version: VersionInt,
        timestamp: DateTime,
        encrypted: &[u8],
    ) -> Result<WorkspaceManifest, ValidateManifestError> {
        validate_manifest::validate_workspace_manifest(
            self,
            realm_id,
            realm_key,
            certificate_index,
            author,
            version,
            timestamp,
            encrypted,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn validate_child_manifest(
        &self,
        realm_id: VlobID,
        realm_key: &SecretKey,
        vlob_id: VlobID,
        certificate_index: IndexInt,
        author: &DeviceID,
        version: VersionInt,
        timestamp: DateTime,
        encrypted: &[u8],
    ) -> Result<ChildManifest, ValidateManifestError> {
        validate_manifest::validate_child_manifest(
            self,
            realm_id,
            realm_key,
            vlob_id,
            certificate_index,
            author,
            version,
            timestamp,
            encrypted,
        )
        .await
    }

    pub(crate) async fn encrypt_for_user(
        &self,
        user_id: UserID,
        data: &[u8],
    ) -> anyhow::Result<Option<Vec<u8>>> {
        encrypt::encrypt_for_user(self, user_id, data).await
    }

    pub(crate) async fn encrypt_for_sequester_services(
        &self,
        data: &[u8],
    ) -> anyhow::Result<Option<HashMap<SequesterServiceID, Bytes>>> {
        encrypt::encrypt_for_sequester_services(self, data).await
    }

    /*
     * Public interface
     */

    pub async fn get_current_self_profile(&self) -> anyhow::Result<UserProfile> {
        list::get_current_self_profile(self).await
    }

    /// List all realms we are and used to be part of, along with the role
    /// currently have in them
    pub async fn get_current_self_realms_roles(
        &self,
    ) -> anyhow::Result<HashMap<VlobID, Option<RealmRole>>> {
        list::get_current_self_realms_roles(self).await
    }

    pub async fn list_users(
        &self,
        skip_revoked: bool,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> anyhow::Result<Vec<UserInfo>> {
        list::list_users(self, skip_revoked, offset, limit).await
    }

    pub async fn list_user_devices(&self, user_id: UserID) -> anyhow::Result<Vec<DeviceInfo>> {
        list::list_user_devices(self, user_id).await
    }

    pub async fn get_user_device(
        &self,
        device_id: DeviceID,
    ) -> Result<(UserInfo, DeviceInfo), GetUserDeviceError> {
        list::get_user_device(self, device_id).await
    }

    /// List users currently part of the given workspace (i.e. user not revoked
    /// and with a valid role)
    pub async fn list_workspace_users(
        &self,
        realm_id: VlobID,
    ) -> anyhow::Result<Vec<WorkspaceUserAccessInfo>> {
        list::list_workspace_users(self, realm_id).await
    }

    /// Ensure the realm exists on the server and hence can be shared and synced.
    ///
    /// Only realm creation is needed for using a workspace: given each device
    /// starts using the workspace by creating a speculative placeholder workspace
    /// manifest, any of them could sync it placeholder which would become the
    /// initial workspace manifest.
    ///
    /// (Note Parsec <= 2.4.2 used to download the workspace manifest instead
    /// of starting with a speculative placeholder so it could be an issue here).
    pub async fn ensure_realms_created(
        &self,
        realms_ids: &[VlobID],
    ) -> Result<(), EnsureRealmsCreatedError> {
        realm_creation::ensure_realms_created(self, realms_ids).await
    }
}
