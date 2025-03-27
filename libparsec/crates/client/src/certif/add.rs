// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use libparsec_types::prelude::*;

use super::{
    store::{CertificatesStoreWriteGuard, GetCertificateError, LastShamirRecovery},
    CertificateOps, UpTo,
};
use crate::{event_bus::EventInvalidCertificate, EventNewCertificates};

#[derive(Debug, Clone, thiserror::Error)]
pub enum InvalidCertificateError {
    #[error("Certificate `{hint}` is corrupted: {error}")]
    Corrupted { hint: String, error: DataError },

    // Consistency errors mean there is nothing wrong with the certificate content,
    // but it is incompatible with the other certificates we already have.
    #[error("Certificate `{hint}` breaks consistency: it declares to be signed by a author that doesn't exist")]
    NonExistingAuthor { hint: String },
    #[error("Certificate `{hint}` breaks consistency: it declares to be older than its author (created on {author_created_on})")]
    OlderThanAuthor {
        hint: String,
        author_created_on: DateTime,
    },
    #[error("Certificate `{hint}` breaks consistency: it declares to be signed by itself, which is not allowed")]
    SelfSigned { hint: String },
    #[error("Certificate `{hint}` breaks consistency: its author has already been revoked on {author_revoked_on}")]
    RevokedAuthor {
        hint: String,
        author_revoked_on: DateTime,
    },
    #[error("Certificate `{hint}` breaks consistency: its author is expected to be Admin but instead has profile {author_profile:?}")]
    AuthorNotAdmin {
        hint: String,
        author_profile: UserProfile,
    },
    #[error("Certificate `{hint}` breaks consistency: there is already a certificate with similar content")]
    ContentAlreadyExists { hint: String },
    #[error("Certificate `{hint}` breaks consistency: it is older than the previous certificate we know about ({last_certificate_timestamp})")]
    InvalidTimestamp {
        hint: String,
        last_certificate_timestamp: DateTime,
    },
    #[error("Certificate `{hint}` breaks consistency: it is signed by the root key, which is only allowed for the certificates created during the organization bootstrap")]
    RootSignatureOutOfBootstrap { hint: String },
    #[error("Certificate `{hint}` breaks consistency: it is signed by the root key but with a different timestamp than previous certificates ({last_root_signature_timestamp})")]
    RootSignatureTimestampMismatch {
        hint: String,
        last_root_signature_timestamp: DateTime,
    },
    #[error("Certificate `{hint}` breaks consistency: a sequestered service can only be added to a sequestered organization")]
    NotASequesteredOrganization { hint: String },
    #[error("Certificate `{hint}` breaks consistency: as sequester authority it must be provided first, but others certificates already exist")]
    SequesterAuthorityMustBeFirst { hint: String },
    #[error("Certificate `{hint}` breaks consistency: it refers to a user that doesn't exist")]
    NonExistingRelatedUser { hint: String },
    #[error("Certificate `{hint}` breaks consistency: it declares to be older than the user it refers to (created on {user_created_on})")]
    OlderThanRelatedUser {
        hint: String,
        user_created_on: DateTime,
    },
    #[error("Certificate `{hint}` breaks consistency: it refers to a user that has already been revoked on {user_revoked_on}")]
    RelatedUserAlreadyRevoked {
        hint: String,
        user_revoked_on: DateTime,
    },
    #[error("Certificate `{hint}` breaks consistency: it refers to a sequester service that has already been revoked on {service_revoked_on}")]
    RelatedSequesterServiceAlreadyRevoked {
        hint: String,
        service_revoked_on: DateTime,
    },
    #[error("Certificate `{hint}` breaks consistency: as first device certificate for its user it must have the same author that the user certificate ({user_author:?})")]
    UserFirstDeviceAuthorMismatch {
        hint: String,
        user_author: CertificateSigner,
    },
    #[error("Certificate `{hint}` breaks consistency: as first device certificate for its user it must have the same timestamp that the user certificate ({user_timestamp})")]
    UserFirstDeviceTimestampMismatch {
        hint: String,
        user_timestamp: DateTime,
    },
    #[error("Certificate `{hint}` breaks consistency: as first certificate for the realm, author must give the role to itself")]
    RealmFirstRoleMustBeSelfSigned { hint: String },
    #[error("Certificate `{hint}` breaks consistency: as first certificate for the realm it was expected to be a role certificate")]
    RealmFirstCertificateMustBeRole { hint: String },
    #[error("Certificate `{hint}` breaks consistency: author cannot change it own role")]
    RealmCannotChangeOwnRole { hint: String },
    #[error("Certificate `{hint}` breaks consistency: as first certificate for the realm, it must have Owner role")]
    RealmFirstRoleMustBeOwner { hint: String },
    #[error("Certificate `{hint}` breaks consistency: author is not part of the realm, so it cannot give access to it")]
    RealmAuthorHasNoRole { hint: String },
    #[error("Certificate `{hint}` breaks consistency: author is expected to have Owner role in the realm, but only has {author_role:?}")]
    RealmAuthorNotOwner {
        hint: String,
        author_role: RealmRole,
    },
    #[error("Certificate `{hint}` breaks consistency: author is expected to have Owner or Manager role in the realm, but only has {author_role:?}")]
    RealmAuthorNotOwnerOrManager {
        hint: String,
        author_role: RealmRole,
    },
    #[error("Certificate `{hint}` breaks consistency: user has Outsider profile, and hence cannot have Owner/Manager role in the realm")]
    RealmOutsiderCannotBeOwnerOrManager { hint: String },
    #[error("Certificate `{hint}` breaks consistency: author already has a shamir recovery setup")]
    ShamirRecoveryAlreadySetup { hint: String },
    #[error("Certificate `{hint}` breaks consistency: author cannot be among the recipients")]
    ShamirRecoverySelfAmongRecipients { hint: String },
    #[error(
        "Certificate `{hint}` breaks consistency: we were expecting the shamir recovery share certificate for user `{allowed_recipient}` corresponding to `{brief_hint}`"
    )]
    ShamirRecoveryMissingShare {
        hint: String,
        brief_hint: String,
        allowed_recipient: UserID,
    },
    #[error("Certificate `{hint}` breaks consistency: it shouldn't be provided to us as we are neither its author or among its recipients")]
    ShamirRecoveryUnrelatedToUs { hint: String },
    #[error(
        "Certificate `{hint}` breaks consistency: it refers to a shamir recovery that doesn't exist or is not the last one"
    )]
    ShamirRecoveryDeletionMustReferenceLastSetup { hint: String },
    #[error(
        "Certificate `{hint}` breaks consistency: it refers to a shamir recovery with different recipients ({setup_recipients:?})"
    )]
    ShamirRecoveryDeletionRecipientsMismatch {
        hint: String,
        setup_recipients: HashSet<UserID>,
    },
    #[error(
        "Certificate `{hint}` breaks consistency: it refers to a shamir recovery that has already been deleted on {deleted_on}"
    )]
    ShamirRecoveryAlreadyDeleted { hint: String, deleted_on: DateTime },
    #[error(
        "Certificate `{hint}` breaks consistency: no related shamir recovery brief certificate"
    )]
    ShamirRecoveryMissingBriefCertificate { hint: String },
    #[error("Certificate `{hint}` breaks consistency: it is about a different user ({user_id}) than its author ({author_user_id}), which is not allowed")]
    ShamirRecoveryNotAboutSelf {
        hint: String,
        user_id: UserID,
        author_user_id: UserID,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum CertifAddCertificatesBatchError {
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub enum MaybeRedactedSwitch {
    Switched,
    NoSwitch { new_certificates: usize },
}

pub(super) async fn add_certificates_batch(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    common_certificates: &[Bytes],
    sequester_certificates: &[Bytes],
    shamir_recovery_certificates: &[Bytes],
    realm_certificates: &HashMap<VlobID, Vec<Bytes>>,
) -> Result<MaybeRedactedSwitch, CertifAddCertificatesBatchError> {
    let mut new_certificates = 0;

    // Early exit if there is nothing to do
    if common_certificates.is_empty()
        && sequester_certificates.is_empty()
        && shamir_recovery_certificates.is_empty()
        && realm_certificates.is_empty()
    {
        return Ok(MaybeRedactedSwitch::NoSwitch { new_certificates });
    }

    let send_event_on_invalid_certificate = |err: CertifAddCertificatesBatchError| {
        if let CertifAddCertificatesBatchError::InvalidCertificate(what) = err {
            let event = EventInvalidCertificate(what);
            ops.event_bus.send(&event);
            CertifAddCertificatesBatchError::InvalidCertificate(event.0)
        } else {
            err
        }
    };

    let initial_stored_last_timestamps = store.get_last_timestamps().await?;
    let storage_initially_empty = initial_stored_last_timestamps.is_empty();
    let initial_self_profile = store.get_current_self_profile().await?;

    // If a certificate is invalid we exit without any further validation: the
    // write operation is going to be rolled back.

    // First add the sequester certificates:
    // - Sequester authority is expected to be the very first certificate added.
    // - Sequester certificate topic doesn't depend on any other topic.

    for signed in sequester_certificates {
        let cooked = validate_sequester_certificate(ops, store, signed.to_owned())
            .await
            .map_err(send_event_on_invalid_certificate)?;

        store.add_next_sequester_certificate(cooked, signed).await?;
        new_certificates += 1;
    }

    // Then add the common certificates...

    for signed in common_certificates {
        let cooked = validate_common_certificate(ops, store, signed.to_owned())
            .await
            .map_err(send_event_on_invalid_certificate)?;

        store.add_next_common_certificate(cooked, signed).await?;
        new_certificates += 1;
    }

    // ...now the remaining topics can be added (given they may depend on common)

    for (realm_id, certificates) in realm_certificates.iter() {
        for signed in certificates {
            let cooked = validate_realm_certificate(ops, store, *realm_id, signed.to_owned())
                .await
                .map_err(send_event_on_invalid_certificate)?;

            store.add_next_realm_x_certificate(cooked, signed).await?;
            new_certificates += 1;
        }
    }

    enum NextShamirCertifExpect {
        Anything,
        Share {
            brief: Arc<ShamirRecoveryBriefCertificate>,
        },
    }

    let mut expect_next_certif = NextShamirCertifExpect::Anything;
    for signed in shamir_recovery_certificates.iter() {
        let cooked = validate_shamir_recovery_certificate(ops, store, signed.to_owned())
            .await
            .map_err(send_event_on_invalid_certificate)?;

        match (&expect_next_certif, &cooked) {
            (
                NextShamirCertifExpect::Anything,
                ShamirRecoveryTopicArcCertificate::ShamirRecoveryBrief(cooked),
            ) => {
                if cooked
                    .per_recipient_shares
                    .contains_key(&ops.device.user_id)
                {
                    // Next certificate must be the shamir recovery share for our user
                    // (this is because the shamir recovery share has the same timestamp
                    // than the related shamir recovery brief, and also because only
                    // the shamir recovery share related to our user are sent to us).
                    expect_next_certif = NextShamirCertifExpect::Share {
                        brief: cooked.clone(),
                    };
                }
            }
            (NextShamirCertifExpect::Anything, _) => (),
            (
                NextShamirCertifExpect::Share { .. },
                ShamirRecoveryTopicArcCertificate::ShamirRecoveryShare(cooked),
            ) => {
                // In theory we should only receive shamir recovery share certificates
                // related to our own user.
                // Hence a sanity check here given this should have already been
                // enforced when validating the certificate.
                assert_eq!(cooked.recipient, ops.device.user_id);

                expect_next_certif = NextShamirCertifExpect::Anything;
            }
            (NextShamirCertifExpect::Share { brief }, unexpected_cooked) => {
                let hint = format!("{:?}", unexpected_cooked);
                let brief_hint = format!("{:?}", brief);
                let what = Box::new(InvalidCertificateError::ShamirRecoveryMissingShare {
                    hint,
                    brief_hint,
                    allowed_recipient: ops.device.user_id,
                });
                return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
            }
        }

        store
            .add_next_shamir_recovery_certificate(cooked, signed)
            .await?;
        new_certificates += 1;
    }
    if let NextShamirCertifExpect::Share { brief } = expect_next_certif {
        let hint = "<no more certificates>".to_string();
        let brief_hint = format!("{:?}", brief);
        let what = Box::new(InvalidCertificateError::ShamirRecoveryMissingShare {
            hint,
            brief_hint,
            allowed_recipient: ops.device.user_id,
        });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    }

    // Detect if we have switched to/from redacted certificates

    // If the certificate changes *our profile* from/to Outsider, then our certificate
    // storage must be cleared given we no longer use the right flavour: Outsider
    // must use redacted certificates while other profile use non-redacted ones.
    if !storage_initially_empty {
        let new_self_profile = store.get_current_self_profile().await?;
        match (initial_self_profile, new_self_profile) {
            // No need to switch.
            (UserProfile::Outsider, UserProfile::Outsider) => (),
            // Switching from/to Outsider !
            (UserProfile::Outsider, _) | (_, UserProfile::Outsider) => {
                // Clear the storage and notify the caller (which should in
                // turn re-poll the server, this time asking for all
                // certificates instead of just a subset)
                store.forget_all_certificates().await?;
                return Ok(MaybeRedactedSwitch::Switched);
            }
            // Switching profile without Outsider involved
            _ => (),
        }
    }

    // TODO: sent dedicated events which certificates are added (e.g. share to self,
    //       realm rename) unless `storage_initially_empty` is true, in which case
    //       only send a single generic event.

    // Finally send an event to notify the monitors that new certificates must now be
    // taken into account.
    // Note that the store write guard is still held (i.e. we are still in a SQLite
    // transaction that might get rolled back), so the event might not correspond to
    // any change in the end. This is considered okay given 1) the monitor cannot access
    // the certificates until we drop the guard and 2) it is equivalent of having
    // a concurrent operation doing the roll back between the time we send the event
    // and the time the monitors process it.

    let event = EventNewCertificates {
        storage_initially_empty,
        common_new_since: if common_certificates.is_empty() {
            None
        } else {
            initial_stored_last_timestamps.common
        },
        sequester_new_since: if sequester_certificates.is_empty() {
            None
        } else {
            initial_stored_last_timestamps.sequester
        },
        shamir_recovery_new_since: if shamir_recovery_certificates.is_empty() {
            None
        } else {
            initial_stored_last_timestamps.shamir_recovery
        },
        realm_new_since: realm_certificates
            .iter()
            .filter_map(|(realm_id, _)| {
                initial_stored_last_timestamps
                    .realm
                    .get(realm_id)
                    .map(|ts| (*realm_id, *ts))
            })
            .collect(),
    };
    ops.event_bus.send(&event);

    Ok(MaybeRedactedSwitch::NoSwitch { new_certificates })
}

macro_rules! verify_certificate_signature {

    // Entry points

    (Device, $unsecure:ident, $ops:expr, $store:expr) => {
        verify_certificate_signature!(
            @internal,
            Device,
            $unsecure,
            $unsecure.author().to_owned(),
            $ops,
            $store
        )
        .map(|(certif, serialized)| (Arc::new(certif), serialized))
        .map_err(|what| {
            CertifAddCertificatesBatchError::InvalidCertificate(what)
        })
    };

    (DeviceOrRoot, $unsecure:ident, $ops:expr, $store:expr) => {
        verify_certificate_signature!(
            @internal,
            DeviceOrRoot,
            $unsecure,
            $ops,
            $store
        )
        .map(|(certif, serialized)| (Arc::new(certif), serialized))
        .map_err(|what| {
            CertifAddCertificatesBatchError::InvalidCertificate(what)
        })
    };

    // Internal macro implementation stuff

    (@internal, Device, $unsecure:ident, $author:expr, $ops:expr, $store:expr) => {
        match $store.get_device_verify_key_and_user_id(UpTo::Timestamp($unsecure.timestamp().to_owned()), $author).await {
            Ok((author_verify_key, author_user_id)) => $unsecure
                .verify_signature(&author_verify_key)
                .map(|(certif, _)| (certif, author_user_id))
                .map_err(|(unsecure, error)| {
                    let hint = unsecure.hint();
                    Box::new(InvalidCertificateError::Corrupted { hint, error })
                }),
            Err(GetCertificateError::ExistButTooRecent {
                certificate_timestamp,
                ..
            }) => {
                // The author didn't exist at the time the certificate was made...
                let hint = $unsecure.hint();
                let what = Box::new(InvalidCertificateError::OlderThanAuthor {
                    hint,
                    author_created_on: certificate_timestamp,
                });
                Err(what)
            }
            Err(GetCertificateError::NonExisting) => {
                // Unknown author... we don't try here to poll the server
                // for new certificates: this is because certificate are
                // supposed to be added in a strictly causal order, hence
                // we are supposed to have already added all the certificates
                // needed to validate this one. And if that's not the case
                // it's suspicious and error should be raised !
                let what = Box::new(InvalidCertificateError::NonExistingAuthor {
                    hint: $unsecure.hint(),
                });
                Err(what)
            }
            Err(err @ GetCertificateError::Internal(_)) => {
                return Err(anyhow::anyhow!(err).into());
            }
        }
    };

    (@internal, DeviceOrRoot, $unsecure:ident, $ops:expr, $store:expr) => {
        match $unsecure.author() {
            CertificateSigner::Root => $unsecure
                .verify_signature($ops.device.root_verify_key())
                .map(|(certif, _)| (certif, None))
                .map_err(|(unsecure, error)| {
                    let hint = unsecure.hint();
                    Box::new(InvalidCertificateError::Corrupted { hint, error })
                }),

            CertificateSigner::User(author) => {
                verify_certificate_signature!(
                    @internal,
                    Device,
                    $unsecure,
                    author,
                    $ops,
                    $store
                ).map(|(certif, author_user_id)| (certif, Some(author_user_id)))
            }
        }
    };

}

/// Validate the given certificate by both checking it content (format, signature, etc.)
/// and it global consistency with the others certificates.
async fn validate_common_certificate(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    signed: Bytes,
) -> Result<CommonTopicArcCertificate, CertifAddCertificatesBatchError> {
    // 1) Deserialize the certificate

    let unsecure = match CommonTopicCertificate::unsecure_load(signed) {
        Ok(unsecure) => unsecure,
        Err(error) => {
            // No information can be extracted from the binary data...
            let hint = "<unknown>".into();
            let what = Box::new(InvalidCertificateError::Corrupted { hint, error });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    };

    // 2) Verify the certificate signature
    // By doing so we also verify author's device existence, but we don't go
    // any further, hence additional checks on author must be done at step 3 !

    match unsecure {
        UnsecureCommonTopicCertificate::User(unsecure) => {
            let (cooked, author_user_id) =
                verify_certificate_signature!(DeviceOrRoot, unsecure, ops, store)?;

            // 3) The certificate is valid, last check is the consistency with other certificates
            check_user_certificate_consistency(ops, store, &cooked, author_user_id).await?;

            Ok(CommonTopicArcCertificate::User(cooked))
        }
        UnsecureCommonTopicCertificate::Device(unsecure) => {
            let (cooked, author_user_id) =
                verify_certificate_signature!(DeviceOrRoot, unsecure, ops, store)?;

            // 3) The certificate is valid, last check is the consistency with other certificates
            check_device_certificate_consistency(ops, store, &cooked, author_user_id).await?;

            Ok(CommonTopicArcCertificate::Device(cooked))
        }
        UnsecureCommonTopicCertificate::UserUpdate(unsecure) => {
            let (cooked, author_user_id) =
                verify_certificate_signature!(Device, unsecure, ops, store)?;

            // 3) The certificate is valid, last check is the consistency with other certificates
            check_user_update_certificate_consistency(ops, store, &cooked, author_user_id).await?;

            Ok(CommonTopicArcCertificate::UserUpdate(cooked))
        }
        UnsecureCommonTopicCertificate::RevokedUser(unsecure) => {
            let (cooked, author_user_id) =
                verify_certificate_signature!(Device, unsecure, ops, store)?;

            // 3) The certificate is valid, last check is the consistency with other certificates
            check_revoked_user_certificate_consistency(ops, store, &cooked, author_user_id).await?;

            Ok(CommonTopicArcCertificate::RevokedUser(cooked))
        }
    }
}

/// Validate the given certificate by both checking it content (format, signature, etc.)
/// and it global consistency with the others certificates.
async fn validate_realm_certificate(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    realm_id: VlobID,
    signed: Bytes,
) -> Result<RealmTopicArcCertificate, CertifAddCertificatesBatchError> {
    // 1) Deserialize the certificate first

    let unsecure = match RealmTopicCertificate::unsecure_load(signed) {
        Ok(unsecure) => unsecure,
        Err(error) => {
            // No information can be extracted from the binary data...
            let hint = "<unknown>".into();
            let what = Box::new(InvalidCertificateError::Corrupted { hint, error });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    };

    // 2) Verify the certificate signature
    // By doing so we also verify author's device existence, but we don't go
    // any further, hence additional checks on author must be done at step 3 !

    match unsecure {
        UnsecureRealmTopicCertificate::RealmRole(unsecure) => {
            let (cooked, author_user_id) =
                verify_certificate_signature!(Device, unsecure, ops, store)?;

            // 3) Ensure the certificate corresponds to the considered realm
            if cooked.realm_id != realm_id {
                let hint = format!("{:?}", cooked);
                let error = DataError::UnexpectedRealmID {
                    expected: realm_id,
                    got: cooked.realm_id,
                };
                let what = Box::new(InvalidCertificateError::Corrupted { hint, error });
                return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
            }

            // 4) The certificate is valid, last check is the consistency with other certificates
            check_realm_role_certificate_consistency(store, &cooked, author_user_id).await?;

            Ok(RealmTopicArcCertificate::RealmRole(cooked))
        }
        UnsecureRealmTopicCertificate::RealmName(unsecure) => {
            let (cooked, author_user_id) =
                verify_certificate_signature!(Device, unsecure, ops, store)?;

            // 3) Ensure the certificate corresponds to the considered realm
            if cooked.realm_id != realm_id {
                let hint = format!("{:?}", cooked);
                let error = DataError::UnexpectedRealmID {
                    expected: realm_id,
                    got: cooked.realm_id,
                };
                let what = Box::new(InvalidCertificateError::Corrupted { hint, error });
                return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
            }

            // 4) The certificate is valid, last check is the consistency with other certificates
            check_realm_name_certificate_consistency(ops, store, &cooked, author_user_id).await?;

            Ok(RealmTopicArcCertificate::RealmName(cooked))
        }
        UnsecureRealmTopicCertificate::RealmKeyRotation(unsecure) => {
            let (cooked, author_user_id) =
                verify_certificate_signature!(Device, unsecure, ops, store)?;

            // 3) Ensure the certificate corresponds to the considered realm
            if cooked.realm_id != realm_id {
                let hint = format!("{:?}", cooked);
                let error = DataError::UnexpectedRealmID {
                    expected: realm_id,
                    got: cooked.realm_id,
                };
                let what = Box::new(InvalidCertificateError::Corrupted { hint, error });
                return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
            }

            // 4) The certificate is valid, last check is the consistency with other certificates
            check_realm_key_rotation_certificate_consistency(ops, store, &cooked, author_user_id)
                .await?;

            Ok(RealmTopicArcCertificate::RealmKeyRotation(cooked))
        }
        UnsecureRealmTopicCertificate::RealmArchiving(unsecure) => {
            let (cooked, author_user_id) =
                verify_certificate_signature!(Device, unsecure, ops, store)?;

            // 3) Ensure the certificate corresponds to the considered realm
            if cooked.realm_id != realm_id {
                let hint = format!("{:?}", cooked);
                let error = DataError::UnexpectedRealmID {
                    expected: realm_id,
                    got: cooked.realm_id,
                };
                let what = Box::new(InvalidCertificateError::Corrupted { hint, error });
                return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
            }

            // 4) The certificate is valid, last check is the consistency with other certificates
            check_realm_archiving_certificate_consistency(ops, store, &cooked, author_user_id)
                .await?;

            Ok(RealmTopicArcCertificate::RealmArchiving(cooked))
        }
    }
}

/// Validate the given certificate by both checking it content (format, signature, etc.)
/// and it global consistency with the others certificates.
async fn validate_shamir_recovery_certificate(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    signed: Bytes,
) -> Result<ShamirRecoveryTopicArcCertificate, CertifAddCertificatesBatchError> {
    // 1) Deserialize the certificate

    let unsecure = match ShamirRecoveryTopicCertificate::unsecure_load(signed) {
        Ok(unsecure) => unsecure,
        Err(error) => {
            // No information can be extracted from the binary data...
            let hint = "<unknown>".into();
            let what = Box::new(InvalidCertificateError::Corrupted { hint, error });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    };

    // 2) Verify the certificate signature
    // By doing so we also verify author's device existence, but we don't go
    // any further, hence additional checks on author must be done at step 3 !

    match unsecure {
        UnsecureShamirRecoveryTopicCertificate::ShamirRecoveryBrief(unsecure) => {
            let (cooked, author_user_id) =
                verify_certificate_signature!(Device, unsecure, ops, store)?;

            // 3) The certificate is valid, last check is the consistency with other certificates
            check_shamir_recovery_brief_certificate_consistency(
                ops,
                store,
                &cooked,
                author_user_id,
            )
            .await?;

            Ok(ShamirRecoveryTopicArcCertificate::ShamirRecoveryBrief(
                cooked,
            ))
        }
        UnsecureShamirRecoveryTopicCertificate::ShamirRecoveryShare(unsecure) => {
            let (cooked, author_user_id) =
                verify_certificate_signature!(Device, unsecure, ops, store)?;

            // 3) The certificate is valid, last check is the consistency with other certificates
            check_shamir_recovery_share_certificate_consistency(
                ops,
                store,
                &cooked,
                author_user_id,
            )
            .await?;

            Ok(ShamirRecoveryTopicArcCertificate::ShamirRecoveryShare(
                cooked,
            ))
        }
        UnsecureShamirRecoveryTopicCertificate::ShamirRecoveryDeletion(unsecure) => {
            let (cooked, author_user_id) =
                verify_certificate_signature!(Device, unsecure, ops, store)?;

            // 3) The certificate is valid, last check is the consistency with other certificates
            check_shamir_recovery_deletion_certificate_consistency(
                ops,
                store,
                &cooked,
                author_user_id,
            )
            .await?;

            Ok(ShamirRecoveryTopicArcCertificate::ShamirRecoveryDeletion(
                cooked,
            ))
        }
    }
}

/// Validate the given certificate by both checking it content (format, signature, etc.)
/// and it global consistency with the others certificates.
async fn validate_sequester_certificate(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    signed: Bytes,
) -> Result<SequesterTopicArcCertificate, CertifAddCertificatesBatchError> {
    let maybe_last_timestamp = store.get_last_timestamps().await?.sequester;
    match maybe_last_timestamp {
        // No sequester certificates currently exists, that means we must be currently
        // validating the sequester authority certificate.
        None => validate_sequester_authority_certificate(ops, store, signed)
            .await
            .map(SequesterTopicArcCertificate::SequesterAuthority),
        // Some sequester certificates already exist. Hence the current certificate
        // cannot be a sequester authority.
        Some(last_sequester_timestamp) => {
            validate_sequester_non_authority_certificate(store, last_sequester_timestamp, signed)
                .await
        }
    }
}

async fn validate_sequester_authority_certificate(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    signed: Bytes,
) -> Result<Arc<SequesterAuthorityCertificate>, CertifAddCertificatesBatchError> {
    // 1) Deserialize the certificate first

    let unsecure = match SequesterAuthorityCertificate::unsecure_load(signed) {
        Ok(unsecure) => unsecure,
        Err(error) => {
            // No information can be extracted from the binary data...
            let hint = "<unknown>".into();
            let what = Box::new(InvalidCertificateError::Corrupted { hint, error });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    };

    // 2) Verify the certificate signature

    let cooked = unsecure
        .verify_signature(ops.device.root_verify_key())
        .map(|(certif, _)| Arc::new(certif))
        .map_err(|(unsecure, error)| {
            let hint = unsecure.hint();
            Box::new(InvalidCertificateError::Corrupted { hint, error })
        })?;

    // 4) The certificate is valid, last check is the consistency with other certificates

    check_sequester_authority_certificate_consistency(store, &cooked).await?;

    Ok(cooked)
}

async fn validate_sequester_non_authority_certificate(
    store: &mut CertificatesStoreWriteGuard<'_>,
    last_stored_sequester_timestamp: DateTime,
    signed: Bytes,
) -> Result<SequesterTopicArcCertificate, CertifAddCertificatesBatchError> {
    // 1) Ensure the organization is a sequestered one

    let authority = match store.get_sequester_authority_certificate().await? {
        Some(authority) => authority,
        None => {
            // No information can be extracted from the binary data...
            let hint = "<unknown>".into();
            let what = Box::new(InvalidCertificateError::NotASequesteredOrganization { hint });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    };

    // 2) Deserialize the certificate and verify its signature

    match SequesterServiceCertificate::verify_and_load(&signed, &authority.verify_key_der) {
        Ok(cooked) => {
            let cooked = Arc::new(cooked);

            // 3) The certificate is valid, last check is the consistency with other certificates
            check_sequester_service_certificate_consistency(
                store,
                last_stored_sequester_timestamp,
                &cooked,
            )
            .await?;

            Ok(SequesterTopicArcCertificate::SequesterService(cooked))
        }
        Err(_) => match SequesterRevokedServiceCertificate::verify_and_load(
            &signed,
            &authority.verify_key_der,
        ) {
            Ok(cooked) => {
                let cooked = Arc::new(cooked);

                // 3) The certificate is valid, last check is the consistency with other certificates
                check_sequester_revoked_service_certificate_consistency(
                    store,
                    last_stored_sequester_timestamp,
                    &cooked,
                )
                .await?;

                Ok(SequesterTopicArcCertificate::SequesterRevokedService(
                    cooked,
                ))
            }
            Err(error) => {
                // No information can be extracted from the binary data...
                let hint = "<unknown>".into();
                let what = Box::new(InvalidCertificateError::Corrupted { hint, error });
                Err(CertifAddCertificatesBatchError::InvalidCertificate(what))
            }
        },
    }
}

async fn check_user_exists(
    store: &mut CertificatesStoreWriteGuard<'_>,
    up_to: DateTime,
    user_id: UserID,
    mk_hint: impl FnOnce() -> String,
) -> Result<Arc<UserCertificate>, CertifAddCertificatesBatchError> {
    match store
        .get_user_certificate(UpTo::Timestamp(up_to), user_id.to_owned())
        .await
    {
        // User exists as we expected :)
        Ok(certif) => Ok(certif),

        // User doesn't exist :(
        Err(GetCertificateError::NonExisting) => {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::NonExistingRelatedUser { hint });
            Err(CertifAddCertificatesBatchError::InvalidCertificate(what))
        }

        // User doesn't exist... yet :(
        Err(GetCertificateError::ExistButTooRecent {
            certificate_timestamp,
            ..
        }) => {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::OlderThanRelatedUser {
                hint,
                user_created_on: certificate_timestamp,
            });
            Err(CertifAddCertificatesBatchError::InvalidCertificate(what))
        }

        // D'oh :/
        Err(err @ GetCertificateError::Internal(_)) => {
            Err(CertifAddCertificatesBatchError::Internal(err.into()))
        }
    }
}

async fn check_user_not_revoked(
    store: &mut CertificatesStoreWriteGuard<'_>,
    up_to: DateTime,
    user_id: UserID,
    mk_hint: impl FnOnce() -> String,
) -> Result<(), CertifAddCertificatesBatchError> {
    match store
        .get_revoked_user_certificate(UpTo::Timestamp(up_to), user_id.to_owned())
        .await?
    {
        // Not revoked, as expected :)
        None => (),

        // User can only be revoked once :(
        Some(revoked_certificate) => {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::RelatedUserAlreadyRevoked {
                hint,
                user_revoked_on: revoked_certificate.timestamp,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    Ok(())
}

/// Admin existence has already been checked while fetching it device's verify key,
/// so what's left is checking it is not revoked and (optionally) it profile
async fn check_author_not_revoked_and_profile(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    up_to: DateTime,
    author: UserID,
    author_must_be_admin: bool,
    mk_hint: impl FnOnce() -> String + std::marker::Copy,
) -> Result<(), CertifAddCertificatesBatchError> {
    match store
        .get_revoked_user_certificate(UpTo::Timestamp(up_to), author)
        .await?
    {
        // Not revoked, as expected :)
        None => (),

        // A revoked user cannot author anything !
        Some(revoked_certificate) => {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::RevokedAuthor {
                hint,
                author_revoked_on: revoked_certificate.timestamp,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    if author_must_be_admin {
        match get_user_profile(store, up_to, author, mk_hint, None).await {
            Ok(profile) => {
                if profile != UserProfile::Admin {
                    let hint = mk_hint();
                    let what = Box::new(InvalidCertificateError::AuthorNotAdmin {
                        hint,
                        author_profile: profile,
                    });
                    return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
                }
            }
            // When the check functions are called, device certificate has already
            // been retrieved for the author (to get it verify key).
            // Hence the user certificate must exist and be available (guaranteed by
            // consistency checks when the author device certificate was added).
            //
            // If we end up here, something went very wrong (Parsec bug, or the certificates
            // SQLite database got tempered with), so we don't return this as a regular
            // error but instead as an internal one with error report.
            Err(CertifAddCertificatesBatchError::InvalidCertificate(err)) => {
                // TODO: improve error logging (struct log, org/device already captured by default, sentry compat etc.)
                let org = ops.device.organization_id();
                let device_id = &ops.device.device_id;
                log::error!("{org}#{device_id}: Got author device, but fail to get corresponding user: {err:?}");
                return Err(CertifAddCertificatesBatchError::Internal(anyhow::anyhow!(
                    "Got author device, but fail to get corresponding user: {err:?}"
                )));
            }
            // D'oh :/
            Err(err) => return Err(err),
        }
    }

    Ok(())
}

async fn get_user_profile(
    store: &mut CertificatesStoreWriteGuard<'_>,
    up_to: DateTime,
    user_id: UserID,
    mk_hint: impl FnOnce() -> String,
    already_fetched_user_certificate: Option<Arc<UserCertificate>>,
) -> Result<UserProfile, CertifAddCertificatesBatchError> {
    // Updates overwrite each others, so last one contains the current profile
    let maybe_update = store
        .get_last_user_update_certificate(UpTo::Timestamp(up_to), user_id.to_owned())
        .await?;
    if let Some(last_update) = maybe_update {
        return Ok(last_update.new_profile);
    }

    // No updates, the current profile is the one specified in the user certificate
    let user_certificate = match already_fetched_user_certificate {
        Some(user_certificate) => user_certificate,
        None => check_user_exists(store, up_to, user_id, mk_hint).await?,
    };

    Ok(user_certificate.profile)
}

async fn check_user_certificate_consistency(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    cooked: &UserCertificate,
    author_user_id: Option<UserID>,
) -> Result<(), CertifAddCertificatesBatchError> {
    let mk_hint = || format!("{:?}", cooked);

    // 1) Certificate must be the newest among the ones in common topic.
    //
    // There is however a single case where we accept same timestamp: during
    // user creation, a user and device certificates are created together.
    // In that case the device certificate always provided second, so we don't
    // have any check to do here.
    //
    // On top of that, the sequester authority certificate (if it exists) have
    // the same timestamp than the first user certificate. However they belong
    // to different topics so they just going to be ignored in this step (and
    // actually checked at step 2).

    // It is possible for the storage to be empty here (if we are validating the
    // very first user certificate that is created during organization bootstrap)
    let last_stored_common_timestamp = store.get_last_timestamps().await?.common;
    if let Some(last_stored_timestamp) = &last_stored_common_timestamp {
        if &cooked.timestamp <= last_stored_timestamp {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::InvalidTimestamp {
                hint,
                last_certificate_timestamp: *last_stored_timestamp,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    // 2) Check author

    match author_user_id {
        // 2.a) If author is a user, it should have has ADMIN profile and not be revoked.
        Some(author_user_id) => {
            check_author_not_revoked_and_profile(
                ops,
                store,
                cooked.timestamp,
                author_user_id,
                true,
                mk_hint,
            )
            .await?;
        }
        // 2.b) Otherwise we are checking the first user created during organization bootstrap.
        // In that case no other certificates should be present (except for the
        // sequester authority certificate if it exists).
        None => {
            // Other topics depend on common topic (i.e. their certificates are signed
            // by a device), so need to ensure they are empty.
            if last_stored_common_timestamp.is_some() {
                let hint = mk_hint();
                let what = Box::new(InvalidCertificateError::RootSignatureOutOfBootstrap { hint });
                return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
            }
            if let Some(sequester_authority_certif) =
                store.get_sequester_authority_certificate().await?
            {
                if sequester_authority_certif.timestamp != cooked.timestamp {
                    let hint = mk_hint();
                    let what = Box::new(InvalidCertificateError::RootSignatureTimestampMismatch {
                        hint,
                        last_root_signature_timestamp: sequester_authority_certif.timestamp,
                    });
                    return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
                }
            }
        }
    }

    // 3) Make sure the user doesn't already exists

    match store
        .get_user_certificate(UpTo::Timestamp(cooked.timestamp), cooked.user_id)
        .await
    {
        // This user doesn't already exist, this is what we expected :)
        Err(GetCertificateError::NonExisting) => (),

        // This user already exists :(
        Ok(_) => {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::ContentAlreadyExists { hint });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }

        // This case should never happen given we have already checked our current
        // certificate is more recent than the ones stored.
        Err(GetCertificateError::ExistButTooRecent {
            certificate_timestamp,
            ..
        }) => {
            // TODO: improve error logging (struct log, org/device already captured by default, sentry compat etc.)
            let org = ops.device.organization_id();
            let device_id = &ops.device.device_id;
            let hint = mk_hint();
            let timestamp = cooked.timestamp;
            log::error!("{org}#{device_id}: `{hint}`: already checked timestamp, but now storage says it is too recent ({timestamp} vs {certificate_timestamp} in storage)");
            return Err(CertifAddCertificatesBatchError::Internal(anyhow::anyhow!("`{hint}`: already checked timestamp, but now storage says it is too recent ({timestamp} vs {certificate_timestamp} in storage)")));
        }

        // D'oh :/
        Err(err @ GetCertificateError::Internal(_)) => {
            return Err(CertifAddCertificatesBatchError::Internal(err.into()));
        }
    }

    // If the user doesn't already exist, we are guaranteed we also don't have other
    // types of certificate related to this user ID in the local database
    // So those conditions are already guaranteed:
    // - the user has no related device
    // - the user is not revoked
    // - the user certificate is not signed by one of the user's devices

    Ok(())
}

async fn check_device_certificate_consistency(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    cooked: &DeviceCertificate,
    author_user_id: Option<UserID>,
) -> Result<(), CertifAddCertificatesBatchError> {
    let mk_hint = || format!("{:?}", cooked);
    let user_id = cooked.user_id;

    let is_first_user_device = match author_user_id {
        // The user has created a new device for himself
        Some(author_user_id) if author_user_id == user_id => false,
        // The author is an ADMIN that have enrolled the user
        Some(_) => true,
        // First device of the user that have bootstrapped the organization
        None => true,
    };

    // 1) Certificate must be the newest among the ones in common topic.
    // There is however a single case where we accept same timestamp: during
    // user creation, a user and device certificates are created together.

    // It is possible for the storage to be empty here: if we are validating the
    // very first device certificate that is created during organization bootstrap.
    // This is unlikely though given the user certificate is required to be provided
    // first (and if that's not the case the validation will fail in later check).
    let last_stored_common_timestamp = store.get_last_timestamps().await?.common;
    if let Some(last_stored_timestamp) = &last_stored_common_timestamp {
        // If `is_first_user_device` is true, then the previous certificate must be the
        // user certificate which has been created together with our device certificate
        // (the it actually is is checked at step 3).
        match (
            cooked.timestamp.cmp(last_stored_timestamp),
            is_first_user_device,
        ) {
            (std::cmp::Ordering::Less, _) | (std::cmp::Ordering::Equal, false) => {
                let hint = mk_hint();
                let what = Box::new(InvalidCertificateError::InvalidTimestamp {
                    hint,
                    last_certificate_timestamp: *last_stored_timestamp,
                });
                return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
            }
            _ => (),
        }
    }

    // 2) Check author

    match author_user_id {
        // 2.a) Same-user-signed: the user has created a new device for himself (hence doesn't need to be ADMIN)
        Some(author_user_id) if author_user_id == user_id => {
            check_author_not_revoked_and_profile(
                ops,
                store,
                cooked.timestamp,
                author_user_id,
                false,
                mk_hint,
            )
            .await?;
        }

        // 2.b) Not same-user-signed: The author is an ADMIN that have enrolled the user
        Some(author_user_id) => {
            check_author_not_revoked_and_profile(
                ops,
                store,
                cooked.timestamp,
                author_user_id,
                true,
                mk_hint,
            )
            .await?;
        }

        // 2.c) Not same-user-signed: First device of the user that have bootstrapped the organization
        // In that case no other certificates should be present (except for the corresponding
        // user certificate and the sequester authority certificate if it exists).
        None => (),
    }

    // 3) Make sure the user exists

    let user_certif = check_user_exists(store, cooked.timestamp, user_id, mk_hint).await?;
    if is_first_user_device {
        // Only the very first device of a user can be signed by someone else. Hence
        // the previous certificate must be user certificate with the same timestamp
        // and author than our current one.
        if user_certif.author != cooked.author {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::UserFirstDeviceAuthorMismatch {
                hint,
                user_author: user_certif.author,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
        if user_certif.timestamp != cooked.timestamp {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::UserFirstDeviceTimestampMismatch {
                hint,
                user_timestamp: user_certif.timestamp,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    // 4) Make sure the user is not already revoked

    check_user_not_revoked(store, cooked.timestamp, user_id, mk_hint).await?;

    // 5) Make sure this device doesn't already exist

    match store
        .get_device_certificate(
            UpTo::Timestamp(cooked.timestamp),
            cooked.device_id.to_owned(),
        )
        .await
    {
        // This device doesn't already exist, this is what we expected :)
        Err(GetCertificateError::NonExisting) => (),

        // This device already exists :(
        Ok(_) => {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::ContentAlreadyExists { hint });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }

        // This case should never happen given we have already checked our current
        // certificate is more recent than the ones stored.
        Err(GetCertificateError::ExistButTooRecent {
            certificate_timestamp,
            ..
        }) => {
            // TODO: improve error logging (struct log, org/device already captured by default, sentry compat etc.)
            let org = ops.device.organization_id();
            let device_id = &ops.device.device_id;
            let hint = mk_hint();
            let timestamp = cooked.timestamp;
            log::error!("{org}#{device_id}: `{hint}`: already checked timestamp, but now storage says it is too recent ({timestamp} vs {certificate_timestamp} in storage)");
            return Err(CertifAddCertificatesBatchError::Internal(anyhow::anyhow!("`{hint}`: already checked timestamp, but now storage says it is too recent ({timestamp} vs {certificate_timestamp} in storage)")));
        }

        // D'oh :/
        Err(err @ GetCertificateError::Internal(_)) => {
            return Err(CertifAddCertificatesBatchError::Internal(err.into()));
        }
    }

    Ok(())
}

async fn check_user_update_certificate_consistency(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    cooked: &UserUpdateCertificate,
    author_user_id: UserID,
) -> Result<(), CertifAddCertificatesBatchError> {
    let mk_hint = || format!("{:?}", cooked);

    // 1) Certificate must be the newest among the ones in common topic.
    // Note we also reject same timestamp given revoked user certificates is
    // always created alone.

    // We have already fetched the author's device certificate while validating the
    // certificate, so `last_stored_timestamp` should never be `None`.
    // And even if that's the case, the following checks involve fetching stored
    // certificates and hence will fail.
    if let Some(last_stored_timestamp) = store.get_last_timestamps().await?.common {
        if cooked.timestamp <= last_stored_timestamp {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::InvalidTimestamp {
                hint,
                last_certificate_timestamp: last_stored_timestamp,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    // 2) Check the certificate is not self-signed

    if author_user_id == cooked.user_id {
        let hint = mk_hint();
        let what = Box::new(InvalidCertificateError::SelfSigned { hint });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    }

    // 3) Check author is not revoked and has ADMIN profile

    check_author_not_revoked_and_profile(
        ops,
        store,
        cooked.timestamp,
        author_user_id,
        true,
        mk_hint,
    )
    .await?;

    // 4) Make sure the user exists

    let user_certificate =
        check_user_exists(store, cooked.timestamp, cooked.user_id, mk_hint).await?;

    // 5) Make sure the user is not already revoked

    check_user_not_revoked(store, cooked.timestamp, cooked.user_id, mk_hint).await?;

    // 6) Make sure the user doesn't already have this profile

    let user_current_profile = get_user_profile(
        store,
        cooked.timestamp,
        cooked.user_id,
        mk_hint,
        Some(user_certificate),
    )
    .await?;

    if user_current_profile == cooked.new_profile {
        let hint = mk_hint();
        let what = Box::new(InvalidCertificateError::ContentAlreadyExists { hint });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    }

    // Note that an Outsider is not allowed to be Owner/Manager of a realm, however we
    // allow it if it was due to a profile change. Hence here we don't need to check the
    // role the user has in the realms he is part of.

    Ok(())
}

async fn check_revoked_user_certificate_consistency(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    cooked: &RevokedUserCertificate,
    author_user_id: UserID,
) -> Result<(), CertifAddCertificatesBatchError> {
    let mk_hint = || format!("{:?}", cooked);

    // 1) Certificate must be the newest among the ones in common topic.
    // Note we also reject same timestamp given revoked user certificates is
    // always created alone.

    // We have already fetched the author's device certificate while validating the
    // certificate, so `last_stored_timestamp` should never be `None`.
    // And even if that's the case, the following checks involve fetching stored
    // certificates and hence will fail.
    if let Some(last_stored_timestamp) = store.get_last_timestamps().await?.common {
        if cooked.timestamp <= last_stored_timestamp {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::InvalidTimestamp {
                hint,
                last_certificate_timestamp: last_stored_timestamp,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    // 2) Check the certificate is not self-signed

    if author_user_id == cooked.user_id {
        let hint = mk_hint();
        let what = Box::new(InvalidCertificateError::SelfSigned { hint });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    }

    // 3) Check author is not revoked and has ADMIN profile

    check_author_not_revoked_and_profile(
        ops,
        store,
        cooked.timestamp,
        author_user_id,
        true,
        mk_hint,
    )
    .await?;

    // 4) Make sure the user exists

    check_user_exists(store, cooked.timestamp, cooked.user_id, mk_hint).await?;

    // 5) Make sure the user is not already revoked

    check_user_not_revoked(store, cooked.timestamp, cooked.user_id, mk_hint).await?;

    Ok(())
}

async fn check_realm_role_certificate_consistency(
    store: &mut CertificatesStoreWriteGuard<'_>,
    cooked: &RealmRoleCertificate,
    author_user_id: UserID,
) -> Result<(), CertifAddCertificatesBatchError> {
    let mk_hint = || format!("{:?}", cooked);

    // 1) Certificate must be the newest among the ones in it realm's topic.
    // Note we also reject same timestamp given realm role certificate is always
    // created alone.

    let last_stored_timestamp = store
        .get_last_timestamps()
        .await?
        .realm
        .get(&cooked.realm_id)
        .cloned();
    if let Some(last_stored_timestamp) = last_stored_timestamp {
        if cooked.timestamp <= last_stored_timestamp {
            // We already know more recent certificates, hence this certificate
            // cannot be added without breaking causality !
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::InvalidTimestamp {
                hint,
                last_certificate_timestamp: last_stored_timestamp,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    // 2) Check author's realm role and if certificate is self-signed.

    let realm_roles = store
        .get_realm_roles(UpTo::Timestamp(cooked.timestamp), cooked.realm_id)
        .await?;
    let is_realm_creation = realm_roles.is_empty();

    if is_realm_creation {
        // 2.a) The realm is a new one, so certificate must be self-signed with a OWNER role.

        if author_user_id != cooked.user_id {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::RealmFirstRoleMustBeSelfSigned { hint });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }

        if cooked.role != Some(RealmRole::Owner) {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::RealmFirstRoleMustBeOwner { hint });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    } else {
        // 2.b) The realm already exists, so certificate cannot be self-signed and
        // author must have a sufficient role.

        let author_current_role = match realm_roles
            .iter()
            .rev() // realm roles are sorted from oldest to newest, we need the last one
            .find(|role| role.user_id == author_user_id)
            .and_then(|role| role.role)
        {
            // As expected, author currently has a role :)
            Some(author_current_role) => author_current_role,
            // Author never had role, or its role got revoked :(
            None => {
                let hint = mk_hint();
                let what = Box::new(InvalidCertificateError::RealmAuthorHasNoRole { hint });
                return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
            }
        };

        let user_current_role = realm_roles
            .iter()
            .rev() // realm roles are sorted from oldest to newest, we need the last one
            .find(|role| role.user_id == cooked.user_id)
            .and_then(|role| role.role);
        let user_new_role = cooked.role;

        match (user_current_role, user_new_role) {
            // Cannot remove the role if the user already doesn't have one !
            (None, None) => {
                let hint = mk_hint();
                let what = Box::new(InvalidCertificateError::ContentAlreadyExists { hint });
                return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
            }
            // The certificate must provide a new role !
            (Some(current_role), Some(new_role)) if current_role == new_role => {
                let hint = mk_hint();
                let what = Box::new(InvalidCertificateError::ContentAlreadyExists { hint });
                return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
            }
            // Only an Owner can give/remove Owner/Manager roles
            (Some(RealmRole::Owner), _)
            | (Some(RealmRole::Manager), _)
            | (_, Some(RealmRole::Owner))
            | (_, Some(RealmRole::Manager)) => {
                if author_current_role != RealmRole::Owner {
                    let hint = mk_hint();
                    let what = Box::new(InvalidCertificateError::RealmAuthorNotOwner {
                        hint,
                        author_role: author_current_role,
                    });
                    return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
                }
            }
            // Owner and Manager can both give/remove Reader/Contributor roles
            (Some(RealmRole::Reader), _)
            | (Some(RealmRole::Contributor), _)
            | (_, Some(RealmRole::Reader))
            | (_, Some(RealmRole::Contributor)) => {
                if author_current_role != RealmRole::Owner
                    && author_current_role != RealmRole::Manager
                {
                    let hint = mk_hint();
                    let what = Box::new(InvalidCertificateError::RealmAuthorNotOwnerOrManager {
                        hint,
                        author_role: author_current_role,
                    });
                    return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
                }
            }
        }
    }

    // 3) Make sure the user exists

    let user_certificate =
        check_user_exists(store, cooked.timestamp, cooked.user_id, mk_hint).await?;

    // 4) Make sure the user is not already revoked if we are giving it a role.
    // (note revoked users *will* be revoked from any realm they are part of once
    // an OWNER notifies the change).

    let user_new_role = cooked.role;
    if user_new_role.is_some() {
        check_user_not_revoked(store, cooked.timestamp, cooked.user_id, mk_hint).await?;
    }

    // 5) Make sure the user's profile is compatible with the realm and its given role

    let profile = get_user_profile(
        store,
        cooked.timestamp,
        cooked.user_id,
        mk_hint,
        Some(user_certificate),
    )
    .await?;

    match (profile, cooked.role) {
        // Be exhaustive on all allowed cases, as this would help whenever
        // `UserProfile`/`RealmRole` gets modified.
        (_, None)
        | (UserProfile::Outsider, Some(RealmRole::Reader | RealmRole::Contributor))
        | (UserProfile::Standard | UserProfile::Admin, _) => (),
        // Forbidden cases
        (UserProfile::Outsider, Some(RealmRole::Manager | RealmRole::Owner)) => {
            let hint = mk_hint();
            let what =
                Box::new(InvalidCertificateError::RealmOutsiderCannotBeOwnerOrManager { hint });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    Ok(())
}

async fn check_realm_name_certificate_consistency(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    cooked: &RealmNameCertificate,
    author_user_id: UserID,
) -> Result<(), CertifAddCertificatesBatchError> {
    let mk_hint = || format!("{:?}", cooked);

    // 1) Certificate must be the newest among the ones in it realm's topic.
    // Note we also reject same timestamp given realm role certificate is always
    // created alone.

    let last_stored_timestamp = store
        .get_last_timestamps()
        .await?
        .realm
        .get(&cooked.realm_id)
        .cloned();
    if let Some(last_stored_timestamp) = last_stored_timestamp {
        if cooked.timestamp <= last_stored_timestamp {
            // We already know more recent certificates, hence this certificate
            // cannot be added without breaking causality !
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::InvalidTimestamp {
                hint,
                last_certificate_timestamp: last_stored_timestamp,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    // 2) Check author's realm role and if certificate is self-signed.

    let realm_roles = store
        .get_realm_roles(UpTo::Timestamp(cooked.timestamp), cooked.realm_id)
        .await?;

    if realm_roles.is_empty() {
        // 2.a) The realm is a new one, but only realm role certificate are allowed at this time !
        let hint = mk_hint();
        let what = Box::new(InvalidCertificateError::RealmFirstCertificateMustBeRole { hint });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    } else {
        // 2.b) The realm already exists, as expected.

        let author_current_role = match realm_roles
            .iter()
            .rev() // realm roles are sorted from oldest to newest, we need the last one
            .find(|role| role.user_id == author_user_id)
            .and_then(|role| role.role)
        {
            // As expected, author currently has a role :)
            Some(author_current_role) => author_current_role,
            // Author never had role, or it role got revoked :(
            None => {
                let hint = mk_hint();
                let what = Box::new(InvalidCertificateError::RealmAuthorHasNoRole { hint });
                return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
            }
        };

        // Only an Owner can change the realm's name
        if author_current_role != RealmRole::Owner {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::RealmAuthorNotOwner {
                hint,
                author_role: author_current_role,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    // 3) Make sure the author is not already revoked

    check_author_not_revoked_and_profile(
        ops,
        store,
        cooked.timestamp,
        author_user_id,
        false,
        mk_hint,
    )
    .await?;

    Ok(())
}

async fn check_realm_key_rotation_certificate_consistency(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    cooked: &RealmKeyRotationCertificate,
    author_user_id: UserID,
) -> Result<(), CertifAddCertificatesBatchError> {
    let mk_hint = || format!("{:?}", cooked);

    // 1) Certificate must be the newest among the ones in it realm's topic.
    // Note we also reject same timestamp given realm role certificate is always
    // created alone.

    let last_stored_timestamp = store
        .get_last_timestamps()
        .await?
        .realm
        .get(&cooked.realm_id)
        .cloned();
    if let Some(last_stored_timestamp) = last_stored_timestamp {
        if cooked.timestamp <= last_stored_timestamp {
            // We already know more recent certificates, hence this certificate
            // cannot be added without breaking causality !
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::InvalidTimestamp {
                hint,
                last_certificate_timestamp: last_stored_timestamp,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    // 2) Check author's realm role and if certificate is self-signed.

    let realm_roles = store
        .get_realm_roles(UpTo::Timestamp(cooked.timestamp), cooked.realm_id)
        .await?;

    if realm_roles.is_empty() {
        // 2.a) The realm is a new one, but only realm role certificate are allowed at this time !
        let hint = mk_hint();
        let what = Box::new(InvalidCertificateError::RealmFirstCertificateMustBeRole { hint });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    } else {
        // 2.b) The realm already exists, as expected.

        let author_current_role = match realm_roles
            .iter()
            .rev() // realm roles are sorted from oldest to newest, we need the last one
            .find(|role| role.user_id == author_user_id)
            .and_then(|role| role.role)
        {
            // As expected, author currently has a role :)
            Some(author_current_role) => author_current_role,
            // Author never had role, or it role got revoked :(
            None => {
                let hint = mk_hint();
                let what = Box::new(InvalidCertificateError::RealmAuthorHasNoRole { hint });
                return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
            }
        };

        // Only an Owner can rotate the realm's key
        if author_current_role != RealmRole::Owner {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::RealmAuthorNotOwner {
                hint,
                author_role: author_current_role,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    // 3) Make sure the author is not already revoked

    check_author_not_revoked_and_profile(
        ops,
        store,
        cooked.timestamp,
        author_user_id,
        false,
        mk_hint,
    )
    .await?;

    Ok(())
}

async fn check_realm_archiving_certificate_consistency(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    cooked: &RealmArchivingCertificate,
    author_user_id: UserID,
) -> Result<(), CertifAddCertificatesBatchError> {
    let mk_hint = || format!("{:?}", cooked);

    // 1) Certificate must be the newest among the ones in it realm's topic.
    // Note we also reject same timestamp given realm role certificate is always
    // created alone.

    let last_stored_timestamp = store
        .get_last_timestamps()
        .await?
        .realm
        .get(&cooked.realm_id)
        .cloned();
    if let Some(last_stored_timestamp) = last_stored_timestamp {
        if cooked.timestamp <= last_stored_timestamp {
            // We already know more recent certificates, hence this certificate
            // cannot be added without breaking causality !
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::InvalidTimestamp {
                hint,
                last_certificate_timestamp: last_stored_timestamp,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    // 2) Check author's realm role and if certificate is self-signed.

    let realm_roles = store
        .get_realm_roles(UpTo::Timestamp(cooked.timestamp), cooked.realm_id)
        .await?;

    if realm_roles.is_empty() {
        // 2.a) The realm is a new one, but only realm role certificate are allowed at this time !
        let hint = mk_hint();
        let what = Box::new(InvalidCertificateError::RealmFirstCertificateMustBeRole { hint });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    } else {
        // 2.b) The realm already exists, as expected.

        let author_current_role = match realm_roles
            .iter()
            .rev() // realm roles are sorted from oldest to newest, we need the last one
            .find(|role| role.user_id == author_user_id)
            .and_then(|role| role.role)
        {
            // As expected, author currently has a role :)
            Some(author_current_role) => author_current_role,
            // Author never had role, or it role got revoked :(
            None => {
                let hint = mk_hint();
                let what = Box::new(InvalidCertificateError::RealmAuthorHasNoRole { hint });
                return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
            }
        };

        // Only an Owner can archive realm
        if author_current_role != RealmRole::Owner {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::RealmAuthorNotOwner {
                hint,
                author_role: author_current_role,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    // 3) Make sure the author is not already revoked

    check_author_not_revoked_and_profile(
        ops,
        store,
        cooked.timestamp,
        author_user_id,
        false,
        mk_hint,
    )
    .await?;

    Ok(())
}

async fn check_shamir_recovery_brief_certificate_consistency(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    cooked: &ShamirRecoveryBriefCertificate,
    author_user_id: UserID,
) -> Result<(), CertifAddCertificatesBatchError> {
    let mk_hint = || format!("{:?}", cooked);

    // 1) Certificate must be the newest among the ones in shamir recovery topic.
    // Note we also reject same timestamp given, while shamir recovery brief
    // certificate is created along with it related shamir recovery share certificates,
    // the brief certificate is guaranteed to be provided first by the server.

    let last_stored_timestamp = store.get_last_timestamps().await?.shamir_recovery;
    if let Some(last_stored_timestamp) = last_stored_timestamp {
        if cooked.timestamp <= last_stored_timestamp {
            // We already know more recent certificates, hence this certificate
            // cannot be added without breaking causality !
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::InvalidTimestamp {
                hint,
                last_certificate_timestamp: last_stored_timestamp,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    // 2) Check `user_id` field matches with author

    if cooked.user_id != author_user_id {
        let hint = mk_hint();
        let what = Box::new(InvalidCertificateError::ShamirRecoveryNotAboutSelf {
            hint,
            user_id: cooked.user_id,
            author_user_id,
        });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    }

    // 3) Check author is not part of the recipients

    if cooked.per_recipient_shares.contains_key(&author_user_id) {
        let hint = mk_hint();
        let what = Box::new(InvalidCertificateError::ShamirRecoverySelfAmongRecipients { hint });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    }

    // 4) Check we are related to this shamir recovery

    if cooked.user_id != ops.device.user_id
        && !cooked
            .per_recipient_shares
            .contains_key(&ops.device.user_id)
    {
        let hint = mk_hint();
        let what = Box::new(InvalidCertificateError::ShamirRecoveryUnrelatedToUs { hint });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    }

    // 5) Make sure there is no shamir recovery already valid for this user

    let last_shamir_recovery = store
        .get_last_shamir_recovery_for_author(UpTo::Timestamp(cooked.timestamp), author_user_id)
        .await?;
    if let LastShamirRecovery::Valid(_) = last_shamir_recovery {
        let hint = mk_hint();
        let what = Box::new(InvalidCertificateError::ShamirRecoveryAlreadySetup { hint });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    }

    // 6) Check author is not revoked

    check_author_not_revoked_and_profile(
        ops,
        store,
        cooked.timestamp,
        author_user_id,
        false,
        mk_hint,
    )
    .await?;

    // 7) Make sure all recipients exist and are not revoked

    for recipient in cooked.per_recipient_shares.keys() {
        check_user_exists(store, cooked.timestamp, *recipient, mk_hint).await?;
        check_user_not_revoked(store, cooked.timestamp, *recipient, mk_hint).await?;
    }

    Ok(())
}

async fn check_shamir_recovery_share_certificate_consistency(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    cooked: &ShamirRecoveryShareCertificate,
    author_user_id: UserID,
) -> Result<(), CertifAddCertificatesBatchError> {
    let mk_hint = || format!("{:?}", cooked);

    // 1) Shamir recovery share is always created together with a shamir recovery
    // brief.
    // Hence the current certificate timestamp must always be the same as the last stored
    // timestamp (and the last stored certificate must be a shamir recovery brief or share).

    let last_stored_timestamp = store.get_last_timestamps().await?.shamir_recovery;
    if let Some(last_stored_timestamp) = last_stored_timestamp {
        if cooked.timestamp < last_stored_timestamp {
            // We already know more recent certificates, hence this certificate
            // cannot be added without breaking causality !
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::InvalidTimestamp {
                hint,
                last_certificate_timestamp: last_stored_timestamp,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    // 2) Make sure the related brief certificate exists

    match store
        .get_last_shamir_recovery_brief_certificate(UpTo::Timestamp(cooked.timestamp))
        .await?
    {
        Some(brief)
            if (brief.timestamp == cooked.timestamp
                && brief.author == cooked.author
                && brief.per_recipient_shares.contains_key(&cooked.recipient)) => {}
        _ => {
            let hint = mk_hint();
            let what =
                Box::new(InvalidCertificateError::ShamirRecoveryMissingBriefCertificate { hint });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    // 3) Shamir recovery share should only be provided to its recipient

    if cooked.recipient != ops.device.user_id {
        let hint = mk_hint();
        let what = Box::new(InvalidCertificateError::ShamirRecoveryUnrelatedToUs { hint });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    }

    // 4) Check share doesn't already exists

    // With this query, we get the shamir recovery share that is the closest to
    // `cooked.timestamp`, from there:
    // a) If there is no share... then there is no share related to our brief !
    // b) If the timestamp is not exactly `cooked.timestamp`, then this share corresponds
    //   to a different brief and hence can be ignored.
    //   This also means there is no share related to our brief.
    // c) If the share's author is different from ours, then it also means both shares
    //   are unrelated.
    //
    // Also note c) implies b) since we have a single shamir topic in which only
    // related brief and share certificates are allowed to have the same timestamp.
    match store
        .get_last_shamir_recovery_share_certificate_for_recipient(
            UpTo::Timestamp(cooked.timestamp),
            author_user_id.to_owned(),
            cooked.recipient,
        )
        .await?
    {
        Some(other_share)
            if (other_share.author == cooked.author
                && other_share.timestamp == cooked.timestamp) =>
        {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::ContentAlreadyExists { hint });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
        _ => (),
    }

    // 5) Check `user_id` field matches with author

    if cooked.user_id != author_user_id {
        let hint = mk_hint();
        let what = Box::new(InvalidCertificateError::ShamirRecoveryNotAboutSelf {
            hint,
            user_id: cooked.user_id,
            author_user_id,
        });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    }

    // 6) Check author is not revoked
    // This should have already been checked during the related brief certificate's
    // validation, but better safe than sorry !

    check_author_not_revoked_and_profile(
        ops,
        store,
        cooked.timestamp,
        author_user_id,
        false,
        mk_hint,
    )
    .await?;

    Ok(())
}

async fn check_shamir_recovery_deletion_certificate_consistency(
    ops: &CertificateOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    cooked: &ShamirRecoveryDeletionCertificate,
    author_user_id: UserID,
) -> Result<(), CertifAddCertificatesBatchError> {
    let mk_hint = || format!("{:?}", cooked);

    // 1) Certificate must be the newest among the ones in shamir recovery topic.

    let last_stored_timestamp = store.get_last_timestamps().await?.shamir_recovery;
    if let Some(last_stored_timestamp) = last_stored_timestamp {
        if cooked.timestamp <= last_stored_timestamp {
            // We already know more recent certificates, hence this certificate
            // cannot be added without breaking causality !
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::InvalidTimestamp {
                hint,
                last_certificate_timestamp: last_stored_timestamp,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    // Given we now know there is no certificate with a timestamp >= `cooked.timestamp`,
    // we don't need to have a dedicated check than ensure `cooked.setup_timestamp` < `cooked.timestamp`.
    // Indeed, in such case there is no certificate corresponding to `cooked.setup_timestamp`
    // and so we fall into the generic "invalid setup_timestamp" case.

    // 2) Check `user_id` field matches with author

    if cooked.setup_to_delete_user_id != author_user_id {
        let hint = mk_hint();
        let what = Box::new(InvalidCertificateError::ShamirRecoveryNotAboutSelf {
            hint,
            user_id: cooked.setup_to_delete_user_id,
            author_user_id,
        });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    }

    // 3) Make sure this certificate refers to the last valid shamir recovery
    // Note there could be at most one non-removed shamir recovery, hence it is
    // guaranteed that all but the last one has already been removed.

    let last_shamir_recovery = store
        .get_last_shamir_recovery_for_author(UpTo::Timestamp(cooked.timestamp), author_user_id)
        .await?;
    let setup_recipients = match last_shamir_recovery {
        LastShamirRecovery::Valid(last_certif)
            if last_certif.timestamp == cooked.setup_to_delete_timestamp =>
        {
            HashSet::from_iter(last_certif.per_recipient_shares.keys().cloned())
        }
        LastShamirRecovery::Deleted(last_certif, removed_certif)
            if last_certif.timestamp == cooked.setup_to_delete_timestamp =>
        {
            let hint = mk_hint();
            let what = Box::new(InvalidCertificateError::ShamirRecoveryAlreadyDeleted {
                hint,
                deleted_on: removed_certif.timestamp,
            });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
        _ => {
            let hint = mk_hint();
            let what = Box::new(
                InvalidCertificateError::ShamirRecoveryDeletionMustReferenceLastSetup { hint },
            );
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    };

    // 4) Check recipients match between setup and deletion certificates.

    if cooked.share_recipients != setup_recipients {
        let hint = mk_hint();
        let what = Box::new(
            InvalidCertificateError::ShamirRecoveryDeletionRecipientsMismatch {
                hint,
                setup_recipients,
            },
        );
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    }

    // Since at this point we have made sure the deletion certificate matches
    // with a brief we know about, it means we have already checked we are
    // related to the shamir recovery (as it has been done when the brief
    // certificate was added).
    assert!(
        cooked.setup_to_delete_user_id == ops.device.user_id
            || cooked.share_recipients.contains(&ops.device.user_id)
    );

    // 5) Check author is not revoked

    check_author_not_revoked_and_profile(
        ops,
        store,
        cooked.timestamp,
        author_user_id,
        false,
        mk_hint,
    )
    .await?;

    Ok(())
}

async fn check_sequester_authority_certificate_consistency(
    store: &mut CertificatesStoreWriteGuard<'_>,
    cooked: &SequesterAuthorityCertificate,
) -> Result<(), CertifAddCertificatesBatchError> {
    // Sequester authority certificate must be the very first certificate provided,
    // so we just have to check the storage is currently empty !

    let stored_last_timestamps = store.get_last_timestamps().await?;

    if !stored_last_timestamps.is_empty() {
        let hint = format!("{:?}", cooked);
        let what = Box::new(InvalidCertificateError::SequesterAuthorityMustBeFirst { hint });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    }

    Ok(())
}

async fn check_sequester_service_certificate_consistency(
    store: &mut CertificatesStoreWriteGuard<'_>,
    last_stored_sequester_timestamp: DateTime,
    cooked: &SequesterServiceCertificate,
) -> Result<(), CertifAddCertificatesBatchError> {
    let mk_hint = || format!("{:?}", cooked);

    // 1) Certificate must be the newest among the ones in sequester topic.
    // Note we also reject same timestamp given sequester server certificates
    // is always created alone.

    if cooked.timestamp <= last_stored_sequester_timestamp {
        // We already know more recent certificates, hence this certificate
        // cannot be added without breaking causality !
        let hint = mk_hint();
        let what = Box::new(InvalidCertificateError::InvalidTimestamp {
            hint,
            last_certificate_timestamp: last_stored_sequester_timestamp,
        });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    }

    // 2) Make sure the service doesn't already exist

    let existing_services = store
        .get_sequester_service_certificates(UpTo::Timestamp(cooked.timestamp))
        .await?;
    for existing_service in existing_services {
        if existing_service.service_id == cooked.service_id {
            let hint = format!("{:?}", cooked);
            let what = Box::new(InvalidCertificateError::ContentAlreadyExists { hint });
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    Ok(())
}

async fn check_sequester_revoked_service_certificate_consistency(
    store: &mut CertificatesStoreWriteGuard<'_>,
    last_stored_sequester_timestamp: DateTime,
    cooked: &SequesterRevokedServiceCertificate,
) -> Result<(), CertifAddCertificatesBatchError> {
    let mk_hint = || format!("{:?}", cooked);

    // 1) Certificate must be the newest among the ones in sequester topic.
    // Note we also reject same timestamp given sequester server certificates
    // is always created alone.

    if cooked.timestamp <= last_stored_sequester_timestamp {
        // We already know more recent certificates, hence this certificate
        // cannot be added without breaking causality !
        let hint = mk_hint();
        let what = Box::new(InvalidCertificateError::InvalidTimestamp {
            hint,
            last_certificate_timestamp: last_stored_sequester_timestamp,
        });
        return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
    }

    // 2) Make sure the sequester service is not already revoked

    match store
        .get_sequester_revoked_service_certificate(
            UpTo::Timestamp(cooked.timestamp),
            cooked.service_id.to_owned(),
        )
        .await?
    {
        // Not revoked, as expected :)
        None => (),

        // Sequester service can only be revoked once :(
        Some(revoked_certificate) => {
            let hint = mk_hint();
            let what = Box::new(
                InvalidCertificateError::RelatedSequesterServiceAlreadyRevoked {
                    hint,
                    service_revoked_on: revoked_certificate.timestamp,
                },
            );
            return Err(CertifAddCertificatesBatchError::InvalidCertificate(what));
        }
    }

    Ok(())
}
