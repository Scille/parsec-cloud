// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { registerPlugin } from '@capacitor/core';

import { LibParsecPlugin } from '@/plugins/libparsec/definitions';
import { LoadWebLibParsecPlugin } from '@libparsec_trampoline';

const _libparsec = registerPlugin<LibParsecPlugin>('LibParsec', {
  electron: () => (window as any).libparsec_plugin,

  // In Web we have the whole libparsec compiled in Wasm and provided as a module
  // that we can import just like regular js.
  // This is cool but has an unexpected side effect: this module is going to be
  // packaged even if we are not building the application for web.
  // Worst: if we are building from a clean repo, the Wasm module doesn't exist
  // at all and the packaging fails :'(
  //
  // To avoid this we have to make sure the code that contains the import of the
  // Wasm module is only present when building the application for web.
  // We do this by extracting the `LoadWebLibParsecPlugin` into a `trampoline.ts`, this
  // file having two implementation (the real one when building for the web and a dummy
  // one otherwise) that are selected when `vue-cli-service` is called (see `vue.config.js`).
  web: LoadWebLibParsecPlugin,

  // In Android, capacitor already knows about the plugin given it has been registered from Java
});

function logError(level: 'warn' | 'error', message: string): void {
  // may not be available right at the start
  if (window.electronAPI && window.electronAPI.log) {
    window.electronAPI.log(level, message);
  } else {
    console[level](message);
  }
}

interface SkipData {
  tag: string;
  function: string;
}

const FUNCTIONS_TO_SKIP: Array<SkipData> = [
  {
    tag: 'WorkspaceStatEntryErrorEntryNotFound',
    function: 'workspaceStatEntry',
  },
];

class ParsecProxy {
  get(target: LibParsecPlugin, name: any) {
    return async (...args: any[]): Promise<any> => {
      try {
        // @ts-expect-error Dont know how to fix the `...arg` properly so that the linter doesn't complain
        const result = await target[name as keyof LibParsecPlugin](...args);
        if (
          result &&
          (result as any).ok === false &&
          !FUNCTIONS_TO_SKIP.find((sd) => sd.function === name && (result as any).error && (result as any).error.tag === sd.tag)
        ) {
          const resultError = result as { ok: boolean; error: { tag: string; error: string } };
          logError('warn', `Error when calling ${name}: ${JSON.stringify(resultError.error)}`);
        }
        return result;
      } catch (err: any) {
        logError('error', `Exception when calling ${name}: ${String(err)}`);
        return { ok: false, error: { tag: 'bindings-exception', error: String(err) } };
      }
    };
  }
}

const proxy = new Proxy(_libparsec, new ParsecProxy());

export const libparsec = proxy;

export * from '@/plugins/libparsec/definitions';

// Global exposition of libparsec for easier debugging with console
declare global {
  interface Window {
    libparsec: LibParsecPlugin;
  }
}
window.libparsec = proxy;
