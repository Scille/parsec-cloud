// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
    sync::Arc,
};

use libparsec_platform_storage::certificates::UpTo;
use libparsec_types::prelude::*;

use super::{
    store::{CertifStoreError, LastShamirRecovery},
    CertificateOps,
};

#[derive(Debug, thiserror::Error)]
pub enum CertifListShamirRecoveryError {
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<CertifStoreError> for CertifListShamirRecoveryError {
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
) -> Result<SelfShamirRecoveryInfo, CertifListShamirRecoveryError> {
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
) -> Result<Vec<OtherShamirRecoveryInfo>, CertifListShamirRecoveryError> {
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

pub async fn get_shamir_recovery_share_data(
    ops: &CertificateOps,
    user_id: UserID,
) -> Result<ShamirRecoveryShareData, CertifListShamirRecoveryError> {
    ops.store
        .for_read(|store| async move {
            // TODO: check that the shamir is actually recoverable
            let certificate = store
                .get_last_shamir_recovery_share_certificate_for_recipient(
                    UpTo::Current,
                    user_id,
                    ops.device.user_id,
                )
                .await?;
            let certificate = match certificate {
                Some(certificate) => Ok(certificate),
                None => Err(CertifListShamirRecoveryError::Internal(anyhow::anyhow!(
                    "No share data found for user {}",
                    user_id
                ))),
            }?;
            let author_verify_key = store
                .get_device_verify_key(UpTo::Current, certificate.author)
                .await
                .map_err(|e| CertifListShamirRecoveryError::Internal(e.into()))?;
            ShamirRecoveryShareData::decrypt_verify_and_load_for(
                &certificate.ciphered_share,
                &ops.device.private_key,
                &author_verify_key,
                certificate.author,
                certificate.timestamp,
            )
            .map_err(|e| CertifListShamirRecoveryError::Internal(e.into()))
        })
        .await?
}
