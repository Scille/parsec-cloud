// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, num::NonZeroU8, sync::Arc};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_types::prelude::*;

mod add;
mod block_validate;
mod encrypt;
mod forget_all_certificates;
mod list;
mod manifest_validate;
mod poll;
mod realm_create;
mod realm_decrypt_name;
mod realm_key_rotation;
mod realm_keys_bundle;
mod realm_rename;
mod realm_share;
mod realms_needs;
mod shamir_recovery_delete;
mod shamir_recovery_list;
mod shamir_recovery_setup;
mod store;
mod user_revoke;
mod user_update_profile;
mod workspace_bootstrap;

pub use add::{CertifAddCertificatesBatchError, InvalidCertificateError, MaybeRedactedSwitch};
pub use block_validate::{CertifValidateBlockError, InvalidBlockAccessError};
pub use encrypt::CertifEncryptForSequesterServicesError;
pub use forget_all_certificates::CertifForgetAllCertificatesError;
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
pub use realm_keys_bundle::{
    CertifDecryptForRealmError, CertifEncryptForRealmError, EncrytionUsage, InvalidKeysBundleError,
};
pub use realm_rename::CertifRenameRealmError;
pub use realm_share::CertifShareRealmError;
pub use realms_needs::{CertifGetRealmNeedsError, RealmNeeds};
pub use shamir_recovery_delete::CertifDeleteShamirRecoveryError;
pub use shamir_recovery_list::{
    CertifGetSelfShamirRecoveryError, CertifGetShamirRecoveryShareDataError,
    CertifListShamirRecoveriesForOthersError, OtherShamirRecoveryInfo, SelfShamirRecoveryInfo,
};
pub use shamir_recovery_setup::{
    CertifSetupShamirRecoveryError, ShamirRecoverySetupCertificateTimestamps,
};
pub use store::{CertifStoreError, UpTo};
pub use user_revoke::CertifRevokeUserError;
pub use user_update_profile::CertifUpdateUserProfileError;
pub use workspace_bootstrap::CertifBootstrapWorkspaceError;

use crate::{event_bus::EventBus, ClientConfig};

// The following values define an offset to be used when processing
// a require greater timestamp error. They are different to give priority
// to some kind of certificates (ex: a certificate that revokes a user
// should have more chances to be accepted than a vlob update one)
// The base offset is the average network round trip time to compensate
// the first round trip.

// An optimization could be to measure the real round trip time for each request.
// The issue with that is that we tackling two distinct issues
// 1. the client clock is lagging behind
// 2. the client connection is degraded
// The current approach is only compensating for case 1.
// Case 2 would benefit from a offset depending on the real round trip time,
// but if we base our estimate of the round trip time on the expected timestamp
// provided by the server, case 2 would benefit from an unjustified priority, as
// it would pass for a client with a very degraded connection (within the limits of the ballpark though).
// That issue would not arise if we monitor the round trip time only from the client point of view.

/// This value is used to increment the timestamp provided by the backend
/// when a manifest restamping is required. This value should be kept small
/// compared to the certificate stamp ahead value, so the certificate updates have
/// priority over manifest updates.
/// # in microseconds, or 0.1 seconds, 100 ms (average network round trip time)
pub const MANIFEST_STAMP_AHEAD_US: i64 = 100_000;

/// This value is used to increment the timestamp provided by the backend
/// when a realm certificate restamping is required. This value should be kept big
/// compared to the manifest stamp ahead value, but less than USER_CERTIFICATE_STAMP_AHEAD_US
///  so the certificate updates have priority over manifest updates.
///  # microseconds, or 0.25 seconds
pub const REALM_CERTIFICATE_STAMP_AHEAD_US: i64 = 250_000;

/// This value is used to increment the timestamp provided by the backend
/// when a user or device certificate restamping is required. This value should be kept big
/// compared to the manifest stamp ahead value, so the certificate updates have
/// priority over manifest updates.
///  # microseconds, or 0.5 seconds
pub const USER_CERTIFICATE_STAMP_AHEAD_US: i64 = 500_000;

/// Archiving doesn't have to use a value as large 0.5 seconds
/// # microseconds, or 1 milliseconds
/// TODO: not used yet, see #6092
pub const ARCHIVING_CERTIFICATE_STAMP_AHEAD_US: i64 = 1_000;

/// GreaterTimestampOffset represent the kind of certificate
/// that was sent to the server. It is used to give different priority
/// depending on the kind a certificate.
///
/// Note: it's counterintuitive but a greater offset means a greater priority.
/// If the offset is small there is a greater chance that something else was acknowledged
/// by the server before the new request was sent.
/// For example, if a server answers with requireGreaterTimestamp(T0) to client A,
/// then client B sends a new certificate at T0+x, and the server accepts it.
/// When client A will answer if the offset is smaller or equal to x,
/// the corresponding certificate will be refused, whereas if it's greater it will be accepted.
pub enum GreaterTimestampOffset {
    Manifest,
    /// User and device certificates
    User,
    Realm,
    /// TODO: not used yet, see #6092
    Archive,
}

impl From<GreaterTimestampOffset> for i64 {
    fn from(value: GreaterTimestampOffset) -> Self {
        match value {
            GreaterTimestampOffset::Manifest => MANIFEST_STAMP_AHEAD_US,
            GreaterTimestampOffset::User => USER_CERTIFICATE_STAMP_AHEAD_US,
            GreaterTimestampOffset::Archive => ARCHIVING_CERTIFICATE_STAMP_AHEAD_US,
            GreaterTimestampOffset::Realm => REALM_CERTIFICATE_STAMP_AHEAD_US,
        }
    }
}

/// Return the timestamp to be used when the server requires a greater timestamp.
///
/// The timestamp to be used will be whichever is greater between:
/// - now (obtained from `time_provider`)
/// - `strictly_greater_than` (expected by the server) + `offset` (depending on certificate type)
pub(crate) fn greater_timestamp(
    time_provider: &TimeProvider,
    offset: GreaterTimestampOffset,
    strictly_greater_than: DateTime,
) -> DateTime {
    std::cmp::max(
        time_provider.now(),
        strictly_greater_than.add_us(offset.into()),
    )
}

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
pub(crate) struct CertificateOps {
    #[allow(unused)]
    config: Arc<ClientConfig>,
    device: Arc<LocalDevice>,
    event_bus: EventBus,
    cmds: Arc<AuthenticatedCmds>,
    store: store::CertificatesStore,
    /// Updating the certificate ops basically consist of:
    /// 1. Checking the store to know up to what point we are up to date.
    /// 2. Fetching new certificates from the server.
    /// 3. Adding the new certificates to the store.
    ///
    /// Hence there is no point in allowing this to be done concurrently (worst !
    /// we would then need to protect ourself against adding the same certificate
    /// multiple times), so a lock is needed.
    ///
    /// The obvious lock to use would be the store itself (since we have an exclusive
    /// access on it with `CertificatesStore::for_write`), this is what we used to do
    /// but it brought two issues:
    /// - The lock was held for too long (i.e. the time to fetch the certificates from
    ///   the server), during which the store was unavailable for any other operation.
    /// - In web, the store uses IndexedDB internally which has a very peculiar way of
    ///   implementing transactions: a transaction is automatically terminated after
    ///   any `await` that doesn't use the transaction (╯°□°）╯︵ ┻━┻
    ///
    /// So, long story short, we have to get rid of the request to the server occurring
    /// while the store is locked with a transaction. Hence this new lock !
    update_lock: AsyncMutex<()>,
}

// For readability, we define the public interface here and let the actual
// implementation in separated submodules
impl CertificateOps {
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
            update_lock: AsyncMutex::new(()),
        })
    }

    /// Stop the underlying storage (and flush whatever data is not yet on disk)
    ///
    /// Once stopped, it can still theoretically be used (i.e. `stop` doesn't
    /// consume `self`), but will only returns stopped error.
    pub async fn stop(&self) -> anyhow::Result<()> {
        self.store.stop().await
    }

    /// Forget all certificates from the local database, this is not needed under normal circumstances.
    ///
    /// Clearing the certificates might be useful in case the server database got rolled back
    /// to a previous state, resulting in the local database containing certificates that are no
    /// longer valid.
    ///
    /// Note that this scenario is technically similar to a server compromise, so this
    /// operation should only result from a manual user action (e.g. CLI command).
    pub async fn forget_all_certificates(&self) -> Result<(), CertifForgetAllCertificatesError> {
        forget_all_certificates::forget_all_certificates(self).await
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
        let _guard = self.update_lock.lock().await;

        self.store
            .for_write(async |store| {
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

    //     self.store.for_read(async |store| {
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

    /// Return the number of new certificates added to the storage.
    pub async fn poll_server_for_new_certificates(
        &self,
        latest_known_timestamps: Option<&PerTopicLastTimestamps>,
    ) -> Result<usize, CertifPollServerError> {
        poll::poll_server_for_new_certificates(self, latest_known_timestamps).await
    }

    pub async fn validate_user_manifest(
        &self,
        needed_realm_certificate_timestamp: DateTime,
        needed_common_certificate_timestamp: DateTime,
        author: DeviceID,
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
        author: DeviceID,
        version: VersionInt,
        timestamp: DateTime,
        encrypted: &[u8],
    ) -> Result<FolderManifest, CertifValidateManifestError> {
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
        author: DeviceID,
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
        needed_realm_certificate_timestamp: DateTime,
        realm_id: VlobID,
        key_index: IndexInt,
        manifest: &FileManifest,
        access: &BlockAccess,
        encrypted: &[u8],
    ) -> Result<Vec<u8>, CertifValidateBlockError> {
        block_validate::validate_block(
            self,
            needed_realm_certificate_timestamp,
            realm_id,
            key_index,
            manifest,
            access,
            encrypted,
        )
        .await
    }

    /// Encrypt the data with the last known key from the most recent realm keys bundle.
    ///
    /// Be aware this function potentially do server accesses (to fetch the keys bundle).
    pub async fn encrypt_for_realm(
        &self,
        usage: EncrytionUsage,
        realm_id: VlobID,
        data: &[u8],
    ) -> Result<(Vec<u8>, IndexInt), CertifEncryptForRealmError> {
        self.store
            .for_read(async |store| {
                realm_keys_bundle::encrypt_for_realm(self, store, usage, realm_id, data).await
            })
            .await
            .map_err(|e| match e {
                CertifStoreError::Stopped => CertifEncryptForRealmError::Stopped,
                CertifStoreError::Internal(err) => err.context("cannot access storage").into(),
            })?
    }

    /// Decrypt the data with the key at the given index from the most recent realm keys bundle.
    /// You most likely want to use the `validate_*` methods instead.
    /// This method should only be used to decrypt data not controlled by the server (given in
    /// that case we don't have a `needed_realm_certificate_timestamp`). So far only the
    /// encrypted path in the file link url need this.
    ///
    /// Be aware this function potentially do server accesses (to fetch the keys bundle).
    pub async fn decrypt_opaque_data_for_realm(
        &self,
        usage: EncrytionUsage,
        realm_id: VlobID,
        key_index: IndexInt,
        encrypted: &[u8],
    ) -> Result<Vec<u8>, CertifDecryptForRealmError> {
        let outcome = self
            .store
            .for_read(async |store| {
                realm_keys_bundle::decrypt_for_realm(
                    self, store, usage, realm_id, key_index, encrypted,
                )
                .await
            })
            .await
            .map_err(|e| match e {
                CertifStoreError::Stopped => CertifDecryptForRealmError::Stopped,
                CertifStoreError::Internal(err) => err.context("cannot access storage").into(),
            })?;

        // If the current keys bundle doesn't contains the specified key index, we have
        // no choice but to poll the server & retry in case we were actually lagging
        // behind.

        if !matches!(outcome, Err(CertifDecryptForRealmError::KeyNotFound)) {
            return outcome;
        }

        self.poll_server_for_new_certificates(None)
            .await
            .map_err(|err| match err {
                CertifPollServerError::Stopped => CertifDecryptForRealmError::Stopped,
                CertifPollServerError::Offline(e) => CertifDecryptForRealmError::Offline(e),
                CertifPollServerError::InvalidCertificate(err) => {
                    CertifDecryptForRealmError::InvalidCertificate(err)
                }
                CertifPollServerError::Internal(err) => err.context("cannot poll server").into(),
            })?;

        self.store
            .for_read(async |store| {
                realm_keys_bundle::decrypt_for_realm(
                    self, store, usage, realm_id, key_index, encrypted,
                )
                .await
            })
            .await
            .map_err(|e| match e {
                CertifStoreError::Stopped => CertifDecryptForRealmError::Stopped,
                CertifStoreError::Internal(err) => err.context("cannot access storage").into(),
            })?
    }

    #[allow(unused)]
    pub async fn encrypt_for_sequester_services(
        &self,
        data: &[u8],
    ) -> Result<Option<Vec<(SequesterServiceID, Bytes)>>, CertifEncryptForSequesterServicesError>
    {
        self.store
            .for_read(async |store| encrypt::encrypt_for_sequester_services(store, data).await)
            .await
            .map_err(|e| match e {
                CertifStoreError::Stopped => CertifEncryptForSequesterServicesError::Stopped,
                CertifStoreError::Internal(err) => err.context("cannot access storage").into(),
            })?
    }

    pub async fn revoke_user(
        &self,
        user: UserID,
    ) -> Result<CertificateBasedActionOutcome, CertifRevokeUserError> {
        user_revoke::revoke_user(self, user).await
    }

    pub async fn user_update_profile(
        &self,
        user_id: UserID,
        new_profile: UserProfile,
    ) -> Result<CertificateBasedActionOutcome, CertifUpdateUserProfileError> {
        user_update_profile::update_profile(self, user_id, new_profile).await
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

    /// Do a key rotation for the given realm, provided that `target_key_index`
    /// actually corresponds to the next key index (and otherwise consider a
    /// concurrent operation has made the key rotation we were supposed to do).
    pub async fn rotate_realm_key_idempotent(
        &self,
        realm_id: VlobID,
        target_key_index: IndexInt,
    ) -> Result<CertificateBasedActionOutcome, CertifRotateRealmKeyError> {
        realm_key_rotation::rotate_realm_key_idempotent(self, realm_id, target_key_index).await
    }

    /// Returns the needs of a given realm, i.e. if new key rotation (and users
    /// unsharing) is needed.
    pub async fn get_realm_needs(
        &self,
        realm_id: VlobID,
    ) -> Result<RealmNeeds, CertifGetRealmNeedsError> {
        realms_needs::get_realm_needs(self, realm_id).await
    }

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

    #[allow(unused)]
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

    pub async fn get_self_shamir_recovery(
        &self,
    ) -> Result<SelfShamirRecoveryInfo, CertifGetSelfShamirRecoveryError> {
        shamir_recovery_list::get_self_shamir_recovery(self).await
    }

    pub async fn list_shamir_recoveries_for_others(
        &self,
    ) -> Result<Vec<OtherShamirRecoveryInfo>, CertifListShamirRecoveriesForOthersError> {
        shamir_recovery_list::list_shamir_recoveries_for_others(self).await
    }

    pub async fn get_shamir_recovery_share_data(
        &self,
        user_id: UserID,
        shamir_recovery_created_on: DateTime,
    ) -> Result<ShamirRecoveryShareData, CertifGetShamirRecoveryShareDataError> {
        shamir_recovery_list::get_shamir_recovery_share_data(
            self,
            user_id,
            shamir_recovery_created_on,
        )
        .await
    }

    pub async fn setup_shamir_recovery(
        &self,
        per_recipient_shares: HashMap<UserID, NonZeroU8>,
        threshold: NonZeroU8,
    ) -> Result<ShamirRecoverySetupCertificateTimestamps, CertifSetupShamirRecoveryError> {
        shamir_recovery_setup::setup_shamir_recovery(self, per_recipient_shares, threshold).await
    }

    pub async fn delete_shamir_recovery(
        &self,
    ) -> Result<CertificateBasedActionOutcome, CertifDeleteShamirRecoveryError> {
        shamir_recovery_delete::delete_shamir_recovery(self).await
    }
}

#[cfg(test)]
#[path = "../../tests/unit/certif/mod.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
