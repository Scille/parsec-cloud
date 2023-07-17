// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use libparsec_platform_async::RwLockWriteGuard;
use libparsec_types::prelude::*;

use super::{
    storage::{CertificatesCachedStorage, GetCertificateError, GetTimestampBoundsError, UpTo},
    CertificatesOps,
};
use crate::event_bus::EventInvalidCertificate;

#[derive(Debug, thiserror::Error)]
pub enum InvalidCertificateError {
    #[error("Certificate `{hint}` is corrupted: {error}")]
    Corrupted { hint: String, error: DataError },

    // Consistency errors mean there is nothing wrong with the certificate content,
    // but it is incompatible with the other certificates we already have.
    #[error("Certificate `{hint}` breaks consistency: it declares to be signed by a author that doesn't exist")]
    NonExistingAuthor { hint: String },
    #[error("Certificate `{hint}` breaks consistency: it declares to be older than it author (created on {author_created_on})")]
    OlderThanAuthor {
        hint: String,
        author_created_on: DateTime,
    },
    #[error("Certificate `{hint}` breaks consistency: it declares to be signed by itself, which is not allowed")]
    SelfSigned { hint: String },
    #[error("Certificate `{hint}` breaks consistency: it author has already been revoked on {author_revoked_on}")]
    RevokedAuthor {
        hint: String,
        author_revoked_on: DateTime,
    },
    #[error("Certificate `{hint}` breaks consistency: it author is expected to be Admin but instead has profile {author_profile:?}")]
    AuthorNotAdmin {
        hint: String,
        author_profile: UserProfile,
    },
    #[error("Certificate `{hint}` breaks consistency: there is already a certificate with similar content")]
    ContentAlreadyExists { hint: String },
    #[error("Certificate `{hint}` breaks consistency: there is already a certificate with index #{certificate_index}")]
    IndexAlreadyExists {
        hint: String,
        certificate_index: IndexInt,
    },
    #[error("Certificate `{hint}` breaks consistency: index #{expected_index} was expected but instead got #{certificate_index}")]
    InvalidIndex {
        hint: String,
        certificate_index: IndexInt,
        expected_index: IndexInt,
    },
    #[error("Certificate `{hint}` breaks consistency: it is older than the previous certificate we know about ({last_certificate_timestamp})")]
    InvalidTimestamp {
        hint: String,
        last_certificate_timestamp: DateTime,
    },
    #[error("Certificate `{hint}` breaks consistency: a sequestered service can only be added to a sequestered organization")]
    NotASequesteredOrganization { hint: String },
    #[error("Certificate `{hint}` breaks consistency: it refers to a user than doesn't exist")]
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
    #[error("Certificate `{hint}` breaks consistency: related user cannot change profile to Outsider given it still has Owner/Manager role in some realms")]
    CannotDowngradeUserToOutsider { hint: String },
    #[error("Certificate `{hint}` breaks consistency: as first certificate for the realm, author must give the role to itself")]
    RealmFirstRoleMustBeSelfSigned { hint: String },
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
    #[error("Certificate `{hint}` breaks consistency: user has Outsider profile, and hence cannot have Owner/Manager role in the realm given it is shared with others !")]
    RealmOutsiderCannotBeOwnerOrManager { hint: String },
}

#[derive(Debug, thiserror::Error)]
pub enum AddCertificateError {
    #[error(transparent)]
    InvalidCertificate(#[from] InvalidCertificateError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub enum MaybeRedactedSwitch {
    Switched,
    NoSwitch,
}

pub(super) async fn add_certificates_batch<'a>(
    ops: &'a CertificatesOps,
    storage: &mut RwLockWriteGuard<'a, CertificatesCachedStorage>,
    last_index: IndexInt,
    certificates: impl Iterator<Item = Bytes>,
) -> Result<MaybeRedactedSwitch, AddCertificateError> {
    let initial_self_profile = storage.get_current_self_profile().await?;

    for (serialized, certificate_index) in certificates.zip(last_index + 1..) {
        // Start by validating the certificate and, if something goes wrong, send
        // the invalid certificate event

        let any_cooked = validate_certificate(ops, storage, certificate_index, serialized.clone())
            .await
            // Here we send the event in case the certificate was invalid
            .map_err(|err| {
                if let AddCertificateError::InvalidCertificate(what) = err {
                    let event = EventInvalidCertificate(what);
                    ops.event_bus.send(&event);
                    AddCertificateError::InvalidCertificate(event.0)
                } else {
                    err
                }
            })?;

        // At this point, we are guaranteed the certificate is valid, so we can
        // insert it in database...

        // ...but there is one more weird check before that !
        // If the certificate changes *our profile* from/to Outsider, then our certificate
        // storage must be cleared given we no longer use the right flavour: Outsider
        // must use redacted certificates while other profile use non-redacted ones.
        if let AnyArcCertificate::UserUpdate(update) = &any_cooked {
            if update.user_id == *ops.device.user_id() {
                match (last_index, initial_self_profile, update.new_profile) {
                    // The storage was empty before this batch, so there is nothing outdated
                    // Note at this point `last_index` has been indirectly checked by the
                    // first call to `validated_certificate` so it value can be trusted.
                    (0, _, _) => (),
                    // No need to switch.
                    (_, UserProfile::Outsider, UserProfile::Outsider) => (),
                    // Switching from/to Outsider !
                    (_, UserProfile::Outsider, _) | (_, _, UserProfile::Outsider) => {
                        // So we clear the storage and don't try to go any further given
                        // the index is no longer the right one (we must instead re-poll
                        // the server to get certificates from index 0)
                        storage.forget_all_certificates().await?;
                        return Ok(MaybeRedactedSwitch::Switched);
                    }
                    // Switching profile without Outsider involved
                    _ => (),
                }
            }
        }

        // At last we can insert the certificate

        match any_cooked {
            AnyArcCertificate::User(cooked) => {
                storage
                    .add_user_certificate(certificate_index, cooked, serialized)
                    .await?;
            }
            AnyArcCertificate::Device(cooked) => {
                storage
                    .add_device_certificate(certificate_index, cooked, serialized)
                    .await?;
            }
            AnyArcCertificate::UserUpdate(cooked) => {
                storage
                    .add_user_update_certificate(certificate_index, cooked, serialized)
                    .await?;
            }
            AnyArcCertificate::RevokedUser(cooked) => {
                storage
                    .add_revoked_user_certificate(certificate_index, cooked, serialized)
                    .await?;
            }
            AnyArcCertificate::RealmRole(cooked) => {
                storage
                    .add_realm_role_certificate(certificate_index, cooked, serialized)
                    .await?;
            }
            AnyArcCertificate::SequesterAuthority(cooked) => {
                storage
                    .add_sequester_authority_certificate(certificate_index, cooked, serialized)
                    .await?;
            }
            AnyArcCertificate::SequesterService(cooked) => {
                storage
                    .add_sequester_service_certificate(certificate_index, cooked, serialized)
                    .await?;
            }
        }
    }

    Ok(MaybeRedactedSwitch::NoSwitch)
}

/// Validate the given certificate by both checking it content (format, signature, etc.)
/// and it global consistency with the others certificates.
async fn validate_certificate<'a>(
    ops: &'a CertificatesOps,
    storage: &mut RwLockWriteGuard<'a, CertificatesCachedStorage>,
    index: IndexInt,
    certificate: Bytes,
) -> Result<AnyArcCertificate, AddCertificateError> {
    // 1) Deserialize the certificate first, as this doesn't need a lock on the storage

    let unsecure = match AnyCertificate::unsecure_load(certificate) {
        Ok(unsecure) => unsecure,
        Err(error) => {
            // No information can be extracted from the binary data...
            let hint = "<unknown>".into();
            let what = InvalidCertificateError::Corrupted { hint, error };
            return Err(AddCertificateError::InvalidCertificate(what));
        }
    };

    // 2) Run consistency checks on index & timestamp:
    // - Certificates are added in deterministic order following their index,
    //   hence only certificate with index <last known index + 1> can be added.
    // - A certificate must have a timestamp greater than the previous one (can
    //   be equal e.g. when creating a new user or during organization bootstrap).

    // Special case for the first index given there is no previous one to retrieve !
    let current_index = if index == 1 {
        // Ensure index 1 doesn't already exists
        match storage.get_timestamp_bounds(1).await {
            Err(GetTimestampBoundsError::NonExisting) => {
                // As expected !
                0
            }
            Ok(_) => {
                // We already know index 1 !
                let hint = unsecure.hint();
                let what = InvalidCertificateError::IndexAlreadyExists {
                    hint,
                    certificate_index: index,
                };
                return Err(AddCertificateError::InvalidCertificate(what));
            }
            Err(err @ GetTimestampBoundsError::Internal(_)) => {
                return Err(anyhow::anyhow!(err).into())
            }
        }
    } else {
        let guessed_current_index = index - 1;
        match storage.get_timestamp_bounds(guessed_current_index).await {
            Ok((_, Some(_))) => {
                // We already know some certificates with an higher index, hence this certificate
                // cannot be added without breaking causality !
                let hint = unsecure.hint();
                let what = InvalidCertificateError::IndexAlreadyExists {
                    hint,
                    certificate_index: index,
                };
                return Err(AddCertificateError::InvalidCertificate(what));
            }
            Err(GetTimestampBoundsError::NonExisting) => {
                // We have a hole between the our last index and the one of the certificate to add.
                let hint = unsecure.hint();
                let expected_index = storage
                    .get_last_certificate_index()
                    .await?
                    .unwrap_or_default()
                    + 1;
                let what = InvalidCertificateError::InvalidIndex {
                    hint,
                    certificate_index: index,
                    expected_index,
                };
                return Err(AddCertificateError::InvalidCertificate(what));
            }
            Ok((lower, None)) => {
                // The index is the one we expected, now ensure it timestamp is compatible
                // with the previous certificate. Note we allow both certificates to have
                // the same timestamp, this is the case when creating a new user given
                // the device certificate must also be created at the same time.
                if *unsecure.timestamp() < lower {
                    let hint = unsecure.hint();
                    let what = InvalidCertificateError::InvalidTimestamp {
                        hint,
                        last_certificate_timestamp: lower,
                    };
                    return Err(AddCertificateError::InvalidCertificate(what));
                } else {
                    guessed_current_index
                }
            }
            Err(err @ GetTimestampBoundsError::Internal(_)) => {
                return Err(anyhow::anyhow!(err).into())
            }
        }
    };

    // 3) Verify the certificate signature
    // By doing so we also verify author's device existence, but we don't go
    // any further, hence additional checks on author must be done at step 4 !

    macro_rules! verify_certificate_signature {

        // Entry points

        (Device, $unsecure:ident) => {
            verify_certificate_signature!(
                @internal,
                Device,
                $unsecure,
                $unsecure.author()
            )
            .map(|(certif, serialized)| (Arc::new(certif), serialized))
            .map_err(|what| {
                AddCertificateError::InvalidCertificate(what)
            })
        };

        (DeviceOrRoot, $unsecure:ident) => {
            verify_certificate_signature!(
                @internal,
                DeviceOrRoot,
                $unsecure
            )
            .map(|(certif, serialized)| (Arc::new(certif), serialized))
            .map_err(|what| {
                AddCertificateError::InvalidCertificate(what)
            })
        };

        // Internal macro implementation stuff

        (@internal, Device, $unsecure:ident, $author:expr) => {
            match storage.get_device_certificate(UpTo::Index(index), $author).await {
                Ok(author_certif) => $unsecure
                    .verify_signature(&author_certif.verify_key)
                    .map_err(|(unsecure, error)| {
                        let hint = unsecure.hint();
                        InvalidCertificateError::Corrupted { hint, error }
                    }),
                Err(GetCertificateError::ExistButTooRecent {
                    certificate_timestamp,
                    ..
                }) => {
                    // The author didn't exist at the time the certificate was made...
                    let hint = $unsecure.hint();
                    let what = InvalidCertificateError::OlderThanAuthor {
                        hint,
                        author_created_on: certificate_timestamp,
                    };
                    Err(what)
                }
                Err(GetCertificateError::NonExisting) => {
                    // Unknown author... we don't try here to poll the server
                    // for new certificates: this is because certificate are
                    // supposed to be added in a strictly causal order, hence
                    // we are supposed to have already added all the certificates
                    // needed to validate this one. And if that's not the case
                    // it's suspicious and error should be raised !
                    let what = InvalidCertificateError::NonExistingAuthor {
                        hint: $unsecure.hint(),
                    };
                    Err(what)
                }
                Err(err @ GetCertificateError::Internal(_)) => {
                    return Err(anyhow::anyhow!(err).into());
                }
            }
        };

        (@internal, DeviceOrRoot, $unsecure:ident) => {
            match $unsecure.author() {
                CertificateSignerOwned::Root => $unsecure
                    .verify_signature(ops.device.root_verify_key())
                    .map_err(|(unsecure, error)| {
                        let hint = unsecure.hint();
                        InvalidCertificateError::Corrupted { hint, error }
                    }),

                CertificateSignerOwned::User(author) => {
                    verify_certificate_signature!(
                        @internal,
                        Device,
                        $unsecure,
                        author
                    )
                }
            }
        };

    }

    match unsecure {
        UnsecureAnyCertificate::User(unsecure) => {
            let (cooked, _) = verify_certificate_signature!(DeviceOrRoot, unsecure)?;

            // 4) The certificate is valid, last check is the consistency with other certificates
            check_user_certificate_consistency(ops, storage, current_index, &cooked).await?;

            Ok(AnyArcCertificate::User(cooked))
        }
        UnsecureAnyCertificate::Device(unsecure) => {
            let (cooked, _) = verify_certificate_signature!(DeviceOrRoot, unsecure)?;

            // 4) The certificate is valid, last check is the consistency with other certificates
            check_device_certificate_consistency(ops, storage, current_index, &cooked).await?;

            Ok(AnyArcCertificate::Device(cooked))
        }
        UnsecureAnyCertificate::RevokedUser(unsecure) => {
            let (cooked, _) = verify_certificate_signature!(Device, unsecure)?;

            // 4) The certificate is valid, last check is the consistency with other certificates
            check_revoked_user_certificate_consistency(ops, storage, current_index, &cooked)
                .await?;

            Ok(AnyArcCertificate::RevokedUser(cooked))
        }
        UnsecureAnyCertificate::UserUpdate(unsecure) => {
            let (cooked, _) = verify_certificate_signature!(Device, unsecure)?;

            // 4) The certificate is valid, last check is the consistency with other certificates
            check_user_update_certificate_consistency(ops, storage, current_index, &cooked).await?;

            Ok(AnyArcCertificate::UserUpdate(cooked))
        }
        UnsecureAnyCertificate::RealmRole(unsecure) => {
            let (cooked, _) = verify_certificate_signature!(DeviceOrRoot, unsecure)?;

            // 4) The certificate is valid, last check is the consistency with other certificates
            check_realm_role_certificate_consistency(storage, current_index, &cooked).await?;

            Ok(AnyArcCertificate::RealmRole(cooked))
        }

        UnsecureAnyCertificate::SequesterAuthority(unsecure) => {
            // Sequester authority can only be signed by root
            let (cooked, _) = unsecure
                .verify_signature(ops.device.root_verify_key())
                .map(|(certif, serialized)| (Arc::new(certif), serialized))
                .map_err(|(unsecure, error)| {
                    let hint = unsecure.hint();
                    let what = InvalidCertificateError::Corrupted { hint, error };
                    AddCertificateError::InvalidCertificate(what)
                })?;

            // 4) The certificate is valid, last check is the consistency with other certificates
            check_sequester_authority_certificate_consistency(ops, storage, current_index, &cooked)
                .await?;

            Ok(AnyArcCertificate::SequesterAuthority(cooked))
        }

        UnsecureAnyCertificate::SequesterService(unsecure) => {
            // Sequester services can only be signed by authority
            let (cooked, _) = match storage
                .get_sequester_authority_certificate(UpTo::Index(current_index))
                .await
            {
                Ok(authority) => unsecure
                    .verify_signature(&authority.verify_key_der)
                    .map(|(certif, serialized)| (Arc::new(certif), serialized))
                    .map_err(|(unsecure, error)| {
                        let hint = unsecure.hint();
                        let what = InvalidCertificateError::Corrupted { hint, error };
                        AddCertificateError::InvalidCertificate(what)
                    }),
                Err(GetCertificateError::NonExisting) => {
                    // Our organization is not a sequestered one :(
                    let hint = unsecure.hint();
                    let what = InvalidCertificateError::NotASequesteredOrganization { hint };
                    Err(AddCertificateError::InvalidCertificate(what))
                }
                // `ExistButTooRecent` case should never occur given the authority
                // certificate is added during organization bootstrap along with the
                // initial user and device certificates. Hence, if we are currently
                // checking a sequester service certificate, the index must be higher !
                Err(GetCertificateError::ExistButTooRecent {
                    certificate_index, ..
                }) => {
                    let hint = unsecure.hint();
                    // TODO: improve error logging (struct log, org/device already captured by default, sentry compat etc.)
                    let org = ops.device.organization_id();
                    let device_id = &ops.device.device_id;
                    log::error!(
                        "{org}#{device_id}: `{hint}`: the sequester service we tried to add is said to be older than sequester authority (#{index} vs #{certificate_index})"
                    );
                    return Err(AddCertificateError::Internal(anyhow::anyhow!("`{hint}`: the sequester service we tried to add is said to be older than sequester authority (#{index} vs #{certificate_index})")));
                }
                Err(err @ GetCertificateError::Internal(_)) => Err(anyhow::anyhow!(err).into()),
            }?;

            // 4) The certificate is valid, last check is the consistency with other certificates
            check_sequester_service_certificate_consistency(storage, current_index, &cooked)
                .await?;

            Ok(AnyArcCertificate::SequesterService(cooked))
        }
    }
}

async fn check_user_exists<'a>(
    storage: &mut RwLockWriteGuard<'a, CertificatesCachedStorage>,
    up_to_index: IndexInt,
    user_id: &UserID,
    mk_hint: impl FnOnce() -> String,
) -> Result<Arc<UserCertificate>, AddCertificateError> {
    match storage
        .get_user_certificate(UpTo::Index(up_to_index), user_id)
        .await
    {
        // User exists as we expected :)
        Ok(certif) => Ok(certif),

        // User doesn't exist :(
        Err(GetCertificateError::NonExisting) => {
            let hint = mk_hint();
            let what = InvalidCertificateError::NonExistingRelatedUser { hint };
            Err(AddCertificateError::InvalidCertificate(what))
        }

        // User doesn't exist... yet :(
        Err(GetCertificateError::ExistButTooRecent {
            certificate_timestamp,
            ..
        }) => {
            let hint = mk_hint();
            let what = InvalidCertificateError::OlderThanRelatedUser {
                hint,
                user_created_on: certificate_timestamp,
            };
            Err(AddCertificateError::InvalidCertificate(what))
        }

        // D'oh :/
        Err(err @ GetCertificateError::Internal(_)) => {
            Err(AddCertificateError::Internal(err.into()))
        }
    }
}

async fn check_user_not_revoked<'a>(
    storage: &mut RwLockWriteGuard<'a, CertificatesCachedStorage>,
    up_to_index: IndexInt,
    user_id: &UserID,
    mk_hint: impl FnOnce() -> String,
) -> Result<(), AddCertificateError> {
    match storage
        .get_revoked_user_certificate(UpTo::Index(up_to_index), user_id)
        .await?
    {
        // Not revoked, as expected :)
        None => (),

        // User can only be revoked once :(
        Some(revoked_certificate) => {
            let hint = mk_hint();
            let what = InvalidCertificateError::RelatedUserAlreadyRevoked {
                hint,
                user_revoked_on: revoked_certificate.timestamp,
            };
            return Err(AddCertificateError::InvalidCertificate(what));
        }
    }

    Ok(())
}

/// Admin existence has already been checked while fetching it device's verify key,
/// so what's left is checking it is not revoked and (optionally) it profile
async fn check_author_not_revoked_and_profile<'a>(
    ops: &'a CertificatesOps,
    storage: &mut RwLockWriteGuard<'a, CertificatesCachedStorage>,
    up_to_index: IndexInt,
    author: &UserID,
    author_must_be_admin: bool,
    mk_hint: impl FnOnce() -> String + std::marker::Copy,
) -> Result<(), AddCertificateError> {
    match storage
        .get_revoked_user_certificate(UpTo::Index(up_to_index), author)
        .await?
    {
        // Not revoked, as expected :)
        None => (),

        // A revoked user cannot author anything !
        Some(revoked_certificate) => {
            let hint = mk_hint();
            let what = InvalidCertificateError::RevokedAuthor {
                hint,
                author_revoked_on: revoked_certificate.timestamp,
            };
            return Err(AddCertificateError::InvalidCertificate(what));
        }
    }

    if author_must_be_admin {
        match get_user_profile(storage, up_to_index, author, mk_hint, None).await {
            Ok(profile) => {
                if profile != UserProfile::Admin {
                    let hint = mk_hint();
                    let what = InvalidCertificateError::AuthorNotAdmin {
                        hint,
                        author_profile: profile,
                    };
                    return Err(AddCertificateError::InvalidCertificate(what));
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
            Err(AddCertificateError::InvalidCertificate(err)) => {
                // TODO: improve error logging (struct log, org/device already captured by default, sentry compat etc.)
                let org = ops.device.organization_id();
                let device_id = &ops.device.device_id;
                log::error!("{org}#{device_id}: Got author device, but fail to get corresponding user: {err:?}");
                return Err(AddCertificateError::Internal(anyhow::anyhow!(
                    "Got author device, but fail to get corresponding user: {err:?}"
                )));
            }
            // D'oh :/
            Err(err) => {
                return Err(AddCertificateError::Internal(err.into()));
            }
        }
    }

    Ok(())
}

async fn get_user_profile<'a>(
    storage: &mut RwLockWriteGuard<'a, CertificatesCachedStorage>,
    up_to_index: IndexInt,
    user_id: &UserID,
    mk_hint: impl FnOnce() -> String,
    already_fetched_user_certificate: Option<Arc<UserCertificate>>,
) -> Result<UserProfile, AddCertificateError> {
    // Updates are ordered by index, so last one contains the current profile
    let updates = storage
        .get_user_update_certificates(UpTo::Index(up_to_index), user_id)
        .await?;
    if let Some(last_update) = updates.last() {
        return Ok(last_update.new_profile);
    }

    // No updates, the current profile is the one specified in the user certificate
    let user_certificate = match already_fetched_user_certificate {
        Some(user_certificate) => user_certificate,
        None => check_user_exists(storage, up_to_index, user_id, mk_hint).await?,
    };

    Ok(user_certificate.profile)
}

async fn check_user_certificate_consistency<'a>(
    ops: &'a CertificatesOps,
    storage: &mut RwLockWriteGuard<'a, CertificatesCachedStorage>,
    current_index: IndexInt,
    cooked: &UserCertificate,
) -> Result<(), AddCertificateError> {
    let mk_hint = || format!("{:?}", cooked);

    // 1) Check author is not revoked and has ADMIN profile

    if let CertificateSignerOwned::User(author) = &cooked.author {
        check_author_not_revoked_and_profile(
            ops,
            storage,
            current_index,
            author.user_id(),
            true,
            mk_hint,
        )
        .await?;
    }
    // TODO: should we go further in the checking ?
    // - only the very first user (accross all users) should be allowed to be signed by root
    // (This check is not enforced in Parsec <= 2.15, and might be overkill ?)

    // 2) Make sure the user doesn't already exists

    match storage
        .get_user_certificate(UpTo::Index(current_index), &cooked.user_id)
        .await
    {
        // This user doesn't already exist, this is what we expected :)
        Err(GetCertificateError::NonExisting) => (),

        // This user already exists :(
        Ok(_) => {
            let hint = mk_hint();
            let what = InvalidCertificateError::ContentAlreadyExists { hint };
            return Err(AddCertificateError::InvalidCertificate(what));
        }

        // This case should never happen given we have already checked
        // `index` is higher than the current stored index.
        Err(GetCertificateError::ExistButTooRecent {
            certificate_index, ..
        }) => {
            // TODO: improve error logging (struct log, org/device already captured by default, sentry compat etc.)
            let org = ops.device.organization_id();
            let device_id = &ops.device.device_id;
            let hint = mk_hint();
            let index = current_index + 1;
            log::error!("{org}#{device_id}: `{hint}`: already checked index, but now storage says it is too recent ({index} vs {certificate_index} in storage)");
            return Err(AddCertificateError::Internal(anyhow::anyhow!("`{hint}`: already checked index, but now storage says it is too recent ({index} vs {certificate_index} in storage)")));
        }

        // D'oh :/
        Err(err @ GetCertificateError::Internal(_)) => {
            return Err(AddCertificateError::Internal(err.into()));
        }
    }

    // If the user doesn't already exist, we are guaranteed we also don't have other
    // types of certificate related to this user id in the local database
    // So those conditions are already guaranteed:
    // - the user has no related device
    // - the user is not revoked
    // - the user certificate is not signed by one of the user's devices

    Ok(())
}

async fn check_revoked_user_certificate_consistency<'a>(
    ops: &'a CertificatesOps,
    storage: &mut RwLockWriteGuard<'a, CertificatesCachedStorage>,
    current_index: IndexInt,
    cooked: &RevokedUserCertificate,
) -> Result<(), AddCertificateError> {
    let mk_hint = || format!("{:?}", cooked);

    // 1) Check the certificate is not self-signed

    if cooked.author.user_id() == &cooked.user_id {
        let hint = mk_hint();
        let what = InvalidCertificateError::SelfSigned { hint };
        return Err(AddCertificateError::InvalidCertificate(what));
    }

    // 2) Check author is not revoked and has ADMIN profile

    check_author_not_revoked_and_profile(
        ops,
        storage,
        current_index,
        cooked.author.user_id(),
        true,
        mk_hint,
    )
    .await?;

    // 3) Make sure the user exists

    check_user_exists(storage, current_index, &cooked.user_id, mk_hint).await?;

    // 4) Make sure the user is not already revoked

    check_user_not_revoked(storage, current_index, &cooked.user_id, mk_hint).await?;

    Ok(())
}

async fn check_user_update_certificate_consistency<'a>(
    ops: &'a CertificatesOps,
    storage: &mut RwLockWriteGuard<'a, CertificatesCachedStorage>,
    current_index: IndexInt,
    cooked: &UserUpdateCertificate,
) -> Result<(), AddCertificateError> {
    let mk_hint = || format!("{:?}", cooked);

    // 1) Check the certificate is not self-signed

    if cooked.author.user_id() == &cooked.user_id {
        let hint = mk_hint();
        let what = InvalidCertificateError::SelfSigned { hint };
        return Err(AddCertificateError::InvalidCertificate(what));
    }

    // 2) Check author is not revoked and has ADMIN profile

    check_author_not_revoked_and_profile(
        ops,
        storage,
        current_index,
        cooked.author.user_id(),
        true,
        mk_hint,
    )
    .await?;

    // 3) Make sure the user exists

    let user_certificate =
        check_user_exists(storage, current_index, &cooked.user_id, mk_hint).await?;

    // 4) Make sure the user is not already revoked

    check_user_not_revoked(storage, current_index, &cooked.user_id, mk_hint).await?;

    // 5) Make sure the user doesn't already have this profile

    let user_current_profile = get_user_profile(
        storage,
        current_index,
        &cooked.user_id,
        mk_hint,
        Some(user_certificate),
    )
    .await?;
    if user_current_profile == cooked.new_profile {
        let hint = mk_hint();
        let what = InvalidCertificateError::ContentAlreadyExists { hint };
        return Err(AddCertificateError::InvalidCertificate(what));
    }

    // 6) If user is downgraded to Outsider, it should not be Owner/Manager of any shared realm
    if cooked.new_profile == UserProfile::Outsider {
        for certif in storage
            .get_user_realms_roles(UpTo::Index(current_index), cooked.user_id.clone())
            .await?
        {
            match certif.role {
                // Outsider can be owner only if the workspace is not shared
                Some(RealmRole::Owner) => {
                    let roles = storage
                        .get_realm_certificates(UpTo::Index(current_index), certif.realm_id)
                        .await?;
                    // If the workspace is not shared, there should be only a single certificate
                    // (given one cannot change it own role !)
                    if roles.len() != 1 {
                        let hint = mk_hint();
                        let what = InvalidCertificateError::CannotDowngradeUserToOutsider { hint };
                        return Err(AddCertificateError::InvalidCertificate(what));
                    }
                }
                // Outsider can never be manager
                Some(RealmRole::Manager) => {
                    let hint = mk_hint();
                    let what = InvalidCertificateError::CannotDowngradeUserToOutsider { hint };
                    return Err(AddCertificateError::InvalidCertificate(what));
                }
                None | Some(RealmRole::Contributor) | Some(RealmRole::Reader) => (),
            }
        }
    }

    Ok(())
}

async fn check_device_certificate_consistency<'a>(
    ops: &'a CertificatesOps,
    storage: &mut RwLockWriteGuard<'a, CertificatesCachedStorage>,
    current_index: IndexInt,
    cooked: &DeviceCertificate,
) -> Result<(), AddCertificateError> {
    let mk_hint = || format!("{:?}", cooked);
    let user_id = cooked.device_id.user_id();

    // 1) Device certificate can be self-signed
    match &cooked.author {
        // Self-signed: the user has created a new device for himself (hence doesn't need to be ADMIN)
        CertificateSignerOwned::User(author) if author.user_id() == user_id => {
            check_author_not_revoked_and_profile(
                ops,
                storage,
                current_index,
                author.user_id(),
                false,
                mk_hint,
            )
            .await?;
        }

        // Not self-signed: The author is an ADMIN that have enrolled the user
        CertificateSignerOwned::User(author) => {
            check_author_not_revoked_and_profile(
                ops,
                storage,
                current_index,
                author.user_id(),
                true,
                mk_hint,
            )
            .await?;
        }

        // Not self-signed: First device of the user that have bootstrapped the organization
        CertificateSignerOwned::Root => {}
    }
    // TODO: should we go further in the checking ?
    // - only the first device of a given user should be allowed to not be self-signed
    // - only the very first device (accross all users) should be allowed to be signed by root
    // (Those checks are not enforced in Parsec <= 2.15, and might be overkill ?)

    // 2) Make sure the user exists

    check_user_exists(storage, current_index, user_id, mk_hint).await?;

    // 3) Make sure the user is not already revoked

    check_user_not_revoked(storage, current_index, user_id, mk_hint).await?;

    // 4) Make sure this device doesn't already exist

    match storage
        .get_device_certificate(UpTo::Index(current_index), &cooked.device_id)
        .await
    {
        // This device doesn't already exist, this is what we expected :)
        Err(GetCertificateError::NonExisting) => (),

        // This device already exists :(
        Ok(_) => {
            let hint = mk_hint();
            let what = InvalidCertificateError::ContentAlreadyExists { hint };
            return Err(AddCertificateError::InvalidCertificate(what));
        }

        // This case should never happen given we have already checked
        // `index` is higher than the current stored index.
        Err(GetCertificateError::ExistButTooRecent {
            certificate_index, ..
        }) => {
            // TODO: improve error logging (struct log, org/device already captured by default, sentry compat etc.)
            let org = ops.device.organization_id();
            let device_id = &ops.device.device_id;
            let hint = mk_hint();
            let index = current_index + 1;
            log::error!("{org}#{device_id}: `{hint}`: already checked index, but now storage says it is too recent ({index} vs {certificate_index} in storage)");
            return Err(AddCertificateError::Internal(anyhow::anyhow!("`{hint}`: already checked index, but now storage says it is too recent ({index} vs {certificate_index} in storage)")));
        }

        // D'oh :/
        Err(err @ GetCertificateError::Internal(_)) => {
            return Err(AddCertificateError::Internal(err.into()));
        }
    }

    Ok(())
}

async fn check_realm_role_certificate_consistency<'a>(
    storage: &mut RwLockWriteGuard<'a, CertificatesCachedStorage>,
    current_index: IndexInt,
    cooked: &RealmRoleCertificate,
) -> Result<(), AddCertificateError> {
    let mk_hint = || format!("{:?}", cooked);

    let author = match &cooked.author {
        CertificateSignerOwned::User(author) => author,
        // TODO: Currently realm role certificate allow root as author, but there is
        // no reason for that, this is most likely a mistake and should be removed !
        // So for the moment we handle this with a hacky workaround by pretending
        // the serialization format doesn't allow root as author.
        CertificateSignerOwned::Root => {
            let hint = mk_hint();
            let what = InvalidCertificateError::Corrupted {
                hint,
                error: DataError::Serialization,
            };
            return Err(AddCertificateError::InvalidCertificate(what));
        }
    };

    let realm_current_roles = storage
        .get_realm_certificates(UpTo::Index(current_index), cooked.realm_id)
        .await?;

    // 1) Check author's realm role and if certificate is self-signed

    match realm_current_roles.is_empty() {
        // 1.a) The realm is a new one, so certificate must be self-signed with a OWNER role
        true => {
            if author.user_id() != &cooked.user_id {
                let hint = mk_hint();
                let what = InvalidCertificateError::RealmFirstRoleMustBeSelfSigned { hint };
                return Err(AddCertificateError::InvalidCertificate(what));
            }

            if cooked.role != Some(RealmRole::Owner) {
                let hint = mk_hint();
                let what = InvalidCertificateError::RealmFirstRoleMustBeOwner { hint };
                return Err(AddCertificateError::InvalidCertificate(what));
            }
        }

        // 1.b) The realm already exists, so certificate cannot be self-signed and
        // author must have a sufficient role
        false => {
            let author_current_role = match realm_current_roles
                .iter()
                .find(|role| &role.user_id == author.user_id())
                .and_then(|role| role.role)
            {
                // As expected, author currently has a role :)
                Some(author_current_role) => author_current_role,
                // Author never had role, or it role got revoked :(
                None => {
                    let hint = mk_hint();
                    let what = InvalidCertificateError::RealmAuthorHasNoRole { hint };
                    return Err(AddCertificateError::InvalidCertificate(what));
                }
            };

            let user_current_role = realm_current_roles
                .iter()
                .find(|role| role.user_id == cooked.user_id)
                .and_then(|role| role.role);
            let user_new_role = cooked.role;
            match (user_current_role, user_new_role) {
                // Cannot remove the role if the user already doesn't have one !
                (None, None) => {
                    let hint = mk_hint();
                    let what = InvalidCertificateError::ContentAlreadyExists { hint };
                    return Err(AddCertificateError::InvalidCertificate(what));
                }
                // The certificate must provide a new role !
                (Some(current_role), Some(new_role)) if current_role == new_role => {
                    let hint = mk_hint();
                    let what = InvalidCertificateError::ContentAlreadyExists { hint };
                    return Err(AddCertificateError::InvalidCertificate(what));
                }
                // Only an Owner can give/remove Owner/Manager roles
                (Some(RealmRole::Owner), _)
                | (Some(RealmRole::Manager), _)
                | (_, Some(RealmRole::Owner))
                | (_, Some(RealmRole::Manager)) => {
                    if author_current_role != RealmRole::Owner {
                        let hint = mk_hint();
                        let what = InvalidCertificateError::RealmAuthorNotOwner {
                            hint,
                            author_role: author_current_role,
                        };
                        return Err(AddCertificateError::InvalidCertificate(what));
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
                        let what = InvalidCertificateError::RealmAuthorNotOwnerOrManager {
                            hint,
                            author_role: author_current_role,
                        };
                        return Err(AddCertificateError::InvalidCertificate(what));
                    }
                }
            }
        }
    }

    // 2) Make sure the user exists

    let user_certificate =
        check_user_exists(storage, current_index, &cooked.user_id, mk_hint).await?;

    // 3) Make sure the user is not already revoked

    check_user_not_revoked(storage, current_index, &cooked.user_id, mk_hint).await?;

    // 3) An Outsider user can  Make sure the user's profile is compatible with the realm and it given role

    let profile = get_user_profile(
        storage,
        current_index,
        &cooked.user_id,
        mk_hint,
        Some(user_certificate),
    )
    .await?;
    match profile {
        UserProfile::Standard | UserProfile::Admin => (),
        // OUTSIDER user:
        // - can be READER/COLLABORATOR
        // - cannot be MANAGER
        // - can only be OWNER of not-shared workspaces
        UserProfile::Outsider => {
            // Given self-signing is only allowed for the first realm role certificate,
            // a workspace with an outsider owner necessarily contains only this first
            // realm role certificate. Hence the only valid situation is if we are
            // currently adding this initial realm role certificate.
            if !realm_current_roles.is_empty() {
                let hint = mk_hint();
                let what = InvalidCertificateError::RealmOutsiderCannotBeOwnerOrManager { hint };
                return Err(AddCertificateError::InvalidCertificate(what));
            }
        }
    }

    Ok(())
}

async fn check_sequester_authority_certificate_consistency<'a>(
    ops: &CertificatesOps,
    storage: &mut RwLockWriteGuard<'a, CertificatesCachedStorage>,
    current_index: IndexInt,
    cooked: &SequesterAuthorityCertificate,
) -> Result<(), AddCertificateError> {
    // 1) Make sure the authority doesn't already exist

    match storage
        .get_sequester_authority_certificate(UpTo::Index(current_index))
        .await
    {
        // This device doesn't already exist, this is what we expected :)
        Err(GetCertificateError::NonExisting) => (),

        // This device already exists :(
        Ok(_) => {
            let hint = format!("{:?}", cooked);
            let what = InvalidCertificateError::ContentAlreadyExists { hint };
            return Err(AddCertificateError::InvalidCertificate(what));
        }

        // This case should never happen given we have already checked
        // `index` is higher than the current stored index.
        Err(GetCertificateError::ExistButTooRecent {
            certificate_index, ..
        }) => {
            // TODO: improve error logging (struct log, org/device already captured by default, sentry compat etc.)
            let org = ops.device.organization_id();
            let device_id = &ops.device.device_id;
            let hint = format!("{:?}", cooked);
            let index = current_index + 1;
            log::error!("{org}#{device_id}: `{hint}`: already checked index, but now storage says it is too recent ({index} vs {certificate_index} in storage)");
            return Err(AddCertificateError::Internal(anyhow::anyhow!("`{hint}`: already checked index, but now storage says it is too recent ({index} vs {certificate_index} in storage)")));
        }

        // D'oh :/
        Err(err @ GetCertificateError::Internal(_)) => {
            return Err(AddCertificateError::Internal(err.into()));
        }
    }

    // TODO: should we go further in the checking ?
    // - If it exists, the authority certificate should be the very first certificate index
    // (This check is not enforced in Parsec <= 2.15, and might be overkill ?)

    Ok(())
}

async fn check_sequester_service_certificate_consistency<'a>(
    storage: &mut RwLockWriteGuard<'a, CertificatesCachedStorage>,
    current_index: IndexInt,
    cooked: &SequesterServiceCertificate,
) -> Result<(), AddCertificateError> {
    // No need to check the existence of sequester authority given it has already
    // been made while verifying sequester service certificate signature

    // 1) Make sure the service doesn't already exist

    let existing_services = storage
        .get_sequester_service_certificates(UpTo::Index(current_index))
        .await?;
    for existing_service in existing_services {
        if existing_service.service_id == cooked.service_id {
            let hint = format!("{:?}", cooked);
            let what = InvalidCertificateError::ContentAlreadyExists { hint };
            return Err(AddCertificateError::InvalidCertificate(what));
        }
    }

    Ok(())
}
