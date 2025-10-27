// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import type { ClientEvent, LibParsecPlugin } from '@/plugins/libparsec/definitions';

// @ts-expect-error: `libparsec_bindings_web` is a wasm module with exotic loading
// eslint-disable-next-line camelcase
import init_module from 'libparsec_bindings_web';
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

    // Since libparsec bindings never raise exceptions under normal conditions,
    // we don't need to catch them: any exception here is a bug and we should
    // just let it bubble up so that Sentry takes care of it.
    const result = await promise;

    port.postMessage({ id, result, isException: false });
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
    console.log('Done initializing libparsec module');

    _libparsec = module;
  }
  return _libparsec as LibParsecPlugin;
}
