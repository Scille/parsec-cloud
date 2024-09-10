// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

#[derive(Debug, Clone, thiserror::Error)]
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
    #[error("Certificate `{hint}` breaks consistency: related user cannot change profile to Outsider given it still has Owner/Manager role in some realms")]
    CannotDowngradeUserToOutsider { hint: String },
    #[error("Certificate `{hint}` breaks consistency: as first device certificate for its user it must have the same author that the user certificate ({user_author:?})")]
    UserFirstDeviceAuthorMismatch {
        hint: String,
        user_author: CertificateSignerOwned,
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
    #[error("Certificate `{hint}` breaks consistency: user has Outsider profile, and hence cannot have Owner/Manager role in the realm given it is shared with others !")]
    RealmOutsiderCannotBeOwnerOrManager { hint: String },
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
