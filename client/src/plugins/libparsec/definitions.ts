// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */

export type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E }

export enum CancelledGreetingAttemptReason {
    AutomaticallyCancelled = 'CancelledGreetingAttemptReasonAutomaticallyCancelled',
    InconsistentPayload = 'CancelledGreetingAttemptReasonInconsistentPayload',
    InvalidNonceHash = 'CancelledGreetingAttemptReasonInvalidNonceHash',
    InvalidSasCode = 'CancelledGreetingAttemptReasonInvalidSasCode',
    ManuallyCancelled = 'CancelledGreetingAttemptReasonManuallyCancelled',
    UndecipherablePayload = 'CancelledGreetingAttemptReasonUndecipherablePayload',
    UndeserializablePayload = 'CancelledGreetingAttemptReasonUndeserializablePayload',
}

export enum DeviceFileType {
    Keyring = 'DeviceFileTypeKeyring',
    Password = 'DeviceFileTypePassword',
    Recovery = 'DeviceFileTypeRecovery',
    Smartcard = 'DeviceFileTypeSmartcard',
}

export enum DevicePurpose {
    PassphraseRecovery = 'DevicePurposePassphraseRecovery',
    ShamirRecovery = 'DevicePurposeShamirRecovery',
    Standard = 'DevicePurposeStandard',
    WebAuth = 'DevicePurposeWebAuth',
}

export enum GreeterOrClaimer {
    Claimer = 'GreeterOrClaimerClaimer',
    Greeter = 'GreeterOrClaimerGreeter',
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
export type ParsecPkiEnrollmentAddr = string
export type ParsecWorkspacePathAddr = string
export type Password = string
export type Path = string
export type SASCode = string
export type UserID = string
export type VlobID = string
export type SequesterVerifyKeyDer = Uint8Array
export type NonZeroU8 = number
export type U8 = number
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
    createdOn: DateTime
    protectedOn: DateTime
    serverUrl: string
    organizationId: OrganizationID
    userId: UserID
    deviceId: DeviceID
    humanHandle: HumanHandle
    deviceLabel: DeviceLabel
    ty: DeviceFileType
}

export interface ClientConfig {
    configDir: Path
    dataBaseDir: Path
    mountpointMountStrategy: MountpointMountStrategy
    workspaceStorageCacheSize: WorkspaceStorageCacheSize
    withMonitors: boolean
    preventSyncPattern: string | null
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
    purpose: DevicePurpose
    deviceLabel: DeviceLabel
    createdOn: DateTime
    createdBy: DeviceID | null
}

export interface FileStat {
    id: VlobID
    created: DateTime
    updated: DateTime
    baseVersion: VersionInt
    isPlaceholder: boolean
    needSync: boolean
    size: SizeInt
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

export interface ShamirRecoveryClaimInProgress1Info {
    handle: Handle
    greeterSas: SASCode
    greeterSasChoices: Array<SASCode>
}

export interface ShamirRecoveryClaimInProgress2Info {
    handle: Handle
    claimerSas: SASCode
}

export interface ShamirRecoveryClaimInProgress3Info {
    handle: Handle
}

export interface ShamirRecoveryClaimInitialInfo {
    handle: Handle
    greeterUserId: UserID
    greeterHumanHandle: HumanHandle
}

export interface ShamirRecoveryClaimShareInfo {
    handle: Handle
}

export interface ShamirRecoveryGreetInProgress1Info {
    handle: Handle
    greeterSas: SASCode
}

export interface ShamirRecoveryGreetInProgress2Info {
    handle: Handle
    claimerSas: SASCode
    claimerSasChoices: Array<SASCode>
}

export interface ShamirRecoveryGreetInProgress3Info {
    handle: Handle
}

export interface ShamirRecoveryGreetInitialInfo {
    handle: Handle
}

export interface ShamirRecoveryRecipient {
    userId: UserID
    humanHandle: HumanHandle
    revokedOn: DateTime | null
    shares: NonZeroU8
}

export interface StartedWorkspaceInfo {
    client: Handle
    id: VlobID
    currentName: EntryName
    currentSelfRole: RealmRole
    mountpoints: Array<[Handle, Path]>
}

export interface Tos {
    perLocaleUrls: Map<string, string>
    updatedOn: DateTime
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

export interface WorkspaceHistoryFileStat {
    id: VlobID
    created: DateTime
    updated: DateTime
    version: VersionInt
    size: SizeInt
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

// AnyClaimRetrievedInfo
export enum AnyClaimRetrievedInfoTag {
    Device = 'AnyClaimRetrievedInfoDevice',
    ShamirRecovery = 'AnyClaimRetrievedInfoShamirRecovery',
    User = 'AnyClaimRetrievedInfoUser',
}

export interface AnyClaimRetrievedInfoDevice {
    tag: AnyClaimRetrievedInfoTag.Device
    handle: Handle
    greeterUserId: UserID
    greeterHumanHandle: HumanHandle
}
export interface AnyClaimRetrievedInfoShamirRecovery {
    tag: AnyClaimRetrievedInfoTag.ShamirRecovery
    handle: Handle
    claimerUserId: UserID
    claimerHumanHandle: HumanHandle
    shamirRecoveryCreatedOn: DateTime
    recipients: Array<ShamirRecoveryRecipient>
    threshold: NonZeroU8
    isRecoverable: boolean
}
export interface AnyClaimRetrievedInfoUser {
    tag: AnyClaimRetrievedInfoTag.User
    handle: Handle
    claimerEmail: string
    greeterUserId: UserID
    greeterHumanHandle: HumanHandle
}
export type AnyClaimRetrievedInfo =
  | AnyClaimRetrievedInfoDevice
  | AnyClaimRetrievedInfoShamirRecovery
  | AnyClaimRetrievedInfoUser

// ArchiveDeviceError
export enum ArchiveDeviceErrorTag {
    Internal = 'ArchiveDeviceErrorInternal',
}

export interface ArchiveDeviceErrorInternal {
    tag: ArchiveDeviceErrorTag.Internal
    error: string
}
export type ArchiveDeviceError =
  | ArchiveDeviceErrorInternal

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
    AlreadyUsedOrDeleted = 'ClaimInProgressErrorAlreadyUsedOrDeleted',
    Cancelled = 'ClaimInProgressErrorCancelled',
    CorruptedConfirmation = 'ClaimInProgressErrorCorruptedConfirmation',
    GreeterNotAllowed = 'ClaimInProgressErrorGreeterNotAllowed',
    GreetingAttemptCancelled = 'ClaimInProgressErrorGreetingAttemptCancelled',
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
export interface ClaimInProgressErrorAlreadyUsedOrDeleted {
    tag: ClaimInProgressErrorTag.AlreadyUsedOrDeleted
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
export interface ClaimInProgressErrorGreeterNotAllowed {
    tag: ClaimInProgressErrorTag.GreeterNotAllowed
    error: string
}
export interface ClaimInProgressErrorGreetingAttemptCancelled {
    tag: ClaimInProgressErrorTag.GreetingAttemptCancelled
    error: string
    origin: GreeterOrClaimer
    reason: CancelledGreetingAttemptReason
    timestamp: DateTime
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
  | ClaimInProgressErrorAlreadyUsedOrDeleted
  | ClaimInProgressErrorCancelled
  | ClaimInProgressErrorCorruptedConfirmation
  | ClaimInProgressErrorGreeterNotAllowed
  | ClaimInProgressErrorGreetingAttemptCancelled
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
    AlreadyUsedOrDeleted = 'ClaimerRetrieveInfoErrorAlreadyUsedOrDeleted',
    Internal = 'ClaimerRetrieveInfoErrorInternal',
    NotFound = 'ClaimerRetrieveInfoErrorNotFound',
    Offline = 'ClaimerRetrieveInfoErrorOffline',
    OrganizationExpired = 'ClaimerRetrieveInfoErrorOrganizationExpired',
}

export interface ClaimerRetrieveInfoErrorAlreadyUsedOrDeleted {
    tag: ClaimerRetrieveInfoErrorTag.AlreadyUsedOrDeleted
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
export interface ClaimerRetrieveInfoErrorOrganizationExpired {
    tag: ClaimerRetrieveInfoErrorTag.OrganizationExpired
    error: string
}
export type ClaimerRetrieveInfoError =
  | ClaimerRetrieveInfoErrorAlreadyUsedOrDeleted
  | ClaimerRetrieveInfoErrorInternal
  | ClaimerRetrieveInfoErrorNotFound
  | ClaimerRetrieveInfoErrorOffline
  | ClaimerRetrieveInfoErrorOrganizationExpired

// ClientAcceptTosError
export enum ClientAcceptTosErrorTag {
    Internal = 'ClientAcceptTosErrorInternal',
    NoTos = 'ClientAcceptTosErrorNoTos',
    Offline = 'ClientAcceptTosErrorOffline',
    TosMismatch = 'ClientAcceptTosErrorTosMismatch',
}

export interface ClientAcceptTosErrorInternal {
    tag: ClientAcceptTosErrorTag.Internal
    error: string
}
export interface ClientAcceptTosErrorNoTos {
    tag: ClientAcceptTosErrorTag.NoTos
    error: string
}
export interface ClientAcceptTosErrorOffline {
    tag: ClientAcceptTosErrorTag.Offline
    error: string
}
export interface ClientAcceptTosErrorTosMismatch {
    tag: ClientAcceptTosErrorTag.TosMismatch
    error: string
}
export type ClientAcceptTosError =
  | ClientAcceptTosErrorInternal
  | ClientAcceptTosErrorNoTos
  | ClientAcceptTosErrorOffline
  | ClientAcceptTosErrorTosMismatch

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

// ClientDeleteShamirRecoveryError
export enum ClientDeleteShamirRecoveryErrorTag {
    Internal = 'ClientDeleteShamirRecoveryErrorInternal',
    InvalidCertificate = 'ClientDeleteShamirRecoveryErrorInvalidCertificate',
    Offline = 'ClientDeleteShamirRecoveryErrorOffline',
    Stopped = 'ClientDeleteShamirRecoveryErrorStopped',
    TimestampOutOfBallpark = 'ClientDeleteShamirRecoveryErrorTimestampOutOfBallpark',
}

export interface ClientDeleteShamirRecoveryErrorInternal {
    tag: ClientDeleteShamirRecoveryErrorTag.Internal
    error: string
}
export interface ClientDeleteShamirRecoveryErrorInvalidCertificate {
    tag: ClientDeleteShamirRecoveryErrorTag.InvalidCertificate
    error: string
}
export interface ClientDeleteShamirRecoveryErrorOffline {
    tag: ClientDeleteShamirRecoveryErrorTag.Offline
    error: string
}
export interface ClientDeleteShamirRecoveryErrorStopped {
    tag: ClientDeleteShamirRecoveryErrorTag.Stopped
    error: string
}
export interface ClientDeleteShamirRecoveryErrorTimestampOutOfBallpark {
    tag: ClientDeleteShamirRecoveryErrorTag.TimestampOutOfBallpark
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export type ClientDeleteShamirRecoveryError =
  | ClientDeleteShamirRecoveryErrorInternal
  | ClientDeleteShamirRecoveryErrorInvalidCertificate
  | ClientDeleteShamirRecoveryErrorOffline
  | ClientDeleteShamirRecoveryErrorStopped
  | ClientDeleteShamirRecoveryErrorTimestampOutOfBallpark

// ClientEvent
export enum ClientEventTag {
    ExpiredOrganization = 'ClientEventExpiredOrganization',
    IncompatibleServer = 'ClientEventIncompatibleServer',
    InvitationChanged = 'ClientEventInvitationChanged',
    MustAcceptTos = 'ClientEventMustAcceptTos',
    Offline = 'ClientEventOffline',
    Online = 'ClientEventOnline',
    Ping = 'ClientEventPing',
    RevokedSelfUser = 'ClientEventRevokedSelfUser',
    ServerConfigChanged = 'ClientEventServerConfigChanged',
    TooMuchDriftWithServerClock = 'ClientEventTooMuchDriftWithServerClock',
    WorkspaceLocallyCreated = 'ClientEventWorkspaceLocallyCreated',
    WorkspaceOpsInboundSyncDone = 'ClientEventWorkspaceOpsInboundSyncDone',
    WorkspaceOpsOutboundSyncAborted = 'ClientEventWorkspaceOpsOutboundSyncAborted',
    WorkspaceOpsOutboundSyncDone = 'ClientEventWorkspaceOpsOutboundSyncDone',
    WorkspaceOpsOutboundSyncProgress = 'ClientEventWorkspaceOpsOutboundSyncProgress',
    WorkspaceOpsOutboundSyncStarted = 'ClientEventWorkspaceOpsOutboundSyncStarted',
    WorkspaceWatchedEntryChanged = 'ClientEventWorkspaceWatchedEntryChanged',
    WorkspacesSelfListChanged = 'ClientEventWorkspacesSelfListChanged',
}

export interface ClientEventExpiredOrganization {
    tag: ClientEventTag.ExpiredOrganization
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
export interface ClientEventMustAcceptTos {
    tag: ClientEventTag.MustAcceptTos
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
export interface ClientEventRevokedSelfUser {
    tag: ClientEventTag.RevokedSelfUser
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
export interface ClientEventWorkspaceLocallyCreated {
    tag: ClientEventTag.WorkspaceLocallyCreated
}
export interface ClientEventWorkspaceOpsInboundSyncDone {
    tag: ClientEventTag.WorkspaceOpsInboundSyncDone
    realmId: VlobID
    entryId: VlobID
}
export interface ClientEventWorkspaceOpsOutboundSyncAborted {
    tag: ClientEventTag.WorkspaceOpsOutboundSyncAborted
    realmId: VlobID
    entryId: VlobID
}
export interface ClientEventWorkspaceOpsOutboundSyncDone {
    tag: ClientEventTag.WorkspaceOpsOutboundSyncDone
    realmId: VlobID
    entryId: VlobID
}
export interface ClientEventWorkspaceOpsOutboundSyncProgress {
    tag: ClientEventTag.WorkspaceOpsOutboundSyncProgress
    realmId: VlobID
    entryId: VlobID
    blocks: IndexInt
    blockIndex: IndexInt
    blocksize: SizeInt
}
export interface ClientEventWorkspaceOpsOutboundSyncStarted {
    tag: ClientEventTag.WorkspaceOpsOutboundSyncStarted
    realmId: VlobID
    entryId: VlobID
}
export interface ClientEventWorkspaceWatchedEntryChanged {
    tag: ClientEventTag.WorkspaceWatchedEntryChanged
    realmId: VlobID
    entryId: VlobID
}
export interface ClientEventWorkspacesSelfListChanged {
    tag: ClientEventTag.WorkspacesSelfListChanged
}
export type ClientEvent =
  | ClientEventExpiredOrganization
  | ClientEventIncompatibleServer
  | ClientEventInvitationChanged
  | ClientEventMustAcceptTos
  | ClientEventOffline
  | ClientEventOnline
  | ClientEventPing
  | ClientEventRevokedSelfUser
  | ClientEventServerConfigChanged
  | ClientEventTooMuchDriftWithServerClock
  | ClientEventWorkspaceLocallyCreated
  | ClientEventWorkspaceOpsInboundSyncDone
  | ClientEventWorkspaceOpsOutboundSyncAborted
  | ClientEventWorkspaceOpsOutboundSyncDone
  | ClientEventWorkspaceOpsOutboundSyncProgress
  | ClientEventWorkspaceOpsOutboundSyncStarted
  | ClientEventWorkspaceWatchedEntryChanged
  | ClientEventWorkspacesSelfListChanged

// ClientExportRecoveryDeviceError
export enum ClientExportRecoveryDeviceErrorTag {
    Internal = 'ClientExportRecoveryDeviceErrorInternal',
    InvalidCertificate = 'ClientExportRecoveryDeviceErrorInvalidCertificate',
    Offline = 'ClientExportRecoveryDeviceErrorOffline',
    Stopped = 'ClientExportRecoveryDeviceErrorStopped',
    TimestampOutOfBallpark = 'ClientExportRecoveryDeviceErrorTimestampOutOfBallpark',
}

export interface ClientExportRecoveryDeviceErrorInternal {
    tag: ClientExportRecoveryDeviceErrorTag.Internal
    error: string
}
export interface ClientExportRecoveryDeviceErrorInvalidCertificate {
    tag: ClientExportRecoveryDeviceErrorTag.InvalidCertificate
    error: string
}
export interface ClientExportRecoveryDeviceErrorOffline {
    tag: ClientExportRecoveryDeviceErrorTag.Offline
    error: string
}
export interface ClientExportRecoveryDeviceErrorStopped {
    tag: ClientExportRecoveryDeviceErrorTag.Stopped
    error: string
}
export interface ClientExportRecoveryDeviceErrorTimestampOutOfBallpark {
    tag: ClientExportRecoveryDeviceErrorTag.TimestampOutOfBallpark
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export type ClientExportRecoveryDeviceError =
  | ClientExportRecoveryDeviceErrorInternal
  | ClientExportRecoveryDeviceErrorInvalidCertificate
  | ClientExportRecoveryDeviceErrorOffline
  | ClientExportRecoveryDeviceErrorStopped
  | ClientExportRecoveryDeviceErrorTimestampOutOfBallpark

// ClientGetSelfShamirRecoveryError
export enum ClientGetSelfShamirRecoveryErrorTag {
    Internal = 'ClientGetSelfShamirRecoveryErrorInternal',
    Stopped = 'ClientGetSelfShamirRecoveryErrorStopped',
}

export interface ClientGetSelfShamirRecoveryErrorInternal {
    tag: ClientGetSelfShamirRecoveryErrorTag.Internal
    error: string
}
export interface ClientGetSelfShamirRecoveryErrorStopped {
    tag: ClientGetSelfShamirRecoveryErrorTag.Stopped
    error: string
}
export type ClientGetSelfShamirRecoveryError =
  | ClientGetSelfShamirRecoveryErrorInternal
  | ClientGetSelfShamirRecoveryErrorStopped

// ClientGetTosError
export enum ClientGetTosErrorTag {
    Internal = 'ClientGetTosErrorInternal',
    NoTos = 'ClientGetTosErrorNoTos',
    Offline = 'ClientGetTosErrorOffline',
}

export interface ClientGetTosErrorInternal {
    tag: ClientGetTosErrorTag.Internal
    error: string
}
export interface ClientGetTosErrorNoTos {
    tag: ClientGetTosErrorTag.NoTos
    error: string
}
export interface ClientGetTosErrorOffline {
    tag: ClientGetTosErrorTag.Offline
    error: string
}
export type ClientGetTosError =
  | ClientGetTosErrorInternal
  | ClientGetTosErrorNoTos
  | ClientGetTosErrorOffline

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

// ClientListFrozenUsersError
export enum ClientListFrozenUsersErrorTag {
    AuthorNotAllowed = 'ClientListFrozenUsersErrorAuthorNotAllowed',
    Internal = 'ClientListFrozenUsersErrorInternal',
    Offline = 'ClientListFrozenUsersErrorOffline',
}

export interface ClientListFrozenUsersErrorAuthorNotAllowed {
    tag: ClientListFrozenUsersErrorTag.AuthorNotAllowed
    error: string
}
export interface ClientListFrozenUsersErrorInternal {
    tag: ClientListFrozenUsersErrorTag.Internal
    error: string
}
export interface ClientListFrozenUsersErrorOffline {
    tag: ClientListFrozenUsersErrorTag.Offline
    error: string
}
export type ClientListFrozenUsersError =
  | ClientListFrozenUsersErrorAuthorNotAllowed
  | ClientListFrozenUsersErrorInternal
  | ClientListFrozenUsersErrorOffline

// ClientListShamirRecoveriesForOthersError
export enum ClientListShamirRecoveriesForOthersErrorTag {
    Internal = 'ClientListShamirRecoveriesForOthersErrorInternal',
    Stopped = 'ClientListShamirRecoveriesForOthersErrorStopped',
}

export interface ClientListShamirRecoveriesForOthersErrorInternal {
    tag: ClientListShamirRecoveriesForOthersErrorTag.Internal
    error: string
}
export interface ClientListShamirRecoveriesForOthersErrorStopped {
    tag: ClientListShamirRecoveriesForOthersErrorTag.Stopped
    error: string
}
export type ClientListShamirRecoveriesForOthersError =
  | ClientListShamirRecoveriesForOthersErrorInternal
  | ClientListShamirRecoveriesForOthersErrorStopped

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

// ClientNewShamirRecoveryInvitationError
export enum ClientNewShamirRecoveryInvitationErrorTag {
    Internal = 'ClientNewShamirRecoveryInvitationErrorInternal',
    NotAllowed = 'ClientNewShamirRecoveryInvitationErrorNotAllowed',
    Offline = 'ClientNewShamirRecoveryInvitationErrorOffline',
    UserNotFound = 'ClientNewShamirRecoveryInvitationErrorUserNotFound',
}

export interface ClientNewShamirRecoveryInvitationErrorInternal {
    tag: ClientNewShamirRecoveryInvitationErrorTag.Internal
    error: string
}
export interface ClientNewShamirRecoveryInvitationErrorNotAllowed {
    tag: ClientNewShamirRecoveryInvitationErrorTag.NotAllowed
    error: string
}
export interface ClientNewShamirRecoveryInvitationErrorOffline {
    tag: ClientNewShamirRecoveryInvitationErrorTag.Offline
    error: string
}
export interface ClientNewShamirRecoveryInvitationErrorUserNotFound {
    tag: ClientNewShamirRecoveryInvitationErrorTag.UserNotFound
    error: string
}
export type ClientNewShamirRecoveryInvitationError =
  | ClientNewShamirRecoveryInvitationErrorInternal
  | ClientNewShamirRecoveryInvitationErrorNotAllowed
  | ClientNewShamirRecoveryInvitationErrorOffline
  | ClientNewShamirRecoveryInvitationErrorUserNotFound

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

// ClientSetupShamirRecoveryError
export enum ClientSetupShamirRecoveryErrorTag {
    AuthorAmongRecipients = 'ClientSetupShamirRecoveryErrorAuthorAmongRecipients',
    Internal = 'ClientSetupShamirRecoveryErrorInternal',
    InvalidCertificate = 'ClientSetupShamirRecoveryErrorInvalidCertificate',
    Offline = 'ClientSetupShamirRecoveryErrorOffline',
    RecipientNotFound = 'ClientSetupShamirRecoveryErrorRecipientNotFound',
    RecipientRevoked = 'ClientSetupShamirRecoveryErrorRecipientRevoked',
    ShamirRecoveryAlreadyExists = 'ClientSetupShamirRecoveryErrorShamirRecoveryAlreadyExists',
    Stopped = 'ClientSetupShamirRecoveryErrorStopped',
    ThresholdBiggerThanSumOfShares = 'ClientSetupShamirRecoveryErrorThresholdBiggerThanSumOfShares',
    TimestampOutOfBallpark = 'ClientSetupShamirRecoveryErrorTimestampOutOfBallpark',
    TooManyShares = 'ClientSetupShamirRecoveryErrorTooManyShares',
}

export interface ClientSetupShamirRecoveryErrorAuthorAmongRecipients {
    tag: ClientSetupShamirRecoveryErrorTag.AuthorAmongRecipients
    error: string
}
export interface ClientSetupShamirRecoveryErrorInternal {
    tag: ClientSetupShamirRecoveryErrorTag.Internal
    error: string
}
export interface ClientSetupShamirRecoveryErrorInvalidCertificate {
    tag: ClientSetupShamirRecoveryErrorTag.InvalidCertificate
    error: string
}
export interface ClientSetupShamirRecoveryErrorOffline {
    tag: ClientSetupShamirRecoveryErrorTag.Offline
    error: string
}
export interface ClientSetupShamirRecoveryErrorRecipientNotFound {
    tag: ClientSetupShamirRecoveryErrorTag.RecipientNotFound
    error: string
}
export interface ClientSetupShamirRecoveryErrorRecipientRevoked {
    tag: ClientSetupShamirRecoveryErrorTag.RecipientRevoked
    error: string
}
export interface ClientSetupShamirRecoveryErrorShamirRecoveryAlreadyExists {
    tag: ClientSetupShamirRecoveryErrorTag.ShamirRecoveryAlreadyExists
    error: string
}
export interface ClientSetupShamirRecoveryErrorStopped {
    tag: ClientSetupShamirRecoveryErrorTag.Stopped
    error: string
}
export interface ClientSetupShamirRecoveryErrorThresholdBiggerThanSumOfShares {
    tag: ClientSetupShamirRecoveryErrorTag.ThresholdBiggerThanSumOfShares
    error: string
}
export interface ClientSetupShamirRecoveryErrorTimestampOutOfBallpark {
    tag: ClientSetupShamirRecoveryErrorTag.TimestampOutOfBallpark
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export interface ClientSetupShamirRecoveryErrorTooManyShares {
    tag: ClientSetupShamirRecoveryErrorTag.TooManyShares
    error: string
}
export type ClientSetupShamirRecoveryError =
  | ClientSetupShamirRecoveryErrorAuthorAmongRecipients
  | ClientSetupShamirRecoveryErrorInternal
  | ClientSetupShamirRecoveryErrorInvalidCertificate
  | ClientSetupShamirRecoveryErrorOffline
  | ClientSetupShamirRecoveryErrorRecipientNotFound
  | ClientSetupShamirRecoveryErrorRecipientRevoked
  | ClientSetupShamirRecoveryErrorShamirRecoveryAlreadyExists
  | ClientSetupShamirRecoveryErrorStopped
  | ClientSetupShamirRecoveryErrorThresholdBiggerThanSumOfShares
  | ClientSetupShamirRecoveryErrorTimestampOutOfBallpark
  | ClientSetupShamirRecoveryErrorTooManyShares

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
    DeviceUsedByAnotherProcess = 'ClientStartErrorDeviceUsedByAnotherProcess',
    Internal = 'ClientStartErrorInternal',
    LoadDeviceDecryptionFailed = 'ClientStartErrorLoadDeviceDecryptionFailed',
    LoadDeviceInvalidData = 'ClientStartErrorLoadDeviceInvalidData',
    LoadDeviceInvalidPath = 'ClientStartErrorLoadDeviceInvalidPath',
}

export interface ClientStartErrorDeviceUsedByAnotherProcess {
    tag: ClientStartErrorTag.DeviceUsedByAnotherProcess
    error: string
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
  | ClientStartErrorDeviceUsedByAnotherProcess
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

// ClientStartShamirRecoveryInvitationGreetError
export enum ClientStartShamirRecoveryInvitationGreetErrorTag {
    CorruptedShareData = 'ClientStartShamirRecoveryInvitationGreetErrorCorruptedShareData',
    Internal = 'ClientStartShamirRecoveryInvitationGreetErrorInternal',
    InvalidCertificate = 'ClientStartShamirRecoveryInvitationGreetErrorInvalidCertificate',
    InvitationNotFound = 'ClientStartShamirRecoveryInvitationGreetErrorInvitationNotFound',
    Offline = 'ClientStartShamirRecoveryInvitationGreetErrorOffline',
    ShamirRecoveryDeleted = 'ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryDeleted',
    ShamirRecoveryNotFound = 'ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryNotFound',
    ShamirRecoveryUnusable = 'ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryUnusable',
    Stopped = 'ClientStartShamirRecoveryInvitationGreetErrorStopped',
}

export interface ClientStartShamirRecoveryInvitationGreetErrorCorruptedShareData {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.CorruptedShareData
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorInternal {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.Internal
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorInvalidCertificate {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.InvalidCertificate
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorInvitationNotFound {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.InvitationNotFound
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorOffline {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.Offline
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryDeleted {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.ShamirRecoveryDeleted
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryNotFound {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.ShamirRecoveryNotFound
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryUnusable {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.ShamirRecoveryUnusable
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorStopped {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.Stopped
    error: string
}
export type ClientStartShamirRecoveryInvitationGreetError =
  | ClientStartShamirRecoveryInvitationGreetErrorCorruptedShareData
  | ClientStartShamirRecoveryInvitationGreetErrorInternal
  | ClientStartShamirRecoveryInvitationGreetErrorInvalidCertificate
  | ClientStartShamirRecoveryInvitationGreetErrorInvitationNotFound
  | ClientStartShamirRecoveryInvitationGreetErrorOffline
  | ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryDeleted
  | ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryNotFound
  | ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryUnusable
  | ClientStartShamirRecoveryInvitationGreetErrorStopped

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
    parent: VlobID
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
    parent: VlobID
    created: DateTime
    updated: DateTime
    baseVersion: VersionInt
    isPlaceholder: boolean
    needSync: boolean
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
    GreeterNotAllowed = 'GreetInProgressErrorGreeterNotAllowed',
    GreetingAttemptCancelled = 'GreetInProgressErrorGreetingAttemptCancelled',
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
export interface GreetInProgressErrorGreeterNotAllowed {
    tag: GreetInProgressErrorTag.GreeterNotAllowed
    error: string
}
export interface GreetInProgressErrorGreetingAttemptCancelled {
    tag: GreetInProgressErrorTag.GreetingAttemptCancelled
    error: string
    origin: GreeterOrClaimer
    reason: CancelledGreetingAttemptReason
    timestamp: DateTime
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
  | GreetInProgressErrorGreeterNotAllowed
  | GreetInProgressErrorGreetingAttemptCancelled
  | GreetInProgressErrorHumanHandleAlreadyTaken
  | GreetInProgressErrorInternal
  | GreetInProgressErrorNonceMismatch
  | GreetInProgressErrorNotFound
  | GreetInProgressErrorOffline
  | GreetInProgressErrorPeerReset
  | GreetInProgressErrorTimestampOutOfBallpark
  | GreetInProgressErrorUserAlreadyExists
  | GreetInProgressErrorUserCreateNotAllowed

// ImportRecoveryDeviceError
export enum ImportRecoveryDeviceErrorTag {
    DecryptionFailed = 'ImportRecoveryDeviceErrorDecryptionFailed',
    Internal = 'ImportRecoveryDeviceErrorInternal',
    InvalidCertificate = 'ImportRecoveryDeviceErrorInvalidCertificate',
    InvalidData = 'ImportRecoveryDeviceErrorInvalidData',
    InvalidPassphrase = 'ImportRecoveryDeviceErrorInvalidPassphrase',
    InvalidPath = 'ImportRecoveryDeviceErrorInvalidPath',
    Offline = 'ImportRecoveryDeviceErrorOffline',
    Stopped = 'ImportRecoveryDeviceErrorStopped',
    TimestampOutOfBallpark = 'ImportRecoveryDeviceErrorTimestampOutOfBallpark',
}

export interface ImportRecoveryDeviceErrorDecryptionFailed {
    tag: ImportRecoveryDeviceErrorTag.DecryptionFailed
    error: string
}
export interface ImportRecoveryDeviceErrorInternal {
    tag: ImportRecoveryDeviceErrorTag.Internal
    error: string
}
export interface ImportRecoveryDeviceErrorInvalidCertificate {
    tag: ImportRecoveryDeviceErrorTag.InvalidCertificate
    error: string
}
export interface ImportRecoveryDeviceErrorInvalidData {
    tag: ImportRecoveryDeviceErrorTag.InvalidData
    error: string
}
export interface ImportRecoveryDeviceErrorInvalidPassphrase {
    tag: ImportRecoveryDeviceErrorTag.InvalidPassphrase
    error: string
}
export interface ImportRecoveryDeviceErrorInvalidPath {
    tag: ImportRecoveryDeviceErrorTag.InvalidPath
    error: string
}
export interface ImportRecoveryDeviceErrorOffline {
    tag: ImportRecoveryDeviceErrorTag.Offline
    error: string
}
export interface ImportRecoveryDeviceErrorStopped {
    tag: ImportRecoveryDeviceErrorTag.Stopped
    error: string
}
export interface ImportRecoveryDeviceErrorTimestampOutOfBallpark {
    tag: ImportRecoveryDeviceErrorTag.TimestampOutOfBallpark
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export type ImportRecoveryDeviceError =
  | ImportRecoveryDeviceErrorDecryptionFailed
  | ImportRecoveryDeviceErrorInternal
  | ImportRecoveryDeviceErrorInvalidCertificate
  | ImportRecoveryDeviceErrorInvalidData
  | ImportRecoveryDeviceErrorInvalidPassphrase
  | ImportRecoveryDeviceErrorInvalidPath
  | ImportRecoveryDeviceErrorOffline
  | ImportRecoveryDeviceErrorStopped
  | ImportRecoveryDeviceErrorTimestampOutOfBallpark

// InviteListItem
export enum InviteListItemTag {
    Device = 'InviteListItemDevice',
    ShamirRecovery = 'InviteListItemShamirRecovery',
    User = 'InviteListItemUser',
}

export interface InviteListItemDevice {
    tag: InviteListItemTag.Device
    addr: ParsecInvitationAddr
    token: InvitationToken
    createdOn: DateTime
    status: InvitationStatus
}
export interface InviteListItemShamirRecovery {
    tag: InviteListItemTag.ShamirRecovery
    addr: ParsecInvitationAddr
    token: InvitationToken
    createdOn: DateTime
    claimerUserId: UserID
    shamirRecoveryCreatedOn: DateTime
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
  | InviteListItemShamirRecovery
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

// MoveEntryMode
export enum MoveEntryModeTag {
    CanReplace = 'MoveEntryModeCanReplace',
    CanReplaceFileOnly = 'MoveEntryModeCanReplaceFileOnly',
    Exchange = 'MoveEntryModeExchange',
    NoReplace = 'MoveEntryModeNoReplace',
}

export interface MoveEntryModeCanReplace {
    tag: MoveEntryModeTag.CanReplace
}
export interface MoveEntryModeCanReplaceFileOnly {
    tag: MoveEntryModeTag.CanReplaceFileOnly
}
export interface MoveEntryModeExchange {
    tag: MoveEntryModeTag.Exchange
}
export interface MoveEntryModeNoReplace {
    tag: MoveEntryModeTag.NoReplace
}
export type MoveEntryMode =
  | MoveEntryModeCanReplace
  | MoveEntryModeCanReplaceFileOnly
  | MoveEntryModeExchange
  | MoveEntryModeNoReplace

// OtherShamirRecoveryInfo
export enum OtherShamirRecoveryInfoTag {
    Deleted = 'OtherShamirRecoveryInfoDeleted',
    SetupAllValid = 'OtherShamirRecoveryInfoSetupAllValid',
    SetupButUnusable = 'OtherShamirRecoveryInfoSetupButUnusable',
    SetupWithRevokedRecipients = 'OtherShamirRecoveryInfoSetupWithRevokedRecipients',
}

export interface OtherShamirRecoveryInfoDeleted {
    tag: OtherShamirRecoveryInfoTag.Deleted
    userId: UserID
    createdOn: DateTime
    createdBy: DeviceID
    threshold: NonZeroU8
    perRecipientShares: Map<UserID, NonZeroU8>
    deletedOn: DateTime
    deletedBy: DeviceID
}
export interface OtherShamirRecoveryInfoSetupAllValid {
    tag: OtherShamirRecoveryInfoTag.SetupAllValid
    userId: UserID
    createdOn: DateTime
    createdBy: DeviceID
    threshold: NonZeroU8
    perRecipientShares: Map<UserID, NonZeroU8>
}
export interface OtherShamirRecoveryInfoSetupButUnusable {
    tag: OtherShamirRecoveryInfoTag.SetupButUnusable
    userId: UserID
    createdOn: DateTime
    createdBy: DeviceID
    threshold: NonZeroU8
    perRecipientShares: Map<UserID, NonZeroU8>
    revokedRecipients: Array<UserID>
}
export interface OtherShamirRecoveryInfoSetupWithRevokedRecipients {
    tag: OtherShamirRecoveryInfoTag.SetupWithRevokedRecipients
    userId: UserID
    createdOn: DateTime
    createdBy: DeviceID
    threshold: NonZeroU8
    perRecipientShares: Map<UserID, NonZeroU8>
    revokedRecipients: Array<UserID>
}
export type OtherShamirRecoveryInfo =
  | OtherShamirRecoveryInfoDeleted
  | OtherShamirRecoveryInfoSetupAllValid
  | OtherShamirRecoveryInfoSetupButUnusable
  | OtherShamirRecoveryInfoSetupWithRevokedRecipients

// ParseParsecAddrError
export enum ParseParsecAddrErrorTag {
    InvalidUrl = 'ParseParsecAddrErrorInvalidUrl',
}

export interface ParseParsecAddrErrorInvalidUrl {
    tag: ParseParsecAddrErrorTag.InvalidUrl
    error: string
}
export type ParseParsecAddrError =
  | ParseParsecAddrErrorInvalidUrl

// ParsedParsecAddr
export enum ParsedParsecAddrTag {
    InvitationDevice = 'ParsedParsecAddrInvitationDevice',
    InvitationShamirRecovery = 'ParsedParsecAddrInvitationShamirRecovery',
    InvitationUser = 'ParsedParsecAddrInvitationUser',
    Organization = 'ParsedParsecAddrOrganization',
    OrganizationBootstrap = 'ParsedParsecAddrOrganizationBootstrap',
    PkiEnrollment = 'ParsedParsecAddrPkiEnrollment',
    Server = 'ParsedParsecAddrServer',
    WorkspacePath = 'ParsedParsecAddrWorkspacePath',
}

export interface ParsedParsecAddrInvitationDevice {
    tag: ParsedParsecAddrTag.InvitationDevice
    hostname: string
    port: U32
    useSsl: boolean
    organizationId: OrganizationID
    token: InvitationToken
}
export interface ParsedParsecAddrInvitationShamirRecovery {
    tag: ParsedParsecAddrTag.InvitationShamirRecovery
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
export interface ParsedParsecAddrWorkspacePath {
    tag: ParsedParsecAddrTag.WorkspacePath
    hostname: string
    port: U32
    useSsl: boolean
    organizationId: OrganizationID
    workspaceId: VlobID
    keyIndex: IndexInt
    encryptedPath: Uint8Array
}
export type ParsedParsecAddr =
  | ParsedParsecAddrInvitationDevice
  | ParsedParsecAddrInvitationShamirRecovery
  | ParsedParsecAddrInvitationUser
  | ParsedParsecAddrOrganization
  | ParsedParsecAddrOrganizationBootstrap
  | ParsedParsecAddrPkiEnrollment
  | ParsedParsecAddrServer
  | ParsedParsecAddrWorkspacePath

// SelfShamirRecoveryInfo
export enum SelfShamirRecoveryInfoTag {
    Deleted = 'SelfShamirRecoveryInfoDeleted',
    NeverSetup = 'SelfShamirRecoveryInfoNeverSetup',
    SetupAllValid = 'SelfShamirRecoveryInfoSetupAllValid',
    SetupButUnusable = 'SelfShamirRecoveryInfoSetupButUnusable',
    SetupWithRevokedRecipients = 'SelfShamirRecoveryInfoSetupWithRevokedRecipients',
}

export interface SelfShamirRecoveryInfoDeleted {
    tag: SelfShamirRecoveryInfoTag.Deleted
    createdOn: DateTime
    createdBy: DeviceID
    threshold: NonZeroU8
    perRecipientShares: Map<UserID, NonZeroU8>
    deletedOn: DateTime
    deletedBy: DeviceID
}
export interface SelfShamirRecoveryInfoNeverSetup {
    tag: SelfShamirRecoveryInfoTag.NeverSetup
}
export interface SelfShamirRecoveryInfoSetupAllValid {
    tag: SelfShamirRecoveryInfoTag.SetupAllValid
    createdOn: DateTime
    createdBy: DeviceID
    threshold: NonZeroU8
    perRecipientShares: Map<UserID, NonZeroU8>
}
export interface SelfShamirRecoveryInfoSetupButUnusable {
    tag: SelfShamirRecoveryInfoTag.SetupButUnusable
    createdOn: DateTime
    createdBy: DeviceID
    threshold: NonZeroU8
    perRecipientShares: Map<UserID, NonZeroU8>
    revokedRecipients: Array<UserID>
}
export interface SelfShamirRecoveryInfoSetupWithRevokedRecipients {
    tag: SelfShamirRecoveryInfoTag.SetupWithRevokedRecipients
    createdOn: DateTime
    createdBy: DeviceID
    threshold: NonZeroU8
    perRecipientShares: Map<UserID, NonZeroU8>
    revokedRecipients: Array<UserID>
}
export type SelfShamirRecoveryInfo =
  | SelfShamirRecoveryInfoDeleted
  | SelfShamirRecoveryInfoNeverSetup
  | SelfShamirRecoveryInfoSetupAllValid
  | SelfShamirRecoveryInfoSetupButUnusable
  | SelfShamirRecoveryInfoSetupWithRevokedRecipients

// ShamirRecoveryClaimAddShareError
export enum ShamirRecoveryClaimAddShareErrorTag {
    CorruptedSecret = 'ShamirRecoveryClaimAddShareErrorCorruptedSecret',
    Internal = 'ShamirRecoveryClaimAddShareErrorInternal',
    RecipientNotFound = 'ShamirRecoveryClaimAddShareErrorRecipientNotFound',
}

export interface ShamirRecoveryClaimAddShareErrorCorruptedSecret {
    tag: ShamirRecoveryClaimAddShareErrorTag.CorruptedSecret
    error: string
}
export interface ShamirRecoveryClaimAddShareErrorInternal {
    tag: ShamirRecoveryClaimAddShareErrorTag.Internal
    error: string
}
export interface ShamirRecoveryClaimAddShareErrorRecipientNotFound {
    tag: ShamirRecoveryClaimAddShareErrorTag.RecipientNotFound
    error: string
}
export type ShamirRecoveryClaimAddShareError =
  | ShamirRecoveryClaimAddShareErrorCorruptedSecret
  | ShamirRecoveryClaimAddShareErrorInternal
  | ShamirRecoveryClaimAddShareErrorRecipientNotFound

// ShamirRecoveryClaimMaybeFinalizeInfo
export enum ShamirRecoveryClaimMaybeFinalizeInfoTag {
    Finalize = 'ShamirRecoveryClaimMaybeFinalizeInfoFinalize',
    Offline = 'ShamirRecoveryClaimMaybeFinalizeInfoOffline',
}

export interface ShamirRecoveryClaimMaybeFinalizeInfoFinalize {
    tag: ShamirRecoveryClaimMaybeFinalizeInfoTag.Finalize
    handle: Handle
}
export interface ShamirRecoveryClaimMaybeFinalizeInfoOffline {
    tag: ShamirRecoveryClaimMaybeFinalizeInfoTag.Offline
    handle: Handle
}
export type ShamirRecoveryClaimMaybeFinalizeInfo =
  | ShamirRecoveryClaimMaybeFinalizeInfoFinalize
  | ShamirRecoveryClaimMaybeFinalizeInfoOffline

// ShamirRecoveryClaimMaybeRecoverDeviceInfo
export enum ShamirRecoveryClaimMaybeRecoverDeviceInfoTag {
    PickRecipient = 'ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient',
    RecoverDevice = 'ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice',
}

export interface ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient {
    tag: ShamirRecoveryClaimMaybeRecoverDeviceInfoTag.PickRecipient
    handle: Handle
    claimerUserId: UserID
    claimerHumanHandle: HumanHandle
    shamirRecoveryCreatedOn: DateTime
    recipients: Array<ShamirRecoveryRecipient>
    threshold: NonZeroU8
    recoveredShares: Map<UserID, NonZeroU8>
    isRecoverable: boolean
}
export interface ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice {
    tag: ShamirRecoveryClaimMaybeRecoverDeviceInfoTag.RecoverDevice
    handle: Handle
    claimerUserId: UserID
    claimerHumanHandle: HumanHandle
}
export type ShamirRecoveryClaimMaybeRecoverDeviceInfo =
  | ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient
  | ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice

// ShamirRecoveryClaimPickRecipientError
export enum ShamirRecoveryClaimPickRecipientErrorTag {
    Internal = 'ShamirRecoveryClaimPickRecipientErrorInternal',
    RecipientAlreadyPicked = 'ShamirRecoveryClaimPickRecipientErrorRecipientAlreadyPicked',
    RecipientNotFound = 'ShamirRecoveryClaimPickRecipientErrorRecipientNotFound',
    RecipientRevoked = 'ShamirRecoveryClaimPickRecipientErrorRecipientRevoked',
}

export interface ShamirRecoveryClaimPickRecipientErrorInternal {
    tag: ShamirRecoveryClaimPickRecipientErrorTag.Internal
    error: string
}
export interface ShamirRecoveryClaimPickRecipientErrorRecipientAlreadyPicked {
    tag: ShamirRecoveryClaimPickRecipientErrorTag.RecipientAlreadyPicked
    error: string
}
export interface ShamirRecoveryClaimPickRecipientErrorRecipientNotFound {
    tag: ShamirRecoveryClaimPickRecipientErrorTag.RecipientNotFound
    error: string
}
export interface ShamirRecoveryClaimPickRecipientErrorRecipientRevoked {
    tag: ShamirRecoveryClaimPickRecipientErrorTag.RecipientRevoked
    error: string
}
export type ShamirRecoveryClaimPickRecipientError =
  | ShamirRecoveryClaimPickRecipientErrorInternal
  | ShamirRecoveryClaimPickRecipientErrorRecipientAlreadyPicked
  | ShamirRecoveryClaimPickRecipientErrorRecipientNotFound
  | ShamirRecoveryClaimPickRecipientErrorRecipientRevoked

// ShamirRecoveryClaimRecoverDeviceError
export enum ShamirRecoveryClaimRecoverDeviceErrorTag {
    AlreadyUsed = 'ShamirRecoveryClaimRecoverDeviceErrorAlreadyUsed',
    CipheredDataNotFound = 'ShamirRecoveryClaimRecoverDeviceErrorCipheredDataNotFound',
    CorruptedCipheredData = 'ShamirRecoveryClaimRecoverDeviceErrorCorruptedCipheredData',
    Internal = 'ShamirRecoveryClaimRecoverDeviceErrorInternal',
    NotFound = 'ShamirRecoveryClaimRecoverDeviceErrorNotFound',
    OrganizationExpired = 'ShamirRecoveryClaimRecoverDeviceErrorOrganizationExpired',
    RegisterNewDeviceError = 'ShamirRecoveryClaimRecoverDeviceErrorRegisterNewDeviceError',
}

export interface ShamirRecoveryClaimRecoverDeviceErrorAlreadyUsed {
    tag: ShamirRecoveryClaimRecoverDeviceErrorTag.AlreadyUsed
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorCipheredDataNotFound {
    tag: ShamirRecoveryClaimRecoverDeviceErrorTag.CipheredDataNotFound
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorCorruptedCipheredData {
    tag: ShamirRecoveryClaimRecoverDeviceErrorTag.CorruptedCipheredData
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorInternal {
    tag: ShamirRecoveryClaimRecoverDeviceErrorTag.Internal
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorNotFound {
    tag: ShamirRecoveryClaimRecoverDeviceErrorTag.NotFound
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorOrganizationExpired {
    tag: ShamirRecoveryClaimRecoverDeviceErrorTag.OrganizationExpired
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorRegisterNewDeviceError {
    tag: ShamirRecoveryClaimRecoverDeviceErrorTag.RegisterNewDeviceError
    error: string
}
export type ShamirRecoveryClaimRecoverDeviceError =
  | ShamirRecoveryClaimRecoverDeviceErrorAlreadyUsed
  | ShamirRecoveryClaimRecoverDeviceErrorCipheredDataNotFound
  | ShamirRecoveryClaimRecoverDeviceErrorCorruptedCipheredData
  | ShamirRecoveryClaimRecoverDeviceErrorInternal
  | ShamirRecoveryClaimRecoverDeviceErrorNotFound
  | ShamirRecoveryClaimRecoverDeviceErrorOrganizationExpired
  | ShamirRecoveryClaimRecoverDeviceErrorRegisterNewDeviceError

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

// WaitForDeviceAvailableError
export enum WaitForDeviceAvailableErrorTag {
    Internal = 'WaitForDeviceAvailableErrorInternal',
}

export interface WaitForDeviceAvailableErrorInternal {
    tag: WaitForDeviceAvailableErrorTag.Internal
    error: string
}
export type WaitForDeviceAvailableError =
  | WaitForDeviceAvailableErrorInternal

// WorkspaceCreateFileError
export enum WorkspaceCreateFileErrorTag {
    EntryExists = 'WorkspaceCreateFileErrorEntryExists',
    Internal = 'WorkspaceCreateFileErrorInternal',
    InvalidCertificate = 'WorkspaceCreateFileErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceCreateFileErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceCreateFileErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceCreateFileErrorNoRealmAccess',
    Offline = 'WorkspaceCreateFileErrorOffline',
    ParentNotAFolder = 'WorkspaceCreateFileErrorParentNotAFolder',
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
export interface WorkspaceCreateFileErrorParentNotAFolder {
    tag: WorkspaceCreateFileErrorTag.ParentNotAFolder
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
  | WorkspaceCreateFileErrorParentNotAFolder
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
    ParentNotAFolder = 'WorkspaceCreateFolderErrorParentNotAFolder',
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
export interface WorkspaceCreateFolderErrorParentNotAFolder {
    tag: WorkspaceCreateFolderErrorTag.ParentNotAFolder
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
  | WorkspaceCreateFolderErrorParentNotAFolder
  | WorkspaceCreateFolderErrorParentNotFound
  | WorkspaceCreateFolderErrorReadOnlyRealm
  | WorkspaceCreateFolderErrorStopped

// WorkspaceDecryptPathAddrError
export enum WorkspaceDecryptPathAddrErrorTag {
    CorruptedData = 'WorkspaceDecryptPathAddrErrorCorruptedData',
    CorruptedKey = 'WorkspaceDecryptPathAddrErrorCorruptedKey',
    Internal = 'WorkspaceDecryptPathAddrErrorInternal',
    InvalidCertificate = 'WorkspaceDecryptPathAddrErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceDecryptPathAddrErrorInvalidKeysBundle',
    KeyNotFound = 'WorkspaceDecryptPathAddrErrorKeyNotFound',
    NotAllowed = 'WorkspaceDecryptPathAddrErrorNotAllowed',
    Offline = 'WorkspaceDecryptPathAddrErrorOffline',
    Stopped = 'WorkspaceDecryptPathAddrErrorStopped',
}

export interface WorkspaceDecryptPathAddrErrorCorruptedData {
    tag: WorkspaceDecryptPathAddrErrorTag.CorruptedData
    error: string
}
export interface WorkspaceDecryptPathAddrErrorCorruptedKey {
    tag: WorkspaceDecryptPathAddrErrorTag.CorruptedKey
    error: string
}
export interface WorkspaceDecryptPathAddrErrorInternal {
    tag: WorkspaceDecryptPathAddrErrorTag.Internal
    error: string
}
export interface WorkspaceDecryptPathAddrErrorInvalidCertificate {
    tag: WorkspaceDecryptPathAddrErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceDecryptPathAddrErrorInvalidKeysBundle {
    tag: WorkspaceDecryptPathAddrErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceDecryptPathAddrErrorKeyNotFound {
    tag: WorkspaceDecryptPathAddrErrorTag.KeyNotFound
    error: string
}
export interface WorkspaceDecryptPathAddrErrorNotAllowed {
    tag: WorkspaceDecryptPathAddrErrorTag.NotAllowed
    error: string
}
export interface WorkspaceDecryptPathAddrErrorOffline {
    tag: WorkspaceDecryptPathAddrErrorTag.Offline
    error: string
}
export interface WorkspaceDecryptPathAddrErrorStopped {
    tag: WorkspaceDecryptPathAddrErrorTag.Stopped
    error: string
}
export type WorkspaceDecryptPathAddrError =
  | WorkspaceDecryptPathAddrErrorCorruptedData
  | WorkspaceDecryptPathAddrErrorCorruptedKey
  | WorkspaceDecryptPathAddrErrorInternal
  | WorkspaceDecryptPathAddrErrorInvalidCertificate
  | WorkspaceDecryptPathAddrErrorInvalidKeysBundle
  | WorkspaceDecryptPathAddrErrorKeyNotFound
  | WorkspaceDecryptPathAddrErrorNotAllowed
  | WorkspaceDecryptPathAddrErrorOffline
  | WorkspaceDecryptPathAddrErrorStopped

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

// WorkspaceFdStatError
export enum WorkspaceFdStatErrorTag {
    BadFileDescriptor = 'WorkspaceFdStatErrorBadFileDescriptor',
    Internal = 'WorkspaceFdStatErrorInternal',
}

export interface WorkspaceFdStatErrorBadFileDescriptor {
    tag: WorkspaceFdStatErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceFdStatErrorInternal {
    tag: WorkspaceFdStatErrorTag.Internal
    error: string
}
export type WorkspaceFdStatError =
  | WorkspaceFdStatErrorBadFileDescriptor
  | WorkspaceFdStatErrorInternal

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

// WorkspaceGeneratePathAddrError
export enum WorkspaceGeneratePathAddrErrorTag {
    Internal = 'WorkspaceGeneratePathAddrErrorInternal',
    InvalidKeysBundle = 'WorkspaceGeneratePathAddrErrorInvalidKeysBundle',
    NoKey = 'WorkspaceGeneratePathAddrErrorNoKey',
    NotAllowed = 'WorkspaceGeneratePathAddrErrorNotAllowed',
    Offline = 'WorkspaceGeneratePathAddrErrorOffline',
    Stopped = 'WorkspaceGeneratePathAddrErrorStopped',
}

export interface WorkspaceGeneratePathAddrErrorInternal {
    tag: WorkspaceGeneratePathAddrErrorTag.Internal
    error: string
}
export interface WorkspaceGeneratePathAddrErrorInvalidKeysBundle {
    tag: WorkspaceGeneratePathAddrErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceGeneratePathAddrErrorNoKey {
    tag: WorkspaceGeneratePathAddrErrorTag.NoKey
    error: string
}
export interface WorkspaceGeneratePathAddrErrorNotAllowed {
    tag: WorkspaceGeneratePathAddrErrorTag.NotAllowed
    error: string
}
export interface WorkspaceGeneratePathAddrErrorOffline {
    tag: WorkspaceGeneratePathAddrErrorTag.Offline
    error: string
}
export interface WorkspaceGeneratePathAddrErrorStopped {
    tag: WorkspaceGeneratePathAddrErrorTag.Stopped
    error: string
}
export type WorkspaceGeneratePathAddrError =
  | WorkspaceGeneratePathAddrErrorInternal
  | WorkspaceGeneratePathAddrErrorInvalidKeysBundle
  | WorkspaceGeneratePathAddrErrorNoKey
  | WorkspaceGeneratePathAddrErrorNotAllowed
  | WorkspaceGeneratePathAddrErrorOffline
  | WorkspaceGeneratePathAddrErrorStopped

// WorkspaceHistoryEntryStat
export enum WorkspaceHistoryEntryStatTag {
    File = 'WorkspaceHistoryEntryStatFile',
    Folder = 'WorkspaceHistoryEntryStatFolder',
}

export interface WorkspaceHistoryEntryStatFile {
    tag: WorkspaceHistoryEntryStatTag.File
    id: VlobID
    parent: VlobID
    created: DateTime
    updated: DateTime
    version: VersionInt
    size: SizeInt
}
export interface WorkspaceHistoryEntryStatFolder {
    tag: WorkspaceHistoryEntryStatTag.Folder
    id: VlobID
    parent: VlobID
    created: DateTime
    updated: DateTime
    version: VersionInt
}
export type WorkspaceHistoryEntryStat =
  | WorkspaceHistoryEntryStatFile
  | WorkspaceHistoryEntryStatFolder

// WorkspaceHistoryFdCloseError
export enum WorkspaceHistoryFdCloseErrorTag {
    BadFileDescriptor = 'WorkspaceHistoryFdCloseErrorBadFileDescriptor',
    Internal = 'WorkspaceHistoryFdCloseErrorInternal',
}

export interface WorkspaceHistoryFdCloseErrorBadFileDescriptor {
    tag: WorkspaceHistoryFdCloseErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceHistoryFdCloseErrorInternal {
    tag: WorkspaceHistoryFdCloseErrorTag.Internal
    error: string
}
export type WorkspaceHistoryFdCloseError =
  | WorkspaceHistoryFdCloseErrorBadFileDescriptor
  | WorkspaceHistoryFdCloseErrorInternal

// WorkspaceHistoryFdReadError
export enum WorkspaceHistoryFdReadErrorTag {
    BadFileDescriptor = 'WorkspaceHistoryFdReadErrorBadFileDescriptor',
    BlockNotFound = 'WorkspaceHistoryFdReadErrorBlockNotFound',
    Internal = 'WorkspaceHistoryFdReadErrorInternal',
    InvalidBlockAccess = 'WorkspaceHistoryFdReadErrorInvalidBlockAccess',
    InvalidCertificate = 'WorkspaceHistoryFdReadErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceHistoryFdReadErrorInvalidKeysBundle',
    NoRealmAccess = 'WorkspaceHistoryFdReadErrorNoRealmAccess',
    Offline = 'WorkspaceHistoryFdReadErrorOffline',
    Stopped = 'WorkspaceHistoryFdReadErrorStopped',
}

export interface WorkspaceHistoryFdReadErrorBadFileDescriptor {
    tag: WorkspaceHistoryFdReadErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceHistoryFdReadErrorBlockNotFound {
    tag: WorkspaceHistoryFdReadErrorTag.BlockNotFound
    error: string
}
export interface WorkspaceHistoryFdReadErrorInternal {
    tag: WorkspaceHistoryFdReadErrorTag.Internal
    error: string
}
export interface WorkspaceHistoryFdReadErrorInvalidBlockAccess {
    tag: WorkspaceHistoryFdReadErrorTag.InvalidBlockAccess
    error: string
}
export interface WorkspaceHistoryFdReadErrorInvalidCertificate {
    tag: WorkspaceHistoryFdReadErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceHistoryFdReadErrorInvalidKeysBundle {
    tag: WorkspaceHistoryFdReadErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceHistoryFdReadErrorNoRealmAccess {
    tag: WorkspaceHistoryFdReadErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceHistoryFdReadErrorOffline {
    tag: WorkspaceHistoryFdReadErrorTag.Offline
    error: string
}
export interface WorkspaceHistoryFdReadErrorStopped {
    tag: WorkspaceHistoryFdReadErrorTag.Stopped
    error: string
}
export type WorkspaceHistoryFdReadError =
  | WorkspaceHistoryFdReadErrorBadFileDescriptor
  | WorkspaceHistoryFdReadErrorBlockNotFound
  | WorkspaceHistoryFdReadErrorInternal
  | WorkspaceHistoryFdReadErrorInvalidBlockAccess
  | WorkspaceHistoryFdReadErrorInvalidCertificate
  | WorkspaceHistoryFdReadErrorInvalidKeysBundle
  | WorkspaceHistoryFdReadErrorNoRealmAccess
  | WorkspaceHistoryFdReadErrorOffline
  | WorkspaceHistoryFdReadErrorStopped

// WorkspaceHistoryFdStatError
export enum WorkspaceHistoryFdStatErrorTag {
    BadFileDescriptor = 'WorkspaceHistoryFdStatErrorBadFileDescriptor',
    Internal = 'WorkspaceHistoryFdStatErrorInternal',
}

export interface WorkspaceHistoryFdStatErrorBadFileDescriptor {
    tag: WorkspaceHistoryFdStatErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceHistoryFdStatErrorInternal {
    tag: WorkspaceHistoryFdStatErrorTag.Internal
    error: string
}
export type WorkspaceHistoryFdStatError =
  | WorkspaceHistoryFdStatErrorBadFileDescriptor
  | WorkspaceHistoryFdStatErrorInternal

// WorkspaceHistoryGetWorkspaceManifestV1TimestampError
export enum WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorTag {
    Internal = 'WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInternal',
    InvalidCertificate = 'WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorNoRealmAccess',
    Offline = 'WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorOffline',
    Stopped = 'WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorStopped',
}

export interface WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInternal {
    tag: WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorTag.Internal
    error: string
}
export interface WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInvalidCertificate {
    tag: WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInvalidKeysBundle {
    tag: WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInvalidManifest {
    tag: WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorNoRealmAccess {
    tag: WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorOffline {
    tag: WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorTag.Offline
    error: string
}
export interface WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorStopped {
    tag: WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorTag.Stopped
    error: string
}
export type WorkspaceHistoryGetWorkspaceManifestV1TimestampError =
  | WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInternal
  | WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInvalidCertificate
  | WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInvalidKeysBundle
  | WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInvalidManifest
  | WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorNoRealmAccess
  | WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorOffline
  | WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorStopped

// WorkspaceHistoryOpenFileError
export enum WorkspaceHistoryOpenFileErrorTag {
    EntryNotAFile = 'WorkspaceHistoryOpenFileErrorEntryNotAFile',
    EntryNotFound = 'WorkspaceHistoryOpenFileErrorEntryNotFound',
    Internal = 'WorkspaceHistoryOpenFileErrorInternal',
    InvalidCertificate = 'WorkspaceHistoryOpenFileErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceHistoryOpenFileErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceHistoryOpenFileErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceHistoryOpenFileErrorNoRealmAccess',
    Offline = 'WorkspaceHistoryOpenFileErrorOffline',
    Stopped = 'WorkspaceHistoryOpenFileErrorStopped',
}

export interface WorkspaceHistoryOpenFileErrorEntryNotAFile {
    tag: WorkspaceHistoryOpenFileErrorTag.EntryNotAFile
    error: string
}
export interface WorkspaceHistoryOpenFileErrorEntryNotFound {
    tag: WorkspaceHistoryOpenFileErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInternal {
    tag: WorkspaceHistoryOpenFileErrorTag.Internal
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInvalidCertificate {
    tag: WorkspaceHistoryOpenFileErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInvalidKeysBundle {
    tag: WorkspaceHistoryOpenFileErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInvalidManifest {
    tag: WorkspaceHistoryOpenFileErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceHistoryOpenFileErrorNoRealmAccess {
    tag: WorkspaceHistoryOpenFileErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceHistoryOpenFileErrorOffline {
    tag: WorkspaceHistoryOpenFileErrorTag.Offline
    error: string
}
export interface WorkspaceHistoryOpenFileErrorStopped {
    tag: WorkspaceHistoryOpenFileErrorTag.Stopped
    error: string
}
export type WorkspaceHistoryOpenFileError =
  | WorkspaceHistoryOpenFileErrorEntryNotAFile
  | WorkspaceHistoryOpenFileErrorEntryNotFound
  | WorkspaceHistoryOpenFileErrorInternal
  | WorkspaceHistoryOpenFileErrorInvalidCertificate
  | WorkspaceHistoryOpenFileErrorInvalidKeysBundle
  | WorkspaceHistoryOpenFileErrorInvalidManifest
  | WorkspaceHistoryOpenFileErrorNoRealmAccess
  | WorkspaceHistoryOpenFileErrorOffline
  | WorkspaceHistoryOpenFileErrorStopped

// WorkspaceHistoryStatEntryError
export enum WorkspaceHistoryStatEntryErrorTag {
    EntryNotFound = 'WorkspaceHistoryStatEntryErrorEntryNotFound',
    Internal = 'WorkspaceHistoryStatEntryErrorInternal',
    InvalidCertificate = 'WorkspaceHistoryStatEntryErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceHistoryStatEntryErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceHistoryStatEntryErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceHistoryStatEntryErrorNoRealmAccess',
    Offline = 'WorkspaceHistoryStatEntryErrorOffline',
    Stopped = 'WorkspaceHistoryStatEntryErrorStopped',
}

export interface WorkspaceHistoryStatEntryErrorEntryNotFound {
    tag: WorkspaceHistoryStatEntryErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInternal {
    tag: WorkspaceHistoryStatEntryErrorTag.Internal
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInvalidCertificate {
    tag: WorkspaceHistoryStatEntryErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInvalidKeysBundle {
    tag: WorkspaceHistoryStatEntryErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInvalidManifest {
    tag: WorkspaceHistoryStatEntryErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceHistoryStatEntryErrorNoRealmAccess {
    tag: WorkspaceHistoryStatEntryErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceHistoryStatEntryErrorOffline {
    tag: WorkspaceHistoryStatEntryErrorTag.Offline
    error: string
}
export interface WorkspaceHistoryStatEntryErrorStopped {
    tag: WorkspaceHistoryStatEntryErrorTag.Stopped
    error: string
}
export type WorkspaceHistoryStatEntryError =
  | WorkspaceHistoryStatEntryErrorEntryNotFound
  | WorkspaceHistoryStatEntryErrorInternal
  | WorkspaceHistoryStatEntryErrorInvalidCertificate
  | WorkspaceHistoryStatEntryErrorInvalidKeysBundle
  | WorkspaceHistoryStatEntryErrorInvalidManifest
  | WorkspaceHistoryStatEntryErrorNoRealmAccess
  | WorkspaceHistoryStatEntryErrorOffline
  | WorkspaceHistoryStatEntryErrorStopped

// WorkspaceHistoryStatFolderChildrenError
export enum WorkspaceHistoryStatFolderChildrenErrorTag {
    EntryIsFile = 'WorkspaceHistoryStatFolderChildrenErrorEntryIsFile',
    EntryNotFound = 'WorkspaceHistoryStatFolderChildrenErrorEntryNotFound',
    Internal = 'WorkspaceHistoryStatFolderChildrenErrorInternal',
    InvalidCertificate = 'WorkspaceHistoryStatFolderChildrenErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceHistoryStatFolderChildrenErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceHistoryStatFolderChildrenErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceHistoryStatFolderChildrenErrorNoRealmAccess',
    Offline = 'WorkspaceHistoryStatFolderChildrenErrorOffline',
    Stopped = 'WorkspaceHistoryStatFolderChildrenErrorStopped',
}

export interface WorkspaceHistoryStatFolderChildrenErrorEntryIsFile {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.EntryIsFile
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorEntryNotFound {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInternal {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.Internal
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInvalidCertificate {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInvalidKeysBundle {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInvalidManifest {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorNoRealmAccess {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorOffline {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.Offline
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorStopped {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.Stopped
    error: string
}
export type WorkspaceHistoryStatFolderChildrenError =
  | WorkspaceHistoryStatFolderChildrenErrorEntryIsFile
  | WorkspaceHistoryStatFolderChildrenErrorEntryNotFound
  | WorkspaceHistoryStatFolderChildrenErrorInternal
  | WorkspaceHistoryStatFolderChildrenErrorInvalidCertificate
  | WorkspaceHistoryStatFolderChildrenErrorInvalidKeysBundle
  | WorkspaceHistoryStatFolderChildrenErrorInvalidManifest
  | WorkspaceHistoryStatFolderChildrenErrorNoRealmAccess
  | WorkspaceHistoryStatFolderChildrenErrorOffline
  | WorkspaceHistoryStatFolderChildrenErrorStopped

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

// WorkspaceMoveEntryError
export enum WorkspaceMoveEntryErrorTag {
    CannotMoveRoot = 'WorkspaceMoveEntryErrorCannotMoveRoot',
    DestinationExists = 'WorkspaceMoveEntryErrorDestinationExists',
    DestinationNotFound = 'WorkspaceMoveEntryErrorDestinationNotFound',
    Internal = 'WorkspaceMoveEntryErrorInternal',
    InvalidCertificate = 'WorkspaceMoveEntryErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceMoveEntryErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceMoveEntryErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceMoveEntryErrorNoRealmAccess',
    Offline = 'WorkspaceMoveEntryErrorOffline',
    ReadOnlyRealm = 'WorkspaceMoveEntryErrorReadOnlyRealm',
    SourceNotFound = 'WorkspaceMoveEntryErrorSourceNotFound',
    Stopped = 'WorkspaceMoveEntryErrorStopped',
}

export interface WorkspaceMoveEntryErrorCannotMoveRoot {
    tag: WorkspaceMoveEntryErrorTag.CannotMoveRoot
    error: string
}
export interface WorkspaceMoveEntryErrorDestinationExists {
    tag: WorkspaceMoveEntryErrorTag.DestinationExists
    error: string
}
export interface WorkspaceMoveEntryErrorDestinationNotFound {
    tag: WorkspaceMoveEntryErrorTag.DestinationNotFound
    error: string
}
export interface WorkspaceMoveEntryErrorInternal {
    tag: WorkspaceMoveEntryErrorTag.Internal
    error: string
}
export interface WorkspaceMoveEntryErrorInvalidCertificate {
    tag: WorkspaceMoveEntryErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceMoveEntryErrorInvalidKeysBundle {
    tag: WorkspaceMoveEntryErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceMoveEntryErrorInvalidManifest {
    tag: WorkspaceMoveEntryErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceMoveEntryErrorNoRealmAccess {
    tag: WorkspaceMoveEntryErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceMoveEntryErrorOffline {
    tag: WorkspaceMoveEntryErrorTag.Offline
    error: string
}
export interface WorkspaceMoveEntryErrorReadOnlyRealm {
    tag: WorkspaceMoveEntryErrorTag.ReadOnlyRealm
    error: string
}
export interface WorkspaceMoveEntryErrorSourceNotFound {
    tag: WorkspaceMoveEntryErrorTag.SourceNotFound
    error: string
}
export interface WorkspaceMoveEntryErrorStopped {
    tag: WorkspaceMoveEntryErrorTag.Stopped
    error: string
}
export type WorkspaceMoveEntryError =
  | WorkspaceMoveEntryErrorCannotMoveRoot
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

// WorkspaceStatFolderChildrenError
export enum WorkspaceStatFolderChildrenErrorTag {
    EntryIsFile = 'WorkspaceStatFolderChildrenErrorEntryIsFile',
    EntryNotFound = 'WorkspaceStatFolderChildrenErrorEntryNotFound',
    Internal = 'WorkspaceStatFolderChildrenErrorInternal',
    InvalidCertificate = 'WorkspaceStatFolderChildrenErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceStatFolderChildrenErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceStatFolderChildrenErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceStatFolderChildrenErrorNoRealmAccess',
    Offline = 'WorkspaceStatFolderChildrenErrorOffline',
    Stopped = 'WorkspaceStatFolderChildrenErrorStopped',
}

export interface WorkspaceStatFolderChildrenErrorEntryIsFile {
    tag: WorkspaceStatFolderChildrenErrorTag.EntryIsFile
    error: string
}
export interface WorkspaceStatFolderChildrenErrorEntryNotFound {
    tag: WorkspaceStatFolderChildrenErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceStatFolderChildrenErrorInternal {
    tag: WorkspaceStatFolderChildrenErrorTag.Internal
    error: string
}
export interface WorkspaceStatFolderChildrenErrorInvalidCertificate {
    tag: WorkspaceStatFolderChildrenErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceStatFolderChildrenErrorInvalidKeysBundle {
    tag: WorkspaceStatFolderChildrenErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceStatFolderChildrenErrorInvalidManifest {
    tag: WorkspaceStatFolderChildrenErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceStatFolderChildrenErrorNoRealmAccess {
    tag: WorkspaceStatFolderChildrenErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceStatFolderChildrenErrorOffline {
    tag: WorkspaceStatFolderChildrenErrorTag.Offline
    error: string
}
export interface WorkspaceStatFolderChildrenErrorStopped {
    tag: WorkspaceStatFolderChildrenErrorTag.Stopped
    error: string
}
export type WorkspaceStatFolderChildrenError =
  | WorkspaceStatFolderChildrenErrorEntryIsFile
  | WorkspaceStatFolderChildrenErrorEntryNotFound
  | WorkspaceStatFolderChildrenErrorInternal
  | WorkspaceStatFolderChildrenErrorInvalidCertificate
  | WorkspaceStatFolderChildrenErrorInvalidKeysBundle
  | WorkspaceStatFolderChildrenErrorInvalidManifest
  | WorkspaceStatFolderChildrenErrorNoRealmAccess
  | WorkspaceStatFolderChildrenErrorOffline
  | WorkspaceStatFolderChildrenErrorStopped

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

// WorkspaceWatchEntryOneShotError
export enum WorkspaceWatchEntryOneShotErrorTag {
    EntryNotFound = 'WorkspaceWatchEntryOneShotErrorEntryNotFound',
    Internal = 'WorkspaceWatchEntryOneShotErrorInternal',
    InvalidCertificate = 'WorkspaceWatchEntryOneShotErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceWatchEntryOneShotErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceWatchEntryOneShotErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceWatchEntryOneShotErrorNoRealmAccess',
    Offline = 'WorkspaceWatchEntryOneShotErrorOffline',
    Stopped = 'WorkspaceWatchEntryOneShotErrorStopped',
}

export interface WorkspaceWatchEntryOneShotErrorEntryNotFound {
    tag: WorkspaceWatchEntryOneShotErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorInternal {
    tag: WorkspaceWatchEntryOneShotErrorTag.Internal
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorInvalidCertificate {
    tag: WorkspaceWatchEntryOneShotErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorInvalidKeysBundle {
    tag: WorkspaceWatchEntryOneShotErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorInvalidManifest {
    tag: WorkspaceWatchEntryOneShotErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorNoRealmAccess {
    tag: WorkspaceWatchEntryOneShotErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorOffline {
    tag: WorkspaceWatchEntryOneShotErrorTag.Offline
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorStopped {
    tag: WorkspaceWatchEntryOneShotErrorTag.Stopped
    error: string
}
export type WorkspaceWatchEntryOneShotError =
  | WorkspaceWatchEntryOneShotErrorEntryNotFound
  | WorkspaceWatchEntryOneShotErrorInternal
  | WorkspaceWatchEntryOneShotErrorInvalidCertificate
  | WorkspaceWatchEntryOneShotErrorInvalidKeysBundle
  | WorkspaceWatchEntryOneShotErrorInvalidManifest
  | WorkspaceWatchEntryOneShotErrorNoRealmAccess
  | WorkspaceWatchEntryOneShotErrorOffline
  | WorkspaceWatchEntryOneShotErrorStopped

export interface LibParsecPlugin {
    archiveDevice(
        device_path: Path
    ): Promise<Result<null, ArchiveDeviceError>>
    bootstrapOrganization(
        config: ClientConfig,
        on_event_callback: (handle: number, event: ClientEvent) => void,
        bootstrap_organization_addr: ParsecOrganizationBootstrapAddr,
        save_strategy: DeviceSaveStrategy,
        human_handle: HumanHandle,
        device_label: DeviceLabel,
        sequester_authority_verify_key: SequesterVerifyKeyDer | null
    ): Promise<Result<AvailableDevice, BootstrapOrganizationError>>
    buildParsecOrganizationBootstrapAddr(
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
    claimerDeviceInProgress1DoDenyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<null, ClaimInProgressError>>
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
        on_event_callback: (handle: number, event: ClientEvent) => void,
        addr: ParsecInvitationAddr
    ): Promise<Result<AnyClaimRetrievedInfo, ClaimerRetrieveInfoError>>
    claimerShamirRecoveryAddShare(
        recipient_pick_handle: Handle,
        share_handle: Handle
    ): Promise<Result<ShamirRecoveryClaimMaybeRecoverDeviceInfo, ShamirRecoveryClaimAddShareError>>
    claimerShamirRecoveryFinalizeSaveLocalDevice(
        handle: Handle,
        save_strategy: DeviceSaveStrategy
    ): Promise<Result<AvailableDevice, ClaimInProgressError>>
    claimerShamirRecoveryInProgress1DoDenyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<null, ClaimInProgressError>>
    claimerShamirRecoveryInProgress1DoSignifyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<ShamirRecoveryClaimInProgress2Info, ClaimInProgressError>>
    claimerShamirRecoveryInProgress2DoWaitPeerTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<ShamirRecoveryClaimInProgress3Info, ClaimInProgressError>>
    claimerShamirRecoveryInProgress3DoClaim(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<ShamirRecoveryClaimShareInfo, ClaimInProgressError>>
    claimerShamirRecoveryInitialDoWaitPeer(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<ShamirRecoveryClaimInProgress1Info, ClaimInProgressError>>
    claimerShamirRecoveryPickRecipient(
        handle: Handle,
        recipient_user_id: UserID
    ): Promise<Result<ShamirRecoveryClaimInitialInfo, ShamirRecoveryClaimPickRecipientError>>
    claimerShamirRecoveryRecoverDevice(
        handle: Handle,
        requested_device_label: DeviceLabel
    ): Promise<Result<ShamirRecoveryClaimMaybeFinalizeInfo, ShamirRecoveryClaimRecoverDeviceError>>
    claimerUserFinalizeSaveLocalDevice(
        handle: Handle,
        save_strategy: DeviceSaveStrategy
    ): Promise<Result<AvailableDevice, ClaimInProgressError>>
    claimerUserInProgress1DoDenyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<null, ClaimInProgressError>>
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
    clientAcceptTos(
        client: Handle,
        tos_updated_on: DateTime
    ): Promise<Result<null, ClientAcceptTosError>>
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
    clientDeleteShamirRecovery(
        client_handle: Handle
    ): Promise<Result<null, ClientDeleteShamirRecoveryError>>
    clientExportRecoveryDevice(
        client_handle: Handle,
        device_label: DeviceLabel
    ): Promise<Result<[string, Uint8Array], ClientExportRecoveryDeviceError>>
    clientGetSelfShamirRecovery(
        client_handle: Handle
    ): Promise<Result<SelfShamirRecoveryInfo, ClientGetSelfShamirRecoveryError>>
    clientGetTos(
        client: Handle
    ): Promise<Result<Tos, ClientGetTosError>>
    clientGetUserDevice(
        client: Handle,
        device: DeviceID
    ): Promise<Result<[UserInfo, DeviceInfo], ClientGetUserDeviceError>>
    clientInfo(
        client: Handle
    ): Promise<Result<ClientInfo, ClientInfoError>>
    clientListFrozenUsers(
        client_handle: Handle
    ): Promise<Result<Array<UserID>, ClientListFrozenUsersError>>
    clientListInvitations(
        client: Handle
    ): Promise<Result<Array<InviteListItem>, ListInvitationsError>>
    clientListShamirRecoveriesForOthers(
        client_handle: Handle
    ): Promise<Result<Array<OtherShamirRecoveryInfo>, ClientListShamirRecoveriesForOthersError>>
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
    clientNewShamirRecoveryInvitation(
        client: Handle,
        claimer_user_id: UserID,
        send_email: boolean
    ): Promise<Result<NewInvitationInfo, ClientNewShamirRecoveryInvitationError>>
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
    clientSetupShamirRecovery(
        client_handle: Handle,
        per_recipient_shares: Map<UserID, NonZeroU8>,
        threshold: NonZeroU8
    ): Promise<Result<null, ClientSetupShamirRecoveryError>>
    clientShareWorkspace(
        client: Handle,
        realm_id: VlobID,
        recipient: UserID,
        role: RealmRole | null
    ): Promise<Result<null, ClientShareWorkspaceError>>
    clientStart(
        config: ClientConfig,
        on_event_callback: (handle: number, event: ClientEvent) => void,
        access: DeviceAccessStrategy
    ): Promise<Result<Handle, ClientStartError>>
    clientStartDeviceInvitationGreet(
        client: Handle,
        token: InvitationToken
    ): Promise<Result<DeviceGreetInitialInfo, ClientStartInvitationGreetError>>
    clientStartShamirRecoveryInvitationGreet(
        client: Handle,
        token: InvitationToken
    ): Promise<Result<ShamirRecoveryGreetInitialInfo, ClientStartShamirRecoveryInvitationGreetError>>
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
    greeterDeviceInProgress2DoDenyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<null, GreetInProgressError>>
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
    greeterShamirRecoveryInProgress1DoWaitPeerTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<ShamirRecoveryGreetInProgress2Info, GreetInProgressError>>
    greeterShamirRecoveryInProgress2DoDenyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<null, GreetInProgressError>>
    greeterShamirRecoveryInProgress2DoSignifyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<ShamirRecoveryGreetInProgress3Info, GreetInProgressError>>
    greeterShamirRecoveryInProgress3DoGetClaimRequests(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<null, GreetInProgressError>>
    greeterShamirRecoveryInitialDoWaitPeer(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<ShamirRecoveryGreetInProgress1Info, GreetInProgressError>>
    greeterUserInProgress1DoWaitPeerTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<UserGreetInProgress2Info, GreetInProgressError>>
    greeterUserInProgress2DoDenyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<null, GreetInProgressError>>
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
    importRecoveryDevice(
        config: ClientConfig,
        recovery_device: Uint8Array,
        passphrase: string,
        device_label: DeviceLabel,
        save_strategy: DeviceSaveStrategy
    ): Promise<Result<AvailableDevice, ImportRecoveryDeviceError>>
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
    parseParsecAddr(
        url: string
    ): Promise<Result<ParsedParsecAddr, ParseParsecAddrError>>
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
    waitForDeviceAvailable(
        config_dir: Path,
        device_id: DeviceID
    ): Promise<Result<null, WaitForDeviceAvailableError>>
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
    workspaceDecryptPathAddr(
        workspace: Handle,
        link: ParsecWorkspacePathAddr
    ): Promise<Result<FsPath, WorkspaceDecryptPathAddrError>>
    workspaceFdClose(
        workspace: Handle,
        fd: FileDescriptor
    ): Promise<Result<null, WorkspaceFdCloseError>>
    workspaceFdFlush(
        workspace: Handle,
        fd: FileDescriptor
    ): Promise<Result<null, WorkspaceFdFlushError>>
    workspaceFdRead(
        workspace: Handle,
        fd: FileDescriptor,
        offset: U64,
        size: U64
    ): Promise<Result<Uint8Array, WorkspaceFdReadError>>
    workspaceFdResize(
        workspace: Handle,
        fd: FileDescriptor,
        length: U64,
        truncate_only: boolean
    ): Promise<Result<null, WorkspaceFdResizeError>>
    workspaceFdStat(
        workspace: Handle,
        fd: FileDescriptor
    ): Promise<Result<FileStat, WorkspaceFdStatError>>
    workspaceFdWrite(
        workspace: Handle,
        fd: FileDescriptor,
        offset: U64,
        data: Uint8Array
    ): Promise<Result<U64, WorkspaceFdWriteError>>
    workspaceFdWriteConstrainedIo(
        workspace: Handle,
        fd: FileDescriptor,
        offset: U64,
        data: Uint8Array
    ): Promise<Result<U64, WorkspaceFdWriteError>>
    workspaceFdWriteStartEof(
        workspace: Handle,
        fd: FileDescriptor,
        data: Uint8Array
    ): Promise<Result<U64, WorkspaceFdWriteError>>
    workspaceGeneratePathAddr(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<ParsecWorkspacePathAddr, WorkspaceGeneratePathAddrError>>
    workspaceHistoryFdClose(
        workspace: Handle,
        fd: FileDescriptor
    ): Promise<Result<null, WorkspaceHistoryFdCloseError>>
    workspaceHistoryFdRead(
        workspace: Handle,
        fd: FileDescriptor,
        offset: U64,
        size: U64
    ): Promise<Result<Uint8Array, WorkspaceHistoryFdReadError>>
    workspaceHistoryFdStat(
        workspace: Handle,
        fd: FileDescriptor
    ): Promise<Result<WorkspaceHistoryFileStat, WorkspaceHistoryFdStatError>>
    workspaceHistoryGetWorkspaceManifestV1Timestamp(
        workspace: Handle
    ): Promise<Result<DateTime | null, WorkspaceHistoryGetWorkspaceManifestV1TimestampError>>
    workspaceHistoryOpenFile(
        workspace: Handle,
        at: DateTime,
        path: FsPath
    ): Promise<Result<FileDescriptor, WorkspaceHistoryOpenFileError>>
    workspaceHistoryOpenFileById(
        workspace: Handle,
        at: DateTime,
        entry_id: VlobID
    ): Promise<Result<FileDescriptor, WorkspaceHistoryOpenFileError>>
    workspaceHistoryStatEntry(
        workspace: Handle,
        at: DateTime,
        path: FsPath
    ): Promise<Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError>>
    workspaceHistoryStatEntryById(
        workspace: Handle,
        at: DateTime,
        entry_id: VlobID
    ): Promise<Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError>>
    workspaceHistoryStatFolderChildren(
        workspace: Handle,
        at: DateTime,
        path: FsPath
    ): Promise<Result<Array<[EntryName, WorkspaceHistoryEntryStat]>, WorkspaceHistoryStatFolderChildrenError>>
    workspaceHistoryStatFolderChildrenById(
        workspace: Handle,
        at: DateTime,
        entry_id: VlobID
    ): Promise<Result<Array<[EntryName, WorkspaceHistoryEntryStat]>, WorkspaceHistoryStatFolderChildrenError>>
    workspaceInfo(
        workspace: Handle
    ): Promise<Result<StartedWorkspaceInfo, WorkspaceInfoError>>
    workspaceMount(
        workspace: Handle
    ): Promise<Result<[Handle, Path], WorkspaceMountError>>
    workspaceMoveEntry(
        workspace: Handle,
        src: FsPath,
        dst: FsPath,
        mode: MoveEntryMode
    ): Promise<Result<null, WorkspaceMoveEntryError>>
    workspaceOpenFile(
        workspace: Handle,
        path: FsPath,
        mode: OpenOptions
    ): Promise<Result<FileDescriptor, WorkspaceOpenFileError>>
    workspaceOpenFileAndGetId(
        workspace: Handle,
        path: FsPath,
        mode: OpenOptions
    ): Promise<Result<[FileDescriptor, VlobID], WorkspaceOpenFileError>>
    workspaceOpenFileById(
        workspace: Handle,
        entry_id: VlobID,
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
    workspaceRenameEntryById(
        workspace: Handle,
        src_parent_id: VlobID,
        src_name: EntryName,
        dst_name: EntryName,
        mode: MoveEntryMode
    ): Promise<Result<null, WorkspaceMoveEntryError>>
    workspaceStatEntry(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<EntryStat, WorkspaceStatEntryError>>
    workspaceStatEntryById(
        workspace: Handle,
        entry_id: VlobID
    ): Promise<Result<EntryStat, WorkspaceStatEntryError>>
    workspaceStatEntryByIdIgnoreConfinementPoint(
        workspace: Handle,
        entry_id: VlobID
    ): Promise<Result<EntryStat, WorkspaceStatEntryError>>
    workspaceStatFolderChildren(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<Array<[EntryName, EntryStat]>, WorkspaceStatFolderChildrenError>>
    workspaceStatFolderChildrenById(
        workspace: Handle,
        entry_id: VlobID
    ): Promise<Result<Array<[EntryName, EntryStat]>, WorkspaceStatFolderChildrenError>>
    workspaceStop(
        workspace: Handle
    ): Promise<Result<null, WorkspaceStopError>>
    workspaceWatchEntryOneshot(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<VlobID, WorkspaceWatchEntryOneShotError>>
}
