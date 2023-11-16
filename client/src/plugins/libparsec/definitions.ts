// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */

export type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E }

export enum DeviceFileType {
    Password = 'DeviceFileTypePassword',
    Recovery = 'DeviceFileTypeRecovery',
    Smartcard = 'DeviceFileTypeSmartcard',
}

export enum InvitationEmailSentStatus {
    BadRecipient = 'InvitationEmailSentStatusBadRecipient',
    NotAvailable = 'InvitationEmailSentStatusNotAvailable',
    Success = 'InvitationEmailSentStatusSuccess',
}

export enum InvitationStatus {
    Deleted = 'InvitationStatusDeleted',
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
export type BackendAddr = string
export type BackendInvitationAddr = string
export type BackendOrganizationAddr = string
export type BackendOrganizationBootstrapAddr = string
export type BackendOrganizationFileLinkAddr = string
export type BackendPkiEnrollmentAddr = string
export type DeviceID = string
export type DeviceLabel = string
export type EntryName = string
export type FsPath = string
export type InvitationToken = string
export type OrganizationID = string
export type Password = string
export type Path = string
export type SASCode = string
export type UserID = string
export type VlobID = string
export type SequesterVerifyKeyDer = Uint8Array
export type I32 = number
export type CacheSize = number
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
    mountpointBaseDir: Path
    workspaceStorageCacheSize: WorkspaceStorageCacheSize
}

export interface ClientInfo {
    organizationAddr: BackendOrganizationAddr
    organizationId: OrganizationID
    deviceId: DeviceID
    userId: UserID
    deviceLabel: DeviceLabel
    humanHandle: HumanHandle
    currentProfile: UserProfile
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
    addr: BackendInvitationAddr
    token: InvitationToken
    emailSentStatus: InvitationEmailSentStatus
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
    name: EntryName
    selfCurrentRole: RealmRole
    isStarted: boolean
}

export interface WorkspaceUserAccessInfo {
    userId: UserID
    humanHandle: HumanHandle
    currentProfile: UserProfile
    currentRole: RealmRole
}

// BootstrapOrganizationError
export enum BootstrapOrganizationErrorTag {
    AlreadyUsedToken = 'BootstrapOrganizationErrorAlreadyUsedToken',
    BadTimestamp = 'BootstrapOrganizationErrorBadTimestamp',
    Internal = 'BootstrapOrganizationErrorInternal',
    InvalidToken = 'BootstrapOrganizationErrorInvalidToken',
    Offline = 'BootstrapOrganizationErrorOffline',
    SaveDeviceError = 'BootstrapOrganizationErrorSaveDeviceError',
}

export interface BootstrapOrganizationErrorAlreadyUsedToken {
    tag: BootstrapOrganizationErrorTag.AlreadyUsedToken
    error: string
}
export interface BootstrapOrganizationErrorBadTimestamp {
    tag: BootstrapOrganizationErrorTag.BadTimestamp
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
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
export interface BootstrapOrganizationErrorSaveDeviceError {
    tag: BootstrapOrganizationErrorTag.SaveDeviceError
    error: string
}
export type BootstrapOrganizationError =
  | BootstrapOrganizationErrorAlreadyUsedToken
  | BootstrapOrganizationErrorBadTimestamp
  | BootstrapOrganizationErrorInternal
  | BootstrapOrganizationErrorInvalidToken
  | BootstrapOrganizationErrorOffline
  | BootstrapOrganizationErrorSaveDeviceError

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

// ClientCreateWorkspaceError
export enum ClientCreateWorkspaceErrorTag {
    Internal = 'ClientCreateWorkspaceErrorInternal',
}

export interface ClientCreateWorkspaceErrorInternal {
    tag: ClientCreateWorkspaceErrorTag.Internal
    error: string
}
export type ClientCreateWorkspaceError =
  | ClientCreateWorkspaceErrorInternal

// ClientEvent
export enum ClientEventTag {
    Ping = 'ClientEventPing',
}

export interface ClientEventPing {
    tag: ClientEventTag.Ping
    ping: string
}
export type ClientEvent =
  | ClientEventPing

// ClientGetUserDeviceError
export enum ClientGetUserDeviceErrorTag {
    Internal = 'ClientGetUserDeviceErrorInternal',
    NonExisting = 'ClientGetUserDeviceErrorNonExisting',
}

export interface ClientGetUserDeviceErrorInternal {
    tag: ClientGetUserDeviceErrorTag.Internal
    error: string
}
export interface ClientGetUserDeviceErrorNonExisting {
    tag: ClientGetUserDeviceErrorTag.NonExisting
    error: string
}
export type ClientGetUserDeviceError =
  | ClientGetUserDeviceErrorInternal
  | ClientGetUserDeviceErrorNonExisting

// ClientInfoError
export enum ClientInfoErrorTag {
    Internal = 'ClientInfoErrorInternal',
}

export interface ClientInfoErrorInternal {
    tag: ClientInfoErrorTag.Internal
    error: string
}
export type ClientInfoError =
  | ClientInfoErrorInternal

// ClientListUserDevicesError
export enum ClientListUserDevicesErrorTag {
    Internal = 'ClientListUserDevicesErrorInternal',
}

export interface ClientListUserDevicesErrorInternal {
    tag: ClientListUserDevicesErrorTag.Internal
    error: string
}
export type ClientListUserDevicesError =
  | ClientListUserDevicesErrorInternal

// ClientListUsersError
export enum ClientListUsersErrorTag {
    Internal = 'ClientListUsersErrorInternal',
}

export interface ClientListUsersErrorInternal {
    tag: ClientListUsersErrorTag.Internal
    error: string
}
export type ClientListUsersError =
  | ClientListUsersErrorInternal

// ClientListWorkspaceUsersError
export enum ClientListWorkspaceUsersErrorTag {
    Internal = 'ClientListWorkspaceUsersErrorInternal',
}

export interface ClientListWorkspaceUsersErrorInternal {
    tag: ClientListWorkspaceUsersErrorTag.Internal
    error: string
}
export type ClientListWorkspaceUsersError =
  | ClientListWorkspaceUsersErrorInternal

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

// ClientRenameWorkspaceError
export enum ClientRenameWorkspaceErrorTag {
    Internal = 'ClientRenameWorkspaceErrorInternal',
    UnknownWorkspace = 'ClientRenameWorkspaceErrorUnknownWorkspace',
}

export interface ClientRenameWorkspaceErrorInternal {
    tag: ClientRenameWorkspaceErrorTag.Internal
    error: string
}
export interface ClientRenameWorkspaceErrorUnknownWorkspace {
    tag: ClientRenameWorkspaceErrorTag.UnknownWorkspace
    error: string
}
export type ClientRenameWorkspaceError =
  | ClientRenameWorkspaceErrorInternal
  | ClientRenameWorkspaceErrorUnknownWorkspace

// ClientShareWorkspaceError
export enum ClientShareWorkspaceErrorTag {
    BadTimestamp = 'ClientShareWorkspaceErrorBadTimestamp',
    Internal = 'ClientShareWorkspaceErrorInternal',
    NotAllowed = 'ClientShareWorkspaceErrorNotAllowed',
    Offline = 'ClientShareWorkspaceErrorOffline',
    OutsiderCannotBeManagerOrOwner = 'ClientShareWorkspaceErrorOutsiderCannotBeManagerOrOwner',
    RevokedRecipient = 'ClientShareWorkspaceErrorRevokedRecipient',
    ShareToSelf = 'ClientShareWorkspaceErrorShareToSelf',
    UnknownRecipient = 'ClientShareWorkspaceErrorUnknownRecipient',
    UnknownRecipientOrWorkspace = 'ClientShareWorkspaceErrorUnknownRecipientOrWorkspace',
    UnknownWorkspace = 'ClientShareWorkspaceErrorUnknownWorkspace',
    WorkspaceInMaintenance = 'ClientShareWorkspaceErrorWorkspaceInMaintenance',
}

export interface ClientShareWorkspaceErrorBadTimestamp {
    tag: ClientShareWorkspaceErrorTag.BadTimestamp
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export interface ClientShareWorkspaceErrorInternal {
    tag: ClientShareWorkspaceErrorTag.Internal
    error: string
}
export interface ClientShareWorkspaceErrorNotAllowed {
    tag: ClientShareWorkspaceErrorTag.NotAllowed
    error: string
}
export interface ClientShareWorkspaceErrorOffline {
    tag: ClientShareWorkspaceErrorTag.Offline
    error: string
}
export interface ClientShareWorkspaceErrorOutsiderCannotBeManagerOrOwner {
    tag: ClientShareWorkspaceErrorTag.OutsiderCannotBeManagerOrOwner
    error: string
}
export interface ClientShareWorkspaceErrorRevokedRecipient {
    tag: ClientShareWorkspaceErrorTag.RevokedRecipient
    error: string
}
export interface ClientShareWorkspaceErrorShareToSelf {
    tag: ClientShareWorkspaceErrorTag.ShareToSelf
    error: string
}
export interface ClientShareWorkspaceErrorUnknownRecipient {
    tag: ClientShareWorkspaceErrorTag.UnknownRecipient
    error: string
}
export interface ClientShareWorkspaceErrorUnknownRecipientOrWorkspace {
    tag: ClientShareWorkspaceErrorTag.UnknownRecipientOrWorkspace
    error: string
}
export interface ClientShareWorkspaceErrorUnknownWorkspace {
    tag: ClientShareWorkspaceErrorTag.UnknownWorkspace
    error: string
}
export interface ClientShareWorkspaceErrorWorkspaceInMaintenance {
    tag: ClientShareWorkspaceErrorTag.WorkspaceInMaintenance
    error: string
}
export type ClientShareWorkspaceError =
  | ClientShareWorkspaceErrorBadTimestamp
  | ClientShareWorkspaceErrorInternal
  | ClientShareWorkspaceErrorNotAllowed
  | ClientShareWorkspaceErrorOffline
  | ClientShareWorkspaceErrorOutsiderCannotBeManagerOrOwner
  | ClientShareWorkspaceErrorRevokedRecipient
  | ClientShareWorkspaceErrorShareToSelf
  | ClientShareWorkspaceErrorUnknownRecipient
  | ClientShareWorkspaceErrorUnknownRecipientOrWorkspace
  | ClientShareWorkspaceErrorUnknownWorkspace
  | ClientShareWorkspaceErrorWorkspaceInMaintenance

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
    NoAccess = 'ClientStartWorkspaceErrorNoAccess',
}

export interface ClientStartWorkspaceErrorInternal {
    tag: ClientStartWorkspaceErrorTag.Internal
    error: string
}
export interface ClientStartWorkspaceErrorNoAccess {
    tag: ClientStartWorkspaceErrorTag.NoAccess
    error: string
}
export type ClientStartWorkspaceError =
  | ClientStartWorkspaceErrorInternal
  | ClientStartWorkspaceErrorNoAccess

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

// DeleteInvitationError
export enum DeleteInvitationErrorTag {
    AlreadyDeleted = 'DeleteInvitationErrorAlreadyDeleted',
    Internal = 'DeleteInvitationErrorInternal',
    NotFound = 'DeleteInvitationErrorNotFound',
    Offline = 'DeleteInvitationErrorOffline',
}

export interface DeleteInvitationErrorAlreadyDeleted {
    tag: DeleteInvitationErrorTag.AlreadyDeleted
    error: string
}
export interface DeleteInvitationErrorInternal {
    tag: DeleteInvitationErrorTag.Internal
    error: string
}
export interface DeleteInvitationErrorNotFound {
    tag: DeleteInvitationErrorTag.NotFound
    error: string
}
export interface DeleteInvitationErrorOffline {
    tag: DeleteInvitationErrorTag.Offline
    error: string
}
export type DeleteInvitationError =
  | DeleteInvitationErrorAlreadyDeleted
  | DeleteInvitationErrorInternal
  | DeleteInvitationErrorNotFound
  | DeleteInvitationErrorOffline

// DeviceAccessStrategy
export enum DeviceAccessStrategyTag {
    Password = 'DeviceAccessStrategyPassword',
    Smartcard = 'DeviceAccessStrategySmartcard',
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
  | DeviceAccessStrategyPassword
  | DeviceAccessStrategySmartcard

// DeviceSaveStrategy
export enum DeviceSaveStrategyTag {
    Password = 'DeviceSaveStrategyPassword',
    Smartcard = 'DeviceSaveStrategySmartcard',
}

export interface DeviceSaveStrategyPassword {
    tag: DeviceSaveStrategyTag.Password
    password: Password
}
export interface DeviceSaveStrategySmartcard {
    tag: DeviceSaveStrategyTag.Smartcard
}
export type DeviceSaveStrategy =
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
    children: Array<EntryName>
}
export type EntryStat =
  | EntryStatFile
  | EntryStatFolder

// GreetInProgressError
export enum GreetInProgressErrorTag {
    ActiveUsersLimitReached = 'GreetInProgressErrorActiveUsersLimitReached',
    AlreadyUsed = 'GreetInProgressErrorAlreadyUsed',
    BadTimestamp = 'GreetInProgressErrorBadTimestamp',
    Cancelled = 'GreetInProgressErrorCancelled',
    CorruptedInviteUserData = 'GreetInProgressErrorCorruptedInviteUserData',
    DeviceAlreadyExists = 'GreetInProgressErrorDeviceAlreadyExists',
    Internal = 'GreetInProgressErrorInternal',
    NonceMismatch = 'GreetInProgressErrorNonceMismatch',
    NotFound = 'GreetInProgressErrorNotFound',
    Offline = 'GreetInProgressErrorOffline',
    PeerReset = 'GreetInProgressErrorPeerReset',
    UserAlreadyExists = 'GreetInProgressErrorUserAlreadyExists',
    UserCreateNotAllowed = 'GreetInProgressErrorUserCreateNotAllowed',
}

export interface GreetInProgressErrorActiveUsersLimitReached {
    tag: GreetInProgressErrorTag.ActiveUsersLimitReached
    error: string
}
export interface GreetInProgressErrorAlreadyUsed {
    tag: GreetInProgressErrorTag.AlreadyUsed
    error: string
}
export interface GreetInProgressErrorBadTimestamp {
    tag: GreetInProgressErrorTag.BadTimestamp
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
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
  | GreetInProgressErrorAlreadyUsed
  | GreetInProgressErrorBadTimestamp
  | GreetInProgressErrorCancelled
  | GreetInProgressErrorCorruptedInviteUserData
  | GreetInProgressErrorDeviceAlreadyExists
  | GreetInProgressErrorInternal
  | GreetInProgressErrorNonceMismatch
  | GreetInProgressErrorNotFound
  | GreetInProgressErrorOffline
  | GreetInProgressErrorPeerReset
  | GreetInProgressErrorUserAlreadyExists
  | GreetInProgressErrorUserCreateNotAllowed

// InviteListItem
export enum InviteListItemTag {
    Device = 'InviteListItemDevice',
    User = 'InviteListItemUser',
}

export interface InviteListItemDevice {
    tag: InviteListItemTag.Device
    addr: BackendInvitationAddr
    token: InvitationToken
    createdOn: DateTime
    status: InvitationStatus
}
export interface InviteListItemUser {
    tag: InviteListItemTag.User
    addr: BackendInvitationAddr
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

// NewDeviceInvitationError
export enum NewDeviceInvitationErrorTag {
    Internal = 'NewDeviceInvitationErrorInternal',
    Offline = 'NewDeviceInvitationErrorOffline',
    SendEmailToUserWithoutEmail = 'NewDeviceInvitationErrorSendEmailToUserWithoutEmail',
}

export interface NewDeviceInvitationErrorInternal {
    tag: NewDeviceInvitationErrorTag.Internal
    error: string
}
export interface NewDeviceInvitationErrorOffline {
    tag: NewDeviceInvitationErrorTag.Offline
    error: string
}
export interface NewDeviceInvitationErrorSendEmailToUserWithoutEmail {
    tag: NewDeviceInvitationErrorTag.SendEmailToUserWithoutEmail
    error: string
}
export type NewDeviceInvitationError =
  | NewDeviceInvitationErrorInternal
  | NewDeviceInvitationErrorOffline
  | NewDeviceInvitationErrorSendEmailToUserWithoutEmail

// NewUserInvitationError
export enum NewUserInvitationErrorTag {
    AlreadyMember = 'NewUserInvitationErrorAlreadyMember',
    Internal = 'NewUserInvitationErrorInternal',
    NotAllowed = 'NewUserInvitationErrorNotAllowed',
    Offline = 'NewUserInvitationErrorOffline',
}

export interface NewUserInvitationErrorAlreadyMember {
    tag: NewUserInvitationErrorTag.AlreadyMember
    error: string
}
export interface NewUserInvitationErrorInternal {
    tag: NewUserInvitationErrorTag.Internal
    error: string
}
export interface NewUserInvitationErrorNotAllowed {
    tag: NewUserInvitationErrorTag.NotAllowed
    error: string
}
export interface NewUserInvitationErrorOffline {
    tag: NewUserInvitationErrorTag.Offline
    error: string
}
export type NewUserInvitationError =
  | NewUserInvitationErrorAlreadyMember
  | NewUserInvitationErrorInternal
  | NewUserInvitationErrorNotAllowed
  | NewUserInvitationErrorOffline

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

// ParsedBackendAddr
export enum ParsedBackendAddrTag {
    InvitationDevice = 'ParsedBackendAddrInvitationDevice',
    InvitationUser = 'ParsedBackendAddrInvitationUser',
    Organization = 'ParsedBackendAddrOrganization',
    OrganizationBootstrap = 'ParsedBackendAddrOrganizationBootstrap',
    OrganizationFileLink = 'ParsedBackendAddrOrganizationFileLink',
    PkiEnrollment = 'ParsedBackendAddrPkiEnrollment',
    Server = 'ParsedBackendAddrServer',
}

export interface ParsedBackendAddrInvitationDevice {
    tag: ParsedBackendAddrTag.InvitationDevice
    hostname: string
    port: U32
    useSsl: boolean
    organizationId: OrganizationID
    token: InvitationToken
}
export interface ParsedBackendAddrInvitationUser {
    tag: ParsedBackendAddrTag.InvitationUser
    hostname: string
    port: U32
    useSsl: boolean
    organizationId: OrganizationID
    token: InvitationToken
}
export interface ParsedBackendAddrOrganization {
    tag: ParsedBackendAddrTag.Organization
    hostname: string
    port: U32
    useSsl: boolean
    organizationId: OrganizationID
}
export interface ParsedBackendAddrOrganizationBootstrap {
    tag: ParsedBackendAddrTag.OrganizationBootstrap
    hostname: string
    port: U32
    useSsl: boolean
    organizationId: OrganizationID
    token: string | null
}
export interface ParsedBackendAddrOrganizationFileLink {
    tag: ParsedBackendAddrTag.OrganizationFileLink
    hostname: string
    port: U32
    useSsl: boolean
    organizationId: OrganizationID
    workspaceId: VlobID
    encryptedPath: Uint8Array
    encryptedTimestamp: Uint8Array | null
}
export interface ParsedBackendAddrPkiEnrollment {
    tag: ParsedBackendAddrTag.PkiEnrollment
    hostname: string
    port: U32
    useSsl: boolean
    organizationId: OrganizationID
}
export interface ParsedBackendAddrServer {
    tag: ParsedBackendAddrTag.Server
    hostname: string
    port: U32
    useSsl: boolean
}
export type ParsedBackendAddr =
  | ParsedBackendAddrInvitationDevice
  | ParsedBackendAddrInvitationUser
  | ParsedBackendAddrOrganization
  | ParsedBackendAddrOrganizationBootstrap
  | ParsedBackendAddrOrganizationFileLink
  | ParsedBackendAddrPkiEnrollment
  | ParsedBackendAddrServer

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

// WorkspaceFsOperationError
export enum WorkspaceFsOperationErrorTag {
    BadTimestamp = 'WorkspaceFsOperationErrorBadTimestamp',
    CannotRenameRoot = 'WorkspaceFsOperationErrorCannotRenameRoot',
    EntryExists = 'WorkspaceFsOperationErrorEntryExists',
    EntryNotFound = 'WorkspaceFsOperationErrorEntryNotFound',
    FolderNotEmpty = 'WorkspaceFsOperationErrorFolderNotEmpty',
    Internal = 'WorkspaceFsOperationErrorInternal',
    InvalidCertificate = 'WorkspaceFsOperationErrorInvalidCertificate',
    InvalidManifest = 'WorkspaceFsOperationErrorInvalidManifest',
    IsAFolder = 'WorkspaceFsOperationErrorIsAFolder',
    NoRealmAccess = 'WorkspaceFsOperationErrorNoRealmAccess',
    NotAFolder = 'WorkspaceFsOperationErrorNotAFolder',
    Offline = 'WorkspaceFsOperationErrorOffline',
    ReadOnlyRealm = 'WorkspaceFsOperationErrorReadOnlyRealm',
}

export interface WorkspaceFsOperationErrorBadTimestamp {
    tag: WorkspaceFsOperationErrorTag.BadTimestamp
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export interface WorkspaceFsOperationErrorCannotRenameRoot {
    tag: WorkspaceFsOperationErrorTag.CannotRenameRoot
    error: string
}
export interface WorkspaceFsOperationErrorEntryExists {
    tag: WorkspaceFsOperationErrorTag.EntryExists
    error: string
}
export interface WorkspaceFsOperationErrorEntryNotFound {
    tag: WorkspaceFsOperationErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceFsOperationErrorFolderNotEmpty {
    tag: WorkspaceFsOperationErrorTag.FolderNotEmpty
    error: string
}
export interface WorkspaceFsOperationErrorInternal {
    tag: WorkspaceFsOperationErrorTag.Internal
    error: string
}
export interface WorkspaceFsOperationErrorInvalidCertificate {
    tag: WorkspaceFsOperationErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceFsOperationErrorInvalidManifest {
    tag: WorkspaceFsOperationErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceFsOperationErrorIsAFolder {
    tag: WorkspaceFsOperationErrorTag.IsAFolder
    error: string
}
export interface WorkspaceFsOperationErrorNoRealmAccess {
    tag: WorkspaceFsOperationErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceFsOperationErrorNotAFolder {
    tag: WorkspaceFsOperationErrorTag.NotAFolder
    error: string
}
export interface WorkspaceFsOperationErrorOffline {
    tag: WorkspaceFsOperationErrorTag.Offline
    error: string
}
export interface WorkspaceFsOperationErrorReadOnlyRealm {
    tag: WorkspaceFsOperationErrorTag.ReadOnlyRealm
    error: string
}
export type WorkspaceFsOperationError =
  | WorkspaceFsOperationErrorBadTimestamp
  | WorkspaceFsOperationErrorCannotRenameRoot
  | WorkspaceFsOperationErrorEntryExists
  | WorkspaceFsOperationErrorEntryNotFound
  | WorkspaceFsOperationErrorFolderNotEmpty
  | WorkspaceFsOperationErrorInternal
  | WorkspaceFsOperationErrorInvalidCertificate
  | WorkspaceFsOperationErrorInvalidManifest
  | WorkspaceFsOperationErrorIsAFolder
  | WorkspaceFsOperationErrorNoRealmAccess
  | WorkspaceFsOperationErrorNotAFolder
  | WorkspaceFsOperationErrorOffline
  | WorkspaceFsOperationErrorReadOnlyRealm

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
        bootstrap_organization_addr: BackendOrganizationBootstrapAddr,
        save_strategy: DeviceSaveStrategy,
        human_handle: HumanHandle,
        device_label: DeviceLabel,
        sequester_authority_verify_key: SequesterVerifyKeyDer | null
    ): Promise<Result<AvailableDevice, BootstrapOrganizationError>>
    buildBackendOrganizationBootstrapAddr(
        addr: BackendAddr,
        organization_id: OrganizationID
    ): Promise<BackendOrganizationBootstrapAddr>
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
        addr: BackendInvitationAddr
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
    clientCreateWorkspace(
        client: Handle,
        name: EntryName
    ): Promise<Result<VlobID, ClientCreateWorkspaceError>>
    clientDeleteInvitation(
        client: Handle,
        token: InvitationToken
    ): Promise<Result<null, DeleteInvitationError>>
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
    ): Promise<Result<NewInvitationInfo, NewDeviceInvitationError>>
    clientNewUserInvitation(
        client: Handle,
        claimer_email: string,
        send_email: boolean
    ): Promise<Result<NewInvitationInfo, NewUserInvitationError>>
    clientRenameWorkspace(
        client: Handle,
        realm_id: VlobID,
        new_name: EntryName
    ): Promise<Result<null, ClientRenameWorkspaceError>>
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
    listAvailableDevices(
        path: Path
    ): Promise<Array<AvailableDevice>>
    newCanceller(
    ): Promise<Handle>
    parseBackendAddr(
        url: string
    ): Promise<Result<ParsedBackendAddr, ParseBackendAddrError>>
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
    ): Promise<null>
    testGetTestbedBootstrapOrganizationAddr(
        discriminant_dir: Path
    ): Promise<BackendOrganizationBootstrapAddr | null>
    testGetTestbedOrganizationId(
        discriminant_dir: Path
    ): Promise<OrganizationID | null>
    testNewTestbed(
        template: string,
        test_server: BackendAddr | null
    ): Promise<Path>
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
    ): Promise<Result<VlobID, WorkspaceFsOperationError>>
    workspaceCreateFolder(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<VlobID, WorkspaceFsOperationError>>
    workspaceCreateFolderAll(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<VlobID, WorkspaceFsOperationError>>
    workspaceRemoveEntry(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<null, WorkspaceFsOperationError>>
    workspaceRemoveFile(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<null, WorkspaceFsOperationError>>
    workspaceRemoveFolder(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<null, WorkspaceFsOperationError>>
    workspaceRemoveFolderAll(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<null, WorkspaceFsOperationError>>
    workspaceRenameEntry(
        workspace: Handle,
        path: FsPath,
        new_name: EntryName,
        overwrite: boolean
    ): Promise<Result<null, WorkspaceFsOperationError>>
    workspaceStatEntry(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<EntryStat, WorkspaceFsOperationError>>
    workspaceStop(
        workspace: Handle
    ): Promise<Result<null, WorkspaceStopError>>
}
