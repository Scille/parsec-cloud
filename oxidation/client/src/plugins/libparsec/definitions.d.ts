// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */

export type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E };

type OrganizationID = string;
type DeviceLabel = string;
type HumanHandle = string;
type StrPath = string;
type DeviceID = string;
type LoggedCoreHandle = number;

export interface AvailableDevice {
    keyFilePath: StrPath;
    organizationId: OrganizationID;
    deviceId: DeviceID;
    humanHandle: HumanHandle | null;
    deviceLabel: DeviceLabel | null;
    slug: string;
    ty: DeviceFileType;
}

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

// LoggedCoreError
export interface LoggedCoreErrorDisconnected {
    tag: 'Disconnected'
}
export interface LoggedCoreErrorInvalidHandle {
    tag: 'InvalidHandle'
    handle: number;
}
export interface LoggedCoreErrorLoginFailed {
    tag: 'LoginFailed'
    help: string;
}
export type LoggedCoreError =
  | LoggedCoreErrorDisconnected
  | LoggedCoreErrorInvalidHandle
  | LoggedCoreErrorLoginFailed

export interface LibParsecPlugin {
    listAvailableDevices(path: StrPath): Promise<Array<AvailableDevice>>;
    login(key: string, password: string): Promise<Result<number, LoggedCoreError>>;
    loggedCoreGetDeviceId(handle: number): Promise<Result<DeviceID, LoggedCoreError>>;
    loggedCoreGetDeviceDisplay(handle: number): Promise<Result<string, LoggedCoreError>>;
}
