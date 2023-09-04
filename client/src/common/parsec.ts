// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  // Types
  EntryID,
  AvailableDevice,
  Result,
  Handle,
  ClientEvent,
  DeviceAccessStrategyPassword,
  ClientConfig,
  EntryName,

  // Errors
  ClientStartError,
} from '@/plugins/libparsec/definitions';
import { libparsec } from '@/plugins/libparsec';

const DEFAULT_HANDLE = 42;

export type WorkspaceID = EntryID;
export type WorkspaceName = EntryName;

export async function listAvailableDevices(): Promise<Array<AvailableDevice>> {
  return await libparsec.listAvailableDevices(window.getConfigDir());
}

export async function login(device: AvailableDevice, password: string): Promise<Result<Handle, ClientStartError>> {
  function parsecEventCallback(event: ClientEvent): void {
    console.log('Event received', event);
  }

  if (window.isDesktop()) {
    const clientConfig: ClientConfig = {
      configDir: window.getConfigDir(),
      dataBaseDir: window.getDataBaseDir(),
      mountpointBaseDir: window.getMountpointDir(),
      workspaceStorageCacheSize: {tag: 'Default'},
    };
    const strategy: DeviceAccessStrategyPassword = {
      tag: 'Password',
      password: password,
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      // @ts-ignore
      // eslint-disable-next-line camelcase
      key_file: device.keyFilePath,
      keyFile: device.keyFilePath,
    };
    return await libparsec.clientStart(clientConfig, parsecEventCallback, strategy);
  } else {
    return new Promise<Result<Handle, ClientStartError>>((resolve, _reject) => {
      resolve({ok: true, value: DEFAULT_HANDLE });
    });
  }
}
