// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod add;
mod block_validate;
mod encrypt;
mod list;
mod manifest_validate;
mod poll;
mod realm_create;
mod realm_decrypt_name;
mod realm_key_rotation;
mod realm_keys_bundle;
mod realm_rename;
mod realm_share;
mod store;
mod user_revoke;
mod workspace_bootstrap;

pub use add::{CertifAddCertificatesBatchError, InvalidCertificateError, MaybeRedactedSwitch};
pub use block_validate::{CertifValidateBlockError, InvalidBlockAccessError};
pub use encrypt::CertifEncryptForSequesterServicesError;
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
pub use list::{
    CertifGetCurrentSelfProfileError, CertifGetCurrentSelfRealmRoleError,
    CertifGetCurrentSelfRealmsRoleError, CertifGetUserDeviceError, CertifListUserDevicesError,
    CertifListUsersError, CertifListWorkspaceUsersError, DeviceInfo, UserInfo,
    WorkspaceUserAccessInfo,
};
pub use manifest_validate::{CertifValidateManifestError, InvalidManifestError};
pub use poll::CertifPollServerError;
pub use realm_create::CertifEnsureRealmCreatedError;
pub use realm_decrypt_name::{CertifDecryptCurrentRealmNameError, InvalidEncryptedRealmNameError};
pub use realm_key_rotation::CertifRotateRealmKeyError;
pub use realm_keys_bundle::{CertifEncryptForRealmError, InvalidKeysBundleError};
pub use realm_rename::CertifRenameRealmError;
pub use realm_share::CertifShareRealmError;
pub use store::{CertifStoreError, UpTo};
pub use user_revoke::CertifRevokeUserError;
pub use workspace_bootstrap::CertifBootstrapWorkspaceError;

use std::sync::Arc;

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_types::prelude::*;

use crate::{event_bus::EventBus, ClientConfig};

#[derive(Debug)]
pub enum CertificateBasedActionOutcome {
    /// The action was already done according to the certificates we have locally,
    /// hence no need certificate was uploaded.
    LocalIdempotent,
    /// Our local certificates were lagging behind, but the server told us the
    /// action was already done on it side. Hence a certificate poll is needed.
    RemoteIdempotent { certificate_timestamp: DateTime },
    /// The action was done by uploading a certificate on the server.
    /// We should now trigger a certificate poll to add our new certificate
    /// to the our local storage.
    /// Note the timestamp param is useful to detect if the poll was already
    /// concurrently done (e.g. when doing a realm rename we will do a poll
    /// as part of the rename, and another poll will be done by the certificates
    /// monitor as soon as it gets an event from the server about the new certificate).
    Uploaded { certificate_timestamp: DateTime },
}

#[derive(Debug)]
pub(crate) struct CertifOps {
    #[allow(unused)]
    config: Arc<ClientConfig>,
    device: Arc<LocalDevice>,
    event_bus: EventBus,
    cmds: Arc<AuthenticatedCmds>,
    store: store::CertificatesStore,
}

// For readability, we define the public interface here and let the actual
// implementation in separated submodules
impl CertifOps {
    pub async fn start(
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
    /// consume `self`), but will only returns stopped error.
    pub async fn stop(&self) -> anyhow::Result<()> {
        self.store.stop().await
    }

    /// Only a test helper: in production `poll::poll_server_for_new_certificates` is
    /// used instead (which deals with both fetching certificates and storing them).
    #[cfg(test)]
    pub async fn add_certificates_batch(
        &self,
        common_certificates: &[Bytes],
        sequester_certificates: &[Bytes],
        shamir_recovery_certificates: &[Bytes],
        realm_certificates: &std::collections::HashMap<VlobID, Vec<Bytes>>,
    ) -> Result<MaybeRedactedSwitch, CertifAddCertificatesBatchError> {
        self.store
            .for_write(move |store| async move {
                add::add_certificates_batch(
                    self,
                    store,
                    common_certificates,
                    sequester_certificates,
                    shamir_recovery_certificates,
                    realm_certificates,
                )
                .await
            })
            .await
            .map_err(|err| match err {
                // We don't bother to have a dedicated error for stopped here given
                // this is only a test helper.
                CertifStoreError::Stopped => {
                    CertifAddCertificatesBatchError::Internal(anyhow::anyhow!("Storage stopped"))
                }
                CertifStoreError::Internal(err) => err.into(),
            })?
    }

    // /// This is a test helper: realm keys bundle is supposed to get loaded lazily,
    // /// which involves a server request.
    // /// But This is cumbersome to handle in tests though (given the request must
    // /// be mocked, and the load only occurs when the keys bundle is needed,
    // /// i.e. not when the test doing setup stuff but when the thing being tested
    // /// is run).
    // ///
    // /// Typical usage:
    // /// ```
    // ///     user_ops.certificates_ops.test_inject_loaded_realm_keys(
    // ///         env.get_last_realm_keys_for(wksp1_id, "alice"),
    // ///     ).await.unwrap();
    // /// ```
    // #[cfg(test)]
    // pub async fn test_inject_loaded_realm_keys(
    //     &self,
    //     keys: Arc<RealmKeys>,
    // ) -> anyhow::Result<()> {
    //     use crate::certif::realm_keys_bundle::RealmKeys;

    //     self.store.for_read(|store| async move {
    //         let keys = Arc::new(RealmKeys {
    //             realm_id,
    //             keys
    //             keys_bundle_access_key: keys_bundle_access,
    //         });
    //         store.update_cache_for_realm_keys(realm_id, info)
    //     }).await
    //     todo!()
    // }

    // For readability, we define the public interface here and let the actual
    // implementation in separated submodules

    pub async fn poll_server_for_new_certificates(
        &self,
        latest_known_timestamps: Option<&PerTopicLastTimestamps>,
    ) -> Result<(), CertifPollServerError> {
        self.store
            .for_write(|store| async move {
                poll::poll_server_for_new_certificates(self, store, latest_known_timestamps).await
            })
            .await?
    }

    pub async fn validate_user_manifest(
        &self,
        needed_realm_certificate_timestamp: DateTime,
        needed_common_certificate_timestamp: DateTime,
        author: &DeviceID,
        version: VersionInt,
        timestamp: DateTime,
        encrypted: &[u8],
    ) -> Result<UserManifest, CertifValidateManifestError> {
        manifest_validate::validate_user_manifest(
            self,
            needed_realm_certificate_timestamp,
            needed_common_certificate_timestamp,
            author,
            version,
            timestamp,
            encrypted,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn validate_workspace_manifest(
        &self,
        needed_realm_certificate_timestamp: DateTime,
        needed_common_certificate_timestamp: DateTime,
        realm_id: VlobID,
        key_index: IndexInt,
        author: &DeviceID,
        version: VersionInt,
        timestamp: DateTime,
        encrypted: &[u8],
    ) -> Result<WorkspaceManifest, CertifValidateManifestError> {
        manifest_validate::validate_workspace_manifest(
            self,
            needed_realm_certificate_timestamp,
            needed_common_certificate_timestamp,
            realm_id,
            key_index,
            author,
            version,
            timestamp,
            encrypted,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn validate_child_manifest(
        &self,
        needed_realm_certificate_timestamp: DateTime,
        needed_common_certificate_timestamp: DateTime,
        realm_id: VlobID,
        key_index: IndexInt,
        vlob_id: VlobID,
        author: &DeviceID,
        version: VersionInt,
        timestamp: DateTime,
        encrypted: &[u8],
    ) -> Result<ChildManifest, CertifValidateManifestError> {
        manifest_validate::validate_child_manifest(
            self,
            needed_realm_certificate_timestamp,
            needed_common_certificate_timestamp,
            realm_id,
            key_index,
            vlob_id,
            author,
            version,
            timestamp,
            encrypted,
        )
        .await
    }

    pub async fn validate_block(
        &self,
        realm_id: VlobID,
        manifest: &FileManifest,
        access: &BlockAccess,
        encrypted: &[u8],
    ) -> Result<Vec<u8>, CertifValidateBlockError> {
        block_validate::validate_block(self, realm_id, manifest, access, encrypted).await
    }

    /// Encrypt the data with the last known key from the most recent realm keys bundle.
    ///
    /// Be aware this function potentially do server accesses (to fetch the keys bundle).
    pub async fn encrypt_for_realm(
        &self,
        realm_id: VlobID,
        data: &[u8],
    ) -> Result<(Vec<u8>, IndexInt), CertifEncryptForRealmError> {
        self.store
            .for_read(|store| async move {
                realm_keys_bundle::encrypt_for_realm(self, store, realm_id, data).await
            })
            .await
            .map_err(|e| match e {
                CertifStoreError::Stopped => CertifEncryptForRealmError::Stopped,
                CertifStoreError::Internal(_) => todo!(),
            })?
    }

    pub async fn encrypt_for_sequester_services(
        &self,
        data: &[u8],
    ) -> Result<Option<Vec<(SequesterServiceID, Bytes)>>, CertifEncryptForSequesterServicesError>
    {
        self.store
            .for_read(
                |store| async move { encrypt::encrypt_for_sequester_services(store, data).await },
            )
            .await
            .map_err(|e| match e {
                CertifStoreError::Stopped => CertifEncryptForSequesterServicesError::Stopped,
                CertifStoreError::Internal(err) => err.context("Cannot access storage").into(),
            })?
    }

    pub async fn revoke_user(
        &self,
        user: UserID,
    ) -> Result<CertificateBasedActionOutcome, CertifRevokeUserError> {
        user_revoke::revoke_user(self, user).await
    }

    /// Returns the timestamp of the uploaded certificate
    pub async fn rename_realm(
        &self,
        realm_id: VlobID,
        new_name: EntryName,
    ) -> Result<CertificateBasedActionOutcome, CertifRenameRealmError> {
        realm_rename::rename_realm(self, realm_id, new_name).await
    }

    /// Returns the timestamp of the uploaded certificate
    pub async fn share_realm(
        &self,
        realm_id: VlobID,
        recipient: UserID,
        role: Option<RealmRole>,
    ) -> Result<CertificateBasedActionOutcome, CertifShareRealmError> {
        realm_share::share_realm(self, realm_id, recipient, role).await
    }

    // TODO: determine this API when implementing the related monitor
    // pub async fn rotate_realm_key(
    //     &self,
    //     realm_id: VlobID,
    // ) -> Result<CertificateBasedActionOutcome, CertifRotateRealmKeyError> {
    //     realm_key_rotation::rotate_realm_key(self, realm_id).await
    // }

    /// Bootstrap the workspace in an idempotent way, i.e. ensure the realm
    /// exists on the server and that it can be shared with other users.
    ///
    /// Note we are talking of "workspace" and not "realm" here: this is because
    /// bootstrapping the user realm is meaningless (and never done in practice)
    /// since it is never shared, nor have the need for reencryption & naming.
    pub async fn bootstrap_workspace(
        &self,
        realm_id: VlobID,
        name: &EntryName,
    ) -> Result<CertificateBasedActionOutcome, CertifBootstrapWorkspaceError> {
        workspace_bootstrap::bootstrap_workspace(self, realm_id, name).await
    }

    /// Should only be used for the user realm, as it is the only one that doesn't
    /// need a full bootstrap (user realm is encrypted by the user's private key
    /// and doesn't need a name).
    pub async fn ensure_realm_created(
        &self,
        realm_id: VlobID,
    ) -> Result<CertificateBasedActionOutcome, CertifEnsureRealmCreatedError> {
        realm_create::ensure_realm_created(self, realm_id).await
    }

    pub async fn get_current_self_profile(
        &self,
    ) -> Result<UserProfile, CertifGetCurrentSelfProfileError> {
        list::get_current_self_profile(self).await
    }

    /// List all realms we are and used to be part of, along with the role currently
    /// have in them and the timestamp of the certificate this info comes from.
    pub async fn get_current_self_realms_role(
        &self,
    ) -> Result<Vec<(VlobID, Option<RealmRole>, DateTime)>, CertifGetCurrentSelfRealmsRoleError>
    {
        list::get_current_self_realms_role(self).await
    }

    pub async fn get_current_self_realm_role(
        &self,
        realm_id: VlobID,
    ) -> Result<Option<Option<RealmRole>>, CertifGetCurrentSelfRealmRoleError> {
        list::get_current_self_realm_role(self, realm_id).await
    }

    pub async fn list_users(
        &self,
        skip_revoked: bool,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Vec<UserInfo>, CertifListUsersError> {
        list::list_users(self, skip_revoked, offset, limit).await
    }

    pub async fn list_user_devices(
        &self,
        user_id: UserID,
    ) -> Result<Vec<DeviceInfo>, CertifListUserDevicesError> {
        list::list_user_devices(self, user_id).await
    }

    pub async fn get_user_device(
        &self,
        device_id: DeviceID,
    ) -> Result<(UserInfo, DeviceInfo), CertifGetUserDeviceError> {
        list::get_user_device(self, device_id).await
    }

    /// List users currently part of the given workspace (i.e. user not revoked
    /// and with a valid role)
    pub async fn list_workspace_users(
        &self,
        realm_id: VlobID,
    ) -> Result<Vec<WorkspaceUserAccessInfo>, CertifListWorkspaceUsersError> {
        list::list_workspace_users(self, realm_id).await
    }

    /// Retrieve the realm name from the last realm name certificate, if any).
    /// Returns the name along with the timestamp of certificate it comes from.
    /// Be aware this function potentially do server accesses (to fetch the keys bundle).
    pub async fn decrypt_current_realm_name(
        &self,
        realm_id: VlobID,
    ) -> Result<(EntryName, DateTime), CertifDecryptCurrentRealmNameError> {
        realm_decrypt_name::decrypt_current_realm_name(self, realm_id).await
    }
}

#[cfg(test)]
#[path = "../../tests/unit/certif/mod.rs"]
#[allow(clippy::unwrap_used)]
mod certif_test;
