// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_types::prelude::*;

use crate::EncryptionUsage;

use super::{
    CertificateOps, UpTo, realm_keys_bundle,
    store::{CertifStoreError, CertificatesStoreReadGuard, GetCertificateError},
};

pub type CertifGetCurrentSelfProfileError = super::store::CertifStoreError;
pub type CertifGetCurrentSelfRealmsRoleError = super::store::CertifStoreError;
pub type CertifGetCurrentSelfRealmRoleError = super::store::CertifStoreError;
pub type CertifGetRealmArchivingConfigurationError = super::store::CertifStoreError;
pub type CertifListUsersError = super::store::CertifStoreError;
pub type CertifListUserDevicesError = super::store::CertifStoreError;
pub type CertifListWorkspaceUsersError = super::store::CertifStoreError;
pub type CertifListWorkspaceUserRoleChangesError = super::store::CertifStoreError;
pub type CertifListWorkspaceTimelineError = super::store::CertifStoreError;

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
        // Note this O(n) operation is fine since we expect `certifs` to be small
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
            for user_certif in certifs {
                let revoked_certif = store
                    .get_revoked_user_certificate(UpTo::Current, user_certif.user_id)
                    .await?;
                if skip_revoked && revoked_certif.is_some() {
                    continue;
                }
                let update_certif = store
                    .get_last_user_update_certificate(UpTo::Current, user_certif.user_id)
                    .await?;

                infos.push(
                    cook_user_info_from_certificates(
                        user_certif.as_ref(),
                        revoked_certif.as_deref(),
                        update_certif.as_deref(),
                    )
                    .await,
                );

                if skip_revoked && infos.len() == limit.unwrap_or(u32::MAX) as usize {
                    break;
                }
            }

            Ok(infos)
        })
        .await?
}

#[derive(Debug, thiserror::Error)]
pub enum CertifGetUserInfoError {
    #[error("No user with this UserID")]
    NonExisting,
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<CertifStoreError> for CertifGetUserInfoError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

async fn cook_user_info_from_certificates(
    user_certificate: &UserCertificate,
    revoked_certificate: Option<&RevokedUserCertificate>,
    update_certificate: Option<&UserUpdateCertificate>,
) -> UserInfo {
    let (revoked_on, revoked_by) = match revoked_certificate {
        None => (None, None),
        Some(certif) => (Some(certif.timestamp), Some(certif.author.to_owned())),
    };
    let current_profile = match update_certificate {
        None => user_certificate.profile,
        Some(update_certificate) => update_certificate.new_profile,
    };

    let created_by = match &user_certificate.author {
        CertificateSigner::User(author) => Some(author.to_owned()),
        CertificateSigner::Root => None,
    };

    UserInfo {
        id: user_certificate.user_id.to_owned(),
        human_handle: user_certificate.human_handle.as_ref().to_owned(),
        current_profile,
        created_on: user_certificate.timestamp,
        created_by,
        revoked_on,
        revoked_by,
    }
}

pub(super) async fn get_user_info(
    ops: &CertificateOps,
    user_id: UserID,
) -> Result<UserInfo, CertifGetUserInfoError> {
    ops.store
        .for_read(async |store| {
            let user_certificate = match store.get_user_certificate(UpTo::Current, user_id).await {
                Ok(certif) => certif,
                Err(GetCertificateError::ExistButTooRecent { .. }) => {
                    unreachable!("query up to current")
                }
                Err(GetCertificateError::NonExisting) => {
                    return Err(CertifGetUserInfoError::NonExisting);
                }
                Err(GetCertificateError::Internal(err)) => {
                    return Err(CertifGetUserInfoError::Internal(err));
                }
            };
            let revoked_certificate = store
                .get_revoked_user_certificate(UpTo::Current, user_certificate.user_id)
                .await?;
            let update_certificate = store
                .get_last_user_update_certificate(UpTo::Current, user_certificate.user_id)
                .await?;

            Ok(cook_user_info_from_certificates(
                user_certificate.as_ref(),
                revoked_certificate.as_deref(),
                update_certificate.as_deref(),
            )
            .await)
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

            let device_created_by = match &device_certif.author {
                CertificateSigner::User(author) => Some(author.to_owned()),
                CertificateSigner::Root => None,
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

#[derive(Debug, PartialEq, Eq)]
pub struct WorkspaceUserRoleChange {
    pub granted_by: DeviceID,
    pub granted_on: DateTime,
    /// None means unshared
    pub role: Option<RealmRole>,
}

/// Give a list of role changes for a user on a workspace
/// Roles are returned ordered by timestamp in increasing order (i.e. oldest first)
pub(super) async fn list_workspace_user_role_changes(
    ops: &CertificateOps,
    realm_id: VlobID,
    user_id: UserID,
) -> Result<Vec<WorkspaceUserRoleChange>, CertifListWorkspaceUserRoleChangesError> {
    ops.store
        .for_read(async |store| {
            let realm_certifs = store.get_realm_roles(UpTo::Current, realm_id).await?;
            Ok::<_, anyhow::Error>(
                realm_certifs
                    .into_iter()
                    .filter(|cert| cert.user_id == user_id)
                    .map(|cert| WorkspaceUserRoleChange {
                        granted_by: cert.author,
                        granted_on: cert.timestamp,
                        role: cert.role,
                    })
                    .collect(),
            )
        })
        .await?
        .map_err(|err| err.into())
}

/// An entry of a workspace's timeline, built from the realm certificates.
///
/// `author` is the device that issued the certificate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceTimelineEntry {
    /// The workspace has been created (first realm role certificate, the
    /// creator gives itself the `Owner` role).
    /// `initial_name` is `None` if the workspace has never been named
    /// or if the name cannot be decrypted.
    Created {
        timestamp: DateTime,
        author: DeviceID,
        initial_name: Option<EntryName>,
    },
    /// The role of a user on the workspace has been changed:
    /// - `previous_role` is `None` if the workspace has just been shared with the user.
    /// - `new_role` is `None` if the user has been removed from the workspace.
    UserRoleChanged {
        timestamp: DateTime,
        author: DeviceID,
        user_id: UserID,
        previous_role: Option<RealmRole>,
        new_role: Option<RealmRole>,
    },
    /// The workspace has been renamed (the initial naming done during the
    /// workspace bootstrap is not reported).
    /// A name is `None` if it cannot be decrypted.
    Renamed {
        timestamp: DateTime,
        author: DeviceID,
        previous_name: Option<EntryName>,
        new_name: Option<EntryName>,
    },
    /// The workspace has been archived (i.e. it is now read-only).
    Archived {
        timestamp: DateTime,
        author: DeviceID,
    },
    /// The workspace has been planned for deletion.
    DeletionPlanned {
        timestamp: DateTime,
        author: DeviceID,
        deletion_date: DateTime,
    },
    /// The workspace has been made available again after being archived
    /// or planned for deletion.
    Restored {
        timestamp: DateTime,
        author: DeviceID,
    },
}

impl WorkspaceTimelineEntry {
    pub fn timestamp(&self) -> DateTime {
        match self {
            Self::Created { timestamp, .. }
            | Self::UserRoleChanged { timestamp, .. }
            | Self::Renamed { timestamp, .. }
            | Self::Archived { timestamp, .. }
            | Self::DeletionPlanned { timestamp, .. }
            | Self::Restored { timestamp, .. } => *timestamp,
        }
    }
}

/// List all the events that occurred on a workspace, according to its realm certificates.
/// Entries are returned ordered by timestamp in increasing order (i.e. oldest first)
pub(super) async fn list_workspace_timeline(
    ops: &CertificateOps,
    realm_id: VlobID,
) -> Result<Vec<WorkspaceTimelineEntry>, CertifListWorkspaceTimelineError> {
    ops.store
        .for_read(async |store| {
            let role_certifs = store.get_realm_roles(UpTo::Current, realm_id).await?;
            let name_certifs = store
                .get_realm_name_certificates(UpTo::Current, realm_id)
                .await?;
            let archiving_certifs = store
                .get_realm_archiving_certificates(UpTo::Current, realm_id)
                .await?;

            // Maximum size it can be
            let mut entries = Vec::with_capacity(
                role_certifs.len() + name_certifs.len() + archiving_certifs.len(),
            );

            // Decrypt all the names first, since the initial one is needed by the
            // `Created` entry. A name that cannot be decrypted is not considered
            // an error: the entry is still returned, only without the name.
            let mut names = Vec::with_capacity(name_certifs.len());
            for certif in &name_certifs {
                names.push(decrypt_realm_name_certificate(ops, store, certif).await);
            }

            // We keep track of the current role of each user to provide
            // the previous role (`None` meaning the user had no access).
            let mut current_roles: HashMap<UserID, RealmRole> = HashMap::new();
            for (index, certif) in role_certifs.iter().enumerate() {
                let previous_role = match certif.role {
                    Some(role) => current_roles.insert(certif.user_id, role),
                    None => current_roles.remove(&certif.user_id),
                };
                // The very first role certificate is the creator giving themselves
                // the `Owner` role.
                let entry = if index == 0 {
                    WorkspaceTimelineEntry::Created {
                        timestamp: certif.timestamp,
                        author: certif.author,
                        initial_name: names.first().cloned().flatten(),
                    }
                } else {
                    match (previous_role, certif.role) {
                        // Unsharing a user that has no access, should not occur
                        // since certificates are validated before being stored.
                        (None, None) => continue,
                        (previous_role, new_role) => WorkspaceTimelineEntry::UserRoleChanged {
                            timestamp: certif.timestamp,
                            author: certif.author,
                            user_id: certif.user_id,
                            previous_role,
                            new_role,
                        },
                    }
                };
                entries.push(entry);
            }

            // The first name certificate is the initial naming done
            // during the workspace bootstrap, so it is not a rename.
            for index in 1..name_certifs.len() {
                let certif = &name_certifs[index];
                entries.push(WorkspaceTimelineEntry::Renamed {
                    timestamp: certif.timestamp,
                    author: certif.author,
                    previous_name: names[index - 1].clone(),
                    new_name: names[index].clone(),
                });
            }

            entries.extend(
                archiving_certifs
                    .iter()
                    .map(|certif| match certif.configuration {
                        RealmArchivingConfiguration::Available => {
                            WorkspaceTimelineEntry::Restored {
                                timestamp: certif.timestamp,
                                author: certif.author,
                            }
                        }
                        RealmArchivingConfiguration::Archived => WorkspaceTimelineEntry::Archived {
                            timestamp: certif.timestamp,
                            author: certif.author,
                        },
                        RealmArchivingConfiguration::DeletionPlanned { deletion_date } => {
                            WorkspaceTimelineEntry::DeletionPlanned {
                                timestamp: certif.timestamp,
                                author: certif.author,
                                deletion_date,
                            }
                        }
                    }),
            );

            entries.sort_by_key(|entry| entry.timestamp());

            Ok::<_, anyhow::Error>(entries)
        })
        .await?
        .map_err(|err| err.into())
}

/// Decrypt the name contained in a realm name certificate.
///
/// Returns `None` (and logs a warning) if the name cannot be decrypted, e.g.
/// because the keys bundle cannot be fetched (offline) or is corrupted.
async fn decrypt_realm_name_certificate(
    ops: &CertificateOps,
    store: &mut CertificatesStoreReadGuard<'_>,
    certif: &RealmNameCertificate,
) -> Option<EntryName> {
    let decrypted = match realm_keys_bundle::decrypt_for_realm(
        ops,
        store,
        EncryptionUsage::RealmRename,
        certif.realm_id,
        certif.key_index,
        &certif.encrypted_name,
    )
    .await
    {
        Ok(decrypted) => decrypted,
        Err(err) => {
            log::warn!(
                "Cannot decrypt realm name certificate (realm `{}`, created by `{}` on {}): {err}",
                certif.realm_id,
                certif.author,
                certif.timestamp,
            );
            return None;
        }
    };

    let name = std::str::from_utf8(&decrypted)
        .ok()
        .and_then(|decrypted_str| decrypted_str.parse().ok());
    if name.is_none() {
        log::warn!(
            "Invalid name in realm name certificate (realm `{}`, created by `{}` on {})",
            certif.realm_id,
            certif.author,
            certif.timestamp,
        );
    }
    name
}

pub(super) async fn get_realm_archiving_configuration(
    ops: &CertificateOps,
    realm_id: VlobID,
) -> Result<
    (RealmArchivingConfiguration, Option<DateTime>),
    CertifGetRealmArchivingConfigurationError,
> {
    ops.store
        .for_read(async |store| {
            let certif = store
                .get_realm_last_archiving_certificate(UpTo::Current, realm_id)
                .await?;
            Ok(match certif {
                Some(certif) => (certif.configuration, Some(certif.timestamp)),
                None => (RealmArchivingConfiguration::Available, None),
            })
        })
        .await?
}
