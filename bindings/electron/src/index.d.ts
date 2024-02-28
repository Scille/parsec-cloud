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


export interface AvailableDevice {
    keyFilePath: string
    organizationId: string
    deviceId: string
    humanHandle: HumanHandle
    deviceLabel: string
    slug: string
    ty: DeviceFileType
}


export interface ClientConfig {
    configDir: string
    dataBaseDir: string
    mountpointBaseDir: string
    workspaceStorageCacheSize: WorkspaceStorageCacheSize
    withMonitors: boolean
}


export interface ClientInfo {
    organizationAddr: string
    organizationId: string
    deviceId: string
    userId: string
    deviceLabel: string
    humanHandle: HumanHandle
    currentProfile: UserProfile
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
    deviceLabel: string
    createdOn: number
    createdBy: string | null
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
    append: boolean
    truncate: boolean
    create: boolean
    createNew: boolean
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


export interface WorkspaceInfo {
    id: string
    name: string
    selfCurrentRole: RealmRole
    isStarted: boolean
    isBootstrapped: boolean
}


export interface WorkspaceUserAccessInfo {
    userId: string
    humanHandle: HumanHandle
    currentProfile: UserProfile
    currentRole: RealmRole
}


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
export interface ClaimInProgressErrorAlreadyUsed {
    tag: "AlreadyUsed"
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
  | ClaimInProgressErrorAlreadyUsed
  | ClaimInProgressErrorCancelled
  | ClaimInProgressErrorCorruptedConfirmation
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
export interface ClaimerRetrieveInfoErrorAlreadyUsed {
    tag: "AlreadyUsed"
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
export type ClaimerRetrieveInfoError =
  | ClaimerRetrieveInfoErrorAlreadyUsed
  | ClaimerRetrieveInfoErrorInternal
  | ClaimerRetrieveInfoErrorNotFound
  | ClaimerRetrieveInfoErrorOffline


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


// ClientChangeAuthentificationError
export interface ClientChangeAuthentificationErrorDecryptionFailed {
    tag: "DecryptionFailed"
    error: string
}
export interface ClientChangeAuthentificationErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientChangeAuthentificationErrorInvalidData {
    tag: "InvalidData"
    error: string
}
export interface ClientChangeAuthentificationErrorInvalidPath {
    tag: "InvalidPath"
    error: string
}
export type ClientChangeAuthentificationError =
  | ClientChangeAuthentificationErrorDecryptionFailed
  | ClientChangeAuthentificationErrorInternal
  | ClientChangeAuthentificationErrorInvalidData
  | ClientChangeAuthentificationErrorInvalidPath


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


// ClientEvent
export interface ClientEventPing {
    tag: "Ping"
    ping: string
}
export type ClientEvent =
  | ClientEventPing


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
  | DeviceAccessStrategyPassword
  | DeviceAccessStrategySmartcard


// DeviceSaveStrategy
export interface DeviceSaveStrategyPassword {
    tag: "Password"
    password: string
}
export interface DeviceSaveStrategySmartcard {
    tag: "Smartcard"
}
export type DeviceSaveStrategy =
  | DeviceSaveStrategyPassword
  | DeviceSaveStrategySmartcard


// EntryStat
export interface EntryStatFile {
    tag: "File"
    confinement_point: string | null
    id: string
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
    created: number
    updated: number
    base_version: number
    is_placeholder: boolean
    need_sync: boolean
    children: Array<[string, string]>
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
export interface InviteListItemDevice {
    tag: "Device"
    addr: string
    token: string
    created_on: number
    status: InvitationStatus
}
export interface InviteListItemUser {
    tag: "User"
    addr: string
    token: string
    created_on: number
    claimer_email: string
    status: InvitationStatus
}
export type InviteListItem =
  | InviteListItemDevice
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


// ParseBackendAddrError
export interface ParseBackendAddrErrorInvalidUrl {
    tag: "InvalidUrl"
    error: string
}
export type ParseBackendAddrError =
  | ParseBackendAddrErrorInvalidUrl


// ParsedParsecAddr
export interface ParsedParsecAddrInvitationDevice {
    tag: "InvitationDevice"
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
export interface ParsedParsecAddrOrganizationFileLink {
    tag: "OrganizationFileLink"
    hostname: string
    port: number
    use_ssl: boolean
    organization_id: string
    workspace_id: string
    encrypted_path: Uint8Array
    encrypted_timestamp: Uint8Array | null
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
export type ParsedParsecAddr =
  | ParsedParsecAddrInvitationDevice
  | ParsedParsecAddrInvitationUser
  | ParsedParsecAddrOrganization
  | ParsedParsecAddrOrganizationBootstrap
  | ParsedParsecAddrOrganizationFileLink
  | ParsedParsecAddrPkiEnrollment
  | ParsedParsecAddrServer


// UserOrDeviceClaimInitialInfo
export interface UserOrDeviceClaimInitialInfoDevice {
    tag: "Device"
    handle: number
    greeter_user_id: string
    greeter_human_handle: HumanHandle
}
export interface UserOrDeviceClaimInitialInfoUser {
    tag: "User"
    handle: number
    claimer_email: string
    greeter_user_id: string
    greeter_human_handle: HumanHandle
}
export type UserOrDeviceClaimInitialInfo =
  | UserOrDeviceClaimInitialInfoDevice
  | UserOrDeviceClaimInitialInfoUser


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
export interface WorkspaceCreateFileErrorParentIsFile {
    tag: "ParentIsFile"
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
  | WorkspaceCreateFileErrorParentIsFile
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
export interface WorkspaceCreateFolderErrorParentIsFile {
    tag: "ParentIsFile"
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
  | WorkspaceCreateFolderErrorParentIsFile
  | WorkspaceCreateFolderErrorParentNotFound
  | WorkspaceCreateFolderErrorReadOnlyRealm
  | WorkspaceCreateFolderErrorStopped


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


// WorkspaceRenameEntryError
export interface WorkspaceRenameEntryErrorCannotRenameRoot {
    tag: "CannotRenameRoot"
    error: string
}
export interface WorkspaceRenameEntryErrorDestinationExists {
    tag: "DestinationExists"
    error: string
}
export interface WorkspaceRenameEntryErrorEntryNotFound {
    tag: "EntryNotFound"
    error: string
}
export interface WorkspaceRenameEntryErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceRenameEntryErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceRenameEntryErrorInvalidKeysBundle {
    tag: "InvalidKeysBundle"
    error: string
}
export interface WorkspaceRenameEntryErrorInvalidManifest {
    tag: "InvalidManifest"
    error: string
}
export interface WorkspaceRenameEntryErrorNoRealmAccess {
    tag: "NoRealmAccess"
    error: string
}
export interface WorkspaceRenameEntryErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceRenameEntryErrorReadOnlyRealm {
    tag: "ReadOnlyRealm"
    error: string
}
export interface WorkspaceRenameEntryErrorStopped {
    tag: "Stopped"
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


export function bootstrapOrganization(
    config: ClientConfig,
    on_event_callback: (event: ClientEvent) => void,
    bootstrap_organization_addr: string,
    save_strategy: DeviceSaveStrategy,
    human_handle: HumanHandle,
    device_label: string,
    sequester_authority_verify_key: Uint8Array | null
): Promise<Result<AvailableDevice, BootstrapOrganizationError>>
export function buildBackendOrganizationBootstrapAddr(
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
    on_event_callback: (event: ClientEvent) => void,
    addr: string
): Promise<Result<UserOrDeviceClaimInitialInfo, ClaimerRetrieveInfoError>>
export function claimerUserFinalizeSaveLocalDevice(
    handle: number,
    save_strategy: DeviceSaveStrategy
): Promise<Result<AvailableDevice, ClaimInProgressError>>
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
export function clientCancelInvitation(
    client: number,
    token: string
): Promise<Result<null, ClientCancelInvitationError>>
export function clientChangeAuthentification(
    client_config: ClientConfig,
    current_auth: DeviceAccessStrategy,
    new_auth: DeviceSaveStrategy,
    with_testbed_template: boolean
): Promise<Result<null, ClientChangeAuthentificationError>>
export function clientCreateWorkspace(
    client: number,
    name: string
): Promise<Result<string, ClientCreateWorkspaceError>>
export function clientGetUserDevice(
    client: number,
    device: string
): Promise<Result<[UserInfo, DeviceInfo], ClientGetUserDeviceError>>
export function clientInfo(
    client: number
): Promise<Result<ClientInfo, ClientInfoError>>
export function clientListInvitations(
    client: number
): Promise<Result<Array<InviteListItem>, ListInvitationsError>>
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
export function clientShareWorkspace(
    client: number,
    realm_id: string,
    recipient: string,
    role: RealmRole | null
): Promise<Result<null, ClientShareWorkspaceError>>
export function clientStart(
    config: ClientConfig,
    on_event_callback: (event: ClientEvent) => void,
    access: DeviceAccessStrategy,
    with_testbed_template: boolean
): Promise<Result<number, ClientStartError>>
export function clientStartDeviceInvitationGreet(
    client: number,
    token: string
): Promise<Result<DeviceGreetInitialInfo, ClientStartInvitationGreetError>>
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
export function fdClose(
    workspace: number,
    fd: number
): Promise<Result<null, WorkspaceFdCloseError>>
export function fdFlush(
    workspace: number,
    fd: number
): Promise<Result<null, WorkspaceFdFlushError>>
export function fdRead(
    workspace: number,
    fd: number,
    offset: number,
    size: number
): Promise<Result<Uint8Array, WorkspaceFdReadError>>
export function fdResize(
    workspace: number,
    fd: number,
    length: number,
    truncate_only: boolean
): Promise<Result<null, WorkspaceFdResizeError>>
export function fdWrite(
    workspace: number,
    fd: number,
    offset: number,
    data: Uint8Array
): Promise<Result<number, WorkspaceFdWriteError>>
export function fdWriteWithConstrainedIo(
    workspace: number,
    fd: number,
    offset: number,
    data: Uint8Array
): Promise<Result<number, WorkspaceFdWriteError>>
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
export function greeterUserInProgress1DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<UserGreetInProgress2Info, GreetInProgressError>>
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
export function listAvailableDevices(
    path: string
): Promise<Array<AvailableDevice>>
export function newCanceller(
): Promise<number>
export function parseBackendAddr(
    url: string
): Promise<Result<ParsedParsecAddr, ParseBackendAddrError>>
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
): Promise<null>
export function testGetTestbedBootstrapOrganizationAddr(
    discriminant_dir: string
): Promise<string | null>
export function testGetTestbedOrganizationId(
    discriminant_dir: string
): Promise<string | null>
export function testNewTestbed(
    template: string,
    test_server: string | null
): Promise<string>
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
export function workspaceOpenFile(
    workspace: number,
    path: string,
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
export function workspaceRenameEntry(
    workspace: number,
    path: string,
    new_name: string,
    overwrite: boolean
): Promise<Result<null, WorkspaceRenameEntryError>>
export function workspaceStatEntry(
    workspace: number,
    path: string
): Promise<Result<EntryStat, WorkspaceStatEntryError>>
export function workspaceStop(
    workspace: number
): Promise<Result<null, WorkspaceStopError>>
