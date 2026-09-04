// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Editics connection glue (todo step_2 §5). This is the shrunk version of the
// former `client/public/onlyoffice-editics-client.js`: it instantiates the
// pure `EditicsTranslator` (from `protocol.js`) with real capabilities, manages
// the `EventSource` SSE join stream + `fetch` RPC, and implements the OnlyOffice
// `connectMockServer` API by delegating protocol logic to the translator.
//
// This file is browser-only (uses `EventSource`, `fetch`, `crypto`,
// `document`); it is never loaded by tests. Only `protocol.js` is.
//
// The `Authorization: Editics <device_id_hex>.<participant_uuid_hex>` identity
// (todo step_0 §3.3) is still the lightweight scheme; real Parsec auth is
// deferred.

import { Panel } from './debug_panel.js';
import { EditicsTranslator } from './protocol.js';

// Encode the `Authorization: Editics <device_id_hex>.<participant_uuid_hex>`
// header (todo step_0 §3.3).
function editicsAuthHeader(deviceIdHex, participantUuid) {
  return `Editics ${deviceIdHex}.${participantUuid}`;
}

// Build the `/authenticated/{org}/editics/sessions/{realm}/{vlob}` path prefix.
// Kept as a helper so the constructor's template literal stays under the line
// length limit (prettier would otherwise reflow it back to one long line).
function buildSessionPath(organizationId, workspaceId, vlobId) {
  return `/authenticated/${encodeURIComponent(organizationId)}/editics/sessions/${workspaceId}/${vlobId}`;
}

// --- byte <-> base64 (transport encoding, used only by the browser glue) ---

function bytesToB64(bytes) {
  let s = '';
  for (let i = 0; i < bytes.length; i++) s += String.fromCharCode(bytes[i]);
  return btoa(s);
}

function b64ToBytes(b64) {
  const bin = atob(b64);
  const out = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) out[i] = bin.charCodeAt(i);
  return out;
}

function strToBytes(s) {
  // Reuse the host page's `TextEncoder` if available, else manual UTF-8.
  if (typeof TextEncoder !== 'undefined') {
    return new TextEncoder().encode(s);
  }
  const out = [];
  for (let i = 0; i < s.length; i++) {
    const c = s.charCodeAt(i);
    if (c < 0x80) out.push(c);
    else if (c < 0x800) {
      out.push(0xc0 | (c >> 6), 0x80 | (c & 0x3f));
    } else {
      out.push(0xe0 | (c >> 12), 0x80 | ((c >> 6) & 0x3f), 0x80 | (c & 0x3f));
    }
  }
  return new Uint8Array(out);
}

// --- EditicsCapabilities injected into the translator ----------------------
//
// For now (step 2) encryption is a base64 passthrough (wrap the existing
// `_toB64`/`_fromB64` as `Uint8Array` ↔ base64 at the transport boundary): the
// *interface* is locked down so real libparsec sealing drops in later without
// touching the translator (todo §2.4 / §8). `resolveUserName`/`resolveUserId`
// bridge to the parent window (libparsec lives there).

/**
 * @param {OpenDocumentOptions['editics']} editics
 * @returns {import('./protocol.js').EditicsCapabilities}
 */
function buildCapabilities(editics) {
  return {
    resolveUserName: editics.resolveUserName || (async () => undefined),
    resolveUserId: editics.resolveUserId || (async () => undefined),
    // Pass opaque content through as base64 for now. Real encryption is layered
    // in a later step (§2.4); the interface is the only thing that matters here.
    encrypt: (plain) => strToBytes(bytesToB64(plain)),
    decrypt: (cipher) => b64ToBytes(bytesToStr(cipher)),
  };
}

function bytesToStr(bytes) {
  if (typeof TextDecoder !== 'undefined') {
    return new TextDecoder().decode(bytes);
  }
  let s = '';
  for (let i = 0; i < bytes.length; i++) s += String.fromCharCode(bytes[i]);
  return s;
}

// --- Connection glue -------------------------------------------------------

/**
 * @typedef {Object} EditicsClientConfig
 * @property {string} baseUrl - Origin + path prefix up to (but not including)
 *   the org id, e.g. "http://parsec.invalid".
 * @property {string} organizationId
 * @property {string} workspaceId - WorkspaceID (VlobID) hex.
 * @property {string} vlobId - VlobID hex of the document.
 * @property {string} deviceIdHex - DeviceID hex of the client's device.
 * @property {number} vlobVersion - loaded vlob version (RFC §1.2).
 * @property {number} editorType - 0=Word, 1=Spreadsheet, 2=Presentation, 3=Visio.
 * @property {string} [userId] - per-person userId (provisional self seed).
 * @property {string} [userName] - display name (provisional self seed).
 * @property {(deviceIdHex:string)=>Promise<string|undefined>} [resolveUserName]
 * @property {(deviceIdHex:string)=>Promise<string|undefined>} [resolveUserId]
 */

class EditicsClient {
  /** @param {EditicsClientConfig} config */
  constructor(config) {
    this.config = config;
    this.participantUuid = typeof crypto !== 'undefined' && crypto.randomUUID ? crypto.randomUUID() : String(Math.random());
    this.authHeader = editicsAuthHeader(config.deviceIdHex, this.participantUuid);
    const sessionPath = buildSessionPath(config.organizationId, config.workspaceId, config.vlobId);
    this.joinUrl = `${config.baseUrl + sessionPath}/join`;
    this.sendUrl = `${config.baseUrl + sessionPath}/send`;

    /** @type {EditicsTranslator} */
    this.translator = new EditicsTranslator({
      workspaceId: config.workspaceId,
      vlobId: config.vlobId,
      deviceIdHex: config.deviceIdHex,
      userId: config.userId,
      userName: config.userName,
      vlobVersion: config.vlobVersion,
      editorType: config.editorType,
      capabilities: buildCapabilities(config),
    });

    this._abort = null;
    this._closed = false;
    // Start the join flow as early as possible (in the constructor, before
    // `connectMockServer`/`handleAuth` run) so the server-assigned `indexUser`
    // is resolved before the editor reads `getParticipants().index` during
    // `handleAuth`. The editor sets its `_indexUser` ONCE from that first
    // `auth` OO event; if it reads the provisional 0 while the RPC is still in
    // flight, every `getLock`/`saveChanges` `user` field would mismatch and
    // structural edits (region locks) would get stuck.
    this._join().catch((err) => {
      console.warn('[editics] join flow failed, opening editor without collaboration', err);
      Panel.log({
        type: 'onAuth',
        note: `join failed: ${(err && err.message) || err}`,
        net: { dir: 'in', payload: { error: String(err) } },
      });
      Panel.setStatus(this._statusText('join failed'));
    });
    Panel.setStatus(this._statusText('constructed (joining...)'));
  }

  _statusText(state) {
    const doc = `${this.config.workspaceId}/${this.config.vlobId}`;
    return `editics ${doc} · indexUser=${this.translator.indexUser} · ${this.translator._participants.size} participant(s) · ${state}`;
  }

  setEditor(docEditor) {
    this.config.docEditor = docEditor;
  }

  // --- OnlyOffice connectMockServer API -----------------------------------

  getParticipants() {
    const result = this.translator.getParticipants();
    Panel.log({ type: 'getParticipants', oo: { dir: 'in', payload: { returns: result } } });
    return result;
  }

  getInitialChanges() {
    const changes = this.translator.getInitialChanges();
    Panel.log({ type: 'getInitialChanges', oo: { dir: 'in', payload: { returns: changes } } });
    return changes;
  }

  getImageURL() {
    Panel.log({ type: 'getImageURL', oo: { dir: 'in', payload: { returns: '' } } });
    return this.translator.getImageURL();
  }

  onAuth() {
    // The join flow is started in the constructor so the server-assigned
    // `indexUser` is resolved before `handleAuth` reads `getParticipants()`
    // (see the constructor comment). Nothing to do here except log.
    Panel.log({
      type: 'onAuth',
      note: 'editor triggers join (join already started in constructor)',
      oo: { dir: 'in', payload: {} },
    });
  }

  async _join() {
    await this._openSse();
    await this._sendAuth();
  }

  /** @param {object} msg - OnlyOffice client event payload (editor -> "server"). */
  async onMessage(msg) {
    const type = msg && msg.type;
    Panel.log({ type: type || 'onMessage', oo: { dir: 'in', payload: msg } });
    /** @type {EditicsClientEvent|null} */
    const cooked = await this.translator.cookClientEvent(msg);
    if (cooked === null) {
      return;
    }
    await this._post(cooked);
  }

  async _post(body) {
    // Fire-and-forget RPC: the reply (a server event to the sender, or 204)
    // is logged but most flows get their sender-visible result over SSE.
    // Serialize the editics event to its JSON wire form: encrypted `bytes`
    // fields become base64 here (the transport boundary), matching pydantic.
    const wire = this._toWire(body);
    Panel.log({
      type: body.type,
      note: 'forward to server',
      net: { dir: 'out', payload: { POST: this.sendUrl, body: wire } },
    });
    try {
      const rep = await fetch(this.sendUrl, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', Authorization: this.authHeader },
        body: JSON.stringify(wire),
      });
      if (!rep.ok) {
        Panel.log({ type: body.type, note: `RPC ${rep.status}`, net: { dir: 'in', payload: { status: rep.status } } });
        return;
      }
      if (rep.status === 204) {
        Panel.log({ type: body.type, note: '204 no reply', net: { dir: 'in', payload: { status: 204 } } });
        return;
      }
      const data = await rep.json();
      Panel.log({ type: `${body.type} reply`, net: { dir: 'in', payload: data } });
      await this._applyReply(body.type, data);
    } catch (err) {
      Panel.log({
        type: body.type,
        note: `RPC error: ${(err && err.message) || err}`,
        net: { dir: 'in', payload: { error: String(err) } },
      });
    }
  }

  // Convert an `EditicsClientEvent` (with `Uint8Array` encrypted fields) into
  // the plain-JSON wire form (encrypted fields as base64 strings).
  _toWire(body) {
    const out = { type: body.type };
    for (const key of Object.keys(body)) {
      if (key === 'type') continue;
      const v = body[key];
      if (v instanceof Uint8Array) {
        out[key] = bytesToB64(v);
      } else if (Array.isArray(v) && v.length && v[0] instanceof Uint8Array) {
        out[key] = v.map((b) => bytesToB64(b));
      } else {
        out[key] = v;
      }
    }
    return out;
  }

  // Convert a server JSON reply (base64 strings for bytes) into the
  // `EditicsServerEvent` shape the translator expects (Uint8Array for bytes).
  _fromWire(data) {
    return _coerceServerEvent(data);
  }

  async _applyReply(_requestType, data) {
    // Map the RPC reply (a server event addressed to the sender) back to the
    // OnlyOffice shape the editor expects, and push it via sendMessageToOO.
    if (!data || !data.type) return;
    const editics = this._fromWire(data);
    const oo = await this.translator.cookServerEvent(editics);
    if (oo === null) {
      // No OO counterpart (e.g. `auth` success/rejection are handled in
      // `_sendAuth`; some replies have no forward shape). Nothing to push.
      return;
    }
    Panel.log({ type: editics.type, note: 'reply -> editor (translated)', oo: { dir: 'out', payload: oo } });
    this._sendToClient(oo);
  }

  destroy() {
    this._closed = true;
    if (this._abort) {
      this._abort.abort();
      this._abort = null;
    }
    if (this._sse && this._sse.readyState !== EventSource.CLOSED) {
      this._sse.close();
    }
    this._sse = null;
    Panel.log({ type: 'destroy', note: 'SSE closed, leaving session', net: { dir: 'out', payload: { close: 'SSE' } } });
    Panel.setStatus(this._statusText('destroyed'));
  }

  // --- internals -----------------------------------------------------------

  async _openSse() {
    // Open the SSE join stream. `EventSource` cannot set custom headers, so
    // the `Authorization` header is passed as a query parameter fallback the
    // server accepts for the SSE route (the RPC route uses the real header).
    const url = `${this.joinUrl}?authorization=${encodeURIComponent(this.authHeader)}`;
    // Wait for the EventSource to actually connect (readyState OPEN) before
    // resolving, so the `auth` RPC (sent next in `_join`) finds the pending
    // SSE connection the server registered on the GET.
    await new Promise((resolve) => {
      this._sse = new EventSource(url, { withCredentials: false });
      const done = () => resolve(undefined);
      this._sse.addEventListener('open', done, { once: true });
      this._sse.addEventListener('error', done, { once: true }); // best-effort; retry below
      // Safety timeout in case `open` never fires.
      setTimeout(done, 2000);
    });
    this._sse.onmessage = (ev) => this._onSseData(ev.data);
    this._sse.addEventListener('keepalive', () => {});
    this._sse.onerror = (ev) => {
      if (this._closed) return;
      console.warn('[editics] SSE error', ev);
      Panel.log({ type: 'SSE error', net: { dir: 'in', payload: { readyState: this._sse && this._sse.readyState } } });
      Panel.setStatus(this._statusText('SSE error'));
    };
    Panel.setStatus(this._statusText('SSE opened, waiting for auth'));
  }

  async _sendAuth() {
    // `indexUser` is -1 on first open (todo step_0 §4.1).
    const oo = {
      type: 'auth',
      indexUser: -1,
      editorType: this.config.editorType,
      vlobVersion: this.config.vlobVersion,
    };
    const cooked = await this.translator.cookClientEvent(oo);
    const wire = this._toWire(cooked);
    Panel.log({
      type: 'auth',
      note: 'join request',
      oo: { dir: 'in', payload: oo },
      net: { dir: 'out', payload: { POST: this.sendUrl, body: wire } },
    });
    const rep = await fetch(this.sendUrl, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json', Authorization: this.authHeader },
      body: JSON.stringify(wire),
    });
    if (!rep.ok) {
      const text = await rep.text();
      console.error('[editics] auth RPC failed', rep.status, text);
      Panel.log({ type: 'auth', note: 'RPC failed', net: { dir: 'in', payload: { status: rep.status, body: text } } });
      Panel.setStatus(this._statusText(`auth RPC failed (${rep.status})`));
      return;
    }
    const data = await rep.json();
    const editics = this._fromWire(data);
    if (editics.type === 'waitAuth') {
      // Auth lock held by another participant (todo step_1 §6.2): we are
      // parked. The completion (authChanges + connectState{waitAuth:false})
      // arrives over SSE once the holder releases the lock. Still forward the
      // OO `waitAuth` event so the editor shows the waiting state.
      const ooWaitAuth = await this.translator.cookServerEvent(editics);
      Panel.log({
        type: 'auth',
        note: 'parked (waitAuth)',
        net: { dir: 'in', payload: data },
        oo: ooWaitAuth ? { dir: 'out', payload: ooWaitAuth } : undefined,
      });
      if (ooWaitAuth) this._sendToClient(ooWaitAuth);
      Panel.setStatus(this._statusText('parked (waitAuth)'));
      return;
    }
    if (editics.type !== 'auth') {
      console.error('[editics] unexpected auth reply', data);
      Panel.log({ type: 'auth', note: 'unexpected reply', net: { dir: 'in', payload: data } });
      return;
    }
    if (editics.result !== 1) {
      // Rejection (todo §7.1): the client should reload to
      // `latestAllowedVersion` and retry. Step 2 only logs it.
      console.warn('[editics] auth rejected', data);
      Panel.log({ type: 'auth', note: 'rejected', net: { dir: 'in', payload: data } });
      Panel.setStatus(this._statusText('auth rejected'));
      return;
    }
    // Success: the Editics/network side is the `auth` server reply. The
    // OnlyOffice side is the `connectState` we push to the editor (the mock
    // server never sent an `auth` OO event; the editor gets participants via
    // `connectState`).
    const ooMsg = await this.translator.cookServerEvent(editics);
    Panel.log({
      type: 'auth',
      note: 'join accepted → push connectState',
      net: { dir: 'in', payload: data },
      oo: ooMsg ? { dir: 'out', payload: ooMsg } : undefined,
    });
    if (ooMsg) this._sendToClient(ooMsg);
    Panel.setStatus(this._statusText('joined'));
  }

  async _onSseData(raw) {
    let event;
    try {
      event = JSON.parse(raw);
    } catch (_e) {
      Panel.log({ type: 'SSE data', note: 'non-JSON', net: { dir: 'in', payload: raw } });
      return;
    }
    const editics = _coerceServerEvent(event);
    const oo = await this.translator.cookServerEvent(editics);
    if (oo === null) {
      Panel.log({ type: editics.type || 'unknown', note: 'no OO counterpart (not forwarded)', net: { dir: 'in', payload: event } });
      return;
    }
    Panel.log({ type: editics.type, net: { dir: 'in', payload: event }, oo: { dir: 'out', payload: oo } });
    this._sendToClient(oo);
    if (editics.type === 'drop' && this._sse) {
      // We have been force-removed: stop the SSE stream.
      this._sse.close();
    }
  }

  _sendToClient(msg) {
    if (this.config.docEditor && this.config.docEditor.sendMessageToOO) {
      this.config.docEditor.sendMessageToOO(msg);
    }
  }
}

// Coerce a raw JSON object from the wire (base64 strings for bytes fields)
// into the `EditicsServerEvent` shape the translator expects (`Uint8Array` for
// bytes). The translator's interface deliberately uses `Uint8Array` so the
// encrypt/decrypt capability is codec-agnostic (todo §4.1).
function _coerceServerEvent(event) {
  if (!event || typeof event.type !== 'string') return event;
  const out = { type: event.type };
  for (const key of Object.keys(event)) {
    if (key === 'type') continue;
    out[key] = _coerceField(event[key], key, event.type);
  }
  return out;
}

// Per-event bytes-field map (mirrors the pydantic `bytes` fields in
// `server/parsec/components/editics.py`). Everything not listed here passes
// through as-is.
const _BYTES_FIELDS = {
  message: { messages: 'encryptedMessage' },
  cursor: { messages: 'encryptedCursor' },
  authChanges: { changes: 1 /* tuple (index, bytes): only index 1 is bytes */ },
  saveChanges: { changes: 'change', encryptedCursor: 0 /* top-level bytes */, locks: null /* no bytes */ },
};

function _coerceField(value, key, eventType) {
  const map = _BYTES_FIELDS[eventType];
  if (!map) return value;
  // Top-level bytes field (value is a base64 string): `encryptedCursor`.
  if (key in map && typeof map[key] === 'number' && map[key] === 0 && typeof value === 'string') {
    return b64ToBytes(value);
  }
  // Array of records with a bytes sub-field.
  if (key === 'messages' && map.messages && Array.isArray(value)) {
    const sub = map.messages;
    return value.map((rec) => ({ ...rec, [sub]: typeof rec[sub] === 'string' ? b64ToBytes(rec[sub]) : rec[sub] }));
  }
  if (key === 'changes' && eventType === 'saveChanges' && Array.isArray(value)) {
    const sub = map.changes;
    return value.map((rec) => ({ ...rec, [sub]: typeof rec[sub] === 'string' ? b64ToBytes(rec[sub]) : rec[sub] }));
  }
  if (key === 'changes' && eventType === 'authChanges' && Array.isArray(value)) {
    // Each entry is a tuple [index, base64blob].
    return value.map((entry) => (Array.isArray(entry) && typeof entry[1] === 'string' ? [entry[0], b64ToBytes(entry[1])] : entry));
  }
  return value;
}

export const OnlyOfficeEditicsClient = {
  /** @param {EditicsClientConfig} config */
  create(config) {
    return new EditicsClient(config);
  },
  editicsAuthHeader,
};
