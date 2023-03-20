// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{collections::HashMap, sync::Arc};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_client_types::LocalDevice;
use libparsec_core::RemoteDevicesManager;
use libparsec_platform_async::Mutex;
use libparsec_protocol::authenticated_cmds::v2::{
    realm_create, realm_get_role_certificates, vlob_list_versions,
};
use libparsec_types::{
    CertificateSignerOwned, CertificateSignerRef, DataResult, DateTime, DeviceCertificate,
    DeviceID, EntryID, RealmID, RealmRole, RealmRoleCertificate, RevokedUserCertificate,
    UserCertificate, UserID, VlobID,
};

use crate::{FSError, FSResult};

pub struct UserRemoteLoader {
    device: Arc<LocalDevice>,
    workspace_id: EntryID,
    backend_cmds: AuthenticatedCmds,
    remote_devices_manager: Arc<Mutex<RemoteDevicesManager>>,
    realm_role_certificates_caches: Vec<RealmRoleCertificate>,
}

impl UserRemoteLoader {
    pub fn new(
        device: Arc<LocalDevice>,
        workspace_id: EntryID,
        backend_cmds: AuthenticatedCmds,
        remote_devices_manager: Arc<Mutex<RemoteDevicesManager>>,
    ) -> Self {
        Self {
            device,
            workspace_id,
            backend_cmds,
            remote_devices_manager,
            realm_role_certificates_caches: vec![],
        }
    }

    pub fn clear_realm_role_certificate_cache(&mut self) {
        self.realm_role_certificates_caches.clear();
    }

    async fn verify_unsecure_certificate(
        &mut self,
        current_roles: &mut HashMap<UserID, RealmRole>,
        unsecure_certif: &RealmRoleCertificate,
        raw_certif: &[u8],
    ) -> FSResult<()> {
        let certif_author = match &unsecure_certif.author {
            CertificateSignerOwned::Root => {
                return Err(FSError::Custom(
                    "Expected a certificate signed by a user".into(),
                ))
            }
            CertificateSignerOwned::User(device_id) => device_id,
        };

        let author = self
            .remote_devices_manager
            .lock()
            .await
            .get_device(certif_author, false)
            .await?;

        let verified_certif = RealmRoleCertificate::verify_and_load(
            raw_certif,
            &author.verify_key,
            CertificateSignerRef::User(&author.device_id),
            None,
            None,
        )
        .map_err(|e| FSError::InvalidRealmRoleCertificates(e.to_string()))?;

        // Make sure author had the right to do this
        let needed_roles = match current_roles.get(&verified_certif.user_id) {
            None if current_roles.is_empty()
                && &verified_certif.user_id == author.device_id.user_id() =>
            {
                // First user is auto-signed
                None
            }
            Some(RealmRole::Owner) | Some(RealmRole::Manager) => Some(RealmRole::Owner),
            _ if verified_certif.role == Some(RealmRole::Owner)
                || verified_certif.role == Some(RealmRole::Manager) =>
            {
                Some(RealmRole::Owner)
            }
            _ => Some(RealmRole::Manager),
        };

        match (needed_roles, current_roles.get(certif_author.user_id())) {
            // If `needed_roles` is Manager, role must be higher: Manager or Owner
            (Some(RealmRole::Manager), Some(RealmRole::Manager))
            | (Some(RealmRole::Manager), Some(RealmRole::Owner))
            | (Some(RealmRole::Owner), Some(RealmRole::Owner))
            | (None, _) => (),
            _ => {
                return Err(FSError::InvalidRealmRoleCertificates(format!(
                    "{:?} has not right to give {:?} role to {} on {}",
                    verified_certif.author,
                    verified_certif.role,
                    verified_certif.user_id,
                    verified_certif.timestamp,
                )))
            }
        }

        if let Some(role) = verified_certif.role {
            current_roles.insert(verified_certif.user_id, role);
        } else {
            current_roles.remove(&verified_certif.user_id);
        }
        Ok(())
    }

    async fn load_realm_role_certificates_and_roles(
        &mut self,
        realm_id: Option<EntryID>,
    ) -> FSResult<(Vec<RealmRoleCertificate>, HashMap<UserID, RealmRole>)> {
        let realm_id = RealmID::from(*realm_id.unwrap_or(self.workspace_id));
        let rep = self
            .backend_cmds
            .send(realm_get_role_certificates::Req { realm_id })
            .await?;

        match rep {
            realm_get_role_certificates::Rep::Ok { certificates } => {
                let mut unsecure_certifs = certificates
                    .into_iter()
                    .map(|uv| RealmRoleCertificate::unsecure_load(&uv).map(|rc| (rc, uv)))
                    .collect::<DataResult<Vec<_>>>()
                    .map_err(|e| FSError::InvalidRealmRoleCertificates(e.to_string()))?;

                unsecure_certifs.sort_by(|a, b| a.0.timestamp.cmp(&b.0.timestamp));

                let mut current_roles = HashMap::new();

                for (unsecure_certif, raw_certif) in unsecure_certifs.iter() {
                    self.verify_unsecure_certificate(
                        &mut current_roles,
                        unsecure_certif,
                        raw_certif,
                    )
                    .await?
                }

                // Now unsecure_certifs is no longer insecure given we have validated its items
                Ok((
                    unsecure_certifs.into_iter().map(|x| x.0).collect(),
                    current_roles,
                ))
            }
            realm_get_role_certificates::Rep::NotAllowed => Err(FSError::WorkspaceNoReadAccess),
            _ => Err(FSError::Custom(format!(
                "Cannot retrieve workspace roles: {rep:?}"
            ))),
        }
    }

    pub async fn load_realm_role_certificates(
        &mut self,
        realm_id: Option<EntryID>,
    ) -> FSResult<Vec<RealmRoleCertificate>> {
        self.load_realm_role_certificates_and_roles(realm_id)
            .await
            .map(|x| x.0)
    }

    pub async fn load_realm_current_roles(
        &mut self,
        realm_id: Option<EntryID>,
    ) -> FSResult<HashMap<UserID, RealmRole>> {
        self.load_realm_role_certificates_and_roles(realm_id)
            .await
            .map(|x| x.1)
    }

    pub async fn get_user(
        &mut self,
        user_id: &UserID,
        no_cache: bool,
    ) -> FSResult<(UserCertificate, Option<RevokedUserCertificate>)> {
        self.remote_devices_manager
            .lock()
            .await
            .get_user(user_id, no_cache)
            .await
            .map_err(FSError::from)
    }

    pub async fn get_device(
        &mut self,
        device_id: &DeviceID,
        no_cache: bool,
    ) -> FSResult<DeviceCertificate> {
        self.remote_devices_manager
            .lock()
            .await
            .get_device(device_id, no_cache)
            .await
            .map_err(FSError::from)
    }

    pub async fn list_versions(
        &self,
        entry_id: EntryID,
    ) -> FSResult<HashMap<u64, (DateTime, DeviceID)>> {
        let vlob_id = VlobID::from(*entry_id);
        let rep = self
            .backend_cmds
            .send(vlob_list_versions::Req { vlob_id })
            .await?;

        match rep {
            vlob_list_versions::Rep::NotAllowed => Err(FSError::WorkspaceNoReadAccess),
            vlob_list_versions::Rep::NotFound { .. } => {
                Err(FSError::RemoteManifestNotFound(entry_id))
            }
            vlob_list_versions::Rep::InMaintenance => Err(FSError::WorkspaceInMaintenance),
            vlob_list_versions::Rep::Ok { versions } => Ok(versions),
            vlob_list_versions::Rep::UnknownStatus { .. } => Err(FSError::Custom(format!(
                "Cannot fetch vlob {entry_id}: {rep:?}"
            ))),
        }
    }

    pub async fn create_realm(&self, realm_id: RealmID) -> FSResult<()> {
        let timestamp = self.device.time_provider.now();
        let role_certificate = RealmRoleCertificate {
            author: CertificateSignerOwned::User(self.device.device_id.clone()),
            timestamp,
            realm_id,
            user_id: self.device.device_id.user_id().clone(),
            role: Some(RealmRole::Owner),
        }
        .dump_and_sign(&self.device.signing_key);

        let rep = self
            .backend_cmds
            .send(realm_create::Req { role_certificate })
            .await?;

        match rep {
            // It's possible a previous attempt to create this realm
            // succeeded but we didn't receive the confirmation, hence
            // we play idempotent here.
            realm_create::Rep::AlreadyExists => Ok(()),
            realm_create::Rep::Ok => Ok(()),
            _ => Err(FSError::Custom(format!(
                "Cannot create realm {realm_id}: {rep:?}"
            ))),
        }
    }
}
