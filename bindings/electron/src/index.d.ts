// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */


export type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E }


export interface ClientConfig {
    configDir: string
    dataBaseDir: string
    mountpointBaseDir: string
    workspaceStorageCacheSize: WorkspaceStorageCacheSize
}


export interface AvailableDevice {
    keyFilePath: string
    organizationId: string
    deviceId: string
    humanHandle: string | null
    deviceLabel: string | null
    slug: string
    ty: DeviceFileType
}


export interface UserClaimInProgress1Info {
    handle: number
    greeterSas: string
    greeterSasChoices: Array<string>
}


export interface DeviceClaimInProgress1Info {
    handle: number
    greeterSas: string
    greeterSasChoices: Array<string>
}


export interface UserClaimInProgress2Info {
    handle: number
    claimerSas: string
}


export interface DeviceClaimInProgress2Info {
    handle: number
    claimerSas: string
}


export interface UserClaimInProgress3Info {
    handle: number
}


export interface DeviceClaimInProgress3Info {
    handle: number
}


export interface UserClaimFinalizeInfo {
    handle: number
}


export interface DeviceClaimFinalizeInfo {
    handle: number
}


export interface UserGreetInitialInfo {
    handle: number
}


export interface DeviceGreetInitialInfo {
    handle: number
}


export interface UserGreetInProgress1Info {
    handle: number
    greeterSas: string
}


export interface DeviceGreetInProgress1Info {
    handle: number
    greeterSas: string
}


export interface UserGreetInProgress2Info {
    handle: number
    claimerSas: string
    claimerSasChoices: Array<string>
}


export interface DeviceGreetInProgress2Info {
    handle: number
    claimerSas: string
    claimerSasChoices: Array<string>
}


export interface UserGreetInProgress3Info {
    handle: number
}


export interface DeviceGreetInProgress3Info {
    handle: number
}


export interface UserGreetInProgress4Info {
    handle: number
    requestedHumanHandle: string | null
    requestedDeviceLabel: string | null
}


export interface DeviceGreetInProgress4Info {
    handle: number
    requestedDeviceLabel: string | null
}


// CancelError
export interface CancelErrorInternal {
    tag: "Internal"
    error: string
}
export interface CancelErrorNotBinded {
    tag: "NotBinded"
    error: string
}
export type CancelError =
  | CancelErrorInternal
  | CancelErrorNotBinded


// RealmRole
export interface RealmRoleContributor {
    tag: "Contributor"
}
export interface RealmRoleManager {
    tag: "Manager"
}
export interface RealmRoleOwner {
    tag: "Owner"
}
export interface RealmRoleReader {
    tag: "Reader"
}
export type RealmRole =
  | RealmRoleContributor
  | RealmRoleManager
  | RealmRoleOwner
  | RealmRoleReader


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


// ClientStopError
export interface ClientStopErrorInternal {
    tag: "Internal"
    error: string
}
export type ClientStopError =
  | ClientStopErrorInternal


// ClientListWorkspacesError
export interface ClientListWorkspacesErrorInternal {
    tag: "Internal"
    error: string
}
export type ClientListWorkspacesError =
  | ClientListWorkspacesErrorInternal


// ClientWorkspaceCreateError
export interface ClientWorkspaceCreateErrorInternal {
    tag: "Internal"
    error: string
}
export type ClientWorkspaceCreateError =
  | ClientWorkspaceCreateErrorInternal


// UserOpsError
export interface UserOpsErrorInternal {
    tag: "Internal"
    error: string
}
export interface UserOpsErrorUnknownWorkspace {
    tag: "UnknownWorkspace"
    error: string
}
export type UserOpsError =
  | UserOpsErrorInternal
  | UserOpsErrorUnknownWorkspace


// UserOpsWorkspaceShareError
export interface UserOpsWorkspaceShareErrorBadTimestamp {
    tag: "BadTimestamp"
    error: string
    server_timestamp: string
    client_timestamp: string
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export interface UserOpsWorkspaceShareErrorInternal {
    tag: "Internal"
    error: string
}
export interface UserOpsWorkspaceShareErrorNotAllowed {
    tag: "NotAllowed"
    error: string
}
export interface UserOpsWorkspaceShareErrorOffline {
    tag: "Offline"
    error: string
}
export interface UserOpsWorkspaceShareErrorOutsiderCannotBeManagerOrOwner {
    tag: "OutsiderCannotBeManagerOrOwner"
    error: string
}
export interface UserOpsWorkspaceShareErrorRevokedRecipient {
    tag: "RevokedRecipient"
    error: string
}
export interface UserOpsWorkspaceShareErrorShareToSelf {
    tag: "ShareToSelf"
    error: string
}
export interface UserOpsWorkspaceShareErrorUnknownRecipient {
    tag: "UnknownRecipient"
    error: string
}
export interface UserOpsWorkspaceShareErrorUnknownRecipientOrWorkspace {
    tag: "UnknownRecipientOrWorkspace"
    error: string
}
export interface UserOpsWorkspaceShareErrorUnknownWorkspace {
    tag: "UnknownWorkspace"
    error: string
}
export interface UserOpsWorkspaceShareErrorWorkspaceInMaintenance {
    tag: "WorkspaceInMaintenance"
    error: string
}
export type UserOpsWorkspaceShareError =
  | UserOpsWorkspaceShareErrorBadTimestamp
  | UserOpsWorkspaceShareErrorInternal
  | UserOpsWorkspaceShareErrorNotAllowed
  | UserOpsWorkspaceShareErrorOffline
  | UserOpsWorkspaceShareErrorOutsiderCannotBeManagerOrOwner
  | UserOpsWorkspaceShareErrorRevokedRecipient
  | UserOpsWorkspaceShareErrorShareToSelf
  | UserOpsWorkspaceShareErrorUnknownRecipient
  | UserOpsWorkspaceShareErrorUnknownRecipientOrWorkspace
  | UserOpsWorkspaceShareErrorUnknownWorkspace
  | UserOpsWorkspaceShareErrorWorkspaceInMaintenance


// UserProfile
export interface UserProfileAdmin {
    tag: "Admin"
}
export interface UserProfileOutsider {
    tag: "Outsider"
}
export interface UserProfileStandard {
    tag: "Standard"
}
export type UserProfile =
  | UserProfileAdmin
  | UserProfileOutsider
  | UserProfileStandard


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


// ClientEvent
export interface ClientEventPing {
    tag: "Ping"
    ping: string
}
export type ClientEvent =
  | ClientEventPing


// ClaimerGreeterAbortOperationError
export interface ClaimerGreeterAbortOperationErrorInternal {
    tag: "Internal"
    error: string
}
export type ClaimerGreeterAbortOperationError =
  | ClaimerGreeterAbortOperationErrorInternal


// DeviceFileType
export interface DeviceFileTypePassword {
    tag: "Password"
}
export interface DeviceFileTypeRecovery {
    tag: "Recovery"
}
export interface DeviceFileTypeSmartcard {
    tag: "Smartcard"
}
export type DeviceFileType =
  | DeviceFileTypePassword
  | DeviceFileTypeRecovery
  | DeviceFileTypeSmartcard


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


// BootstrapOrganizationError
export interface BootstrapOrganizationErrorAlreadyUsedToken {
    tag: "AlreadyUsedToken"
    error: string
}
export interface BootstrapOrganizationErrorBadTimestamp {
    tag: "BadTimestamp"
    error: string
    server_timestamp: string
    client_timestamp: string
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


// UserOrDeviceClaimInitialInfo
export interface UserOrDeviceClaimInitialInfoDevice {
    tag: "Device"
    handle: number
    greeter_user_id: string
    greeter_human_handle: string | null
}
export interface UserOrDeviceClaimInitialInfoUser {
    tag: "User"
    handle: number
    claimer_email: string
    greeter_user_id: string
    greeter_human_handle: string | null
}
export type UserOrDeviceClaimInitialInfo =
  | UserOrDeviceClaimInitialInfoDevice
  | UserOrDeviceClaimInitialInfoUser


// InvitationStatus
export interface InvitationStatusDeleted {
    tag: "Deleted"
}
export interface InvitationStatusIdle {
    tag: "Idle"
}
export interface InvitationStatusReady {
    tag: "Ready"
}
export type InvitationStatus =
  | InvitationStatusDeleted
  | InvitationStatusIdle
  | InvitationStatusReady


// InvitationEmailSentStatus
export interface InvitationEmailSentStatusBadRecipient {
    tag: "BadRecipient"
}
export interface InvitationEmailSentStatusNotAvailable {
    tag: "NotAvailable"
}
export interface InvitationEmailSentStatusSuccess {
    tag: "Success"
}
export type InvitationEmailSentStatus =
  | InvitationEmailSentStatusBadRecipient
  | InvitationEmailSentStatusNotAvailable
  | InvitationEmailSentStatusSuccess


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


// InviteListItem
export interface InviteListItemDevice {
    tag: "Device"
    token: Uint8Array
    created_on: string
    status: InvitationStatus
}
export interface InviteListItemUser {
    tag: "User"
    token: Uint8Array
    created_on: string
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


// ClientStartInvitationGreetError
export interface ClientStartInvitationGreetErrorInternal {
    tag: "Internal"
    error: string
}
export type ClientStartInvitationGreetError =
  | ClientStartInvitationGreetErrorInternal


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
    server_timestamp: string
    client_timestamp: string
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


export function cancel(
    canceller: number
): Promise<Result<null, CancelError>>
export function newCanceller(
): Promise<number>
export function clientStart(
    config: ClientConfig,
    on_event_callback: (event: ClientEvent) => void,
    access: DeviceAccessStrategy
): Promise<Result<number, ClientStartError>>
export function clientStop(
    client: number
): Promise<Result<null, ClientStopError>>
export function clientListWorkspaces(
    client: number
): Promise<Result<Array<[Uint8Array, string]>, ClientListWorkspacesError>>
export function clientWorkspaceCreate(
    client: number,
    name: string
): Promise<Result<Uint8Array, ClientWorkspaceCreateError>>
export function clientWorkspaceRename(
    client: number,
    workspace_id: Uint8Array,
    new_name: string
): Promise<Result<null, UserOpsError>>
export function clientWorkspaceShare(
    client: number,
    workspace_id: Uint8Array,
    recipient: string,
    role: RealmRole | null
): Promise<Result<null, UserOpsWorkspaceShareError>>
export function claimerGreeterAbortOperation(
    handle: number
): Promise<Result<null, ClaimerGreeterAbortOperationError>>
export function listAvailableDevices(
    path: string
): Promise<Array<AvailableDevice>>
export function bootstrapOrganization(
    config: ClientConfig,
    on_event_callback: (event: ClientEvent) => void,
    bootstrap_organization_addr: string,
    save_strategy: DeviceSaveStrategy,
    human_handle: string | null,
    device_label: string | null,
    sequester_authority_verify_key: Uint8Array | null
): Promise<Result<AvailableDevice, BootstrapOrganizationError>>
export function claimerRetrieveInfo(
    config: ClientConfig,
    on_event_callback: (event: ClientEvent) => void,
    addr: string
): Promise<Result<UserOrDeviceClaimInitialInfo, ClaimerRetrieveInfoError>>
export function claimerUserInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<UserClaimInProgress1Info, ClaimInProgressError>>
export function claimerDeviceInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<DeviceClaimInProgress1Info, ClaimInProgressError>>
export function claimerUserInProgress1DoSignifyTrust(
    canceller: number,
    handle: number
): Promise<Result<UserClaimInProgress2Info, ClaimInProgressError>>
export function claimerDeviceInProgress1DoSignifyTrust(
    canceller: number,
    handle: number
): Promise<Result<DeviceClaimInProgress2Info, ClaimInProgressError>>
export function claimerUserInProgress2DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<UserClaimInProgress3Info, ClaimInProgressError>>
export function claimerDeviceInProgress2DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<DeviceClaimInProgress3Info, ClaimInProgressError>>
export function claimerUserInProgress3DoClaim(
    canceller: number,
    handle: number,
    requested_device_label: string | null,
    requested_human_handle: string | null
): Promise<Result<UserClaimFinalizeInfo, ClaimInProgressError>>
export function claimerDeviceInProgress3DoClaim(
    canceller: number,
    handle: number,
    requested_device_label: string | null
): Promise<Result<DeviceClaimFinalizeInfo, ClaimInProgressError>>
export function claimerUserFinalizeSaveLocalDevice(
    handle: number,
    save_strategy: DeviceSaveStrategy
): Promise<Result<AvailableDevice, ClaimInProgressError>>
export function claimerDeviceFinalizeSaveLocalDevice(
    handle: number,
    save_strategy: DeviceSaveStrategy
): Promise<Result<AvailableDevice, ClaimInProgressError>>
export function clientNewUserInvitation(
    client: number,
    claimer_email: string,
    send_email: boolean
): Promise<Result<[Uint8Array, InvitationEmailSentStatus], NewUserInvitationError>>
export function clientNewDeviceInvitation(
    client: number,
    send_email: boolean
): Promise<Result<[Uint8Array, InvitationEmailSentStatus], NewDeviceInvitationError>>
export function clientDeleteInvitation(
    client: number,
    token: Uint8Array
): Promise<Result<null, DeleteInvitationError>>
export function clientListInvitations(
    client: number
): Promise<Result<Array<InviteListItem>, ListInvitationsError>>
export function clientStartUserInvitationGreet(
    client: number,
    token: Uint8Array
): Promise<Result<UserGreetInitialInfo, ClientStartInvitationGreetError>>
export function clientStartDeviceInvitationGreet(
    client: number,
    token: Uint8Array
): Promise<Result<DeviceGreetInitialInfo, ClientStartInvitationGreetError>>
export function greeterUserInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<UserGreetInProgress1Info, GreetInProgressError>>
export function greeterDeviceInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<DeviceGreetInProgress1Info, GreetInProgressError>>
export function greeterUserInProgress1DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<UserGreetInProgress2Info, GreetInProgressError>>
export function greeterDeviceInProgress1DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<DeviceGreetInProgress2Info, GreetInProgressError>>
export function greeterUserInProgress2DoSignifyTrust(
    canceller: number,
    handle: number
): Promise<Result<UserGreetInProgress3Info, GreetInProgressError>>
export function greeterDeviceInProgress2DoSignifyTrust(
    canceller: number,
    handle: number
): Promise<Result<DeviceGreetInProgress3Info, GreetInProgressError>>
export function greeterUserInProgress3DoGetClaimRequests(
    canceller: number,
    handle: number
): Promise<Result<UserGreetInProgress4Info, GreetInProgressError>>
export function greeterDeviceInProgress3DoGetClaimRequests(
    canceller: number,
    handle: number
): Promise<Result<DeviceGreetInProgress4Info, GreetInProgressError>>
export function greeterUserInProgress4DoCreate(
    canceller: number,
    handle: number,
    human_handle: string | null,
    device_label: string | null,
    profile: UserProfile
): Promise<Result<null, GreetInProgressError>>
export function greeterDeviceInProgress4DoCreate(
    canceller: number,
    handle: number,
    device_label: string | null
): Promise<Result<null, GreetInProgressError>>
export function testNewTestbed(
    template: string,
    test_server: string | null
): Promise<string>
export function testGetTestbedOrganizationId(
    discriminant_dir: string
): Promise<string | null>
export function testGetTestbedBootstrapOrganizationAddr(
    discriminant_dir: string
): Promise<string | null>
export function testDropTestbed(
    path: string
): Promise<null>
