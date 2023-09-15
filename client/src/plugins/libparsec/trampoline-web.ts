// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/* eslint-disable @typescript-eslint/ban-ts-comment */

// @ts-ignore
// eslint-disable-next-line camelcase
import init_module from 'libparsec_bindings_web';
// @ts-ignore
import * as module from 'libparsec_bindings_web';

export async function LoadWebLibParsecPlugin(): Promise<any> {
  // Vitest runs it tests on node.js, however capacitor plugin loader defaults
  // to Web (given we are not running on Electron nor Android).
  // This is unfortunate given our web plugin logic (this code) is going to
  // load a .wasm file which is not supported by node.js !
  //
  // From there what we can do:
  // 1. Use `vi.mock` to overwrite the plugin logic and instead load the electron
  //    version of the plugin (which is provided as a .node file).
  // 2. Use `vi.mock` to overwrite the plugin logic to load the .wasm through
  //    the special node.js Webassembly support system.
  // 3. The special solution ಠ_ಠ
  //
  // Solution 1 is not great given it means we need to compile electron bindings
  // for running Vitest tests (while Cypress tests require to compile web bindings).
  //
  // Solution 2 is probably a mess: Vitest doesn't provide the web APIs, but
  // libparsec relies on them to send HTTP requests to the testbed server (so
  // more mocks are needed...).
  //
  // Hence we go with solution 3: prevent use of libparsec from Vitest.
  if (import.meta.env.VITEST) {
    throw new Error("libparsec is not available with Vitest, use Cypress instead !");
  }

  await init_module();
  module.initLogger();
  return module;
}
