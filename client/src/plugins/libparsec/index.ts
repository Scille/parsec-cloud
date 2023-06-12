// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { registerPlugin } from '@capacitor/core';

import type { LibParsecPlugin } from './definitions';
import { LoadWebLibParsecPlugin } from './trampoline';

export const libparsec = registerPlugin<LibParsecPlugin>(
  'LibParsec',
  {
    electron: () => (window as any).libparsec_plugin,

    // In Web we have the whole libparsec compiled in Wasm and provided as a module
    // that we can import just like regular js.
    // This is cool but has an unexpected side effect: Webpack is going to package
    // this module event if we are not building the application for web.
    // Worst: if we are building from a clean repo, the Wasm module doesn't exist
    // at all and the packaging fails :'(
    //
    // To avoid this we have to make sure the code that contain the import of the
    // Wasm module is only present when building the application for web.
    // We do this by extracting the `LoadWebLibParsecPlugin` into a `trampoline.ts`, this
    // file having two implementation (the real one when building for the web and a dummy
    // one otherwise) that are selected when `vue-cli-service` is called (see `vue.config.js`).
    web: LoadWebLibParsecPlugin

    // In Android, capacitor already knows about the plugin given it has been registered from Java
  }
);

// Global exposition of libparsec for easier debugging with console
declare global {
  interface Window { libparsec: LibParsecPlugin; }
}
window.libparsec = libparsec;
