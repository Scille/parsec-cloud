// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

import { registerPlugin } from '@capacitor/core';

import type { LibParsecPlugin } from './definitions';

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

  public async encrypt(key: string, cleartext: string): Promise<string> {
    const ret = await libparsecPlugin.submitJob({cmd: 'encrypt', payload: `${key}:${cleartext}`});
    return ret.value;
  }

  public async decrypt(key: string, cyphertext: string): Promise<string> {
    const ret = await libparsecPlugin.submitJob({cmd: 'decrypt', payload: `${key}:${cyphertext}`});
    return ret.value;
  }
}

const libparsec = LibParsec.getInstance();

// Global exposition of libparsec for easier debugging with console
declare global {
    interface Window { libparsec: LibParsec; }
}
window.libparsec = libparsec;

export { LibParsec, libparsec };
