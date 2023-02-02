// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */


export type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E };


export interface AvailableDevice {
    keyFilePath: string;
    organizationId: string;
    deviceId: string;
    humanHandle: string | null;
    deviceLabel: string | null;
    slug: string;
    ty: DeviceFileType;
}


export interface ClientConfig {
    configDir: string;
    dataBaseDir: string;
    mountpointBaseDir: string;
    preferredOrgCreationBackendAddr: string;
    workspaceStorageCacheSize: WorkspaceStorageCacheSize;
}


// ClientEvent
export interface ClientEventClientConnectionChanged {
    tag: "ClientConnectionChanged"
    client: number;
}
export interface ClientEventWorkspaceReencryptionEnded {
    tag: "WorkspaceReencryptionEnded"
}
export interface ClientEventWorkspaceReencryptionNeeded {
    tag: "WorkspaceReencryptionNeeded"
}
export interface ClientEventWorkspaceReencryptionStarted {
    tag: "WorkspaceReencryptionStarted"
}
export type ClientEvent =
  | ClientEventClientConnectionChanged
  | ClientEventWorkspaceReencryptionEnded
  | ClientEventWorkspaceReencryptionNeeded
  | ClientEventWorkspaceReencryptionStarted


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


// WorkspaceStorageCacheSize
export interface WorkspaceStorageCacheSizeCustom {
    tag: "Custom"
    size: number;
}
export interface WorkspaceStorageCacheSizeDefault {
    tag: "Default"
}
export type WorkspaceStorageCacheSize =
  | WorkspaceStorageCacheSizeCustom
  | WorkspaceStorageCacheSizeDefault


// DeviceAccessParams
export interface DeviceAccessParamsPassword {
    tag: "Password"
    path: string;
    password: string;
}
export interface DeviceAccessParamsSmartcard {
    tag: "Smartcard"
    path: string;
}
export type DeviceAccessParams =
  | DeviceAccessParamsPassword
  | DeviceAccessParamsSmartcard


// ClientLoginError
export interface ClientLoginErrorAccessMethodNotAvailable {
    tag: "AccessMethodNotAvailable"
}
export interface ClientLoginErrorDecryptionFailed {
    tag: "DecryptionFailed"
}
export interface ClientLoginErrorDeviceAlreadyLoggedIn {
    tag: "DeviceAlreadyLoggedIn"
}
export interface ClientLoginErrorDeviceInvalidFormat {
    tag: "DeviceInvalidFormat"
}
export type ClientLoginError =
  | ClientLoginErrorAccessMethodNotAvailable
  | ClientLoginErrorDecryptionFailed
  | ClientLoginErrorDeviceAlreadyLoggedIn
  | ClientLoginErrorDeviceInvalidFormat


// ClientGetterError
export interface ClientGetterErrorDisconnected {
    tag: "Disconnected"
}
export interface ClientGetterErrorInvalidHandle {
    tag: "InvalidHandle"
    handle: number;
}
export type ClientGetterError =
  | ClientGetterErrorDisconnected
  | ClientGetterErrorInvalidHandle


export function clientListAvailableDevices(
    path: string
): Promise<Array<AvailableDevice>>;
export function clientLogin(
    load_device_params: DeviceAccessParams,
    config: ClientConfig,
    on_event_callback: (event: ClientEvent) => void
): Promise<Result<number, ClientLoginError>>;
export function clientGetDeviceId(
    handle: number
): Promise<Result<string, ClientGetterError>>;
