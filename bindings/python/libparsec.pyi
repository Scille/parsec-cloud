# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#
# /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
#

from typing import TypedDict, Literal
from collections.abc import Callable

DeviceFileType = (
    Literal["Keyring"] | Literal["Password"] | Literal["Recovery"] | Literal["Smartcard"]
)
InvitationEmailSentStatus = (
    Literal["RecipientRefused"] | Literal["ServerUnavailable"] | Literal["Success"]
)
InvitationStatus = Literal["Cancelled"] | Literal["Finished"] | Literal["Idle"] | Literal["Ready"]
Platform = (
    Literal["Android"] | Literal["Linux"] | Literal["MacOS"] | Literal["Web"] | Literal["Windows"]
)
RealmRole = Literal["Contributor"] | Literal["Manager"] | Literal["Owner"] | Literal["Reader"]
UserProfile = Literal["Admin"] | Literal["Outsider"] | Literal["Standard"]
DeviceID = str
DeviceLabel = str
EntryName = str
FsPath = str
InvitationToken = str
OrganizationID = str
ParsecAddr = str
ParsecInvitationAddr = str
ParsecOrganizationAddr = str
ParsecOrganizationBootstrapAddr = str
ParsecPkiEnrollmentAddr = str
ParsecWorkspacePathAddr = str
Password = str
Path = str
SASCode = str
UserID = str
VlobID = str
SequesterVerifyKeyDer = bytes
I32 = int
CacheSize = int
FileDescriptor = int
Handle = int
U32 = int
VersionInt = int
I64 = int
IndexInt = int
SizeInt = int
U64 = int
DateTime = float

class AvailableDevice(TypedDict):
    key_file_path: Path
    created_on: DateTime
    protected_on: DateTime
    server_url: str
    organization_id: OrganizationID
    user_id: UserID
    device_id: DeviceID
    human_handle: HumanHandle
    device_label: DeviceLabel
    ty: DeviceFileType

class ClientConfig(TypedDict):
    config_dir: Path
    data_base_dir: Path
    mountpoint_mount_strategy: MountpointMountStrategy
    workspace_storage_cache_size: WorkspaceStorageCacheSize
    with_monitors: bool

class ClientInfo(TypedDict):
    organization_addr: ParsecOrganizationAddr
    organization_id: OrganizationID
    device_id: DeviceID
    user_id: UserID
    device_label: DeviceLabel
    human_handle: HumanHandle
    current_profile: UserProfile
    server_config: ServerConfig

class DeviceClaimFinalizeInfo(TypedDict):
    handle: Handle

class DeviceClaimInProgress1Info(TypedDict):
    handle: Handle
    greeter_sas: SASCode
    greeter_sas_choices: list[SASCode]

class DeviceClaimInProgress2Info(TypedDict):
    handle: Handle
    claimer_sas: SASCode

class DeviceClaimInProgress3Info(TypedDict):
    handle: Handle

class DeviceGreetInProgress1Info(TypedDict):
    handle: Handle
    greeter_sas: SASCode

class DeviceGreetInProgress2Info(TypedDict):
    handle: Handle
    claimer_sas: SASCode
    claimer_sas_choices: list[SASCode]

class DeviceGreetInProgress3Info(TypedDict):
    handle: Handle

class DeviceGreetInProgress4Info(TypedDict):
    handle: Handle
    requested_device_label: DeviceLabel

class DeviceGreetInitialInfo(TypedDict):
    handle: Handle

class DeviceInfo(TypedDict):
    id: DeviceID
    device_label: DeviceLabel
    created_on: DateTime
    created_by: DeviceID | None

class HumanHandle(TypedDict):
    email: str
    label: str

class NewInvitationInfo(TypedDict):
    addr: ParsecInvitationAddr
    token: InvitationToken
    email_sent_status: InvitationEmailSentStatus

class OpenOptions(TypedDict):
    read: bool
    write: bool
    truncate: bool
    create: bool
    create_new: bool

class ServerConfig(TypedDict):
    user_profile_outsider_allowed: bool
    active_users_limit: ActiveUsersLimit

class StartedWorkspaceInfo(TypedDict):
    client: Handle
    id: VlobID
    current_name: EntryName
    current_self_role: RealmRole
    mountpoints: list[tuple[Handle, Path]]

class UserClaimFinalizeInfo(TypedDict):
    handle: Handle

class UserClaimInProgress1Info(TypedDict):
    handle: Handle
    greeter_sas: SASCode
    greeter_sas_choices: list[SASCode]

class UserClaimInProgress2Info(TypedDict):
    handle: Handle
    claimer_sas: SASCode

class UserClaimInProgress3Info(TypedDict):
    handle: Handle

class UserGreetInProgress1Info(TypedDict):
    handle: Handle
    greeter_sas: SASCode

class UserGreetInProgress2Info(TypedDict):
    handle: Handle
    claimer_sas: SASCode
    claimer_sas_choices: list[SASCode]

class UserGreetInProgress3Info(TypedDict):
    handle: Handle

class UserGreetInProgress4Info(TypedDict):
    handle: Handle
    requested_human_handle: HumanHandle
    requested_device_label: DeviceLabel

class UserGreetInitialInfo(TypedDict):
    handle: Handle

class UserInfo(TypedDict):
    id: UserID
    human_handle: HumanHandle
    current_profile: UserProfile
    created_on: DateTime
    created_by: DeviceID | None
    revoked_on: DateTime | None
    revoked_by: DeviceID | None

class WorkspaceInfo(TypedDict):
    id: VlobID
    current_name: EntryName
    current_self_role: RealmRole
    is_started: bool
    is_bootstrapped: bool

class WorkspaceUserAccessInfo(TypedDict):
    user_id: UserID
    human_handle: HumanHandle
    current_profile: UserProfile
    current_role: RealmRole

# ActiveUsersLimit
class ActiveUsersLimitLimitedTo(TypedDict):
    tag: Literal["LimitedTo"]
    x1: U64

class ActiveUsersLimitNoLimit(TypedDict):
    tag: Literal["NoLimit"]

ActiveUsersLimit = ActiveUsersLimitLimitedTo | ActiveUsersLimitNoLimit

# BootstrapOrganizationError
class BootstrapOrganizationErrorAlreadyUsedToken(TypedDict):
    tag: Literal["AlreadyUsedToken"]
    error: str

class BootstrapOrganizationErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class BootstrapOrganizationErrorInvalidToken(TypedDict):
    tag: Literal["InvalidToken"]
    error: str

class BootstrapOrganizationErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

class BootstrapOrganizationErrorOrganizationExpired(TypedDict):
    tag: Literal["OrganizationExpired"]
    error: str

class BootstrapOrganizationErrorSaveDeviceError(TypedDict):
    tag: Literal["SaveDeviceError"]
    error: str

class BootstrapOrganizationErrorTimestampOutOfBallpark(TypedDict):
    tag: Literal["TimestampOutOfBallpark"]
    error: str
    server_timestamp: DateTime
    client_timestamp: DateTime
    ballpark_client_early_offset: float
    ballpark_client_late_offset: float

BootstrapOrganizationError = (
    BootstrapOrganizationErrorAlreadyUsedToken
    | BootstrapOrganizationErrorInternal
    | BootstrapOrganizationErrorInvalidToken
    | BootstrapOrganizationErrorOffline
    | BootstrapOrganizationErrorOrganizationExpired
    | BootstrapOrganizationErrorSaveDeviceError
    | BootstrapOrganizationErrorTimestampOutOfBallpark
)

# CancelError
class CancelErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class CancelErrorNotBound(TypedDict):
    tag: Literal["NotBound"]
    error: str

CancelError = CancelErrorInternal | CancelErrorNotBound

# ClaimInProgressError
class ClaimInProgressErrorActiveUsersLimitReached(TypedDict):
    tag: Literal["ActiveUsersLimitReached"]
    error: str

class ClaimInProgressErrorAlreadyUsed(TypedDict):
    tag: Literal["AlreadyUsed"]
    error: str

class ClaimInProgressErrorCancelled(TypedDict):
    tag: Literal["Cancelled"]
    error: str

class ClaimInProgressErrorCorruptedConfirmation(TypedDict):
    tag: Literal["CorruptedConfirmation"]
    error: str

class ClaimInProgressErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClaimInProgressErrorNotFound(TypedDict):
    tag: Literal["NotFound"]
    error: str

class ClaimInProgressErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

class ClaimInProgressErrorOrganizationExpired(TypedDict):
    tag: Literal["OrganizationExpired"]
    error: str

class ClaimInProgressErrorPeerReset(TypedDict):
    tag: Literal["PeerReset"]
    error: str

ClaimInProgressError = (
    ClaimInProgressErrorActiveUsersLimitReached
    | ClaimInProgressErrorAlreadyUsed
    | ClaimInProgressErrorCancelled
    | ClaimInProgressErrorCorruptedConfirmation
    | ClaimInProgressErrorInternal
    | ClaimInProgressErrorNotFound
    | ClaimInProgressErrorOffline
    | ClaimInProgressErrorOrganizationExpired
    | ClaimInProgressErrorPeerReset
)

# ClaimerGreeterAbortOperationError
class ClaimerGreeterAbortOperationErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

ClaimerGreeterAbortOperationError = ClaimerGreeterAbortOperationErrorInternal

# ClaimerRetrieveInfoError
class ClaimerRetrieveInfoErrorAlreadyUsed(TypedDict):
    tag: Literal["AlreadyUsed"]
    error: str

class ClaimerRetrieveInfoErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClaimerRetrieveInfoErrorNotFound(TypedDict):
    tag: Literal["NotFound"]
    error: str

class ClaimerRetrieveInfoErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

ClaimerRetrieveInfoError = (
    ClaimerRetrieveInfoErrorAlreadyUsed
    | ClaimerRetrieveInfoErrorInternal
    | ClaimerRetrieveInfoErrorNotFound
    | ClaimerRetrieveInfoErrorOffline
)

# ClientCancelInvitationError
class ClientCancelInvitationErrorAlreadyDeleted(TypedDict):
    tag: Literal["AlreadyDeleted"]
    error: str

class ClientCancelInvitationErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClientCancelInvitationErrorNotFound(TypedDict):
    tag: Literal["NotFound"]
    error: str

class ClientCancelInvitationErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

ClientCancelInvitationError = (
    ClientCancelInvitationErrorAlreadyDeleted
    | ClientCancelInvitationErrorInternal
    | ClientCancelInvitationErrorNotFound
    | ClientCancelInvitationErrorOffline
)

# ClientChangeAuthenticationError
class ClientChangeAuthenticationErrorDecryptionFailed(TypedDict):
    tag: Literal["DecryptionFailed"]
    error: str

class ClientChangeAuthenticationErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClientChangeAuthenticationErrorInvalidData(TypedDict):
    tag: Literal["InvalidData"]
    error: str

class ClientChangeAuthenticationErrorInvalidPath(TypedDict):
    tag: Literal["InvalidPath"]
    error: str

ClientChangeAuthenticationError = (
    ClientChangeAuthenticationErrorDecryptionFailed
    | ClientChangeAuthenticationErrorInternal
    | ClientChangeAuthenticationErrorInvalidData
    | ClientChangeAuthenticationErrorInvalidPath
)

# ClientCreateWorkspaceError
class ClientCreateWorkspaceErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClientCreateWorkspaceErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

ClientCreateWorkspaceError = ClientCreateWorkspaceErrorInternal | ClientCreateWorkspaceErrorStopped

# ClientEvent
class ClientEventExpiredOrganization(TypedDict):
    tag: Literal["ExpiredOrganization"]

class ClientEventIncompatibleServer(TypedDict):
    tag: Literal["IncompatibleServer"]
    detail: str

class ClientEventInvitationChanged(TypedDict):
    tag: Literal["InvitationChanged"]
    token: InvitationToken
    status: InvitationStatus

class ClientEventOffline(TypedDict):
    tag: Literal["Offline"]

class ClientEventOnline(TypedDict):
    tag: Literal["Online"]

class ClientEventPing(TypedDict):
    tag: Literal["Ping"]
    ping: str

class ClientEventRevokedSelfUser(TypedDict):
    tag: Literal["RevokedSelfUser"]

class ClientEventServerConfigChanged(TypedDict):
    tag: Literal["ServerConfigChanged"]

class ClientEventTooMuchDriftWithServerClock(TypedDict):
    tag: Literal["TooMuchDriftWithServerClock"]
    server_timestamp: DateTime
    client_timestamp: DateTime
    ballpark_client_early_offset: float
    ballpark_client_late_offset: float

class ClientEventWorkspaceLocallyCreated(TypedDict):
    tag: Literal["WorkspaceLocallyCreated"]

class ClientEventWorkspacesSelfAccessChanged(TypedDict):
    tag: Literal["WorkspacesSelfAccessChanged"]

ClientEvent = (
    ClientEventExpiredOrganization
    | ClientEventIncompatibleServer
    | ClientEventInvitationChanged
    | ClientEventOffline
    | ClientEventOnline
    | ClientEventPing
    | ClientEventRevokedSelfUser
    | ClientEventServerConfigChanged
    | ClientEventTooMuchDriftWithServerClock
    | ClientEventWorkspaceLocallyCreated
    | ClientEventWorkspacesSelfAccessChanged
)

# ClientGetUserDeviceError
class ClientGetUserDeviceErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClientGetUserDeviceErrorNonExisting(TypedDict):
    tag: Literal["NonExisting"]
    error: str

class ClientGetUserDeviceErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

ClientGetUserDeviceError = (
    ClientGetUserDeviceErrorInternal
    | ClientGetUserDeviceErrorNonExisting
    | ClientGetUserDeviceErrorStopped
)

# ClientInfoError
class ClientInfoErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClientInfoErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

ClientInfoError = ClientInfoErrorInternal | ClientInfoErrorStopped

# ClientListUserDevicesError
class ClientListUserDevicesErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClientListUserDevicesErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

ClientListUserDevicesError = ClientListUserDevicesErrorInternal | ClientListUserDevicesErrorStopped

# ClientListUsersError
class ClientListUsersErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClientListUsersErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

ClientListUsersError = ClientListUsersErrorInternal | ClientListUsersErrorStopped

# ClientListWorkspaceUsersError
class ClientListWorkspaceUsersErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClientListWorkspaceUsersErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

ClientListWorkspaceUsersError = (
    ClientListWorkspaceUsersErrorInternal | ClientListWorkspaceUsersErrorStopped
)

# ClientListWorkspacesError
class ClientListWorkspacesErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

ClientListWorkspacesError = ClientListWorkspacesErrorInternal

# ClientNewDeviceInvitationError
class ClientNewDeviceInvitationErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClientNewDeviceInvitationErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

ClientNewDeviceInvitationError = (
    ClientNewDeviceInvitationErrorInternal | ClientNewDeviceInvitationErrorOffline
)

# ClientNewUserInvitationError
class ClientNewUserInvitationErrorAlreadyMember(TypedDict):
    tag: Literal["AlreadyMember"]
    error: str

class ClientNewUserInvitationErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClientNewUserInvitationErrorNotAllowed(TypedDict):
    tag: Literal["NotAllowed"]
    error: str

class ClientNewUserInvitationErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

ClientNewUserInvitationError = (
    ClientNewUserInvitationErrorAlreadyMember
    | ClientNewUserInvitationErrorInternal
    | ClientNewUserInvitationErrorNotAllowed
    | ClientNewUserInvitationErrorOffline
)

# ClientRenameWorkspaceError
class ClientRenameWorkspaceErrorAuthorNotAllowed(TypedDict):
    tag: Literal["AuthorNotAllowed"]
    error: str

class ClientRenameWorkspaceErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClientRenameWorkspaceErrorInvalidCertificate(TypedDict):
    tag: Literal["InvalidCertificate"]
    error: str

class ClientRenameWorkspaceErrorInvalidEncryptedRealmName(TypedDict):
    tag: Literal["InvalidEncryptedRealmName"]
    error: str

class ClientRenameWorkspaceErrorInvalidKeysBundle(TypedDict):
    tag: Literal["InvalidKeysBundle"]
    error: str

class ClientRenameWorkspaceErrorNoKey(TypedDict):
    tag: Literal["NoKey"]
    error: str

class ClientRenameWorkspaceErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

class ClientRenameWorkspaceErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

class ClientRenameWorkspaceErrorTimestampOutOfBallpark(TypedDict):
    tag: Literal["TimestampOutOfBallpark"]
    error: str
    server_timestamp: DateTime
    client_timestamp: DateTime
    ballpark_client_early_offset: float
    ballpark_client_late_offset: float

class ClientRenameWorkspaceErrorWorkspaceNotFound(TypedDict):
    tag: Literal["WorkspaceNotFound"]
    error: str

ClientRenameWorkspaceError = (
    ClientRenameWorkspaceErrorAuthorNotAllowed
    | ClientRenameWorkspaceErrorInternal
    | ClientRenameWorkspaceErrorInvalidCertificate
    | ClientRenameWorkspaceErrorInvalidEncryptedRealmName
    | ClientRenameWorkspaceErrorInvalidKeysBundle
    | ClientRenameWorkspaceErrorNoKey
    | ClientRenameWorkspaceErrorOffline
    | ClientRenameWorkspaceErrorStopped
    | ClientRenameWorkspaceErrorTimestampOutOfBallpark
    | ClientRenameWorkspaceErrorWorkspaceNotFound
)

# ClientRevokeUserError
class ClientRevokeUserErrorAuthorNotAllowed(TypedDict):
    tag: Literal["AuthorNotAllowed"]
    error: str

class ClientRevokeUserErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClientRevokeUserErrorInvalidCertificate(TypedDict):
    tag: Literal["InvalidCertificate"]
    error: str

class ClientRevokeUserErrorInvalidKeysBundle(TypedDict):
    tag: Literal["InvalidKeysBundle"]
    error: str

class ClientRevokeUserErrorNoKey(TypedDict):
    tag: Literal["NoKey"]
    error: str

class ClientRevokeUserErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

class ClientRevokeUserErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

class ClientRevokeUserErrorTimestampOutOfBallpark(TypedDict):
    tag: Literal["TimestampOutOfBallpark"]
    error: str

class ClientRevokeUserErrorUserIsSelf(TypedDict):
    tag: Literal["UserIsSelf"]
    error: str

class ClientRevokeUserErrorUserNotFound(TypedDict):
    tag: Literal["UserNotFound"]
    error: str

ClientRevokeUserError = (
    ClientRevokeUserErrorAuthorNotAllowed
    | ClientRevokeUserErrorInternal
    | ClientRevokeUserErrorInvalidCertificate
    | ClientRevokeUserErrorInvalidKeysBundle
    | ClientRevokeUserErrorNoKey
    | ClientRevokeUserErrorOffline
    | ClientRevokeUserErrorStopped
    | ClientRevokeUserErrorTimestampOutOfBallpark
    | ClientRevokeUserErrorUserIsSelf
    | ClientRevokeUserErrorUserNotFound
)

# ClientShareWorkspaceError
class ClientShareWorkspaceErrorAuthorNotAllowed(TypedDict):
    tag: Literal["AuthorNotAllowed"]
    error: str

class ClientShareWorkspaceErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClientShareWorkspaceErrorInvalidCertificate(TypedDict):
    tag: Literal["InvalidCertificate"]
    error: str

class ClientShareWorkspaceErrorInvalidKeysBundle(TypedDict):
    tag: Literal["InvalidKeysBundle"]
    error: str

class ClientShareWorkspaceErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

class ClientShareWorkspaceErrorRecipientIsSelf(TypedDict):
    tag: Literal["RecipientIsSelf"]
    error: str

class ClientShareWorkspaceErrorRecipientNotFound(TypedDict):
    tag: Literal["RecipientNotFound"]
    error: str

class ClientShareWorkspaceErrorRecipientRevoked(TypedDict):
    tag: Literal["RecipientRevoked"]
    error: str

class ClientShareWorkspaceErrorRoleIncompatibleWithOutsider(TypedDict):
    tag: Literal["RoleIncompatibleWithOutsider"]
    error: str

class ClientShareWorkspaceErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

class ClientShareWorkspaceErrorTimestampOutOfBallpark(TypedDict):
    tag: Literal["TimestampOutOfBallpark"]
    error: str
    server_timestamp: DateTime
    client_timestamp: DateTime
    ballpark_client_early_offset: float
    ballpark_client_late_offset: float

class ClientShareWorkspaceErrorWorkspaceNotFound(TypedDict):
    tag: Literal["WorkspaceNotFound"]
    error: str

ClientShareWorkspaceError = (
    ClientShareWorkspaceErrorAuthorNotAllowed
    | ClientShareWorkspaceErrorInternal
    | ClientShareWorkspaceErrorInvalidCertificate
    | ClientShareWorkspaceErrorInvalidKeysBundle
    | ClientShareWorkspaceErrorOffline
    | ClientShareWorkspaceErrorRecipientIsSelf
    | ClientShareWorkspaceErrorRecipientNotFound
    | ClientShareWorkspaceErrorRecipientRevoked
    | ClientShareWorkspaceErrorRoleIncompatibleWithOutsider
    | ClientShareWorkspaceErrorStopped
    | ClientShareWorkspaceErrorTimestampOutOfBallpark
    | ClientShareWorkspaceErrorWorkspaceNotFound
)

# ClientStartError
class ClientStartErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClientStartErrorLoadDeviceDecryptionFailed(TypedDict):
    tag: Literal["LoadDeviceDecryptionFailed"]
    error: str

class ClientStartErrorLoadDeviceInvalidData(TypedDict):
    tag: Literal["LoadDeviceInvalidData"]
    error: str

class ClientStartErrorLoadDeviceInvalidPath(TypedDict):
    tag: Literal["LoadDeviceInvalidPath"]
    error: str

ClientStartError = (
    ClientStartErrorInternal
    | ClientStartErrorLoadDeviceDecryptionFailed
    | ClientStartErrorLoadDeviceInvalidData
    | ClientStartErrorLoadDeviceInvalidPath
)

# ClientStartInvitationGreetError
class ClientStartInvitationGreetErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

ClientStartInvitationGreetError = ClientStartInvitationGreetErrorInternal

# ClientStartWorkspaceError
class ClientStartWorkspaceErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ClientStartWorkspaceErrorWorkspaceNotFound(TypedDict):
    tag: Literal["WorkspaceNotFound"]
    error: str

ClientStartWorkspaceError = (
    ClientStartWorkspaceErrorInternal | ClientStartWorkspaceErrorWorkspaceNotFound
)

# ClientStopError
class ClientStopErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

ClientStopError = ClientStopErrorInternal

# DeviceAccessStrategy
class DeviceAccessStrategyKeyring(TypedDict):
    tag: Literal["Keyring"]
    key_file: Path

class DeviceAccessStrategyPassword(TypedDict):
    tag: Literal["Password"]
    password: Password
    key_file: Path

class DeviceAccessStrategySmartcard(TypedDict):
    tag: Literal["Smartcard"]
    key_file: Path

DeviceAccessStrategy = (
    DeviceAccessStrategyKeyring | DeviceAccessStrategyPassword | DeviceAccessStrategySmartcard
)

# DeviceSaveStrategy
class DeviceSaveStrategyKeyring(TypedDict):
    tag: Literal["Keyring"]

class DeviceSaveStrategyPassword(TypedDict):
    tag: Literal["Password"]
    password: Password

class DeviceSaveStrategySmartcard(TypedDict):
    tag: Literal["Smartcard"]

DeviceSaveStrategy = (
    DeviceSaveStrategyKeyring | DeviceSaveStrategyPassword | DeviceSaveStrategySmartcard
)

# EntryStat
class EntryStatFile(TypedDict):
    tag: Literal["File"]
    confinement_point: VlobID | None
    id: VlobID
    parent: VlobID
    created: DateTime
    updated: DateTime
    base_version: VersionInt
    is_placeholder: bool
    need_sync: bool
    size: SizeInt

class EntryStatFolder(TypedDict):
    tag: Literal["Folder"]
    confinement_point: VlobID | None
    id: VlobID
    parent: VlobID
    created: DateTime
    updated: DateTime
    base_version: VersionInt
    is_placeholder: bool
    need_sync: bool

EntryStat = EntryStatFile | EntryStatFolder

# GreetInProgressError
class GreetInProgressErrorActiveUsersLimitReached(TypedDict):
    tag: Literal["ActiveUsersLimitReached"]
    error: str

class GreetInProgressErrorAlreadyDeleted(TypedDict):
    tag: Literal["AlreadyDeleted"]
    error: str

class GreetInProgressErrorCancelled(TypedDict):
    tag: Literal["Cancelled"]
    error: str

class GreetInProgressErrorCorruptedInviteUserData(TypedDict):
    tag: Literal["CorruptedInviteUserData"]
    error: str

class GreetInProgressErrorDeviceAlreadyExists(TypedDict):
    tag: Literal["DeviceAlreadyExists"]
    error: str

class GreetInProgressErrorHumanHandleAlreadyTaken(TypedDict):
    tag: Literal["HumanHandleAlreadyTaken"]
    error: str

class GreetInProgressErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class GreetInProgressErrorNonceMismatch(TypedDict):
    tag: Literal["NonceMismatch"]
    error: str

class GreetInProgressErrorNotFound(TypedDict):
    tag: Literal["NotFound"]
    error: str

class GreetInProgressErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

class GreetInProgressErrorPeerReset(TypedDict):
    tag: Literal["PeerReset"]
    error: str

class GreetInProgressErrorTimestampOutOfBallpark(TypedDict):
    tag: Literal["TimestampOutOfBallpark"]
    error: str
    server_timestamp: DateTime
    client_timestamp: DateTime
    ballpark_client_early_offset: float
    ballpark_client_late_offset: float

class GreetInProgressErrorUserAlreadyExists(TypedDict):
    tag: Literal["UserAlreadyExists"]
    error: str

class GreetInProgressErrorUserCreateNotAllowed(TypedDict):
    tag: Literal["UserCreateNotAllowed"]
    error: str

GreetInProgressError = (
    GreetInProgressErrorActiveUsersLimitReached
    | GreetInProgressErrorAlreadyDeleted
    | GreetInProgressErrorCancelled
    | GreetInProgressErrorCorruptedInviteUserData
    | GreetInProgressErrorDeviceAlreadyExists
    | GreetInProgressErrorHumanHandleAlreadyTaken
    | GreetInProgressErrorInternal
    | GreetInProgressErrorNonceMismatch
    | GreetInProgressErrorNotFound
    | GreetInProgressErrorOffline
    | GreetInProgressErrorPeerReset
    | GreetInProgressErrorTimestampOutOfBallpark
    | GreetInProgressErrorUserAlreadyExists
    | GreetInProgressErrorUserCreateNotAllowed
)

# InviteListItem
class InviteListItemDevice(TypedDict):
    tag: Literal["Device"]
    addr: ParsecInvitationAddr
    token: InvitationToken
    created_on: DateTime
    status: InvitationStatus

class InviteListItemUser(TypedDict):
    tag: Literal["User"]
    addr: ParsecInvitationAddr
    token: InvitationToken
    created_on: DateTime
    claimer_email: str
    status: InvitationStatus

InviteListItem = InviteListItemDevice | InviteListItemUser

# ListInvitationsError
class ListInvitationsErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class ListInvitationsErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

ListInvitationsError = ListInvitationsErrorInternal | ListInvitationsErrorOffline

# MountpointMountStrategy
class MountpointMountStrategyDirectory(TypedDict):
    tag: Literal["Directory"]
    base_dir: Path

class MountpointMountStrategyDisabled(TypedDict):
    tag: Literal["Disabled"]

class MountpointMountStrategyDriveLetter(TypedDict):
    tag: Literal["DriveLetter"]

MountpointMountStrategy = (
    MountpointMountStrategyDirectory
    | MountpointMountStrategyDisabled
    | MountpointMountStrategyDriveLetter
)

# MountpointToOsPathError
class MountpointToOsPathErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

MountpointToOsPathError = MountpointToOsPathErrorInternal

# MountpointUnmountError
class MountpointUnmountErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

MountpointUnmountError = MountpointUnmountErrorInternal

# MoveEntryMode
class MoveEntryModeCanReplace(TypedDict):
    tag: Literal["CanReplace"]

class MoveEntryModeExchange(TypedDict):
    tag: Literal["Exchange"]

class MoveEntryModeNoReplace(TypedDict):
    tag: Literal["NoReplace"]

MoveEntryMode = MoveEntryModeCanReplace | MoveEntryModeExchange | MoveEntryModeNoReplace

# ParseParsecAddrError
class ParseParsecAddrErrorInvalidUrl(TypedDict):
    tag: Literal["InvalidUrl"]
    error: str

ParseParsecAddrError = ParseParsecAddrErrorInvalidUrl

# ParsedParsecAddr
class ParsedParsecAddrInvitationDevice(TypedDict):
    tag: Literal["InvitationDevice"]
    hostname: str
    port: U32
    use_ssl: bool
    organization_id: OrganizationID
    token: InvitationToken

class ParsedParsecAddrInvitationUser(TypedDict):
    tag: Literal["InvitationUser"]
    hostname: str
    port: U32
    use_ssl: bool
    organization_id: OrganizationID
    token: InvitationToken

class ParsedParsecAddrOrganization(TypedDict):
    tag: Literal["Organization"]
    hostname: str
    port: U32
    use_ssl: bool
    organization_id: OrganizationID

class ParsedParsecAddrOrganizationBootstrap(TypedDict):
    tag: Literal["OrganizationBootstrap"]
    hostname: str
    port: U32
    use_ssl: bool
    organization_id: OrganizationID
    token: str | None

class ParsedParsecAddrPkiEnrollment(TypedDict):
    tag: Literal["PkiEnrollment"]
    hostname: str
    port: U32
    use_ssl: bool
    organization_id: OrganizationID

class ParsedParsecAddrServer(TypedDict):
    tag: Literal["Server"]
    hostname: str
    port: U32
    use_ssl: bool

class ParsedParsecAddrWorkspacePath(TypedDict):
    tag: Literal["WorkspacePath"]
    hostname: str
    port: U32
    use_ssl: bool
    organization_id: OrganizationID
    workspace_id: VlobID
    key_index: IndexInt
    encrypted_path: bytes

ParsedParsecAddr = (
    ParsedParsecAddrInvitationDevice
    | ParsedParsecAddrInvitationUser
    | ParsedParsecAddrOrganization
    | ParsedParsecAddrOrganizationBootstrap
    | ParsedParsecAddrPkiEnrollment
    | ParsedParsecAddrServer
    | ParsedParsecAddrWorkspacePath
)

# TestbedError
class TestbedErrorDisabled(TypedDict):
    tag: Literal["Disabled"]
    error: str

class TestbedErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

TestbedError = TestbedErrorDisabled | TestbedErrorInternal

# UserOrDeviceClaimInitialInfo
class UserOrDeviceClaimInitialInfoDevice(TypedDict):
    tag: Literal["Device"]
    handle: Handle
    greeter_user_id: UserID
    greeter_human_handle: HumanHandle

class UserOrDeviceClaimInitialInfoUser(TypedDict):
    tag: Literal["User"]
    handle: Handle
    claimer_email: str
    greeter_user_id: UserID
    greeter_human_handle: HumanHandle

UserOrDeviceClaimInitialInfo = UserOrDeviceClaimInitialInfoDevice | UserOrDeviceClaimInitialInfoUser

# WorkspaceCreateFileError
class WorkspaceCreateFileErrorEntryExists(TypedDict):
    tag: Literal["EntryExists"]
    error: str

class WorkspaceCreateFileErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class WorkspaceCreateFileErrorInvalidCertificate(TypedDict):
    tag: Literal["InvalidCertificate"]
    error: str

class WorkspaceCreateFileErrorInvalidKeysBundle(TypedDict):
    tag: Literal["InvalidKeysBundle"]
    error: str

class WorkspaceCreateFileErrorInvalidManifest(TypedDict):
    tag: Literal["InvalidManifest"]
    error: str

class WorkspaceCreateFileErrorNoRealmAccess(TypedDict):
    tag: Literal["NoRealmAccess"]
    error: str

class WorkspaceCreateFileErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

class WorkspaceCreateFileErrorParentNotAFolder(TypedDict):
    tag: Literal["ParentNotAFolder"]
    error: str

class WorkspaceCreateFileErrorParentNotFound(TypedDict):
    tag: Literal["ParentNotFound"]
    error: str

class WorkspaceCreateFileErrorReadOnlyRealm(TypedDict):
    tag: Literal["ReadOnlyRealm"]
    error: str

class WorkspaceCreateFileErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

WorkspaceCreateFileError = (
    WorkspaceCreateFileErrorEntryExists
    | WorkspaceCreateFileErrorInternal
    | WorkspaceCreateFileErrorInvalidCertificate
    | WorkspaceCreateFileErrorInvalidKeysBundle
    | WorkspaceCreateFileErrorInvalidManifest
    | WorkspaceCreateFileErrorNoRealmAccess
    | WorkspaceCreateFileErrorOffline
    | WorkspaceCreateFileErrorParentNotAFolder
    | WorkspaceCreateFileErrorParentNotFound
    | WorkspaceCreateFileErrorReadOnlyRealm
    | WorkspaceCreateFileErrorStopped
)

# WorkspaceCreateFolderError
class WorkspaceCreateFolderErrorEntryExists(TypedDict):
    tag: Literal["EntryExists"]
    error: str

class WorkspaceCreateFolderErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class WorkspaceCreateFolderErrorInvalidCertificate(TypedDict):
    tag: Literal["InvalidCertificate"]
    error: str

class WorkspaceCreateFolderErrorInvalidKeysBundle(TypedDict):
    tag: Literal["InvalidKeysBundle"]
    error: str

class WorkspaceCreateFolderErrorInvalidManifest(TypedDict):
    tag: Literal["InvalidManifest"]
    error: str

class WorkspaceCreateFolderErrorNoRealmAccess(TypedDict):
    tag: Literal["NoRealmAccess"]
    error: str

class WorkspaceCreateFolderErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

class WorkspaceCreateFolderErrorParentNotAFolder(TypedDict):
    tag: Literal["ParentNotAFolder"]
    error: str

class WorkspaceCreateFolderErrorParentNotFound(TypedDict):
    tag: Literal["ParentNotFound"]
    error: str

class WorkspaceCreateFolderErrorReadOnlyRealm(TypedDict):
    tag: Literal["ReadOnlyRealm"]
    error: str

class WorkspaceCreateFolderErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

WorkspaceCreateFolderError = (
    WorkspaceCreateFolderErrorEntryExists
    | WorkspaceCreateFolderErrorInternal
    | WorkspaceCreateFolderErrorInvalidCertificate
    | WorkspaceCreateFolderErrorInvalidKeysBundle
    | WorkspaceCreateFolderErrorInvalidManifest
    | WorkspaceCreateFolderErrorNoRealmAccess
    | WorkspaceCreateFolderErrorOffline
    | WorkspaceCreateFolderErrorParentNotAFolder
    | WorkspaceCreateFolderErrorParentNotFound
    | WorkspaceCreateFolderErrorReadOnlyRealm
    | WorkspaceCreateFolderErrorStopped
)

# WorkspaceDecryptPathAddrError
class WorkspaceDecryptPathAddrErrorCorruptedData(TypedDict):
    tag: Literal["CorruptedData"]
    error: str

class WorkspaceDecryptPathAddrErrorCorruptedKey(TypedDict):
    tag: Literal["CorruptedKey"]
    error: str

class WorkspaceDecryptPathAddrErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class WorkspaceDecryptPathAddrErrorInvalidCertificate(TypedDict):
    tag: Literal["InvalidCertificate"]
    error: str

class WorkspaceDecryptPathAddrErrorInvalidKeysBundle(TypedDict):
    tag: Literal["InvalidKeysBundle"]
    error: str

class WorkspaceDecryptPathAddrErrorKeyNotFound(TypedDict):
    tag: Literal["KeyNotFound"]
    error: str

class WorkspaceDecryptPathAddrErrorNotAllowed(TypedDict):
    tag: Literal["NotAllowed"]
    error: str

class WorkspaceDecryptPathAddrErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

class WorkspaceDecryptPathAddrErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

WorkspaceDecryptPathAddrError = (
    WorkspaceDecryptPathAddrErrorCorruptedData
    | WorkspaceDecryptPathAddrErrorCorruptedKey
    | WorkspaceDecryptPathAddrErrorInternal
    | WorkspaceDecryptPathAddrErrorInvalidCertificate
    | WorkspaceDecryptPathAddrErrorInvalidKeysBundle
    | WorkspaceDecryptPathAddrErrorKeyNotFound
    | WorkspaceDecryptPathAddrErrorNotAllowed
    | WorkspaceDecryptPathAddrErrorOffline
    | WorkspaceDecryptPathAddrErrorStopped
)

# WorkspaceFdCloseError
class WorkspaceFdCloseErrorBadFileDescriptor(TypedDict):
    tag: Literal["BadFileDescriptor"]
    error: str

class WorkspaceFdCloseErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class WorkspaceFdCloseErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

WorkspaceFdCloseError = (
    WorkspaceFdCloseErrorBadFileDescriptor
    | WorkspaceFdCloseErrorInternal
    | WorkspaceFdCloseErrorStopped
)

# WorkspaceFdFlushError
class WorkspaceFdFlushErrorBadFileDescriptor(TypedDict):
    tag: Literal["BadFileDescriptor"]
    error: str

class WorkspaceFdFlushErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class WorkspaceFdFlushErrorNotInWriteMode(TypedDict):
    tag: Literal["NotInWriteMode"]
    error: str

class WorkspaceFdFlushErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

WorkspaceFdFlushError = (
    WorkspaceFdFlushErrorBadFileDescriptor
    | WorkspaceFdFlushErrorInternal
    | WorkspaceFdFlushErrorNotInWriteMode
    | WorkspaceFdFlushErrorStopped
)

# WorkspaceFdReadError
class WorkspaceFdReadErrorBadFileDescriptor(TypedDict):
    tag: Literal["BadFileDescriptor"]
    error: str

class WorkspaceFdReadErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class WorkspaceFdReadErrorInvalidBlockAccess(TypedDict):
    tag: Literal["InvalidBlockAccess"]
    error: str

class WorkspaceFdReadErrorInvalidCertificate(TypedDict):
    tag: Literal["InvalidCertificate"]
    error: str

class WorkspaceFdReadErrorInvalidKeysBundle(TypedDict):
    tag: Literal["InvalidKeysBundle"]
    error: str

class WorkspaceFdReadErrorNoRealmAccess(TypedDict):
    tag: Literal["NoRealmAccess"]
    error: str

class WorkspaceFdReadErrorNotInReadMode(TypedDict):
    tag: Literal["NotInReadMode"]
    error: str

class WorkspaceFdReadErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

class WorkspaceFdReadErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

WorkspaceFdReadError = (
    WorkspaceFdReadErrorBadFileDescriptor
    | WorkspaceFdReadErrorInternal
    | WorkspaceFdReadErrorInvalidBlockAccess
    | WorkspaceFdReadErrorInvalidCertificate
    | WorkspaceFdReadErrorInvalidKeysBundle
    | WorkspaceFdReadErrorNoRealmAccess
    | WorkspaceFdReadErrorNotInReadMode
    | WorkspaceFdReadErrorOffline
    | WorkspaceFdReadErrorStopped
)

# WorkspaceFdResizeError
class WorkspaceFdResizeErrorBadFileDescriptor(TypedDict):
    tag: Literal["BadFileDescriptor"]
    error: str

class WorkspaceFdResizeErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class WorkspaceFdResizeErrorNotInWriteMode(TypedDict):
    tag: Literal["NotInWriteMode"]
    error: str

WorkspaceFdResizeError = (
    WorkspaceFdResizeErrorBadFileDescriptor
    | WorkspaceFdResizeErrorInternal
    | WorkspaceFdResizeErrorNotInWriteMode
)

# WorkspaceFdWriteError
class WorkspaceFdWriteErrorBadFileDescriptor(TypedDict):
    tag: Literal["BadFileDescriptor"]
    error: str

class WorkspaceFdWriteErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class WorkspaceFdWriteErrorNotInWriteMode(TypedDict):
    tag: Literal["NotInWriteMode"]
    error: str

WorkspaceFdWriteError = (
    WorkspaceFdWriteErrorBadFileDescriptor
    | WorkspaceFdWriteErrorInternal
    | WorkspaceFdWriteErrorNotInWriteMode
)

# WorkspaceGeneratePathAddrError
class WorkspaceGeneratePathAddrErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class WorkspaceGeneratePathAddrErrorInvalidKeysBundle(TypedDict):
    tag: Literal["InvalidKeysBundle"]
    error: str

class WorkspaceGeneratePathAddrErrorNoKey(TypedDict):
    tag: Literal["NoKey"]
    error: str

class WorkspaceGeneratePathAddrErrorNotAllowed(TypedDict):
    tag: Literal["NotAllowed"]
    error: str

class WorkspaceGeneratePathAddrErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

class WorkspaceGeneratePathAddrErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

WorkspaceGeneratePathAddrError = (
    WorkspaceGeneratePathAddrErrorInternal
    | WorkspaceGeneratePathAddrErrorInvalidKeysBundle
    | WorkspaceGeneratePathAddrErrorNoKey
    | WorkspaceGeneratePathAddrErrorNotAllowed
    | WorkspaceGeneratePathAddrErrorOffline
    | WorkspaceGeneratePathAddrErrorStopped
)

# WorkspaceInfoError
class WorkspaceInfoErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

WorkspaceInfoError = WorkspaceInfoErrorInternal

# WorkspaceMountError
class WorkspaceMountErrorDisabled(TypedDict):
    tag: Literal["Disabled"]
    error: str

class WorkspaceMountErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

WorkspaceMountError = WorkspaceMountErrorDisabled | WorkspaceMountErrorInternal

# WorkspaceMoveEntryError
class WorkspaceMoveEntryErrorCannotMoveRoot(TypedDict):
    tag: Literal["CannotMoveRoot"]
    error: str

class WorkspaceMoveEntryErrorDestinationExists(TypedDict):
    tag: Literal["DestinationExists"]
    error: str

class WorkspaceMoveEntryErrorDestinationNotFound(TypedDict):
    tag: Literal["DestinationNotFound"]
    error: str

class WorkspaceMoveEntryErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class WorkspaceMoveEntryErrorInvalidCertificate(TypedDict):
    tag: Literal["InvalidCertificate"]
    error: str

class WorkspaceMoveEntryErrorInvalidKeysBundle(TypedDict):
    tag: Literal["InvalidKeysBundle"]
    error: str

class WorkspaceMoveEntryErrorInvalidManifest(TypedDict):
    tag: Literal["InvalidManifest"]
    error: str

class WorkspaceMoveEntryErrorNoRealmAccess(TypedDict):
    tag: Literal["NoRealmAccess"]
    error: str

class WorkspaceMoveEntryErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

class WorkspaceMoveEntryErrorReadOnlyRealm(TypedDict):
    tag: Literal["ReadOnlyRealm"]
    error: str

class WorkspaceMoveEntryErrorSourceNotFound(TypedDict):
    tag: Literal["SourceNotFound"]
    error: str

class WorkspaceMoveEntryErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

WorkspaceMoveEntryError = (
    WorkspaceMoveEntryErrorCannotMoveRoot
    | WorkspaceMoveEntryErrorDestinationExists
    | WorkspaceMoveEntryErrorDestinationNotFound
    | WorkspaceMoveEntryErrorInternal
    | WorkspaceMoveEntryErrorInvalidCertificate
    | WorkspaceMoveEntryErrorInvalidKeysBundle
    | WorkspaceMoveEntryErrorInvalidManifest
    | WorkspaceMoveEntryErrorNoRealmAccess
    | WorkspaceMoveEntryErrorOffline
    | WorkspaceMoveEntryErrorReadOnlyRealm
    | WorkspaceMoveEntryErrorSourceNotFound
    | WorkspaceMoveEntryErrorStopped
)

# WorkspaceOpenFileError
class WorkspaceOpenFileErrorEntryExistsInCreateNewMode(TypedDict):
    tag: Literal["EntryExistsInCreateNewMode"]
    error: str

class WorkspaceOpenFileErrorEntryNotAFile(TypedDict):
    tag: Literal["EntryNotAFile"]
    error: str

class WorkspaceOpenFileErrorEntryNotFound(TypedDict):
    tag: Literal["EntryNotFound"]
    error: str

class WorkspaceOpenFileErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class WorkspaceOpenFileErrorInvalidCertificate(TypedDict):
    tag: Literal["InvalidCertificate"]
    error: str

class WorkspaceOpenFileErrorInvalidKeysBundle(TypedDict):
    tag: Literal["InvalidKeysBundle"]
    error: str

class WorkspaceOpenFileErrorInvalidManifest(TypedDict):
    tag: Literal["InvalidManifest"]
    error: str

class WorkspaceOpenFileErrorNoRealmAccess(TypedDict):
    tag: Literal["NoRealmAccess"]
    error: str

class WorkspaceOpenFileErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

class WorkspaceOpenFileErrorReadOnlyRealm(TypedDict):
    tag: Literal["ReadOnlyRealm"]
    error: str

class WorkspaceOpenFileErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

WorkspaceOpenFileError = (
    WorkspaceOpenFileErrorEntryExistsInCreateNewMode
    | WorkspaceOpenFileErrorEntryNotAFile
    | WorkspaceOpenFileErrorEntryNotFound
    | WorkspaceOpenFileErrorInternal
    | WorkspaceOpenFileErrorInvalidCertificate
    | WorkspaceOpenFileErrorInvalidKeysBundle
    | WorkspaceOpenFileErrorInvalidManifest
    | WorkspaceOpenFileErrorNoRealmAccess
    | WorkspaceOpenFileErrorOffline
    | WorkspaceOpenFileErrorReadOnlyRealm
    | WorkspaceOpenFileErrorStopped
)

# WorkspaceRemoveEntryError
class WorkspaceRemoveEntryErrorCannotRemoveRoot(TypedDict):
    tag: Literal["CannotRemoveRoot"]
    error: str

class WorkspaceRemoveEntryErrorEntryIsFile(TypedDict):
    tag: Literal["EntryIsFile"]
    error: str

class WorkspaceRemoveEntryErrorEntryIsFolder(TypedDict):
    tag: Literal["EntryIsFolder"]
    error: str

class WorkspaceRemoveEntryErrorEntryIsNonEmptyFolder(TypedDict):
    tag: Literal["EntryIsNonEmptyFolder"]
    error: str

class WorkspaceRemoveEntryErrorEntryNotFound(TypedDict):
    tag: Literal["EntryNotFound"]
    error: str

class WorkspaceRemoveEntryErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class WorkspaceRemoveEntryErrorInvalidCertificate(TypedDict):
    tag: Literal["InvalidCertificate"]
    error: str

class WorkspaceRemoveEntryErrorInvalidKeysBundle(TypedDict):
    tag: Literal["InvalidKeysBundle"]
    error: str

class WorkspaceRemoveEntryErrorInvalidManifest(TypedDict):
    tag: Literal["InvalidManifest"]
    error: str

class WorkspaceRemoveEntryErrorNoRealmAccess(TypedDict):
    tag: Literal["NoRealmAccess"]
    error: str

class WorkspaceRemoveEntryErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

class WorkspaceRemoveEntryErrorReadOnlyRealm(TypedDict):
    tag: Literal["ReadOnlyRealm"]
    error: str

class WorkspaceRemoveEntryErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

WorkspaceRemoveEntryError = (
    WorkspaceRemoveEntryErrorCannotRemoveRoot
    | WorkspaceRemoveEntryErrorEntryIsFile
    | WorkspaceRemoveEntryErrorEntryIsFolder
    | WorkspaceRemoveEntryErrorEntryIsNonEmptyFolder
    | WorkspaceRemoveEntryErrorEntryNotFound
    | WorkspaceRemoveEntryErrorInternal
    | WorkspaceRemoveEntryErrorInvalidCertificate
    | WorkspaceRemoveEntryErrorInvalidKeysBundle
    | WorkspaceRemoveEntryErrorInvalidManifest
    | WorkspaceRemoveEntryErrorNoRealmAccess
    | WorkspaceRemoveEntryErrorOffline
    | WorkspaceRemoveEntryErrorReadOnlyRealm
    | WorkspaceRemoveEntryErrorStopped
)

# WorkspaceStatEntryError
class WorkspaceStatEntryErrorEntryNotFound(TypedDict):
    tag: Literal["EntryNotFound"]
    error: str

class WorkspaceStatEntryErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class WorkspaceStatEntryErrorInvalidCertificate(TypedDict):
    tag: Literal["InvalidCertificate"]
    error: str

class WorkspaceStatEntryErrorInvalidKeysBundle(TypedDict):
    tag: Literal["InvalidKeysBundle"]
    error: str

class WorkspaceStatEntryErrorInvalidManifest(TypedDict):
    tag: Literal["InvalidManifest"]
    error: str

class WorkspaceStatEntryErrorNoRealmAccess(TypedDict):
    tag: Literal["NoRealmAccess"]
    error: str

class WorkspaceStatEntryErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

class WorkspaceStatEntryErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

WorkspaceStatEntryError = (
    WorkspaceStatEntryErrorEntryNotFound
    | WorkspaceStatEntryErrorInternal
    | WorkspaceStatEntryErrorInvalidCertificate
    | WorkspaceStatEntryErrorInvalidKeysBundle
    | WorkspaceStatEntryErrorInvalidManifest
    | WorkspaceStatEntryErrorNoRealmAccess
    | WorkspaceStatEntryErrorOffline
    | WorkspaceStatEntryErrorStopped
)

# WorkspaceStatFolderChildrenError
class WorkspaceStatFolderChildrenErrorEntryIsFile(TypedDict):
    tag: Literal["EntryIsFile"]
    error: str

class WorkspaceStatFolderChildrenErrorEntryNotFound(TypedDict):
    tag: Literal["EntryNotFound"]
    error: str

class WorkspaceStatFolderChildrenErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

class WorkspaceStatFolderChildrenErrorInvalidCertificate(TypedDict):
    tag: Literal["InvalidCertificate"]
    error: str

class WorkspaceStatFolderChildrenErrorInvalidKeysBundle(TypedDict):
    tag: Literal["InvalidKeysBundle"]
    error: str

class WorkspaceStatFolderChildrenErrorInvalidManifest(TypedDict):
    tag: Literal["InvalidManifest"]
    error: str

class WorkspaceStatFolderChildrenErrorNoRealmAccess(TypedDict):
    tag: Literal["NoRealmAccess"]
    error: str

class WorkspaceStatFolderChildrenErrorOffline(TypedDict):
    tag: Literal["Offline"]
    error: str

class WorkspaceStatFolderChildrenErrorStopped(TypedDict):
    tag: Literal["Stopped"]
    error: str

WorkspaceStatFolderChildrenError = (
    WorkspaceStatFolderChildrenErrorEntryIsFile
    | WorkspaceStatFolderChildrenErrorEntryNotFound
    | WorkspaceStatFolderChildrenErrorInternal
    | WorkspaceStatFolderChildrenErrorInvalidCertificate
    | WorkspaceStatFolderChildrenErrorInvalidKeysBundle
    | WorkspaceStatFolderChildrenErrorInvalidManifest
    | WorkspaceStatFolderChildrenErrorNoRealmAccess
    | WorkspaceStatFolderChildrenErrorOffline
    | WorkspaceStatFolderChildrenErrorStopped
)

# WorkspaceStopError
class WorkspaceStopErrorInternal(TypedDict):
    tag: Literal["Internal"]
    error: str

WorkspaceStopError = WorkspaceStopErrorInternal

# WorkspaceStorageCacheSize
class WorkspaceStorageCacheSizeCustom(TypedDict):
    tag: Literal["Custom"]
    size: CacheSize

class WorkspaceStorageCacheSizeDefault(TypedDict):
    tag: Literal["Default"]

WorkspaceStorageCacheSize = WorkspaceStorageCacheSizeCustom | WorkspaceStorageCacheSizeDefault

async def bootstrap_organization(
    config: ClientConfig,
    on_event_callback: Callable[[ClientEvent], None],
    bootstrap_organization_addr: ParsecOrganizationBootstrapAddr,
    save_strategy: DeviceSaveStrategy,
    human_handle: HumanHandle,
    device_label: DeviceLabel,
    sequester_authority_verify_key: SequesterVerifyKeyDer | None,
) -> AvailableDevice | BootstrapOrganizationError: ...
def build_parsec_organization_bootstrap_addr(
    addr: ParsecAddr, organization_id: OrganizationID
) -> ParsecOrganizationBootstrapAddr: ...
def cancel(canceller: Handle) -> None | CancelError: ...
async def claimer_device_finalize_save_local_device(
    handle: Handle, save_strategy: DeviceSaveStrategy
) -> AvailableDevice | ClaimInProgressError: ...
async def claimer_device_in_progress_1_do_signify_trust(
    canceller: Handle, handle: Handle
) -> DeviceClaimInProgress2Info | ClaimInProgressError: ...
async def claimer_device_in_progress_2_do_wait_peer_trust(
    canceller: Handle, handle: Handle
) -> DeviceClaimInProgress3Info | ClaimInProgressError: ...
async def claimer_device_in_progress_3_do_claim(
    canceller: Handle, handle: Handle, requested_device_label: DeviceLabel
) -> DeviceClaimFinalizeInfo | ClaimInProgressError: ...
async def claimer_device_initial_do_wait_peer(
    canceller: Handle, handle: Handle
) -> DeviceClaimInProgress1Info | ClaimInProgressError: ...
def claimer_greeter_abort_operation(handle: Handle) -> None | ClaimerGreeterAbortOperationError: ...
async def claimer_retrieve_info(
    config: ClientConfig,
    on_event_callback: Callable[[ClientEvent], None],
    addr: ParsecInvitationAddr,
) -> UserOrDeviceClaimInitialInfo | ClaimerRetrieveInfoError: ...
async def claimer_user_finalize_save_local_device(
    handle: Handle, save_strategy: DeviceSaveStrategy
) -> AvailableDevice | ClaimInProgressError: ...
async def claimer_user_in_progress_1_do_signify_trust(
    canceller: Handle, handle: Handle
) -> UserClaimInProgress2Info | ClaimInProgressError: ...
async def claimer_user_in_progress_2_do_wait_peer_trust(
    canceller: Handle, handle: Handle
) -> UserClaimInProgress3Info | ClaimInProgressError: ...
async def claimer_user_in_progress_3_do_claim(
    canceller: Handle,
    handle: Handle,
    requested_device_label: DeviceLabel,
    requested_human_handle: HumanHandle,
) -> UserClaimFinalizeInfo | ClaimInProgressError: ...
async def claimer_user_initial_do_wait_peer(
    canceller: Handle, handle: Handle
) -> UserClaimInProgress1Info | ClaimInProgressError: ...
async def client_cancel_invitation(
    client: Handle, token: InvitationToken
) -> None | ClientCancelInvitationError: ...
async def client_change_authentication(
    client_config: ClientConfig, current_auth: DeviceAccessStrategy, new_auth: DeviceSaveStrategy
) -> None | ClientChangeAuthenticationError: ...
async def client_create_workspace(
    client: Handle, name: EntryName
) -> VlobID | ClientCreateWorkspaceError: ...
async def client_get_user_device(
    client: Handle, device: DeviceID
) -> tuple[UserInfo, DeviceInfo] | ClientGetUserDeviceError: ...
async def client_info(client: Handle) -> ClientInfo | ClientInfoError: ...
async def client_list_invitations(
    client: Handle,
) -> list[InviteListItem] | ListInvitationsError: ...
async def client_list_user_devices(
    client: Handle, user: UserID
) -> list[DeviceInfo] | ClientListUserDevicesError: ...
async def client_list_users(
    client: Handle, skip_revoked: bool
) -> list[UserInfo] | ClientListUsersError: ...
async def client_list_workspace_users(
    client: Handle, realm_id: VlobID
) -> list[WorkspaceUserAccessInfo] | ClientListWorkspaceUsersError: ...
async def client_list_workspaces(
    client: Handle,
) -> list[WorkspaceInfo] | ClientListWorkspacesError: ...
async def client_new_device_invitation(
    client: Handle, send_email: bool
) -> NewInvitationInfo | ClientNewDeviceInvitationError: ...
async def client_new_user_invitation(
    client: Handle, claimer_email: str, send_email: bool
) -> NewInvitationInfo | ClientNewUserInvitationError: ...
async def client_rename_workspace(
    client: Handle, realm_id: VlobID, new_name: EntryName
) -> None | ClientRenameWorkspaceError: ...
async def client_revoke_user(client: Handle, user: UserID) -> None | ClientRevokeUserError: ...
async def client_share_workspace(
    client: Handle, realm_id: VlobID, recipient: UserID, role: RealmRole | None
) -> None | ClientShareWorkspaceError: ...
async def client_start(
    config: ClientConfig,
    on_event_callback: Callable[[ClientEvent], None],
    access: DeviceAccessStrategy,
) -> Handle | ClientStartError: ...
async def client_start_device_invitation_greet(
    client: Handle, token: InvitationToken
) -> DeviceGreetInitialInfo | ClientStartInvitationGreetError: ...
async def client_start_user_invitation_greet(
    client: Handle, token: InvitationToken
) -> UserGreetInitialInfo | ClientStartInvitationGreetError: ...
async def client_start_workspace(
    client: Handle, realm_id: VlobID
) -> Handle | ClientStartWorkspaceError: ...
async def client_stop(client: Handle) -> None | ClientStopError: ...
async def fd_close(workspace: Handle, fd: FileDescriptor) -> None | WorkspaceFdCloseError: ...
async def fd_flush(workspace: Handle, fd: FileDescriptor) -> None | WorkspaceFdFlushError: ...
async def fd_read(
    workspace: Handle, fd: FileDescriptor, offset: U64, size: U64
) -> bytes | WorkspaceFdReadError: ...
async def fd_resize(
    workspace: Handle, fd: FileDescriptor, length: U64, truncate_only: bool
) -> None | WorkspaceFdResizeError: ...
async def fd_write(
    workspace: Handle, fd: FileDescriptor, offset: U64, data: bytes
) -> U64 | WorkspaceFdWriteError: ...
async def fd_write_constrained_io(
    workspace: Handle, fd: FileDescriptor, offset: U64, data: bytes
) -> U64 | WorkspaceFdWriteError: ...
async def fd_write_start_eof(
    workspace: Handle, fd: FileDescriptor, data: bytes
) -> U64 | WorkspaceFdWriteError: ...
def get_default_config_dir() -> Path: ...
def get_default_data_base_dir() -> Path: ...
def get_default_mountpoint_base_dir() -> Path: ...
def get_platform() -> Platform: ...
async def greeter_device_in_progress_1_do_wait_peer_trust(
    canceller: Handle, handle: Handle
) -> DeviceGreetInProgress2Info | GreetInProgressError: ...
async def greeter_device_in_progress_2_do_signify_trust(
    canceller: Handle, handle: Handle
) -> DeviceGreetInProgress3Info | GreetInProgressError: ...
async def greeter_device_in_progress_3_do_get_claim_requests(
    canceller: Handle, handle: Handle
) -> DeviceGreetInProgress4Info | GreetInProgressError: ...
async def greeter_device_in_progress_4_do_create(
    canceller: Handle, handle: Handle, device_label: DeviceLabel
) -> None | GreetInProgressError: ...
async def greeter_device_initial_do_wait_peer(
    canceller: Handle, handle: Handle
) -> DeviceGreetInProgress1Info | GreetInProgressError: ...
async def greeter_user_in_progress_1_do_wait_peer_trust(
    canceller: Handle, handle: Handle
) -> UserGreetInProgress2Info | GreetInProgressError: ...
async def greeter_user_in_progress_2_do_signify_trust(
    canceller: Handle, handle: Handle
) -> UserGreetInProgress3Info | GreetInProgressError: ...
async def greeter_user_in_progress_3_do_get_claim_requests(
    canceller: Handle, handle: Handle
) -> UserGreetInProgress4Info | GreetInProgressError: ...
async def greeter_user_in_progress_4_do_create(
    canceller: Handle,
    handle: Handle,
    human_handle: HumanHandle,
    device_label: DeviceLabel,
    profile: UserProfile,
) -> None | GreetInProgressError: ...
async def greeter_user_initial_do_wait_peer(
    canceller: Handle, handle: Handle
) -> UserGreetInProgress1Info | GreetInProgressError: ...
def is_keyring_available() -> bool: ...
async def list_available_devices(path: Path) -> list[AvailableDevice]: ...
async def mountpoint_to_os_path(
    mountpoint: Handle, parsec_path: FsPath
) -> Path | MountpointToOsPathError: ...
async def mountpoint_unmount(mountpoint: Handle) -> None | MountpointUnmountError: ...
def new_canceller() -> Handle: ...
def parse_parsec_addr(url: str) -> ParsedParsecAddr | ParseParsecAddrError: ...
def path_filename(path: FsPath) -> EntryName | None: ...
def path_join(parent: FsPath, child: EntryName) -> FsPath: ...
def path_normalize(path: FsPath) -> FsPath: ...
def path_parent(path: FsPath) -> FsPath: ...
def path_split(path: FsPath) -> list[EntryName]: ...
async def test_drop_testbed(path: Path) -> None | TestbedError: ...
def test_get_testbed_bootstrap_organization_addr(
    discriminant_dir: Path,
) -> ParsecOrganizationBootstrapAddr | None | TestbedError: ...
def test_get_testbed_organization_id(
    discriminant_dir: Path,
) -> OrganizationID | None | TestbedError: ...
async def test_new_testbed(
    template: str, test_server: ParsecAddr | None
) -> Path | TestbedError: ...
def validate_device_label(raw: str) -> bool: ...
def validate_email(raw: str) -> bool: ...
def validate_entry_name(raw: str) -> bool: ...
def validate_human_handle_label(raw: str) -> bool: ...
def validate_invitation_token(raw: str) -> bool: ...
def validate_organization_id(raw: str) -> bool: ...
def validate_path(raw: str) -> bool: ...
async def workspace_create_file(
    workspace: Handle, path: FsPath
) -> VlobID | WorkspaceCreateFileError: ...
async def workspace_create_folder(
    workspace: Handle, path: FsPath
) -> VlobID | WorkspaceCreateFolderError: ...
async def workspace_create_folder_all(
    workspace: Handle, path: FsPath
) -> VlobID | WorkspaceCreateFolderError: ...
async def workspace_decrypt_path_addr(
    workspace: Handle, link: ParsecWorkspacePathAddr
) -> FsPath | WorkspaceDecryptPathAddrError: ...
async def workspace_generate_path_addr(
    workspace: Handle, path: FsPath
) -> ParsecWorkspacePathAddr | WorkspaceGeneratePathAddrError: ...
async def workspace_info(workspace: Handle) -> StartedWorkspaceInfo | WorkspaceInfoError: ...
async def workspace_mount(workspace: Handle) -> tuple[Handle, Path] | WorkspaceMountError: ...
async def workspace_move_entry(
    workspace: Handle, src: FsPath, dst: FsPath, mode: MoveEntryMode
) -> None | WorkspaceMoveEntryError: ...
async def workspace_open_file(
    workspace: Handle, path: FsPath, mode: OpenOptions
) -> FileDescriptor | WorkspaceOpenFileError: ...
async def workspace_remove_entry(
    workspace: Handle, path: FsPath
) -> None | WorkspaceRemoveEntryError: ...
async def workspace_remove_file(
    workspace: Handle, path: FsPath
) -> None | WorkspaceRemoveEntryError: ...
async def workspace_remove_folder(
    workspace: Handle, path: FsPath
) -> None | WorkspaceRemoveEntryError: ...
async def workspace_remove_folder_all(
    workspace: Handle, path: FsPath
) -> None | WorkspaceRemoveEntryError: ...
async def workspace_stat_entry(
    workspace: Handle, path: FsPath
) -> EntryStat | WorkspaceStatEntryError: ...
async def workspace_stat_entry_by_id(
    workspace: Handle, entry_id: VlobID
) -> EntryStat | WorkspaceStatEntryError: ...
async def workspace_stat_folder_children(
    workspace: Handle, path: FsPath
) -> list[tuple[EntryName, EntryStat]] | WorkspaceStatFolderChildrenError: ...
async def workspace_stat_folder_children_by_id(
    workspace: Handle, entry_id: VlobID
) -> list[tuple[EntryName, EntryStat]] | WorkspaceStatFolderChildrenError: ...
async def workspace_stop(workspace: Handle) -> None | WorkspaceStopError: ...
