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


export interface AvailableDevice {
    keyFilePath: string
    organizationId: string
    deviceId: string
    humanHandle: HumanHandle | null
    deviceLabel: string | null
    slug: string
    ty: DeviceFileType
}


export interface ClientConfig {
    configDir: string
    dataBaseDir: string
    mountpointBaseDir: string
    workspaceStorageCacheSize: WorkspaceStorageCacheSize
}


export interface ClientInfo {
    organizationId: string
    deviceId: string
    userId: string
    deviceLabel: string | null
    humanHandle: HumanHandle | null
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
    requestedDeviceLabel: string | null
}


export interface DeviceGreetInitialInfo {
    handle: number
}


export interface DeviceInfo {
    id: string
    deviceLabel: string | null
    createdOn: number
    createdBy: string | null
}


export interface HumanHandle {
    email: string
    label: string
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
    requestedHumanHandle: HumanHandle | null
    requestedDeviceLabel: string | null
}


export interface UserGreetInitialInfo {
    handle: number
}


export interface UserInfo {
    id: string
    humanHandle: HumanHandle | null
    currentProfile: UserProfile
    createdOn: number
    createdBy: string | null
    revokedOn: number | null
    revokedBy: string | null
}


export interface WorkspaceInfo {
    id: string
    name: string
    selfRole: RealmRole
}


export interface WorkspaceUserAccessInfo {
    userId: string
    humanHandle: HumanHandle | null
    role: RealmRole
}


// BootstrapOrganizationError
export interface BootstrapOrganizationErrorAlreadyUsedToken {
    tag: "AlreadyUsedToken"
    error: string
}
export interface BootstrapOrganizationErrorBadTimestamp {
    tag: "BadTimestamp"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
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
export interface BootstrapOrganizationErrorSaveDeviceError {
    tag: "SaveDeviceError"
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


// ClientCreateWorkspaceError
export interface ClientCreateWorkspaceErrorInternal {
    tag: "Internal"
    error: string
}
export type ClientCreateWorkspaceError =
  | ClientCreateWorkspaceErrorInternal


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
export type ClientGetUserDeviceError =
  | ClientGetUserDeviceErrorInternal
  | ClientGetUserDeviceErrorNonExisting


// ClientInfoError
export interface ClientInfoErrorInternal {
    tag: "Internal"
    error: string
}
export type ClientInfoError =
  | ClientInfoErrorInternal


// ClientListUserDevicesError
export interface ClientListUserDevicesErrorInternal {
    tag: "Internal"
    error: string
}
export type ClientListUserDevicesError =
  | ClientListUserDevicesErrorInternal


// ClientListUsersError
export interface ClientListUsersErrorInternal {
    tag: "Internal"
    error: string
}
export type ClientListUsersError =
  | ClientListUsersErrorInternal


// ClientListWorkspaceUsersError
export interface ClientListWorkspaceUsersErrorInternal {
    tag: "Internal"
    error: string
}
export type ClientListWorkspaceUsersError =
  | ClientListWorkspaceUsersErrorInternal


// ClientListWorkspacesError
export interface ClientListWorkspacesErrorInternal {
    tag: "Internal"
    error: string
}
export type ClientListWorkspacesError =
  | ClientListWorkspacesErrorInternal


// ClientRenameWorkspaceError
export interface ClientRenameWorkspaceErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientRenameWorkspaceErrorUnknownWorkspace {
    tag: "UnknownWorkspace"
    error: string
}
export type ClientRenameWorkspaceError =
  | ClientRenameWorkspaceErrorInternal
  | ClientRenameWorkspaceErrorUnknownWorkspace


// ClientShareWorkspaceError
export interface ClientShareWorkspaceErrorBadTimestamp {
    tag: "BadTimestamp"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export interface ClientShareWorkspaceErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientShareWorkspaceErrorNotAllowed {
    tag: "NotAllowed"
    error: string
}
export interface ClientShareWorkspaceErrorOffline {
    tag: "Offline"
    error: string
}
export interface ClientShareWorkspaceErrorOutsiderCannotBeManagerOrOwner {
    tag: "OutsiderCannotBeManagerOrOwner"
    error: string
}
export interface ClientShareWorkspaceErrorRevokedRecipient {
    tag: "RevokedRecipient"
    error: string
}
export interface ClientShareWorkspaceErrorShareToSelf {
    tag: "ShareToSelf"
    error: string
}
export interface ClientShareWorkspaceErrorUnknownRecipient {
    tag: "UnknownRecipient"
    error: string
}
export interface ClientShareWorkspaceErrorUnknownRecipientOrWorkspace {
    tag: "UnknownRecipientOrWorkspace"
    error: string
}
export interface ClientShareWorkspaceErrorUnknownWorkspace {
    tag: "UnknownWorkspace"
    error: string
}
export interface ClientShareWorkspaceErrorWorkspaceInMaintenance {
    tag: "WorkspaceInMaintenance"
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
export interface ClientStartErrorDeviceAlreadyRunning {
    tag: "DeviceAlreadyRunning"
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
  | ClientStartErrorDeviceAlreadyRunning
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
export interface ClientStartWorkspaceErrorAlreadyStarted {
    tag: "AlreadyStarted"
    error: string
}
export interface ClientStartWorkspaceErrorInternal {
    tag: "Internal"
    error: string
}
export interface ClientStartWorkspaceErrorNoAccess {
    tag: "NoAccess"
    error: string
}
export type ClientStartWorkspaceError =
  | ClientStartWorkspaceErrorAlreadyStarted
  | ClientStartWorkspaceErrorInternal
  | ClientStartWorkspaceErrorNoAccess


// ClientStopError
export interface ClientStopErrorInternal {
    tag: "Internal"
    error: string
}
export type ClientStopError =
  | ClientStopErrorInternal


// DeleteInvitationError
export interface DeleteInvitationErrorAlreadyDeleted {
    tag: "AlreadyDeleted"
    error: string
}
export interface DeleteInvitationErrorInternal {
    tag: "Internal"
    error: string
}
export interface DeleteInvitationErrorNotFound {
    tag: "NotFound"
    error: string
}
export interface DeleteInvitationErrorOffline {
    tag: "Offline"
    error: string
}
export type DeleteInvitationError =
  | DeleteInvitationErrorAlreadyDeleted
  | DeleteInvitationErrorInternal
  | DeleteInvitationErrorNotFound
  | DeleteInvitationErrorOffline


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
    children: Array<string>
}
export type EntryStat =
  | EntryStatFile
  | EntryStatFolder


// GreetInProgressError
export interface GreetInProgressErrorActiveUsersLimitReached {
    tag: "ActiveUsersLimitReached"
    error: string
}
export interface GreetInProgressErrorAlreadyUsed {
    tag: "AlreadyUsed"
    error: string
}
export interface GreetInProgressErrorBadTimestamp {
    tag: "BadTimestamp"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
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
export interface InviteListItemDevice {
    tag: "Device"
    token: string
    created_on: number
    status: InvitationStatus
}
export interface InviteListItemUser {
    tag: "User"
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


// NewDeviceInvitationError
export interface NewDeviceInvitationErrorInternal {
    tag: "Internal"
    error: string
}
export interface NewDeviceInvitationErrorOffline {
    tag: "Offline"
    error: string
}
export interface NewDeviceInvitationErrorSendEmailToUserWithoutEmail {
    tag: "SendEmailToUserWithoutEmail"
    error: string
}
export type NewDeviceInvitationError =
  | NewDeviceInvitationErrorInternal
  | NewDeviceInvitationErrorOffline
  | NewDeviceInvitationErrorSendEmailToUserWithoutEmail


// NewUserInvitationError
export interface NewUserInvitationErrorAlreadyMember {
    tag: "AlreadyMember"
    error: string
}
export interface NewUserInvitationErrorInternal {
    tag: "Internal"
    error: string
}
export interface NewUserInvitationErrorNotAllowed {
    tag: "NotAllowed"
    error: string
}
export interface NewUserInvitationErrorOffline {
    tag: "Offline"
    error: string
}
export type NewUserInvitationError =
  | NewUserInvitationErrorAlreadyMember
  | NewUserInvitationErrorInternal
  | NewUserInvitationErrorNotAllowed
  | NewUserInvitationErrorOffline


// ParseBackendAddrError
export interface ParseBackendAddrErrorInvalidUrl {
    tag: "InvalidUrl"
    error: string
}
export type ParseBackendAddrError =
  | ParseBackendAddrErrorInvalidUrl


// ParsedBackendAddr
export interface ParsedBackendAddrBase {
    tag: "Base"
    hostname: string
    port: number
    use_ssl: boolean
}
export interface ParsedBackendAddrInvitationDevice {
    tag: "InvitationDevice"
    hostname: string
    port: number
    use_ssl: boolean
    organization_id: string
    token: string
}
export interface ParsedBackendAddrInvitationUser {
    tag: "InvitationUser"
    hostname: string
    port: number
    use_ssl: boolean
    organization_id: string
    token: string
}
export interface ParsedBackendAddrOrganizationBootstrap {
    tag: "OrganizationBootstrap"
    hostname: string
    port: number
    use_ssl: boolean
    organization_id: string
    token: string | null
}
export interface ParsedBackendAddrOrganizationFileLink {
    tag: "OrganizationFileLink"
    hostname: string
    port: number
    use_ssl: boolean
    organization_id: string
    workspace_id: string
    encrypted_path: Uint8Array
    encrypted_timestamp: Uint8Array | null
}
export interface ParsedBackendAddrPkiEnrollment {
    tag: "PkiEnrollment"
    hostname: string
    port: number
    use_ssl: boolean
    organization_id: string
}
export type ParsedBackendAddr =
  | ParsedBackendAddrBase
  | ParsedBackendAddrInvitationDevice
  | ParsedBackendAddrInvitationUser
  | ParsedBackendAddrOrganizationBootstrap
  | ParsedBackendAddrOrganizationFileLink
  | ParsedBackendAddrPkiEnrollment


// UserOrDeviceClaimInitialInfo
export interface UserOrDeviceClaimInitialInfoDevice {
    tag: "Device"
    handle: number
    greeter_user_id: string
    greeter_human_handle: HumanHandle | null
}
export interface UserOrDeviceClaimInitialInfoUser {
    tag: "User"
    handle: number
    claimer_email: string
    greeter_user_id: string
    greeter_human_handle: HumanHandle | null
}
export type UserOrDeviceClaimInitialInfo =
  | UserOrDeviceClaimInitialInfoDevice
  | UserOrDeviceClaimInitialInfoUser


// WorkspaceFsOperationError
export interface WorkspaceFsOperationErrorBadTimestamp {
    tag: "BadTimestamp"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export interface WorkspaceFsOperationErrorCannotRenameRoot {
    tag: "CannotRenameRoot"
    error: string
}
export interface WorkspaceFsOperationErrorEntryExists {
    tag: "EntryExists"
    error: string
}
export interface WorkspaceFsOperationErrorEntryNotFound {
    tag: "EntryNotFound"
    error: string
}
export interface WorkspaceFsOperationErrorFolderNotEmpty {
    tag: "FolderNotEmpty"
    error: string
}
export interface WorkspaceFsOperationErrorInternal {
    tag: "Internal"
    error: string
}
export interface WorkspaceFsOperationErrorInvalidCertificate {
    tag: "InvalidCertificate"
    error: string
}
export interface WorkspaceFsOperationErrorInvalidManifest {
    tag: "InvalidManifest"
    error: string
}
export interface WorkspaceFsOperationErrorIsAFolder {
    tag: "IsAFolder"
    error: string
}
export interface WorkspaceFsOperationErrorNoRealmAccess {
    tag: "NoRealmAccess"
    error: string
}
export interface WorkspaceFsOperationErrorNotAFolder {
    tag: "NotAFolder"
    error: string
}
export interface WorkspaceFsOperationErrorOffline {
    tag: "Offline"
    error: string
}
export interface WorkspaceFsOperationErrorReadOnlyRealm {
    tag: "ReadOnlyRealm"
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
    human_handle: HumanHandle | null,
    device_label: string | null,
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
    requested_device_label: string | null
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
    requested_device_label: string | null,
    requested_human_handle: HumanHandle | null
): Promise<Result<UserClaimFinalizeInfo, ClaimInProgressError>>
export function claimerUserInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<UserClaimInProgress1Info, ClaimInProgressError>>
export function clientCreateWorkspace(
    client: number,
    name: string
): Promise<Result<string, ClientCreateWorkspaceError>>
export function clientDeleteInvitation(
    client: number,
    token: string
): Promise<Result<null, DeleteInvitationError>>
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
): Promise<Result<[string, InvitationEmailSentStatus], NewDeviceInvitationError>>
export function clientNewUserInvitation(
    client: number,
    claimer_email: string,
    send_email: boolean
): Promise<Result<[string, InvitationEmailSentStatus], NewUserInvitationError>>
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
    access: DeviceAccessStrategy
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
    device_label: string | null
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
    human_handle: HumanHandle | null,
    device_label: string | null,
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
): Promise<Result<ParsedBackendAddr, ParseBackendAddrError>>
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
export function validatePath(
    raw: string
): Promise<boolean>
export function workspaceCreateFile(
    workspace: number,
    path: string
): Promise<Result<string, WorkspaceFsOperationError>>
export function workspaceCreateFolder(
    workspace: number,
    path: string
): Promise<Result<string, WorkspaceFsOperationError>>
export function workspaceCreateFolderAll(
    workspace: number,
    path: string
): Promise<Result<string, WorkspaceFsOperationError>>
export function workspaceRemoveEntry(
    workspace: number,
    path: string
): Promise<Result<null, WorkspaceFsOperationError>>
export function workspaceRemoveFile(
    workspace: number,
    path: string
): Promise<Result<null, WorkspaceFsOperationError>>
export function workspaceRemoveFolder(
    workspace: number,
    path: string
): Promise<Result<null, WorkspaceFsOperationError>>
export function workspaceRemoveFolderAll(
    workspace: number,
    path: string
): Promise<Result<null, WorkspaceFsOperationError>>
export function workspaceRenameEntry(
    workspace: number,
    path: string,
    new_name: string,
    overwrite: boolean
): Promise<Result<null, WorkspaceFsOperationError>>
export function workspaceStatEntry(
    workspace: number,
    path: string
): Promise<Result<EntryStat, WorkspaceFsOperationError>>
export function workspaceStop(
    workspace: number
): Promise<Result<null, WorkspaceStopError>>
