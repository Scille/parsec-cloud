// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

// Collaborative edition mode host page.
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
//     GET  {serverUrl}/authenticated/{organizationId}/editics/{realmId}/{documentId}/listen?psid={participantSessionId}
//     POST {serverUrl}/authenticated/{organizationId}/editics/{realmId}/{documentId}/send?psid={participantSessionId}
//
//     GET  {serverUrl}/authenticated/{organizationId}/editics/{realmId}/{documentId}/initial_content
//     POST {serverUrl}/authenticated/{organizationId}/editics/{realmId}/{documentId}/initial_content?psid={participantSessionId}
//
// This file is browser-only (uses SSE over `fetch` & `EventSource`-like parsing).

import type OO from 'onlyoffice-editor';
import { EDITICS_API_VERSION, EditicsTranslator } from './collaborative_protocol';
import type { EditicsDocumentLoader } from './common';
import { openEditor, postToParent, setCollaborativeSessionTracking, startHostPage } from './common';
import type { EditicsCollaborativeSession, EditicsDocumentTypes, EditicsOpenOptions, EditicsParentToHostMessage } from './parent_host_api';

// OnlyOffice document type -> editics protocol `editorType`
// (0=Word, 1=Spreadsheet, 2=Presentation, 3=Visio).
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

// The SSE stream is re-opened with this delay when it drops (see `sseLoop`):
// the server keeps the participant slot and its state on a transport
// disconnection, so the stream can be restored transparently.
const SSE_RECONNECT_DELAY_MS = 2000;

// When the `auth` reply travels on the HTTP POST response (a fresh join not
// parked behind another editor's auth lock), the changes backlog pushed by
// the server *before* replying travels on the SSE stream instead: the two
// connections carry no ordering guarantee. The OnlyOffice editor buffers the
// backlog and only applies it when the `auth` event completes its handshake
// (see `_onAuth`/`_updateAuthChanges` in `sdkjs/common/docscoapi.js`), so the
// backlog must reach it *first*: we give it this grace period to arrive
// before delivering the auth reply.
const AUTH_REPLY_GRACE_MS = 250;

// Time we give the parent to answer an original-content crypto request
// (encrypt/decrypt) before giving up: sealing a big converted document with
// the workspace key can take a while, but the document opening should not be
// blocked forever either.
const CONTENT_CRYPTO_TIMEOUT_MS = 30000;

// Time we give the parent to answer a session-payload crypto request
// (encrypt/decrypt of a change fragment, cursor or message). The crypto is
// purely local so this is only a safety net against a stuck parent.
const CHANGE_CRYPTO_TIMEOUT_MS = 10000;

// The initial content is exchanged as a "bundle" (see
// `server/parsec/asgi/editics.py`):
//
//     bundle = <key_index as u32 big endian> + <part 0> + <part 1> + ...
//     part = <part size as u32 big endian> + <part ciphertext>
//
// The parts are opaque to the server: our libparsec encrypt/decrypt pair (see
// `oo-encrypt-original-content`/`oo-decrypt-original-content` below) uses a
// single archive blob, sent as a single part.
const INITIAL_CONTENT_MEDIA_TYPE = 'application/vnd.parsec.bundled-initial-content';

// The session payloads are freshly allocated buffers (from `TextEncoder.encode`
// or `Uint8Array.fromBase64`), so they can be transferred to the parent
// instead of being copied by the structured clone. The ownership check keeps
// us safe should a future caller pass a view on a shared buffer (transferring
// it would detach the other views' data); empty buffers are left to the
// structured clone, which copies nothing for them.
function transferBuffer(data: Uint8Array): Transferable[] {
  return data.byteOffset === 0 && data.byteLength > 0 && data.byteLength === data.buffer.byteLength && data.buffer instanceof ArrayBuffer
    ? [data.buffer]
    : [];
}

// The batch crypto requests carry their payloads as a single flat buffer (the
// i-th payload is `flat[offsets[i]..offsets[i+1]]`), so the whole batch crosses
// the iframe boundary as one buffer instead of one per payload.
function flattenBuffers(payloads: Uint8Array[]): { flat: Uint8Array; offsets: number[] } {
  const offsets = [0];
  for (const payload of payloads) {
    offsets.push(offsets[offsets.length - 1] + payload.byteLength);
  }
  const flat = new Uint8Array(offsets[offsets.length - 1]);
  let pos = 0;
  for (const payload of payloads) {
    flat.set(payload, pos);
    pos += payload.byteLength;
  }
  return { flat, offsets };
}

// A raw editics server event, as serialized on the wire (see
// `server/parsec/editics_protocol.py`: the encrypted fields are base64-encoded
// strings, which is exactly what the translator expects).
type RawServerEvent = { type?: string } & Record<string, unknown>;

// The subset of the editor api's co-authoring wrapper needed to signal the
// transport state to the editor (see `notifyEditorOfTransportDrop`): the
// OnlyOffice typings only expose a fraction of the editor api, hence the
// loose shapes.
interface EditorCoAuthoringApi {
  _CoAuthoringApi?: {
    _onServerClose?: (explicit: boolean) => void;
    socketio?: { onopen?: () => void };
    isCloseCoAuthoring?: boolean;
  };
}

// A parent reply to an original-content crypto request (see
// `parent_host_api.ts`): the payload fields on success, the `error` string on
// failure.
type ContentCryptoReply = { error?: string } & Record<string, unknown>;

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
  // Loads the document into the editor (see `EditicsDocumentLoader`): the
  // session drives the load through it (fetch-or-convert-and-upload)
  // instead of `openEditor` doing it right away.
  private loader: EditicsDocumentLoader;
  // Arbitrary UUID chosen client-side: identifies this edition session
  // participation towards the server (it survives SSE reconnections, see
  // `server/parsec/components/editics.py`).
  private participantId: string = crypto.randomUUID().replace(/-/g, '');
  private translator: EditicsTranslator;
  // The SSE stream is implemented with `fetch` (the `EventSource` API cannot
  // set the `Authorization`/`Api-Version` headers the server expects): the
  // controller aborts the in-flight request, and the loop flag stops the
  // reconnection attempts.
  private sseAbort: AbortController | undefined = undefined;
  private closed = false;

  // Where the document currently loaded (or being loaded) into the editor
  // comes from: `null` until a document is on its way, 'local' for our own x2t
  // conversion (a viewer before the first editor's upload), 'server' for the
  // session's initial content. A viewer holding a local
  // conversion must reload the host page when the content gets uploaded
  // (`parsecInitialContentUploaded`), and so does anyone whose auth says the
  // content is uploaded while holding a local conversion (the restore case).
  private documentSource: 'local' | 'server' | null = null;
  // The initial-content flow runs at most once per host page (the document
  // is loaded exactly once): a parked newcomer completing its handshake
  // receives a second `auth` server event, which must not restart it.
  private contentFlowStarted = false;
  // The local conversion kept around while our upload is in flight: it is
  // loaded into the editor if the upload wins (first-upload-wins), abandoned
  // if the session already has a content.
  private pendingLocalContent: { bin: Uint8Array; medias: Array<[string, Uint8Array]> } | undefined;

  // Pending original-content crypto requests towards the parent (the parent
  // is the only side with access to libparsec, hence the one able to seal the
  // content with the workspace key). A reply carries the
  // payload fields on success, the `error` string on failure.
  private pendingContentRequests = new Map<number, (result: ContentCryptoReply) => void>();
  private contentRequestCounter = 0;
  // Pending session-payload crypto requests towards the parent (change
  // fragments, cursors, messages): same pending map/reply convention as the
  // original-content crypto requests above.
  private pendingChangeRequests = new Map<number, (result: ContentCryptoReply) => void>();
  private changeRequestCounter = 0;
  // Whether the session has changes that are not persisted in the workspace
  // file yet (set when changes are exchanged with the session, cleared when a
  // manual save persists the document, see `onSaveRequest` in `common.ts`).
  private unpersistedChanges = false;

  // Whether the workspace file is missing some of the session's changes: the
  // collaborative mode persists it only on a manual save (the editor's own
  // "modified" state is not a valid indicator, as the changes are flushed to
  // the session continuously).
  hasUnpersistedChanges(): boolean {
    return this.unpersistedChanges;
  }

  markChangesPersisted(): void {
    this.unpersistedChanges = false;
  }

  // Server events received on the SSE stream are held back until the editor's
  // `auth` handshake has been delivered: the real OnlyOffice server never
  // sends anything before the `auth` reply, and the editor's state machine is
  // not expected to handle e.g. a `connectState` while still waiting for it.
  // (Our server does broadcast `connectState` to the parked participants.)
  // The changes backlog (`authChanges`) is the exception, see `onServerEvent`.
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
  // participant's device ID into a display name).
  private pendingNameRequests = new Map<number, (userName: string | undefined) => void>();
  private nameRequestCounter = 0;

  constructor(editor: OO.DocEditor, options: EditicsOpenOptions, loader: EditicsDocumentLoader) {
    if (options.collaborative === undefined) {
      // Defense in depth: the parent is expected to serve the offline host
      // page when no collaborative session is provided (see
      // `src/services/editics.ts`).
      throw new Error('Missing collaborative session info');
    }
    this.editor = editor;
    this.options = options;
    this.session = options.collaborative;
    this.loader = loader;

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
        getHumanLabelFromDeviceId: async (deviceIdHex: string) => {
          const userName = await this.resolveUserName(deviceIdHex);
          return userName !== undefined
            ? { ok: true, value: userName }
            : { ok: false, kind: 'libparsec', error: `Cannot resolve the device ID ${deviceIdHex}` };
        },
        // The session payloads (change fragments, cursors, messages) are
        // sealed with the workspace's latest realm key by the parent (the
        // only side with libparsec access): the editics server only ever sees
        // the opaque ciphertext, and the key index travels along the wire so
        // the other participants can resolve the key on their side.
        encryptBatch: async (cleartexts: Uint8Array[]): Promise<[number, Uint8Array][]> => {
          return await this.requestParentEncryptChanges(cleartexts);
        },
        decryptBatch: async (items: [string, number, Uint8Array][]) => {
          try {
            const cleartexts = await this.requestParentDecryptChanges(items);
            return { ok: true as const, value: cleartexts };
          } catch (e: unknown) {
            return { ok: false as const, kind: 'libparsec' as const, error: String(e) };
          }
        },
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
        const result = await this.translator.cookClientEvent(msg);
        if (!result.ok) {
          logError(`Failed to translate the editor '${msg.type}' event`, result.error);
          return;
        }
        // `null` when the event has no editics counterpart (e.g. `rpc`,
        // `clientLog`) or is not supported yet (e.g. `getLock`, `close`).
        const cooked = result.value;
        if (cooked === null || cooked === undefined) {
          return;
        }
        if (cooked.type === 'saveChanges') {
          this.unpersistedChanges = true;
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
      let settled = false;
      const done = (): void => {
        if (!settled) {
          settled = true;
          resolve();
        }
      };
      // The stream is (re)opened by the loop below: resolve as soon as the
      // first connection is established, the loop keeps it alive afterwards.
      this.sseAbort = new AbortController();
      this.sseAbort.signal.addEventListener('abort', done, { once: true });
      this.sseLoop(done, this.sseAbort.signal).catch((e: unknown) => logError('SSE loop crashed', e));
      setTimeout(done, SSE_OPEN_TIMEOUT_MS);
    });
  }

  // Keeps the SSE stream alive: the transport-level connection is restored
  // transparently when it drops (the server keeps the participant slot and
  // its state, see `participant_listen` in `server/parsec/components/editics.py`).
  // Note the events broadcast while disconnected are lost: the editics
  // protocol relies on OnlyOffice's own resynchronization logic.
  //
  // A transport reconnection alone does not re-auth the participant: the
  // server only restores it when the editor re-sends its `auth` event with
  // reconnection info (the `participantSessionId`/`lastOtherSaveTime` of
  // the first handshake, see the `reconnect` translation in
  // `collaborative_protocol.js`). Hence each transport drop/reopen is
  // signaled to the editor (`notifyEditorOfTransportDrop`/
  // `notifyEditorOfTransportReopen` below), which then replays the very
  // reconnection flow it runs against the real OnlyOffice server: the
  // editor switches to its "reconnect" state (buffering what it was doing,
  // e.g. flagging an interrupted save for a re-send), re-sends `auth`, and
  // resumes once the `auth` reply arrives on the reconnected transport.
  private async sseLoop(onFirstOpen: () => void, signal: AbortSignal): Promise<void> {
    let first = true;
    while (!signal.aborted && !this.closed) {
      const controller = new AbortController();
      const onAbort = (): void => controller.abort();
      signal.addEventListener('abort', onAbort, { once: true });
      let streamEstablished = false;
      try {
        const rep = await fetch(this.listenUrl, {
          headers: { ...this.baseHeaders, Accept: 'text/event-stream' },
          signal: controller.signal,
        });
        if (!rep.ok) {
          logError(`The editics server refused the SSE connection: HTTP ${rep.status}`);
        } else {
          if (first) {
            first = false;
            onFirstOpen();
          } else {
            this.notifyEditorOfTransportReopen();
          }
          streamEstablished = true;
          await this.consumeSseStream(rep, controller.signal);
        }
      } catch (e: unknown) {
        if (!controller.signal.aborted) {
          logError('SSE stream error', e);
        }
      } finally {
        signal.removeEventListener('abort', onAbort);
        if (streamEstablished && !signal.aborted && !this.closed) {
          this.notifyEditorOfTransportDrop();
        }
      }
      if (signal.aborted || this.closed) {
        return;
      }
      await new Promise((resolve) => setTimeout(resolve, SSE_RECONNECT_DELAY_MS));
    }
  }

  // --- Transport drop/reopen signaling towards the editor -----------------
  //
  // The fork's editor expects its "socketio" transport to drive its
  // reconnection flow (see `_initSocksJs`/`_onServerOpen`/`_onServerClose` in
  // `sdkjs/common/docscoapi.js`): the transport calls `socketio.onopen` when
  // the connection is (re)established and `_onServerClose(false)` when it
  // dies without a close frame. Our SSE loop *is* the transport, so it does
  // the driving here, reaching the editor's co-authoring protocol handler
  // through its api (`api.CoAuthoringApi._CoAuthoringApi`, the
  // `CDocsCoApi`/`DocsCoApi` pair).

  // Returns the editor's co-authoring protocol handler, or undefined when the
  // transport state must not be signaled to it: either the editor hasn't
  // booted far enough to expose it (the signaling is best-effort, the
  // reconnection then just doesn't happen and the user retries), or it has
  // deliberately closed the co-authoring session (e.g. after a `drop`/`error`
  // event, like the "document modified" restore refusal) — reconnecting the
  // transport must not resurrect it.
  private editorDocsCoApi(): EditorCoAuthoringApi['_CoAuthoringApi'] | undefined {
    const api = this.editor.getApi() as (OO.OOApi & { CoAuthoringApi?: EditorCoAuthoringApi }) | null;
    const docsCoApi = api?.CoAuthoringApi?._CoAuthoringApi;
    if (docsCoApi === undefined || typeof docsCoApi._onServerClose !== 'function' || typeof docsCoApi.socketio?.onopen !== 'function') {
      logError('Cannot reach the editor co-authoring handler to signal the transport state');
      return undefined;
    }
    if (docsCoApi.isCloseCoAuthoring === true) {
      return undefined;
    }
    return docsCoApi;
  }

  // The transport died without a close frame: the editor switches to its
  // "reconnect" state, notably buffering the events it emits from now on
  // (nothing is sent into the void) and flagging an interrupted save for a
  // re-send after the re-auth.
  private notifyEditorOfTransportDrop(): void {
    this.editorDocsCoApi()?._onServerClose?.(false);
  }

  // The transport is back: the editor re-runs its "first connect" flow,
  // which re-sends the `auth` event (carrying reconnection info once the
  // first handshake completed) and resumes (flushing what it buffered while
  // disconnected) when the `auth` reply arrives.
  private notifyEditorOfTransportReopen(): void {
    this.editorDocsCoApi()?.socketio?.onopen?.();
  }

  // Parses the SSE wire format (`data: <payload>\n\n`, see the SSE spec):
  // only the `data` fields are used here, the (keepalive) comments and the
  // other fields are ignored.
  private async consumeSseStream(rep: Response, signal: AbortSignal): Promise<void> {
    if (rep.body === null) {
      return;
    }
    const reader = rep.body.getReader();
    const decoder = new TextDecoder();
    let buffer = '';
    const handleEvent = (data: string): void => {
      if (data.length === 0) {
        return;
      }
      let event: RawServerEvent | undefined;
      try {
        event = JSON.parse(data) as RawServerEvent;
      } catch (e) {
        logError('Received a non-JSON server event', e);
        return;
      }
      this.onServerEvent(event, 'sse');
    };
    try {
      for (;;) {
        const { done, value } = await reader.read();
        if (done) {
          break;
        }
        buffer += decoder.decode(value, { stream: true });
        for (;;) {
          const separator = buffer.indexOf('\n\n');
          if (separator === -1) {
            break;
          }
          const rawEvent = buffer.slice(0, separator);
          buffer = buffer.slice(separator + 2);
          const data = rawEvent
            .split('\n')
            .filter((line) => line.startsWith('data:'))
            .map((line) => line.slice(5).trimStart())
            .join('\n');
          handleEvent(data);
        }
      }
    } finally {
      reader.releaseLock();
      if (!signal.aborted) {
        // The stream ended on its own (e.g. the server dropped the
        // connection): the loop will reopen it.
        logError('SSE stream ended');
      }
    }
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
    this.sseAbort?.abort();
    this.sseAbort = undefined;
    // Don't leave the name resolutions hanging forever.
    for (const resolve of this.pendingNameRequests.values()) {
      resolve(undefined);
    }
    this.pendingNameRequests.clear();
    // Ditto for the crypto requests (initial content & session payloads).
    for (const resolve of this.pendingContentRequests.values()) {
      resolve({ error: 'The session was closed' });
    }
    this.pendingContentRequests.clear();
    for (const resolve of this.pendingChangeRequests.values()) {
      resolve({ error: 'The session was closed' });
    }
    this.pendingChangeRequests.clear();
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

  // --- Initial content -----------------------------------------------------

  // Called by the host page message dispatch (see `startHostPage`) when the
  // parent replies to an `oo-encrypt-original-content` request.
  handleEncryptOriginalContentResult(requestId: number, result: { keyIndex?: number; parts?: Uint8Array[]; error?: string }): void {
    this.resolveContentRequest(requestId, result);
  }

  // Ditto for `oo-decrypt-original-content`.
  handleDecryptOriginalContentResult(
    requestId: number,
    result: { bin?: Uint8Array; medias?: Array<[string, Uint8Array]>; error?: string },
  ): void {
    this.resolveContentRequest(requestId, result);
  }

  // Fulfills (or rejects) a pending crypto request by id, with the success
  // payload or the parent's `error` string.
  private resolveContentRequest(requestId: number, result: ContentCryptoReply): void {
    const resolve = this.pendingContentRequests.get(requestId);
    if (resolve) {
      this.pendingContentRequests.delete(requestId);
      resolve(result);
    }
  }

  // Ask the parent to seal the initial content files with the workspace key
  // (the parent is the only side with libparsec access): one encrypted part
  // per file, part 0 being the document `.bin` (see the bundle wire format
  // in `server/parsec/asgi/editics.py`).
  private requestParentEncrypt(bin: Uint8Array, medias: Array<[string, Uint8Array]>): Promise<{ keyIndex: number; parts: Uint8Array[] }> {
    return new Promise((resolve, reject) => {
      const requestId = ++this.contentRequestCounter;
      const timer = setTimeout(() => {
        this.pendingContentRequests.delete(requestId);
        reject(new Error('The parent did not answer the initial content encryption request in time'));
      }, CONTENT_CRYPTO_TIMEOUT_MS);
      this.pendingContentRequests.set(requestId, (result) => {
        clearTimeout(timer);
        const { keyIndex, parts } = result;
        if (result.error !== undefined || keyIndex === undefined || parts === undefined) {
          reject(new Error(result.error ?? 'The parent failed to encrypt the initial content'));
        } else {
          resolve({ keyIndex: keyIndex as number, parts: parts as Uint8Array[] });
        }
      });
      postToParent({ command: 'oo-encrypt-original-content', requestId, bin, medias });
    });
  }

  // Ditto to open the content parts fetched from the session.
  private requestParentDecrypt(keyIndex: number, parts: Uint8Array[]): Promise<{ bin: Uint8Array; medias: Array<[string, Uint8Array]> }> {
    return new Promise((resolve, reject) => {
      const requestId = ++this.contentRequestCounter;
      const timer = setTimeout(() => {
        this.pendingContentRequests.delete(requestId);
        reject(new Error('The parent did not answer the initial content decryption request in time'));
      }, CONTENT_CRYPTO_TIMEOUT_MS);
      this.pendingContentRequests.set(requestId, (result) => {
        clearTimeout(timer);
        const { bin, medias } = result;
        if (result.error !== undefined || bin === undefined || medias === undefined) {
          reject(new Error(result.error ?? 'The parent failed to decrypt the initial content'));
        } else {
          resolve({
            bin: bin as Uint8Array,
            medias: medias as Array<[string, Uint8Array]>,
          });
        }
      });
      postToParent({ command: 'oo-decrypt-original-content', requestId, keyIndex, parts });
    });
  }

  // --- Session payload crypto (changes, cursors, messages) -----------------

  // Called by the host page message dispatch (see `startHostPage`) when the
  // parent replies to an `oo-encrypt-batch` request.
  handleEncryptBatchResult(
    requestId: number,
    result: { keyIndexes?: number[]; flat?: Uint8Array; offsets?: number[]; error?: string },
  ): void {
    this.resolveChangeRequest(requestId, result);
  }

  // Ditto for `oo-decrypt-batch`.
  handleDecryptBatchResult(requestId: number, result: { flat?: Uint8Array; offsets?: number[]; error?: string }): void {
    this.resolveChangeRequest(requestId, result);
  }

  // Fulfills (or rejects) a pending session-payload crypto request by id, with
  // the success payload or the parent's `error` string.
  private resolveChangeRequest(requestId: number, result: ContentCryptoReply): void {
    const resolve = this.pendingChangeRequests.get(requestId);
    if (resolve) {
      this.pendingChangeRequests.delete(requestId);
      resolve(result);
    }
  }

  // Ask the parent to seal session payloads with the workspace's latest realm
  // key (the parent is the only side with libparsec access): the payloads of
  // the whole batch travel as a single flat buffer (see `flattenBuffers`).
  private requestParentEncryptChanges(cleartexts: Uint8Array[]): Promise<[number, Uint8Array][]> {
    return new Promise((resolve, reject) => {
      const requestId = ++this.changeRequestCounter;
      const timer = setTimeout(() => {
        this.pendingChangeRequests.delete(requestId);
        reject(new Error('The parent did not answer the encryption request in time'));
      }, CHANGE_CRYPTO_TIMEOUT_MS);
      this.pendingChangeRequests.set(requestId, (result) => {
        clearTimeout(timer);
        const keyIndexes = result.keyIndexes as number[] | undefined;
        const flat = result.flat as Uint8Array | undefined;
        const offsets = result.offsets as number[] | undefined;
        if (result.error !== undefined || keyIndexes === undefined || flat === undefined || offsets === undefined) {
          reject(new Error(result.error ?? 'The parent failed to encrypt the payloads'));
        } else {
          resolve(keyIndexes.map((keyIndex, i) => [keyIndex, flat.subarray(offsets[i], offsets[i + 1])] as [number, Uint8Array]));
        }
      });
      const { flat, offsets } = flattenBuffers(cleartexts);
      postToParent({ command: 'oo-encrypt-batch', requestId, flat, offsets }, transferBuffer(flat));
    });
  }

  // Ditto to open the session payloads received from the session. The parent
  // resolves the key of each payload by index, the author device ID is not
  // needed on its side.
  private requestParentDecryptChanges(items: [string, number, Uint8Array][]): Promise<Uint8Array[]> {
    return new Promise((resolve, reject) => {
      const requestId = ++this.changeRequestCounter;
      const timer = setTimeout(() => {
        this.pendingChangeRequests.delete(requestId);
        reject(new Error('The parent did not answer the decryption request in time'));
      }, CHANGE_CRYPTO_TIMEOUT_MS);
      this.pendingChangeRequests.set(requestId, (result) => {
        clearTimeout(timer);
        const flat = result.flat as Uint8Array | undefined;
        const offsets = result.offsets as number[] | undefined;
        if (result.error !== undefined || flat === undefined || offsets === undefined) {
          reject(new Error(result.error ?? 'The parent failed to decrypt the payloads'));
        } else {
          resolve(offsets.slice(0, -1).map((offset, i) => flat.subarray(offset, offsets[i + 1])));
        }
      });
      const keyIndexes = items.map(([_authorDeviceId, keyIndex]) => keyIndex);
      const { flat, offsets } = flattenBuffers(items.map(([_authorDeviceId, _keyIndex, ciphertext]) => ciphertext));
      postToParent({ command: 'oo-decrypt-batch', requestId, keyIndexes, flat, offsets }, transferBuffer(flat));
    });
  }

  // Drive the document load once the `auth` handshake completes (chained
  // from the `auth` handling in `onServerEvent`):
  // - the session already has a content → fetch it and load it (no x2t
  //   conversion at all);
  // - no content yet → run the local conversion; an editor then uploads its
  //   result (so the session always edits against a single canonical base)
  //   and loads it if the upload wins, while a viewer loads it and waits for
  //   the `parsecInitialContentUploaded` broadcast.
  private handleAuthReply(event: RawServerEvent): void {
    if (this.closed) {
      return;
    }
    if (event.initialContentUploaded === true && this.documentSource === 'local') {
      // The restore case: we display our own local conversion
      // but the session now has a content — we were disconnected when the
      // upload happened and missed the `parsecInitialContentUploaded`
      // broadcast. The whole host page reboots: fresh participant, fresh
      // auth → fetch → load.
      window.location.reload();
      return;
    }
    if (this.contentFlowStarted) {
      // The flow runs at most once per host page: a parked newcomer completing
      // its handshake receives a second `auth` server event, and a restored
      // connection re-replies too: the document is already on its way.
      return;
    }
    this.contentFlowStarted = true;
    void this.runContentFlow(event).catch((e: unknown) => {
      logError('Failed to load the session document', e);
      postToParent({
        command: 'oo-error',
        details: JSON.stringify(e instanceof Error ? e : String(e)),
      });
    });
  }

  private async runContentFlow(authEvent: RawServerEvent): Promise<void> {
    if (authEvent.initialContentUploaded === true) {
      await this.fetchSessionContent();
      return;
    }

    // The session has no content yet: run our own x2t conversion.
    const { bin, medias } = await this.loader.convertLocally();
    if (this.options.mode === 'edit') {
      // Upload the conversion *before* loading it into the editor: the server
      // refuses document modifications while the session has no initial
      // content, and the editor cannot produce changes before
      // `loadBinary` anyway.
      this.pendingLocalContent = { bin, medias };
      const { keyIndex, parts } = await this.requestParentEncrypt(bin, medias);
      await this.uploadSessionContent(keyIndex, parts);
    } else {
      // Viewer: load our conversion and wait for the
      // `parsecInitialContentUploaded` broadcast (the whole host page
      // reloads when it arrives).
      this.documentSource = 'local';
      await this.loader.loadDocument(bin, medias);
    }
  }

  // Uploads our local conversion as the session's initial content (see the
  // bundle wire format in `server/parsec/asgi/editics.py`): one part per
  // encrypted file — part 0 the document `.bin`, then the media files — so
  // a fetcher can start loading the document before its media parts arrive.
  // The first upload wins, a concurrent one is rejected with `486
  // InitialContentAlreadyUploaded` in which case the winner's content is
  // fetched instead.
  private async uploadSessionContent(keyIndex: number, parts: Uint8Array[]): Promise<void> {
    let total = 4;
    for (const part of parts) {
      total += 4 + part.byteLength;
    }
    const body = new Uint8Array(total);
    const view = new DataView(body.buffer);
    let offset = 0;
    view.setUint32(offset, keyIndex);
    offset += 4;
    for (const part of parts) {
      view.setUint32(offset, part.byteLength);
      offset += 4;
      body.set(part, offset);
      offset += part.byteLength;
    }

    let rep: Response;
    try {
      rep = await fetch(this.initialContentUrl, {
        method: 'POST',
        headers: {
          ...this.baseHeaders,
          'Content-Type': INITIAL_CONTENT_MEDIA_TYPE,
        },
        body,
      });
    } catch (e) {
      throw new Error(`Failed to upload the initial content: ${String(e)}`);
    }
    if (rep.status === 204) {
      // We won the first-upload-wins race: our local conversion *is* the
      // session's content.
      const local = this.pendingLocalContent;
      this.pendingLocalContent = undefined;
      if (local !== undefined) {
        this.documentSource = 'server';
        await this.loader.loadDocument(local.bin, local.medias);
      }
      return;
    }
    // A concurrent upload won (486): abandon ours and fetch the winner's.
    this.pendingLocalContent = undefined;
    if (rep.status !== 486) {
      throw new Error(`The editics server rejected the initial content upload: HTTP ${rep.status}`);
    }
    await this.fetchSessionContent();
  }

  // Fetches the session's initial content: the bundle is unwrapped, the
  // parts are decrypted by the parent (the only side with libparsec access)
  // and loaded into the editor.
  private async fetchSessionContent(): Promise<void> {
    let rep: Response;
    try {
      rep = await fetch(this.initialContentUrl, {
        headers: this.baseHeaders,
      });
    } catch (e) {
      throw new Error(`Failed to fetch the initial content: ${String(e)}`);
    }
    if (rep.status === 485) {
      // `InitialContentMissing`: unexpected (we only fetch after an auth
      // saying the content is uploaded), but the session may have been
      // restarted server-side: the user retries with a fresh page.
      throw new Error('The session has no initial content yet');
    }
    if (!rep.ok) {
      throw new Error(`The editics server rejected the initial content fetch: HTTP ${rep.status}`);
    }
    const bundle = new Uint8Array(await rep.arrayBuffer());
    if (bundle.byteLength < 8) {
      throw new Error('Truncated initial content bundle');
    }
    const view = new DataView(bundle.buffer, bundle.byteOffset, bundle.byteLength);
    const keyIndex = view.getUint32(0);
    // One part per encrypted file (see `uploadSessionContent`): part 0 is
    // the document `.bin`, the following ones the media files.
    const parts: Uint8Array[] = [];
    let offset = 4;
    while (offset < bundle.byteLength) {
      if (offset + 4 > bundle.byteLength) {
        throw new Error('Truncated initial content bundle part header');
      }
      const partSize = view.getUint32(offset);
      offset += 4;
      if (offset + partSize > bundle.byteLength) {
        throw new Error('Truncated initial content bundle part');
      }
      parts.push(bundle.slice(offset, offset + partSize));
      offset += partSize;
    }
    if (parts.length === 0) {
      throw new Error('The initial content bundle has no part');
    }
    const { bin, medias } = await this.requestParentDecrypt(keyIndex, parts);
    this.documentSource = 'server';
    await this.loader.loadDocument(bin, medias);
  }

  // --- Editics session HTTP requests ----------------------------------------

  private get baseHeaders(): Record<string, string> {
    return {
      Authorization: `Bearer ${this.session.token}`,
      'Api-Version': EDITICS_API_VERSION,
    };
  }

  private get sessionBasePath(): string {
    const { serverUrl, organizationId, workspaceId, vlobId } = this.session;
    return `${serverUrl}/authenticated/${encodeURIComponent(organizationId)}/editics/${workspaceId}/${vlobId}`;
  }

  private get listenUrl(): string {
    return `${this.sessionBasePath}/listen?psid=${this.participantId}`;
  }

  private get sendUrl(): string {
    return `${this.sessionBasePath}/send?psid=${this.participantId}`;
  }

  private get initialContentUrl(): string {
    return `${this.sessionBasePath}/initial_content?psid=${this.participantId}`;
  }

  // Sends an editics client event to the server. The reply (a single server
  // event addressed to us, or 204 no content) is processed like an SSE event.
  // Note the event may carry more fields than its type (the wire schema, see
  // `server/parsec/editics_protocol.py`).
  private async postClientEvent(event: { type: string } & Record<string, unknown>, opts: { keepalive?: boolean } = {}): Promise<void> {
    let rep: Response;
    try {
      rep = await fetch(this.sendUrl, {
        method: 'POST',
        headers: {
          ...this.baseHeaders,
          'Content-Type': 'application/json',
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
    if (event.type === 'parsecInitialContentUploaded') {
      // The session's initial content just became available: only a
      // participant that opened the document from its own local x2t
      // conversion must reload it (editors are either the uploader, or get
      // `486 InitialContentAlreadyUploaded` on their own upload attempt).
      // The reload reboots the whole host page: fresh participant, fresh
      // auth → fetch.
      if (this.documentSource === 'local') {
        window.location.reload();
      }
      return;
    }
    if (event.type === 'saveChanges' || event.type === 'authChanges') {
      // Other participants' changes (live broadcasts, or the backlog we caught up
      // on when joining) also need to be persisted in the workspace file at the
      // next manual save.
      this.unpersistedChanges = true;
    }
    if (event.type === 'authChanges') {
      // The changes backlog is always delivered before the `auth` reply (the
      // OnlyOffice protocol delivers it first too, see the `auth` handling in
      // `DocsCoServer.js`): the editor buffers it and applies it when the auth
      // completes its handshake. So, unlike the other events received before
      // the handshake, it is delivered right away: holding it back would leave
      // it stuck after the auth, never applied (see `heldServerEvents`).
      this.enqueueDelivery(event);
      return;
    }
    if (event.type === 'auth') {
      // The `auth` handshake completes: deliver it, then release the events
      // that were held back while the editor was waiting for it.
      const deliverAuth = (): void => {
        if (this.closed) {
          // The session was closed during the grace period (see below): nothing
          // left to deliver to the (gone) editor.
          return;
        }
        this.authDelivered = true;
        this.enqueueDelivery(event);
        for (const held of this.heldServerEvents.splice(0)) {
          this.enqueueDelivery(held);
        }
      };
      // The handshake carrying the session's initial-content state drives the
      // document load (fetch-or-convert-and-upload). The
      // editor's `auth` handshake timing is unchanged: the editor sends its
      // `auth` during its boot (before `loadBinary` is ever called), which is
      // what makes the upload-before-load invariant hold.
      this.handleAuthReply(event);
      if (source === 'reply') {
        // See `AUTH_REPLY_GRACE_MS`: let the backlog racing on the SSE stream
        // reach the editor first (it was delivered above as soon as it
        // arrived). Note `authDelivered` stays false during the grace period
        // so the events received in the meantime keep being held back.
        setTimeout(deliverAuth, AUTH_REPLY_GRACE_MS);
      } else {
        // The server pushes the backlog and the auth on the SSE stream in this
        // order (see the auth lock release handling in
        // `server/parsec/components/editics.py`), and the deliveries are
        // chained (see `enqueueDelivery`), so the ordering is already right.
        deliverAuth();
      }
      return;
    }
    if (source === 'sse' && !this.authDelivered) {
      // See `heldServerEvents`: nothing but the handshake events (`auth`,
      // `authChanges`) is expected before the handshake completes (e.g. a
      // `connectState` broadcast while we are parked behind another editor's
      // auth lock).
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
        const result = await this.translator.cookServerEvent(event as never);
        if (!result.ok) {
          logError(`Failed to translate the '${event.type}' server event`, result.error);
          return;
        }
        const oo = result.value;
        if (oo.type === 'auth' && oo.settings?.reconnection !== undefined) {
          // Adaptation for the CryptPad fork's editor: it expects engine.io/
          // socket.io and crashes on the reconnection settings when running
          // against a mock server (no socket.io object is ever created, see
          // the auth handler in `sdkjs/common/docscoapi.js`). The SSE transport
          // reconnection is handled by `sseLoop` itself anyway.
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
  await openEditor(options, documentContent, async (editor, loader) => {
    const session = new CollaborativeEditicsServer(editor, options, loader);
    // Start the SSE stream before the editor boots (it takes several seconds
    // to load) so the stream is ready when the editor sends its `auth` event.
    await session.start();
    activeSession = session;
    // Lets the shared save flow know when the workspace file is missing some of
    // the session's changes (see `onSaveRequest` in `common.ts`).
    setCollaborativeSessionTracking(session);
    return session;
  });
}

startHostPage({
  onOpen: openDocument,
  onMessage: (message: EditicsParentToHostMessage): void => {
    switch (message.command) {
      case 'oo-get-user-name-result':
        activeSession?.handleUserNameResult(message.requestId, message.userName);
        break;
      case 'oo-encrypt-original-content-result':
        activeSession?.handleEncryptOriginalContentResult(message.requestId, message);
        break;
      case 'oo-decrypt-original-content-result':
        activeSession?.handleDecryptOriginalContentResult(message.requestId, message);
        break;
      case 'oo-encrypt-batch-result':
        activeSession?.handleEncryptBatchResult(message.requestId, message);
        break;
      case 'oo-decrypt-batch-result':
        activeSession?.handleDecryptBatchResult(message.requestId, message);
        break;
      default:
        break;
    }
  },
});

// When the page goes away (e.g. the user leaves the file editor), notify the
// server so the other participants are told we left (`unLockDocument` +
// `close`). The SSE stream is closed by the browser
// itself, this is about releasing the locks and the participant slot.
window.addEventListener('pagehide', () => {
  activeSession?.close();
  activeSession = undefined;
});
