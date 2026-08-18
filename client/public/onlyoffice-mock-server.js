// POC (step 3.2 in the root CLAUDE.md): "server" side of the OnlyOffice client/server protocol,
// mocked entirely client-side, loaded by onlyoffice-host.html.
//
// Goal: investigate what the OnlyOffice client actually sends/expects (see notes/communication_protocol.md
// for the write-up) without talking to any real backend:
//   - Every message the client sends is logged (and shown in an on-page debug panel), never forwarded
//     over the network.
//   - The "server" replies are faked in-memory / via localStorage + BroadcastChannel (same-origin,
//     no network involved).
//   - Two browser tabs opening the *same* document (same `documentId`) join the same simulated
//     session: this is how you observe genuine non-ephemeral `saveChanges` messages produced by a
//     real second OnlyOffice client instance, real cursor broadcasts, and real join/leave. The debug
//     panel also exposes manual "simulate" buttons for single-tab testing of join/leave/cursor, plus
//     a "replay last edit as <fake user>" button for saveChanges (see caveat in the panel/notes: we
//     can't hand-craft a *novel* OnlyOffice binary patch, only rebroadcast a genuine one).
//
// This file intentionally has no build step / bundler dependency, consistent with the rest of the
// `client/vendors/onlyoffice` POC wiring (see client/vendors/onlyoffice/README.md).
(function (global) {
  'use strict';

  // ---------------------------------------------------------------------------------------------
  // Message classification
  //
  // Investigation finding: there is no explicit "ephemeral" flag on messages. The OnlyOffice client
  // protocol (as (ab)used by CryptPad, see notes/communication_protocol.md) has a small, fixed
  // vocabulary of `type`s, and exactly ONE of them carries an actual document mutation that must be
  // durably stored: `saveChanges` (its `changes` field). Every other type is session bootstrap,
  // presence/lock coordination, or an acknowledgement - safe to lose without corrupting the document.
  // ---------------------------------------------------------------------------------------------
  const PERSISTENT_TYPE = 'saveChanges';
  function classify(type) {
    return type === PERSISTENT_TYPE ? 'persistent' : 'ephemeral';
  }

  const CHECKPOINT_WARN_INTERVAL = 20; // POC-scale stand-in for CryptPad's CHECKPOINT_INTERVAL (100)

  // ---------------------------------------------------------------------------------------------
  // Debug panel: a singleton overlay on the host page showing every message crossing the mock
  // "server" boundary, plus manual simulate-other-user controls.
  // ---------------------------------------------------------------------------------------------
  const Panel = (function () {
    let root, logEl, statusEl, controlsEl;
    let entryCount = 0;
    const MAX_ENTRIES = 300;

    function ensure() {
      if (root) return;
      const style = document.createElement('style');
      style.textContent = `
        #oo-mock-panel { position: fixed; right: 0; bottom: 0; width: 420px; max-height: 55vh;
          display: flex; flex-direction: column; font: 11px/1.4 monospace; background: #1e1e1eee;
          color: #ddd; border-top-left-radius: 6px; z-index: 999999; box-shadow: 0 0 12px #0008; }
        #oo-mock-panel.collapsed { max-height: unset; }
        #oo-mock-panel.collapsed .oo-mock-body { display: none; }
        #oo-mock-panel .oo-mock-header { display: flex; justify-content: space-between; align-items: center;
          padding: 4px 8px; background: #333; cursor: pointer; user-select: none; border-top-left-radius: 6px; }
        #oo-mock-panel .oo-mock-body { display: flex; flex-direction: column; min-height: 0; }
        #oo-mock-panel .oo-mock-controls { display: flex; flex-wrap: wrap; gap: 4px; padding: 6px 8px; border-bottom: 1px solid #444; }
        #oo-mock-panel button { font: 10px/1 monospace; background: #444; color: #eee; border: 1px solid #666;
          border-radius: 3px; padding: 4px 6px; cursor: pointer; }
        #oo-mock-panel button:hover { background: #555; }
        #oo-mock-panel button:disabled { opacity: 0.4; cursor: default; }
        #oo-mock-panel .oo-mock-log { overflow-y: auto; padding: 4px 8px; flex: 1; }
        #oo-mock-panel .oo-mock-entry { padding: 2px 0; border-bottom: 1px solid #2a2a2a; }
        #oo-mock-panel .oo-mock-entry summary { cursor: pointer; white-space: pre-wrap; word-break: break-word; }
        #oo-mock-panel .oo-mock-entry pre { margin: 2px 0 0 12px; white-space: pre-wrap; word-break: break-word; color: #9cdcfe; }
        #oo-mock-panel .dir-out { color: #f2c94c; }
        #oo-mock-panel .dir-in { color: #6fcf97; }
        #oo-mock-panel .tag-persistent { color: #eb5757; font-weight: bold; }
        #oo-mock-panel .tag-ephemeral { color: #828282; }
        #oo-mock-panel .oo-mock-status { padding: 4px 8px; border-top: 1px solid #444; color: #9cdcfe; }
      `;
      document.head.appendChild(style);

      root = document.createElement('div');
      root.id = 'oo-mock-panel';
      root.innerHTML = `
        <div class="oo-mock-header">
          <span>OnlyOffice protocol (mocked server)</span>
          <span class="oo-mock-toggle">–</span>
        </div>
        <div class="oo-mock-body">
          <div class="oo-mock-controls"></div>
          <div class="oo-mock-log"></div>
          <div class="oo-mock-status"></div>
        </div>
      `;
      document.body.appendChild(root);
      logEl = root.querySelector('.oo-mock-log');
      statusEl = root.querySelector('.oo-mock-status');
      controlsEl = root.querySelector('.oo-mock-controls');
      const header = root.querySelector('.oo-mock-header');
      const toggle = root.querySelector('.oo-mock-toggle');
      header.addEventListener('click', () => {
        root.classList.toggle('collapsed');
        toggle.textContent = root.classList.contains('collapsed') ? '+' : '–';
      });
    }

    function addControls(buttons) {
      ensure();
      buttons.forEach(({ label, title, onClick }) => {
        const btn = document.createElement('button');
        btn.textContent = label;
        if (title) btn.title = title;
        btn.addEventListener('click', onClick);
        controlsEl.appendChild(btn);
      });
    }

    function setStatus(text) {
      ensure();
      statusEl.textContent = text;
    }

    function log(direction, msg, meta) {
      ensure();
      const kind = classify(msg.type);
      const entry = document.createElement('details');
      entry.className = 'oo-mock-entry';
      const time = new Date().toLocaleTimeString();
      const arrow = direction === 'out' ? 'client → server' : 'server → client';
      const dirClass = direction === 'out' ? 'dir-out' : 'dir-in';
      const summary = document.createElement('summary');
      const metaSuffix = meta ? ` · ${meta}` : '';
      summary.innerHTML =
        `<span class="${dirClass}">${arrow}</span> ` +
        `<b>${msg.type}</b> ` +
        `<span class="tag-${kind}">[${kind}]</span> ` +
        `<span style="color:#666">${time}${metaSuffix}</span>`;
      const pre = document.createElement('pre');
      pre.textContent = safeStringify(msg);
      entry.appendChild(summary);
      entry.appendChild(pre);
      logEl.appendChild(entry);
      entryCount++;
      while (entryCount > MAX_ENTRIES && logEl.firstChild) {
        logEl.removeChild(logEl.firstChild);
        entryCount--;
      }
      logEl.scrollTop = logEl.scrollHeight;
    }

    function safeStringify(obj) {
      try {
        return JSON.stringify(obj, null, 1);
      } catch (_e) {
        return String(obj);
      }
    }

    return { addControls, setStatus, log };
  })();

  // ---------------------------------------------------------------------------------------------
  // Mock server
  // ---------------------------------------------------------------------------------------------
  class MockServer {
    constructor({ documentId, userId, userName, mode }) {
      this.documentId = documentId;
      this.userId = userId;
      this.userName = userName;
      this.mode = mode; // 'view' | 'edit'
      this.clientId = (global.crypto && global.crypto.randomUUID) ? global.crypto.randomUUID() : String(Math.random());
      this.docEditor = null;
      this.channelKey = `oo-mock:${documentId}`;
      this.logStorageKey = `${this.channelKey}:log`;
      this.presenceStorageKey = `${this.channelKey}:presence`;
      this.bc = ('BroadcastChannel' in global) ? new BroadcastChannel(this.channelKey) : null;
      this._fakeParticipants = []; // manually-simulated (non-tab) participants, for single-tab testing
      this._cursorHistory = []; // this session's own recent real cursor values, see the 'cursor' case below
      this._lastParticipantsKey = '';
      this._heartbeatTimer = null;
      this._onBcMessage = this._onBcMessage.bind(this);
      if (this.bc) this.bc.addEventListener('message', this._onBcMessage);

      this._setupControls();
    }

    setEditor(docEditor) {
      this.docEditor = docEditor;
    }

    destroy() {
      clearInterval(this._heartbeatTimer);
      this._leavePresence();
      if (this.bc) this.bc.removeEventListener('message', this._onBcMessage);
      if (this.bc) this.bc.close();
    }

    // -- connectMockServer API expected by the vendored CryptPad wrapper (see api.js's `o` class) --

    getParticipants() {
      const presence = this._readPresence();
      const ids = Object.keys(presence)
        .filter((id) => Date.now() - presence[id].ts < 20000) // prune stale (crashed/closed) tabs
        .sort();
      let myIndex = 0;
      // `id` must be unique per *connection* (so the same real user in two tabs shows as two rows
      // merged under one name) while `idOriginal` is the per-*person* identity the client actually
      // groups/colors by (see notes/communication_protocol.md) - omitting `idOriginal` made every
      // participant collapse into a single "(N)" row keyed by the shared `undefined` value.
      const list = ids.map((id, i) => {
        if (id === this.clientId) myIndex = i;
        const p = presence[id];
        return { id, idOriginal: p.userId, username: p.userName, indexUser: i, view: p.mode === 'view' };
      });
      this._fakeParticipants.forEach((p, i) => {
        list.push({ id: p.userId, idOriginal: p.userId, username: p.userName, indexUser: ids.length + i, view: false });
      });
      return { list, index: myIndex };
    }

    getInitialChanges() {
      // Joining an already-active session: replay the durable log accumulated so far instead of
      // re-downloading/re-converting the document. Mirrors CryptPad's getInitialChanges()/authChanges.
      const log = this._readLog();
      const changes = [];
      log.forEach((entry) => Array.prototype.push.apply(changes, entry.changes));
      return changes;
    }

    getImageURL() {
      return Promise.resolve('');
    }

    onAuth() {
      const presence = this._readPresence();
      presence[this.clientId] = { userId: this.userId, userName: this.userName, mode: this.mode, ts: Date.now() };
      this._writePresence(presence);
      this._heartbeatTimer = setInterval(() => this._heartbeat(), 5000);
      this._broadcast({ kind: 'presence' });
      this._pushParticipantsIfChanged();
      this._updateStatus();
    }

    _updateStatus() {
      Panel.setStatus(
        `documentId=${this.documentId} · you are ${this.userName} (${this.mode}) · ${this._readLog().length} persisted change(s)`,
      );
    }

    onMessage(msg) {
      Panel.log('out', msg);
      switch (msg.type) {
        case 'isSaveLock':
          // POC: never simulate an in-progress checkpoint blocking edits.
          this._sendToClient('saveLock', { saveLock: false });
          break;
        case 'cursor':
          // Ephemeral: never persisted, just relayed live to whoever else is connected right now.
          // `from` (this tab's connection id) must match the `id` a connectState used for this tab's
          // participant entry, and `userId` (the person) must match that entry's `idOriginal` - the
          // client only draws a labelled/colored foreign cursor for an id it already knows about via
          // connectState (see notes/communication_protocol.md).
          // Used by the "Simulate cursor move" button, see below: keep a short history rather than
          // just the latest value, so the simulated cursor lands somewhere *near* but not exactly on
          // top of this session's current position (see the CURSOR_COLLISION note further down).
          this._cursorHistory.push(msg.cursor);
          if (this._cursorHistory.length > 8) this._cursorHistory.shift();
          this._broadcast({ kind: 'cursor', from: this.clientId, userId: this.userId, cursor: msg.cursor });
          break;
        case 'getLock': {
          // Unlike saveChanges (which has a separate unSaveLock ack), getLock's grant *is* the ack:
          // the requester blocks further local edits until it receives one back. BroadcastChannel
          // never delivers to its own sender, so without an explicit self-delivery here, Alice's own
          // lock request would never be acknowledged and she'd be stuck unable to edit - this was a
          // real bug (see notes/communication_protocol.md).
          //
          // The client also expects `locks` as an id-keyed map of `{time, user, block}` entries (per
          // CryptPad's `getLock()`), not the raw `block` array the *request* carries.
          const uid = (global.crypto && global.crypto.randomUUID) ? global.crypto.randomUUID() : String(Math.random());
          const locks = { [uid]: { time: Date.now(), user: this.userId, block: msg.block && msg.block[0] } };
          this._sendToClient('getLock', { locks });
          this._broadcast({ kind: 'lock', from: this.clientId, locks });
          break;
        }
        case 'unLockDocument':
          this._sendToClient('releaseLock', { locks: {} });
          this._broadcast({ kind: 'releaseLock', from: this.clientId });
          break;
        case 'saveChanges': {
          // The one non-ephemeral message type: append to the durable per-document log (localStorage
          // stands in for what would be Parsec-server-side storage) and broadcast its index to every
          // connected client (self included - the real protocol acks this way too).
          //
          // `msg.changes` is a JSON string holding an array of raw op fragments; OnlyOffice's engine
          // expects to receive them back wrapped as `{docid, change, time, user, useridoriginal}`
          // (this is CryptPad's `parseChanges()` - see notes/communication_protocol.md), so wrap once
          // here at submission time and store/relay the wrapped form from then on.
          const entry = this._appendLog({
            changes: this._parseChanges(msg.changes),
            from: this.clientId,
            userId: this.userId,
            ts: Date.now(),
          });
          // Immediate local ack, unblocks further local edits (mirrors CryptPad's direct unSaveLock
          // reply, sent as soon as the "RPC" to store the patch succeeds - this happens *before* the
          // ordered broadcast loop-back below, exactly like the real client/server pair).
          this._sendToClient('unSaveLock', { index: entry.seq, time: Date.now() });
          this._broadcast({ kind: 'log', seq: entry.seq });
          this._maybeWarnCheckpoint(entry.seq);
          this._updateStatus();
          break;
        }
        case 'auth':
        case 'getMessages':
        case 'forceSaveStart':
        case 'openDocument':
          // Logged above; no reply needed for this POC (see notes/communication_protocol.md).
          break;
      }
    }

    // -- internals --

    _sendToClient(type, extra) {
      const msg = Object.assign({ type }, extra || {});
      Panel.log('in', msg);
      if (this.docEditor) this.docEditor.sendMessageToOO(msg);
    }

    _parseChanges(rawChangesJson) {
      let arr;
      try {
        arr = JSON.parse(rawChangesJson);
      } catch (_e) {
        return [];
      }
      const time = Date.now();
      return arr.map((change) => ({ docid: 'fresh', change: `"${change}"`, time, user: this.userId, useridoriginal: this.userId }));
    }

    _readLog() {
      try {
        return JSON.parse(localStorage.getItem(this.logStorageKey) || '[]');
      } catch (_e) {
        return [];
      }
    }

    _appendLog(entry) {
      const log = this._readLog();
      entry.seq = log.length;
      log.push(entry);
      localStorage.setItem(this.logStorageKey, JSON.stringify(log));
      return entry;
    }

    _readPresence() {
      try {
        return JSON.parse(localStorage.getItem(this.presenceStorageKey) || '{}');
      } catch (_e) {
        return {};
      }
    }

    _writePresence(map) {
      localStorage.setItem(this.presenceStorageKey, JSON.stringify(map));
    }

    _heartbeat() {
      const presence = this._readPresence();
      // prune stale entries left behind by tabs that closed without cleaning up after themselves
      let changed = false;
      Object.keys(presence).forEach((id) => {
        if (id !== this.clientId && Date.now() - presence[id].ts > 20000) {
          delete presence[id];
          changed = true;
        }
      });
      presence[this.clientId] = { userId: this.userId, userName: this.userName, mode: this.mode, ts: Date.now() };
      this._writePresence(presence);
      if (changed) this._broadcast({ kind: 'presence' });
      this._pushParticipantsIfChanged();
    }

    _leavePresence() {
      const presence = this._readPresence();
      delete presence[this.clientId];
      this._writePresence(presence);
      this._broadcast({ kind: 'presence' });
    }

    _broadcast(data) {
      if (this.bc) this.bc.postMessage(data);
    }

    _pushParticipantsIfChanged() {
      const p = this.getParticipants();
      const key = JSON.stringify(p.list.map((u) => u.id));
      if (key === this._lastParticipantsKey) return;
      this._lastParticipantsKey = key;
      this._sendToClient('connectState', { participantsTimestamp: Date.now(), participants: p.list, waitAuth: false });
    }

    _maybeWarnCheckpoint(seq) {
      if (seq > 0 && seq % CHECKPOINT_WARN_INTERVAL === 0) {
        Panel.log(
          'in',
          { type: '(info) checkpoint-threshold', seq, note: 'a real server would snapshot the document here and trim the log' },
          'not a real OnlyOffice message',
        );
      }
    }

    _onBcMessage(ev) {
      const data = ev.data || {};
      switch (data.kind) {
        case 'log': {
          const entry = this._readLog()[data.seq];
          // startSaveChanges/endSaveChanges mark the first/last chunk of a (possibly multi-part) save
          // (see notes/onlyoffice_protocol_types.md, "Large saves are chunked"). The POC never splits
          // a save into multiple chunks, so every broadcast is both the first and the last - but the
          // flags must still be sent as `true`: the receiving client's _onSaveChanges buffers `changes`
          // into an internal queue and returns *without applying them* whenever `endSaveChanges` is
          // falsy, waiting for a final chunk that (without this) would never arrive. Omitting these
          // flags was a real bug: edits from one tab silently never appeared in another.
          if (entry) {
            this._sendToClient('saveChanges', {
              changes: entry.changes,
              changesIndex: entry.seq,
              startSaveChanges: true,
              endSaveChanges: true,
            });
          }
          this._updateStatus();
          break;
        }
        case 'presence':
          this._pushParticipantsIfChanged();
          break;
        case 'cursor':
          if (data.from !== this.clientId) {
            this._sendToClient('cursor', {
              messages: [{ cursor: data.cursor, time: Date.now(), user: data.from, useridoriginal: data.userId }],
            });
          }
          break;
        case 'lock':
          if (data.from !== this.clientId) {
            this._sendToClient('getLock', { locks: data.locks });
          }
          break;
        case 'releaseLock':
          if (data.from !== this.clientId) {
            this._sendToClient('releaseLock', { locks: {} });
          }
          break;
      }
    }

    // -- manual "simulate another user" controls, for single-tab testing --

    _setupControls() {
      Panel.addControls([
        {
          label: 'Simulate user joining',
          title: 'Adds a fake participant (no real tab) and pushes an updated connectState to this client',
          onClick: () => {
            const n = this._fakeParticipants.length + 1;
            this._fakeParticipants.push({ userId: `fake-bob-${n}`, userName: `Bob (simulated #${n})` });
            this._pushParticipantsIfChanged();
          },
        },
        {
          label: 'Simulate user leaving',
          title: 'Removes the most recently simulated fake participant',
          onClick: () => {
            this._fakeParticipants.pop();
            this._pushParticipantsIfChanged();
          },
        },
        {
          label: 'Simulate cursor move',
          title:
            'Sends an ephemeral cursor message as if from a simulated user (auto-joins one first if ' +
            'needed - the client only draws a cursor for a participant it already knows about). Reuses ' +
            'one of this session’s own past real cursor positions (not the current one - see the ' +
            'CURSOR_COLLISION note in the code/notes/communication_protocol.md for why), since - like ' +
            'saveChanges - the position value is an OnlyOffice-internal encoding we can’t hand-craft.',
          onClick: () => {
            if (!this._fakeParticipants.length) {
              this._fakeParticipants.push({ userId: 'fake-bob-1', userName: 'Bob (simulated #1)' });
              this._pushParticipantsIfChanged();
            }
            const from = this._fakeParticipants[this._fakeParticipants.length - 1].userId;
            // CURSOR_COLLISION: deliberately avoid reusing this session's *current* cursor position.
            // A real second user's cursor essentially never lands on your exact current character
            // offset - but ours would, every time, if we always reused the latest value, and OnlyOffice's
            // own vendored engine has a reproducible edge case there: local typing silently stops being
            // processed (no message is even attempted - confirmed by watching this very panel stay
            // empty) until the tab loses and regains focus and the cursor is moved. That's internal to
            // the minified sdkjs engine, not something this mock server's protocol layer can fix, so we
            // just avoid manufacturing the exact-overlap case in the first place.
            //
            // OnlyOffice only emits a 'cursor' message on explicit repositioning (a click, arrow-key
            // navigation), not per keystroke while typing continuously, and a position can coincidentally
            // repeat across captures (e.g. re-clicking near the same spot) - so "the oldest entry" isn't
            // reliably different from "the current one" either. Search for *any* captured value that
            // actually differs from the current one, and only fall back to skipping (with an explanation)
            // if every capture we have happens to be identical to it.
            const current = this._cursorHistory[this._cursorHistory.length - 1];
            const cursor = this._cursorHistory.find((value) => value !== current);
            if (!cursor) {
              Panel.log(
                'in',
                {
                  type: '(info) cursor-collision-avoided',
                  note:
                    'no captured cursor position differs from this session’s current one yet - click ' +
                    'around the document a bit (not just type) then try again',
                },
                'not a real OnlyOffice message',
              );
              return;
            }
            this._sendToClient('cursor', {
              messages: [{ cursor, time: Date.now(), user: from, useridoriginal: from }],
            });
          },
        },
        {
          label: 'Replay last edit as Bob',
          title:
            "Non-ephemeral saveChanges can't be hand-crafted (they carry OnlyOffice's internal binary op " +
            'format) - this rebroadcasts the last genuine persisted change under a fake identity to show ' +
            'the message flow. To see real independent edits, open the same document in a second tab.',
          onClick: () => {
            const log = this._readLog();
            if (!log.length) return;
            const last = log[log.length - 1];
            const entry = this._appendLog({
              changes: last.changes,
              from: 'fake-bob',
              userId: 'fake-bob',
              ts: Date.now(),
              replayOf: last.seq,
            });
            // Unlike a genuine local edit (already applied optimistically by the editor before it
            // even reaches onMessage), this synthetic entry has no local origin to apply it - and
            // BroadcastChannel never delivers back to its own sender - so this tab needs an explicit
            // delivery too, or "Bob"'s replayed edit would only show up in *other* tabs.
            // See the 'log' case in _onBcMessage for why startSaveChanges/endSaveChanges must be true.
            this._sendToClient('saveChanges', {
              changes: entry.changes,
              changesIndex: entry.seq,
              startSaveChanges: true,
              endSaveChanges: true,
            });
            this._broadcast({ kind: 'log', seq: entry.seq });
            this._maybeWarnCheckpoint(entry.seq);
            this._updateStatus();
          },
        },
      ]);
    }
  }

  global.OnlyOfficeMockServer = {
    create(options) {
      return new MockServer(options);
    },
  };
})(window);
