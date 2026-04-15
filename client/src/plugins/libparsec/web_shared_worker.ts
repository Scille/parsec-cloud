// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import type { ClientEvent, LibParsecPlugin } from '@/plugins/libparsec/definitions';

/** @lintignore */ // exclude from knip report
// @ts-expect-error: `libparsec_bindings_web` is a wasm module with exotic loading
// eslint-disable-next-line camelcase
import init_module from 'libparsec_bindings_web';
/** @lintignore */ // exclude from knip report
// @ts-expect-error: `libparsec_bindings_web` is a wasm module with exotic loading
import * as module from 'libparsec_bindings_web';

interface SharedWorkerGlobalScope {
  onconnect: (msg: MessageEvent) => void;
}
declare const self: SharedWorkerGlobalScope;

const onEventBroadcast = new BroadcastChannel('libparsec_on_event');

self.onconnect = async (msg: MessageEvent): Promise<void> => {
  const libparsec = await maybeInit();
  const port = msg.ports[0];

  port.onmessageerror = (msg: MessageEvent): void => {
    console.error(`libparsec_port: error on the client side: ${msg.data}`);
  };

  port.onmessage = async (msg: MessageEvent): Promise<void> => {
    const { id, name, args } = msg.data as { id: number; name: string; args: [...any] };
    if (!id) {
      console.error('libparsec_worker: skipping unknown message', msg.data);
      return;
    }
    let promise;
    try {
      // An exception can be raised here in case the function doesn't exist, or the
      // provided parameters are of the wrong type
      const fn = libparsec[name as keyof LibParsecPlugin] as (...args: any[]) => Promise<any>;
      promise = fn(...args);
    } catch (err) {
      port.postMessage({ id, result: err, isException: true });
      return;
    }

    try {
      const result = await promise;
      port.postMessage({ id, result, isException: false });
    } catch (err) {
      // Forward exceptions to the GUI where they are treated as generic errors,
      // otherwise it may block for all eternity because it doesn't get a return value.
      // It's also helpful to have the error directly in the console instead of having to inspect
      // the worker.
      port.postMessage({ id, result: err, isException: true });
      // re-throw for Sentry
      throw err;
    }
  };

  // Notify the client that the worker is ready to process messages.
  // This needs to be done after setting up the event listener.
  console.log('Worker is ready to process events from client');
  port.start();
};

let _libparsec: LibParsecPlugin | undefined = undefined;
async function maybeInit(): Promise<LibParsecPlugin> {
  // Initialization is done once the shared worker is created.
  if (_libparsec === undefined) {
    console.log('Initializing libparsec module');
    await init_module();
    module.initLogger(import.meta.env.PARSEC_APP_DEFAULT_LIB_LOG_LEVEL);

    module.libparsecInitSetOnEventCallback((handle: number, event: ClientEvent) => {
      onEventBroadcast.postMessage({ handle, event });
    });

    const wrappedLibparsec = new Proxy(module, {
      get(target, prop, receiver) {
        // SCWS needs to be dynamically loaded.
        // We're loading it in TypeScript inside the shared worker and attaching
        // it to the global context.
        if (prop === 'pkiInitForScws') {
          return async (configDir: string, parsecAddr: string, scwsLocation: string, certificate: string) => {
            try {
              console.log('Loading SCWS...');
              const scwsapi = await import(/* @vite-ignore */ scwsLocation);
              (globalThis as any).SCWS = scwsapi.SCWS;
              (globalThis as any).SCWS_WEBAPP_CERT = certificate;
            } catch (err: any) {
              console.error(`Failed to import scwsapi: ${err.toString()}`);
              return { ok: false, error: { tag: 'PkiSystemInitErrorNotAvailable', error: `Failed to import scwsapi (${err.toString()})` } };
            }

            return await target.pkiInitForScws(configDir, parsecAddr);
          };
        }
        return Reflect.get(target, prop, receiver);
      },
    });

    console.log('Done initializing libparsec module');

    _libparsec = wrappedLibparsec;
  }
  return _libparsec as LibParsecPlugin;
}
