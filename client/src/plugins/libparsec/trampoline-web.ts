// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ClientEvent } from '@/plugins/libparsec/definitions';

const onEventBroadcast = new BroadcastChannel('libparsec_on_event');

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
  // for running Vitest tests (while playwright tests require to compile web bindings).
  //
  // Solution 2 is probably a mess: Vitest doesn't provide the web APIs, but
  // libparsec relies on them to send HTTP requests to the testbed server (so
  // more mocks are needed...).
  //
  // Hence we go with solution 3: prevent use of libparsec from Vitest.
  if (import.meta.env.VITEST) {
    throw new Error('libparsec is not available with Vitest, use playwright instead !');
  }

  // In order to be accessible from all tabs, the libparsec web plugin runs in a shared worker.
  //
  // This makes communication between the GUI (i.e. this code which is running in a tab)
  // and libparsec (i.e. the code running in the shared worker) a bit tricky:
  // - The shared worker's port is used to send function calls request.
  // - Once executed, the shared worker posts the result of the function call
  //   through the same port.
  // - Client events are broadcasted through a `BroadcastChannel` (i.e. the events are
  //   broadcasted to all the tabs).

  const worker = new SharedWorker(new URL('./web_shared_worker.ts', import.meta.url), {
    type: 'module',
    name: 'libparsec',
  });
  worker.port.start();

  const inFlight: Array<{ id: number; resolve: (value: unknown) => void }> = [];
  let nextInFlightId = 1;

  worker.port.onmessage = (e: MessageEvent): void => {
    // console.debug('libparsec_port: onmessage', e.data);
    const data = e.data as { id: number; result: any };
    for (let i = 0; i < inFlight.length; i += 1) {
      const candidate = inFlight[i];
      if (candidate.id === data.id) {
        inFlight.splice(i, 1);
        candidate.resolve(data.result);
        // console.debug('libparsec_port: method call resolved', e.data);
        return;
      }
    }
    console.error('libparsec_port: Invalid message ID libparsec', e.data);
  };

  class WorkerProxy {
    get(target: SharedWorker, name: any): any {
      // Are you ready for another Javascript clusterfuck ?
      // - Awaiting a promise is done recursively: if the promise returns another promise,
      //   it is awaited too.
      // - Promise are just syntactic sugar for calling `then` method on the promise object.
      //
      // This is a mess since it means the await mechanism has to rely on introspection
      // to detect if the object should be awaited or not...
      //
      // Hence this guard: since we return this proxy object from an async function,
      // Javascript will look for a `then` method on it and call it if it exists ><''
      if (name === 'then') {
        return undefined;
      }

      // Configuring the event callback requires a special trick:
      // - The actual configuration is done once in the shared worker when it is initializing.
      // - The shared worker then broadcasts the events using a `BroadcastChannel`.
      // So here we only need to plug the callback to the existing broadcast channel.
      if (name === 'libparsecInitSetOnEventCallback') {
        return async (callback: (handle: number, event: ClientEvent) => void) => {
          onEventBroadcast.onmessage = (e: MessageEvent): void => {
            // console.debug('libparsec_port: onEventBroadcast', e.data);
            const { handle, event } = e.data as { handle: number; event: ClientEvent };
            callback(handle, event);
          };
        };
      }

      return async (...args: any[]): Promise<any> => {
        // console.debug('libparsec_port: method call', nextInFlightId, name, args);
        return new Promise((resolve, _reject) => {
          target.port.postMessage({ id: nextInFlightId, name, args });
          inFlight.push({ id: nextInFlightId, resolve });
          nextInFlightId += 1;
        });
      };
    }
  }

  const proxy = new Proxy(worker, new WorkerProxy());

  return proxy;
}
