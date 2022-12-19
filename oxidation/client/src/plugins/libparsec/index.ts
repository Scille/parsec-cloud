// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { registerPlugin } from '@capacitor/core';

import type { LibParsecPlugin } from './definitions';
export type { LibParsecPlugin, Result, HelloError } from './definitions';

export const libparsec = registerPlugin<LibParsecPlugin>(
  'LibParsec',
  {
    web: async () => {
      // Don't put the module name as a literal in the import function,
      // otherwise the compilation will eagerly try to access this module
      // which doesn't exists when building from scratch for non-web.
      const moduleName = '../../../pkg';
      const libparsecWasm: any = await import(moduleName);
      await libparsecWasm.default();  // Call wasm module's init
      return libparsecWasm;
    },
    electron: () => (window as any).libparsec_plugin
    // In Android, capacitor already knows about the plugin given it has been registered from Java
  }
);

// Global exposition of libparsec for easier debugging with console
declare global {
  interface Window { libparsec: LibParsecPlugin; }
}
window.libparsec = libparsec;
