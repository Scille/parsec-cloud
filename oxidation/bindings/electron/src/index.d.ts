// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */


export type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E };


export interface AvailableDevice {
    key_file_path: string;
    organization_id: string;
    device_id: string;
    human_handle: string | null;
    device_label: string | null;
    slug: string;
    ty: DeviceFileType;
}


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


export function listAvailableDevices(path: string): Promise<Array<AvailableDevice>>;
