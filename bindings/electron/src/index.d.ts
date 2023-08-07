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


export interface UserClaimInProgress1CtxHandle {
    handle: number
    greeterSas: string
    greeterSasChoices: Array<string>
}


export interface DeviceClaimInProgress1CtxHandle {
    handle: number
    greeterSas: string
    greeterSasChoices: Array<string>
}


export interface UserClaimInProgress2CtxHandle {
    handle: number
    claimerSas: string
}


export interface DeviceClaimInProgress2CtxHandle {
    handle: number
    claimerSas: string
}


export interface UserClaimInProgress3CtxHandle {
    handle: number
}


export interface DeviceClaimInProgress3CtxHandle {
    handle: number
}


export interface UserClaimFinalizeCtxHandle {
    handle: number
}


export interface DeviceClaimFinalizeCtxHandle {
    handle: number
}


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
  | ClaimInProgressErrorCorruptedConfirmation
  | ClaimInProgressErrorInternal
  | ClaimInProgressErrorNotFound
  | ClaimInProgressErrorOffline
  | ClaimInProgressErrorPeerReset


// UserOrDeviceClaimInitialCtxHandle
export interface UserOrDeviceClaimInitialCtxHandleDevice {
    tag: "Device"
    handle: number
    greeter_user_id: string
    greeter_human_handle: string | null
}
export interface UserOrDeviceClaimInitialCtxHandleUser {
    tag: "User"
    handle: number
    claimer_email: string
    greeter_user_id: string
    greeter_human_handle: string | null
}
export type UserOrDeviceClaimInitialCtxHandle =
  | UserOrDeviceClaimInitialCtxHandleDevice
  | UserOrDeviceClaimInitialCtxHandleUser


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
): Promise<Result<UserOrDeviceClaimInitialCtxHandle, ClaimerRetrieveInfoError>>
export function claimerUserInitialCtxDoWaitPeer(
    handle: number
): Promise<Result<UserClaimInProgress1CtxHandle, ClaimInProgressError>>
export function claimerDeviceInitialCtxDoWaitPeer(
    handle: number
): Promise<Result<DeviceClaimInProgress1CtxHandle, ClaimInProgressError>>
export function claimerUserInProgress2DoSignifyTrust(
    handle: number
): Promise<Result<UserClaimInProgress2CtxHandle, ClaimInProgressError>>
export function claimerDeviceInProgress2DoSignifyTrust(
    handle: number
): Promise<Result<DeviceClaimInProgress2CtxHandle, ClaimInProgressError>>
export function claimerUserInProgress2DoWaitPeerTrust(
    handle: number
): Promise<Result<UserClaimInProgress3CtxHandle, ClaimInProgressError>>
export function claimerDeviceInProgress2DoWaitPeerTrust(
    handle: number
): Promise<Result<DeviceClaimInProgress3CtxHandle, ClaimInProgressError>>
export function claimerUserInProgress3DoClaim(
    handle: number,
    requested_device_label: string | null,
    requested_human_handle: string | null
): Promise<Result<UserClaimFinalizeCtxHandle, ClaimInProgressError>>
export function claimerDeviceInProgress3DoClaim(
    handle: number,
    requested_device_label: string | null
): Promise<Result<DeviceClaimFinalizeCtxHandle, ClaimInProgressError>>
export function claimerUserFinalizeSaveLocalDevice(
    handle: number,
    save_strategy: DeviceSaveStrategy
): Promise<Result<AvailableDevice, ClaimInProgressError>>
export function claimerDeviceFinalizeSaveLocalDevice(
    handle: number,
    save_strategy: DeviceSaveStrategy
): Promise<Result<AvailableDevice, ClaimInProgressError>>
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
