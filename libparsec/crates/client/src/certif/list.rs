// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::{
    CertificateOps, UpTo,
    store::{CertifStoreError, GetCertificateError},
};

pub use super::store::CertifStoreError as CertifGetCurrentSelfProfileError;
pub use super::store::CertifStoreError as CertifGetCurrentSelfRealmsRoleError;
pub use super::store::CertifStoreError as CertifGetCurrentSelfRealmRoleError;
pub use super::store::CertifStoreError as CertifListUsersError;
pub use super::store::CertifStoreError as CertifListUserDevicesError;
pub use super::store::CertifStoreError as CertifListWorkspaceUsersError;

pub(super) async fn get_current_self_profile(
    ops: &CertificateOps,
) -> Result<UserProfile, CertifGetCurrentSelfProfileError> {
    ops.store
        .for_read(async |store| store.get_current_self_profile().await)
        .await?
        .map_err(|err| err.into())
}

pub(super) async fn get_current_self_realms_role(
    ops: &CertificateOps,
) -> Result<Vec<(VlobID, Option<RealmRole>, DateTime)>, CertifGetCurrentSelfRealmsRoleError> {
    // TODO: cache !
    let certifs = ops
        .store
        .for_read(async |store| {
            store
                .get_user_realms_roles(UpTo::Current, ops.device.user_id)
                .await
        })
        .await??;

    // Replay the history of all changes

    let mut roles = Vec::with_capacity(certifs.len());
    for certif in certifs {
        let maybe_exists = roles
            .iter()
            .position(|(realm_id, _, _)| *realm_id == certif.realm_id);
        match maybe_exists {
            Some(index) => {
                roles[index] = (certif.realm_id, certif.role, certif.timestamp);
            }
            None => {
                roles.push((certif.realm_id, certif.role, certif.timestamp));
            }
        }
    }

    Ok(roles)
}

pub(super) async fn get_current_self_realm_role(
    ops: &CertificateOps,
    realm_id: VlobID,
) -> Result<Option<Option<RealmRole>>, CertifGetCurrentSelfRealmRoleError> {
    let role = ops
        .store
        .for_read(async |store| store.get_self_user_realm_role(realm_id).await)
        .await??;
    Ok(role)
}

#[derive(Debug)]
pub struct UserInfo {
    pub id: UserID,
    pub human_handle: HumanHandle,
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

pub(super) async fn list_users(
    ops: &CertificateOps,
    skip_revoked: bool,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<Vec<UserInfo>, CertifListUsersError> {
    ops.store
        .for_read(async |store| {
            let certifs = {
                // If `skip_revoked` is enabled, we cannot limit the number of users
                // certificates to fetch given we will then filter out the ones corresponding
                // to revoked users (and hence return less than `limit` users).
                // TODO: A better approach would be call `get_user_certificates` again
                //       if we detect we are short on users.
                let limit = if skip_revoked { None } else { limit };
                store
                    .get_user_certificates(UpTo::Current, offset, limit)
                    .await?
            };

            let mut infos = Vec::with_capacity(certifs.len());
            for certif in certifs {
                let maybe_revoked = store
                    .get_revoked_user_certificate(UpTo::Current, certif.user_id)
                    .await?;
                if skip_revoked && maybe_revoked.is_some() {
                    continue;
                }
                let (revoked_on, revoked_by) = match maybe_revoked {
                    None => (None, None),
                    Some(certif) => (Some(certif.timestamp), Some(certif.author.to_owned())),
                };

                let maybe_update = store
                    .get_last_user_update_certificate(UpTo::Current, certif.user_id)
                    .await?;
                let current_profile = match maybe_update {
                    Some(update) => update.new_profile,
                    None => certif.profile,
                };

                let created_by = match &certif.author {
                    CertificateSigner::User(author) => Some(author.to_owned()),
                    CertificateSigner::Root => None,
                };

                let info = UserInfo {
                    id: certif.user_id.to_owned(),
                    human_handle: certif.human_handle.as_ref().to_owned(),
                    current_profile,
                    created_on: certif.timestamp,
                    created_by,
                    revoked_on,
                    revoked_by,
                };
                infos.push(info);

                if skip_revoked && infos.len() == limit.unwrap_or(u32::MAX) as usize {
                    break;
                }
            }

            Ok(infos)
        })
        .await?
}

#[derive(Debug)]
pub struct DeviceInfo {
    pub id: DeviceID,
    pub device_label: DeviceLabel,
    pub purpose: DevicePurpose,
    pub created_on: DateTime,
    // `None` if signed by root verify key (i.e. the user that bootstrapped the organization)
    pub created_by: Option<DeviceID>,
}

pub(super) async fn list_user_devices(
    ops: &CertificateOps,
    user_id: UserID,
) -> Result<Vec<DeviceInfo>, CertifListUserDevicesError> {
    let certifs = ops
        .store
        .for_read(async |store| {
            store
                .get_user_devices_certificates(UpTo::Current, user_id)
                .await
        })
        .await??;

    let items = certifs
        .into_iter()
        .map(|certif| {
            let created_by = match &certif.author {
                CertificateSigner::User(author) => Some(author.to_owned()),
                CertificateSigner::Root => None,
            };

            DeviceInfo {
                id: certif.device_id.to_owned(),
                device_label: certif.device_label.as_ref().to_owned(),
                purpose: certif.purpose,
                created_on: certif.timestamp,
                created_by,
            }
        })
        .collect();

    Ok(items)
}

#[derive(Debug, thiserror::Error)]
pub enum CertifGetUserDeviceError {
    #[error("No user/device with this device ID")]
    NonExisting,
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<CertifStoreError> for CertifGetUserDeviceError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

pub(super) async fn get_user_device(
    ops: &CertificateOps,
    device_id: DeviceID,
) -> Result<(UserInfo, DeviceInfo), CertifGetUserDeviceError> {
    ops.store
        .for_read(async |store| {
            let device_certif = match store.get_device_certificate(UpTo::Current, device_id).await {
                Ok(certif) => certif,
                Err(GetCertificateError::ExistButTooRecent { .. }) => {
                    unreachable!("query up to current")
                }
                Err(GetCertificateError::NonExisting) => {
                    return Err(CertifGetUserDeviceError::NonExisting);
                }
                Err(GetCertificateError::Internal(err)) => {
                    return Err(CertifGetUserDeviceError::Internal(err));
                }
            };

            let user_id = device_certif.user_id;
            let user_certif = match store.get_user_certificate(UpTo::Current, user_id).await {
                Ok(certif) => certif,
                Err(GetCertificateError::ExistButTooRecent { .. }) => {
                    unreachable!("query up to current")
                }
                Err(GetCertificateError::NonExisting) => {
                    // This should never happen (unless the database is corrupted) since
                    // we got the user ID from the device certificate.
                    // TODO: add a warning ?
                    return Err(CertifGetUserDeviceError::NonExisting);
                }
                Err(GetCertificateError::Internal(err)) => {
                    return Err(CertifGetUserDeviceError::Internal(err));
                }
            };

            let user_created_by = match &user_certif.author {
                CertificateSigner::User(author) => Some(author.to_owned()),
                CertificateSigner::Root => None,
            };

            let maybe_revoked = store
                .get_revoked_user_certificate(UpTo::Current, user_id)
                .await?;
            let (revoked_on, revoked_by) = match maybe_revoked {
                None => (None, None),
                Some(certif) => (Some(certif.timestamp), Some(certif.author.to_owned())),
            };

            let maybe_update = store
                .get_last_user_update_certificate(UpTo::Current, user_id)
                .await?;
            let current_profile = match maybe_update {
                Some(update) => update.new_profile,
                None => user_certif.profile,
            };

            let user_info = UserInfo {
                id: user_id,
                human_handle: user_certif.human_handle.as_ref().to_owned(),
                current_profile,
                created_on: user_certif.timestamp,
                created_by: user_created_by,
                revoked_on,
                revoked_by,
            };

            let device_certif = match store.get_device_certificate(UpTo::Current, device_id).await {
                Ok(certif) => certif,
                Err(GetCertificateError::ExistButTooRecent { .. }) => unreachable!(),
                Err(GetCertificateError::NonExisting) => {
                    return Err(CertifGetUserDeviceError::NonExisting);
                }
                Err(GetCertificateError::Internal(err)) => {
                    return Err(CertifGetUserDeviceError::Internal(err));
                }
            };

            let device_created_by = match &device_certif.author {
                CertificateSigner::User(author) => Some(author.to_owned()),
                CertificateSigner::Root => None,
            };

            let device_info = DeviceInfo {
                id: device_id,
                device_label: device_certif.device_label.as_ref().to_owned(),
                purpose: device_certif.purpose,
                created_on: device_certif.timestamp,
                created_by: device_created_by,
            };

            Ok((user_info, device_info))
        })
        .await?
}

#[derive(Debug, PartialEq, Eq)]
pub struct WorkspaceUserAccessInfo {
    pub user_id: UserID,
    pub human_handle: HumanHandle,
    pub current_profile: UserProfile,
    pub current_role: RealmRole,
}

/// List users currently part of the given workspace (i.e. user not revoked
/// and with a valid role)
pub(super) async fn list_workspace_users(
    ops: &CertificateOps,
    realm_id: VlobID,
) -> Result<Vec<WorkspaceUserAccessInfo>, CertifListWorkspaceUsersError> {
    let mut infos = Vec::new();

    ops.store
        .for_read(async |store| {
            let per_user_certifs = store
                .get_realm_current_users_roles(UpTo::Current, realm_id)
                .await?;

            for (user_id, role_certif) in per_user_certifs {
                let current_role = role_certif
                    .role
                    .expect("unshared user should not be listed");

                // Ignore revoked users
                let maybe_revoked = store
                    .get_revoked_user_certificate(UpTo::Current, user_id)
                    .await?;
                if maybe_revoked.is_some() {
                    continue;
                }

                let user_certif = match store.get_user_certificate(UpTo::Current, user_id).await {
                    Ok(user_certif) => user_certif,
                    // We got the user ID from the certificate store, it is guaranteed to
                    // be present !
                    Err(
                        GetCertificateError::NonExisting
                        | GetCertificateError::ExistButTooRecent { .. },
                    ) => unreachable!(),
                    Err(GetCertificateError::Internal(err)) => return Err(err),
                };

                let current_profile = match store
                    .get_last_user_update_certificate(UpTo::Current, user_id)
                    .await?
                {
                    Some(user_update_certif) => user_update_certif.new_profile,
                    // Profile has never been updated, use the initial one
                    None => user_certif.profile,
                };

                let user_info = WorkspaceUserAccessInfo {
                    user_id,
                    human_handle: user_certif.human_handle.as_ref().to_owned(),
                    current_profile,
                    current_role,
                };
                infos.push(user_info);
            }

            Ok(infos)
        })
        .await?
        .map_err(|err| err.into())
}
