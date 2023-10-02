// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod add;
mod poll;
mod store;
mod validate_manifest;
mod validate_message;

pub use add::{AddCertificateError, InvalidCertificateError, MaybeRedactedSwitch};
pub use poll::PollServerError;
pub use store::{GetCertificateError, UpTo};
pub use validate_manifest::{InvalidManifestError, ValidateManifestError};
pub use validate_message::{InvalidMessageError, ValidateMessageError};

use std::{collections::HashMap, sync::Arc};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_types::prelude::*;

use crate::{certificates_ops::store::CertificatesStoreReadExt, event_bus::EventBus, ClientConfig};

pub struct UserInfo {
    pub id: UserID,
    pub human_handle: Option<HumanHandle>,
    pub current_profile: UserProfile,
    pub created_on: DateTime,
    // `None` if signed by root verify key (i.e. the user that bootstrapped the organization)
    pub created_by: Option<DeviceID>,
    /// Note that we might consider a user revoked even though our current time is still
    /// below the revocation timestamp. This is because there is no clear causality between
    /// our time and the production of the revocation timestamp (as it might have been produced
    /// by another device). So we simply consider a user revoked if a revocation timestamp has
    /// been issued.
    pub revoked_on: Option<DateTime>,
    pub revoked_by: Option<DeviceID>,
}

pub struct DeviceInfo {
    pub id: DeviceID,
    pub device_label: Option<DeviceLabel>,
    pub created_on: DateTime,
    // `None` if signed by root verify key (i.e. the user that bootstrapped the organization)
    pub created_by: Option<DeviceID>,
}

pub struct WorkspaceUserAccessInfo {
    pub user_id: UserID,
    pub human_handle: Option<HumanHandle>,
    pub role: RealmRole,
}

#[derive(Debug, thiserror::Error)]
pub enum GetUserDeviceError {
    #[error("No user/device with this device ID")]
    NonExisting,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

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
        let store = self.store.for_read().await;
        let recipient_certif = match store.get_user_certificate(UpTo::Current, user_id).await {
            Ok(certif) => certif,
            Err(
                GetCertificateError::NonExisting | GetCertificateError::ExistButTooRecent { .. },
            ) => return Ok(None),
            Err(GetCertificateError::Internal(err)) => return Err(err),
        };

        let encrypted = recipient_certif.public_key.encrypt_for_self(data);

        Ok(Some(encrypted))
    }

    pub(crate) async fn encrypt_for_sequester_services(
        &self,
        data: &[u8],
    ) -> anyhow::Result<Option<HashMap<SequesterServiceID, Bytes>>> {
        let store = self.store.for_read().await;
        match store
            .get_sequester_authority_certificate(UpTo::Current)
            .await
        {
            Ok(_) => (),
            // The organization is not sequestered (and `ExistButTooRecent` should never occur !)
            Err(GetCertificateError::NonExisting) => return Ok(None),
            Err(GetCertificateError::ExistButTooRecent { .. }) => unreachable!(),
            Err(GetCertificateError::Internal(err)) => return Err(err),
        }

        let services = store
            .get_sequester_service_certificates(UpTo::Current)
            .await?;
        let per_service_encrypted = services
            .into_iter()
            .map(|service| {
                (
                    service.service_id,
                    service.encryption_key_der.encrypt(data).into(),
                )
            })
            .collect();

        Ok(Some(per_service_encrypted))
    }

    /*
     * Public interface
     */

    pub async fn get_current_self_profile(&self) -> anyhow::Result<UserProfile> {
        let store = self.store.for_read().await;
        store.get_current_self_profile().await
    }

    pub async fn get_current_self_realms_roles(
        &self,
    ) -> anyhow::Result<HashMap<VlobID, Option<RealmRole>>> {
        // TODO: cache !
        let store = self.store.for_read().await;
        let certifs = store
            .get_user_realms_roles(UpTo::Current, self.device.user_id().to_owned())
            .await?;

        let mut roles = HashMap::new();
        // Replay the history of all changes
        for certif in certifs {
            roles.insert(certif.realm_id, certif.role);
        }

        Ok(roles)
    }

    pub async fn list_users(
        &self,
        skip_revoked: bool,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> anyhow::Result<Vec<UserInfo>> {
        let store = self.store.for_read().await;
        let certifs = store
            .get_user_certificates(UpTo::Current, offset, limit)
            .await?;

        let mut infos = Vec::with_capacity(certifs.len());
        for certif in certifs {
            let maybe_revoked = store
                .get_revoked_user_certificate(UpTo::Current, certif.user_id.clone())
                .await?;
            if skip_revoked && maybe_revoked.is_some() {
                continue;
            }
            let (revoked_on, revoked_by) = match maybe_revoked {
                None => (None, None),
                Some(certif) => (Some(certif.timestamp), Some(certif.author.to_owned())),
            };

            let maybe_update = store
                .get_last_user_update_certificate(UpTo::Current, certif.user_id.clone())
                .await?;
            let current_profile = match maybe_update {
                Some(update) => update.new_profile,
                None => certif.profile,
            };

            let created_by = match &certif.author {
                CertificateSignerOwned::User(author) => Some(author.to_owned()),
                CertificateSignerOwned::Root => None,
            };

            let info = UserInfo {
                id: certif.user_id.to_owned(),
                human_handle: certif.human_handle.to_owned(),
                current_profile,
                created_on: certif.timestamp,
                created_by,
                revoked_on,
                revoked_by,
            };
            infos.push(info);
        }

        Ok(infos)
    }

    pub async fn list_user_devices(&self, user_id: UserID) -> anyhow::Result<Vec<DeviceInfo>> {
        let store = self.store.for_read().await;
        let certifs = store
            .get_user_devices_certificates(UpTo::Current, user_id)
            .await?;

        let items = certifs
            .into_iter()
            .map(|certif| {
                let created_by = match &certif.author {
                    CertificateSignerOwned::User(author) => Some(author.to_owned()),
                    CertificateSignerOwned::Root => None,
                };

                DeviceInfo {
                    id: certif.device_id.to_owned(),
                    device_label: certif.device_label.to_owned(),
                    created_on: certif.timestamp,
                    created_by,
                }
            })
            .collect();

        Ok(items)
    }

    pub async fn get_user_device(
        &self,
        device_id: DeviceID,
    ) -> Result<(UserInfo, DeviceInfo), GetUserDeviceError> {
        let store = self.store.for_read().await;

        let user_id = device_id.user_id().to_owned();

        let user_certif = match store
            .get_user_certificate(UpTo::Current, user_id.clone())
            .await
        {
            Ok(certif) => certif,
            Err(GetCertificateError::ExistButTooRecent { .. }) => unreachable!(),
            Err(GetCertificateError::NonExisting) => return Err(GetUserDeviceError::NonExisting),
            Err(GetCertificateError::Internal(err)) => {
                return Err(GetUserDeviceError::Internal(err))
            }
        };

        let user_created_by = match &user_certif.author {
            CertificateSignerOwned::User(author) => Some(author.to_owned()),
            CertificateSignerOwned::Root => None,
        };

        let maybe_revoked = store
            .get_revoked_user_certificate(UpTo::Current, user_id.clone())
            .await?;
        let (revoked_on, revoked_by) = match maybe_revoked {
            None => (None, None),
            Some(certif) => (Some(certif.timestamp), Some(certif.author.to_owned())),
        };

        let maybe_update = store
            .get_last_user_update_certificate(UpTo::Current, user_id.clone())
            .await?;
        let current_profile = match maybe_update {
            Some(update) => update.new_profile,
            None => user_certif.profile,
        };

        let user_info = UserInfo {
            id: user_id,
            human_handle: user_certif.human_handle.to_owned(),
            current_profile,
            created_on: user_certif.timestamp,
            created_by: user_created_by,
            revoked_on,
            revoked_by,
        };

        let device_certif = match store
            .get_device_certificate(UpTo::Current, device_id.clone())
            .await
        {
            Ok(certif) => certif,
            Err(GetCertificateError::ExistButTooRecent { .. }) => unreachable!(),
            Err(GetCertificateError::NonExisting) => return Err(GetUserDeviceError::NonExisting),
            Err(GetCertificateError::Internal(err)) => {
                return Err(GetUserDeviceError::Internal(err))
            }
        };

        let device_created_by = match &device_certif.author {
            CertificateSignerOwned::User(author) => Some(author.to_owned()),
            CertificateSignerOwned::Root => None,
        };

        let device_info = DeviceInfo {
            id: device_id,
            device_label: device_certif.device_label.to_owned(),
            created_on: device_certif.timestamp,
            created_by: device_created_by,
        };

        Ok((user_info, device_info))
    }

    /// List users currently part of the given workspace (i.e. user not revoked
    /// and with a valid role)
    pub async fn list_workspace_users(
        &self,
        realm_id: VlobID,
    ) -> anyhow::Result<Vec<WorkspaceUserAccessInfo>> {
        let store = self.store.for_read().await;
        let role_certifs = store.get_realm_roles(UpTo::Current, realm_id).await?;

        let mut infos = HashMap::with_capacity(role_certifs.len());
        for role_certif in role_certifs {
            // Ignore user that have lost their access
            let role = match role_certif.role {
                None => {
                    infos.remove(&role_certif.user_id);
                    continue;
                }
                Some(role) => role,
            };
            let user_id = role_certif.user_id.clone();

            // Ignore revoked users
            let maybe_revoked = store
                .get_revoked_user_certificate(UpTo::Current, user_id.clone())
                .await?;
            if maybe_revoked.is_some() {
                continue;
            }

            let user_certif = match store
                .get_user_certificate(UpTo::Current, user_id.clone())
                .await
            {
                Ok(user_certif) => user_certif,
                // We got the user ID from the certificate store, it is guaranteed to
                // be present !
                Err(
                    GetCertificateError::NonExisting
                    | GetCertificateError::ExistButTooRecent { .. },
                ) => unreachable!(),
                Err(GetCertificateError::Internal(err)) => return Err(err),
            };

            let user_info = WorkspaceUserAccessInfo {
                user_id: user_id.clone(),
                human_handle: user_certif.human_handle.to_owned(),
                role,
            };
            infos.insert(user_id, user_info);
        }

        Ok(infos.into_values().collect())
    }
}
