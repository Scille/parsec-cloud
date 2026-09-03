// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Editics SSE + RPC client (step 0: auth subset).
//
// This is the browser-side counterpart of the server routes in
// `server/parsec/asgi/editics.py` (see `docs/rfcs/1030-collaborative-editics.md`
// and `todo/step_0.md` §9.2). It replaces the localStorage/BroadcastChannel-only
// `connectMockServer` transport with a real connection to the Parsec server
// when a Parsec editics endpoint is available:
//
//   - Opens `GET .../join` (SSE) and registers a pending connection.
//   - Posts the `auth` client event to `POST .../send` and receives the `auth`
//     server event as the RPC reply.
//   - Listens on the SSE stream for `connectState` (and, later, other server
//     events) and translates them to the OnlyOffice-shaped messages the
//     vendored editor expects via `docEditor.sendMessageToOO`.
//   - Maintains the local `[indexUser, deviceId, user_name]` table using a
//     `resolveUserName(deviceId)` callback (the server is not trusted for
//     names, RFC §3.3 / todo §2).
//
// It intentionally has no build step / bundler dependency, consistent with
// `onlyoffice-mock-server.js`. OnlyOffice names are kept verbatim.
//
// On-page debug panel: this file embeds a `Panel` singleton (mirroring the one
// in `onlyoffice-mock-server.js`) that shows every event exchanged. Each entry
// displays BOTH protocol layers side by side:
//   - OnlyOffice protocol: the event as seen by the OnlyOffice editor
//     (editor ↔ this editics client, which stands in for the OnlyOffice
//     "server" via `connectMockServer`).
//   - Editics protocol: the event as carried over the network to the Parsec
//     server (this editics client ↔ Parsec server, over SSE + RPC).
(function (global) {
  'use strict';

  // ---------------------------------------------------------------------------------------------
  // Debug panel
  //
  // One overlay on the host page. Each log entry is a `<details>` whose summary
  // shows a colored flow title describing the direction the event travels, the
  // event type and the time. Expanding it reveals one or two labeled `<pre>`
  // blocks: the OnlyOffice representation and/or the Editics (network)
  // representation.
  //
  // Flow titles (each with its own color) make the direction unambiguous without
  // the "client"/"server" ambiguity (our editics client is the OnlyOffice
  // "server" but the Parsec "client"):
  //   - `OO -> editics`            editor sends to the editics client, local only.
  //   - `OO <- editics`            editics client sends to the editor, local only.
  //   - `OO -> editics -> server`  editor -> editics client, forwarded to Parsec.
  //   - `OO <- editics <- server`  Parsec -> editics client, forwarded to editor.
  //   - `editics -> server`        editics client -> Parsec, no OnlyOffice side.
  //   - `editics <- server`        Parsec -> editics client, no OnlyOffice side.
  // ---------------------------------------------------------------------------------------------
  const Panel = (function () {
    let root, logEl, statusEl, filterSel;
    let entryCount = 0;
    const MAX_ENTRIES = 300;
    // `filter`: 'all' | 'oo' | 'net'. Hides entries that don't match the chosen
    // layer (paired entries always match).
    let filter = 'all';

    function ensure() {
      if (root) return;
      const style = document.createElement('style');
      style.textContent = `
        #ed-panel { position: fixed; right: 0; bottom: 0; width: 560px; max-height: 60vh;
          display: flex; flex-direction: column; font: 11px/1.4 monospace; background: #1e1e1eee;
          color: #ddd; border-top-left-radius: 6px; z-index: 999999; box-shadow: 0 0 12px #0008; }
        #ed-panel.collapsed { max-height: unset; }
        #ed-panel.collapsed .ed-body { display: none; }
        #ed-panel .ed-header { display: flex; justify-content: space-between; align-items: center;
          padding: 4px 8px; background: #333; cursor: pointer; user-select: none;
          border-top-left-radius: 6px; }
        #ed-panel .ed-body { display: flex; flex-direction: column; min-height: 0; }
        #ed-panel .ed-toolbar { display: flex; align-items: center; gap: 6px; padding: 4px 8px;
          border-bottom: 1px solid #444; }
        #ed-panel .ed-toolbar select { font: 10px/1 monospace; background: #222; color: #ddd;
          border: 1px solid #555; border-radius: 3px; padding: 2px 4px; }
        #ed-panel .ed-toolbar button { font: 10px/1 monospace; background: #444; color: #eee;
          border: 1px solid #666; border-radius: 3px; padding: 3px 6px; cursor: pointer; }
        #ed-panel .ed-toolbar button:hover { background: #555; }
        #ed-panel .ed-log { overflow-y: auto; padding: 4px 8px; flex: 1; }
        #ed-panel .ed-entry { padding: 2px 0; border-bottom: 1px solid #2a2a2a; }
        #ed-panel .ed-entry[hidden] { display: none; }
        #ed-panel .ed-entry summary { cursor: pointer; white-space: pre-wrap; word-break: break-word; }
        #ed-panel .ed-section { margin: 2px 0 4px 12px; }
        #ed-panel .ed-section-label { color: #888; font-size: 10px; margin-bottom: 1px; }
        #ed-panel .ed-entry pre { margin: 0; white-space: pre-wrap; word-break: break-word; }
        #ed-panel .ed-oo pre { color: #f2c94c; }
        #ed-panel .ed-net pre { color: #9cdcfe; }
        #ed-panel .ed-flow { font-size: 10px; font-weight: bold; padding: 1px 6px; border-radius: 3px;
          margin-right: 4px; white-space: nowrap; }
        #ed-panel .ed-flow-oo-in   { background: #6b5a1a; color: #ffe08a; } /* OO -> editics            (gold)     */
        #ed-panel .ed-flow-oo-out  { background: #3d5a1a; color: #c8e08a; } /* OO <- editics            (olive)    */
        #ed-panel .ed-flow-oo2srv  { background: #5a1a6b; color: #e08aff; } /* OO -> editics -> server  (purple)   */
        #ed-panel .ed-flow-srv2oo  { background: #1a5a6b; color: #8ae0ff; } /* OO <- editics <- server  (teal)     */
        #ed-panel .ed-flow-net-out { background: #1a3d6b; color: #8ab8ff; } /* editics -> server        (blue)     */
        #ed-panel .ed-flow-net-in  { background: #6b1a3d; color: #ff8ac8; } /* editics <- server        (magenta)  */
        #ed-panel .ed-dir { color: #aaa; }
        #ed-panel .ed-type { color: #fff; font-weight: bold; }
        #ed-panel .ed-time { color: #666; }
        #ed-panel .ed-status { padding: 4px 8px; border-top: 1px solid #444; color: #9cdcfe; }
      `;
      document.head.appendChild(style);

      root = document.createElement('div');
      root.id = 'ed-panel';
      root.innerHTML = `
        <div class="ed-header">
          <span>Editics protocol (OnlyOffice ↔ editics client ↔ Parsec server)</span>
          <span class="ed-toggle">–</span>
        </div>
        <div class="ed-body">
          <div class="ed-toolbar">
            <select class="ed-filter" title="Filter by protocol layer">
              <option value="all">all layers</option>
              <option value="oo">OnlyOffice only</option>
              <option value="net">Editics (network) only</option>
            </select>
            <button class="ed-clear" title="Clear the log">clear</button>
          </div>
          <div class="ed-log"></div>
          <div class="ed-status"></div>
        </div>
      `;
      document.body.appendChild(root);
      logEl = root.querySelector('.ed-log');
      statusEl = root.querySelector('.ed-status');
      filterSel = root.querySelector('.ed-filter');
      const header = root.querySelector('.ed-header');
      const toggle = root.querySelector('.ed-toggle');
      header.addEventListener('click', () => {
        root.classList.toggle('collapsed');
        toggle.textContent = root.classList.contains('collapsed') ? '+' : '–';
      });
      filterSel.addEventListener('change', () => {
        filter = filterSel.value;
        applyFilter();
      });
      root.querySelector('.ed-clear').addEventListener('click', (ev) => {
        ev.stopPropagation();
        logEl.innerHTML = '';
        entryCount = 0;
      });
    }

    function applyFilter() {
      const entries = logEl.querySelectorAll('.ed-entry');
      entries.forEach((el) => {
        const layers = el.getAttribute('data-layers') || '';
        const show = filter === 'all' || layers === 'paired' || layers.indexOf(filter) !== -1;
        el.hidden = !show;
      });
    }

    function setStatus(text) {
      ensure();
      statusEl.textContent = text;
    }

    // Derive the flow title (`ed-flow` CSS class + label) from an entry's
    // `oo`/`net` directions. The title describes the whole path the event
    // travels, so paired entries get a single combined title:
    //   - OO in  + NET out  -> `OO -> editics -> server` (editor -> us -> Parsec)
    //   - NET in  + OO out  -> `OO <- editics <- server` (Parsec -> us -> editor)
    //   - OO in  only        -> `OO -> editics`
    //   - OO out only        -> `OO <- editics`
    //   - NET out only       -> `editics -> server`
    //   - NET in  only       -> `editics <- server`
    function flowTitle(hasOo, hasNet, ooDir, netDir) {
      if (hasOo && hasNet) {
        // Paired: the OO and NET directions are consistent (in/out), so the
        // combined title reads end-to-end. The editor side drives the title.
        if (ooDir === 'in') return { cls: 'ed-flow-oo2srv', label: 'OO -> editics -> server' };
        return { cls: 'ed-flow-srv2oo', label: 'OO <- editics <- server' };
      }
      if (hasOo)
        return ooDir === 'in' ? { cls: 'ed-flow-oo-in', label: 'OO -> editics' } : { cls: 'ed-flow-oo-out', label: 'OO <- editics' };
      return netDir === 'out'
        ? { cls: 'ed-flow-net-out', label: 'editics -> server' }
        : { cls: 'ed-flow-net-in', label: 'editics <- server' };
    }

    function safeStringify(obj) {
      try {
        return typeof obj === 'string' ? obj : JSON.stringify(obj, null, 1);
      } catch (_e) {
        return String(obj);
      }
    }

    // `entry` shape:
    //   { type, note, oo: {dir, payload}|null, net: {dir, payload}|null }
    // `oo.dir`: 'in' = editor -> editics, 'out' = editics -> editor.
    // `net.dir`: 'out' = editics -> server, 'in' = server -> editics.
    function log(entry) {
      ensure();
      const hasOo = !!entry.oo;
      const hasNet = !!entry.net;
      const layers = hasOo && hasNet ? 'paired' : hasOo ? 'oo' : 'net';
      const details = document.createElement('details');
      details.className = 'ed-entry';
      details.setAttribute('data-layers', layers);
      const time = new Date().toLocaleTimeString();
      const summary = document.createElement('summary');
      const flow = flowTitle(hasOo, hasNet, hasOo && entry.oo.dir, hasNet && entry.net.dir);
      const noteSuffix = entry.note ? ` <span class=\"ed-dir\">\u00b7 ${entry.note}</span>` : '';
      summary.innerHTML =
        `<span class=\"ed-flow ${flow.cls}\">${flow.label}</span>` +
        `<span class=\"ed-type\">${entry.type}</span> ` +
        `<span class=\"ed-time\">${time}</span>` +
        noteSuffix;
      details.appendChild(summary);

      if (hasOo) {
        const sec = document.createElement('div');
        sec.className = 'ed-section ed-oo';
        sec.innerHTML = `<div class="ed-section-label">OnlyOffice protocol</div>`;
        const pre = document.createElement('pre');
        pre.textContent = safeStringify(entry.oo.payload);
        sec.appendChild(pre);
        details.appendChild(sec);
      }
      if (hasNet) {
        const sec = document.createElement('div');
        sec.className = 'ed-section ed-net';
        sec.innerHTML = `<div class="ed-section-label">Editics protocol / network</div>`;
        const pre = document.createElement('pre');
        pre.textContent = safeStringify(entry.net.payload);
        sec.appendChild(pre);
        details.appendChild(sec);
      }

      logEl.appendChild(details);
      entryCount++;
      while (entryCount > MAX_ENTRIES && logEl.firstChild) {
        logEl.removeChild(logEl.firstChild);
        entryCount--;
      }
      // Apply current filter to the new entry.
      const show = filter === 'all' || layers === 'paired' || layers.indexOf(filter) !== -1;
      details.hidden = !show;
      // Only auto-scroll to the bottom when the user is already (roughly) at the
      // bottom; if they scrolled up to inspect history, don't yank the view back
      // down on every new event.
      const nearBottom = logEl.scrollHeight - logEl.scrollTop - logEl.clientHeight < details.offsetHeight + 24;
      if (nearBottom) logEl.scrollTop = logEl.scrollHeight;
    }

    return { log, setStatus };
  })();

  // Encode the `Authorization: Editics <device_id_hex>.<participant_uuid_hex>` header
  // (todo step_0 §3.3). `deviceIdHex` is the hex of the client's DeviceID.
  function editicsAuthHeader(deviceIdHex, participantUuid) {
    return 'Editics ' + deviceIdHex + '.' + participantUuid;
  }

  class EditicsClient {
    // `config`:
    //   baseUrl:        Origin + path prefix up to (but not including) the org id,
    //                   e.g. "http://parsec.invalid" (the routes are under
    //                   `/authenticated/{org}/editics/sessions/...`).
    //   organizationId: OrganizationID string.
    //   workspaceId:    WorkspaceID (VlobID) hex.
    //   vlobId:         VlobID hex of the document.
    //   deviceIdHex:    DeviceID hex of the client's device.
    //   vlobVersion:    Loaded vlob version (RFC §1.2).
    //   editorType:     0=Word, 1=Spreadsheet, 2=Presentation, 3=Visio.
    //   resolveUserName: async (deviceIdHex) => user_name (libparsec lookup).
    //   docEditor:      The OnlyOffice DocEditor instance (for sendMessageToOO).
    constructor(config) {
      this.config = config;
      this.participantUuid = global.crypto && global.crypto.randomUUID ? global.crypto.randomUUID() : String(Math.random());
      this.authHeader = editicsAuthHeader(config.deviceIdHex, this.participantUuid);
      const sessionPath =
        '/authenticated/' + encodeURIComponent(config.organizationId) + '/editics/sessions/' + config.workspaceId + '/' + config.vlobId;
      this.joinUrl = config.baseUrl + sessionPath + '/join';
      this.sendUrl = config.baseUrl + sessionPath + '/send';
      this.indexUser = 0; // provisional, overridden by the server-assigned index on auth
      // [indexUser, deviceId, user_name] table (todo §2). Resolved lazily.
      this._participants = new Map(); // indexUser -> { deviceId, userName }
      // Seed ourselves as a provisional participant (index 0) so the editor's
      // initial `getParticipants()` call during `connectMockServer` has someone
      // to show before the server-assigned `indexUser` arrives over the auth
      // RPC. Mirrors the local mock server, which seeds presence on `onAuth`.
      this._participants.set(0, { deviceId: config.deviceIdHex, userName: config.userName || config.userId || config.deviceIdHex });
      this._abort = null;
      this._closed = false;
      Panel.setStatus(this._statusText('constructed (no session yet)'));
    }

    _statusText(state) {
      const doc = this.config.workspaceId + '/' + this.config.vlobId;
      return `editics ${doc} · indexUser=${this.indexUser} · ${this._participants.size} participant(s) · ${state}`;
    }

    setEditor(docEditor) {
      this.config.docEditor = docEditor;
    }

    // --- OnlyOffice connectMockServer API (the parts step 0 needs) ---------

    getParticipants() {
      // OnlyOffice expects `{ list, index }`. In step 0 the only participant is
      // ourselves (the newcomer); the full list comes from the `auth`/connectState
      // server events, which we translate into `connectState` messages below.
      const list = [];
      let index = -1;
      this._participants.forEach((p, indexUser) => {
        const entry = {
          id: String(indexUser),
          idOriginal: p.deviceId,
          username: p.userName || p.deviceId,
          indexUser: indexUser,
          view: false,
        };
        list.push(entry);
        if (indexUser === this.indexUser) index = list.length - 1;
      });
      const result = { list, index };
      // Pure OnlyOffice-protocol query (editor asks its "server" for the current
      // participant map). No Editics/network counterpart.
      Panel.log({
        type: 'getParticipants',
        oo: { dir: 'in', payload: { returns: result } },
      });
      return result;
    }

    getInitialChanges() {
      // No durable change log on the client side in step 0 (fresh session, no
      // modifications). The server delivers the backlog (empty here) as
      // `authChanges` SSE events, not through this hook.
      const changes = [];
      Panel.log({
        type: 'getInitialChanges',
        oo: { dir: 'in', payload: { returns: changes } },
      });
      return changes;
    }

    getImageURL() {
      Panel.log({ type: 'getImageURL', oo: { dir: 'in', payload: { returns: '' } } });
      return Promise.resolve('');
    }

    onAuth() {
      // Open the SSE stream and post the `auth` RPC. This is the join flow
      // (todo step_0 §6). The `auth` server event is returned as the RPC reply;
      // the `connectState` broadcast arrives over SSE.
      //
      // The collaborative transport must never break the editor open flow: if
      // the Parsec server is unreachable or rejects the join, we log and bail
      // out so the editor still opens (degraded, local-only). The OnlyOffice
      // `connectMockServer` contract (CryptPad fork) does not require a server
      // `auth` reply to finish opening (the original mock server never sent
      // one), so missing it is not fatal.
      //
      // `onAuth` intentionally does NOT return a promise: the editor calls it
      // synchronously during `connectMockServer` and the original mock server's
      // `onAuth` returns `undefined`. Returning a thenable here could make the
      // editor branch on a truthy value, so the async join runs in the
      // background instead.
      // The OnlyOffice side of the join: the editor calls `onAuth()`. The
      // Editics/network side is the SSE `GET .../join` + the `auth` RPC, logged
      // individually as they happen below.
      Panel.log({
        type: 'onAuth',
        note: 'editor triggers join',
        oo: { dir: 'in', payload: {} },
        net: { dir: 'out', payload: { GET: this.joinUrl, note: 'open SSE (pending connection)' } },
      });
      this._join().catch((err) => {
        console.warn('[editics] join flow failed, opening editor without collaboration', err);
        Panel.log({
          type: 'onAuth',
          note: 'join failed: ' + ((err && err.message) || err),
          net: { dir: 'in', payload: { error: String(err) } },
        });
        Panel.setStatus(this._statusText('join failed'));
      });
    }

    async _join() {
      await this._openSse();
      await this._sendAuth();
    }

    onMessage(msg) {
      // The OnlyOffice editor sent a message to its "server" (us). Forward the
      // step-1 client events to the Parsec server over the RPC, mapping fields
      // per the editics protocol (RFC §2.2). Encrypted-shaped fields are passed
      // through as opaque base64 for now (§2.4: real encryption is deferred).
      const type = msg && msg.type;
      Panel.log({ type: type || 'onMessage', oo: { dir: 'in', payload: msg } });
      switch (type) {
        case 'message':
          this._post({ type: 'message', encryptedMessage: this._toB64(msg.message) });
          break;
        case 'cursor':
          this._post({ type: 'cursor', encryptedCursor: this._toB64(msg.cursor) });
          break;
        case 'getLock':
          this._post({ type: 'getLock', block: msg.block });
          break;
        case 'isSaveLock':
          this._post({ type: 'isSaveLock', syncChangesIndex: msg.syncChangesIndex });
          break;
        case 'saveChanges':
          this._post({
            type: 'saveChanges',
            // Binary changes mode: `changes` is a real JSON array (one entry per
            // fragment); base64-encode each fragment independently (§2.2).
            encryptedChanges: (msg.changes || []).map((c) => this._toB64(c)),
            startSaveChanges: !!msg.startSaveChanges,
            endSaveChanges: !!msg.endSaveChanges,
            deleteIndex: msg.deleteIndex,
            excel_info: msg.excel_info,
            encryptedCursor: msg.encryptedCursor != null ? this._toB64(msg.encryptedCursor) : null,
            releaseLocks: !!msg.releaseLocks,
          });
          break;
        case 'unSaveLock':
          this._post({ type: 'unSaveLock' });
          break;
        case 'unLockDocument':
          this._post({
            type: 'unLockDocument',
            isSave: !!msg.isSave,
            unlock: !!msg.unlock,
            deleteIndex: msg.deleteIndex,
            releaseLocks: !!msg.releaseLocks,
          });
          break;
        case 'close':
          this._post({ type: 'close' });
          break;
        case 'authChangesAck':
          this._post({ type: 'authChangesAck' });
          break;
        case 'saveDone':
          // Editics addition (no OO equivalent); the host page posts it after a
          // vlob upload. Forward as-is.
          this._post({ type: 'saveDone', savedUpToIndex: msg.savedUpToIndex, newVersion: msg.newVersion });
          break;
        default:
          // Unknown OO event: no editics counterpart (logged on the OO side
          // above). Do not forward.
          break;
      }
    }

    _toB64(value) {
      // Pass opaque content through as base64. OnlyOffice cursor/change
      // fragments may already be strings (JSON-encoded); treat strings as
      // UTF-8 bytes. Real encryption is layered in a later step (§2.4).
      if (value == null) return '';
      if (typeof value === 'string') {
        // encodeURIComponent+unescape handles UTF-8; btoa then base64-encodes.
        try {
          return btoa(unescape(encodeURIComponent(value)));
        } catch (_e) {
          return btoa(String(value));
        }
      }
      if (value instanceof Uint8Array) {
        let s = '';
        for (let i = 0; i < value.length; i++) s += String.fromCharCode(value[i]);
        return btoa(s);
      }
      return btoa(String(value));
    }

    _fromB64(b64) {
      // Decode a base64 string back to a UTF-8 string (for opaque pass-through
      // to OnlyOffice). Real decryption is deferred (§2.4).
      try {
        return decodeURIComponent(escape(atob(b64)));
      } catch (_e) {
        return atob(b64);
      }
    }

    async _post(body) {
      // Fire-and-forget RPC: the reply (a server event to the sender, or 204)
      // is logged but most flows get their sender-visible result over SSE.
      try {
        const rep = await fetch(this.sendUrl, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', Authorization: this.authHeader },
          body: JSON.stringify(body),
        });
        if (!rep.ok) {
          Panel.log({ type: body.type, note: 'RPC ' + rep.status, net: { dir: 'in', payload: { status: rep.status } } });
          return;
        }
        if (rep.status === 204) return;
        const data = await rep.json();
        Panel.log({ type: body.type + ' reply', net: { dir: 'in', payload: data } });
        this._applyReply(body.type, data);
      } catch (err) {
        Panel.log({
          type: body.type,
          note: 'RPC error: ' + ((err && err.message) || err),
          net: { dir: 'in', payload: { error: String(err) } },
        });
      }
    }

    _applyReply(requestType, data) {
      // Map the RPC reply (a server event addressed to the sender) back to the
      // OnlyOffice shape the editor expects, and push it via sendMessageToOO.
      if (!data || !data.type) return;
      switch (data.type) {
        case 'saveLock':
          this._sendToClient({ type: 'saveLock', saveLock: !!data.saveLock });
          break;
        case 'savePartChanges':
          this._sendToClient({
            type: 'savePartChanges',
            changesIndex: data.changesIndex,
            syncChangesIndex: data.syncChangesIndex,
          });
          break;
        case 'unSaveLock':
          this._sendToClient({
            type: 'unSaveLock',
            index: data.index,
            time: data.time,
            syncChangesIndex: data.syncChangesIndex,
          });
          break;
        case 'getLock':
          // `getLock` reply == the broadcast (§6.6); the SSE broadcast also
          // delivers it to others. The sender's view is identical, so push it
          // once here (the SSE broadcast to self is also delivered, but the
          // server excludes the sender from broadcasts — see §6.6 — so we
          // push the sender's view here).
          this._sendToClient({ type: 'getLock', locks: data.locks });
          break;
        default:
          // Other replies (e.g. `auth`, `waitAuth`) are handled in `_sendAuth`.
          break;
      }
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

    // --- internals ---------------------------------------------------------

    async _openSse() {
      // Open the SSE join stream. `EventSource` cannot set custom headers, so
      // the `Authorization` header is passed as a query parameter fallback the
      // server accepts in step 0 for the SSE route (the RPC route uses the real
      // header). This keeps the browser transport simple without a polyfill.
      const url = this.joinUrl + '?authorization=' + encodeURIComponent(this.authHeader);
      this._sse = new EventSource(url, { withCredentials: false });
      this._sse.onmessage = (ev) => this._onSseData(ev.data);
      // Keepalive uses the `event:keepalive` line; EventSource surfaces it as a
      // named event listener. It only proves liveness, so we ignore it and don't
      // log it (it would spam the panel every keepalive interval).
      this._sse.addEventListener('keepalive', () => {});
      this._sse.onerror = (ev) => {
        if (this._closed) return;
        // EventSource auto-reconnects; in step 0 a real error is unexpected.
        console.warn('[editics] SSE error', ev);
        Panel.log({ type: 'SSE error', net: { dir: 'in', payload: { readyState: this._sse && this._sse.readyState } } });
        Panel.setStatus(this._statusText('SSE error'));
      };
      Panel.setStatus(this._statusText('SSE opened, waiting for auth'));
    }

    async _sendAuth() {
      // `indexUser` is -1 on first open (todo step_0 §4.1). The local
      // `this.indexUser` is a provisional value for the editor's participant
      // view; the RPC always sends -1 in step 0 (no reconnect).
      const body = {
        type: 'auth',
        indexUser: -1,
        editorType: this.config.editorType,
        vlobVersion: this.config.vlobVersion,
      };
      // Paired: the OnlyOffice side is the `auth` client event (here built from
      // the editor's `onAuth` trigger), the Editics side is the `POST /send`
      // carrying it over the network.
      Panel.log({
        type: 'auth',
        note: 'join request',
        oo: {
          dir: 'in',
          payload: { type: 'auth', indexUser: -1, editorType: this.config.editorType, vlobVersion: this.config.vlobVersion },
        },
        net: { dir: 'out', payload: { POST: this.sendUrl, body } },
      });
      const rep = await fetch(this.sendUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: this.authHeader,
        },
        body: JSON.stringify(body),
      });
      if (!rep.ok) {
        const text = await rep.text();
        console.error('[editics] auth RPC failed', rep.status, text);
        Panel.log({ type: 'auth', note: 'RPC failed', net: { dir: 'in', payload: { status: rep.status, body: text } } });
        Panel.setStatus(this._statusText('auth RPC failed (' + rep.status + ')'));
        return;
      }
      const data = await rep.json();
      if (data.type === 'waitAuth') {
        // Auth lock held by another participant (todo step_1 §6.2): we are
        // parked. The completion (authChanges + connectState{waitAuth:false})
        // arrives over SSE once the holder releases the lock.
        Panel.log({ type: 'auth', note: 'parked (waitAuth)', net: { dir: 'in', payload: data } });
        Panel.setStatus(this._statusText('parked (waitAuth)'));
        return;
      }
      if (data.type !== 'auth') {
        console.error('[editics] unexpected auth reply', data);
        Panel.log({ type: 'auth', note: 'unexpected reply', net: { dir: 'in', payload: data } });
        return;
      }
      if (data.result !== 1) {
        // Rejection (todo §7.1): the client should reload to
        // `latestAllowedVersion` and retry. Step 0 only logs it.
        console.warn('[editics] auth rejected', data);
        Panel.log({ type: 'auth', note: 'rejected', net: { dir: 'in', payload: data } });
        Panel.setStatus(this._statusText('auth rejected'));
        return;
      }
      // Success: the Editics/network side is the `auth` server reply. The
      // OnlyOffice side is the `connectState` we push to the editor (the mock
      // server never sent an `auth` OO event; the editor gets participants via
      // `connectState`).
      this.indexUser = data.indexUser;
      // The server's participant list is authoritative: drop the provisional
      // self seed and rebuild from it.
      this._participants.clear();
      // Resolve names for the participant map and store the local table.
      await this._mergeParticipants(data.participants);
      const ooMsg = {
        type: 'connectState',
        participantsTimestamp: Date.now(),
        participants: this._onlyofficeParticipants(),
        waitAuth: false,
      };
      Panel.log({
        type: 'auth',
        note: 'join accepted → push connectState',
        net: { dir: 'in', payload: data },
        oo: { dir: 'out', payload: ooMsg },
      });
      this._sendToClient(ooMsg);
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
      switch (event.type) {
        case 'connectState':
          await this._mergeParticipants(event.participants);
          {
            const ooMsg = {
              type: 'connectState',
              participantsTimestamp: event.participantsTimestamp,
              participants: this._onlyofficeParticipants(),
              waitAuth: !!event.waitAuth,
            };
            // Paired: server `connectState` over SSE → `connectState` to editor.
            Panel.log({
              type: 'connectState',
              net: { dir: 'in', payload: event },
              oo: { dir: 'out', payload: ooMsg },
            });
            this._sendToClient(ooMsg);
          }
          break;
        case 'authChanges':
          // Backlog of changes since the session was created (§6.3). Forward
          // as-is; the host layer base64-decodes each blob when applying.
          {
            const ooMsg = { type: 'authChanges', changes: event.changes || [] };
            Panel.log({
              type: 'authChanges',
              net: { dir: 'in', payload: event },
              oo: { dir: 'out', payload: ooMsg },
            });
            this._sendToClient(ooMsg);
          }
          break;
        case 'message':
          {
            const recs = (event.messages || []).map((m) => ({
              message: this._fromB64(m.encryptedMessage),
              time: m.time,
              user: String(m.authorIndexUser),
              useridoriginal: String(m.authorIndexUser),
              username: this._userName(m.authorIndexUser),
            }));
            const ooMsg = { type: 'message', messages: recs };
            Panel.log({ type: 'message', net: { dir: 'in', payload: event }, oo: { dir: 'out', payload: ooMsg } });
            this._sendToClient(ooMsg);
          }
          break;
        case 'cursor':
          {
            const recs = (event.messages || []).map((m) => ({
              cursor: this._fromB64(m.encryptedCursor),
              time: m.time,
              user: String(m.authorIndexUser),
              useridoriginal: String(m.authorIndexUser),
            }));
            const ooMsg = { type: 'cursor', messages: recs };
            Panel.log({ type: 'cursor', net: { dir: 'in', payload: event }, oo: { dir: 'out', payload: ooMsg } });
            this._sendToClient(ooMsg);
          }
          break;
        case 'getLock':
          // The full lock table after the server attempted to acquire the
          // requested blocks for the requester. Forward as-is.
          Panel.log({ type: 'getLock', net: { dir: 'in', payload: event }, oo: { dir: 'out', payload: event } });
          this._sendToClient(event);
          break;
        case 'releaseLock':
          // OnlyOffice expects `locks` records with the original block shape.
          Panel.log({ type: 'releaseLock', net: { dir: 'in', payload: event }, oo: { dir: 'out', payload: event } });
          this._sendToClient(event);
          break;
        case 'saveChanges':
          {
            // Broadcast to other participants: map the editics records back to
            // OnlyOffice's { docid, change, time, user, useridoriginal } shape.
            const docid = this.config.workspaceId + '/' + this.config.vlobId;
            const changes = (event.changes || []).map((c) => ({
              docid,
              change: this._fromB64(c.change),
              time: c.time,
              user: String(c.authorIndexUser),
              useridoriginal: String(c.authorIndexUser),
            }));
            const ooMsg = {
              type: 'saveChanges',
              changes,
              changesIndex: event.changesIndex,
              syncChangesIndex: event.syncChangesIndex,
              endSaveChanges: !!event.endSaveChanges,
              locks: event.locks || [],
              excelAdditionalInfo: event.encryptedCursor != null ? event.encryptedCursor : undefined,
            };
            Panel.log({ type: 'saveChanges', net: { dir: 'in', payload: event }, oo: { dir: 'out', payload: ooMsg } });
            this._sendToClient(ooMsg);
          }
          break;
        case 'savePartChanges':
        case 'unSaveLock':
          // Replies to the saver are delivered via the RPC reply path
          // (`_applyReply`), not over SSE. Ignore the SSE copy (the server only
          // sends these to the saver over RPC, so this is just defensive).
          Panel.log({ type: event.type, net: { dir: 'in', payload: event } });
          break;
        case 'drop':
          Panel.log({ type: 'drop', net: { dir: 'in', payload: event }, oo: { dir: 'out', payload: event } });
          this._sendToClient(event);
          // We have been force-removed: stop the SSE stream.
          if (this._sse) this._sse.close();
          break;
        case 'warning':
          Panel.log({ type: 'warning', net: { dir: 'in', payload: event }, oo: { dir: 'out', payload: event } });
          this._sendToClient(event);
          break;
        default:
          // Unknown server event: pass through (the client does not strictly
          // validate server events, todo §2).
          Panel.log({
            type: event.type || 'unknown',
            note: 'unknown server event (pass-through)',
            net: { dir: 'in', payload: event },
            oo: { dir: 'out', payload: event },
          });
          this._sendToClient(event);
      }
    }

    async _mergeParticipants(participants) {
      for (const p of participants) {
        if (!this._participants.has(p.indexUser)) {
          let userName = p.deviceId;
          try {
            if (this.config.resolveUserName) {
              const resolved = await this.config.resolveUserName(p.deviceId);
              if (resolved) userName = resolved;
            }
          } catch (_e) {
            // Fall back to the device id (the server is not trusted for names).
          }
          this._participants.set(p.indexUser, { deviceId: p.deviceId, userName });
        }
      }
    }

    _onlyofficeParticipants() {
      const list = [];
      this._participants.forEach((p, indexUser) => {
        list.push({
          id: String(indexUser),
          idOriginal: p.deviceId,
          username: p.userName,
          indexUser: indexUser,
          view: false,
        });
      });
      return list;
    }

    _userName(indexUser) {
      const p = this._participants.get(indexUser);
      return p ? p.userName : String(indexUser);
    }

    _sendToClient(msg) {
      if (this.config.docEditor && this.config.docEditor.sendMessageToOO) {
        this.config.docEditor.sendMessageToOO(msg);
      }
    }
  }

  global.OnlyOfficeEditicsClient = {
    create(config) {
      return new EditicsClient(config);
    },
    editicsAuthHeader,
  };
})(window);
