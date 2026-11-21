// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

import { openDocument as openCollaborativeDocument } from './collaborative';
import {
  getCollaborativeSession,
  onConvertToPdfRequest,
  onSaveRequest,
  PARENT_ORIGIN,
  parentWindow,
  postToParent,
  setTheme,
} from './common';
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
        // Nothing to do on resolution: the session (if any) registered
        // itself on construction, see `CollaborativeEditicsServer`.
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

  // When the page goes away (e.g. the user leaves the file editor), notify the
  // server so the other participants are told we left (`unLockDocument` +
  // `close`). The SSE stream is closed by the browser
  // itself, this is about releasing the locks and the participant slot.
  window.addEventListener('pagehide', () => {
    getCollaborativeSession()?.close();
  });

  postToParent({ command: 'oo-event-host-ready' });
}

startHostPage({
  onOpen: async (options: EditicsOpenOptions): Promise<void> => {
    if (options.collaborative !== undefined) {
      await openCollaborativeDocument(options);
    } else {
      await openOfflineDocument(options);
    }
  },
});
