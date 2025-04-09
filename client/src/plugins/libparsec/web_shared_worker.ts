// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/* eslint-disable @typescript-eslint/ban-ts-comment */

import type { ClientEvent, LibParsecPlugin } from '@/plugins/libparsec/definitions';

// @ts-ignore
// eslint-disable-next-line camelcase
import init_module from 'libparsec_bindings_web';
// @ts-ignore
import * as module from 'libparsec_bindings_web';

interface SharedWorkerGlobalScope {
  onconnect: (msg: MessageEvent) => void;
}
const _self: SharedWorkerGlobalScope = self as any;

const onEventBroadcast = new BroadcastChannel('libparsec_on_event');

_self.onconnect = async (msg: MessageEvent): Promise<void> => {
  // console.debug('libparsec_worker: onconnect', msg.data);

  const libparsec = await maybeInit();
  const port = msg.ports[0];
  port.start();

  port.onmessage = async (msg: MessageEvent): Promise<void> => {
    // console.debug('libparsec_worker: onmessage', msg.data);
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
    // console.debug('libparsec_worker: onmessage result', id, name, result);
  };
};

let _libparsec: LibParsecPlugin | undefined = undefined;
async function maybeInit(): Promise<LibParsecPlugin> {
  // Initialization is done once when the sharded worker is created.
  if (_libparsec === undefined) {
    // console.debug('libparsec_worker: Init libparsec...');

    await init_module();
    module.initLogger();

    module.libparsecInitSetOnEventCallback((handle: number, event: ClientEvent) => {
      // console.debug('libparsec_worker: onEventCallback', handle, event);
      onEventBroadcast.postMessage({ handle, event });
    });

    _libparsec = module;
    // console.debug('libparsec_worker: Init libparsec done !');
  }
  return _libparsec as LibParsecPlugin;
}
