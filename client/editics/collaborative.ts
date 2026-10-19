// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Collaborative edition mode host page (see docs/rfcs/1030-collaborative-editics.md).
//
// This page plays the same role as `offline.ts` (load the document into the
// OnlyOffice editor with x2t, handle saves/images with the parent window, see
// the shared `common.ts`), except the OnlyOffice editor's "server" is not a
// local mock but the actual Parsec editics server:
//
// - Editor events (OnlyOffice protocol) are translated into the editics
//   protocol by `EditicsTranslator` (see `collaborative_protocol.js`) and sent
//   with a HTTP POST.
// - Server events received on a SSE stream are translated back and pushed to
//   the editor.
//
// The routes and the event schemas are the ones implemented by the server
// (see `server/parsec/asgi/editics.py` & `server/parsec/editics_protocol.py`):
//
//     GET  {serverUrl}/authenticated/{organizationId}/editics/sessions/{workspaceId}/{vlobId}/listen/{participantId}
//     POST {serverUrl}/authenticated/{organizationId}/editics/sessions/{workspaceId}/{vlobId}/send/{participantId}
//
// This file is browser-only (uses `EventSource` & `fetch`).

import type OO from 'onlyoffice-editor';
import { EditicsTranslator } from './collaborative_protocol';
import { openEditor, postToParent, startHostPage } from './common';
import type { EditicsCollaborativeSession, EditicsDocumentTypes, EditicsOpenOptions, EditicsParentToHostMessage } from './parent_host_api';

// OnlyOffice document type -> editics protocol `editorType`
// (0=Word, 1=Spreadsheet, 2=Presentation, 3=Visio, see RFC 1030 §2.2).
const EDITICS_EDITOR_TYPES: Record<EditicsDocumentTypes, number> = {
  word: 0,
  cell: 1,
  slide: 2,
  pdf: 3,
};

// Time we give the parent to answer a device name resolution request before
// giving up (the participant display names are not critical, so the document
// opening is not blocked for long: the translator falls back on a placeholder).
const RESOLVE_NAME_TIMEOUT_MS = 3000;

// Time we give the SSE stream to be open before starting the editor anyway
// (the editor takes several seconds to load, so the stream is expected to be
// long ready by the time the editor sends its `auth` event; this is just a
// safety net so the two don't race).
const SSE_OPEN_TIMEOUT_MS = 2000;

// A raw editics server event, as serialized on the wire (see
// `server/parsec/editics_protocol.py`: the encrypted fields are base64-encoded
// strings, which is exactly what the translator expects).
type RawServerEvent = { type?: string } & Record<string, unknown>;

function logError(message: string, error?: unknown): void {
  console.error(`[editics-collaborative] ${message}`, error ?? '');
}

// The OnlyOffice "server" of the collaborative mode: implements the
// `MockServer` interface (i.e. answers the editor's co-authoring protocol)
// by forwarding the translated events to/from the editics server.
class CollaborativeEditicsServer implements OO.MockServer {
  private editor: OO.DocEditor;
  private options: EditicsOpenOptions;
  private session: EditicsCollaborativeSession;
  // Arbitrary UUID chosen client-side: identifies this edition session
  // participation towards the server (it survives SSE reconnections, see
  // `server/parsec/components/editics.py`).
  private participantId: string = crypto.randomUUID();
  private translator: EditicsTranslator;
  private sse: EventSource | undefined;
  private closed = false;

  // Server events received on the SSE stream are held back until the editor's
  // `auth` handshake has been delivered: the real OnlyOffice server never
  // sends anything before the `auth` reply, and the editor's state machine is
  // not expected to handle e.g. a `connectState` while still waiting for it.
  // (Our server does broadcast `connectState` to the parked participants.)
  private authDelivered = false;
  private heldServerEvents: RawServerEvent[] = [];

  // The editor's events are translated (async, the encryption capabilities are
  // async) then POSTed: both steps are serialized on this chain so the events
  // keep their emission order towards the server.
  private sends: Promise<void> = Promise.resolve();

  // Same for the server events: cooked (async, the decryption/name resolution
  // capabilities are async) then pushed to the editor in reception order.
  private deliveries: Promise<void> = Promise.resolve();

  // Pending `oo-get-user-name` requests towards the parent (the parent is
  // the only side with access to libparsec, hence the one able to resolve a
  // participant's device ID into a display name, see RFC 1030 §3.3).
  private pendingNameRequests = new Map<number, (userName: string | undefined) => void>();
  private nameRequestCounter = 0;

  constructor(editor: OO.DocEditor, options: EditicsOpenOptions) {
    if (options.collaborative === undefined) {
      // Defense in depth: the parent is expected to serve the offline host
      // page when no collaborative session is provided (see
      // `src/services/editics.ts`).
      throw new Error('Missing collaborative session info');
    }
    this.editor = editor;
    this.options = options;
    this.session = options.collaborative;

    this.translator = new EditicsTranslator(
      {
        workspaceId: this.session.workspaceId,
        vlobId: this.session.vlobId,
        deviceIdHex: this.session.deviceIdHex,
        userId: this.options.userId,
        userName: this.options.userName,
        vlobVersion: this.session.vlobVersion,
        editorType: EDITICS_EDITOR_TYPES[this.options.documentType],
      },
      {
        getHumanLabelFromDeviceId: (deviceIdHex: string) => this.resolveUserName(deviceIdHex),
        // POC: the content is passed through as-is (the server only sees
        // opaque blobs, so a null encryption is valid, see RFC 1030 §1.6).
        // Real sealing with the workspace key will be layered in later without
        // touching the translator (that's the point of the capabilities).
        encrypt: (cleartext: Uint8Array) => Promise.resolve<[number, Uint8Array]>([0, cleartext]),
        decrypt: (_keyIndex: number, ciphertext: Uint8Array) => Promise.resolve(ciphertext),
      },
    );
  }

  // --- OnlyOffice MockServer API ------------------------------------------

  onMessage(msg: OO.OOClientEvent): void {
    if (this.closed) {
      return;
    }
    this.sends = this.sends
      .then(async () => {
        const cooked = await this.translator.cookClientEvent(msg);
        // `null` when the event has no editics counterpart (e.g. `rpc`,
        // `clientLog`) or is not supported yet (e.g. `getLock`, `close`).
        if (cooked === null || cooked === undefined) {
          return;
        }
        await this.postClientEvent(cooked as { type: string });
      })
      .catch((e: unknown) => logError(`Failed to handle the editor '${msg.type}' event`, e));
  }

  // --- Session lifecycle ---------------------------------------------------

  // Opens the SSE stream and resolves once it is open (or after a timeout, so
  // a server hiccup cannot block the document opening forever).
  async start(): Promise<void> {
    await new Promise<void>((resolve) => {
      console.log(`[editics-collaborative] Opening SSE stream on ${this.listenUrl}`);
      const sse = new EventSource(this.listenUrl);
      this.sse = sse;
      let settled = false;
      const done = (): void => {
        if (!settled) {
          settled = true;
          resolve();
        }
      };
      sse.addEventListener('open', done, { once: true });
      // `EventSource` reconnects by itself on error (the events emitted while
      // disconnected are lost: the editics protocol relies on OnlyOffice's own
      // resynchronization logic, see RFC 1030 §2.2).
      sse.onmessage = (ev: MessageEvent<string>): void => {
        let data: RawServerEvent | undefined;
        try {
          data = JSON.parse(ev.data) as RawServerEvent;
        } catch (e) {
          logError('Received a non-JSON server event', e);
          return;
        }
        this.onServerEvent(data, 'sse');
      };
      sse.onerror = (): void => {
        if (this.closed) {
          return;
        }
        logError('SSE stream error (EventSource will retry)');
      };
      setTimeout(done, SSE_OPEN_TIMEOUT_MS);
    });
  }

  // Notify the server we are leaving the session (best-effort: the page is
  // going away, so the request may not complete) and close the SSE stream.
  close(): void {
    if (this.closed) {
      return;
    }
    this.closed = true;
    // Don't wait for the pending sends: `close` must not be held back by them
    // (the editor may still flush its `saveChanges` while we leave).
    this.postClientEvent({ type: 'close' }, { keepalive: true }).catch((e: unknown) => {
      // Best-effort: the server also cleans up when it notices the SSE
      // stream disconnection.
      console.log(`[editics-collaborative] Failed to notify the server of the leave: ${String(e)}`);
    });
    this.sse?.close();
    this.sse = undefined;
    // Don't leave the name resolutions hanging forever.
    for (const resolve of this.pendingNameRequests.values()) {
      resolve(undefined);
    }
    this.pendingNameRequests.clear();
  }

  // --- Parent message handling ---------------------------------------------

  // Called by the host page message dispatch (see `startHostPage`) when the
  // parent replies to an `oo-get-user-name` request.
  handleUserNameResult(requestId: number, userName?: string): void {
    const resolve = this.pendingNameRequests.get(requestId);
    if (resolve) {
      this.pendingNameRequests.delete(requestId);
      resolve(userName);
    }
  }

  // Ask the parent to resolve a participant's device ID into a display name.
  // Resolves `undefined` when the parent doesn't know (or doesn't answer in
  // time): the translator then falls back on a placeholder name.
  private resolveUserName(deviceIdHex: string): Promise<string | undefined> {
    return new Promise((resolve) => {
      const requestId = ++this.nameRequestCounter;
      const timer = setTimeout(() => {
        this.pendingNameRequests.delete(requestId);
        resolve(undefined);
      }, RESOLVE_NAME_TIMEOUT_MS);
      this.pendingNameRequests.set(requestId, (userName) => {
        clearTimeout(timer);
        resolve(userName);
      });
      postToParent({ command: 'oo-get-user-name', requestId, deviceIdHex });
    });
  }

  // --- Editics session HTTP requests ----------------------------------------

  private get authorizationHeader(): string {
    return `Bearer ${this.session.token}`;
  }

  private get sessionBasePath(): string {
    const { serverUrl, organizationId, workspaceId, vlobId } = this.session;
    return `${serverUrl}/authenticated/${encodeURIComponent(organizationId)}/editics/sessions/${workspaceId}/${vlobId}`;
  }

  private get listenUrl(): string {
    // `EventSource` cannot set request headers, so the `Authorization` header
    // value is passed as the `authorization` query parameter instead (the
    // server accepts it as a fallback on the SSE route).
    const params = new URLSearchParams({ authorization: this.authorizationHeader });
    return `${this.sessionBasePath}/listen/${this.participantId}?${params.toString()}`;
  }

  private get sendUrl(): string {
    return `${this.sessionBasePath}/send/${this.participantId}`;
  }

  // Sends an editics client event to the server. The reply (a single server
  // event addressed to us, or 204 no content) is processed like an SSE event.
  private async postClientEvent(event: { type: string }, opts: { keepalive?: boolean } = {}): Promise<void> {
    let rep: Response;
    try {
      rep = await fetch(this.sendUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: this.authorizationHeader,
        },
        body: JSON.stringify(event),
        keepalive: opts.keepalive === true,
      });
    } catch (e) {
      logError(`Failed to send the '${event.type}' event to the editics server`, e);
      throw e;
    }
    if (!rep.ok) {
      logError(`The editics server rejected the '${event.type}' event: HTTP ${rep.status}`);
      return;
    }
    if (rep.status === 204) {
      return;
    }
    this.onServerEvent((await rep.json()) as RawServerEvent, 'reply');
  }

  // --- Server events handling ----------------------------------------------

  private onServerEvent(event: RawServerEvent, source: 'sse' | 'reply'): void {
    if (this.closed) {
      return;
    }
    if (event.type === 'auth') {
      // The `auth` handshake completes: deliver it, then release the events
      // that were held back while the editor was waiting for it.
      this.authDelivered = true;
      this.enqueueDelivery(event);
      for (const held of this.heldServerEvents.splice(0)) {
        this.enqueueDelivery(held);
      }
      return;
    }
    if (source === 'sse' && !this.authDelivered) {
      // See `heldServerEvents`: nothing but `auth` is expected before the
      // handshake completes (e.g. a `connectState` broadcast while we are
      // parked behind another editor's auth lock, see RFC 1030 §6.2).
      this.heldServerEvents.push(event);
      return;
    }
    this.enqueueDelivery(event);
  }

  // The server events must be pushed to the editor in reception order, but
  // their translation is asynchronous (the decryption/name resolution
  // capabilities are async): the deliveries are chained so they cannot
  // overtake each other.
  private enqueueDelivery(event: RawServerEvent): void {
    this.deliveries = this.deliveries
      .then(async () => {
        const oo = await this.translator.cookServerEvent(event as never);
        if (oo === null || oo === undefined) {
          // No OnlyOffice counterpart (unexpected though, typically a bug):
          // nothing to push.
          return;
        }
        if (oo.type === 'auth' && oo.settings?.reconnection !== undefined) {
          // Adaptation for the CryptPad fork's editor: it expects engine.io/
          // socket.io and crashes on the reconnection settings when running
          // against a mock server (no socket.io object is ever created, see
          // the auth handler in `sdkjs/common/docscoapi.js`). The SSE transport
          // reconnection is handled by `EventSource` itself anyway.
          // Note the editics protocol itself keeps the field (the real
          // OnlyOffice server sends it, see the record tests).
          delete oo.settings.reconnection;
        }
        this.editor.sendMessageToOO(oo);
      })
      .catch((e: unknown) => logError('Failed to apply a server event', e));
  }
}

// --- open -----------------------------------------------------------------

// The session client of the currently opened document (there is at most one:
// `openDocument` is expected to be called only once, see `openEditor`).
let activeSession: CollaborativeEditicsServer | undefined;

// `openDocument` is expected to be called only once since it loads OnlyOffice
// scripts and set global variables (e.g. `window.APP`, `window.ooDocEditor`).
async function openDocument(options: EditicsOpenOptions, documentContent: Uint8Array): Promise<void> {
  await openEditor(options, documentContent, async (editor) => {
    const session = new CollaborativeEditicsServer(editor, options);
    // Start the SSE stream before the editor boots (it takes several seconds
    // to load) so the stream is ready when the editor sends its `auth` event.
    await session.start();
    activeSession = session;
    return session;
  });
}

startHostPage({
  onOpen: openDocument,
  onMessage: (message: EditicsParentToHostMessage): void => {
    if (message.command === 'oo-get-user-name-result') {
      activeSession?.handleUserNameResult(message.requestId, message.userName);
    }
  },
});

// When the page goes away (e.g. the user leaves the file editor), notify the
// server so the other participants are told we left (`unLockDocument` +
// `close`, see RFC 1030 §2.2). The SSE stream is closed by the browser
// itself, this is about releasing the locks and the participant slot.
window.addEventListener('pagehide', () => {
  activeSession?.close();
  activeSession = undefined;
});
