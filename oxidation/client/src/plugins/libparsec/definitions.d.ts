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

export interface LibParsecPlugin {
    listAvailableDevices(path: string): Promise<Array<AvailableDevice>>;
}
