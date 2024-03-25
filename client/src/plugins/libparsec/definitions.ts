// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */

export type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E }

export enum DeviceFileType {
    Keyring = 'DeviceFileTypeKeyring',
    Password = 'DeviceFileTypePassword',
    Recovery = 'DeviceFileTypeRecovery',
    Smartcard = 'DeviceFileTypeSmartcard',
}

export enum InvitationEmailSentStatus {
    RecipientRefused = 'InvitationEmailSentStatusRecipientRefused',
    ServerUnavailable = 'InvitationEmailSentStatusServerUnavailable',
    Success = 'InvitationEmailSentStatusSuccess',
}

export enum InvitationStatus {
    Cancelled = 'InvitationStatusCancelled',
    Finished = 'InvitationStatusFinished',
    Idle = 'InvitationStatusIdle',
    Ready = 'InvitationStatusReady',
}

export enum Platform {
    Android = 'PlatformAndroid',
    Linux = 'PlatformLinux',
    MacOS = 'PlatformMacOS',
    Web = 'PlatformWeb',
    Windows = 'PlatformWindows',
}

export enum RealmRole {
    Contributor = 'RealmRoleContributor',
    Manager = 'RealmRoleManager',
    Owner = 'RealmRoleOwner',
    Reader = 'RealmRoleReader',
}

export enum UserProfile {
    Admin = 'UserProfileAdmin',
    Outsider = 'UserProfileOutsider',
    Standard = 'UserProfileStandard',
}
export type DeviceID = string
export type DeviceLabel = string
export type EntryName = string
export type FsPath = string
export type InvitationToken = string
export type OrganizationID = string
export type ParsecAddr = string
export type ParsecInvitationAddr = string
export type ParsecOrganizationAddr = string
export type ParsecOrganizationBootstrapAddr = string
export type ParsecOrganizationFileLinkAddr = string
export type ParsecPkiEnrollmentAddr = string
export type Password = string
export type Path = string
export type SASCode = string
export type UserID = string
export type VlobID = string
export type SequesterVerifyKeyDer = Uint8Array
export type I32 = number
export type CacheSize = number
export type FileDescriptor = number
export type Handle = number
export type U32 = number
export type VersionInt = number
export type I64 = number
export type IndexInt = number
export type SizeInt = number
export type U64 = number
export type { DateTime } from 'luxon'; import type { DateTime } from 'luxon';

export interface AvailableDevice {
    keyFilePath: Path
    organizationId: OrganizationID
    deviceId: DeviceID
    humanHandle: HumanHandle
    deviceLabel: DeviceLabel
    slug: string
    ty: DeviceFileType
}

export interface ClientConfig {
    configDir: Path
    dataBaseDir: Path
    mountpointMountStrategy: MountpointMountStrategy
    workspaceStorageCacheSize: WorkspaceStorageCacheSize
    withMonitors: boolean
}

export interface ClientInfo {
    organizationAddr: ParsecOrganizationAddr
    organizationId: OrganizationID
    deviceId: DeviceID
    userId: UserID
    deviceLabel: DeviceLabel
    humanHandle: HumanHandle
    currentProfile: UserProfile
    serverConfig: ServerConfig
}

export interface DeviceClaimFinalizeInfo {
    handle: Handle
}

export interface DeviceClaimInProgress1Info {
    handle: Handle
    greeterSas: SASCode
    greeterSasChoices: Array<SASCode>
}

export interface DeviceClaimInProgress2Info {
    handle: Handle
    claimerSas: SASCode
}

export interface DeviceClaimInProgress3Info {
    handle: Handle
}

export interface DeviceGreetInProgress1Info {
    handle: Handle
    greeterSas: SASCode
}

export interface DeviceGreetInProgress2Info {
    handle: Handle
    claimerSas: SASCode
    claimerSasChoices: Array<SASCode>
}

export interface DeviceGreetInProgress3Info {
    handle: Handle
}

export interface DeviceGreetInProgress4Info {
    handle: Handle
    requestedDeviceLabel: DeviceLabel
}

export interface DeviceGreetInitialInfo {
    handle: Handle
}

export interface DeviceInfo {
    id: DeviceID
    deviceLabel: DeviceLabel
    createdOn: DateTime
    createdBy: DeviceID | null
}

export interface HumanHandle {
    email: string
    label: string
}

export interface NewInvitationInfo {
    addr: ParsecInvitationAddr
    token: InvitationToken
    emailSentStatus: InvitationEmailSentStatus
}

export interface OpenOptions {
    read: boolean
    write: boolean
    truncate: boolean
    create: boolean
    createNew: boolean
}

export interface ServerConfig {
    userProfileOutsiderAllowed: boolean
    activeUsersLimit: ActiveUsersLimit
}

export interface StartedWorkspaceInfo {
    client: Handle
    id: VlobID
    currentName: EntryName
    currentSelfRole: RealmRole
    mountpoints: Array<[Handle, Path]>
}

export interface UserClaimFinalizeInfo {
    handle: Handle
}

export interface UserClaimInProgress1Info {
    handle: Handle
    greeterSas: SASCode
    greeterSasChoices: Array<SASCode>
}

export interface UserClaimInProgress2Info {
    handle: Handle
    claimerSas: SASCode
}

export interface UserClaimInProgress3Info {
    handle: Handle
}

export interface UserGreetInProgress1Info {
    handle: Handle
    greeterSas: SASCode
}

export interface UserGreetInProgress2Info {
    handle: Handle
    claimerSas: SASCode
    claimerSasChoices: Array<SASCode>
}

export interface UserGreetInProgress3Info {
    handle: Handle
}

export interface UserGreetInProgress4Info {
    handle: Handle
    requestedHumanHandle: HumanHandle
    requestedDeviceLabel: DeviceLabel
}

export interface UserGreetInitialInfo {
    handle: Handle
}

export interface UserInfo {
    id: UserID
    humanHandle: HumanHandle
    currentProfile: UserProfile
    createdOn: DateTime
    createdBy: DeviceID | null
    revokedOn: DateTime | null
    revokedBy: DeviceID | null
}

export interface WorkspaceInfo {
    id: VlobID
    currentName: EntryName
    currentSelfRole: RealmRole
    isStarted: boolean
    isBootstrapped: boolean
}

export interface WorkspaceUserAccessInfo {
    userId: UserID
    humanHandle: HumanHandle
    currentProfile: UserProfile
    currentRole: RealmRole
}

// ActiveUsersLimit
export enum ActiveUsersLimitTag {
    LimitedTo = 'ActiveUsersLimitLimitedTo',
    NoLimit = 'ActiveUsersLimitNoLimit',
}

export interface ActiveUsersLimitLimitedTo {
    tag: ActiveUsersLimitTag.LimitedTo
    x1: U64
}
export interface ActiveUsersLimitNoLimit {
    tag: ActiveUsersLimitTag.NoLimit
}
export type ActiveUsersLimit =
  | ActiveUsersLimitLimitedTo
  | ActiveUsersLimitNoLimit

// BootstrapOrganizationError
export enum BootstrapOrganizationErrorTag {
    AlreadyUsedToken = 'BootstrapOrganizationErrorAlreadyUsedToken',
    Internal = 'BootstrapOrganizationErrorInternal',
    InvalidToken = 'BootstrapOrganizationErrorInvalidToken',
    Offline = 'BootstrapOrganizationErrorOffline',
    OrganizationExpired = 'BootstrapOrganizationErrorOrganizationExpired',
    SaveDeviceError = 'BootstrapOrganizationErrorSaveDeviceError',
    TimestampOutOfBallpark = 'BootstrapOrganizationErrorTimestampOutOfBallpark',
}

export interface BootstrapOrganizationErrorAlreadyUsedToken {
    tag: BootstrapOrganizationErrorTag.AlreadyUsedToken
    error: string
}
export interface BootstrapOrganizationErrorInternal {
    tag: BootstrapOrganizationErrorTag.Internal
    error: string
}
export interface BootstrapOrganizationErrorInvalidToken {
    tag: BootstrapOrganizationErrorTag.InvalidToken
    error: string
}
export interface BootstrapOrganizationErrorOffline {
    tag: BootstrapOrganizationErrorTag.Offline
    error: string
}
export interface BootstrapOrganizationErrorOrganizationExpired {
    tag: BootstrapOrganizationErrorTag.OrganizationExpired
    error: string
}
export interface BootstrapOrganizationErrorSaveDeviceError {
    tag: BootstrapOrganizationErrorTag.SaveDeviceError
    error: string
}
export interface BootstrapOrganizationErrorTimestampOutOfBallpark {
    tag: BootstrapOrganizationErrorTag.TimestampOutOfBallpark
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export type BootstrapOrganizationError =
  | BootstrapOrganizationErrorAlreadyUsedToken
  | BootstrapOrganizationErrorInternal
  | BootstrapOrganizationErrorInvalidToken
  | BootstrapOrganizationErrorOffline
  | BootstrapOrganizationErrorOrganizationExpired
  | BootstrapOrganizationErrorSaveDeviceError
  | BootstrapOrganizationErrorTimestampOutOfBallpark

// CancelError
export enum CancelErrorTag {
    Internal = 'CancelErrorInternal',
    NotBound = 'CancelErrorNotBound',
}

export interface CancelErrorInternal {
    tag: CancelErrorTag.Internal
    error: string
}
export interface CancelErrorNotBound {
    tag: CancelErrorTag.NotBound
    error: string
}
export type CancelError =
  | CancelErrorInternal
  | CancelErrorNotBound

// ClaimInProgressError
export enum ClaimInProgressErrorTag {
    ActiveUsersLimitReached = 'ClaimInProgressErrorActiveUsersLimitReached',
    AlreadyUsed = 'ClaimInProgressErrorAlreadyUsed',
    Cancelled = 'ClaimInProgressErrorCancelled',
    CorruptedConfirmation = 'ClaimInProgressErrorCorruptedConfirmation',
    Internal = 'ClaimInProgressErrorInternal',
    NotFound = 'ClaimInProgressErrorNotFound',
    Offline = 'ClaimInProgressErrorOffline',
    OrganizationExpired = 'ClaimInProgressErrorOrganizationExpired',
    PeerReset = 'ClaimInProgressErrorPeerReset',
}

export interface ClaimInProgressErrorActiveUsersLimitReached {
    tag: ClaimInProgressErrorTag.ActiveUsersLimitReached
    error: string
}
export interface ClaimInProgressErrorAlreadyUsed {
    tag: ClaimInProgressErrorTag.AlreadyUsed
    error: string
}
export interface ClaimInProgressErrorCancelled {
    tag: ClaimInProgressErrorTag.Cancelled
    error: string
}
export interface ClaimInProgressErrorCorruptedConfirmation {
    tag: ClaimInProgressErrorTag.CorruptedConfirmation
    error: string
}
export interface ClaimInProgressErrorInternal {
    tag: ClaimInProgressErrorTag.Internal
    error: string
}
export interface ClaimInProgressErrorNotFound {
    tag: ClaimInProgressErrorTag.NotFound
    error: string
}
export interface ClaimInProgressErrorOffline {
    tag: ClaimInProgressErrorTag.Offline
    error: string
}
export interface ClaimInProgressErrorOrganizationExpired {
    tag: ClaimInProgressErrorTag.OrganizationExpired
    error: string
}
export interface ClaimInProgressErrorPeerReset {
    tag: ClaimInProgressErrorTag.PeerReset
    error: string
}
export type ClaimInProgressError =
  | ClaimInProgressErrorActiveUsersLimitReached
  | ClaimInProgressErrorAlreadyUsed
  | ClaimInProgressErrorCancelled
  | ClaimInProgressErrorCorruptedConfirmation
  | ClaimInProgressErrorInternal
  | ClaimInProgressErrorNotFound
  | ClaimInProgressErrorOffline
  | ClaimInProgressErrorOrganizationExpired
  | ClaimInProgressErrorPeerReset

// ClaimerGreeterAbortOperationError
export enum ClaimerGreeterAbortOperationErrorTag {
    Internal = 'ClaimerGreeterAbortOperationErrorInternal',
}

export interface ClaimerGreeterAbortOperationErrorInternal {
    tag: ClaimerGreeterAbortOperationErrorTag.Internal
    error: string
}
export type ClaimerGreeterAbortOperationError =
  | ClaimerGreeterAbortOperationErrorInternal

// ClaimerRetrieveInfoError
export enum ClaimerRetrieveInfoErrorTag {
    AlreadyUsed = 'ClaimerRetrieveInfoErrorAlreadyUsed',
    Internal = 'ClaimerRetrieveInfoErrorInternal',
    NotFound = 'ClaimerRetrieveInfoErrorNotFound',
    Offline = 'ClaimerRetrieveInfoErrorOffline',
}

export interface ClaimerRetrieveInfoErrorAlreadyUsed {
    tag: ClaimerRetrieveInfoErrorTag.AlreadyUsed
    error: string
}
export interface ClaimerRetrieveInfoErrorInternal {
    tag: ClaimerRetrieveInfoErrorTag.Internal
    error: string
}
export interface ClaimerRetrieveInfoErrorNotFound {
    tag: ClaimerRetrieveInfoErrorTag.NotFound
    error: string
}
export interface ClaimerRetrieveInfoErrorOffline {
    tag: ClaimerRetrieveInfoErrorTag.Offline
    error: string
}
export type ClaimerRetrieveInfoError =
  | ClaimerRetrieveInfoErrorAlreadyUsed
  | ClaimerRetrieveInfoErrorInternal
  | ClaimerRetrieveInfoErrorNotFound
  | ClaimerRetrieveInfoErrorOffline

// ClientCancelInvitationError
export enum ClientCancelInvitationErrorTag {
    AlreadyDeleted = 'ClientCancelInvitationErrorAlreadyDeleted',
    Internal = 'ClientCancelInvitationErrorInternal',
    NotFound = 'ClientCancelInvitationErrorNotFound',
    Offline = 'ClientCancelInvitationErrorOffline',
}

export interface ClientCancelInvitationErrorAlreadyDeleted {
    tag: ClientCancelInvitationErrorTag.AlreadyDeleted
    error: string
}
export interface ClientCancelInvitationErrorInternal {
    tag: ClientCancelInvitationErrorTag.Internal
    error: string
}
export interface ClientCancelInvitationErrorNotFound {
    tag: ClientCancelInvitationErrorTag.NotFound
    error: string
}
export interface ClientCancelInvitationErrorOffline {
    tag: ClientCancelInvitationErrorTag.Offline
    error: string
}
export type ClientCancelInvitationError =
  | ClientCancelInvitationErrorAlreadyDeleted
  | ClientCancelInvitationErrorInternal
  | ClientCancelInvitationErrorNotFound
  | ClientCancelInvitationErrorOffline

// ClientChangeAuthenticationError
export enum ClientChangeAuthenticationErrorTag {
    DecryptionFailed = 'ClientChangeAuthenticationErrorDecryptionFailed',
    Internal = 'ClientChangeAuthenticationErrorInternal',
    InvalidData = 'ClientChangeAuthenticationErrorInvalidData',
    InvalidPath = 'ClientChangeAuthenticationErrorInvalidPath',
}

export interface ClientChangeAuthenticationErrorDecryptionFailed {
    tag: ClientChangeAuthenticationErrorTag.DecryptionFailed
    error: string
}
export interface ClientChangeAuthenticationErrorInternal {
    tag: ClientChangeAuthenticationErrorTag.Internal
    error: string
}
export interface ClientChangeAuthenticationErrorInvalidData {
    tag: ClientChangeAuthenticationErrorTag.InvalidData
    error: string
}
export interface ClientChangeAuthenticationErrorInvalidPath {
    tag: ClientChangeAuthenticationErrorTag.InvalidPath
    error: string
}
export type ClientChangeAuthenticationError =
  | ClientChangeAuthenticationErrorDecryptionFailed
  | ClientChangeAuthenticationErrorInternal
  | ClientChangeAuthenticationErrorInvalidData
  | ClientChangeAuthenticationErrorInvalidPath

// ClientCreateWorkspaceError
export enum ClientCreateWorkspaceErrorTag {
    Internal = 'ClientCreateWorkspaceErrorInternal',
    Stopped = 'ClientCreateWorkspaceErrorStopped',
}

export interface ClientCreateWorkspaceErrorInternal {
    tag: ClientCreateWorkspaceErrorTag.Internal
    error: string
}
export interface ClientCreateWorkspaceErrorStopped {
    tag: ClientCreateWorkspaceErrorTag.Stopped
    error: string
}
export type ClientCreateWorkspaceError =
  | ClientCreateWorkspaceErrorInternal
  | ClientCreateWorkspaceErrorStopped

// ClientEvent
export enum ClientEventTag {
    IncompatibleServer = 'ClientEventIncompatibleServer',
    InvitationChanged = 'ClientEventInvitationChanged',
    Offline = 'ClientEventOffline',
    Online = 'ClientEventOnline',
    Ping = 'ClientEventPing',
    ServerConfigChanged = 'ClientEventServerConfigChanged',
    TooMuchDriftWithServerClock = 'ClientEventTooMuchDriftWithServerClock',
}

export interface ClientEventIncompatibleServer {
    tag: ClientEventTag.IncompatibleServer
    detail: string
}
export interface ClientEventInvitationChanged {
    tag: ClientEventTag.InvitationChanged
    token: InvitationToken
    status: InvitationStatus
}
export interface ClientEventOffline {
    tag: ClientEventTag.Offline
}
export interface ClientEventOnline {
    tag: ClientEventTag.Online
}
export interface ClientEventPing {
    tag: ClientEventTag.Ping
    ping: string
}
export interface ClientEventServerConfigChanged {
    tag: ClientEventTag.ServerConfigChanged
}
export interface ClientEventTooMuchDriftWithServerClock {
    tag: ClientEventTag.TooMuchDriftWithServerClock
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export type ClientEvent =
  | ClientEventIncompatibleServer
  | ClientEventInvitationChanged
  | ClientEventOffline
  | ClientEventOnline
  | ClientEventPing
  | ClientEventServerConfigChanged
  | ClientEventTooMuchDriftWithServerClock

// ClientGetUserDeviceError
export enum ClientGetUserDeviceErrorTag {
    Internal = 'ClientGetUserDeviceErrorInternal',
    NonExisting = 'ClientGetUserDeviceErrorNonExisting',
    Stopped = 'ClientGetUserDeviceErrorStopped',
}

export interface ClientGetUserDeviceErrorInternal {
    tag: ClientGetUserDeviceErrorTag.Internal
    error: string
}
export interface ClientGetUserDeviceErrorNonExisting {
    tag: ClientGetUserDeviceErrorTag.NonExisting
    error: string
}
export interface ClientGetUserDeviceErrorStopped {
    tag: ClientGetUserDeviceErrorTag.Stopped
    error: string
}
export type ClientGetUserDeviceError =
  | ClientGetUserDeviceErrorInternal
  | ClientGetUserDeviceErrorNonExisting
  | ClientGetUserDeviceErrorStopped

// ClientInfoError
export enum ClientInfoErrorTag {
    Internal = 'ClientInfoErrorInternal',
    Stopped = 'ClientInfoErrorStopped',
}

export interface ClientInfoErrorInternal {
    tag: ClientInfoErrorTag.Internal
    error: string
}
export interface ClientInfoErrorStopped {
    tag: ClientInfoErrorTag.Stopped
    error: string
}
export type ClientInfoError =
  | ClientInfoErrorInternal
  | ClientInfoErrorStopped

// ClientListUserDevicesError
export enum ClientListUserDevicesErrorTag {
    Internal = 'ClientListUserDevicesErrorInternal',
    Stopped = 'ClientListUserDevicesErrorStopped',
}

export interface ClientListUserDevicesErrorInternal {
    tag: ClientListUserDevicesErrorTag.Internal
    error: string
}
export interface ClientListUserDevicesErrorStopped {
    tag: ClientListUserDevicesErrorTag.Stopped
    error: string
}
export type ClientListUserDevicesError =
  | ClientListUserDevicesErrorInternal
  | ClientListUserDevicesErrorStopped

// ClientListUsersError
export enum ClientListUsersErrorTag {
    Internal = 'ClientListUsersErrorInternal',
    Stopped = 'ClientListUsersErrorStopped',
}

export interface ClientListUsersErrorInternal {
    tag: ClientListUsersErrorTag.Internal
    error: string
}
export interface ClientListUsersErrorStopped {
    tag: ClientListUsersErrorTag.Stopped
    error: string
}
export type ClientListUsersError =
  | ClientListUsersErrorInternal
  | ClientListUsersErrorStopped

// ClientListWorkspaceUsersError
export enum ClientListWorkspaceUsersErrorTag {
    Internal = 'ClientListWorkspaceUsersErrorInternal',
    Stopped = 'ClientListWorkspaceUsersErrorStopped',
}

export interface ClientListWorkspaceUsersErrorInternal {
    tag: ClientListWorkspaceUsersErrorTag.Internal
    error: string
}
export interface ClientListWorkspaceUsersErrorStopped {
    tag: ClientListWorkspaceUsersErrorTag.Stopped
    error: string
}
export type ClientListWorkspaceUsersError =
  | ClientListWorkspaceUsersErrorInternal
  | ClientListWorkspaceUsersErrorStopped

// ClientListWorkspacesError
export enum ClientListWorkspacesErrorTag {
    Internal = 'ClientListWorkspacesErrorInternal',
}

export interface ClientListWorkspacesErrorInternal {
    tag: ClientListWorkspacesErrorTag.Internal
    error: string
}
export type ClientListWorkspacesError =
  | ClientListWorkspacesErrorInternal

// ClientNewDeviceInvitationError
export enum ClientNewDeviceInvitationErrorTag {
    Internal = 'ClientNewDeviceInvitationErrorInternal',
    Offline = 'ClientNewDeviceInvitationErrorOffline',
}

export interface ClientNewDeviceInvitationErrorInternal {
    tag: ClientNewDeviceInvitationErrorTag.Internal
    error: string
}
export interface ClientNewDeviceInvitationErrorOffline {
    tag: ClientNewDeviceInvitationErrorTag.Offline
    error: string
}
export type ClientNewDeviceInvitationError =
  | ClientNewDeviceInvitationErrorInternal
  | ClientNewDeviceInvitationErrorOffline

// ClientNewUserInvitationError
export enum ClientNewUserInvitationErrorTag {
    AlreadyMember = 'ClientNewUserInvitationErrorAlreadyMember',
    Internal = 'ClientNewUserInvitationErrorInternal',
    NotAllowed = 'ClientNewUserInvitationErrorNotAllowed',
    Offline = 'ClientNewUserInvitationErrorOffline',
}

export interface ClientNewUserInvitationErrorAlreadyMember {
    tag: ClientNewUserInvitationErrorTag.AlreadyMember
    error: string
}
export interface ClientNewUserInvitationErrorInternal {
    tag: ClientNewUserInvitationErrorTag.Internal
    error: string
}
export interface ClientNewUserInvitationErrorNotAllowed {
    tag: ClientNewUserInvitationErrorTag.NotAllowed
    error: string
}
export interface ClientNewUserInvitationErrorOffline {
    tag: ClientNewUserInvitationErrorTag.Offline
    error: string
}
export type ClientNewUserInvitationError =
  | ClientNewUserInvitationErrorAlreadyMember
  | ClientNewUserInvitationErrorInternal
  | ClientNewUserInvitationErrorNotAllowed
  | ClientNewUserInvitationErrorOffline

// ClientRenameWorkspaceError
export enum ClientRenameWorkspaceErrorTag {
    AuthorNotAllowed = 'ClientRenameWorkspaceErrorAuthorNotAllowed',
    Internal = 'ClientRenameWorkspaceErrorInternal',
    InvalidCertificate = 'ClientRenameWorkspaceErrorInvalidCertificate',
    InvalidEncryptedRealmName = 'ClientRenameWorkspaceErrorInvalidEncryptedRealmName',
    InvalidKeysBundle = 'ClientRenameWorkspaceErrorInvalidKeysBundle',
    NoKey = 'ClientRenameWorkspaceErrorNoKey',
    Offline = 'ClientRenameWorkspaceErrorOffline',
    Stopped = 'ClientRenameWorkspaceErrorStopped',
    TimestampOutOfBallpark = 'ClientRenameWorkspaceErrorTimestampOutOfBallpark',
    WorkspaceNotFound = 'ClientRenameWorkspaceErrorWorkspaceNotFound',
}

export interface ClientRenameWorkspaceErrorAuthorNotAllowed {
    tag: ClientRenameWorkspaceErrorTag.AuthorNotAllowed
    error: string
}
export interface ClientRenameWorkspaceErrorInternal {
    tag: ClientRenameWorkspaceErrorTag.Internal
    error: string
}
export interface ClientRenameWorkspaceErrorInvalidCertificate {
    tag: ClientRenameWorkspaceErrorTag.InvalidCertificate
    error: string
}
export interface ClientRenameWorkspaceErrorInvalidEncryptedRealmName {
    tag: ClientRenameWorkspaceErrorTag.InvalidEncryptedRealmName
    error: string
}
export interface ClientRenameWorkspaceErrorInvalidKeysBundle {
    tag: ClientRenameWorkspaceErrorTag.InvalidKeysBundle
    error: string
}
export interface ClientRenameWorkspaceErrorNoKey {
    tag: ClientRenameWorkspaceErrorTag.NoKey
    error: string
}
export interface ClientRenameWorkspaceErrorOffline {
    tag: ClientRenameWorkspaceErrorTag.Offline
    error: string
}
export interface ClientRenameWorkspaceErrorStopped {
    tag: ClientRenameWorkspaceErrorTag.Stopped
    error: string
}
export interface ClientRenameWorkspaceErrorTimestampOutOfBallpark {
    tag: ClientRenameWorkspaceErrorTag.TimestampOutOfBallpark
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export interface ClientRenameWorkspaceErrorWorkspaceNotFound {
    tag: ClientRenameWorkspaceErrorTag.WorkspaceNotFound
    error: string
}
export type ClientRenameWorkspaceError =
  | ClientRenameWorkspaceErrorAuthorNotAllowed
  | ClientRenameWorkspaceErrorInternal
  | ClientRenameWorkspaceErrorInvalidCertificate
  | ClientRenameWorkspaceErrorInvalidEncryptedRealmName
  | ClientRenameWorkspaceErrorInvalidKeysBundle
  | ClientRenameWorkspaceErrorNoKey
  | ClientRenameWorkspaceErrorOffline
  | ClientRenameWorkspaceErrorStopped
  | ClientRenameWorkspaceErrorTimestampOutOfBallpark
  | ClientRenameWorkspaceErrorWorkspaceNotFound

// ClientRevokeUserError
export enum ClientRevokeUserErrorTag {
    AuthorNotAllowed = 'ClientRevokeUserErrorAuthorNotAllowed',
    Internal = 'ClientRevokeUserErrorInternal',
    InvalidCertificate = 'ClientRevokeUserErrorInvalidCertificate',
    InvalidKeysBundle = 'ClientRevokeUserErrorInvalidKeysBundle',
    NoKey = 'ClientRevokeUserErrorNoKey',
    Offline = 'ClientRevokeUserErrorOffline',
    Stopped = 'ClientRevokeUserErrorStopped',
    TimestampOutOfBallpark = 'ClientRevokeUserErrorTimestampOutOfBallpark',
    UserIsSelf = 'ClientRevokeUserErrorUserIsSelf',
    UserNotFound = 'ClientRevokeUserErrorUserNotFound',
}

export interface ClientRevokeUserErrorAuthorNotAllowed {
    tag: ClientRevokeUserErrorTag.AuthorNotAllowed
    error: string
}
export interface ClientRevokeUserErrorInternal {
    tag: ClientRevokeUserErrorTag.Internal
    error: string
}
export interface ClientRevokeUserErrorInvalidCertificate {
    tag: ClientRevokeUserErrorTag.InvalidCertificate
    error: string
}
export interface ClientRevokeUserErrorInvalidKeysBundle {
    tag: ClientRevokeUserErrorTag.InvalidKeysBundle
    error: string
}
export interface ClientRevokeUserErrorNoKey {
    tag: ClientRevokeUserErrorTag.NoKey
    error: string
}
export interface ClientRevokeUserErrorOffline {
    tag: ClientRevokeUserErrorTag.Offline
    error: string
}
export interface ClientRevokeUserErrorStopped {
    tag: ClientRevokeUserErrorTag.Stopped
    error: string
}
export interface ClientRevokeUserErrorTimestampOutOfBallpark {
    tag: ClientRevokeUserErrorTag.TimestampOutOfBallpark
    error: string
}
export interface ClientRevokeUserErrorUserIsSelf {
    tag: ClientRevokeUserErrorTag.UserIsSelf
    error: string
}
export interface ClientRevokeUserErrorUserNotFound {
    tag: ClientRevokeUserErrorTag.UserNotFound
    error: string
}
export type ClientRevokeUserError =
  | ClientRevokeUserErrorAuthorNotAllowed
  | ClientRevokeUserErrorInternal
  | ClientRevokeUserErrorInvalidCertificate
  | ClientRevokeUserErrorInvalidKeysBundle
  | ClientRevokeUserErrorNoKey
  | ClientRevokeUserErrorOffline
  | ClientRevokeUserErrorStopped
  | ClientRevokeUserErrorTimestampOutOfBallpark
  | ClientRevokeUserErrorUserIsSelf
  | ClientRevokeUserErrorUserNotFound

// ClientShareWorkspaceError
export enum ClientShareWorkspaceErrorTag {
    AuthorNotAllowed = 'ClientShareWorkspaceErrorAuthorNotAllowed',
    Internal = 'ClientShareWorkspaceErrorInternal',
    InvalidCertificate = 'ClientShareWorkspaceErrorInvalidCertificate',
    InvalidKeysBundle = 'ClientShareWorkspaceErrorInvalidKeysBundle',
    Offline = 'ClientShareWorkspaceErrorOffline',
    RecipientIsSelf = 'ClientShareWorkspaceErrorRecipientIsSelf',
    RecipientNotFound = 'ClientShareWorkspaceErrorRecipientNotFound',
    RecipientRevoked = 'ClientShareWorkspaceErrorRecipientRevoked',
    RoleIncompatibleWithOutsider = 'ClientShareWorkspaceErrorRoleIncompatibleWithOutsider',
    Stopped = 'ClientShareWorkspaceErrorStopped',
    TimestampOutOfBallpark = 'ClientShareWorkspaceErrorTimestampOutOfBallpark',
    WorkspaceNotFound = 'ClientShareWorkspaceErrorWorkspaceNotFound',
}

export interface ClientShareWorkspaceErrorAuthorNotAllowed {
    tag: ClientShareWorkspaceErrorTag.AuthorNotAllowed
    error: string
}
export interface ClientShareWorkspaceErrorInternal {
    tag: ClientShareWorkspaceErrorTag.Internal
    error: string
}
export interface ClientShareWorkspaceErrorInvalidCertificate {
    tag: ClientShareWorkspaceErrorTag.InvalidCertificate
    error: string
}
export interface ClientShareWorkspaceErrorInvalidKeysBundle {
    tag: ClientShareWorkspaceErrorTag.InvalidKeysBundle
    error: string
}
export interface ClientShareWorkspaceErrorOffline {
    tag: ClientShareWorkspaceErrorTag.Offline
    error: string
}
export interface ClientShareWorkspaceErrorRecipientIsSelf {
    tag: ClientShareWorkspaceErrorTag.RecipientIsSelf
    error: string
}
export interface ClientShareWorkspaceErrorRecipientNotFound {
    tag: ClientShareWorkspaceErrorTag.RecipientNotFound
    error: string
}
export interface ClientShareWorkspaceErrorRecipientRevoked {
    tag: ClientShareWorkspaceErrorTag.RecipientRevoked
    error: string
}
export interface ClientShareWorkspaceErrorRoleIncompatibleWithOutsider {
    tag: ClientShareWorkspaceErrorTag.RoleIncompatibleWithOutsider
    error: string
}
export interface ClientShareWorkspaceErrorStopped {
    tag: ClientShareWorkspaceErrorTag.Stopped
    error: string
}
export interface ClientShareWorkspaceErrorTimestampOutOfBallpark {
    tag: ClientShareWorkspaceErrorTag.TimestampOutOfBallpark
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export interface ClientShareWorkspaceErrorWorkspaceNotFound {
    tag: ClientShareWorkspaceErrorTag.WorkspaceNotFound
    error: string
}
export type ClientShareWorkspaceError =
  | ClientShareWorkspaceErrorAuthorNotAllowed
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

// ClientStartError
export enum ClientStartErrorTag {
    Internal = 'ClientStartErrorInternal',
    LoadDeviceDecryptionFailed = 'ClientStartErrorLoadDeviceDecryptionFailed',
    LoadDeviceInvalidData = 'ClientStartErrorLoadDeviceInvalidData',
    LoadDeviceInvalidPath = 'ClientStartErrorLoadDeviceInvalidPath',
}

export interface ClientStartErrorInternal {
    tag: ClientStartErrorTag.Internal
    error: string
}
export interface ClientStartErrorLoadDeviceDecryptionFailed {
    tag: ClientStartErrorTag.LoadDeviceDecryptionFailed
    error: string
}
export interface ClientStartErrorLoadDeviceInvalidData {
    tag: ClientStartErrorTag.LoadDeviceInvalidData
    error: string
}
export interface ClientStartErrorLoadDeviceInvalidPath {
    tag: ClientStartErrorTag.LoadDeviceInvalidPath
    error: string
}
export type ClientStartError =
  | ClientStartErrorInternal
  | ClientStartErrorLoadDeviceDecryptionFailed
  | ClientStartErrorLoadDeviceInvalidData
  | ClientStartErrorLoadDeviceInvalidPath

// ClientStartInvitationGreetError
export enum ClientStartInvitationGreetErrorTag {
    Internal = 'ClientStartInvitationGreetErrorInternal',
}

export interface ClientStartInvitationGreetErrorInternal {
    tag: ClientStartInvitationGreetErrorTag.Internal
    error: string
}
export type ClientStartInvitationGreetError =
  | ClientStartInvitationGreetErrorInternal

// ClientStartWorkspaceError
export enum ClientStartWorkspaceErrorTag {
    Internal = 'ClientStartWorkspaceErrorInternal',
    WorkspaceNotFound = 'ClientStartWorkspaceErrorWorkspaceNotFound',
}

export interface ClientStartWorkspaceErrorInternal {
    tag: ClientStartWorkspaceErrorTag.Internal
    error: string
}
export interface ClientStartWorkspaceErrorWorkspaceNotFound {
    tag: ClientStartWorkspaceErrorTag.WorkspaceNotFound
    error: string
}
export type ClientStartWorkspaceError =
  | ClientStartWorkspaceErrorInternal
  | ClientStartWorkspaceErrorWorkspaceNotFound

// ClientStopError
export enum ClientStopErrorTag {
    Internal = 'ClientStopErrorInternal',
}

export interface ClientStopErrorInternal {
    tag: ClientStopErrorTag.Internal
    error: string
}
export type ClientStopError =
  | ClientStopErrorInternal

// DeviceAccessStrategy
export enum DeviceAccessStrategyTag {
    Keyring = 'DeviceAccessStrategyKeyring',
    Password = 'DeviceAccessStrategyPassword',
    Smartcard = 'DeviceAccessStrategySmartcard',
}

export interface DeviceAccessStrategyKeyring {
    tag: DeviceAccessStrategyTag.Keyring
    keyFile: Path
}
export interface DeviceAccessStrategyPassword {
    tag: DeviceAccessStrategyTag.Password
    password: Password
    keyFile: Path
}
export interface DeviceAccessStrategySmartcard {
    tag: DeviceAccessStrategyTag.Smartcard
    keyFile: Path
}
export type DeviceAccessStrategy =
  | DeviceAccessStrategyKeyring
  | DeviceAccessStrategyPassword
  | DeviceAccessStrategySmartcard

// DeviceSaveStrategy
export enum DeviceSaveStrategyTag {
    Keyring = 'DeviceSaveStrategyKeyring',
    Password = 'DeviceSaveStrategyPassword',
    Smartcard = 'DeviceSaveStrategySmartcard',
}

export interface DeviceSaveStrategyKeyring {
    tag: DeviceSaveStrategyTag.Keyring
}
export interface DeviceSaveStrategyPassword {
    tag: DeviceSaveStrategyTag.Password
    password: Password
}
export interface DeviceSaveStrategySmartcard {
    tag: DeviceSaveStrategyTag.Smartcard
}
export type DeviceSaveStrategy =
  | DeviceSaveStrategyKeyring
  | DeviceSaveStrategyPassword
  | DeviceSaveStrategySmartcard

// EntryStat
export enum EntryStatTag {
    File = 'EntryStatFile',
    Folder = 'EntryStatFolder',
}

export interface EntryStatFile {
    tag: EntryStatTag.File
    confinementPoint: VlobID | null
    id: VlobID
    created: DateTime
    updated: DateTime
    baseVersion: VersionInt
    isPlaceholder: boolean
    needSync: boolean
    size: SizeInt
}
export interface EntryStatFolder {
    tag: EntryStatTag.Folder
    confinementPoint: VlobID | null
    id: VlobID
    created: DateTime
    updated: DateTime
    baseVersion: VersionInt
    isPlaceholder: boolean
    needSync: boolean
    children: Array<[EntryName, VlobID]>
}
export type EntryStat =
  | EntryStatFile
  | EntryStatFolder

// GreetInProgressError
export enum GreetInProgressErrorTag {
    ActiveUsersLimitReached = 'GreetInProgressErrorActiveUsersLimitReached',
    AlreadyDeleted = 'GreetInProgressErrorAlreadyDeleted',
    Cancelled = 'GreetInProgressErrorCancelled',
    CorruptedInviteUserData = 'GreetInProgressErrorCorruptedInviteUserData',
    DeviceAlreadyExists = 'GreetInProgressErrorDeviceAlreadyExists',
    HumanHandleAlreadyTaken = 'GreetInProgressErrorHumanHandleAlreadyTaken',
    Internal = 'GreetInProgressErrorInternal',
    NonceMismatch = 'GreetInProgressErrorNonceMismatch',
    NotFound = 'GreetInProgressErrorNotFound',
    Offline = 'GreetInProgressErrorOffline',
    PeerReset = 'GreetInProgressErrorPeerReset',
    TimestampOutOfBallpark = 'GreetInProgressErrorTimestampOutOfBallpark',
    UserAlreadyExists = 'GreetInProgressErrorUserAlreadyExists',
    UserCreateNotAllowed = 'GreetInProgressErrorUserCreateNotAllowed',
}

export interface GreetInProgressErrorActiveUsersLimitReached {
    tag: GreetInProgressErrorTag.ActiveUsersLimitReached
    error: string
}
export interface GreetInProgressErrorAlreadyDeleted {
    tag: GreetInProgressErrorTag.AlreadyDeleted
    error: string
}
export interface GreetInProgressErrorCancelled {
    tag: GreetInProgressErrorTag.Cancelled
    error: string
}
export interface GreetInProgressErrorCorruptedInviteUserData {
    tag: GreetInProgressErrorTag.CorruptedInviteUserData
    error: string
}
export interface GreetInProgressErrorDeviceAlreadyExists {
    tag: GreetInProgressErrorTag.DeviceAlreadyExists
    error: string
}
export interface GreetInProgressErrorHumanHandleAlreadyTaken {
    tag: GreetInProgressErrorTag.HumanHandleAlreadyTaken
    error: string
}
export interface GreetInProgressErrorInternal {
    tag: GreetInProgressErrorTag.Internal
    error: string
}
export interface GreetInProgressErrorNonceMismatch {
    tag: GreetInProgressErrorTag.NonceMismatch
    error: string
}
export interface GreetInProgressErrorNotFound {
    tag: GreetInProgressErrorTag.NotFound
    error: string
}
export interface GreetInProgressErrorOffline {
    tag: GreetInProgressErrorTag.Offline
    error: string
}
export interface GreetInProgressErrorPeerReset {
    tag: GreetInProgressErrorTag.PeerReset
    error: string
}
export interface GreetInProgressErrorTimestampOutOfBallpark {
    tag: GreetInProgressErrorTag.TimestampOutOfBallpark
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export interface GreetInProgressErrorUserAlreadyExists {
    tag: GreetInProgressErrorTag.UserAlreadyExists
    error: string
}
export interface GreetInProgressErrorUserCreateNotAllowed {
    tag: GreetInProgressErrorTag.UserCreateNotAllowed
    error: string
}
export type GreetInProgressError =
  | GreetInProgressErrorActiveUsersLimitReached
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

// InviteListItem
export enum InviteListItemTag {
    Device = 'InviteListItemDevice',
    User = 'InviteListItemUser',
}

export interface InviteListItemDevice {
    tag: InviteListItemTag.Device
    addr: ParsecInvitationAddr
    token: InvitationToken
    createdOn: DateTime
    status: InvitationStatus
}
export interface InviteListItemUser {
    tag: InviteListItemTag.User
    addr: ParsecInvitationAddr
    token: InvitationToken
    createdOn: DateTime
    claimerEmail: string
    status: InvitationStatus
}
export type InviteListItem =
  | InviteListItemDevice
  | InviteListItemUser

// ListInvitationsError
export enum ListInvitationsErrorTag {
    Internal = 'ListInvitationsErrorInternal',
    Offline = 'ListInvitationsErrorOffline',
}

export interface ListInvitationsErrorInternal {
    tag: ListInvitationsErrorTag.Internal
    error: string
}
export interface ListInvitationsErrorOffline {
    tag: ListInvitationsErrorTag.Offline
    error: string
}
export type ListInvitationsError =
  | ListInvitationsErrorInternal
  | ListInvitationsErrorOffline

// MountpointMountStrategy
export enum MountpointMountStrategyTag {
    Directory = 'MountpointMountStrategyDirectory',
    Disabled = 'MountpointMountStrategyDisabled',
    DriveLetter = 'MountpointMountStrategyDriveLetter',
}

export interface MountpointMountStrategyDirectory {
    tag: MountpointMountStrategyTag.Directory
    baseDir: Path
}
export interface MountpointMountStrategyDisabled {
    tag: MountpointMountStrategyTag.Disabled
}
export interface MountpointMountStrategyDriveLetter {
    tag: MountpointMountStrategyTag.DriveLetter
}
export type MountpointMountStrategy =
  | MountpointMountStrategyDirectory
  | MountpointMountStrategyDisabled
  | MountpointMountStrategyDriveLetter

// MountpointToOsPathError
export enum MountpointToOsPathErrorTag {
    Internal = 'MountpointToOsPathErrorInternal',
}

export interface MountpointToOsPathErrorInternal {
    tag: MountpointToOsPathErrorTag.Internal
    error: string
}
export type MountpointToOsPathError =
  | MountpointToOsPathErrorInternal

// MountpointUnmountError
export enum MountpointUnmountErrorTag {
    Internal = 'MountpointUnmountErrorInternal',
}

export interface MountpointUnmountErrorInternal {
    tag: MountpointUnmountErrorTag.Internal
    error: string
}
export type MountpointUnmountError =
  | MountpointUnmountErrorInternal

// ParseBackendAddrError
export enum ParseBackendAddrErrorTag {
    InvalidUrl = 'ParseBackendAddrErrorInvalidUrl',
}

export interface ParseBackendAddrErrorInvalidUrl {
    tag: ParseBackendAddrErrorTag.InvalidUrl
    error: string
}
export type ParseBackendAddrError =
  | ParseBackendAddrErrorInvalidUrl

// ParsedParsecAddr
export enum ParsedParsecAddrTag {
    InvitationDevice = 'ParsedParsecAddrInvitationDevice',
    InvitationUser = 'ParsedParsecAddrInvitationUser',
    Organization = 'ParsedParsecAddrOrganization',
    OrganizationBootstrap = 'ParsedParsecAddrOrganizationBootstrap',
    OrganizationFileLink = 'ParsedParsecAddrOrganizationFileLink',
    PkiEnrollment = 'ParsedParsecAddrPkiEnrollment',
    Server = 'ParsedParsecAddrServer',
}

export interface ParsedParsecAddrInvitationDevice {
    tag: ParsedParsecAddrTag.InvitationDevice
    hostname: string
    port: U32
    useSsl: boolean
    organizationId: OrganizationID
    token: InvitationToken
}
export interface ParsedParsecAddrInvitationUser {
    tag: ParsedParsecAddrTag.InvitationUser
    hostname: string
    port: U32
    useSsl: boolean
    organizationId: OrganizationID
    token: InvitationToken
}
export interface ParsedParsecAddrOrganization {
    tag: ParsedParsecAddrTag.Organization
    hostname: string
    port: U32
    useSsl: boolean
    organizationId: OrganizationID
}
export interface ParsedParsecAddrOrganizationBootstrap {
    tag: ParsedParsecAddrTag.OrganizationBootstrap
    hostname: string
    port: U32
    useSsl: boolean
    organizationId: OrganizationID
    token: string | null
}
export interface ParsedParsecAddrOrganizationFileLink {
    tag: ParsedParsecAddrTag.OrganizationFileLink
    hostname: string
    port: U32
    useSsl: boolean
    organizationId: OrganizationID
    workspaceId: VlobID
    encryptedPath: Uint8Array
    encryptedTimestamp: Uint8Array | null
}
export interface ParsedParsecAddrPkiEnrollment {
    tag: ParsedParsecAddrTag.PkiEnrollment
    hostname: string
    port: U32
    useSsl: boolean
    organizationId: OrganizationID
}
export interface ParsedParsecAddrServer {
    tag: ParsedParsecAddrTag.Server
    hostname: string
    port: U32
    useSsl: boolean
}
export type ParsedParsecAddr =
  | ParsedParsecAddrInvitationDevice
  | ParsedParsecAddrInvitationUser
  | ParsedParsecAddrOrganization
  | ParsedParsecAddrOrganizationBootstrap
  | ParsedParsecAddrOrganizationFileLink
  | ParsedParsecAddrPkiEnrollment
  | ParsedParsecAddrServer

// TestbedError
export enum TestbedErrorTag {
    Disabled = 'TestbedErrorDisabled',
    Internal = 'TestbedErrorInternal',
}

export interface TestbedErrorDisabled {
    tag: TestbedErrorTag.Disabled
    error: string
}
export interface TestbedErrorInternal {
    tag: TestbedErrorTag.Internal
    error: string
}
export type TestbedError =
  | TestbedErrorDisabled
  | TestbedErrorInternal

// UserOrDeviceClaimInitialInfo
export enum UserOrDeviceClaimInitialInfoTag {
    Device = 'UserOrDeviceClaimInitialInfoDevice',
    User = 'UserOrDeviceClaimInitialInfoUser',
}

export interface UserOrDeviceClaimInitialInfoDevice {
    tag: UserOrDeviceClaimInitialInfoTag.Device
    handle: Handle
    greeterUserId: UserID
    greeterHumanHandle: HumanHandle
}
export interface UserOrDeviceClaimInitialInfoUser {
    tag: UserOrDeviceClaimInitialInfoTag.User
    handle: Handle
    claimerEmail: string
    greeterUserId: UserID
    greeterHumanHandle: HumanHandle
}
export type UserOrDeviceClaimInitialInfo =
  | UserOrDeviceClaimInitialInfoDevice
  | UserOrDeviceClaimInitialInfoUser

// WorkspaceCreateFileError
export enum WorkspaceCreateFileErrorTag {
    EntryExists = 'WorkspaceCreateFileErrorEntryExists',
    Internal = 'WorkspaceCreateFileErrorInternal',
    InvalidCertificate = 'WorkspaceCreateFileErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceCreateFileErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceCreateFileErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceCreateFileErrorNoRealmAccess',
    Offline = 'WorkspaceCreateFileErrorOffline',
    ParentIsFile = 'WorkspaceCreateFileErrorParentIsFile',
    ParentNotFound = 'WorkspaceCreateFileErrorParentNotFound',
    ReadOnlyRealm = 'WorkspaceCreateFileErrorReadOnlyRealm',
    Stopped = 'WorkspaceCreateFileErrorStopped',
}

export interface WorkspaceCreateFileErrorEntryExists {
    tag: WorkspaceCreateFileErrorTag.EntryExists
    error: string
}
export interface WorkspaceCreateFileErrorInternal {
    tag: WorkspaceCreateFileErrorTag.Internal
    error: string
}
export interface WorkspaceCreateFileErrorInvalidCertificate {
    tag: WorkspaceCreateFileErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceCreateFileErrorInvalidKeysBundle {
    tag: WorkspaceCreateFileErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceCreateFileErrorInvalidManifest {
    tag: WorkspaceCreateFileErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceCreateFileErrorNoRealmAccess {
    tag: WorkspaceCreateFileErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceCreateFileErrorOffline {
    tag: WorkspaceCreateFileErrorTag.Offline
    error: string
}
export interface WorkspaceCreateFileErrorParentIsFile {
    tag: WorkspaceCreateFileErrorTag.ParentIsFile
    error: string
}
export interface WorkspaceCreateFileErrorParentNotFound {
    tag: WorkspaceCreateFileErrorTag.ParentNotFound
    error: string
}
export interface WorkspaceCreateFileErrorReadOnlyRealm {
    tag: WorkspaceCreateFileErrorTag.ReadOnlyRealm
    error: string
}
export interface WorkspaceCreateFileErrorStopped {
    tag: WorkspaceCreateFileErrorTag.Stopped
    error: string
}
export type WorkspaceCreateFileError =
  | WorkspaceCreateFileErrorEntryExists
  | WorkspaceCreateFileErrorInternal
  | WorkspaceCreateFileErrorInvalidCertificate
  | WorkspaceCreateFileErrorInvalidKeysBundle
  | WorkspaceCreateFileErrorInvalidManifest
  | WorkspaceCreateFileErrorNoRealmAccess
  | WorkspaceCreateFileErrorOffline
  | WorkspaceCreateFileErrorParentIsFile
  | WorkspaceCreateFileErrorParentNotFound
  | WorkspaceCreateFileErrorReadOnlyRealm
  | WorkspaceCreateFileErrorStopped

// WorkspaceCreateFolderError
export enum WorkspaceCreateFolderErrorTag {
    EntryExists = 'WorkspaceCreateFolderErrorEntryExists',
    Internal = 'WorkspaceCreateFolderErrorInternal',
    InvalidCertificate = 'WorkspaceCreateFolderErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceCreateFolderErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceCreateFolderErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceCreateFolderErrorNoRealmAccess',
    Offline = 'WorkspaceCreateFolderErrorOffline',
    ParentIsFile = 'WorkspaceCreateFolderErrorParentIsFile',
    ParentNotFound = 'WorkspaceCreateFolderErrorParentNotFound',
    ReadOnlyRealm = 'WorkspaceCreateFolderErrorReadOnlyRealm',
    Stopped = 'WorkspaceCreateFolderErrorStopped',
}

export interface WorkspaceCreateFolderErrorEntryExists {
    tag: WorkspaceCreateFolderErrorTag.EntryExists
    error: string
}
export interface WorkspaceCreateFolderErrorInternal {
    tag: WorkspaceCreateFolderErrorTag.Internal
    error: string
}
export interface WorkspaceCreateFolderErrorInvalidCertificate {
    tag: WorkspaceCreateFolderErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceCreateFolderErrorInvalidKeysBundle {
    tag: WorkspaceCreateFolderErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceCreateFolderErrorInvalidManifest {
    tag: WorkspaceCreateFolderErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceCreateFolderErrorNoRealmAccess {
    tag: WorkspaceCreateFolderErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceCreateFolderErrorOffline {
    tag: WorkspaceCreateFolderErrorTag.Offline
    error: string
}
export interface WorkspaceCreateFolderErrorParentIsFile {
    tag: WorkspaceCreateFolderErrorTag.ParentIsFile
    error: string
}
export interface WorkspaceCreateFolderErrorParentNotFound {
    tag: WorkspaceCreateFolderErrorTag.ParentNotFound
    error: string
}
export interface WorkspaceCreateFolderErrorReadOnlyRealm {
    tag: WorkspaceCreateFolderErrorTag.ReadOnlyRealm
    error: string
}
export interface WorkspaceCreateFolderErrorStopped {
    tag: WorkspaceCreateFolderErrorTag.Stopped
    error: string
}
export type WorkspaceCreateFolderError =
  | WorkspaceCreateFolderErrorEntryExists
  | WorkspaceCreateFolderErrorInternal
  | WorkspaceCreateFolderErrorInvalidCertificate
  | WorkspaceCreateFolderErrorInvalidKeysBundle
  | WorkspaceCreateFolderErrorInvalidManifest
  | WorkspaceCreateFolderErrorNoRealmAccess
  | WorkspaceCreateFolderErrorOffline
  | WorkspaceCreateFolderErrorParentIsFile
  | WorkspaceCreateFolderErrorParentNotFound
  | WorkspaceCreateFolderErrorReadOnlyRealm
  | WorkspaceCreateFolderErrorStopped

// WorkspaceFdCloseError
export enum WorkspaceFdCloseErrorTag {
    BadFileDescriptor = 'WorkspaceFdCloseErrorBadFileDescriptor',
    Internal = 'WorkspaceFdCloseErrorInternal',
    Stopped = 'WorkspaceFdCloseErrorStopped',
}

export interface WorkspaceFdCloseErrorBadFileDescriptor {
    tag: WorkspaceFdCloseErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceFdCloseErrorInternal {
    tag: WorkspaceFdCloseErrorTag.Internal
    error: string
}
export interface WorkspaceFdCloseErrorStopped {
    tag: WorkspaceFdCloseErrorTag.Stopped
    error: string
}
export type WorkspaceFdCloseError =
  | WorkspaceFdCloseErrorBadFileDescriptor
  | WorkspaceFdCloseErrorInternal
  | WorkspaceFdCloseErrorStopped

// WorkspaceFdFlushError
export enum WorkspaceFdFlushErrorTag {
    BadFileDescriptor = 'WorkspaceFdFlushErrorBadFileDescriptor',
    Internal = 'WorkspaceFdFlushErrorInternal',
    NotInWriteMode = 'WorkspaceFdFlushErrorNotInWriteMode',
    Stopped = 'WorkspaceFdFlushErrorStopped',
}

export interface WorkspaceFdFlushErrorBadFileDescriptor {
    tag: WorkspaceFdFlushErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceFdFlushErrorInternal {
    tag: WorkspaceFdFlushErrorTag.Internal
    error: string
}
export interface WorkspaceFdFlushErrorNotInWriteMode {
    tag: WorkspaceFdFlushErrorTag.NotInWriteMode
    error: string
}
export interface WorkspaceFdFlushErrorStopped {
    tag: WorkspaceFdFlushErrorTag.Stopped
    error: string
}
export type WorkspaceFdFlushError =
  | WorkspaceFdFlushErrorBadFileDescriptor
  | WorkspaceFdFlushErrorInternal
  | WorkspaceFdFlushErrorNotInWriteMode
  | WorkspaceFdFlushErrorStopped

// WorkspaceFdReadError
export enum WorkspaceFdReadErrorTag {
    BadFileDescriptor = 'WorkspaceFdReadErrorBadFileDescriptor',
    Internal = 'WorkspaceFdReadErrorInternal',
    InvalidBlockAccess = 'WorkspaceFdReadErrorInvalidBlockAccess',
    InvalidCertificate = 'WorkspaceFdReadErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceFdReadErrorInvalidKeysBundle',
    NoRealmAccess = 'WorkspaceFdReadErrorNoRealmAccess',
    NotInReadMode = 'WorkspaceFdReadErrorNotInReadMode',
    Offline = 'WorkspaceFdReadErrorOffline',
    Stopped = 'WorkspaceFdReadErrorStopped',
}

export interface WorkspaceFdReadErrorBadFileDescriptor {
    tag: WorkspaceFdReadErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceFdReadErrorInternal {
    tag: WorkspaceFdReadErrorTag.Internal
    error: string
}
export interface WorkspaceFdReadErrorInvalidBlockAccess {
    tag: WorkspaceFdReadErrorTag.InvalidBlockAccess
    error: string
}
export interface WorkspaceFdReadErrorInvalidCertificate {
    tag: WorkspaceFdReadErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceFdReadErrorInvalidKeysBundle {
    tag: WorkspaceFdReadErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceFdReadErrorNoRealmAccess {
    tag: WorkspaceFdReadErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceFdReadErrorNotInReadMode {
    tag: WorkspaceFdReadErrorTag.NotInReadMode
    error: string
}
export interface WorkspaceFdReadErrorOffline {
    tag: WorkspaceFdReadErrorTag.Offline
    error: string
}
export interface WorkspaceFdReadErrorStopped {
    tag: WorkspaceFdReadErrorTag.Stopped
    error: string
}
export type WorkspaceFdReadError =
  | WorkspaceFdReadErrorBadFileDescriptor
  | WorkspaceFdReadErrorInternal
  | WorkspaceFdReadErrorInvalidBlockAccess
  | WorkspaceFdReadErrorInvalidCertificate
  | WorkspaceFdReadErrorInvalidKeysBundle
  | WorkspaceFdReadErrorNoRealmAccess
  | WorkspaceFdReadErrorNotInReadMode
  | WorkspaceFdReadErrorOffline
  | WorkspaceFdReadErrorStopped

// WorkspaceFdResizeError
export enum WorkspaceFdResizeErrorTag {
    BadFileDescriptor = 'WorkspaceFdResizeErrorBadFileDescriptor',
    Internal = 'WorkspaceFdResizeErrorInternal',
    NotInWriteMode = 'WorkspaceFdResizeErrorNotInWriteMode',
}

export interface WorkspaceFdResizeErrorBadFileDescriptor {
    tag: WorkspaceFdResizeErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceFdResizeErrorInternal {
    tag: WorkspaceFdResizeErrorTag.Internal
    error: string
}
export interface WorkspaceFdResizeErrorNotInWriteMode {
    tag: WorkspaceFdResizeErrorTag.NotInWriteMode
    error: string
}
export type WorkspaceFdResizeError =
  | WorkspaceFdResizeErrorBadFileDescriptor
  | WorkspaceFdResizeErrorInternal
  | WorkspaceFdResizeErrorNotInWriteMode

// WorkspaceFdWriteError
export enum WorkspaceFdWriteErrorTag {
    BadFileDescriptor = 'WorkspaceFdWriteErrorBadFileDescriptor',
    Internal = 'WorkspaceFdWriteErrorInternal',
    NotInWriteMode = 'WorkspaceFdWriteErrorNotInWriteMode',
}

export interface WorkspaceFdWriteErrorBadFileDescriptor {
    tag: WorkspaceFdWriteErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceFdWriteErrorInternal {
    tag: WorkspaceFdWriteErrorTag.Internal
    error: string
}
export interface WorkspaceFdWriteErrorNotInWriteMode {
    tag: WorkspaceFdWriteErrorTag.NotInWriteMode
    error: string
}
export type WorkspaceFdWriteError =
  | WorkspaceFdWriteErrorBadFileDescriptor
  | WorkspaceFdWriteErrorInternal
  | WorkspaceFdWriteErrorNotInWriteMode

// WorkspaceInfoError
export enum WorkspaceInfoErrorTag {
    Internal = 'WorkspaceInfoErrorInternal',
}

export interface WorkspaceInfoErrorInternal {
    tag: WorkspaceInfoErrorTag.Internal
    error: string
}
export type WorkspaceInfoError =
  | WorkspaceInfoErrorInternal

// WorkspaceMountError
export enum WorkspaceMountErrorTag {
    Disabled = 'WorkspaceMountErrorDisabled',
    Internal = 'WorkspaceMountErrorInternal',
}

export interface WorkspaceMountErrorDisabled {
    tag: WorkspaceMountErrorTag.Disabled
    error: string
}
export interface WorkspaceMountErrorInternal {
    tag: WorkspaceMountErrorTag.Internal
    error: string
}
export type WorkspaceMountError =
  | WorkspaceMountErrorDisabled
  | WorkspaceMountErrorInternal

// WorkspaceOpenFileError
export enum WorkspaceOpenFileErrorTag {
    EntryExistsInCreateNewMode = 'WorkspaceOpenFileErrorEntryExistsInCreateNewMode',
    EntryNotAFile = 'WorkspaceOpenFileErrorEntryNotAFile',
    EntryNotFound = 'WorkspaceOpenFileErrorEntryNotFound',
    Internal = 'WorkspaceOpenFileErrorInternal',
    InvalidCertificate = 'WorkspaceOpenFileErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceOpenFileErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceOpenFileErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceOpenFileErrorNoRealmAccess',
    Offline = 'WorkspaceOpenFileErrorOffline',
    ReadOnlyRealm = 'WorkspaceOpenFileErrorReadOnlyRealm',
    Stopped = 'WorkspaceOpenFileErrorStopped',
}

export interface WorkspaceOpenFileErrorEntryExistsInCreateNewMode {
    tag: WorkspaceOpenFileErrorTag.EntryExistsInCreateNewMode
    error: string
}
export interface WorkspaceOpenFileErrorEntryNotAFile {
    tag: WorkspaceOpenFileErrorTag.EntryNotAFile
    error: string
}
export interface WorkspaceOpenFileErrorEntryNotFound {
    tag: WorkspaceOpenFileErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceOpenFileErrorInternal {
    tag: WorkspaceOpenFileErrorTag.Internal
    error: string
}
export interface WorkspaceOpenFileErrorInvalidCertificate {
    tag: WorkspaceOpenFileErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceOpenFileErrorInvalidKeysBundle {
    tag: WorkspaceOpenFileErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceOpenFileErrorInvalidManifest {
    tag: WorkspaceOpenFileErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceOpenFileErrorNoRealmAccess {
    tag: WorkspaceOpenFileErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceOpenFileErrorOffline {
    tag: WorkspaceOpenFileErrorTag.Offline
    error: string
}
export interface WorkspaceOpenFileErrorReadOnlyRealm {
    tag: WorkspaceOpenFileErrorTag.ReadOnlyRealm
    error: string
}
export interface WorkspaceOpenFileErrorStopped {
    tag: WorkspaceOpenFileErrorTag.Stopped
    error: string
}
export type WorkspaceOpenFileError =
  | WorkspaceOpenFileErrorEntryExistsInCreateNewMode
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

// WorkspaceRemoveEntryError
export enum WorkspaceRemoveEntryErrorTag {
    CannotRemoveRoot = 'WorkspaceRemoveEntryErrorCannotRemoveRoot',
    EntryIsFile = 'WorkspaceRemoveEntryErrorEntryIsFile',
    EntryIsFolder = 'WorkspaceRemoveEntryErrorEntryIsFolder',
    EntryIsNonEmptyFolder = 'WorkspaceRemoveEntryErrorEntryIsNonEmptyFolder',
    EntryNotFound = 'WorkspaceRemoveEntryErrorEntryNotFound',
    Internal = 'WorkspaceRemoveEntryErrorInternal',
    InvalidCertificate = 'WorkspaceRemoveEntryErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceRemoveEntryErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceRemoveEntryErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceRemoveEntryErrorNoRealmAccess',
    Offline = 'WorkspaceRemoveEntryErrorOffline',
    ReadOnlyRealm = 'WorkspaceRemoveEntryErrorReadOnlyRealm',
    Stopped = 'WorkspaceRemoveEntryErrorStopped',
}

export interface WorkspaceRemoveEntryErrorCannotRemoveRoot {
    tag: WorkspaceRemoveEntryErrorTag.CannotRemoveRoot
    error: string
}
export interface WorkspaceRemoveEntryErrorEntryIsFile {
    tag: WorkspaceRemoveEntryErrorTag.EntryIsFile
    error: string
}
export interface WorkspaceRemoveEntryErrorEntryIsFolder {
    tag: WorkspaceRemoveEntryErrorTag.EntryIsFolder
    error: string
}
export interface WorkspaceRemoveEntryErrorEntryIsNonEmptyFolder {
    tag: WorkspaceRemoveEntryErrorTag.EntryIsNonEmptyFolder
    error: string
}
export interface WorkspaceRemoveEntryErrorEntryNotFound {
    tag: WorkspaceRemoveEntryErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceRemoveEntryErrorInternal {
    tag: WorkspaceRemoveEntryErrorTag.Internal
    error: string
}
export interface WorkspaceRemoveEntryErrorInvalidCertificate {
    tag: WorkspaceRemoveEntryErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceRemoveEntryErrorInvalidKeysBundle {
    tag: WorkspaceRemoveEntryErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceRemoveEntryErrorInvalidManifest {
    tag: WorkspaceRemoveEntryErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceRemoveEntryErrorNoRealmAccess {
    tag: WorkspaceRemoveEntryErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceRemoveEntryErrorOffline {
    tag: WorkspaceRemoveEntryErrorTag.Offline
    error: string
}
export interface WorkspaceRemoveEntryErrorReadOnlyRealm {
    tag: WorkspaceRemoveEntryErrorTag.ReadOnlyRealm
    error: string
}
export interface WorkspaceRemoveEntryErrorStopped {
    tag: WorkspaceRemoveEntryErrorTag.Stopped
    error: string
}
export type WorkspaceRemoveEntryError =
  | WorkspaceRemoveEntryErrorCannotRemoveRoot
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

// WorkspaceRenameEntryError
export enum WorkspaceRenameEntryErrorTag {
    CannotRenameRoot = 'WorkspaceRenameEntryErrorCannotRenameRoot',
    DestinationExists = 'WorkspaceRenameEntryErrorDestinationExists',
    EntryNotFound = 'WorkspaceRenameEntryErrorEntryNotFound',
    Internal = 'WorkspaceRenameEntryErrorInternal',
    InvalidCertificate = 'WorkspaceRenameEntryErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceRenameEntryErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceRenameEntryErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceRenameEntryErrorNoRealmAccess',
    Offline = 'WorkspaceRenameEntryErrorOffline',
    ReadOnlyRealm = 'WorkspaceRenameEntryErrorReadOnlyRealm',
    Stopped = 'WorkspaceRenameEntryErrorStopped',
}

export interface WorkspaceRenameEntryErrorCannotRenameRoot {
    tag: WorkspaceRenameEntryErrorTag.CannotRenameRoot
    error: string
}
export interface WorkspaceRenameEntryErrorDestinationExists {
    tag: WorkspaceRenameEntryErrorTag.DestinationExists
    error: string
}
export interface WorkspaceRenameEntryErrorEntryNotFound {
    tag: WorkspaceRenameEntryErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceRenameEntryErrorInternal {
    tag: WorkspaceRenameEntryErrorTag.Internal
    error: string
}
export interface WorkspaceRenameEntryErrorInvalidCertificate {
    tag: WorkspaceRenameEntryErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceRenameEntryErrorInvalidKeysBundle {
    tag: WorkspaceRenameEntryErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceRenameEntryErrorInvalidManifest {
    tag: WorkspaceRenameEntryErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceRenameEntryErrorNoRealmAccess {
    tag: WorkspaceRenameEntryErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceRenameEntryErrorOffline {
    tag: WorkspaceRenameEntryErrorTag.Offline
    error: string
}
export interface WorkspaceRenameEntryErrorReadOnlyRealm {
    tag: WorkspaceRenameEntryErrorTag.ReadOnlyRealm
    error: string
}
export interface WorkspaceRenameEntryErrorStopped {
    tag: WorkspaceRenameEntryErrorTag.Stopped
    error: string
}
export type WorkspaceRenameEntryError =
  | WorkspaceRenameEntryErrorCannotRenameRoot
  | WorkspaceRenameEntryErrorDestinationExists
  | WorkspaceRenameEntryErrorEntryNotFound
  | WorkspaceRenameEntryErrorInternal
  | WorkspaceRenameEntryErrorInvalidCertificate
  | WorkspaceRenameEntryErrorInvalidKeysBundle
  | WorkspaceRenameEntryErrorInvalidManifest
  | WorkspaceRenameEntryErrorNoRealmAccess
  | WorkspaceRenameEntryErrorOffline
  | WorkspaceRenameEntryErrorReadOnlyRealm
  | WorkspaceRenameEntryErrorStopped

// WorkspaceStatEntryError
export enum WorkspaceStatEntryErrorTag {
    EntryNotFound = 'WorkspaceStatEntryErrorEntryNotFound',
    Internal = 'WorkspaceStatEntryErrorInternal',
    InvalidCertificate = 'WorkspaceStatEntryErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceStatEntryErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceStatEntryErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceStatEntryErrorNoRealmAccess',
    Offline = 'WorkspaceStatEntryErrorOffline',
    Stopped = 'WorkspaceStatEntryErrorStopped',
}

export interface WorkspaceStatEntryErrorEntryNotFound {
    tag: WorkspaceStatEntryErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceStatEntryErrorInternal {
    tag: WorkspaceStatEntryErrorTag.Internal
    error: string
}
export interface WorkspaceStatEntryErrorInvalidCertificate {
    tag: WorkspaceStatEntryErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceStatEntryErrorInvalidKeysBundle {
    tag: WorkspaceStatEntryErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceStatEntryErrorInvalidManifest {
    tag: WorkspaceStatEntryErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceStatEntryErrorNoRealmAccess {
    tag: WorkspaceStatEntryErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceStatEntryErrorOffline {
    tag: WorkspaceStatEntryErrorTag.Offline
    error: string
}
export interface WorkspaceStatEntryErrorStopped {
    tag: WorkspaceStatEntryErrorTag.Stopped
    error: string
}
export type WorkspaceStatEntryError =
  | WorkspaceStatEntryErrorEntryNotFound
  | WorkspaceStatEntryErrorInternal
  | WorkspaceStatEntryErrorInvalidCertificate
  | WorkspaceStatEntryErrorInvalidKeysBundle
  | WorkspaceStatEntryErrorInvalidManifest
  | WorkspaceStatEntryErrorNoRealmAccess
  | WorkspaceStatEntryErrorOffline
  | WorkspaceStatEntryErrorStopped

// WorkspaceStopError
export enum WorkspaceStopErrorTag {
    Internal = 'WorkspaceStopErrorInternal',
}

export interface WorkspaceStopErrorInternal {
    tag: WorkspaceStopErrorTag.Internal
    error: string
}
export type WorkspaceStopError =
  | WorkspaceStopErrorInternal

// WorkspaceStorageCacheSize
export enum WorkspaceStorageCacheSizeTag {
    Custom = 'WorkspaceStorageCacheSizeCustom',
    Default = 'WorkspaceStorageCacheSizeDefault',
}

export interface WorkspaceStorageCacheSizeCustom {
    tag: WorkspaceStorageCacheSizeTag.Custom
    size: CacheSize
}
export interface WorkspaceStorageCacheSizeDefault {
    tag: WorkspaceStorageCacheSizeTag.Default
}
export type WorkspaceStorageCacheSize =
  | WorkspaceStorageCacheSizeCustom
  | WorkspaceStorageCacheSizeDefault

export interface LibParsecPlugin {
    bootstrapOrganization(
        config: ClientConfig,
        on_event_callback: (event: ClientEvent) => void,
        bootstrap_organization_addr: ParsecOrganizationBootstrapAddr,
        save_strategy: DeviceSaveStrategy,
        human_handle: HumanHandle,
        device_label: DeviceLabel,
        sequester_authority_verify_key: SequesterVerifyKeyDer | null
    ): Promise<Result<AvailableDevice, BootstrapOrganizationError>>
    buildBackendOrganizationBootstrapAddr(
        addr: ParsecAddr,
        organization_id: OrganizationID
    ): Promise<ParsecOrganizationBootstrapAddr>
    cancel(
        canceller: Handle
    ): Promise<Result<null, CancelError>>
    claimerDeviceFinalizeSaveLocalDevice(
        handle: Handle,
        save_strategy: DeviceSaveStrategy
    ): Promise<Result<AvailableDevice, ClaimInProgressError>>
    claimerDeviceInProgress1DoSignifyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<DeviceClaimInProgress2Info, ClaimInProgressError>>
    claimerDeviceInProgress2DoWaitPeerTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<DeviceClaimInProgress3Info, ClaimInProgressError>>
    claimerDeviceInProgress3DoClaim(
        canceller: Handle,
        handle: Handle,
        requested_device_label: DeviceLabel
    ): Promise<Result<DeviceClaimFinalizeInfo, ClaimInProgressError>>
    claimerDeviceInitialDoWaitPeer(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<DeviceClaimInProgress1Info, ClaimInProgressError>>
    claimerGreeterAbortOperation(
        handle: Handle
    ): Promise<Result<null, ClaimerGreeterAbortOperationError>>
    claimerRetrieveInfo(
        config: ClientConfig,
        on_event_callback: (event: ClientEvent) => void,
        addr: ParsecInvitationAddr
    ): Promise<Result<UserOrDeviceClaimInitialInfo, ClaimerRetrieveInfoError>>
    claimerUserFinalizeSaveLocalDevice(
        handle: Handle,
        save_strategy: DeviceSaveStrategy
    ): Promise<Result<AvailableDevice, ClaimInProgressError>>
    claimerUserInProgress1DoSignifyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<UserClaimInProgress2Info, ClaimInProgressError>>
    claimerUserInProgress2DoWaitPeerTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<UserClaimInProgress3Info, ClaimInProgressError>>
    claimerUserInProgress3DoClaim(
        canceller: Handle,
        handle: Handle,
        requested_device_label: DeviceLabel,
        requested_human_handle: HumanHandle
    ): Promise<Result<UserClaimFinalizeInfo, ClaimInProgressError>>
    claimerUserInitialDoWaitPeer(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<UserClaimInProgress1Info, ClaimInProgressError>>
    clientCancelInvitation(
        client: Handle,
        token: InvitationToken
    ): Promise<Result<null, ClientCancelInvitationError>>
    clientChangeAuthentication(
        client_config: ClientConfig,
        current_auth: DeviceAccessStrategy,
        new_auth: DeviceSaveStrategy
    ): Promise<Result<null, ClientChangeAuthenticationError>>
    clientCreateWorkspace(
        client: Handle,
        name: EntryName
    ): Promise<Result<VlobID, ClientCreateWorkspaceError>>
    clientGetUserDevice(
        client: Handle,
        device: DeviceID
    ): Promise<Result<[UserInfo, DeviceInfo], ClientGetUserDeviceError>>
    clientInfo(
        client: Handle
    ): Promise<Result<ClientInfo, ClientInfoError>>
    clientListInvitations(
        client: Handle
    ): Promise<Result<Array<InviteListItem>, ListInvitationsError>>
    clientListUserDevices(
        client: Handle,
        user: UserID
    ): Promise<Result<Array<DeviceInfo>, ClientListUserDevicesError>>
    clientListUsers(
        client: Handle,
        skip_revoked: boolean
    ): Promise<Result<Array<UserInfo>, ClientListUsersError>>
    clientListWorkspaceUsers(
        client: Handle,
        realm_id: VlobID
    ): Promise<Result<Array<WorkspaceUserAccessInfo>, ClientListWorkspaceUsersError>>
    clientListWorkspaces(
        client: Handle
    ): Promise<Result<Array<WorkspaceInfo>, ClientListWorkspacesError>>
    clientNewDeviceInvitation(
        client: Handle,
        send_email: boolean
    ): Promise<Result<NewInvitationInfo, ClientNewDeviceInvitationError>>
    clientNewUserInvitation(
        client: Handle,
        claimer_email: string,
        send_email: boolean
    ): Promise<Result<NewInvitationInfo, ClientNewUserInvitationError>>
    clientRenameWorkspace(
        client: Handle,
        realm_id: VlobID,
        new_name: EntryName
    ): Promise<Result<null, ClientRenameWorkspaceError>>
    clientRevokeUser(
        client: Handle,
        user: UserID
    ): Promise<Result<null, ClientRevokeUserError>>
    clientShareWorkspace(
        client: Handle,
        realm_id: VlobID,
        recipient: UserID,
        role: RealmRole | null
    ): Promise<Result<null, ClientShareWorkspaceError>>
    clientStart(
        config: ClientConfig,
        on_event_callback: (event: ClientEvent) => void,
        access: DeviceAccessStrategy
    ): Promise<Result<Handle, ClientStartError>>
    clientStartDeviceInvitationGreet(
        client: Handle,
        token: InvitationToken
    ): Promise<Result<DeviceGreetInitialInfo, ClientStartInvitationGreetError>>
    clientStartUserInvitationGreet(
        client: Handle,
        token: InvitationToken
    ): Promise<Result<UserGreetInitialInfo, ClientStartInvitationGreetError>>
    clientStartWorkspace(
        client: Handle,
        realm_id: VlobID
    ): Promise<Result<Handle, ClientStartWorkspaceError>>
    clientStop(
        client: Handle
    ): Promise<Result<null, ClientStopError>>
    fdClose(
        workspace: Handle,
        fd: FileDescriptor
    ): Promise<Result<null, WorkspaceFdCloseError>>
    fdFlush(
        workspace: Handle,
        fd: FileDescriptor
    ): Promise<Result<null, WorkspaceFdFlushError>>
    fdRead(
        workspace: Handle,
        fd: FileDescriptor,
        offset: U64,
        size: U64
    ): Promise<Result<Uint8Array, WorkspaceFdReadError>>
    fdResize(
        workspace: Handle,
        fd: FileDescriptor,
        length: U64,
        truncate_only: boolean
    ): Promise<Result<null, WorkspaceFdResizeError>>
    fdWrite(
        workspace: Handle,
        fd: FileDescriptor,
        offset: U64,
        data: Uint8Array
    ): Promise<Result<U64, WorkspaceFdWriteError>>
    fdWriteConstrainedIo(
        workspace: Handle,
        fd: FileDescriptor,
        offset: U64,
        data: Uint8Array
    ): Promise<Result<U64, WorkspaceFdWriteError>>
    fdWriteStartEof(
        workspace: Handle,
        fd: FileDescriptor,
        data: Uint8Array
    ): Promise<Result<U64, WorkspaceFdWriteError>>
    getDefaultConfigDir(
    ): Promise<Path>
    getDefaultDataBaseDir(
    ): Promise<Path>
    getDefaultMountpointBaseDir(
    ): Promise<Path>
    getPlatform(
    ): Promise<Platform>
    greeterDeviceInProgress1DoWaitPeerTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<DeviceGreetInProgress2Info, GreetInProgressError>>
    greeterDeviceInProgress2DoSignifyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<DeviceGreetInProgress3Info, GreetInProgressError>>
    greeterDeviceInProgress3DoGetClaimRequests(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<DeviceGreetInProgress4Info, GreetInProgressError>>
    greeterDeviceInProgress4DoCreate(
        canceller: Handle,
        handle: Handle,
        device_label: DeviceLabel
    ): Promise<Result<null, GreetInProgressError>>
    greeterDeviceInitialDoWaitPeer(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<DeviceGreetInProgress1Info, GreetInProgressError>>
    greeterUserInProgress1DoWaitPeerTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<UserGreetInProgress2Info, GreetInProgressError>>
    greeterUserInProgress2DoSignifyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<UserGreetInProgress3Info, GreetInProgressError>>
    greeterUserInProgress3DoGetClaimRequests(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<UserGreetInProgress4Info, GreetInProgressError>>
    greeterUserInProgress4DoCreate(
        canceller: Handle,
        handle: Handle,
        human_handle: HumanHandle,
        device_label: DeviceLabel,
        profile: UserProfile
    ): Promise<Result<null, GreetInProgressError>>
    greeterUserInitialDoWaitPeer(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<UserGreetInProgress1Info, GreetInProgressError>>
    isKeyringAvailable(
    ): Promise<boolean>
    listAvailableDevices(
        path: Path
    ): Promise<Array<AvailableDevice>>
    mountpointToOsPath(
        mountpoint: Handle,
        parsec_path: FsPath
    ): Promise<Result<Path, MountpointToOsPathError>>
    mountpointUnmount(
        mountpoint: Handle
    ): Promise<Result<null, MountpointUnmountError>>
    newCanceller(
    ): Promise<Handle>
    parseBackendAddr(
        url: string
    ): Promise<Result<ParsedParsecAddr, ParseBackendAddrError>>
    pathFilename(
        path: FsPath
    ): Promise<EntryName | null>
    pathJoin(
        parent: FsPath,
        child: EntryName
    ): Promise<FsPath>
    pathNormalize(
        path: FsPath
    ): Promise<FsPath>
    pathParent(
        path: FsPath
    ): Promise<FsPath>
    pathSplit(
        path: FsPath
    ): Promise<Array<EntryName>>
    testDropTestbed(
        path: Path
    ): Promise<Result<null, TestbedError>>
    testGetTestbedBootstrapOrganizationAddr(
        discriminant_dir: Path
    ): Promise<Result<ParsecOrganizationBootstrapAddr | null, TestbedError>>
    testGetTestbedOrganizationId(
        discriminant_dir: Path
    ): Promise<Result<OrganizationID | null, TestbedError>>
    testNewTestbed(
        template: string,
        test_server: ParsecAddr | null
    ): Promise<Result<Path, TestbedError>>
    validateDeviceLabel(
        raw: string
    ): Promise<boolean>
    validateEmail(
        raw: string
    ): Promise<boolean>
    validateEntryName(
        raw: string
    ): Promise<boolean>
    validateHumanHandleLabel(
        raw: string
    ): Promise<boolean>
    validateInvitationToken(
        raw: string
    ): Promise<boolean>
    validateOrganizationId(
        raw: string
    ): Promise<boolean>
    validatePath(
        raw: string
    ): Promise<boolean>
    workspaceCreateFile(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<VlobID, WorkspaceCreateFileError>>
    workspaceCreateFolder(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<VlobID, WorkspaceCreateFolderError>>
    workspaceCreateFolderAll(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<VlobID, WorkspaceCreateFolderError>>
    workspaceInfo(
        workspace: Handle
    ): Promise<Result<StartedWorkspaceInfo, WorkspaceInfoError>>
    workspaceMount(
        workspace: Handle
    ): Promise<Result<[Handle, Path], WorkspaceMountError>>
    workspaceOpenFile(
        workspace: Handle,
        path: FsPath,
        mode: OpenOptions
    ): Promise<Result<FileDescriptor, WorkspaceOpenFileError>>
    workspaceRemoveEntry(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<null, WorkspaceRemoveEntryError>>
    workspaceRemoveFile(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<null, WorkspaceRemoveEntryError>>
    workspaceRemoveFolder(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<null, WorkspaceRemoveEntryError>>
    workspaceRemoveFolderAll(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<null, WorkspaceRemoveEntryError>>
    workspaceRenameEntry(
        workspace: Handle,
        path: FsPath,
        new_name: EntryName,
        overwrite: boolean
    ): Promise<Result<null, WorkspaceRenameEntryError>>
    workspaceStatEntry(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<EntryStat, WorkspaceStatEntryError>>
    workspaceStop(
        workspace: Handle
    ): Promise<Result<null, WorkspaceStopError>>
}
