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
import type { EditicsDocumentLoader } from './common';
import { openEditor, postToParent, setCollaborativeSessionTracking, startHostPage } from './common';
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

// A raw editics server event, as serialized on the wire (see
// `server/parsec/editics_protocol.py`: the encrypted fields are base64-encoded
// strings, which is exactly what the translator expects).
type RawServerEvent = { type?: string } & Record<string, unknown>;

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
  // session drives the load through it (fetch-or-convert-and-upload, RFC
  // 1030 §1.2) instead of `openEditor` doing it right away.
  private loader: EditicsDocumentLoader;
  // Arbitrary UUID chosen client-side: identifies this edition session
  // participation towards the server (it survives SSE reconnections, see
  // `server/parsec/components/editics.py`).
  private participantId: string = crypto.randomUUID();
  private translator: EditicsTranslator;
  private sse: EventSource | undefined;
  private closed = false;

  // Where the document currently loaded (or being loaded) into the editor
  // comes from: `null` until a document is on its way, 'local' for our own x2t
  // conversion (a viewer before the first editor's upload), 'server' for the
  // session's original content (see RFC 1030 §1.2). A viewer holding a local
  // conversion must reload the host page when the content gets uploaded
  // (`updateVersion`), and so does anyone whose auth says the content is
  // uploaded while holding a local conversion (the restore case).
  private documentSource: 'local' | 'server' | null = null;
  // The original-content flow runs at most once per host page (the document
  // is loaded exactly once): a parked newcomer completing its handshake
  // receives a second `auth` server event, which must not restart it.
  private contentFlowStarted = false;
  // The local conversion kept around while our upload is in flight: it is
  // loaded into the editor if the upload wins (first-upload-wins), abandoned
  // if the session already has a content.
  private pendingLocalContent: { bin: Uint8Array; medias: Array<[string, Uint8Array]> } | undefined;

  // Pending original-content crypto requests towards the parent (the parent
  // is the only side with access to libparsec, hence the one able to seal the
  // content with the workspace key, see RFC 1030 §1.2). A reply carries the
  // payload fields on success, the `error` string on failure.
  private pendingContentRequests = new Map<number, (result: ContentCryptoReply) => void>();
  private contentRequestCounter = 0;
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
  // participant's device ID into a display name, see RFC 1030 §3.3).
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
        if ((cooked as { type: string }).type === 'saveChanges') {
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
    // Ditto for the original-content crypto requests.
    for (const resolve of this.pendingContentRequests.values()) {
      resolve({ error: 'The session was closed' });
    }
    this.pendingContentRequests.clear();
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

  // --- Original content (RFC 1030 §1.2) -----------------------------------

  // Called by the host page message dispatch (see `startHostPage`) when the
  // parent replies to an `oo-encrypt-original-content` request.
  handleEncryptOriginalContentResult(requestId: number, result: { keyIndex?: number; encrypted?: Uint8Array; error?: string }): void {
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

  // Ask the parent to seal the original content archive with the workspace
  // key (the parent is the only side with libparsec access).
  private requestParentEncrypt(bin: Uint8Array, medias: Array<[string, Uint8Array]>): Promise<{ keyIndex: number; encrypted: Uint8Array }> {
    return new Promise((resolve, reject) => {
      const requestId = ++this.contentRequestCounter;
      const timer = setTimeout(() => {
        this.pendingContentRequests.delete(requestId);
        reject(new Error('The parent did not answer the original content encryption request in time'));
      }, CONTENT_CRYPTO_TIMEOUT_MS);
      this.pendingContentRequests.set(requestId, (result) => {
        clearTimeout(timer);
        const { keyIndex, encrypted } = result;
        if (result.error !== undefined || keyIndex === undefined || encrypted === undefined) {
          reject(new Error(result.error ?? 'The parent failed to encrypt the original content'));
        } else {
          resolve({ keyIndex: keyIndex as number, encrypted: encrypted as Uint8Array });
        }
      });
      postToParent({ command: 'oo-encrypt-original-content', requestId, bin, medias });
    });
  }

  // Ditto to open a content archive fetched from the session.
  private requestParentDecrypt(keyIndex: number, encrypted: Uint8Array): Promise<{ bin: Uint8Array; medias: Array<[string, Uint8Array]> }> {
    return new Promise((resolve, reject) => {
      const requestId = ++this.contentRequestCounter;
      const timer = setTimeout(() => {
        this.pendingContentRequests.delete(requestId);
        reject(new Error('The parent did not answer the original content decryption request in time'));
      }, CONTENT_CRYPTO_TIMEOUT_MS);
      this.pendingContentRequests.set(requestId, (result) => {
        clearTimeout(timer);
        const { bin, medias } = result;
        if (result.error !== undefined || bin === undefined || medias === undefined) {
          reject(new Error(result.error ?? 'The parent failed to decrypt the original content'));
        } else {
          resolve({
            bin: bin as Uint8Array,
            medias: medias as Array<[string, Uint8Array]>,
          });
        }
      });
      postToParent({ command: 'oo-decrypt-original-content', requestId, keyIndex, encrypted });
    });
  }

  // Drive the document load once the `auth` handshake completes (chained
  // from the `auth` handling in `onServerEvent`):
  // - the session already has a content → fetch it and load it (no x2t
  //   conversion at all);
  // - no content yet → run the local conversion; an editor then uploads its
  //   result (so the session always edits against a single canonical base)
  //   and loads it if the upload wins, while a viewer loads it and waits for
  //   the `updateVersion` broadcast.
  // See RFC 1030 §1.2.
  private handleAuthReply(event: RawServerEvent): void {
    if (this.closed) {
      return;
    }
    if (event.originalContentUploaded === true && this.documentSource === 'local') {
      // The restore case (RFC 1030 §1.2): we display our own local conversion
      // but the session now has a content — we were disconnected when the
      // upload happened and missed the `updateVersion` broadcast. The whole
      // host page reboots: fresh participant, fresh auth → fetch → load.
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
    if (authEvent.originalContentUploaded === true) {
      await this.fetchSessionContent();
      return;
    }

    // The session has no content yet: run our own x2t conversion.
    const { bin, medias } = await this.loader.convertLocally();
    if (this.options.mode === 'edit') {
      // Upload the conversion *before* loading it into the editor: the server
      // refuses document modifications while the session has no original
      // content (RFC 1030 §1.2), and the editor cannot produce changes before
      // `loadBinary` anyway. The reply is handled in `onServerEvent` below
      // (we won → load our conversion, we lost → fetch the winner's).
      this.pendingLocalContent = { bin, medias };
      const { keyIndex, encrypted } = await this.requestParentEncrypt(bin, medias);
      // Host-initiated posts ride the `sends` chain so they cannot overtake
      // the editor's own events.
      this.sends = this.sends.then(async () => {
        await this.postClientEvent({
          type: 'parsecUploadOriginalContent',
          // Format: (<Key index>, <base64(ciphertext)>), see
          // `server/parsec/editics_protocol.py`.
          encryptedOriginalContent: [keyIndex, encrypted.toBase64()],
        });
      });
      await this.sends;
    } else {
      // Viewer: load our conversion and wait for the `updateVersion`
      // broadcast (the whole host page reloads when it arrives).
      this.documentSource = 'local';
      await this.loader.loadDocument(bin, medias);
    }
  }

  // Fetch the session's original content: the reply (handled in
  // `onServerEvent` like every client event reply) decrypts the archive and
  // loads it into the editor.
  private fetchSessionContent(): Promise<void> {
    this.sends = this.sends.then(async () => {
      await this.postClientEvent({ type: 'parsecFetchOriginalContent' });
    });
    return this.sends;
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
  // Note the event may carry more fields than its type (the wire schema, see
  // `server/parsec/editics_protocol.py`).
  private async postClientEvent(event: { type: string } & Record<string, unknown>, opts: { keepalive?: boolean } = {}): Promise<void> {
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
    // Host-level events never reach the editor (they have no OnlyOffice
    // counterpart): they are intercepted here, before the held-events/
    // translator machinery.
    if (event.type === 'parsecUploadOriginalContent') {
      // Reply to our upload: `alreadyUploaded === false` means we won the
      // first-upload-wins race, so our local conversion *is* the session's
      // content; `true` means a concurrent upload won, so we abandon ours
      // and fetch the winner's content instead.
      if (event.alreadyUploaded === false) {
        const local = this.pendingLocalContent;
        this.pendingLocalContent = undefined;
        if (local !== undefined) {
          this.documentSource = 'server';
          this.loader
            .loadDocument(local.bin, local.medias)
            .catch((e: unknown) => logError('Failed to load the document into the editor', e));
        }
      } else {
        this.pendingLocalContent = undefined;
        this.fetchSessionContent().catch((e: unknown) => logError('Failed to fetch the session content', e));
      }
      return;
    }
    if (event.type === 'parsecFetchOriginalContent') {
      // Reply to our fetch: `null` is unexpected (we only fetch after an
      // auth saying the content is uploaded).
      const encryptedOriginalContent = event.encryptedOriginalContent as [number, string] | null;
      if (encryptedOriginalContent === null) {
        logError('The session has no original content although the auth said otherwise');
        postToParent({ command: 'oo-error', details: 'The session has no original content yet' });
        return;
      }
      const [keyIndex, encryptedB64] = encryptedOriginalContent;
      this.requestParentDecrypt(keyIndex, Uint8Array.fromBase64(encryptedB64))
        .then(async ({ bin, medias }) => {
          this.documentSource = 'server';
          await this.loader.loadDocument(bin, medias);
        })
        .catch((e: unknown) => {
          logError('Failed to load the session content', e);
          postToParent({
            command: 'oo-error',
            details: JSON.stringify(e instanceof Error ? e : String(e)),
          });
        });
      return;
    }
    if (event.type === 'updateVersion') {
      // The session's original content just became available: only a
      // participant that opened the document from its own local x2t
      // conversion must reload it (editors are either the uploader or get
      // `alreadyUploaded: true` on their own upload attempt). The reload
      // reboots the whole host page: fresh participant, fresh auth → fetch.
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
      // The handshake carrying the session's original-content state drives the
      // document load (fetch-or-convert-and-upload, see RFC 1030 §1.2). The
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
      // auth lock, see RFC 1030 §6.2).
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
      default:
        break;
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
