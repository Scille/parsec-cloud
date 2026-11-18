// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

import { onConvertToPdfRequest, onSaveRequest, PARENT_ORIGIN, parentWindow, postToParent, setTheme } from './common';
import { openDocument as openOfflineDocument } from './offline';
import type { EditicsOpenOptions, EditicsParentToHostMessage } from './parent_host_api';

// The reply port of an `oo-rpc-*` message: it is always transferred along with
// the message (see `parent_host_api.ts`), a missing one is a bug on the sender
// (the parent) side.
function expectRpcPort(event: MessageEvent, command: string): MessagePort {
  const port = event.ports[0];
  if (!port) {
    throw new Error(`Bug: the '${command}' RPC message arrived without its reply port`);
  }
  return port;
}

// --- host page entry ------------------------------------------------------

// Wires the parent message dispatch and signals the parent that the host
// page is ready to receive the `oo-event-open` message (this is what the parent
// waits for before loading the document, see `src/services/editics.ts`).
//
// `handlers.onOpen` is mode-specific (it plugs the OnlyOffice "server" of the
// mode into `openEditor`).
function startHostPage(handlers: { onOpen: (options: EditicsOpenOptions) => Promise<void> }): void {
  window.addEventListener('message', function (event: MessageEvent) {
    // Filter messages that are not from the parent window.
    if (event.source !== parentWindow || event.origin !== PARENT_ORIGIN) {
      return;
    }
    const data = event.data as EditicsParentToHostMessage | undefined | null;
    if (!data) {
      return;
    }
    switch (data.command) {
      case 'oo-event-open':
        handlers.onOpen(data.options).catch(function (err: Error) {
          console.error('Error while opening the document', err);
          postToParent({ command: 'oo-event-error', details: String(err) });
        });
        break;

      case 'oo-event-set-theme':
        // The app theme changed while the editor is open.
        setTheme(data.theme);
        break;

      case 'oo-rpc-request-save':
        onSaveRequest(expectRpcPort(event, data.command));
        break;

      case 'oo-rpc-convert-to-pdf':
        onConvertToPdfRequest(expectRpcPort(event, data.command));
        break;

      default:
        // Guard to have the compilation fail whenever a new message is added
        // to the protocol.
        data satisfies never;
        break;
    }
  });

  postToParent({ command: 'oo-event-host-ready' });
}

startHostPage({
  onOpen: async (options: EditicsOpenOptions): Promise<void> => {
    if (options.collaborative !== undefined) {
      // The collaborative edition mode is not implemented yet.
      throw new Error('Editics collaborative edition is not implemented yet');
    } else {
      await openOfflineDocument(options);
    }
  },
});
