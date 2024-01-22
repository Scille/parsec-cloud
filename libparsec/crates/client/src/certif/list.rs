// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_types::prelude::*;

use super::{
    store::{CertifStoreError, GetCertificateError},
    CertifOps, UpTo,
};

pub use super::store::CertifStoreError as CertifGetCurrentSelfProfileError;
pub use super::store::CertifStoreError as CertifGetCurrentSelfRealmsRoleError;
pub use super::store::CertifStoreError as CertifGetCurrentSelfRealmRoleError;
pub use super::store::CertifStoreError as CertifListUsersError;
pub use super::store::CertifStoreError as CertifListUserDevicesError;
pub use super::store::CertifStoreError as CertifListWorkspaceUsersError;

pub(super) async fn get_current_self_profile(
    ops: &CertifOps,
) -> Result<UserProfile, CertifGetCurrentSelfProfileError> {
    ops.store
        .for_read(|store| store.get_current_self_profile())
        .await?
        .map_err(|err| err.into())
}

pub(super) async fn get_current_self_realms_role(
    ops: &CertifOps,
) -> Result<Vec<(VlobID, Option<RealmRole>, DateTime)>, CertifGetCurrentSelfRealmsRoleError> {
    // TODO: cache !
    let certifs = ops
        .store
        .for_read(|store| {
            store.get_user_realms_roles(UpTo::Current, ops.device.user_id().to_owned())
        })
        .await??;

    // Replay the history of all changes
    let roles = certifs
        .into_iter()
        .map(|c| (c.realm_id, c.role, c.timestamp))
        .collect();

    Ok(roles)
}

pub(super) async fn get_current_self_realm_role(
    ops: &CertifOps,
    realm_id: VlobID,
) -> Result<Option<Option<RealmRole>>, CertifGetCurrentSelfRealmRoleError> {
    // TODO: cache !
    let certif = ops
        .store
        .for_read(|store| {
            store.get_user_realm_role(UpTo::Current, ops.device.user_id().to_owned(), realm_id)
        })
        .await??;

    Ok(certif.map(|c| c.role))
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
    ops: &CertifOps,
    skip_revoked: bool,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<Vec<UserInfo>, CertifListUsersError> {
    ops.store
        .for_read(|store| async move {
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
    pub created_on: DateTime,
    // `None` if signed by root verify key (i.e. the user that bootstrapped the organization)
    pub created_by: Option<DeviceID>,
}

pub(super) async fn list_user_devices(
    ops: &CertifOps,
    user_id: UserID,
) -> Result<Vec<DeviceInfo>, CertifListUserDevicesError> {
    let certifs = ops
        .store
        .for_read(|store| store.get_user_devices_certificates(UpTo::Current, user_id))
        .await??;

    let items = certifs
        .into_iter()
        .map(|certif| {
            let created_by = match &certif.author {
                CertificateSignerOwned::User(author) => Some(author.to_owned()),
                CertificateSignerOwned::Root => None,
            };

            DeviceInfo {
                id: certif.device_id.to_owned(),
                device_label: certif.device_label.as_ref().to_owned(),
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
    ops: &CertifOps,
    device_id: DeviceID,
) -> Result<(UserInfo, DeviceInfo), CertifGetUserDeviceError> {
    let user_id = device_id.user_id().to_owned();

    ops.store
        .for_read(|store| async move {
            let user_certif = match store
                .get_user_certificate(UpTo::Current, user_id.clone())
                .await
            {
                Ok(certif) => certif,
                Err(GetCertificateError::ExistButTooRecent { .. }) => unreachable!(),
                Err(GetCertificateError::NonExisting) => {
                    return Err(CertifGetUserDeviceError::NonExisting)
                }
                Err(GetCertificateError::Internal(err)) => {
                    return Err(CertifGetUserDeviceError::Internal(err))
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
                human_handle: user_certif.human_handle.as_ref().to_owned(),
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
                Err(GetCertificateError::NonExisting) => {
                    return Err(CertifGetUserDeviceError::NonExisting)
                }
                Err(GetCertificateError::Internal(err)) => {
                    return Err(CertifGetUserDeviceError::Internal(err))
                }
            };

            let device_created_by = match &device_certif.author {
                CertificateSignerOwned::User(author) => Some(author.to_owned()),
                CertificateSignerOwned::Root => None,
            };

            let device_info = DeviceInfo {
                id: device_id,
                device_label: device_certif.device_label.as_ref().to_owned(),
                created_on: device_certif.timestamp,
                created_by: device_created_by,
            };

            Ok((user_info, device_info))
        })
        .await?
}

#[derive(Debug)]
pub struct WorkspaceUserAccessInfo {
    pub user_id: UserID,
    pub human_handle: HumanHandle,
    pub current_profile: UserProfile,
    pub current_role: RealmRole,
}

/// List users currently part of the given workspace (i.e. user not revoked
/// and with a valid role)
pub(super) async fn list_workspace_users(
    ops: &CertifOps,
    realm_id: VlobID,
) -> Result<Vec<WorkspaceUserAccessInfo>, CertifListWorkspaceUsersError> {
    ops.store
        .for_read(|store| async move {
            let role_certifs = store.get_realm_roles(UpTo::Current, realm_id).await?;

            let mut infos = HashMap::with_capacity(role_certifs.len());
            for role_certif in role_certifs {
                // Ignore user that have lost their access
                let current_role = match role_certif.role {
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

                let current_profile = match store
                    .get_last_user_update_certificate(UpTo::Current, user_id.clone())
                    .await?
                {
                    Some(user_update_certif) => user_update_certif.new_profile,
                    // Profile has never been updated, use the initial one
                    None => user_certif.profile,
                };

                let user_info = WorkspaceUserAccessInfo {
                    user_id: user_id.clone(),
                    human_handle: user_certif.human_handle.as_ref().to_owned(),
                    current_profile,
                    current_role,
                };
                infos.insert(user_id, user_info);
            }

            Ok(infos.into_values().collect())
        })
        .await?
        .map_err(|err| err.into())
}
