# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec_pyi import (
    testbed,  # Only define when build with `test-utils` feature
)
from parsec._parsec_pyi.addrs import (
    ParsecActionAddr,
    ParsecAddr,
    ParsecInvitationAddr,
    ParsecOrganizationAddr,
    ParsecOrganizationBootstrapAddr,
    ParsecPkiEnrollmentAddr,
    ParsecWorkspacePathAddr,
)
from parsec._parsec_pyi.certif import (
    DeviceCertificate,
    HashAlgorithm,
    PrivateKeyAlgorithm,
    RealmArchivingCertificate,
    RealmArchivingConfiguration,
    RealmKeyRotationCertificate,
    RealmNameCertificate,
    RealmRoleCertificate,
    RevokedUserCertificate,
    SecretKeyAlgorithm,
    SequesterAuthorityCertificate,
    SequesterRevokedServiceCertificate,
    SequesterServiceCertificate,
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryDeletionCertificate,
    ShamirRecoveryShareCertificate,
    SigningKeyAlgorithm,
    UserCertificate,
    UserUpdateCertificate,
)
from parsec._parsec_pyi.crypto import (
    CryptoError,
    HashDigest,
    PrivateKey,
    PublicKey,
    SecretKey,
    SequesterPrivateKeyDer,
    SequesterPublicKeyDer,
    SequesterSigningKeyDer,
    SequesterVerifyKeyDer,
    SigningKey,
    UntrustedPasswordAlgorithm,
    UntrustedPasswordAlgorithmArgon2id,
    VerifyKey,
    generate_sas_code_nonce,
)
from parsec._parsec_pyi.enumerate import (
    CancelledGreetingAttemptReason,
    DevicePurpose,
    GreeterOrClaimer,
    InvitationStatus,
    InvitationType,
    RealmRole,
    UserProfile,
)
from parsec._parsec_pyi.ids import (
    AccountAuthMethodID,
    BlockID,
    BootstrapToken,
    ChunkID,
    DeviceID,
    DeviceLabel,
    EmailAddress,
    EnrollmentID,
    GreetingAttemptID,
    HumanHandle,
    InvitationToken,
    OrganizationID,
    SequesterServiceID,
    UserID,
    VlobID,
)
from parsec._parsec_pyi.manifest import (
    BlockAccess,
    ChildManifest,
    EntryName,
    FileManifest,
    FolderManifest,
    UserManifest,
    child_manifest_decrypt_verify_and_load,
    child_manifest_verify_and_load,
)
from parsec._parsec_pyi.misc import ApiVersion, ValidationCode
from parsec._parsec_pyi.pki import (
    LocalPendingEnrollment,
    PkiEnrollmentAnswerPayload,
    PkiEnrollmentSubmitPayload,
    X509Certificate,
)
from parsec._parsec_pyi.protocol import (
    ActiveUsersLimit,
    anonymous_account_cmds,
    anonymous_cmds,
    authenticated_account_cmds,
    authenticated_cmds,
    invited_cmds,
    tos_cmds,
)
from parsec._parsec_pyi.time import DateTime

# ruff: noqa: RUF022
# Ignore ruff rule RUF022 because the fix is marked as unsafe: it does not keep
# track of comments (or "categories") inside __all__ when sorting.
# See: https://docs.astral.sh/ruff/rules/unsorted-dunder-all/

__all__ = [
    "ApiVersion",
    "ValidationCode",
    # Data Error
    # Certif
    "DeviceCertificate",
    "HashAlgorithm",
    "PrivateKeyAlgorithm",
    "RealmArchivingCertificate",
    "RealmArchivingConfiguration",
    "RealmKeyRotationCertificate",
    "RealmNameCertificate",
    "RealmRoleCertificate",
    "RevokedUserCertificate",
    "SecretKeyAlgorithm",
    "SequesterAuthorityCertificate",
    "SequesterRevokedServiceCertificate",
    "SequesterServiceCertificate",
    "ShamirRecoveryBriefCertificate",
    "ShamirRecoveryDeletionCertificate",
    "ShamirRecoveryShareCertificate",
    "SigningKeyAlgorithm",
    "UserCertificate",
    "UserUpdateCertificate",
    # Crypto
    "CryptoError",
    "generate_sas_code_nonce",
    "HashDigest",
    "PrivateKey",
    "PublicKey",
    "SecretKey",
    "SequesterPrivateKeyDer",
    "SequesterPublicKeyDer",
    "SequesterSigningKeyDer",
    "SequesterVerifyKeyDer",
    "SigningKey",
    "VerifyKey",
    "UntrustedPasswordAlgorithm",
    "UntrustedPasswordAlgorithmArgon2id",
    # Enumerate
    "CancelledGreetingAttemptReason",
    "DevicePurpose",
    "GreeterOrClaimer",
    "InvitationStatus",
    "InvitationType",
    "InvitationType",
    "RealmRole",
    "UserProfile",
    # Ids
    "AccountAuthMethodID",
    "BlockID",
    "BootstrapToken",
    "ChunkID",
    "DeviceID",
    "DeviceLabel",
    "EmailAddress",
    "EnrollmentID",
    "GreetingAttemptID",
    "HumanHandle",
    "InvitationToken",
    "OrganizationID",
    "SequesterServiceID",
    "UserID",
    "VlobID",
    "VlobID",
    "VlobID",
    # Addrs
    "ParsecActionAddr",
    "ParsecAddr",
    "ParsecInvitationAddr",
    "ParsecOrganizationAddr",
    "ParsecOrganizationBootstrapAddr",
    "ParsecPkiEnrollmentAddr",
    "ParsecWorkspacePathAddr",
    # Manifest
    "BlockAccess",
    "child_manifest_decrypt_verify_and_load",
    "child_manifest_verify_and_load",
    "ChildManifest",
    "EntryName",
    "FileManifest",
    "FolderManifest",
    "UserManifest",
    # Time
    "DateTime",
    # Pki
    "LocalPendingEnrollment",
    "PkiEnrollmentAnswerPayload",
    "PkiEnrollmentSubmitPayload",
    "X509Certificate",
    # Protocol Cmd
    "ActiveUsersLimit",
    "anonymous_cmds",
    "authenticated_cmds",
    "invited_cmds",
    "tos_cmds",
    "authenticated_account_cmds",
    "anonymous_account_cmds",
    # Testbed
    "testbed",
]
