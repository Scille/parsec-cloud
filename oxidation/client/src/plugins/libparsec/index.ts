// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { registerPlugin } from '@capacitor/core';

import type { LibParsecPlugin } from './definitions';
import { Buffer } from 'buffer';

// TODO: Initialize only for web
import init from '../../../pkg';
init();

// Low-level API

const libparsecPlugin = registerPlugin<LibParsecPlugin>(
  'LibParsec',
  {
    web: () => import('../../../pkg'),
    // electron: () => (window as any).CapacitorCustomPlatform.plugins.LibParsec,
    electron: () => (window as any).libparsec_plugin
  }
);

// High-level API

/* eslint @typescript-eslint/no-empty-function: ["error", { "allow": ["private-constructors"] }] */
class LibParsec {
  // Singleton stuff

  private static instance: LibParsec;
  private constructor() {}

  public static getInstance(): LibParsec {
    if (!LibParsec.instance) {
      LibParsec.instance = new LibParsec();
    }

    return LibParsec.instance;
  }

  // Actual api

  public async version(): Promise<string> {
    const ret = await libparsecPlugin.submitJob({cmd: 'version', payload: ''});
    return ret.value;
  }

  public async encrypt(key: Uint8Array, cleartext: string): Promise<Uint8Array> {
    const b64key = Buffer.from(key).toString('base64');
    const b64clearText = Buffer.from(cleartext).toString('base64');
    const ret = await libparsecPlugin.submitJob({cmd: 'encrypt', payload: `${b64key}:${b64clearText}`});
    return Buffer.from(ret.value, 'base64');
  }

  public async decrypt(key: Uint8Array, cyphertext: Uint8Array): Promise<string> {
    const b64key = Buffer.from(key).toString('base64');
    const b64cyphertext = Buffer.from(cyphertext).toString('base64');
    const ret = await libparsecPlugin.submitJob({cmd: 'decrypt', payload: `${b64key}:${b64cyphertext}`});
    return Buffer.from(ret.value, 'base64').toString();
  }

  public async listAvailableDevices(configDir: string): Promise<Array<AvailableDevice>> {
    const payload = Buffer.from(configDir).toString('base64');
    const ret = await libparsecPlugin.submitJob({cmd: 'list_available_devices', payload });
    return JSON.parse(ret.value);
  }
}

type AvailableDevice = {
  key_file_path: string,
  organization_id: string,
  device_id: string,
  // Email and Label
  human_handle?: Array<string>,
  device_label?: string,
  slug: string,
  type: string,
}

const libparsec = LibParsec.getInstance();

// Global exposition of libparsec for easier debugging with console
declare global {
    interface Window { libparsec: LibParsec; }
}
window.libparsec = libparsec;

export { LibParsec, libparsec };
