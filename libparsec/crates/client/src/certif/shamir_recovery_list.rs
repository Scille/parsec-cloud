// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
    sync::Arc,
};

use libparsec_platform_storage::certificates::{GetCertificateError, PerTopicLastTimestamps, UpTo};
use libparsec_types::prelude::*;

use super::{
    store::{CertifForReadWithRequirementsError, CertifStoreError, LastShamirRecovery},
    CertificateOps, InvalidCertificateError,
};

#[derive(Debug, thiserror::Error)]
pub enum CertifGetSelfShamirRecoveryError {
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<CertifStoreError> for CertifGetSelfShamirRecoveryError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SelfShamirRecoveryInfo {
    NeverSetup,
    Deleted {
        created_on: DateTime,
        created_by: DeviceID,
        threshold: NonZeroU8,
        per_recipient_shares: HashMap<UserID, NonZeroU8>,
        deleted_on: DateTime,
        deleted_by: DeviceID,
    },
    SetupAllValid {
        created_on: DateTime,
        created_by: DeviceID,
        threshold: NonZeroU8,
        per_recipient_shares: HashMap<UserID, NonZeroU8>,
    },
    /// If some recipients are revoked, it's better to setup a new shamir recovery.
    SetupWithRevokedRecipients {
        created_on: DateTime,
        created_by: DeviceID,
        threshold: NonZeroU8,
        per_recipient_shares: HashMap<UserID, NonZeroU8>,
        revoked_recipients: HashSet<UserID>,
    },
    /// Too many recipients has been revoked
    SetupButUnusable {
        created_on: DateTime,
        created_by: DeviceID,
        threshold: NonZeroU8,
        per_recipient_shares: HashMap<UserID, NonZeroU8>,
        revoked_recipients: HashSet<UserID>,
    },
}

pub async fn get_self_shamir_recovery(
    ops: &CertificateOps,
) -> Result<SelfShamirRecoveryInfo, CertifGetSelfShamirRecoveryError> {
    ops.store
        .for_read(|store| async move {
            // 1. Retrieve the shamir recovery and it potential deletion

            let brief = match store
                .get_last_shamir_recovery_for_author(UpTo::Current, ops.device.user_id)
                .await?
            {
                LastShamirRecovery::Valid(brief) => brief,
                // Early exit if the shamir setup doesn't exist or is already deleted
                LastShamirRecovery::NeverSetup => return Ok(SelfShamirRecoveryInfo::NeverSetup),
                LastShamirRecovery::Deleted(brief, deletion) => {
                    return Ok(SelfShamirRecoveryInfo::Deleted {
                        created_on: brief.timestamp,
                        created_by: brief.author,
                        threshold: brief.threshold,
                        per_recipient_shares: brief.per_recipient_shares.clone(),
                        deleted_on: deletion.timestamp,
                        deleted_by: deletion.author,
                    })
                }
            };

            // Shamir recovery exists and is not deleted

            // 2. Now check if some recipients have been revoked

            let mut revoked_recipients = HashSet::new();
            let mut total_share_count = 0;
            let mut unreachable_share_count = 0;
            for (&recipient_user_id, &recipient_share_count) in &brief.per_recipient_shares {
                let recipient_share_count = recipient_share_count.get() as usize;
                let maybe_revoked = store
                    .get_revoked_user_certificate(UpTo::Current, recipient_user_id)
                    .await?;
                if maybe_revoked.is_some() {
                    unreachable_share_count += recipient_share_count;
                    revoked_recipients.insert(recipient_user_id);
                }
                total_share_count += recipient_share_count;
            }

            // 3. We got all the needed data, now time to arrange them !

            let usable =
                (total_share_count - unreachable_share_count) >= brief.threshold.get() as usize;

            let info = match (revoked_recipients.is_empty(), usable) {
                (true, true) => SelfShamirRecoveryInfo::SetupAllValid {
                    created_on: brief.timestamp,
                    created_by: brief.author,
                    threshold: brief.threshold,
                    per_recipient_shares: brief.per_recipient_shares.clone(),
                },
                (true, false) => unreachable!(), // If nobody is revoked, then all shares are reachable
                (false, true) => SelfShamirRecoveryInfo::SetupWithRevokedRecipients {
                    created_on: brief.timestamp,
                    created_by: brief.author,
                    threshold: brief.threshold,
                    per_recipient_shares: brief.per_recipient_shares.clone(),
                    revoked_recipients,
                },
                (false, false) => SelfShamirRecoveryInfo::SetupButUnusable {
                    created_on: brief.timestamp,
                    created_by: brief.author,
                    threshold: brief.threshold,
                    per_recipient_shares: brief.per_recipient_shares.clone(),
                    revoked_recipients,
                },
            };

            Ok(info)
        })
        .await?
}

#[derive(Debug, thiserror::Error)]
pub enum CertifListShamirRecoveriesForOthersError {
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<CertifStoreError> for CertifListShamirRecoveriesForOthersError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OtherShamirRecoveryInfo {
    Deleted {
        user_id: UserID,
        created_on: DateTime,
        created_by: DeviceID,
        threshold: NonZeroU8,
        per_recipient_shares: HashMap<UserID, NonZeroU8>,
        deleted_on: DateTime,
        deleted_by: DeviceID,
    },
    SetupAllValid {
        user_id: UserID,
        created_on: DateTime,
        created_by: DeviceID,
        threshold: NonZeroU8,
        per_recipient_shares: HashMap<UserID, NonZeroU8>,
    },
    /// If some recipients are revoked, it's better to setup a new shamir recovery.
    SetupWithRevokedRecipients {
        user_id: UserID,
        created_on: DateTime,
        created_by: DeviceID,
        threshold: NonZeroU8,
        per_recipient_shares: HashMap<UserID, NonZeroU8>,
        revoked_recipients: HashSet<UserID>,
    },
    /// Too many recipients has been revoked
    SetupButUnusable {
        user_id: UserID,
        created_on: DateTime,
        created_by: DeviceID,
        threshold: NonZeroU8,
        per_recipient_shares: HashMap<UserID, NonZeroU8>,
        revoked_recipients: HashSet<UserID>,
    },
}

pub async fn list_shamir_recoveries_for_others(
    ops: &CertificateOps,
) -> Result<Vec<OtherShamirRecoveryInfo>, CertifListShamirRecoveriesForOthersError> {
    ops.store
        .for_read(|store| async move {
            let mut per_user_last_shamir: HashMap<
                UserID,
                (
                    Arc<ShamirRecoveryBriefCertificate>,
                    Option<Arc<ShamirRecoveryDeletionCertificate>>,
                ),
            > = Default::default();

            // 1. Compute the last shamir recovery for each user that have or used to have one

            let briefs = store.get_shamir_recovery_briefs(UpTo::Current).await?;
            for brief in briefs {
                // We are only interested in the shamir recoveries we are recipient of.
                //
                // In theory this is equivalent to say "we ignore our own recoveries",
                // however in practice it is not the same when using the testbed !
                // This is because in this case, the testbed code always save all
                // the certificates certificates in the local storage (and not only
                // the one meant for us, i.e. the one from our shamir recovery topic).
                //
                // So during tests we can have here a `brief` certificate of which
                // we are neither its author nor among its recipients !
                if !brief.per_recipient_shares.contains_key(&ops.device.user_id) {
                    continue;
                }

                // Since the older recoveries are guaranteed to be deleted, we
                // can overwrite them with newer ones.
                per_user_last_shamir.insert(brief.user_id, (brief, None));
            }

            // 2. Detect if the last shamir recovery have been deleted

            let deletions = store.get_shamir_recovery_deletions(UpTo::Current).await?;
            for deletion in deletions {
                match per_user_last_shamir.entry(deletion.setup_to_delete_user_id) {
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        let (brief, maybe_deletion) = entry.get_mut();
                        if brief.timestamp == deletion.setup_to_delete_timestamp {
                            *maybe_deletion = Some(deletion);
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(_) => continue,
                }
            }

            // 3. Detect recipients that have been revoked

            let all_revoked_recipients = {
                let mut all_recipients = HashSet::new();
                for (brief, _) in per_user_last_shamir.values() {
                    all_recipients.extend(brief.per_recipient_shares.keys().cloned());
                }

                let mut all_revoked_recipients = HashSet::new();
                for recipient in all_recipients {
                    let maybe_revoked = store
                        .get_revoked_user_certificate(UpTo::Current, recipient)
                        .await?;
                    if maybe_revoked.is_some() {
                        all_revoked_recipients.insert(recipient);
                    }
                }

                all_revoked_recipients
            };

            // 4. Now arrange the data for the result

            let mut all_info: Vec<OtherShamirRecoveryInfo> =
                Vec::with_capacity(per_user_last_shamir.len());

            for (brief, maybe_deletion) in per_user_last_shamir.values() {
                let info = if let Some(deletion) = maybe_deletion {
                    OtherShamirRecoveryInfo::Deleted {
                        user_id: brief.user_id,
                        created_on: brief.timestamp,
                        created_by: brief.author,
                        threshold: brief.threshold,
                        per_recipient_shares: brief.per_recipient_shares.clone(),
                        deleted_on: deletion.timestamp,
                        deleted_by: deletion.author,
                    }
                } else {
                    let mut total_share_count = 0;
                    let mut unreachable_share_count = 0;
                    let mut revoked_recipients = HashSet::new();
                    for (&recipient_user_id, &recipient_share_count) in &brief.per_recipient_shares
                    {
                        let recipient_share_count = recipient_share_count.get() as usize;
                        if all_revoked_recipients.contains(&recipient_user_id) {
                            unreachable_share_count += recipient_share_count;
                            revoked_recipients.insert(recipient_user_id);
                        }
                        total_share_count += recipient_share_count;
                    }

                    let usable = (total_share_count - unreachable_share_count)
                        >= brief.threshold.get() as usize;

                    match (revoked_recipients.is_empty(), usable) {
                        (true, true) => OtherShamirRecoveryInfo::SetupAllValid {
                            user_id: brief.user_id,
                            created_on: brief.timestamp,
                            created_by: brief.author,
                            threshold: brief.threshold,
                            per_recipient_shares: brief.per_recipient_shares.clone(),
                        },
                        (true, false) => unreachable!(), // If nobody is revoked, then all shares are reachable
                        (false, true) => OtherShamirRecoveryInfo::SetupWithRevokedRecipients {
                            user_id: brief.user_id,
                            created_on: brief.timestamp,
                            created_by: brief.author,
                            threshold: brief.threshold,
                            per_recipient_shares: brief.per_recipient_shares.clone(),
                            revoked_recipients,
                        },
                        (false, false) => OtherShamirRecoveryInfo::SetupButUnusable {
                            user_id: brief.user_id,
                            created_on: brief.timestamp,
                            created_by: brief.author,
                            threshold: brief.threshold,
                            per_recipient_shares: brief.per_recipient_shares.clone(),
                            revoked_recipients,
                        },
                    }
                };

                all_info.push(info);
            }

            // Order by setup creation is not strictly needed, but it's convenient for testing
            all_info.sort_unstable_by_key(|i| match i {
                OtherShamirRecoveryInfo::Deleted { created_on, .. }
                | OtherShamirRecoveryInfo::SetupAllValid { created_on, .. }
                | OtherShamirRecoveryInfo::SetupWithRevokedRecipients { created_on, .. }
                | OtherShamirRecoveryInfo::SetupButUnusable { created_on, .. } => *created_on,
            });

            Ok(all_info)
        })
        .await?
}

#[derive(Debug, thiserror::Error)]
pub enum CertifGetShamirRecoveryShareDataError {
    #[error("Shamir recovery brief certificate not found")]
    ShamirRecoveryBriefCertificateNotFound,
    #[error("Shamir recovery share certificate not found")]
    ShamirRecoveryShareCertificateNotFound,
    #[error("Shamir recovery has been deleted")]
    ShamirRecoveryDeleted,
    #[error("Shamir recovery is unusable due to revoked recipients")]
    ShamirRecoveryUnusable,
    #[error(transparent)]
    CorruptedShareData(DataError),
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<CertifForReadWithRequirementsError> for CertifGetShamirRecoveryShareDataError {
    fn from(value: CertifForReadWithRequirementsError) -> Self {
        match value {
            CertifForReadWithRequirementsError::Offline => Self::Offline,
            CertifForReadWithRequirementsError::Stopped => Self::Stopped,
            CertifForReadWithRequirementsError::InvalidCertificate(err) => {
                Self::InvalidCertificate(err)
            }
            CertifForReadWithRequirementsError::Internal(err) => err.into(),
        }
    }
}

pub async fn get_shamir_recovery_share_data(
    ops: &CertificateOps,
    user_id: UserID,
    shamir_recovery_created_on: DateTime,
) -> Result<ShamirRecoveryShareData, CertifGetShamirRecoveryShareDataError> {
    let needed_timestamps = PerTopicLastTimestamps::new_for_shamir(shamir_recovery_created_on);
    ops.store
        .for_read_with_requirements(ops, &needed_timestamps, |store| async move {

            // 1. Check that the corresponding shamir recovery has not been deleted

            let deletions = store
                .get_shamir_recovery_deletions(UpTo::Current)
                .await?;
            let maybe_deletion = deletions.iter().find(|deletion| {
                deletion.setup_to_delete_user_id == user_id
                    && deletion.setup_to_delete_timestamp == shamir_recovery_created_on
            });
            if maybe_deletion.is_some() {
                return Err(CertifGetShamirRecoveryShareDataError::ShamirRecoveryDeleted);
            }

            // 2. Retrieve the brief and share certificates

            let brief_certificate = store
                .get_last_shamir_recovery_brief_certificate_for_author(UpTo::Current, &user_id)
                .await?;
            let brief_certificate = match brief_certificate {
                Some(brief_certificate) if brief_certificate.timestamp == shamir_recovery_created_on => Ok(brief_certificate),
                _ => {
                    Err(CertifGetShamirRecoveryShareDataError::ShamirRecoveryBriefCertificateNotFound)
                }
            }?;

            let share_certificate = store
                .get_last_shamir_recovery_share_certificate_for_recipient(
                    UpTo::Current,
                    user_id,
                    ops.device.user_id,
                )
                .await?;
            let share_certificate = match share_certificate {
                Some(share_certificate) if share_certificate.timestamp == shamir_recovery_created_on => Ok(share_certificate),
                _ => {
                    Err(CertifGetShamirRecoveryShareDataError::ShamirRecoveryShareCertificateNotFound)
                }
            }?;

            // 3. Detect if the shamir recovery is usable

            let mut usable_share_count = 0;
            for (&recipient_user_id, &recipient_share_count) in
                &brief_certificate.per_recipient_shares
            {
                let maybe_revoked = store
                    .get_revoked_user_certificate(UpTo::Current, recipient_user_id)
                    .await?;
                if maybe_revoked.is_none() {
                    usable_share_count += recipient_share_count.get() as usize;
                }
            }
            if usable_share_count < brief_certificate.threshold.get() as usize {
                return Err(
                    CertifGetShamirRecoveryShareDataError::ShamirRecoveryUnusable,
                );
            }

            // 4. Retrieve the verify key of the author of the share certificate

            let author_verify_key = store
                .get_device_verify_key(UpTo::Current, share_certificate.author)
                .await
                .map_err(|e| match e {
                    GetCertificateError::NonExisting => {
                        CertifGetShamirRecoveryShareDataError::Internal(anyhow::anyhow!(
                            "Local storage of certificates seems corrupted: Shamir recovery share certificate {:?} has been validated, but its author's certificate doesn't exist", share_certificate
                        )
                    )
                    }
                    GetCertificateError::ExistButTooRecent {..} => unreachable!(),
                    GetCertificateError::Internal(_) => {
                        CertifGetShamirRecoveryShareDataError::Internal(e.into())
                    }
                })?;

            // 5. Decrypt the share data

            ShamirRecoveryShareData::decrypt_verify_and_load_for(
                &share_certificate.ciphered_share,
                &ops.device.private_key,
                &author_verify_key,
                share_certificate.author,
                share_certificate.timestamp,
            )
            .map_err(CertifGetShamirRecoveryShareDataError::CorruptedShareData)
        })
        .await?
}
