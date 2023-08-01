# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec_pyi import (
    DataError,
    EntryNameError,
    testbed,  # Only define when build with `test-utils` feature
)
from parsec._parsec_pyi.addrs import (
    BackendActionAddr,
    BackendAddr,
    BackendInvitationAddr,
    BackendOrganizationAddr,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationFileLinkAddr,
    BackendPkiEnrollmentAddr,
    export_root_verify_key,
)
from parsec._parsec_pyi.backend_events import (
    BackendEvent,
    BackendEventCertificatesUpdated,
    BackendEventInviteConduitUpdated,
    BackendEventInviteStatusChanged,
    BackendEventMessageReceived,
    BackendEventOrganizationExpired,
    BackendEventPinged,
    BackendEventPkiEnrollmentUpdated,
    BackendEventRealmMaintenanceFinished,
    BackendEventRealmMaintenanceStarted,
    BackendEventRealmRolesUpdated,
    BackendEventRealmVlobsUpdated,
    BackendEventUserUpdatedOrRevoked,
)
from parsec._parsec_pyi.certif import (
    DeviceCertificate,
    RealmRoleCertificate,
    RevokedUserCertificate,
    SequesterAuthorityCertificate,
    SequesterServiceCertificate,
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
    VerifyKey,
    generate_nonce,
)
from parsec._parsec_pyi.enumerate import (
    InvitationStatus,
    InvitationType,
    RealmRole,
    UserProfile,
)
from parsec._parsec_pyi.ids import (
    BlockID,
    ChunkID,
    DeviceID,
    DeviceLabel,
    DeviceName,
    EnrollmentID,
    EntryID,
    HumanHandle,
    InvitationToken,
    OrganizationID,
    RealmID,
    SequesterServiceID,
    UserID,
    VlobID,
)
from parsec._parsec_pyi.manifest import (
    AnyRemoteManifest,
    BlockAccess,
    EntryName,
    FileManifest,
    FolderManifest,
    UserManifest,
    WorkspaceEntry,
    WorkspaceManifest,
    manifest_decrypt_verify_and_load,
    manifest_verify_and_load,
)
from parsec._parsec_pyi.message import (
    MessageContent,
    PingMessageContent,
    SharingGrantedMessageContent,
    SharingReencryptedMessageContent,
    SharingRevokedMessageContent,
)
from parsec._parsec_pyi.misc import ApiVersion
from parsec._parsec_pyi.organization import OrganizationConfig, OrganizationStats
from parsec._parsec_pyi.pki import (
    LocalPendingEnrollment,
    PkiEnrollmentAnswerPayload,
    PkiEnrollmentSubmitPayload,
    X509Certificate,
)
from parsec._parsec_pyi.protocol import (
    ActiveUsersLimit,
    ProtocolError,
    ProtocolErrorFields,
    ReencryptionBatchEntry,
    anonymous_cmds,
    authenticated_cmds,
    invited_cmds,
)
from parsec._parsec_pyi.regex import Regex
from parsec._parsec_pyi.time import DateTime, LocalDateTime, TimeProvider, mock_time
from parsec._parsec_pyi.user import UsersPerProfileDetailItem

__all__ = [
    "ApiVersion",
    # Data Error
    "DataError",
    "EntryNameError",
    # Certif
    "UserCertificate",
    "DeviceCertificate",
    "RevokedUserCertificate",
    "UserUpdateCertificate",
    "RealmRoleCertificate",
    "SequesterAuthorityCertificate",
    "SequesterServiceCertificate",
    # Crypto
    "SecretKey",
    "HashDigest",
    "SigningKey",
    "VerifyKey",
    "PrivateKey",
    "PublicKey",
    "SequesterPrivateKeyDer",
    "SequesterPublicKeyDer",
    "SequesterSigningKeyDer",
    "SequesterVerifyKeyDer",
    "generate_nonce",
    "CryptoError",
    # Enumerate
    "InvitationType",
    "InvitationStatus",
    "InvitationType",
    "RealmRole",
    "UserProfile",
    # Ids
    "OrganizationID",
    "EntryID",
    "BlockID",
    "VlobID",
    "ChunkID",
    "HumanHandle",
    "DeviceLabel",
    "DeviceID",
    "DeviceName",
    "UserID",
    "RealmID",
    "SequesterServiceID",
    "EnrollmentID",
    "InvitationToken",
    # Addrs
    "BackendAddr",
    "BackendActionAddr",
    "BackendInvitationAddr",
    "BackendOrganizationAddr",
    "BackendOrganizationBootstrapAddr",
    "BackendOrganizationFileLinkAddr",
    "BackendPkiEnrollmentAddr",
    "export_root_verify_key",
    # Backend internal events
    "BackendEvent",
    "BackendEventCertificatesUpdated",
    "BackendEventInviteConduitUpdated",
    "BackendEventUserUpdatedOrRevoked",
    "BackendEventOrganizationExpired",
    "BackendEventPinged",
    "BackendEventMessageReceived",
    "BackendEventInviteStatusChanged",
    "BackendEventRealmMaintenanceFinished",
    "BackendEventRealmMaintenanceStarted",
    "BackendEventRealmVlobsUpdated",
    "BackendEventRealmRolesUpdated",
    "BackendEventPkiEnrollmentUpdated",
    # Manifest
    "EntryName",
    "WorkspaceEntry",
    "BlockAccess",
    "FolderManifest",
    "FileManifest",
    "WorkspaceManifest",
    "UserManifest",
    "AnyRemoteManifest",
    "manifest_decrypt_verify_and_load",
    "manifest_verify_and_load",
    # Message
    "MessageContent",
    "SharingGrantedMessageContent",
    "SharingReencryptedMessageContent",
    "SharingRevokedMessageContent",
    "PingMessageContent",
    # Organization
    "OrganizationConfig",
    "OrganizationStats",
    # Pki
    "PkiEnrollmentAnswerPayload",
    "PkiEnrollmentSubmitPayload",
    "X509Certificate",
    "LocalPendingEnrollment",
    # User
    "UsersPerProfileDetailItem",
    # Time
    "DateTime",
    "LocalDateTime",
    "TimeProvider",
    "mock_time",
    # Protocol Cmd
    "authenticated_cmds",
    "anonymous_cmds",
    "invited_cmds",
    "ProtocolError",
    "ProtocolErrorFields",
    "ReencryptionBatchEntry",
    "ActiveUsersLimit",
    # Regex
    "Regex",
    # Testbed
    "testbed",
]
