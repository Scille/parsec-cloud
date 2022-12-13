# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec_pyi import (
    DataError,
    EntryNameError,
    PkiEnrollmentError,
    PkiEnrollmentLocalPendingCannotReadError,
    PkiEnrollmentLocalPendingCannotRemoveError,
    PkiEnrollmentLocalPendingCannotSaveError,
    PkiEnrollmentLocalPendingError,
    PkiEnrollmentLocalPendingValidationError,
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
from parsec._parsec_pyi.backend_connection import AuthenticatedCmds
from parsec._parsec_pyi.certif import (
    DeviceCertificate,
    RealmRoleCertificate,
    RevokedUserCertificate,
    SequesterAuthorityCertificate,
    SequesterServiceCertificate,
    UserCertificate,
)
from parsec._parsec_pyi.crypto import (
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
from parsec._parsec_pyi.device import DeviceFileType
from parsec._parsec_pyi.device_file import DeviceFile
from parsec._parsec_pyi.enumerate import (
    ClientType,
    InvitationDeletedReason,
    InvitationEmailSentStatus,
    InvitationStatus,
    InvitationType,
    RealmRole,
    UserProfile,
)
from parsec._parsec_pyi.events import CoreEvent
from parsec._parsec_pyi.file_operation import (
    prepare_read,
    prepare_reshape,
    prepare_resize,
    prepare_write,
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
from parsec._parsec_pyi.invite import (
    InviteDeviceConfirmation,
    InviteDeviceData,
    InviteUserConfirmation,
    InviteUserData,
    SASCode,
    generate_sas_code_candidates,
    generate_sas_codes,
)
from parsec._parsec_pyi.local_device import (
    AvailableDevice,
    DeviceInfo,
    LocalDevice,
    LocalDeviceExc,
    UserInfo,
    change_device_password,
    get_available_device,
    list_available_devices,
    load_recovery_device,
    save_device_with_password,
    save_device_with_password_in_config,
    save_recovery_device,
)
from parsec._parsec_pyi.local_manifest import (
    Chunk,
    LocalFileManifest,
    LocalFolderManifest,
    LocalUserManifest,
    LocalWorkspaceManifest,
    local_manifest_decrypt_and_load,
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
    manifest_decrypt_and_load,
    manifest_decrypt_verify_and_load,
    manifest_unverified_load,
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
    # Cmd
    AnonymousAnyCmdReq,
    AuthenticatedAnyCmdReq,
    AuthenticatedPingRep,
    AuthenticatedPingRepOk,
    AuthenticatedPingRepUnknownStatus,
    # Ping
    AuthenticatedPingReq,
    BlockCreateRep,
    BlockCreateRepAlreadyExists,
    BlockCreateRepInMaintenance,
    BlockCreateRepNotAllowed,
    BlockCreateRepNotFound,
    BlockCreateRepOk,
    BlockCreateRepTimeout,
    BlockCreateRepUnknownStatus,
    # Block
    BlockCreateReq,
    BlockReadRep,
    BlockReadRepInMaintenance,
    BlockReadRepNotAllowed,
    BlockReadRepNotFound,
    BlockReadRepOk,
    BlockReadRepTimeout,
    BlockReadRepUnknownStatus,
    BlockReadReq,
    DeviceCreateRep,
    DeviceCreateRepAlreadyExists,
    DeviceCreateRepBadUserId,
    DeviceCreateRepInvalidCertification,
    DeviceCreateRepInvalidData,
    DeviceCreateRepOk,
    DeviceCreateRepUnknownStatus,
    DeviceCreateReq,
    # Events
    EventsListenRep,
    EventsListenRepCancelled,
    EventsListenRepNoEvents,
    EventsListenRepOk,
    EventsListenRepOkInviteStatusChanged,
    EventsListenRepOkMessageReceived,
    EventsListenRepOkPinged,
    EventsListenRepOkPkiEnrollmentUpdated,
    EventsListenRepOkRealmMaintenanceFinished,
    EventsListenRepOkRealmMaintenanceStarted,
    EventsListenRepOkRealmRolesUpdated,
    EventsListenRepOkRealmVlobsUpdated,
    EventsListenRepUnknownStatus,
    EventsListenReq,
    EventsSubscribeRep,
    EventsSubscribeRepOk,
    EventsSubscribeRepUnknownStatus,
    EventsSubscribeReq,
    HumanFindRep,
    HumanFindRepNotAllowed,
    HumanFindRepOk,
    HumanFindRepUnknownStatus,
    HumanFindReq,
    HumanFindResultItem,
    # Invite
    Invite1ClaimerWaitPeerRep,
    Invite1ClaimerWaitPeerRepInvalidState,
    Invite1ClaimerWaitPeerRepNotFound,
    Invite1ClaimerWaitPeerRepOk,
    Invite1ClaimerWaitPeerRepUnknownStatus,
    Invite1ClaimerWaitPeerReq,
    Invite1GreeterWaitPeerRep,
    Invite1GreeterWaitPeerRepAlreadyDeleted,
    Invite1GreeterWaitPeerRepInvalidState,
    Invite1GreeterWaitPeerRepNotFound,
    Invite1GreeterWaitPeerRepOk,
    Invite1GreeterWaitPeerRepUnknownStatus,
    Invite1GreeterWaitPeerReq,
    Invite2aClaimerSendHashedNonceRep,
    Invite2aClaimerSendHashedNonceRepAlreadyDeleted,
    Invite2aClaimerSendHashedNonceRepInvalidState,
    Invite2aClaimerSendHashedNonceRepNotFound,
    Invite2aClaimerSendHashedNonceRepOk,
    Invite2aClaimerSendHashedNonceRepUnknownStatus,
    Invite2aClaimerSendHashedNonceReq,
    Invite2aGreeterGetHashedNonceRep,
    Invite2aGreeterGetHashedNonceRepAlreadyDeleted,
    Invite2aGreeterGetHashedNonceRepInvalidState,
    Invite2aGreeterGetHashedNonceRepNotFound,
    Invite2aGreeterGetHashedNonceRepOk,
    Invite2aGreeterGetHashedNonceRepUnknownStatus,
    Invite2aGreeterGetHashedNonceReq,
    Invite2bClaimerSendNonceRep,
    Invite2bClaimerSendNonceRepInvalidState,
    Invite2bClaimerSendNonceRepNotFound,
    Invite2bClaimerSendNonceRepOk,
    Invite2bClaimerSendNonceRepUnknownStatus,
    Invite2bClaimerSendNonceReq,
    Invite2bGreeterSendNonceRep,
    Invite2bGreeterSendNonceRepAlreadyDeleted,
    Invite2bGreeterSendNonceRepInvalidState,
    Invite2bGreeterSendNonceRepNotFound,
    Invite2bGreeterSendNonceRepOk,
    Invite2bGreeterSendNonceRepUnknownStatus,
    Invite2bGreeterSendNonceReq,
    Invite3aClaimerSignifyTrustRep,
    Invite3aClaimerSignifyTrustRepInvalidState,
    Invite3aClaimerSignifyTrustRepNotFound,
    Invite3aClaimerSignifyTrustRepOk,
    Invite3aClaimerSignifyTrustRepUnknownStatus,
    Invite3aClaimerSignifyTrustReq,
    Invite3aGreeterWaitPeerTrustRep,
    Invite3aGreeterWaitPeerTrustRepAlreadyDeleted,
    Invite3aGreeterWaitPeerTrustRepInvalidState,
    Invite3aGreeterWaitPeerTrustRepNotFound,
    Invite3aGreeterWaitPeerTrustRepOk,
    Invite3aGreeterWaitPeerTrustRepUnknownStatus,
    Invite3aGreeterWaitPeerTrustReq,
    Invite3bClaimerWaitPeerTrustRep,
    Invite3bClaimerWaitPeerTrustRepInvalidState,
    Invite3bClaimerWaitPeerTrustRepNotFound,
    Invite3bClaimerWaitPeerTrustRepOk,
    Invite3bClaimerWaitPeerTrustRepUnknownStatus,
    Invite3bClaimerWaitPeerTrustReq,
    Invite3bGreeterSignifyTrustRep,
    Invite3bGreeterSignifyTrustRepAlreadyDeleted,
    Invite3bGreeterSignifyTrustRepInvalidState,
    Invite3bGreeterSignifyTrustRepNotFound,
    Invite3bGreeterSignifyTrustRepOk,
    Invite3bGreeterSignifyTrustRepUnknownStatus,
    Invite3bGreeterSignifyTrustReq,
    Invite4ClaimerCommunicateRep,
    Invite4ClaimerCommunicateRepInvalidState,
    Invite4ClaimerCommunicateRepNotFound,
    Invite4ClaimerCommunicateRepOk,
    Invite4ClaimerCommunicateRepUnknownStatus,
    Invite4ClaimerCommunicateReq,
    Invite4GreeterCommunicateRep,
    Invite4GreeterCommunicateRepAlreadyDeleted,
    Invite4GreeterCommunicateRepInvalidState,
    Invite4GreeterCommunicateRepNotFound,
    Invite4GreeterCommunicateRepOk,
    Invite4GreeterCommunicateRepUnknownStatus,
    Invite4GreeterCommunicateReq,
    InvitedAnyCmdReq,
    InviteDeleteRep,
    InviteDeleteRepAlreadyDeleted,
    InviteDeleteRepNotFound,
    InviteDeleteRepOk,
    InviteDeleteRepUnknownStatus,
    InviteDeleteReq,
    InvitedPingRep,
    InvitedPingRepOk,
    InvitedPingRepUnknownStatus,
    InvitedPingReq,
    InviteInfoRep,
    InviteInfoRepOk,
    InviteInfoRepUnknownStatus,
    InviteInfoReq,
    InviteListItem,
    InviteListRep,
    InviteListRepOk,
    InviteListRepUnknownStatus,
    InviteListReq,
    InviteNewRep,
    InviteNewRepAlreadyMember,
    InviteNewRepNotAllowed,
    InviteNewRepNotAvailable,
    InviteNewRepOk,
    InviteNewRepUnknownStatus,
    InviteNewReq,
    MaintenanceType,
    Message,
    MessageGetRep,
    MessageGetRepOk,
    MessageGetRepUnknownStatus,
    # Message
    MessageGetReq,
    OrganizationBootstrapRep,
    OrganizationBootstrapRepAlreadyBootstrapped,
    OrganizationBootstrapRepBadTimestamp,
    OrganizationBootstrapRepInvalidCertification,
    OrganizationBootstrapRepInvalidData,
    OrganizationBootstrapRepNotFound,
    OrganizationBootstrapRepOk,
    OrganizationBootstrapRepUnknownStatus,
    OrganizationBootstrapReq,
    OrganizationConfigRep,
    OrganizationConfigRepNotFound,
    OrganizationConfigRepOk,
    OrganizationConfigRepUnknownStatus,
    OrganizationConfigReq,
    OrganizationStatsRep,
    OrganizationStatsRepNotAllowed,
    OrganizationStatsRepNotFound,
    OrganizationStatsRepOk,
    OrganizationStatsRepUnknownStatus,
    # Organization
    OrganizationStatsReq,
    # Pki commands
    PkiEnrollmentAcceptRep,
    PkiEnrollmentAcceptRepActiveUsersLimitReached,
    PkiEnrollmentAcceptRepAlreadyExists,
    PkiEnrollmentAcceptRepInvalidCertification,
    PkiEnrollmentAcceptRepInvalidData,
    PkiEnrollmentAcceptRepInvalidPayloadData,
    PkiEnrollmentAcceptRepNoLongerAvailable,
    PkiEnrollmentAcceptRepNotAllowed,
    PkiEnrollmentAcceptRepNotFound,
    PkiEnrollmentAcceptRepOk,
    PkiEnrollmentAcceptRepUnknownStatus,
    PkiEnrollmentAcceptReq,
    PkiEnrollmentInfoRep,
    PkiEnrollmentInfoRepNotFound,
    PkiEnrollmentInfoRepOk,
    PkiEnrollmentInfoRepUnknownStatus,
    PkiEnrollmentInfoReq,
    PkiEnrollmentInfoStatus,
    PkiEnrollmentListItem,
    PkiEnrollmentListRep,
    PkiEnrollmentListRepNotAllowed,
    PkiEnrollmentListRepOk,
    PkiEnrollmentListRepUnknownStatus,
    PkiEnrollmentListReq,
    PkiEnrollmentRejectRep,
    PkiEnrollmentRejectRepNoLongerAvailable,
    PkiEnrollmentRejectRepNotAllowed,
    PkiEnrollmentRejectRepNotFound,
    PkiEnrollmentRejectRepOk,
    PkiEnrollmentRejectRepUnknownStatus,
    PkiEnrollmentRejectReq,
    PkiEnrollmentStatus,
    PkiEnrollmentSubmitRep,
    PkiEnrollmentSubmitRepAlreadyEnrolled,
    PkiEnrollmentSubmitRepAlreadySubmitted,
    PkiEnrollmentSubmitRepEmailAlreadyUsed,
    PkiEnrollmentSubmitRepIdAlreadyUsed,
    PkiEnrollmentSubmitRepInvalidPayloadData,
    PkiEnrollmentSubmitRepOk,
    PkiEnrollmentSubmitRepUnknownStatus,
    PkiEnrollmentSubmitReq,
    # Protocol errors
    ProtocolError,
    ProtocolErrorFields,
    RealmCreateRep,
    RealmCreateRepAlreadyExists,
    RealmCreateRepBadTimestamp,
    RealmCreateRepInvalidCertification,
    RealmCreateRepInvalidData,
    RealmCreateRepNotFound,
    RealmCreateRepOk,
    RealmCreateRepUnknownStatus,
    # Realm
    RealmCreateReq,
    RealmFinishReencryptionMaintenanceRep,
    RealmFinishReencryptionMaintenanceRepBadEncryptionRevision,
    RealmFinishReencryptionMaintenanceRepMaintenanceError,
    RealmFinishReencryptionMaintenanceRepNotAllowed,
    RealmFinishReencryptionMaintenanceRepNotFound,
    RealmFinishReencryptionMaintenanceRepNotInMaintenance,
    RealmFinishReencryptionMaintenanceRepOk,
    RealmFinishReencryptionMaintenanceRepUnknownStatus,
    RealmFinishReencryptionMaintenanceReq,
    RealmGetRoleCertificatesRep,
    RealmGetRoleCertificatesRepNotAllowed,
    RealmGetRoleCertificatesRepNotFound,
    RealmGetRoleCertificatesRepOk,
    RealmGetRoleCertificatesRepUnknownStatus,
    RealmGetRoleCertificatesReq,
    RealmStartReencryptionMaintenanceRep,
    RealmStartReencryptionMaintenanceRepBadEncryptionRevision,
    RealmStartReencryptionMaintenanceRepBadTimestamp,
    RealmStartReencryptionMaintenanceRepInMaintenance,
    RealmStartReencryptionMaintenanceRepMaintenanceError,
    RealmStartReencryptionMaintenanceRepNotAllowed,
    RealmStartReencryptionMaintenanceRepNotFound,
    RealmStartReencryptionMaintenanceRepOk,
    RealmStartReencryptionMaintenanceRepParticipantMismatch,
    RealmStartReencryptionMaintenanceRepUnknownStatus,
    RealmStartReencryptionMaintenanceReq,
    RealmStatsRep,
    RealmStatsRepNotAllowed,
    RealmStatsRepNotFound,
    RealmStatsRepOk,
    RealmStatsRepUnknownStatus,
    RealmStatsReq,
    RealmStatusRep,
    RealmStatusRepNotAllowed,
    RealmStatusRepNotFound,
    RealmStatusRepOk,
    RealmStatusRepUnknownStatus,
    RealmStatusReq,
    RealmUpdateRolesRep,
    RealmUpdateRolesRepAlreadyGranted,
    RealmUpdateRolesRepBadTimestamp,
    RealmUpdateRolesRepIncompatibleProfile,
    RealmUpdateRolesRepInMaintenance,
    RealmUpdateRolesRepInvalidCertification,
    RealmUpdateRolesRepInvalidData,
    RealmUpdateRolesRepNotAllowed,
    RealmUpdateRolesRepNotFound,
    RealmUpdateRolesRepOk,
    RealmUpdateRolesRepRequireGreaterTimestamp,
    RealmUpdateRolesRepUnknownStatus,
    RealmUpdateRolesRepUserRevoked,
    RealmUpdateRolesReq,
    ReencryptionBatchEntry,
    Trustchain,
    UserCreateRep,
    UserCreateRepActiveUsersLimitReached,
    UserCreateRepAlreadyExists,
    UserCreateRepInvalidCertification,
    UserCreateRepInvalidData,
    UserCreateRepNotAllowed,
    UserCreateRepOk,
    UserCreateRepUnknownStatus,
    UserCreateReq,
    UserGetRep,
    UserGetRepNotFound,
    UserGetRepOk,
    UserGetRepUnknownStatus,
    # User
    UserGetReq,
    UserRevokeRep,
    UserRevokeRepAlreadyRevoked,
    UserRevokeRepInvalidCertification,
    UserRevokeRepNotAllowed,
    UserRevokeRepNotFound,
    UserRevokeRepOk,
    UserRevokeRepUnknownStatus,
    UserRevokeReq,
    VlobCreateRep,
    VlobCreateRepAlreadyExists,
    VlobCreateRepBadEncryptionRevision,
    VlobCreateRepBadTimestamp,
    VlobCreateRepInMaintenance,
    VlobCreateRepNotAllowed,
    VlobCreateRepNotASequesteredOrganization,
    VlobCreateRepOk,
    VlobCreateRepRejectedBySequesterService,
    VlobCreateRepRequireGreaterTimestamp,
    VlobCreateRepSequesterInconsistency,
    VlobCreateRepTimeout,
    VlobCreateRepUnknownStatus,
    # Vlob
    VlobCreateReq,
    VlobListVersionsRep,
    VlobListVersionsRepInMaintenance,
    VlobListVersionsRepNotAllowed,
    VlobListVersionsRepNotFound,
    VlobListVersionsRepOk,
    VlobListVersionsRepUnknownStatus,
    VlobListVersionsReq,
    VlobMaintenanceGetReencryptionBatchRep,
    VlobMaintenanceGetReencryptionBatchRepBadEncryptionRevision,
    VlobMaintenanceGetReencryptionBatchRepMaintenanceError,
    VlobMaintenanceGetReencryptionBatchRepNotAllowed,
    VlobMaintenanceGetReencryptionBatchRepNotFound,
    VlobMaintenanceGetReencryptionBatchRepNotInMaintenance,
    VlobMaintenanceGetReencryptionBatchRepOk,
    VlobMaintenanceGetReencryptionBatchRepUnknownStatus,
    VlobMaintenanceGetReencryptionBatchReq,
    VlobMaintenanceSaveReencryptionBatchRep,
    VlobMaintenanceSaveReencryptionBatchRepBadEncryptionRevision,
    VlobMaintenanceSaveReencryptionBatchRepMaintenanceError,
    VlobMaintenanceSaveReencryptionBatchRepNotAllowed,
    VlobMaintenanceSaveReencryptionBatchRepNotFound,
    VlobMaintenanceSaveReencryptionBatchRepNotInMaintenance,
    VlobMaintenanceSaveReencryptionBatchRepOk,
    VlobMaintenanceSaveReencryptionBatchRepUnknownStatus,
    VlobMaintenanceSaveReencryptionBatchReq,
    VlobPollChangesRep,
    VlobPollChangesRepInMaintenance,
    VlobPollChangesRepNotAllowed,
    VlobPollChangesRepNotFound,
    VlobPollChangesRepOk,
    VlobPollChangesRepUnknownStatus,
    VlobPollChangesReq,
    VlobReadRep,
    VlobReadRepBadEncryptionRevision,
    VlobReadRepBadVersion,
    VlobReadRepInMaintenance,
    VlobReadRepNotAllowed,
    VlobReadRepNotFound,
    VlobReadRepOk,
    VlobReadRepUnknownStatus,
    VlobReadReq,
    VlobUpdateRep,
    VlobUpdateRepBadEncryptionRevision,
    VlobUpdateRepBadTimestamp,
    VlobUpdateRepBadVersion,
    VlobUpdateRepInMaintenance,
    VlobUpdateRepNotAllowed,
    VlobUpdateRepNotASequesteredOrganization,
    VlobUpdateRepNotFound,
    VlobUpdateRepOk,
    VlobUpdateRepRejectedBySequesterService,
    VlobUpdateRepRequireGreaterTimestamp,
    VlobUpdateRepSequesterInconsistency,
    VlobUpdateRepTimeout,
    VlobUpdateRepUnknownStatus,
    VlobUpdateReq,
)
from parsec._parsec_pyi.regex import Regex
from parsec._parsec_pyi.storage.workspace_storage import (
    PseudoFileDescriptor,
    WorkspaceStorage,
    WorkspaceStorageSnapshot,
    workspace_storage_non_speculative_init,
)
from parsec._parsec_pyi.time import DateTime, LocalDateTime, TimeProvider, mock_time
from parsec._parsec_pyi.trustchain import (
    TrustchainContext,
    TrustchainError,
    TrustchainErrorException,
)
from parsec._parsec_pyi.user import UsersPerProfileDetailItem

__all__ = [
    "ApiVersion",
    # Data Error
    "DataError",
    "EntryNameError",
    "PkiEnrollmentError",
    "PkiEnrollmentLocalPendingError",
    "PkiEnrollmentLocalPendingCannotReadError",
    "PkiEnrollmentLocalPendingCannotRemoveError",
    "PkiEnrollmentLocalPendingCannotSaveError",
    "PkiEnrollmentLocalPendingValidationError",
    # Certif
    "UserCertificate",
    "DeviceCertificate",
    "RevokedUserCertificate",
    "RealmRoleCertificate",
    "SequesterAuthorityCertificate",
    "SequesterServiceCertificate",
    # Device
    "DeviceFileType",
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
    # DeviceFile
    "DeviceFile",
    # Enumerate
    "ClientType",
    "InvitationDeletedReason",
    "InvitationType",
    "InvitationEmailSentStatus",
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
    # Invite
    "SASCode",
    "generate_sas_code_candidates",
    "generate_sas_codes",
    "InviteUserConfirmation",
    "InviteDeviceData",
    "InviteDeviceConfirmation",
    "InviteUserData",
    # Addrs
    "BackendAddr",
    "BackendActionAddr",
    "BackendInvitationAddr",
    "BackendOrganizationAddr",
    "BackendOrganizationBootstrapAddr",
    "BackendOrganizationFileLinkAddr",
    "BackendPkiEnrollmentAddr",
    "export_root_verify_key",
    # Backend connection
    "AuthenticatedCmds",
    # Local Manifest
    "Chunk",
    "LocalFileManifest",
    "LocalFolderManifest",
    "LocalUserManifest",
    "LocalWorkspaceManifest",
    "local_manifest_decrypt_and_load",
    # Manifest
    "EntryName",
    "WorkspaceEntry",
    "BlockAccess",
    "FolderManifest",
    "FileManifest",
    "WorkspaceManifest",
    "UserManifest",
    "AnyRemoteManifest",
    "manifest_decrypt_and_load",
    "manifest_decrypt_verify_and_load",
    "manifest_verify_and_load",
    "manifest_unverified_load",
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
    # Trustchain
    "TrustchainContext",
    "TrustchainError",
    "TrustchainErrorException",
    # Local Device
    "AvailableDevice",
    "DeviceInfo",
    "LocalDevice",
    "LocalDeviceExc",
    "UserInfo",
    "change_device_password",
    "get_available_device",
    "list_available_devices",
    "load_recovery_device",
    "save_device_with_password",
    "save_device_with_password_in_config",
    # Storage
    "WorkspaceStorage",
    "WorkspaceStorageSnapshot",
    "PseudoFileDescriptor",
    "workspace_storage_non_speculative_init",
    "save_recovery_device",
    # File Operations
    "prepare_read",
    "prepare_reshape",
    "prepare_resize",
    "prepare_write",
    # Protocol Cmd
    "AnonymousAnyCmdReq",
    "AuthenticatedAnyCmdReq",
    "InvitedAnyCmdReq",
    # Protocol Block
    "BlockCreateReq",
    "BlockCreateRep",
    "BlockCreateRepOk",
    "BlockCreateRepAlreadyExists",
    "BlockCreateRepInMaintenance",
    "BlockCreateRepNotAllowed",
    "BlockCreateRepNotFound",
    "BlockCreateRepTimeout",
    "BlockCreateRepUnknownStatus",
    "BlockReadReq",
    "BlockReadRep",
    "BlockReadRepOk",
    "BlockReadRepInMaintenance",
    "BlockReadRepNotAllowed",
    "BlockReadRepNotFound",
    "BlockReadRepTimeout",
    "BlockReadRepUnknownStatus",
    # Invite protocol
    "Invite1ClaimerWaitPeerRep",
    "Invite1ClaimerWaitPeerRepInvalidState",
    "Invite1ClaimerWaitPeerRepNotFound",
    "Invite1ClaimerWaitPeerRepOk",
    "Invite1ClaimerWaitPeerRepUnknownStatus",
    "Invite1ClaimerWaitPeerReq",
    "Invite1GreeterWaitPeerRep",
    "Invite1GreeterWaitPeerRepAlreadyDeleted",
    "Invite1GreeterWaitPeerRepInvalidState",
    "Invite1GreeterWaitPeerRepNotFound",
    "Invite1GreeterWaitPeerRepOk",
    "Invite1GreeterWaitPeerRepUnknownStatus",
    "Invite1GreeterWaitPeerReq",
    "Invite2aClaimerSendHashedNonceRep",
    "Invite2aClaimerSendHashedNonceRepAlreadyDeleted",
    "Invite2aClaimerSendHashedNonceRepInvalidState",
    "Invite2aClaimerSendHashedNonceRepNotFound",
    "Invite2aClaimerSendHashedNonceRepOk",
    "Invite2aClaimerSendHashedNonceRepUnknownStatus",
    "Invite2aClaimerSendHashedNonceReq",
    "Invite2aGreeterGetHashedNonceRep",
    "Invite2aGreeterGetHashedNonceRepAlreadyDeleted",
    "Invite2aGreeterGetHashedNonceRepInvalidState",
    "Invite2aGreeterGetHashedNonceRepNotFound",
    "Invite2aGreeterGetHashedNonceRepOk",
    "Invite2aGreeterGetHashedNonceRepUnknownStatus",
    "Invite2aGreeterGetHashedNonceReq",
    "Invite2bClaimerSendNonceRep",
    "Invite2bClaimerSendNonceRepInvalidState",
    "Invite2bClaimerSendNonceRepNotFound",
    "Invite2bClaimerSendNonceRepOk",
    "Invite2bClaimerSendNonceRepUnknownStatus",
    "Invite2bClaimerSendNonceReq",
    "Invite2bGreeterSendNonceRep",
    "Invite2bGreeterSendNonceRepAlreadyDeleted",
    "Invite2bGreeterSendNonceRepInvalidState",
    "Invite2bGreeterSendNonceRepNotFound",
    "Invite2bGreeterSendNonceRepOk",
    "Invite2bGreeterSendNonceRepUnknownStatus",
    "Invite2bGreeterSendNonceReq",
    "Invite3aClaimerSignifyTrustRep",
    "Invite3aClaimerSignifyTrustRepInvalidState",
    "Invite3aClaimerSignifyTrustRepNotFound",
    "Invite3aClaimerSignifyTrustRepOk",
    "Invite3aClaimerSignifyTrustRepUnknownStatus",
    "Invite3aClaimerSignifyTrustReq",
    "Invite3aGreeterWaitPeerTrustRep",
    "Invite3aGreeterWaitPeerTrustRepAlreadyDeleted",
    "Invite3aGreeterWaitPeerTrustRepInvalidState",
    "Invite3aGreeterWaitPeerTrustRepNotFound",
    "Invite3aGreeterWaitPeerTrustRepOk",
    "Invite3aGreeterWaitPeerTrustRepUnknownStatus",
    "Invite3aGreeterWaitPeerTrustReq",
    "Invite3bClaimerWaitPeerTrustRep",
    "Invite3bClaimerWaitPeerTrustRepInvalidState",
    "Invite3bClaimerWaitPeerTrustRepNotFound",
    "Invite3bClaimerWaitPeerTrustRepOk",
    "Invite3bClaimerWaitPeerTrustRepUnknownStatus",
    "Invite3bClaimerWaitPeerTrustReq",
    "Invite3bGreeterSignifyTrustRep",
    "Invite3bGreeterSignifyTrustRepAlreadyDeleted",
    "Invite3bGreeterSignifyTrustRepInvalidState",
    "Invite3bGreeterSignifyTrustRepNotFound",
    "Invite3bGreeterSignifyTrustRepOk",
    "Invite3bGreeterSignifyTrustRepUnknownStatus",
    "Invite3bGreeterSignifyTrustReq",
    "Invite4ClaimerCommunicateRep",
    "Invite4ClaimerCommunicateRepInvalidState",
    "Invite4ClaimerCommunicateRepNotFound",
    "Invite4ClaimerCommunicateRepOk",
    "Invite4ClaimerCommunicateRepUnknownStatus",
    "Invite4ClaimerCommunicateReq",
    "Invite4GreeterCommunicateRep",
    "Invite4GreeterCommunicateRepAlreadyDeleted",
    "Invite4GreeterCommunicateRepInvalidState",
    "Invite4GreeterCommunicateRepNotFound",
    "Invite4GreeterCommunicateRepOk",
    "Invite4GreeterCommunicateRepUnknownStatus",
    "Invite4GreeterCommunicateReq",
    "InviteDeleteRep",
    "InviteDeleteRepAlreadyDeleted",
    "InviteDeleteRepNotFound",
    "InviteDeleteRepOk",
    "InviteDeleteRepUnknownStatus",
    "InviteDeleteReq",
    "InviteInfoRep",
    "InviteInfoRepOk",
    "InviteInfoRepUnknownStatus",
    "InviteInfoReq",
    "InviteListItem",
    "InviteListRep",
    "InviteListRepOk",
    "InviteListRepUnknownStatus",
    "InviteListReq",
    "InviteNewRep",
    "InviteNewRepAlreadyMember",
    "InviteNewRepNotAllowed",
    "InviteNewRepNotAvailable",
    "InviteNewRepOk",
    "InviteNewRepUnknownStatus",
    "InviteNewReq",
    # Events
    "CoreEvent",
    "EventsListenRep",
    "EventsListenRepCancelled",
    "EventsListenRepNoEvents",
    "EventsListenRepOk",
    "EventsListenRepOkInviteStatusChanged",
    "EventsListenRepOkMessageReceived",
    "EventsListenRepOkPinged",
    "EventsListenRepOkPkiEnrollmentUpdated",
    "EventsListenRepOkRealmMaintenanceFinished",
    "EventsListenRepOkRealmMaintenanceStarted",
    "EventsListenRepOkRealmRolesUpdated",
    "EventsListenRepOkRealmVlobsUpdated",
    "EventsListenRepUnknownStatus",
    "EventsListenReq",
    "EventsSubscribeRep",
    "EventsSubscribeRepOk",
    "EventsSubscribeRepUnknownStatus",
    "EventsSubscribeReq",
    # Protocol Message
    "MessageGetReq",
    "MessageGetRep",
    "MessageGetRepOk",
    "MessageGetRepUnknownStatus",
    "Message",
    # Protocol Organization
    "OrganizationBootstrapRep",
    "OrganizationBootstrapRepAlreadyBootstrapped",
    "OrganizationBootstrapRepBadTimestamp",
    "OrganizationBootstrapRepInvalidCertification",
    "OrganizationBootstrapRepInvalidData",
    "OrganizationBootstrapRepNotFound",
    "OrganizationBootstrapRepOk",
    "OrganizationBootstrapRepUnknownStatus",
    "OrganizationBootstrapReq",
    "OrganizationStatsReq",
    "OrganizationStatsRep",
    "OrganizationStatsRepOk",
    "OrganizationStatsRepNotAllowed",
    "OrganizationStatsRepNotFound",
    "OrganizationStatsRepUnknownStatus",
    "OrganizationConfigReq",
    "OrganizationConfigRep",
    "OrganizationConfigRepOk",
    "OrganizationConfigRepNotFound",
    "OrganizationConfigRepUnknownStatus",
    "UsersPerProfileDetailItem",
    "ActiveUsersLimit",
    # Pki commands
    "PkiEnrollmentAcceptRep",
    "PkiEnrollmentAcceptRepActiveUsersLimitReached",
    "PkiEnrollmentAcceptRepAlreadyExists",
    "PkiEnrollmentAcceptRepInvalidCertification",
    "PkiEnrollmentAcceptRepInvalidData",
    "PkiEnrollmentAcceptRepInvalidPayloadData",
    "PkiEnrollmentAcceptRepNoLongerAvailable",
    "PkiEnrollmentAcceptRepNotAllowed",
    "PkiEnrollmentAcceptRepNotFound",
    "PkiEnrollmentAcceptRepOk",
    "PkiEnrollmentAcceptRepUnknownStatus",
    "PkiEnrollmentAcceptReq",
    "PkiEnrollmentInfoRep",
    "PkiEnrollmentInfoRepNotFound",
    "PkiEnrollmentInfoRepOk",
    "PkiEnrollmentInfoRepUnknownStatus",
    "PkiEnrollmentInfoReq",
    "PkiEnrollmentInfoStatus",
    "PkiEnrollmentListItem",
    "PkiEnrollmentListRep",
    "PkiEnrollmentListRepNotAllowed",
    "PkiEnrollmentListRepOk",
    "PkiEnrollmentListRepUnknownStatus",
    "PkiEnrollmentListReq",
    "PkiEnrollmentRejectRep",
    "PkiEnrollmentRejectRepNoLongerAvailable",
    "PkiEnrollmentRejectRepNotAllowed",
    "PkiEnrollmentRejectRepNotFound",
    "PkiEnrollmentRejectRepOk",
    "PkiEnrollmentRejectRepUnknownStatus",
    "PkiEnrollmentRejectReq",
    "PkiEnrollmentSubmitRep",
    "PkiEnrollmentSubmitRepAlreadyEnrolled",
    "PkiEnrollmentSubmitRepAlreadySubmitted",
    "PkiEnrollmentSubmitRepEmailAlreadyUsed",
    "PkiEnrollmentSubmitRepIdAlreadyUsed",
    "PkiEnrollmentSubmitRepInvalidPayloadData",
    "PkiEnrollmentSubmitRepOk",
    "PkiEnrollmentSubmitRepUnknownStatus",
    "PkiEnrollmentSubmitReq",
    "ProtocolError",
    "ProtocolErrorFields",
    "PkiEnrollmentStatus",
    # Protocol Realm
    "RealmCreateReq",
    "RealmCreateRep",
    "RealmCreateRepOk",
    "RealmCreateRepInvalidCertification",
    "RealmCreateRepInvalidData",
    "RealmCreateRepNotFound",
    "RealmCreateRepAlreadyExists",
    "RealmCreateRepBadTimestamp",
    "RealmCreateRepUnknownStatus",
    "RealmStatusReq",
    "RealmStatusRep",
    "RealmStatusRepOk",
    "RealmStatusRepNotAllowed",
    "RealmStatusRepNotFound",
    "RealmStatusRepUnknownStatus",
    "RealmStatsReq",
    "RealmStatsRep",
    "RealmStatsRepOk",
    "RealmStatsRepNotAllowed",
    "RealmStatsRepNotFound",
    "RealmStatsRepUnknownStatus",
    "RealmGetRoleCertificatesReq",
    "RealmGetRoleCertificatesRep",
    "RealmGetRoleCertificatesRepOk",
    "RealmGetRoleCertificatesRepNotAllowed",
    "RealmGetRoleCertificatesRepNotFound",
    "RealmGetRoleCertificatesRepUnknownStatus",
    "RealmUpdateRolesReq",
    "RealmUpdateRolesRep",
    "RealmUpdateRolesRepOk",
    "RealmUpdateRolesRepNotAllowed",
    "RealmUpdateRolesRepInvalidCertification",
    "RealmUpdateRolesRepInvalidData",
    "RealmUpdateRolesRepAlreadyGranted",
    "RealmUpdateRolesRepIncompatibleProfile",
    "RealmUpdateRolesRepNotFound",
    "RealmUpdateRolesRepInMaintenance",
    "RealmUpdateRolesRepUserRevoked",
    "RealmUpdateRolesRepRequireGreaterTimestamp",
    "RealmUpdateRolesRepBadTimestamp",
    "RealmUpdateRolesRepUnknownStatus",
    "RealmStartReencryptionMaintenanceReq",
    "RealmStartReencryptionMaintenanceRep",
    "RealmStartReencryptionMaintenanceRepOk",
    "RealmStartReencryptionMaintenanceRepNotAllowed",
    "RealmStartReencryptionMaintenanceRepNotFound",
    "RealmStartReencryptionMaintenanceRepBadEncryptionRevision",
    "RealmStartReencryptionMaintenanceRepParticipantMismatch",
    "RealmStartReencryptionMaintenanceRepMaintenanceError",
    "RealmStartReencryptionMaintenanceRepInMaintenance",
    "RealmStartReencryptionMaintenanceRepBadTimestamp",
    "RealmStartReencryptionMaintenanceRepUnknownStatus",
    "RealmFinishReencryptionMaintenanceReq",
    "RealmFinishReencryptionMaintenanceRep",
    "RealmFinishReencryptionMaintenanceRepOk",
    "RealmFinishReencryptionMaintenanceRepNotAllowed",
    "RealmFinishReencryptionMaintenanceRepNotFound",
    "RealmFinishReencryptionMaintenanceRepBadEncryptionRevision",
    "RealmFinishReencryptionMaintenanceRepNotInMaintenance",
    "RealmFinishReencryptionMaintenanceRepMaintenanceError",
    "RealmFinishReencryptionMaintenanceRepUnknownStatus",
    "MaintenanceType",
    # Protocol Ping
    "AuthenticatedPingReq",
    "AuthenticatedPingRep",
    "AuthenticatedPingRepOk",
    "AuthenticatedPingRepUnknownStatus",
    "InvitedPingReq",
    "InvitedPingRep",
    "InvitedPingRepOk",
    "InvitedPingRepUnknownStatus",
    # Protocol User
    "UserGetReq",
    "UserGetRep",
    "UserGetRepOk",
    "UserGetRepNotFound",
    "UserGetRepUnknownStatus",
    "UserCreateReq",
    "UserCreateRep",
    "UserCreateRepOk",
    "UserCreateRepActiveUsersLimitReached",
    "UserCreateRepAlreadyExists",
    "UserCreateRepInvalidCertification",
    "UserCreateRepInvalidData",
    "UserCreateRepNotAllowed",
    "UserCreateRepUnknownStatus",
    "UserRevokeReq",
    "UserRevokeRep",
    "UserRevokeRepOk",
    "UserRevokeRepAlreadyRevoked",
    "UserRevokeRepInvalidCertification",
    "UserRevokeRepNotAllowed",
    "UserRevokeRepNotFound",
    "UserRevokeRepUnknownStatus",
    "DeviceCreateReq",
    "DeviceCreateRep",
    "DeviceCreateRepOk",
    "DeviceCreateRepAlreadyExists",
    "DeviceCreateRepBadUserId",
    "DeviceCreateRepInvalidCertification",
    "DeviceCreateRepInvalidData",
    "DeviceCreateRepUnknownStatus",
    "HumanFindReq",
    "HumanFindRep",
    "HumanFindRepOk",
    "HumanFindRepNotAllowed",
    "HumanFindRepUnknownStatus",
    "Trustchain",
    "HumanFindResultItem",
    # Protocol Vlob
    "VlobCreateReq",
    "VlobCreateRep",
    "VlobCreateRepOk",
    "VlobCreateRepAlreadyExists",
    "VlobCreateRepNotAllowed",
    "VlobCreateRepBadEncryptionRevision",
    "VlobCreateRepInMaintenance",
    "VlobCreateRepRequireGreaterTimestamp",
    "VlobCreateRepBadTimestamp",
    "VlobCreateRepNotASequesteredOrganization",
    "VlobCreateRepSequesterInconsistency",
    "VlobCreateRepRejectedBySequesterService",
    "VlobCreateRepTimeout",
    "VlobCreateRepUnknownStatus",
    "VlobReadReq",
    "VlobReadRep",
    "VlobReadRepOk",
    "VlobReadRepNotFound",
    "VlobReadRepNotAllowed",
    "VlobReadRepBadVersion",
    "VlobReadRepBadEncryptionRevision",
    "VlobReadRepInMaintenance",
    "VlobReadRepUnknownStatus",
    "VlobUpdateReq",
    "VlobUpdateRep",
    "VlobUpdateRepOk",
    "VlobUpdateRepNotFound",
    "VlobUpdateRepNotAllowed",
    "VlobUpdateRepBadVersion",
    "VlobUpdateRepBadEncryptionRevision",
    "VlobUpdateRepInMaintenance",
    "VlobUpdateRepRequireGreaterTimestamp",
    "VlobUpdateRepBadTimestamp",
    "VlobUpdateRepNotASequesteredOrganization",
    "VlobUpdateRepSequesterInconsistency",
    "VlobUpdateRepRejectedBySequesterService",
    "VlobUpdateRepTimeout",
    "VlobUpdateRepUnknownStatus",
    "VlobPollChangesReq",
    "VlobPollChangesRep",
    "VlobPollChangesRepOk",
    "VlobPollChangesRepNotFound",
    "VlobPollChangesRepNotAllowed",
    "VlobPollChangesRepInMaintenance",
    "VlobPollChangesRepUnknownStatus",
    "VlobListVersionsReq",
    "VlobListVersionsRep",
    "VlobListVersionsRepOk",
    "VlobListVersionsRepNotFound",
    "VlobListVersionsRepNotAllowed",
    "VlobListVersionsRepInMaintenance",
    "VlobListVersionsRepUnknownStatus",
    "VlobMaintenanceGetReencryptionBatchReq",
    "VlobMaintenanceGetReencryptionBatchRep",
    "VlobMaintenanceGetReencryptionBatchRepOk",
    "VlobMaintenanceGetReencryptionBatchRepNotFound",
    "VlobMaintenanceGetReencryptionBatchRepNotAllowed",
    "VlobMaintenanceGetReencryptionBatchRepNotInMaintenance",
    "VlobMaintenanceGetReencryptionBatchRepBadEncryptionRevision",
    "VlobMaintenanceGetReencryptionBatchRepMaintenanceError",
    "VlobMaintenanceGetReencryptionBatchRepUnknownStatus",
    "VlobMaintenanceSaveReencryptionBatchReq",
    "VlobMaintenanceSaveReencryptionBatchRep",
    "VlobMaintenanceSaveReencryptionBatchRepOk",
    "VlobMaintenanceSaveReencryptionBatchRepNotFound",
    "VlobMaintenanceSaveReencryptionBatchRepNotAllowed",
    "VlobMaintenanceSaveReencryptionBatchRepNotInMaintenance",
    "VlobMaintenanceSaveReencryptionBatchRepBadEncryptionRevision",
    "VlobMaintenanceSaveReencryptionBatchRepMaintenanceError",
    "VlobMaintenanceSaveReencryptionBatchRepUnknownStatus",
    "ReencryptionBatchEntry",
    # Regex
    "Regex",
]
