// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */

export type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E }

type OrganizationID = string
type UserID = string
type DeviceID = string
type DeviceLabel = string
type HumanHandle = string
type DateTime = string
type Password = string
type BackendAddr = string
type BackendOrganizationAddr = string
type BackendOrganizationBootstrapAddr = string
type BackendInvitationAddr = string
type Path = string
type SASCode = string
type SequesterVerifyKeyDer = Uint8Array
type Handle = number
type CacheSize = number

export interface ClientConfig {
    configDir: Path
    dataBaseDir: Path
    mountpointBaseDir: Path
    workspaceStorageCacheSize: WorkspaceStorageCacheSize
}

export interface AvailableDevice {
    keyFilePath: Path
    organizationId: OrganizationID
    deviceId: DeviceID
    humanHandle: HumanHandle | null
    deviceLabel: DeviceLabel | null
    slug: string
    ty: DeviceFileType
}

export interface UserClaimInProgress1CtxHandle {
    handle: number
    greeterSas: SASCode
    greeterSasChoices: Array<SASCode>
}

export interface DeviceClaimInProgress1CtxHandle {
    handle: number
    greeterSas: SASCode
    greeterSasChoices: Array<SASCode>
}

export interface UserClaimInProgress2CtxHandle {
    handle: number
    claimerSas: SASCode
}

export interface DeviceClaimInProgress2CtxHandle {
    handle: number
    claimerSas: SASCode
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
    tag: 'Custom'
    size: number
}
export interface WorkspaceStorageCacheSizeDefault {
    tag: 'Default'
}
export type WorkspaceStorageCacheSize =
  | WorkspaceStorageCacheSizeCustom
  | WorkspaceStorageCacheSizeDefault

// ClientEvent
export interface ClientEventPing {
    tag: 'Ping'
    ping: string
}
export type ClientEvent =
  | ClientEventPing

// DeviceFileType
export interface DeviceFileTypePassword {
    tag: 'Password'
}
export interface DeviceFileTypeRecovery {
    tag: 'Recovery'
}
export interface DeviceFileTypeSmartcard {
    tag: 'Smartcard'
}
export type DeviceFileType =
  | DeviceFileTypePassword
  | DeviceFileTypeRecovery
  | DeviceFileTypeSmartcard

// DeviceSaveStrategy
export interface DeviceSaveStrategyPassword {
    tag: 'Password'
    password: Password
}
export interface DeviceSaveStrategySmartcard {
    tag: 'Smartcard'
}
export type DeviceSaveStrategy =
  | DeviceSaveStrategyPassword
  | DeviceSaveStrategySmartcard

// BootstrapOrganizationError
export interface BootstrapOrganizationErrorAlreadyUsedToken {
    tag: 'AlreadyUsedToken'
    error: string
}
export interface BootstrapOrganizationErrorBadTimestamp {
    tag: 'BadTimestamp'
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export interface BootstrapOrganizationErrorInternal {
    tag: 'Internal'
    error: string
}
export interface BootstrapOrganizationErrorInvalidToken {
    tag: 'InvalidToken'
    error: string
}
export interface BootstrapOrganizationErrorOffline {
    tag: 'Offline'
    error: string
}
export interface BootstrapOrganizationErrorSaveDeviceError {
    tag: 'SaveDeviceError'
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
    tag: 'AlreadyUsed'
    error: string
}
export interface ClaimerRetrieveInfoErrorInternal {
    tag: 'Internal'
    error: string
}
export interface ClaimerRetrieveInfoErrorNotFound {
    tag: 'NotFound'
    error: string
}
export interface ClaimerRetrieveInfoErrorOffline {
    tag: 'Offline'
    error: string
}
export type ClaimerRetrieveInfoError =
  | ClaimerRetrieveInfoErrorAlreadyUsed
  | ClaimerRetrieveInfoErrorInternal
  | ClaimerRetrieveInfoErrorNotFound
  | ClaimerRetrieveInfoErrorOffline

// ClaimInProgressError
export interface ClaimInProgressErrorActiveUsersLimitReached {
    tag: 'ActiveUsersLimitReached'
    error: string
}
export interface ClaimInProgressErrorAlreadyUsed {
    tag: 'AlreadyUsed'
    error: string
}
export interface ClaimInProgressErrorCorruptedConfirmation {
    tag: 'CorruptedConfirmation'
    error: string
}
export interface ClaimInProgressErrorInternal {
    tag: 'Internal'
    error: string
}
export interface ClaimInProgressErrorNotFound {
    tag: 'NotFound'
    error: string
}
export interface ClaimInProgressErrorOffline {
    tag: 'Offline'
    error: string
}
export interface ClaimInProgressErrorPeerReset {
    tag: 'PeerReset'
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
    tag: 'Device'
    handle: number
    greeterUserId: UserID
    greeterHumanHandle: HumanHandle | null
}
export interface UserOrDeviceClaimInitialCtxHandleUser {
    tag: 'User'
    handle: number
    claimerEmail: string
    greeterUserId: UserID
    greeterHumanHandle: HumanHandle | null
}
export type UserOrDeviceClaimInitialCtxHandle =
  | UserOrDeviceClaimInitialCtxHandleDevice
  | UserOrDeviceClaimInitialCtxHandleUser

export interface LibParsecPlugin {
    listAvailableDevices(
        path: Path
    ): Promise<Array<AvailableDevice>>
    bootstrapOrganization(
        config: ClientConfig,
        on_event_callback: (event: ClientEvent) => void,
        bootstrap_organization_addr: BackendOrganizationBootstrapAddr,
        save_strategy: DeviceSaveStrategy,
        human_handle: HumanHandle | null,
        device_label: DeviceLabel | null,
        sequester_authority_verify_key: SequesterVerifyKeyDer | null
    ): Promise<Result<AvailableDevice, BootstrapOrganizationError>>
    claimerRetrieveInfo(
        config: ClientConfig,
        on_event_callback: (event: ClientEvent) => void,
        addr: BackendInvitationAddr
    ): Promise<Result<UserOrDeviceClaimInitialCtxHandle, ClaimerRetrieveInfoError>>
    claimerUserInitialCtxDoWaitPeer(
        handle: number
    ): Promise<Result<UserClaimInProgress1CtxHandle, ClaimInProgressError>>
    claimerDeviceInitialCtxDoWaitPeer(
        handle: number
    ): Promise<Result<DeviceClaimInProgress1CtxHandle, ClaimInProgressError>>
    claimerUserInProgress2DoSignifyTrust(
        handle: number
    ): Promise<Result<UserClaimInProgress2CtxHandle, ClaimInProgressError>>
    claimerDeviceInProgress2DoSignifyTrust(
        handle: number
    ): Promise<Result<DeviceClaimInProgress2CtxHandle, ClaimInProgressError>>
    claimerUserInProgress2DoWaitPeerTrust(
        handle: number
    ): Promise<Result<UserClaimInProgress3CtxHandle, ClaimInProgressError>>
    claimerDeviceInProgress2DoWaitPeerTrust(
        handle: number
    ): Promise<Result<DeviceClaimInProgress3CtxHandle, ClaimInProgressError>>
    claimerUserInProgress3DoClaim(
        handle: number,
        requested_device_label: DeviceLabel | null,
        requested_human_handle: HumanHandle | null
    ): Promise<Result<UserClaimFinalizeCtxHandle, ClaimInProgressError>>
    claimerDeviceInProgress3DoClaim(
        handle: number,
        requested_device_label: DeviceLabel | null
    ): Promise<Result<DeviceClaimFinalizeCtxHandle, ClaimInProgressError>>
    claimerUserFinalizeSaveLocalDevice(
        handle: number,
        save_strategy: DeviceSaveStrategy
    ): Promise<Result<AvailableDevice, ClaimInProgressError>>
    claimerDeviceFinalizeSaveLocalDevice(
        handle: number,
        save_strategy: DeviceSaveStrategy
    ): Promise<Result<AvailableDevice, ClaimInProgressError>>
    testNewTestbed(
        template: string,
        test_server: BackendAddr | null
    ): Promise<Path>
    testGetTestbedOrganizationId(
        discriminant_dir: Path
    ): Promise<OrganizationID | null>
    testGetTestbedBootstrapOrganizationAddr(
        discriminant_dir: Path
    ): Promise<BackendOrganizationBootstrapAddr | null>
    testDropTestbed(
        path: Path
    ): Promise<null>
}
