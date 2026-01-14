// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import type {
  ClientConfig,
  ClientEvent,
  ClientStartError,
  ClientStopError,
  DeviceAccessStrategy,
  Handle,
  Result,
} from '@/plugins/libparsec/definitions';
import { ClientStartErrorTag } from '@/plugins/libparsec/definitions';

// @ts-expect-error: `libparsec_bindings_web` is a wasm module with exotic loading
// eslint-disable-next-line camelcase
import init_module from 'libparsec_bindings_web';
// @ts-expect-error: `libparsec_bindings_web` is a wasm module with exotic loading
import * as module from 'libparsec_bindings_web';

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

  // SharedWorker is not available with Chrome for Android :/
  // So in this case we fallback to have libparsec running in the tab, with some
  // web locks to avoid having multiple tabs using the same device.
  if (window.SharedWorker !== undefined) {
    console.log('Starting libparsec in shared worker mode');
    return await withSharedWorker();
  } else {
    console.log('Starting libparsec in non worker mode');
    return await withoutSharedWorker();
  }
}

async function withSharedWorker(): Promise<any> {
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
  worker.onerror = (e: Event): void => {
    console.error(`libparsec_worker: an error occurred: ${e}`);
  };

  const inFlight: Array<{
    id: number;
    resolve: (value: unknown) => void;
    reject: (value: unknown) => void;
  }> = [];
  let nextInFlightId = 1;

  worker.port.onmessageerror = (e: MessageEvent): void => {
    console.error(`libparsec_port: error on the worker side: ${e.data}`);
  };

  worker.port.onmessage = (e: MessageEvent): void => {
    // console.debug('libparsec_port: onmessage', e.data);
    const data = e.data as { id: number; result: any; isException: boolean };
    for (let i = 0; i < inFlight.length; i += 1) {
      const candidate = inFlight[i];
      if (candidate.id === data.id) {
        inFlight.splice(i, 1);
        if (!data.isException) {
          candidate.resolve(data.result);
        } else {
          candidate.reject(data.result);
        }
        // console.debug('libparsec_port: method call resolved', e.data);
        return;
      }
    }
    console.error('libparsec_port: Invalid message ID libparsec', e.data);
  };

  // Notify that we are ready to process messages.
  // This needs to be done after setting up the event listener.
  console.log('Client is ready to process events from worker');
  worker.port.start();

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
        return new Promise((resolve, reject) => {
          target.port.postMessage({ id: nextInFlightId, name, args });
          inFlight.push({ id: nextInFlightId, resolve, reject });
          nextInFlightId += 1;
        });
      };
    }
  }

  const proxy = new Proxy(worker, new WorkerProxy());

  return proxy;
}

async function withoutSharedWorker(): Promise<any> {
  await init_module();
  module.initLogger(import.meta.env.PARSEC_APP_DEFAULT_LIB_LOG_LEVEL);

  // Array of [<started client handle>, <keyFile>, <callback to release the lock>]
  // Note the client handle can be undefined since we first acquire the lock, then
  // start the client, and only then update the lock info to register the handle.
  //
  // Note only the locks that have been acquired in the current tab are present,
  // so you shouldn't look into this array to check if a lock is already taken !
  interface TabLockInfo {
    handle: Handle | undefined;
    keyFile: string;
    releaseLock: () => void;
  }
  const tabLocks: Array<TabLockInfo> = [];

  function releaseTabLock(keyFile: string | undefined, handle: Handle | undefined): boolean {
    for (let i = 0; i < tabLocks.length; i += 1) {
      const item = tabLocks[i];
      if (item.handle === handle || item.keyFile === keyFile) {
        item.releaseLock();
        tabLocks.splice(i, 1);
        return true;
      }
    }
    return false;
  }

  function registerTabLock(keyFile: string, releaseLock: () => void): void {
    // Note the `undefined` handle since we haven't started the client yet (hence
    // `registerTabLockClientHandle` that will be called once the client is started).
    tabLocks.push({ handle: undefined, keyFile, releaseLock });
  }

  function registerTabLockClientHandle(keyFile: string, handle: Handle): void {
    for (let i = 0; i < tabLocks.length; i += 1) {
      const item = tabLocks[i];
      if (item.keyFile === keyFile) {
        if (item.handle === undefined) {
          item.handle = handle;
        }
        break;
      }
    }
  }

  function isLockFromThisTab(keyFile: string): boolean {
    for (let i = 0; i < tabLocks.length; i += 1) {
      if (tabLocks[i].keyFile === keyFile) {
        return true;
      }
    }
    return false;
  }

  class WorkerProxy {
    get(_target: any, name: any): any {
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

      if (name === 'clientStart') {
        return async (config: ClientConfig, access: DeviceAccessStrategy): Promise<Result<Handle, ClientStartError>> => {
          const onLockTaken = new Promise((onLockTakenResolve: (value: undefined | ClientStartError) => void) => {
            navigator.locks.request(
              access.keyFile,
              {
                mode: 'exclusive',
                ifAvailable: true,
              },
              // The lock will be kept as long as this function lives
              async (lock: Lock | null) => {
                if (lock === null) {
                  // Lock already taken... but by whom ?
                  if (isLockFromThisTab(access.keyFile)) {
                    // Our tab has the lock, libparsec will handle this fine by
                    // going idempotent (i.e. returning the already start client).
                    onLockTakenResolve(undefined);
                  } else {
                    // Another tab has the lock, we can't start the client !
                    onLockTakenResolve({
                      tag: ClientStartErrorTag.DeviceUsedByAnotherProcess,
                      error: 'Device already used in another tab',
                    } as ClientStartError);
                  }
                } else {
                  // We have the lock, it will be held as long as this function lives.

                  // Register in the locks
                  const onDone = new Promise((resolve: (value: any) => void) => {
                    registerTabLock(access.keyFile, () => resolve(null));
                  });

                  onLockTakenResolve(undefined);
                  await onDone;
                }
              },
            );
          });

          const maybeLockError = await onLockTaken;
          if (maybeLockError !== undefined) {
            return { ok: false, error: maybeLockError };
          }

          const ret = (await module.clientStart(config, access)) as Result<Handle, ClientStartError>;

          if (!ret.ok) {
            // Release the lock since we failed to start the client !
            releaseTabLock(access.keyFile, undefined);
          } else {
            registerTabLockClientHandle(access.keyFile, ret.value);
          }

          return ret;
        };
      }

      if (name === 'clientStop') {
        return async (client: Handle): Promise<Result<null, ClientStopError>> => {
          const ret = (await module.clientStop(client)) as Result<null, ClientStopError>;

          // We only release the lock if the client was successfully stopped, else
          // we consider the device is still in use.
          if (ret.ok) {
            releaseTabLock(undefined, client);
          }

          return ret;
        };
      }

      return module[name];
    }
  }

  const proxy = new Proxy({}, new WorkerProxy());

  return proxy;
}
