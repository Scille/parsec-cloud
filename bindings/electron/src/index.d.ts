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

export enum UserOnlineStatus {
    Offline = 'UserOnlineStatusOffline',
    Online = 'UserOnlineStatusOnline',
    Unknown = 'UserOnlineStatusUnknown',
}

export enum UserProfile {
    Admin = 'UserProfileAdmin',
    Outsider = 'UserProfileOutsider',
    Standard = 'UserProfileStandard',
}


export interface AvailableDevice {
    keyFilePath: string
    createdOn: number
    protectedOn: number
    serverUrl: string
    organizationId: string
    userId: string
    deviceId: string
    humanHandle: HumanHandle
    deviceLabel: string
    ty: DeviceFileType
}


export interface ClientConfig {
    configDir: string
    dataBaseDir: string
    mountpointMountStrategy: MountpointMountStrategy
    workspaceStorageCacheSize: WorkspaceStorageCacheSize
    withMonitors: boolean
    preventSyncPattern: string | null
}


export interface ClientInfo {
    organizationAddr: string
    organizationId: string
    deviceId: string
    userId: string
    deviceLabel: string
    humanHandle: HumanHandle
    currentProfile: UserProfile
    serverConfig: ServerConfig
}


export interface DeviceClaimFinalizeInfo {
    handle: number
}


export interface DeviceClaimInProgress1Info {
    handle: number
    greeterSas: string
    greeterSasChoices: Array<string>
}


export interface DeviceClaimInProgress2Info {
    handle: number
    claimerSas: string
}


export interface DeviceClaimInProgress3Info {
    handle: number
}


export interface DeviceGreetInProgress1Info {
    handle: number
    greeterSas: string
}


export interface DeviceGreetInProgress2Info {
    handle: number
    claimerSas: string
    claimerSasChoices: Array<string>
}


export interface DeviceGreetInProgress3Info {
    handle: number
}


export interface DeviceGreetInProgress4Info {
    handle: number
    requestedDeviceLabel: string
}


export interface DeviceGreetInitialInfo {
    handle: number
}


export interface DeviceInfo {
    id: string
    purpose: DevicePurpose
    deviceLabel: string
    createdOn: number
    createdBy: string | null
}


export interface FileStat {
    id: string
    created: number
    updated: number
    baseVersion: number
    isPlaceholder: boolean
    needSync: boolean
    size: number
}


export interface HumanHandle {
    email: string
    label: string
}


export interface NewInvitationInfo {
    addr: string
    token: string
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
    handle: number
    greeterSas: string
    greeterSasChoices: Array<string>
}


export interface ShamirRecoveryClaimInProgress2Info {
    handle: number
    claimerSas: string
}


export interface ShamirRecoveryClaimInProgress3Info {
    handle: number
}


export interface ShamirRecoveryClaimInitialInfo {
    handle: number
    greeterUserId: string
    greeterHumanHandle: HumanHandle
}


export interface ShamirRecoveryClaimShareInfo {
    handle: number
}


export interface ShamirRecoveryGreetInProgress1Info {
    handle: number
    greeterSas: string
}


export interface ShamirRecoveryGreetInProgress2Info {
    handle: number
    claimerSas: string
    claimerSasChoices: Array<string>
}


export interface ShamirRecoveryGreetInProgress3Info {
    handle: number
}


export interface ShamirRecoveryGreetInitialInfo {
    handle: number
}


export interface ShamirRecoveryRecipient {
    userId: string
    humanHandle: HumanHandle
    revokedOn: number | null
    shares: number
    onlineStatus: UserOnlineStatus
}


export interface StartedWorkspaceInfo {
    client: number
    id: string
    currentName: string
    currentSelfRole: RealmRole
    mountpoints: Array<[number, string]>
}


export interface Tos {
    perLocaleUrls: Map<string, string>
    updatedOn: number
}


export interface UserClaimFinalizeInfo {
    handle: number
}


export interface UserClaimInProgress1Info {
    handle: number
    greeterSas: string
    greeterSasChoices: Array<string>
}


export interface UserClaimInProgress2Info {
    handle: number
    claimerSas: string
}


export interface UserClaimInProgress3Info {
    handle: number
}


export interface UserClaimInitialInfo {
    handle: number
    greeterUserId: string
    greeterHumanHandle: HumanHandle
    onlineStatus: UserOnlineStatus
    lastGreetingAttemptJoinedOn: number | null
}


export interface UserGreetInProgress1Info {
    handle: number
    greeterSas: string
}


export interface UserGreetInProgress2Info {
    handle: number
    claimerSas: string
    claimerSasChoices: Array<string>
}


export interface UserGreetInProgress3Info {
    handle: number
}


export interface UserGreetInProgress4Info {
    handle: number
    requestedHumanHandle: HumanHandle
    requestedDeviceLabel: string
}


export interface UserGreetInitialInfo {
    handle: number
}


export interface UserInfo {
    id: string
    humanHandle: HumanHandle
    currentProfile: UserProfile
    createdOn: number
    createdBy: string | null
    revokedOn: number | null
    revokedBy: string | null
}


export interface WorkspaceHistoryFileStat {
    id: string
    created: number
    updated: number
    version: number
    size: number
}


export interface WorkspaceInfo {
    id: string
    currentName: string
    currentSelfRole: RealmRole
    isStarted: boolean
    isBootstrapped: boolean
}


export interface WorkspaceUserAccessInfo {
    userId: string
    humanHandle: HumanHandle
    currentProfile: UserProfile
    currentRole: RealmRole
}


// ActiveUsersLimit
export interface ActiveUsersLimitLimitedTo {
    tag: "LimitedTo"
    x1: number
}
export interface ActiveUsersLimitNoLimit {
    tag: "NoLimit"
}
export type ActiveUsersLimit =
  | ActiveUsersLimitLimitedTo
  | ActiveUsersLimitNoLimit


// AnyClaimRetrievedInfo
export interface AnyClaimRetrievedInfoDevice {
    tag: "Device"
    handle: number
    greeter_user_id: string
    greeter_human_handle: HumanHandle
}
export interface AnyClaimRetrievedInfoShamirRecovery {
    tag: "ShamirRecovery"
    handle: number
    claimer_user_id: string
    claimer_human_handle: HumanHandle
    invitation_created_by: InviteInfoInvitationCreatedBy
    shamir_recovery_created_on: number
    recipients: Array<ShamirRecoveryRecipient>
    threshold: number
    is_recoverable: boolean
}
export interface AnyClaimRetrievedInfoUser {
    tag: "User"
    claimer_email: string
    created_by: InviteInfoInvitationCreatedBy
    user_claim_initial_infos: Array<UserClaimInitialInfo>
}
export type AnyClaimRetrievedInfo =
  | AnyClaimRetrievedInfoDevice
  | AnyClaimRetrievedInfoShamirRecovery
  | AnyClaimRetrievedInfoUser


// ArchiveDeviceError
export interface ArchiveDeviceErrorInternal {
    tag: "Internal"
    error: string
}
export type ArchiveDeviceError =
  | ArchiveDeviceErrorInternal


// BootstrapOrganizationError
export interface BootstrapOrganizationErrorAlreadyUsedToken {
    tag: "AlreadyUsedToken"
    error: string
}
export interface BootstrapOrganizationErrorInternal {
    tag: "Internal"
    error: string
}
export interface BootstrapOrganizationErrorInvalidToken {
    tag: "InvalidToken"
    error: string
}
export interface BootstrapOrganizationErrorOffline {
    tag: "Offline"
    error: string
}
export interface BootstrapOrganizationErrorOrganizationExpired {
    tag: "OrganizationExpired"
    error: string
}
export interface BootstrapOrganizationErrorSaveDeviceError {
    tag: "SaveDeviceError"
    error: string
}
export interface BootstrapOrganizationErrorTimestampOutOfBallpark {
    tag: "TimestampOutOfBallpark"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
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
export interface CancelErrorInternal {
    tag: "Internal"
    error: string
}
export interface CancelErrorNotBound {
    tag: "NotBound"
    error: string
}
export type CancelError =
  | CancelErrorInternal
  | CancelErrorNotBound


// ClaimInProgressError
export interface ClaimInProgressErrorActiveUsersLimitReached {
    tag: "ActiveUsersLimitReached"
    error: string
}
export interface ClaimInProgressErrorAlreadyUsedOrDeleted {
    tag: "AlreadyUsedOrDeleted"
    error: string
}
export interface ClaimInProgressErrorCancelled {
    tag: "Cancelled"
    error: string
}
export interface ClaimInProgressErrorCorruptedConfirmation {
    tag: "CorruptedConfirmation"
    error: string
}
export interface ClaimInProgressErrorGreeterNotAllowed {
    tag: "GreeterNotAllowed"
    error: string
}
export interface ClaimInProgressErrorGreetingAttemptCancelled {
    tag: "GreetingAttemptCancelled"
    error: string
    origin: GreeterOrClaimer
    reason: CancelledGreetingAttemptReason
    timestamp: number
}
export interface ClaimInProgressErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClaimInProgressErrorNotFound {
    tag: "NotFound"
    error: string
}
export interface ClaimInProgressErrorOffline {
    tag: "Offline"
    error: string
}
export interface ClaimInProgressErrorOrganizationExpired {
    tag: "OrganizationExpired"
    error: string
}
export interface ClaimInProgressErrorPeerReset {
    tag: "PeerReset"
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
export interface ClaimerGreeterAbortOperationErrorInternal {
    tag: "Internal"
    error: string
}
export type ClaimerGreeterAbortOperationError =
  | ClaimerGreeterAbortOperationErrorInternal


// ClaimerRetrieveInfoError
export interface ClaimerRetrieveInfoErrorAlreadyUsedOrDeleted {
    tag: "AlreadyUsedOrDeleted"
    error: string
}
export interface ClaimerRetrieveInfoErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClaimerRetrieveInfoErrorNotFound {
    tag: "NotFound"
    error: string
}
export interface ClaimerRetrieveInfoErrorOffline {
    tag: "Offline"
    error: string
}
export interface ClaimerRetrieveInfoErrorOrganizationExpired {
    tag: "OrganizationExpired"
    error: string
}
export type ClaimerRetrieveInfoError =
  | ClaimerRetrieveInfoErrorAlreadyUsedOrDeleted
  | ClaimerRetrieveInfoErrorInternal
  | ClaimerRetrieveInfoErrorNotFound
  | ClaimerRetrieveInfoErrorOffline
  | ClaimerRetrieveInfoErrorOrganizationExpired


// ClientAcceptTosError
export interface ClientAcceptTosErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientAcceptTosErrorNoTos {
    tag: "NoTos"
    error: string
}
export interface ClientAcceptTosErrorOffline {
    tag: "Offline"
    error: string
}
export interface ClientAcceptTosErrorTosMismatch {
    tag: "TosMismatch"
    error: string
}
export type ClientAcceptTosError =
  | ClientAcceptTosErrorInternal
  | ClientAcceptTosErrorNoTos
  | ClientAcceptTosErrorOffline
  | ClientAcceptTosErrorTosMismatch


// ClientCancelInvitationError
export interface ClientCancelInvitationErrorAlreadyDeleted {
    tag: "AlreadyDeleted"
    error: string
}
export interface ClientCancelInvitationErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientCancelInvitationErrorNotFound {
    tag: "NotFound"
    error: string
}
export interface ClientCancelInvitationErrorOffline {
    tag: "Offline"
    error: string
}
export type ClientCancelInvitationError =
  | ClientCancelInvitationErrorAlreadyDeleted
  | ClientCancelInvitationErrorInternal
  | ClientCancelInvitationErrorNotFound
  | ClientCancelInvitationErrorOffline


// ClientChangeAuthenticationError
export interface ClientChangeAuthenticationErrorDecryptionFailed {
    tag: "DecryptionFailed"
    error: string
}
export interface ClientChangeAuthenticationErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientChangeAuthenticationErrorInvalidData {
    tag: "InvalidData"
    error: string
}
export interface ClientChangeAuthenticationErrorInvalidPath {
    tag: "InvalidPath"
    error: string
}
export type ClientChangeAuthenticationError =
  | ClientChangeAuthenticationErrorDecryptionFailed
  | ClientChangeAuthenticationErrorInternal
  | ClientChangeAuthenticationErrorInvalidData
  | ClientChangeAuthenticationErrorInvalidPath


// ClientCreateWorkspaceError
export interface ClientCreateWorkspaceErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientCreateWorkspaceErrorStopped {
    tag: "Stopped"
    error: string
}
export type ClientCreateWorkspaceError =
  | ClientCreateWorkspaceErrorInternal
  | ClientCreateWorkspaceErrorStopped


// ClientDeleteShamirRecoveryError
export interface ClientDeleteShamirRecoveryErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientDeleteShamirRecoveryErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface ClientDeleteShamirRecoveryErrorOffline {
    tag: "Offline"
    error: string
}
export interface ClientDeleteShamirRecoveryErrorStopped {
    tag: "Stopped"
    error: string
}
export interface ClientDeleteShamirRecoveryErrorTimestampOutOfBallpark {
    tag: "TimestampOutOfBallpark"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export type ClientDeleteShamirRecoveryError =
  | ClientDeleteShamirRecoveryErrorInternal
  | ClientDeleteShamirRecoveryErrorInvalidCertificate
  | ClientDeleteShamirRecoveryErrorOffline
  | ClientDeleteShamirRecoveryErrorStopped
  | ClientDeleteShamirRecoveryErrorTimestampOutOfBallpark


// ClientEvent
export interface ClientEventExpiredOrganization {
    tag: "ExpiredOrganization"
}
export interface ClientEventIncompatibleServer {
    tag: "IncompatibleServer"
    detail: string
}
export interface ClientEventInvitationChanged {
    tag: "InvitationChanged"
    token: string
    status: InvitationStatus
}
export interface ClientEventMustAcceptTos {
    tag: "MustAcceptTos"
}
export interface ClientEventOffline {
    tag: "Offline"
}
export interface ClientEventOnline {
    tag: "Online"
}
export interface ClientEventPing {
    tag: "Ping"
    ping: string
}
export interface ClientEventRevokedSelfUser {
    tag: "RevokedSelfUser"
}
export interface ClientEventServerConfigChanged {
    tag: "ServerConfigChanged"
}
export interface ClientEventTooMuchDriftWithServerClock {
    tag: "TooMuchDriftWithServerClock"
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export interface ClientEventWorkspaceLocallyCreated {
    tag: "WorkspaceLocallyCreated"
}
export interface ClientEventWorkspaceOpsInboundSyncDone {
    tag: "WorkspaceOpsInboundSyncDone"
    realm_id: string
    entry_id: string
}
export interface ClientEventWorkspaceOpsOutboundSyncAborted {
    tag: "WorkspaceOpsOutboundSyncAborted"
    realm_id: string
    entry_id: string
}
export interface ClientEventWorkspaceOpsOutboundSyncDone {
    tag: "WorkspaceOpsOutboundSyncDone"
    realm_id: string
    entry_id: string
}
export interface ClientEventWorkspaceOpsOutboundSyncProgress {
    tag: "WorkspaceOpsOutboundSyncProgress"
    realm_id: string
    entry_id: string
    blocks: number
    block_index: number
    blocksize: number
}
export interface ClientEventWorkspaceOpsOutboundSyncStarted {
    tag: "WorkspaceOpsOutboundSyncStarted"
    realm_id: string
    entry_id: string
}
export interface ClientEventWorkspaceWatchedEntryChanged {
    tag: "WorkspaceWatchedEntryChanged"
    realm_id: string
    entry_id: string
}
export interface ClientEventWorkspacesSelfListChanged {
    tag: "WorkspacesSelfListChanged"
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
export interface ClientExportRecoveryDeviceErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientExportRecoveryDeviceErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface ClientExportRecoveryDeviceErrorOffline {
    tag: "Offline"
    error: string
}
export interface ClientExportRecoveryDeviceErrorStopped {
    tag: "Stopped"
    error: string
}
export interface ClientExportRecoveryDeviceErrorTimestampOutOfBallpark {
    tag: "TimestampOutOfBallpark"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export type ClientExportRecoveryDeviceError =
  | ClientExportRecoveryDeviceErrorInternal
  | ClientExportRecoveryDeviceErrorInvalidCertificate
  | ClientExportRecoveryDeviceErrorOffline
  | ClientExportRecoveryDeviceErrorStopped
  | ClientExportRecoveryDeviceErrorTimestampOutOfBallpark


// ClientGetSelfShamirRecoveryError
export interface ClientGetSelfShamirRecoveryErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientGetSelfShamirRecoveryErrorStopped {
    tag: "Stopped"
    error: string
}
export type ClientGetSelfShamirRecoveryError =
  | ClientGetSelfShamirRecoveryErrorInternal
  | ClientGetSelfShamirRecoveryErrorStopped


// ClientGetTosError
export interface ClientGetTosErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientGetTosErrorNoTos {
    tag: "NoTos"
    error: string
}
export interface ClientGetTosErrorOffline {
    tag: "Offline"
    error: string
}
export type ClientGetTosError =
  | ClientGetTosErrorInternal
  | ClientGetTosErrorNoTos
  | ClientGetTosErrorOffline


// ClientGetUserDeviceError
export interface ClientGetUserDeviceErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientGetUserDeviceErrorNonExisting {
    tag: "NonExisting"
    error: string
}
export interface ClientGetUserDeviceErrorStopped {
    tag: "Stopped"
    error: string
}
export type ClientGetUserDeviceError =
  | ClientGetUserDeviceErrorInternal
  | ClientGetUserDeviceErrorNonExisting
  | ClientGetUserDeviceErrorStopped


// ClientInfoError
export interface ClientInfoErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientInfoErrorStopped {
    tag: "Stopped"
    error: string
}
export type ClientInfoError =
  | ClientInfoErrorInternal
  | ClientInfoErrorStopped


// ClientListFrozenUsersError
export interface ClientListFrozenUsersErrorAuthorNotAllowed {
    tag: "AuthorNotAllowed"
    error: string
}
export interface ClientListFrozenUsersErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientListFrozenUsersErrorOffline {
    tag: "Offline"
    error: string
}
export type ClientListFrozenUsersError =
  | ClientListFrozenUsersErrorAuthorNotAllowed
  | ClientListFrozenUsersErrorInternal
  | ClientListFrozenUsersErrorOffline


// ClientListShamirRecoveriesForOthersError
export interface ClientListShamirRecoveriesForOthersErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientListShamirRecoveriesForOthersErrorStopped {
    tag: "Stopped"
    error: string
}
export type ClientListShamirRecoveriesForOthersError =
  | ClientListShamirRecoveriesForOthersErrorInternal
  | ClientListShamirRecoveriesForOthersErrorStopped


// ClientListUserDevicesError
export interface ClientListUserDevicesErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientListUserDevicesErrorStopped {
    tag: "Stopped"
    error: string
}
export type ClientListUserDevicesError =
  | ClientListUserDevicesErrorInternal
  | ClientListUserDevicesErrorStopped


// ClientListUsersError
export interface ClientListUsersErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientListUsersErrorStopped {
    tag: "Stopped"
    error: string
}
export type ClientListUsersError =
  | ClientListUsersErrorInternal
  | ClientListUsersErrorStopped


// ClientListWorkspaceUsersError
export interface ClientListWorkspaceUsersErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientListWorkspaceUsersErrorStopped {
    tag: "Stopped"
    error: string
}
export type ClientListWorkspaceUsersError =
  | ClientListWorkspaceUsersErrorInternal
  | ClientListWorkspaceUsersErrorStopped


// ClientListWorkspacesError
export interface ClientListWorkspacesErrorInternal {
    tag: "Internal"
    error: string
}
export type ClientListWorkspacesError =
  | ClientListWorkspacesErrorInternal


// ClientNewDeviceInvitationError
export interface ClientNewDeviceInvitationErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientNewDeviceInvitationErrorOffline {
    tag: "Offline"
    error: string
}
export type ClientNewDeviceInvitationError =
  | ClientNewDeviceInvitationErrorInternal
  | ClientNewDeviceInvitationErrorOffline


// ClientNewShamirRecoveryInvitationError
export interface ClientNewShamirRecoveryInvitationErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientNewShamirRecoveryInvitationErrorNotAllowed {
    tag: "NotAllowed"
    error: string
}
export interface ClientNewShamirRecoveryInvitationErrorOffline {
    tag: "Offline"
    error: string
}
export interface ClientNewShamirRecoveryInvitationErrorUserNotFound {
    tag: "UserNotFound"
    error: string
}
export type ClientNewShamirRecoveryInvitationError =
  | ClientNewShamirRecoveryInvitationErrorInternal
  | ClientNewShamirRecoveryInvitationErrorNotAllowed
  | ClientNewShamirRecoveryInvitationErrorOffline
  | ClientNewShamirRecoveryInvitationErrorUserNotFound


// ClientNewUserInvitationError
export interface ClientNewUserInvitationErrorAlreadyMember {
    tag: "AlreadyMember"
    error: string
}
export interface ClientNewUserInvitationErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientNewUserInvitationErrorNotAllowed {
    tag: "NotAllowed"
    error: string
}
export interface ClientNewUserInvitationErrorOffline {
    tag: "Offline"
    error: string
}
export type ClientNewUserInvitationError =
  | ClientNewUserInvitationErrorAlreadyMember
  | ClientNewUserInvitationErrorInternal
  | ClientNewUserInvitationErrorNotAllowed
  | ClientNewUserInvitationErrorOffline


// ClientRenameWorkspaceError
export interface ClientRenameWorkspaceErrorAuthorNotAllowed {
    tag: "AuthorNotAllowed"
    error: string
}
export interface ClientRenameWorkspaceErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientRenameWorkspaceErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface ClientRenameWorkspaceErrorInvalidEncryptedRealmName {
    tag: "InvalidEncryptedRealmName"
    error: string
}
export interface ClientRenameWorkspaceErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface ClientRenameWorkspaceErrorNoKey {
    tag: "NoKey"
    error: string
}
export interface ClientRenameWorkspaceErrorOffline {
    tag: "Offline"
    error: string
}
export interface ClientRenameWorkspaceErrorStopped {
    tag: "Stopped"
    error: string
}
export interface ClientRenameWorkspaceErrorTimestampOutOfBallpark {
    tag: "TimestampOutOfBallpark"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export interface ClientRenameWorkspaceErrorWorkspaceNotFound {
    tag: "WorkspaceNotFound"
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
export interface ClientRevokeUserErrorAuthorNotAllowed {
    tag: "AuthorNotAllowed"
    error: string
}
export interface ClientRevokeUserErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientRevokeUserErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface ClientRevokeUserErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface ClientRevokeUserErrorNoKey {
    tag: "NoKey"
    error: string
}
export interface ClientRevokeUserErrorOffline {
    tag: "Offline"
    error: string
}
export interface ClientRevokeUserErrorStopped {
    tag: "Stopped"
    error: string
}
export interface ClientRevokeUserErrorTimestampOutOfBallpark {
    tag: "TimestampOutOfBallpark"
    error: string
}
export interface ClientRevokeUserErrorUserIsSelf {
    tag: "UserIsSelf"
    error: string
}
export interface ClientRevokeUserErrorUserNotFound {
    tag: "UserNotFound"
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
export interface ClientSetupShamirRecoveryErrorAuthorAmongRecipients {
    tag: "AuthorAmongRecipients"
    error: string
}
export interface ClientSetupShamirRecoveryErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientSetupShamirRecoveryErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface ClientSetupShamirRecoveryErrorOffline {
    tag: "Offline"
    error: string
}
export interface ClientSetupShamirRecoveryErrorRecipientNotFound {
    tag: "RecipientNotFound"
    error: string
}
export interface ClientSetupShamirRecoveryErrorRecipientRevoked {
    tag: "RecipientRevoked"
    error: string
}
export interface ClientSetupShamirRecoveryErrorShamirRecoveryAlreadyExists {
    tag: "ShamirRecoveryAlreadyExists"
    error: string
}
export interface ClientSetupShamirRecoveryErrorStopped {
    tag: "Stopped"
    error: string
}
export interface ClientSetupShamirRecoveryErrorThresholdBiggerThanSumOfShares {
    tag: "ThresholdBiggerThanSumOfShares"
    error: string
}
export interface ClientSetupShamirRecoveryErrorTimestampOutOfBallpark {
    tag: "TimestampOutOfBallpark"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export interface ClientSetupShamirRecoveryErrorTooManyShares {
    tag: "TooManyShares"
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
export interface ClientShareWorkspaceErrorAuthorNotAllowed {
    tag: "AuthorNotAllowed"
    error: string
}
export interface ClientShareWorkspaceErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientShareWorkspaceErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface ClientShareWorkspaceErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface ClientShareWorkspaceErrorOffline {
    tag: "Offline"
    error: string
}
export interface ClientShareWorkspaceErrorRecipientIsSelf {
    tag: "RecipientIsSelf"
    error: string
}
export interface ClientShareWorkspaceErrorRecipientNotFound {
    tag: "RecipientNotFound"
    error: string
}
export interface ClientShareWorkspaceErrorRecipientRevoked {
    tag: "RecipientRevoked"
    error: string
}
export interface ClientShareWorkspaceErrorRoleIncompatibleWithOutsider {
    tag: "RoleIncompatibleWithOutsider"
    error: string
}
export interface ClientShareWorkspaceErrorStopped {
    tag: "Stopped"
    error: string
}
export interface ClientShareWorkspaceErrorTimestampOutOfBallpark {
    tag: "TimestampOutOfBallpark"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export interface ClientShareWorkspaceErrorWorkspaceNotFound {
    tag: "WorkspaceNotFound"
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
export interface ClientStartErrorDeviceUsedByAnotherProcess {
    tag: "DeviceUsedByAnotherProcess"
    error: string
}
export interface ClientStartErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientStartErrorLoadDeviceDecryptionFailed {
    tag: "LoadDeviceDecryptionFailed"
    error: string
}
export interface ClientStartErrorLoadDeviceInvalidData {
    tag: "LoadDeviceInvalidData"
    error: string
}
export interface ClientStartErrorLoadDeviceInvalidPath {
    tag: "LoadDeviceInvalidPath"
    error: string
}
export type ClientStartError =
  | ClientStartErrorDeviceUsedByAnotherProcess
  | ClientStartErrorInternal
  | ClientStartErrorLoadDeviceDecryptionFailed
  | ClientStartErrorLoadDeviceInvalidData
  | ClientStartErrorLoadDeviceInvalidPath


// ClientStartInvitationGreetError
export interface ClientStartInvitationGreetErrorInternal {
    tag: "Internal"
    error: string
}
export type ClientStartInvitationGreetError =
  | ClientStartInvitationGreetErrorInternal


// ClientStartShamirRecoveryInvitationGreetError
export interface ClientStartShamirRecoveryInvitationGreetErrorCorruptedShareData {
    tag: "CorruptedShareData"
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorInvitationNotFound {
    tag: "InvitationNotFound"
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorOffline {
    tag: "Offline"
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryDeleted {
    tag: "ShamirRecoveryDeleted"
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryNotFound {
    tag: "ShamirRecoveryNotFound"
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryUnusable {
    tag: "ShamirRecoveryUnusable"
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorStopped {
    tag: "Stopped"
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
export interface ClientStartWorkspaceErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientStartWorkspaceErrorWorkspaceNotFound {
    tag: "WorkspaceNotFound"
    error: string
}
export type ClientStartWorkspaceError =
  | ClientStartWorkspaceErrorInternal
  | ClientStartWorkspaceErrorWorkspaceNotFound


// ClientStopError
export interface ClientStopErrorInternal {
    tag: "Internal"
    error: string
}
export type ClientStopError =
  | ClientStopErrorInternal


// DeviceAccessStrategy
export interface DeviceAccessStrategyKeyring {
    tag: "Keyring"
    key_file: string
}
export interface DeviceAccessStrategyPassword {
    tag: "Password"
    password: string
    key_file: string
}
export interface DeviceAccessStrategySmartcard {
    tag: "Smartcard"
    key_file: string
}
export type DeviceAccessStrategy =
  | DeviceAccessStrategyKeyring
  | DeviceAccessStrategyPassword
  | DeviceAccessStrategySmartcard


// DeviceSaveStrategy
export interface DeviceSaveStrategyKeyring {
    tag: "Keyring"
}
export interface DeviceSaveStrategyPassword {
    tag: "Password"
    password: string
}
export interface DeviceSaveStrategySmartcard {
    tag: "Smartcard"
}
export type DeviceSaveStrategy =
  | DeviceSaveStrategyKeyring
  | DeviceSaveStrategyPassword
  | DeviceSaveStrategySmartcard


// EntryStat
export interface EntryStatFile {
    tag: "File"
    confinement_point: string | null
    id: string
    parent: string
    created: number
    updated: number
    base_version: number
    is_placeholder: boolean
    need_sync: boolean
    size: number
}
export interface EntryStatFolder {
    tag: "Folder"
    confinement_point: string | null
    id: string
    parent: string
    created: number
    updated: number
    base_version: number
    is_placeholder: boolean
    need_sync: boolean
}
export type EntryStat =
  | EntryStatFile
  | EntryStatFolder


// GreetInProgressError
export interface GreetInProgressErrorActiveUsersLimitReached {
    tag: "ActiveUsersLimitReached"
    error: string
}
export interface GreetInProgressErrorAlreadyDeleted {
    tag: "AlreadyDeleted"
    error: string
}
export interface GreetInProgressErrorCancelled {
    tag: "Cancelled"
    error: string
}
export interface GreetInProgressErrorCorruptedInviteUserData {
    tag: "CorruptedInviteUserData"
    error: string
}
export interface GreetInProgressErrorDeviceAlreadyExists {
    tag: "DeviceAlreadyExists"
    error: string
}
export interface GreetInProgressErrorGreeterNotAllowed {
    tag: "GreeterNotAllowed"
    error: string
}
export interface GreetInProgressErrorGreetingAttemptCancelled {
    tag: "GreetingAttemptCancelled"
    error: string
    origin: GreeterOrClaimer
    reason: CancelledGreetingAttemptReason
    timestamp: number
}
export interface GreetInProgressErrorHumanHandleAlreadyTaken {
    tag: "HumanHandleAlreadyTaken"
    error: string
}
export interface GreetInProgressErrorInternal {
    tag: "Internal"
    error: string
}
export interface GreetInProgressErrorNonceMismatch {
    tag: "NonceMismatch"
    error: string
}
export interface GreetInProgressErrorNotFound {
    tag: "NotFound"
    error: string
}
export interface GreetInProgressErrorOffline {
    tag: "Offline"
    error: string
}
export interface GreetInProgressErrorPeerReset {
    tag: "PeerReset"
    error: string
}
export interface GreetInProgressErrorTimestampOutOfBallpark {
    tag: "TimestampOutOfBallpark"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export interface GreetInProgressErrorUserAlreadyExists {
    tag: "UserAlreadyExists"
    error: string
}
export interface GreetInProgressErrorUserCreateNotAllowed {
    tag: "UserCreateNotAllowed"
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
export interface ImportRecoveryDeviceErrorDecryptionFailed {
    tag: "DecryptionFailed"
    error: string
}
export interface ImportRecoveryDeviceErrorInternal {
    tag: "Internal"
    error: string
}
export interface ImportRecoveryDeviceErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface ImportRecoveryDeviceErrorInvalidData {
    tag: "InvalidData"
    error: string
}
export interface ImportRecoveryDeviceErrorInvalidPassphrase {
    tag: "InvalidPassphrase"
    error: string
}
export interface ImportRecoveryDeviceErrorInvalidPath {
    tag: "InvalidPath"
    error: string
}
export interface ImportRecoveryDeviceErrorOffline {
    tag: "Offline"
    error: string
}
export interface ImportRecoveryDeviceErrorStopped {
    tag: "Stopped"
    error: string
}
export interface ImportRecoveryDeviceErrorTimestampOutOfBallpark {
    tag: "TimestampOutOfBallpark"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
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


// InviteInfoInvitationCreatedBy
export interface InviteInfoInvitationCreatedByExternalService {
    tag: "ExternalService"
    service_label: string
}
export interface InviteInfoInvitationCreatedByUser {
    tag: "User"
    user_id: string
    human_handle: HumanHandle
}
export type InviteInfoInvitationCreatedBy =
  | InviteInfoInvitationCreatedByExternalService
  | InviteInfoInvitationCreatedByUser


// InviteListInvitationCreatedBy
export interface InviteListInvitationCreatedByExternalService {
    tag: "ExternalService"
    service_label: string
}
export interface InviteListInvitationCreatedByUser {
    tag: "User"
    user_id: string
    human_handle: HumanHandle
}
export type InviteListInvitationCreatedBy =
  | InviteListInvitationCreatedByExternalService
  | InviteListInvitationCreatedByUser


// InviteListItem
export interface InviteListItemDevice {
    tag: "Device"
    addr: string
    token: string
    created_on: number
    created_by: InviteListInvitationCreatedBy
    status: InvitationStatus
}
export interface InviteListItemShamirRecovery {
    tag: "ShamirRecovery"
    addr: string
    token: string
    created_on: number
    created_by: InviteListInvitationCreatedBy
    claimer_user_id: string
    shamir_recovery_created_on: number
    status: InvitationStatus
}
export interface InviteListItemUser {
    tag: "User"
    addr: string
    token: string
    created_on: number
    created_by: InviteListInvitationCreatedBy
    claimer_email: string
    status: InvitationStatus
}
export type InviteListItem =
  | InviteListItemDevice
  | InviteListItemShamirRecovery
  | InviteListItemUser


// ListInvitationsError
export interface ListInvitationsErrorInternal {
    tag: "Internal"
    error: string
}
export interface ListInvitationsErrorOffline {
    tag: "Offline"
    error: string
}
export type ListInvitationsError =
  | ListInvitationsErrorInternal
  | ListInvitationsErrorOffline


// MountpointMountStrategy
export interface MountpointMountStrategyDirectory {
    tag: "Directory"
    base_dir: string
}
export interface MountpointMountStrategyDisabled {
    tag: "Disabled"
}
export interface MountpointMountStrategyDriveLetter {
    tag: "DriveLetter"
}
export type MountpointMountStrategy =
  | MountpointMountStrategyDirectory
  | MountpointMountStrategyDisabled
  | MountpointMountStrategyDriveLetter


// MountpointToOsPathError
export interface MountpointToOsPathErrorInternal {
    tag: "Internal"
    error: string
}
export type MountpointToOsPathError =
  | MountpointToOsPathErrorInternal


// MountpointUnmountError
export interface MountpointUnmountErrorInternal {
    tag: "Internal"
    error: string
}
export type MountpointUnmountError =
  | MountpointUnmountErrorInternal


// MoveEntryMode
export interface MoveEntryModeCanReplace {
    tag: "CanReplace"
}
export interface MoveEntryModeCanReplaceFileOnly {
    tag: "CanReplaceFileOnly"
}
export interface MoveEntryModeExchange {
    tag: "Exchange"
}
export interface MoveEntryModeNoReplace {
    tag: "NoReplace"
}
export type MoveEntryMode =
  | MoveEntryModeCanReplace
  | MoveEntryModeCanReplaceFileOnly
  | MoveEntryModeExchange
  | MoveEntryModeNoReplace


// OtherShamirRecoveryInfo
export interface OtherShamirRecoveryInfoDeleted {
    tag: "Deleted"
    user_id: string
    created_on: number
    created_by: string
    threshold: number
    per_recipient_shares: Map<string, number>
    deleted_on: number
    deleted_by: string
}
export interface OtherShamirRecoveryInfoSetupAllValid {
    tag: "SetupAllValid"
    user_id: string
    created_on: number
    created_by: string
    threshold: number
    per_recipient_shares: Map<string, number>
}
export interface OtherShamirRecoveryInfoSetupButUnusable {
    tag: "SetupButUnusable"
    user_id: string
    created_on: number
    created_by: string
    threshold: number
    per_recipient_shares: Map<string, number>
    revoked_recipients: Array<string>
}
export interface OtherShamirRecoveryInfoSetupWithRevokedRecipients {
    tag: "SetupWithRevokedRecipients"
    user_id: string
    created_on: number
    created_by: string
    threshold: number
    per_recipient_shares: Map<string, number>
    revoked_recipients: Array<string>
}
export type OtherShamirRecoveryInfo =
  | OtherShamirRecoveryInfoDeleted
  | OtherShamirRecoveryInfoSetupAllValid
  | OtherShamirRecoveryInfoSetupButUnusable
  | OtherShamirRecoveryInfoSetupWithRevokedRecipients


// ParseParsecAddrError
export interface ParseParsecAddrErrorInvalidUrl {
    tag: "InvalidUrl"
    error: string
}
export type ParseParsecAddrError =
  | ParseParsecAddrErrorInvalidUrl


// ParsedParsecAddr
export interface ParsedParsecAddrInvitationDevice {
    tag: "InvitationDevice"
    hostname: string
    port: number
    use_ssl: boolean
    organization_id: string
    token: string
}
export interface ParsedParsecAddrInvitationShamirRecovery {
    tag: "InvitationShamirRecovery"
    hostname: string
    port: number
    use_ssl: boolean
    organization_id: string
    token: string
}
export interface ParsedParsecAddrInvitationUser {
    tag: "InvitationUser"
    hostname: string
    port: number
    use_ssl: boolean
    organization_id: string
    token: string
}
export interface ParsedParsecAddrOrganization {
    tag: "Organization"
    hostname: string
    port: number
    use_ssl: boolean
    organization_id: string
}
export interface ParsedParsecAddrOrganizationBootstrap {
    tag: "OrganizationBootstrap"
    hostname: string
    port: number
    use_ssl: boolean
    organization_id: string
    token: string | null
}
export interface ParsedParsecAddrPkiEnrollment {
    tag: "PkiEnrollment"
    hostname: string
    port: number
    use_ssl: boolean
    organization_id: string
}
export interface ParsedParsecAddrServer {
    tag: "Server"
    hostname: string
    port: number
    use_ssl: boolean
}
export interface ParsedParsecAddrWorkspacePath {
    tag: "WorkspacePath"
    hostname: string
    port: number
    use_ssl: boolean
    organization_id: string
    workspace_id: string
    key_index: number
    encrypted_path: Uint8Array
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
export interface SelfShamirRecoveryInfoDeleted {
    tag: "Deleted"
    created_on: number
    created_by: string
    threshold: number
    per_recipient_shares: Map<string, number>
    deleted_on: number
    deleted_by: string
}
export interface SelfShamirRecoveryInfoNeverSetup {
    tag: "NeverSetup"
}
export interface SelfShamirRecoveryInfoSetupAllValid {
    tag: "SetupAllValid"
    created_on: number
    created_by: string
    threshold: number
    per_recipient_shares: Map<string, number>
}
export interface SelfShamirRecoveryInfoSetupButUnusable {
    tag: "SetupButUnusable"
    created_on: number
    created_by: string
    threshold: number
    per_recipient_shares: Map<string, number>
    revoked_recipients: Array<string>
}
export interface SelfShamirRecoveryInfoSetupWithRevokedRecipients {
    tag: "SetupWithRevokedRecipients"
    created_on: number
    created_by: string
    threshold: number
    per_recipient_shares: Map<string, number>
    revoked_recipients: Array<string>
}
export type SelfShamirRecoveryInfo =
  | SelfShamirRecoveryInfoDeleted
  | SelfShamirRecoveryInfoNeverSetup
  | SelfShamirRecoveryInfoSetupAllValid
  | SelfShamirRecoveryInfoSetupButUnusable
  | SelfShamirRecoveryInfoSetupWithRevokedRecipients


// ShamirRecoveryClaimAddShareError
export interface ShamirRecoveryClaimAddShareErrorCorruptedSecret {
    tag: "CorruptedSecret"
    error: string
}
export interface ShamirRecoveryClaimAddShareErrorInternal {
    tag: "Internal"
    error: string
}
export interface ShamirRecoveryClaimAddShareErrorRecipientNotFound {
    tag: "RecipientNotFound"
    error: string
}
export type ShamirRecoveryClaimAddShareError =
  | ShamirRecoveryClaimAddShareErrorCorruptedSecret
  | ShamirRecoveryClaimAddShareErrorInternal
  | ShamirRecoveryClaimAddShareErrorRecipientNotFound


// ShamirRecoveryClaimMaybeFinalizeInfo
export interface ShamirRecoveryClaimMaybeFinalizeInfoFinalize {
    tag: "Finalize"
    handle: number
}
export interface ShamirRecoveryClaimMaybeFinalizeInfoOffline {
    tag: "Offline"
    handle: number
}
export type ShamirRecoveryClaimMaybeFinalizeInfo =
  | ShamirRecoveryClaimMaybeFinalizeInfoFinalize
  | ShamirRecoveryClaimMaybeFinalizeInfoOffline


// ShamirRecoveryClaimMaybeRecoverDeviceInfo
export interface ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient {
    tag: "PickRecipient"
    handle: number
    claimer_user_id: string
    claimer_human_handle: HumanHandle
    shamir_recovery_created_on: number
    recipients: Array<ShamirRecoveryRecipient>
    threshold: number
    recovered_shares: Map<string, number>
    is_recoverable: boolean
}
export interface ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice {
    tag: "RecoverDevice"
    handle: number
    claimer_user_id: string
    claimer_human_handle: HumanHandle
}
export type ShamirRecoveryClaimMaybeRecoverDeviceInfo =
  | ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient
  | ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice


// ShamirRecoveryClaimPickRecipientError
export interface ShamirRecoveryClaimPickRecipientErrorInternal {
    tag: "Internal"
    error: string
}
export interface ShamirRecoveryClaimPickRecipientErrorRecipientAlreadyPicked {
    tag: "RecipientAlreadyPicked"
    error: string
}
export interface ShamirRecoveryClaimPickRecipientErrorRecipientNotFound {
    tag: "RecipientNotFound"
    error: string
}
export interface ShamirRecoveryClaimPickRecipientErrorRecipientRevoked {
    tag: "RecipientRevoked"
    error: string
}
export type ShamirRecoveryClaimPickRecipientError =
  | ShamirRecoveryClaimPickRecipientErrorInternal
  | ShamirRecoveryClaimPickRecipientErrorRecipientAlreadyPicked
  | ShamirRecoveryClaimPickRecipientErrorRecipientNotFound
  | ShamirRecoveryClaimPickRecipientErrorRecipientRevoked


// ShamirRecoveryClaimRecoverDeviceError
export interface ShamirRecoveryClaimRecoverDeviceErrorAlreadyUsed {
    tag: "AlreadyUsed"
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorCipheredDataNotFound {
    tag: "CipheredDataNotFound"
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorCorruptedCipheredData {
    tag: "CorruptedCipheredData"
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorInternal {
    tag: "Internal"
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorNotFound {
    tag: "NotFound"
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorOrganizationExpired {
    tag: "OrganizationExpired"
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorRegisterNewDeviceError {
    tag: "RegisterNewDeviceError"
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
export interface TestbedErrorDisabled {
    tag: "Disabled"
    error: string
}
export interface TestbedErrorInternal {
    tag: "Internal"
    error: string
}
export type TestbedError =
  | TestbedErrorDisabled
  | TestbedErrorInternal


// WaitForDeviceAvailableError
export interface WaitForDeviceAvailableErrorInternal {
    tag: "Internal"
    error: string
}
export type WaitForDeviceAvailableError =
  | WaitForDeviceAvailableErrorInternal


// WorkspaceCreateFileError
export interface WorkspaceCreateFileErrorEntryExists {
    tag: "EntryExists"
    error: string
}
export interface WorkspaceCreateFileErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceCreateFileErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceCreateFileErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceCreateFileErrorInvalidManifest {
    tag: "InvalidManifest"
    error: string
}
export interface WorkspaceCreateFileErrorNoRealmAccess {
    tag: "NoRealmAccess"
    error: string
}
export interface WorkspaceCreateFileErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceCreateFileErrorParentNotAFolder {
    tag: "ParentNotAFolder"
    error: string
}
export interface WorkspaceCreateFileErrorParentNotFound {
    tag: "ParentNotFound"
    error: string
}
export interface WorkspaceCreateFileErrorReadOnlyRealm {
    tag: "ReadOnlyRealm"
    error: string
}
export interface WorkspaceCreateFileErrorStopped {
    tag: "Stopped"
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
export interface WorkspaceCreateFolderErrorEntryExists {
    tag: "EntryExists"
    error: string
}
export interface WorkspaceCreateFolderErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceCreateFolderErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceCreateFolderErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceCreateFolderErrorInvalidManifest {
    tag: "InvalidManifest"
    error: string
}
export interface WorkspaceCreateFolderErrorNoRealmAccess {
    tag: "NoRealmAccess"
    error: string
}
export interface WorkspaceCreateFolderErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceCreateFolderErrorParentNotAFolder {
    tag: "ParentNotAFolder"
    error: string
}
export interface WorkspaceCreateFolderErrorParentNotFound {
    tag: "ParentNotFound"
    error: string
}
export interface WorkspaceCreateFolderErrorReadOnlyRealm {
    tag: "ReadOnlyRealm"
    error: string
}
export interface WorkspaceCreateFolderErrorStopped {
    tag: "Stopped"
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
export interface WorkspaceDecryptPathAddrErrorCorruptedData {
    tag: "CorruptedData"
    error: string
}
export interface WorkspaceDecryptPathAddrErrorCorruptedKey {
    tag: "CorruptedKey"
    error: string
}
export interface WorkspaceDecryptPathAddrErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceDecryptPathAddrErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceDecryptPathAddrErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceDecryptPathAddrErrorKeyNotFound {
    tag: "KeyNotFound"
    error: string
}
export interface WorkspaceDecryptPathAddrErrorNotAllowed {
    tag: "NotAllowed"
    error: string
}
export interface WorkspaceDecryptPathAddrErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceDecryptPathAddrErrorStopped {
    tag: "Stopped"
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
export interface WorkspaceFdCloseErrorBadFileDescriptor {
    tag: "BadFileDescriptor"
    error: string
}
export interface WorkspaceFdCloseErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceFdCloseErrorStopped {
    tag: "Stopped"
    error: string
}
export type WorkspaceFdCloseError =
  | WorkspaceFdCloseErrorBadFileDescriptor
  | WorkspaceFdCloseErrorInternal
  | WorkspaceFdCloseErrorStopped


// WorkspaceFdFlushError
export interface WorkspaceFdFlushErrorBadFileDescriptor {
    tag: "BadFileDescriptor"
    error: string
}
export interface WorkspaceFdFlushErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceFdFlushErrorNotInWriteMode {
    tag: "NotInWriteMode"
    error: string
}
export interface WorkspaceFdFlushErrorStopped {
    tag: "Stopped"
    error: string
}
export type WorkspaceFdFlushError =
  | WorkspaceFdFlushErrorBadFileDescriptor
  | WorkspaceFdFlushErrorInternal
  | WorkspaceFdFlushErrorNotInWriteMode
  | WorkspaceFdFlushErrorStopped


// WorkspaceFdReadError
export interface WorkspaceFdReadErrorBadFileDescriptor {
    tag: "BadFileDescriptor"
    error: string
}
export interface WorkspaceFdReadErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceFdReadErrorInvalidBlockAccess {
    tag: "InvalidBlockAccess"
    error: string
}
export interface WorkspaceFdReadErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceFdReadErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceFdReadErrorNoRealmAccess {
    tag: "NoRealmAccess"
    error: string
}
export interface WorkspaceFdReadErrorNotInReadMode {
    tag: "NotInReadMode"
    error: string
}
export interface WorkspaceFdReadErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceFdReadErrorStopped {
    tag: "Stopped"
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
export interface WorkspaceFdResizeErrorBadFileDescriptor {
    tag: "BadFileDescriptor"
    error: string
}
export interface WorkspaceFdResizeErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceFdResizeErrorNotInWriteMode {
    tag: "NotInWriteMode"
    error: string
}
export type WorkspaceFdResizeError =
  | WorkspaceFdResizeErrorBadFileDescriptor
  | WorkspaceFdResizeErrorInternal
  | WorkspaceFdResizeErrorNotInWriteMode


// WorkspaceFdStatError
export interface WorkspaceFdStatErrorBadFileDescriptor {
    tag: "BadFileDescriptor"
    error: string
}
export interface WorkspaceFdStatErrorInternal {
    tag: "Internal"
    error: string
}
export type WorkspaceFdStatError =
  | WorkspaceFdStatErrorBadFileDescriptor
  | WorkspaceFdStatErrorInternal


// WorkspaceFdWriteError
export interface WorkspaceFdWriteErrorBadFileDescriptor {
    tag: "BadFileDescriptor"
    error: string
}
export interface WorkspaceFdWriteErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceFdWriteErrorNotInWriteMode {
    tag: "NotInWriteMode"
    error: string
}
export type WorkspaceFdWriteError =
  | WorkspaceFdWriteErrorBadFileDescriptor
  | WorkspaceFdWriteErrorInternal
  | WorkspaceFdWriteErrorNotInWriteMode


// WorkspaceGeneratePathAddrError
export interface WorkspaceGeneratePathAddrErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceGeneratePathAddrErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceGeneratePathAddrErrorNoKey {
    tag: "NoKey"
    error: string
}
export interface WorkspaceGeneratePathAddrErrorNotAllowed {
    tag: "NotAllowed"
    error: string
}
export interface WorkspaceGeneratePathAddrErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceGeneratePathAddrErrorStopped {
    tag: "Stopped"
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
export interface WorkspaceHistoryEntryStatFile {
    tag: "File"
    id: string
    parent: string
    created: number
    updated: number
    version: number
    size: number
}
export interface WorkspaceHistoryEntryStatFolder {
    tag: "Folder"
    id: string
    parent: string
    created: number
    updated: number
    version: number
}
export type WorkspaceHistoryEntryStat =
  | WorkspaceHistoryEntryStatFile
  | WorkspaceHistoryEntryStatFolder


// WorkspaceHistoryFdCloseError
export interface WorkspaceHistoryFdCloseErrorBadFileDescriptor {
    tag: "BadFileDescriptor"
    error: string
}
export interface WorkspaceHistoryFdCloseErrorInternal {
    tag: "Internal"
    error: string
}
export type WorkspaceHistoryFdCloseError =
  | WorkspaceHistoryFdCloseErrorBadFileDescriptor
  | WorkspaceHistoryFdCloseErrorInternal


// WorkspaceHistoryFdReadError
export interface WorkspaceHistoryFdReadErrorBadFileDescriptor {
    tag: "BadFileDescriptor"
    error: string
}
export interface WorkspaceHistoryFdReadErrorBlockNotFound {
    tag: "BlockNotFound"
    error: string
}
export interface WorkspaceHistoryFdReadErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceHistoryFdReadErrorInvalidBlockAccess {
    tag: "InvalidBlockAccess"
    error: string
}
export interface WorkspaceHistoryFdReadErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceHistoryFdReadErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceHistoryFdReadErrorNoRealmAccess {
    tag: "NoRealmAccess"
    error: string
}
export interface WorkspaceHistoryFdReadErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceHistoryFdReadErrorStopped {
    tag: "Stopped"
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
export interface WorkspaceHistoryFdStatErrorBadFileDescriptor {
    tag: "BadFileDescriptor"
    error: string
}
export interface WorkspaceHistoryFdStatErrorInternal {
    tag: "Internal"
    error: string
}
export type WorkspaceHistoryFdStatError =
  | WorkspaceHistoryFdStatErrorBadFileDescriptor
  | WorkspaceHistoryFdStatErrorInternal


// WorkspaceHistoryGetWorkspaceManifestV1TimestampError
export interface WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInvalidManifest {
    tag: "InvalidManifest"
    error: string
}
export interface WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorNoRealmAccess {
    tag: "NoRealmAccess"
    error: string
}
export interface WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorStopped {
    tag: "Stopped"
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
export interface WorkspaceHistoryOpenFileErrorEntryNotAFile {
    tag: "EntryNotAFile"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorEntryNotFound {
    tag: "EntryNotFound"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInvalidManifest {
    tag: "InvalidManifest"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorNoRealmAccess {
    tag: "NoRealmAccess"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorStopped {
    tag: "Stopped"
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
export interface WorkspaceHistoryStatEntryErrorEntryNotFound {
    tag: "EntryNotFound"
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInvalidManifest {
    tag: "InvalidManifest"
    error: string
}
export interface WorkspaceHistoryStatEntryErrorNoRealmAccess {
    tag: "NoRealmAccess"
    error: string
}
export interface WorkspaceHistoryStatEntryErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceHistoryStatEntryErrorStopped {
    tag: "Stopped"
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
export interface WorkspaceHistoryStatFolderChildrenErrorEntryIsFile {
    tag: "EntryIsFile"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorEntryNotFound {
    tag: "EntryNotFound"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInvalidManifest {
    tag: "InvalidManifest"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorNoRealmAccess {
    tag: "NoRealmAccess"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorStopped {
    tag: "Stopped"
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
export interface WorkspaceInfoErrorInternal {
    tag: "Internal"
    error: string
}
export type WorkspaceInfoError =
  | WorkspaceInfoErrorInternal


// WorkspaceMountError
export interface WorkspaceMountErrorDisabled {
    tag: "Disabled"
    error: string
}
export interface WorkspaceMountErrorInternal {
    tag: "Internal"
    error: string
}
export type WorkspaceMountError =
  | WorkspaceMountErrorDisabled
  | WorkspaceMountErrorInternal


// WorkspaceMoveEntryError
export interface WorkspaceMoveEntryErrorCannotMoveRoot {
    tag: "CannotMoveRoot"
    error: string
}
export interface WorkspaceMoveEntryErrorDestinationExists {
    tag: "DestinationExists"
    error: string
}
export interface WorkspaceMoveEntryErrorDestinationNotFound {
    tag: "DestinationNotFound"
    error: string
}
export interface WorkspaceMoveEntryErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceMoveEntryErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceMoveEntryErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceMoveEntryErrorInvalidManifest {
    tag: "InvalidManifest"
    error: string
}
export interface WorkspaceMoveEntryErrorNoRealmAccess {
    tag: "NoRealmAccess"
    error: string
}
export interface WorkspaceMoveEntryErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceMoveEntryErrorReadOnlyRealm {
    tag: "ReadOnlyRealm"
    error: string
}
export interface WorkspaceMoveEntryErrorSourceNotFound {
    tag: "SourceNotFound"
    error: string
}
export interface WorkspaceMoveEntryErrorStopped {
    tag: "Stopped"
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
export interface WorkspaceOpenFileErrorEntryExistsInCreateNewMode {
    tag: "EntryExistsInCreateNewMode"
    error: string
}
export interface WorkspaceOpenFileErrorEntryNotAFile {
    tag: "EntryNotAFile"
    error: string
}
export interface WorkspaceOpenFileErrorEntryNotFound {
    tag: "EntryNotFound"
    error: string
}
export interface WorkspaceOpenFileErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceOpenFileErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceOpenFileErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceOpenFileErrorInvalidManifest {
    tag: "InvalidManifest"
    error: string
}
export interface WorkspaceOpenFileErrorNoRealmAccess {
    tag: "NoRealmAccess"
    error: string
}
export interface WorkspaceOpenFileErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceOpenFileErrorReadOnlyRealm {
    tag: "ReadOnlyRealm"
    error: string
}
export interface WorkspaceOpenFileErrorStopped {
    tag: "Stopped"
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
export interface WorkspaceRemoveEntryErrorCannotRemoveRoot {
    tag: "CannotRemoveRoot"
    error: string
}
export interface WorkspaceRemoveEntryErrorEntryIsFile {
    tag: "EntryIsFile"
    error: string
}
export interface WorkspaceRemoveEntryErrorEntryIsFolder {
    tag: "EntryIsFolder"
    error: string
}
export interface WorkspaceRemoveEntryErrorEntryIsNonEmptyFolder {
    tag: "EntryIsNonEmptyFolder"
    error: string
}
export interface WorkspaceRemoveEntryErrorEntryNotFound {
    tag: "EntryNotFound"
    error: string
}
export interface WorkspaceRemoveEntryErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceRemoveEntryErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceRemoveEntryErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceRemoveEntryErrorInvalidManifest {
    tag: "InvalidManifest"
    error: string
}
export interface WorkspaceRemoveEntryErrorNoRealmAccess {
    tag: "NoRealmAccess"
    error: string
}
export interface WorkspaceRemoveEntryErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceRemoveEntryErrorReadOnlyRealm {
    tag: "ReadOnlyRealm"
    error: string
}
export interface WorkspaceRemoveEntryErrorStopped {
    tag: "Stopped"
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
export interface WorkspaceStatEntryErrorEntryNotFound {
    tag: "EntryNotFound"
    error: string
}
export interface WorkspaceStatEntryErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceStatEntryErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceStatEntryErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceStatEntryErrorInvalidManifest {
    tag: "InvalidManifest"
    error: string
}
export interface WorkspaceStatEntryErrorNoRealmAccess {
    tag: "NoRealmAccess"
    error: string
}
export interface WorkspaceStatEntryErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceStatEntryErrorStopped {
    tag: "Stopped"
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
export interface WorkspaceStatFolderChildrenErrorEntryIsFile {
    tag: "EntryIsFile"
    error: string
}
export interface WorkspaceStatFolderChildrenErrorEntryNotFound {
    tag: "EntryNotFound"
    error: string
}
export interface WorkspaceStatFolderChildrenErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceStatFolderChildrenErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceStatFolderChildrenErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceStatFolderChildrenErrorInvalidManifest {
    tag: "InvalidManifest"
    error: string
}
export interface WorkspaceStatFolderChildrenErrorNoRealmAccess {
    tag: "NoRealmAccess"
    error: string
}
export interface WorkspaceStatFolderChildrenErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceStatFolderChildrenErrorStopped {
    tag: "Stopped"
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
export interface WorkspaceStopErrorInternal {
    tag: "Internal"
    error: string
}
export type WorkspaceStopError =
  | WorkspaceStopErrorInternal


// WorkspaceStorageCacheSize
export interface WorkspaceStorageCacheSizeCustom {
    tag: "Custom"
    size: number
}
export interface WorkspaceStorageCacheSizeDefault {
    tag: "Default"
}
export type WorkspaceStorageCacheSize =
  | WorkspaceStorageCacheSizeCustom
  | WorkspaceStorageCacheSizeDefault


// WorkspaceWatchEntryOneShotError
export interface WorkspaceWatchEntryOneShotErrorEntryNotFound {
    tag: "EntryNotFound"
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorInvalidManifest {
    tag: "InvalidManifest"
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorNoRealmAccess {
    tag: "NoRealmAccess"
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorStopped {
    tag: "Stopped"
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


export function archiveDevice(
    device_path: string
): Promise<Result<null, ArchiveDeviceError>>
export function bootstrapOrganization(
    config: ClientConfig,
    on_event_callback: (handle: number, event: ClientEvent) => void,
    bootstrap_organization_addr: string,
    save_strategy: DeviceSaveStrategy,
    human_handle: HumanHandle,
    device_label: string,
    sequester_authority_verify_key: Uint8Array | null
): Promise<Result<AvailableDevice, BootstrapOrganizationError>>
export function buildParsecOrganizationBootstrapAddr(
    addr: string,
    organization_id: string
): Promise<string>
export function cancel(
    canceller: number
): Promise<Result<null, CancelError>>
export function claimerDeviceFinalizeSaveLocalDevice(
    handle: number,
    save_strategy: DeviceSaveStrategy
): Promise<Result<AvailableDevice, ClaimInProgressError>>
export function claimerDeviceInProgress1DoDenyTrust(
    canceller: number,
    handle: number
): Promise<Result<null, ClaimInProgressError>>
export function claimerDeviceInProgress1DoSignifyTrust(
    canceller: number,
    handle: number
): Promise<Result<DeviceClaimInProgress2Info, ClaimInProgressError>>
export function claimerDeviceInProgress2DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<DeviceClaimInProgress3Info, ClaimInProgressError>>
export function claimerDeviceInProgress3DoClaim(
    canceller: number,
    handle: number,
    requested_device_label: string
): Promise<Result<DeviceClaimFinalizeInfo, ClaimInProgressError>>
export function claimerDeviceInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<DeviceClaimInProgress1Info, ClaimInProgressError>>
export function claimerGreeterAbortOperation(
    handle: number
): Promise<Result<null, ClaimerGreeterAbortOperationError>>
export function claimerRetrieveInfo(
    config: ClientConfig,
    on_event_callback: (handle: number, event: ClientEvent) => void,
    addr: string
): Promise<Result<AnyClaimRetrievedInfo, ClaimerRetrieveInfoError>>
export function claimerShamirRecoveryAddShare(
    recipient_pick_handle: number,
    share_handle: number
): Promise<Result<ShamirRecoveryClaimMaybeRecoverDeviceInfo, ShamirRecoveryClaimAddShareError>>
export function claimerShamirRecoveryFinalizeSaveLocalDevice(
    handle: number,
    save_strategy: DeviceSaveStrategy
): Promise<Result<AvailableDevice, ClaimInProgressError>>
export function claimerShamirRecoveryInProgress1DoDenyTrust(
    canceller: number,
    handle: number
): Promise<Result<null, ClaimInProgressError>>
export function claimerShamirRecoveryInProgress1DoSignifyTrust(
    canceller: number,
    handle: number
): Promise<Result<ShamirRecoveryClaimInProgress2Info, ClaimInProgressError>>
export function claimerShamirRecoveryInProgress2DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<ShamirRecoveryClaimInProgress3Info, ClaimInProgressError>>
export function claimerShamirRecoveryInProgress3DoClaim(
    canceller: number,
    handle: number
): Promise<Result<ShamirRecoveryClaimShareInfo, ClaimInProgressError>>
export function claimerShamirRecoveryInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<ShamirRecoveryClaimInProgress1Info, ClaimInProgressError>>
export function claimerShamirRecoveryPickRecipient(
    handle: number,
    recipient_user_id: string
): Promise<Result<ShamirRecoveryClaimInitialInfo, ShamirRecoveryClaimPickRecipientError>>
export function claimerShamirRecoveryRecoverDevice(
    handle: number,
    requested_device_label: string
): Promise<Result<ShamirRecoveryClaimMaybeFinalizeInfo, ShamirRecoveryClaimRecoverDeviceError>>
export function claimerUserFinalizeSaveLocalDevice(
    handle: number,
    save_strategy: DeviceSaveStrategy
): Promise<Result<AvailableDevice, ClaimInProgressError>>
export function claimerUserInProgress1DoDenyTrust(
    canceller: number,
    handle: number
): Promise<Result<null, ClaimInProgressError>>
export function claimerUserInProgress1DoSignifyTrust(
    canceller: number,
    handle: number
): Promise<Result<UserClaimInProgress2Info, ClaimInProgressError>>
export function claimerUserInProgress2DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<UserClaimInProgress3Info, ClaimInProgressError>>
export function claimerUserInProgress3DoClaim(
    canceller: number,
    handle: number,
    requested_device_label: string,
    requested_human_handle: HumanHandle
): Promise<Result<UserClaimFinalizeInfo, ClaimInProgressError>>
export function claimerUserInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<UserClaimInProgress1Info, ClaimInProgressError>>
export function clientAcceptTos(
    client: number,
    tos_updated_on: number
): Promise<Result<null, ClientAcceptTosError>>
export function clientCancelInvitation(
    client: number,
    token: string
): Promise<Result<null, ClientCancelInvitationError>>
export function clientChangeAuthentication(
    client_config: ClientConfig,
    current_auth: DeviceAccessStrategy,
    new_auth: DeviceSaveStrategy
): Promise<Result<null, ClientChangeAuthenticationError>>
export function clientCreateWorkspace(
    client: number,
    name: string
): Promise<Result<string, ClientCreateWorkspaceError>>
export function clientDeleteShamirRecovery(
    client_handle: number
): Promise<Result<null, ClientDeleteShamirRecoveryError>>
export function clientExportRecoveryDevice(
    client_handle: number,
    device_label: string
): Promise<Result<[string, Uint8Array], ClientExportRecoveryDeviceError>>
export function clientGetSelfShamirRecovery(
    client_handle: number
): Promise<Result<SelfShamirRecoveryInfo, ClientGetSelfShamirRecoveryError>>
export function clientGetTos(
    client: number
): Promise<Result<Tos, ClientGetTosError>>
export function clientGetUserDevice(
    client: number,
    device: string
): Promise<Result<[UserInfo, DeviceInfo], ClientGetUserDeviceError>>
export function clientInfo(
    client: number
): Promise<Result<ClientInfo, ClientInfoError>>
export function clientListFrozenUsers(
    client_handle: number
): Promise<Result<Array<string>, ClientListFrozenUsersError>>
export function clientListInvitations(
    client: number
): Promise<Result<Array<InviteListItem>, ListInvitationsError>>
export function clientListShamirRecoveriesForOthers(
    client_handle: number
): Promise<Result<Array<OtherShamirRecoveryInfo>, ClientListShamirRecoveriesForOthersError>>
export function clientListUserDevices(
    client: number,
    user: string
): Promise<Result<Array<DeviceInfo>, ClientListUserDevicesError>>
export function clientListUsers(
    client: number,
    skip_revoked: boolean
): Promise<Result<Array<UserInfo>, ClientListUsersError>>
export function clientListWorkspaceUsers(
    client: number,
    realm_id: string
): Promise<Result<Array<WorkspaceUserAccessInfo>, ClientListWorkspaceUsersError>>
export function clientListWorkspaces(
    client: number
): Promise<Result<Array<WorkspaceInfo>, ClientListWorkspacesError>>
export function clientNewDeviceInvitation(
    client: number,
    send_email: boolean
): Promise<Result<NewInvitationInfo, ClientNewDeviceInvitationError>>
export function clientNewShamirRecoveryInvitation(
    client: number,
    claimer_user_id: string,
    send_email: boolean
): Promise<Result<NewInvitationInfo, ClientNewShamirRecoveryInvitationError>>
export function clientNewUserInvitation(
    client: number,
    claimer_email: string,
    send_email: boolean
): Promise<Result<NewInvitationInfo, ClientNewUserInvitationError>>
export function clientRenameWorkspace(
    client: number,
    realm_id: string,
    new_name: string
): Promise<Result<null, ClientRenameWorkspaceError>>
export function clientRevokeUser(
    client: number,
    user: string
): Promise<Result<null, ClientRevokeUserError>>
export function clientSetupShamirRecovery(
    client_handle: number,
    per_recipient_shares: Map<string, number>,
    threshold: number
): Promise<Result<null, ClientSetupShamirRecoveryError>>
export function clientShareWorkspace(
    client: number,
    realm_id: string,
    recipient: string,
    role: RealmRole | null
): Promise<Result<null, ClientShareWorkspaceError>>
export function clientStart(
    config: ClientConfig,
    on_event_callback: (handle: number, event: ClientEvent) => void,
    access: DeviceAccessStrategy
): Promise<Result<number, ClientStartError>>
export function clientStartDeviceInvitationGreet(
    client: number,
    token: string
): Promise<Result<DeviceGreetInitialInfo, ClientStartInvitationGreetError>>
export function clientStartShamirRecoveryInvitationGreet(
    client: number,
    token: string
): Promise<Result<ShamirRecoveryGreetInitialInfo, ClientStartShamirRecoveryInvitationGreetError>>
export function clientStartUserInvitationGreet(
    client: number,
    token: string
): Promise<Result<UserGreetInitialInfo, ClientStartInvitationGreetError>>
export function clientStartWorkspace(
    client: number,
    realm_id: string
): Promise<Result<number, ClientStartWorkspaceError>>
export function clientStop(
    client: number
): Promise<Result<null, ClientStopError>>
export function getDefaultConfigDir(
): Promise<string>
export function getDefaultDataBaseDir(
): Promise<string>
export function getDefaultMountpointBaseDir(
): Promise<string>
export function getPlatform(
): Promise<Platform>
export function greeterDeviceInProgress1DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<DeviceGreetInProgress2Info, GreetInProgressError>>
export function greeterDeviceInProgress2DoDenyTrust(
    canceller: number,
    handle: number
): Promise<Result<null, GreetInProgressError>>
export function greeterDeviceInProgress2DoSignifyTrust(
    canceller: number,
    handle: number
): Promise<Result<DeviceGreetInProgress3Info, GreetInProgressError>>
export function greeterDeviceInProgress3DoGetClaimRequests(
    canceller: number,
    handle: number
): Promise<Result<DeviceGreetInProgress4Info, GreetInProgressError>>
export function greeterDeviceInProgress4DoCreate(
    canceller: number,
    handle: number,
    device_label: string
): Promise<Result<null, GreetInProgressError>>
export function greeterDeviceInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<DeviceGreetInProgress1Info, GreetInProgressError>>
export function greeterShamirRecoveryInProgress1DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<ShamirRecoveryGreetInProgress2Info, GreetInProgressError>>
export function greeterShamirRecoveryInProgress2DoDenyTrust(
    canceller: number,
    handle: number
): Promise<Result<null, GreetInProgressError>>
export function greeterShamirRecoveryInProgress2DoSignifyTrust(
    canceller: number,
    handle: number
): Promise<Result<ShamirRecoveryGreetInProgress3Info, GreetInProgressError>>
export function greeterShamirRecoveryInProgress3DoGetClaimRequests(
    canceller: number,
    handle: number
): Promise<Result<null, GreetInProgressError>>
export function greeterShamirRecoveryInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<ShamirRecoveryGreetInProgress1Info, GreetInProgressError>>
export function greeterUserInProgress1DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<UserGreetInProgress2Info, GreetInProgressError>>
export function greeterUserInProgress2DoDenyTrust(
    canceller: number,
    handle: number
): Promise<Result<null, GreetInProgressError>>
export function greeterUserInProgress2DoSignifyTrust(
    canceller: number,
    handle: number
): Promise<Result<UserGreetInProgress3Info, GreetInProgressError>>
export function greeterUserInProgress3DoGetClaimRequests(
    canceller: number,
    handle: number
): Promise<Result<UserGreetInProgress4Info, GreetInProgressError>>
export function greeterUserInProgress4DoCreate(
    canceller: number,
    handle: number,
    human_handle: HumanHandle,
    device_label: string,
    profile: UserProfile
): Promise<Result<null, GreetInProgressError>>
export function greeterUserInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<UserGreetInProgress1Info, GreetInProgressError>>
export function importRecoveryDevice(
    config: ClientConfig,
    recovery_device: Uint8Array,
    passphrase: string,
    device_label: string,
    save_strategy: DeviceSaveStrategy
): Promise<Result<AvailableDevice, ImportRecoveryDeviceError>>
export function isKeyringAvailable(
): Promise<boolean>
export function listAvailableDevices(
    path: string
): Promise<Array<AvailableDevice>>
export function mountpointToOsPath(
    mountpoint: number,
    parsec_path: string
): Promise<Result<string, MountpointToOsPathError>>
export function mountpointUnmount(
    mountpoint: number
): Promise<Result<null, MountpointUnmountError>>
export function newCanceller(
): Promise<number>
export function parseParsecAddr(
    url: string
): Promise<Result<ParsedParsecAddr, ParseParsecAddrError>>
export function pathFilename(
    path: string
): Promise<string | null>
export function pathJoin(
    parent: string,
    child: string
): Promise<string>
export function pathNormalize(
    path: string
): Promise<string>
export function pathParent(
    path: string
): Promise<string>
export function pathSplit(
    path: string
): Promise<Array<string>>
export function testDropTestbed(
    path: string
): Promise<Result<null, TestbedError>>
export function testGetTestbedBootstrapOrganizationAddr(
    discriminant_dir: string
): Promise<Result<string | null, TestbedError>>
export function testGetTestbedOrganizationId(
    discriminant_dir: string
): Promise<Result<string | null, TestbedError>>
export function testNewTestbed(
    template: string,
    test_server: string | null
): Promise<Result<string, TestbedError>>
export function validateDeviceLabel(
    raw: string
): Promise<boolean>
export function validateEmail(
    raw: string
): Promise<boolean>
export function validateEntryName(
    raw: string
): Promise<boolean>
export function validateHumanHandleLabel(
    raw: string
): Promise<boolean>
export function validateInvitationToken(
    raw: string
): Promise<boolean>
export function validateOrganizationId(
    raw: string
): Promise<boolean>
export function validatePath(
    raw: string
): Promise<boolean>
export function waitForDeviceAvailable(
    config_dir: string,
    device_id: string
): Promise<Result<null, WaitForDeviceAvailableError>>
export function workspaceCreateFile(
    workspace: number,
    path: string
): Promise<Result<string, WorkspaceCreateFileError>>
export function workspaceCreateFolder(
    workspace: number,
    path: string
): Promise<Result<string, WorkspaceCreateFolderError>>
export function workspaceCreateFolderAll(
    workspace: number,
    path: string
): Promise<Result<string, WorkspaceCreateFolderError>>
export function workspaceDecryptPathAddr(
    workspace: number,
    link: string
): Promise<Result<string, WorkspaceDecryptPathAddrError>>
export function workspaceFdClose(
    workspace: number,
    fd: number
): Promise<Result<null, WorkspaceFdCloseError>>
export function workspaceFdFlush(
    workspace: number,
    fd: number
): Promise<Result<null, WorkspaceFdFlushError>>
export function workspaceFdRead(
    workspace: number,
    fd: number,
    offset: number,
    size: number
): Promise<Result<Uint8Array, WorkspaceFdReadError>>
export function workspaceFdResize(
    workspace: number,
    fd: number,
    length: number,
    truncate_only: boolean
): Promise<Result<null, WorkspaceFdResizeError>>
export function workspaceFdStat(
    workspace: number,
    fd: number
): Promise<Result<FileStat, WorkspaceFdStatError>>
export function workspaceFdWrite(
    workspace: number,
    fd: number,
    offset: number,
    data: Uint8Array
): Promise<Result<number, WorkspaceFdWriteError>>
export function workspaceFdWriteConstrainedIo(
    workspace: number,
    fd: number,
    offset: number,
    data: Uint8Array
): Promise<Result<number, WorkspaceFdWriteError>>
export function workspaceFdWriteStartEof(
    workspace: number,
    fd: number,
    data: Uint8Array
): Promise<Result<number, WorkspaceFdWriteError>>
export function workspaceGeneratePathAddr(
    workspace: number,
    path: string
): Promise<Result<string, WorkspaceGeneratePathAddrError>>
export function workspaceHistoryFdClose(
    workspace: number,
    fd: number
): Promise<Result<null, WorkspaceHistoryFdCloseError>>
export function workspaceHistoryFdRead(
    workspace: number,
    fd: number,
    offset: number,
    size: number
): Promise<Result<Uint8Array, WorkspaceHistoryFdReadError>>
export function workspaceHistoryFdStat(
    workspace: number,
    fd: number
): Promise<Result<WorkspaceHistoryFileStat, WorkspaceHistoryFdStatError>>
export function workspaceHistoryGetWorkspaceManifestV1Timestamp(
    workspace: number
): Promise<Result<number | null, WorkspaceHistoryGetWorkspaceManifestV1TimestampError>>
export function workspaceHistoryOpenFile(
    workspace: number,
    at: number,
    path: string
): Promise<Result<number, WorkspaceHistoryOpenFileError>>
export function workspaceHistoryOpenFileById(
    workspace: number,
    at: number,
    entry_id: string
): Promise<Result<number, WorkspaceHistoryOpenFileError>>
export function workspaceHistoryStatEntry(
    workspace: number,
    at: number,
    path: string
): Promise<Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError>>
export function workspaceHistoryStatEntryById(
    workspace: number,
    at: number,
    entry_id: string
): Promise<Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError>>
export function workspaceHistoryStatFolderChildren(
    workspace: number,
    at: number,
    path: string
): Promise<Result<Array<[string, WorkspaceHistoryEntryStat]>, WorkspaceHistoryStatFolderChildrenError>>
export function workspaceHistoryStatFolderChildrenById(
    workspace: number,
    at: number,
    entry_id: string
): Promise<Result<Array<[string, WorkspaceHistoryEntryStat]>, WorkspaceHistoryStatFolderChildrenError>>
export function workspaceInfo(
    workspace: number
): Promise<Result<StartedWorkspaceInfo, WorkspaceInfoError>>
export function workspaceMount(
    workspace: number
): Promise<Result<[number, string], WorkspaceMountError>>
export function workspaceMoveEntry(
    workspace: number,
    src: string,
    dst: string,
    mode: MoveEntryMode
): Promise<Result<null, WorkspaceMoveEntryError>>
export function workspaceOpenFile(
    workspace: number,
    path: string,
    mode: OpenOptions
): Promise<Result<number, WorkspaceOpenFileError>>
export function workspaceOpenFileAndGetId(
    workspace: number,
    path: string,
    mode: OpenOptions
): Promise<Result<[number, string], WorkspaceOpenFileError>>
export function workspaceOpenFileById(
    workspace: number,
    entry_id: string,
    mode: OpenOptions
): Promise<Result<number, WorkspaceOpenFileError>>
export function workspaceRemoveEntry(
    workspace: number,
    path: string
): Promise<Result<null, WorkspaceRemoveEntryError>>
export function workspaceRemoveFile(
    workspace: number,
    path: string
): Promise<Result<null, WorkspaceRemoveEntryError>>
export function workspaceRemoveFolder(
    workspace: number,
    path: string
): Promise<Result<null, WorkspaceRemoveEntryError>>
export function workspaceRemoveFolderAll(
    workspace: number,
    path: string
): Promise<Result<null, WorkspaceRemoveEntryError>>
export function workspaceRenameEntryById(
    workspace: number,
    src_parent_id: string,
    src_name: string,
    dst_name: string,
    mode: MoveEntryMode
): Promise<Result<null, WorkspaceMoveEntryError>>
export function workspaceStatEntry(
    workspace: number,
    path: string
): Promise<Result<EntryStat, WorkspaceStatEntryError>>
export function workspaceStatEntryById(
    workspace: number,
    entry_id: string
): Promise<Result<EntryStat, WorkspaceStatEntryError>>
export function workspaceStatEntryByIdIgnoreConfinementPoint(
    workspace: number,
    entry_id: string
): Promise<Result<EntryStat, WorkspaceStatEntryError>>
export function workspaceStatFolderChildren(
    workspace: number,
    path: string
): Promise<Result<Array<[string, EntryStat]>, WorkspaceStatFolderChildrenError>>
export function workspaceStatFolderChildrenById(
    workspace: number,
    entry_id: string
): Promise<Result<Array<[string, EntryStat]>, WorkspaceStatFolderChildrenError>>
export function workspaceStop(
    workspace: number
): Promise<Result<null, WorkspaceStopError>>
export function workspaceWatchEntryOneshot(
    workspace: number,
    path: string
): Promise<Result<string, WorkspaceWatchEntryOneShotError>>
