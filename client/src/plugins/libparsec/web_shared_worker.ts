// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/* eslint-disable @typescript-eslint/ban-ts-comment */

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
    // Since libparsec bindings never raise exceptions under normal conditions,
    // we don't need to catch them: any exception here is a bug and we should
    // just let it bubble up so that Sentry takes care of it.
    const result = await (libparsec[name as keyof LibParsecPlugin] as (...args: any[]) => Promise<any>)(...args);
    port.postMessage({ id, result });
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
    module.initLogger();

    module.libparsecInitSetOnEventCallback((handle: number, event: ClientEvent) => {
      onEventBroadcast.postMessage({ handle, event });
    });
    console.log('Done initializing libparsec module');

    _libparsec = module;
  }
  return _libparsec as LibParsecPlugin;
}
