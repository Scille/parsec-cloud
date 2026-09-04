// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Pure protocol translator between the OnlyOffice editor protocol (RFC §2.2,
// `docs/rfcs/1030-collaborative-editics.md`) and the Parsec editics protocol
// (the SSE+RPC protocol implemented by `server/parsec/components/editics.py`).
//
// This file is **pure and side-effect free**: it performs no I/O and touches no
// browser/node globals (no `fetch`, no `EventSource`, no `document`, no
// `crypto`, no `btoa`/`atob`, no `console`, no timers). The only way it observes
// or affects the world is through the injected `EditicsCapabilities` object
// (see §4.1 of `todo/step_2.md`) and the values passed to / returned from its
// methods. This is what makes it loadable into PyMiniRacer as-is and
// deterministic in tests.
//
// It is plain JavaScript with JSDoc types (no TypeScript, no build step on the
// test path — see `todo/step_2.md` §2.3). It is an ES module: it `export`s
// `EditicsTranslator` and has no imports from the rest of the parsec client, so
// it can be loaded by both the browser bundle and the PyMiniRacer test bridge.
//
// OnlyOffice event/field names are kept verbatim (they are *not* renamed even
// when they are known-bad): this keeps the translation layer thin (RFC §2).

// ---------------------------------------------------------------------------
// Injected capabilities (§4.1)
// ---------------------------------------------------------------------------

/**
 * @typedef {Object} EditicsCapabilities
 * @property {(deviceIdHex: string) => (string|Promise<string>)} resolveUserName
 *    Resolve a deviceId (hex) to a display name. The server is NOT trusted
 *    for names (RFC §3.3); the client resolves them locally (libparsec lookup
 *    in the browser, a fixed map in tests).
 * @property {(deviceIdHex: string) => (string|Promise<string>)} [resolveUserId]
 *    Resolve a deviceId (hex) to the per-person userId used to build
 *    OnlyOffice's composite `<userId><indexUser>` id. Defaults to the deviceId
 *    hex when omitted.
 * @property {(plain: Uint8Array) => Uint8Array} encrypt
 *    Encrypt a cleartext payload into an opaque blob. SYNC, deterministic.
 *    The browser injects real libparsec sealing later; tests inject a
 *    key-prefix fake. Operates on `Uint8Array` (no base64 here — base64 is the
 *    transport encoding handled by the connection layer / pydantic).
 * @property {(cipher: Uint8Array) => Uint8Array} decrypt
 *    Inverse of `encrypt`. SYNC.
 */

// ---------------------------------------------------------------------------
// OnlyOffice protocol types (payload shapes, RFC §2.2 / captured sessions)
// ---------------------------------------------------------------------------
//
// OnlyOffice wraps each event as `{ type, payload }`; the translator works on
// the `payload` (the `msg`/`event` objects the editor's `connectMockServer`
// API hands to `onMessage` / `sendMessageToOO`). These `@typedef`s document
// the payload shapes observed in the captured sessions
// (`docs/rfcs/1030-collaborative-editics/oo_example_session.md`).

/**
 * @typedef {Object} OOParticipantEntry
 * @property {string} id - `<userId><indexUser>` composite id (the editor matches
 *   its own `_userId = editorConfig.user.id + indexUser` against this).
 * @property {string} idOriginal - integrator-provided user id (the userId).
 * @property {string} username - display name.
 * @property {number} indexUser - participant index (order of arrival).
 * @property {boolean} view - viewer (read-only).
 * @property {string} [connectionId] - underlying connection id (= sessionId).
 * @property {boolean} [isCloseCoAuthoring]
 * @property {boolean} [isLiveViewer]
 * @property {boolean} [encrypted]
 */

/**
 * @typedef {Object} OOClientEventAuth
 * @property {'auth'} type
 * @property {string} docid
 * @property {string} token
 * @property {{id:string, username:string, firstname:string|null, lastname:string|null, indexUser:number}} user
 * @property {number} editorType
 * @property {number} lastOtherSaveTime
 * @property {Array} block
 * @property {string|null} sessionId
 * @property {number|null} sessionTimeConnect
 * @property {number} sessionTimeIdle
 * @property {number} documentFormatSave
 * @property {boolean} isCloseCoAuthoring
 * @property {Object|null} openCmd
 * @property {string} lang
 * @property {string} mode
 * @property {{edit:boolean, review:boolean}} permissions
 * @property {boolean} encrypted
 * @property {boolean} IsAnonymousUser
 * @property {number} timezoneOffset
 * @property {string|null} headingsColor
 * @property {string} coEditingMode
 * @property {string} jwtOpen
 * @property {string} [jwtSession]
 * @property {number} time
 * @property {boolean} supportAuthChangesAck
 */

/**
 * @typedef {Object} OOClientEventMessage
 * @property {'message'} type
 * @property {string} message
 */

/**
 * @typedef {Object} OOClientEventCursor
 * @property {'cursor'} type
 * @property {string} cursor - opaque OnlyOffice internal string.
 */

/**
 * @typedef {Object} OOClientEventGetLock
 * @property {'getLock'} type
 * @property {Array} block - opaque block descriptors (shape depends on editor).
 */

/**
 * @typedef {Object} OOClientEventIsSaveLock
 * @property {'isSaveLock'} type
 * @property {number} syncChangesIndex
 */

/**
 * @typedef {Object} OOClientEventSaveChanges
 * @property {'saveChanges'} type
 * @property {string|Array} changes - JSON-encoded string (default mode) of an
 *   array of opaque op fragments, or a real array in binary-changes mode.
 * @property {boolean} startSaveChanges
 * @property {boolean} endSaveChanges
 * @property {boolean} [isCoAuthoring]
 * @property {boolean} [isExcel]
 * @property {number|null} [deleteIndex]
 * @property {string|null} [excelAdditionalInfo]
 * @property {boolean} [unlock]
 * @property {boolean} [releaseLocks]
 * @property {number} [reSave]
 */

/**
 * @typedef {Object} OOClientEventUnSaveLock
 * @property {'unSaveLock'} type
 */

/**
 * @typedef {Object} OOClientEventUnLockDocument
 * @property {'unLockDocument'} type
 * @property {boolean} isSave
 * @property {boolean} unlock
 * @property {number|null} [deleteIndex]
 * @property {boolean} [releaseLocks]
 */

/**
 * @typedef {Object} OOClientEventClose
 * @property {'close'} type
 */

/**
 * @typedef {Object} OOClientEventAuthChangesAck
 * @property {'authChangesAck'} type
 */

/**
 * @typedef {Object} OOClientEventGetMessages
 * @property {'getMessages'} type
 */

/**
 * @typedef {Object} OOClientEventOpenDocument
 * @property {'openDocument'} type
 * @property {Object} message
 */

/**
 * @typedef {Object} OOClientEventClientLog
 * @property {'clientLog'} type
 * @property {string} level
 * @property {string} msg
 */

/**
 * @typedef {Object} OOClientEventExtendSession
 * @property {'extendSession'} type
 * @property {number} idletime
 */

/**
 * @typedef {Object} OOClientEventForceSaveStart
 * @property {'forceSaveStart'} type
 */

/**
 * @typedef {Object} OOClientEventRpc
 * @property {'rpc'} type
 * @property {number} responseKey
 * @property {Object} data
 */

/**
 * @typedef {Object} OOClientEventSaveDone
 *   Editics addition (no OO equivalent): the host page posts it after a vlob
 *   upload so the server bumps the session's allowed vlob version.
 * @property {'saveDone'} type
 * @property {number} savedUpToIndex
 * @property {number} newVersion
 */

/**
 * @typedef {OOClientEventAuth|OOClientEventMessage|OOClientEventCursor|OOClientEventGetLock|
 *   OOClientEventIsSaveLock|OOClientEventSaveChanges|OOClientEventUnSaveLock|OOClientEventUnLockDocument|
 *   OOClientEventClose|OOClientEventAuthChangesAck|OOClientEventGetMessages|OOClientEventOpenDocument|
 *   OOClientEventClientLog|OOClientEventExtendSession|OOClientEventForceSaveStart|OOClientEventRpc|
 *   OOClientEventSaveDone} OOClientEvent
 */

/**
 * @typedef {Object} OOServerEventAuth
 * @property {'auth'} type
 * @property {number} result - 1 = success
 * @property {string} sessionId
 * @property {number} sessionTimeConnect
 * @property {OOParticipantEntry[]} participants
 * @property {Array} [locks]
 * @property {number} indexUser
 * @property {boolean} [hasForgotten]
 * @property {string} [jwt]
 * @property {string} [g_cAscSpellCheckUrl]
 * @property {string} [buildVersion]
 * @property {number} [buildNumber]
 * @property {number} [licenseType]
 * @property {Object} [settings]
 * @property {number} [openedAt]
 */

/**
 * @typedef {Object} OOServerEventWaitAuth
 * @property {'waitAuth'} type
 * @property {OOParticipantEntry} lockDocument - the established editor holding
 *   the auth lock (the newcomer must wait for it to release).
 */

/**
 * @typedef {Object} OOServerEventConnectState
 * @property {'connectState'} type
 * @property {number} participantsTimestamp
 * @property {OOParticipantEntry[]} participants
 * @property {boolean} waitAuth
 */

/**
 * @typedef {Object} OOServerEventAuthChanges
 * @property {'authChanges'} type
 * @property {Array<{docid:string, change:string, time:number, user:string, useridoriginal:string}>} changes
 */

/**
 * @typedef {Object} OOServerEventMessage
 * @property {'message'} type
 * @property {Array<{docid:string, message:string, time:number, user:string, useridoriginal:string, username:string}>} messages
 */

/**
 * @typedef {Object} OOServerEventCursor
 * @property {'cursor'} type
 * @property {Array<{cursor:string, time:number, user:string, useridoriginal:string}>} messages
 */

/**
 * @typedef {Object} OOServerEventGetLock
 * @property {'getLock'} type
 * @property {Record<string, {time:number, user:string, block:*}>} locks
 */

/**
 * @typedef {Object} OOServerEventReleaseLock
 * @property {'releaseLock'} type
 * @property {Array<{block:*, user:string, time:number, changes:null}>} locks
 */

/**
 * @typedef {Object} OOServerEventSaveChanges
 * @property {'saveChanges'} type
 * @property {Array<{docid:string, change:string, time:number, user:string, useridoriginal:string}>|null} changes
 * @property {number} changesIndex
 * @property {number} syncChangesIndex
 * @property {boolean} endSaveChanges
 * @property {Array<{block:*, user:string, time:number, changes:*}>} [locks]
 * @property {string} [excelAdditionalInfo]
 */

/**
 * @typedef {Object} OOServerEventSavePartChanges
 * @property {'savePartChanges'} type
 * @property {number} changesIndex
 * @property {number} syncChangesIndex
 */

/**
 * @typedef {Object} OOServerEventSaveLock
 * @property {'saveLock'} type
 * @property {boolean} saveLock
 */

/**
 * @typedef {Object} OOServerEventUnSaveLock
 * @property {'unSaveLock'} type
 * @property {number} index
 * @property {number} time
 * @property {number} syncChangesIndex
 */

/**
 * @typedef {Object} OOServerEventDrop
 * @property {'drop'} type
 * @property {number} code
 * @property {string} description
 */

/**
 * @typedef {Object} OOServerEventWarning
 * @property {'warning'} type
 * @property {number} code
 * @property {string} message
 */

/** @typedef {OOServerEventAuth|OOServerEventWaitAuth|OOServerEventConnectState|OOServerEventAuthChanges|
 *   OOServerEventMessage|OOServerEventCursor|OOServerEventGetLock|OOServerEventReleaseLock|
 *   OOServerEventSaveChanges|OOServerEventSavePartChanges|OOServerEventSaveLock|OOServerEventUnSaveLock|
 *   OOServerEventDrop|OOServerEventWarning} OOServerEvent */

// ---------------------------------------------------------------------------
// Editics protocol types (mirror `server/parsec/components/editics.py` exactly)
// ---------------------------------------------------------------------------
//
// Field-name parity with the server is a hard requirement (todo §4.2): the test
// harness serializes `EditicsClientEvent` to JSON and POSTs it to the server,
// and parses the server's JSON reply into `EditicsServerEvent`. Any name drift
// breaks the round-trip. Encrypted `bytes` fields are `Uint8Array` in JS (they
// are base64-encoded only at the JSON wire boundary by the connection layer /
// pydantic — `protocol.js` never sees base64).

/**
 * @typedef {Object} EditicsParticipantEntry
 * @property {number} indexUser
 * @property {string} deviceId - DeviceID hex
 * @property {boolean} view
 */

/**
 * @typedef {Object} EditicsClientEventAuth
 * @property {'auth'} type
 * @property {number} indexUser - -1 on first open
 * @property {number} editorType
 * @property {number} vlobVersion
 */

/**
 * @typedef {Object} EditicsClientEventAuthChangesAck
 * @property {'authChangesAck'} type
 */

/**
 * @typedef {Object} EditicsClientEventMessage
 * @property {'message'} type
 * @property {Uint8Array} encryptedMessage
 */

/**
 * @typedef {Object} EditicsClientEventCursor
 * @property {'cursor'} type
 * @property {Uint8Array} encryptedCursor
 */

/**
 * @typedef {Object} EditicsClientEventGetLock
 * @property {'getLock'} type
 * @property {Array} block
 */

/**
 * @typedef {Object} EditicsClientEventIsSaveLock
 * @property {'isSaveLock'} type
 * @property {number} syncChangesIndex
 */

/**
 * @typedef {Object} EditicsClientEventSaveChanges
 * @property {'saveChanges'} type
 * @property {Uint8Array[]} encryptedChanges
 * @property {boolean} startSaveChanges
 * @property {boolean} endSaveChanges
 * @property {number|null} [deleteIndex]
 * @property {Object<string,*>|null} [excel_info]
 * @property {Uint8Array|null} [encryptedCursor]
 * @property {boolean} [releaseLocks]
 */

/**
 * @typedef {Object} EditicsClientEventUnSaveLock
 * @property {'unSaveLock'} type
 */

/**
 * @typedef {Object} EditicsClientEventUnLockDocument
 * @property {'unLockDocument'} type
 * @property {boolean} isSave
 * @property {boolean} unlock
 * @property {number|null} [deleteIndex]
 * @property {boolean} [releaseLocks]
 */

/**
 * @typedef {Object} EditicsClientEventClose
 * @property {'close'} type
 */

/**
 * @typedef {Object} EditicsClientEventSaveDone
 * @property {'saveDone'} type
 * @property {number} savedUpToIndex
 * @property {number} newVersion
 */

/** @typedef {EditicsClientEventAuth|EditicsClientEventAuthChangesAck|EditicsClientEventMessage|
 *   EditicsClientEventCursor|EditicsClientEventGetLock|EditicsClientEventIsSaveLock|EditicsClientEventSaveChanges|
 *   EditicsClientEventUnSaveLock|EditicsClientEventUnLockDocument|EditicsClientEventClose|EditicsClientEventSaveDone} EditicsClientEvent */

/**
 * @typedef {Object} EditicsServerEventAuth
 * @property {'auth'} type
 * @property {number} result - 1 = success
 * @property {EditicsParticipantEntry[]} participants
 * @property {number} indexUser
 * @property {string} sessionId
 * @property {number} sessionTimeConnect
 */

/**
 * @typedef {Object} EditicsServerEventAuthRejected
 * @property {'auth'} type
 * @property {number} result - 0 = rejected
 * @property {number} latestAllowedVersion
 */

/**
 * @typedef {Object} EditicsServerEventConnectState
 * @property {'connectState'} type
 * @property {number} participantsTimestamp
 * @property {EditicsParticipantEntry[]} participants
 * @property {boolean} waitAuth
 */

/**
 * @typedef {Object} EditicsServerEventAuthChanges
 * @property {'authChanges'} type
 * @property {Array<[number, Uint8Array]>} changes - (index, encrypted blob)
 */

/**
 * @typedef {Object} EditicsServerEventWaitAuth
 * @property {'waitAuth'} type
 * @property {number} authLockedBy - the indexUser holding the auth lock
 */

/**
 * @typedef {Object} EditicsServerEventMessage
 * @property {'message'} type
 * @property {Array<{time:number, authorIndexUser:number, encryptedMessage:Uint8Array}>} messages
 */

/**
 * @typedef {Object} EditicsServerEventCursor
 * @property {'cursor'} type
 * @property {Array<{time:number, authorIndexUser:number, encryptedCursor:Uint8Array}>} messages
 */

/**
 * @typedef {Object} EditicsServerEventGetLock
 * @property {'getLock'} type
 * @property {Record<string, {time:number, user:number, block:*}>} locks
 */

/**
 * @typedef {Object} EditicsServerEventReleaseLock
 * @property {'releaseLock'} type
 * @property {Array<{block:*, user:number, time:number, changes:null}>} locks
 */

/**
 * @typedef {Object} EditicsServerEventSaveLock
 * @property {'saveLock'} type
 * @property {boolean} saveLock
 */

/**
 * @typedef {Object} EditicsServerEventSaveChanges
 * @property {'saveChanges'} type
 * @property {Array<{time:number, authorIndexUser:number, change:Uint8Array}>} changes
 * @property {number} changesIndex
 * @property {number} syncChangesIndex
 * @property {boolean} endSaveChanges
 * @property {Array<{block:*, user:number, time:number, changes:null}>} [locks]
 * @property {Object<string,*>|null} [excel_info]
 * @property {Uint8Array|null} [encryptedCursor]
 */

/**
 * @typedef {Object} EditicsServerEventSavePartChanges
 * @property {'savePartChanges'} type
 * @property {number} changesIndex
 * @property {number} syncChangesIndex
 */

/**
 * @typedef {Object} EditicsServerEventUnSaveLock
 * @property {'unSaveLock'} type
 * @property {number} index
 * @property {number} time
 * @property {number} syncChangesIndex
 */

/**
 * @typedef {Object} EditicsServerEventDrop
 * @property {'drop'} type
 * @property {number} code
 * @property {string} description
 */

/**
 * @typedef {Object} EditicsServerEventWarning
 * @property {'warning'} type
 * @property {number} code
 * @property {string} message
 */

/** @typedef {EditicsServerEventAuth|EditicsServerEventAuthRejected|EditicsServerEventConnectState|
 *   EditicsServerEventAuthChanges|EditicsServerEventWaitAuth|EditicsServerEventMessage|EditicsServerEventCursor|
 *   EditicsServerEventGetLock|EditicsServerEventReleaseLock|EditicsServerEventSaveLock|EditicsServerEventSaveChanges|
 *   EditicsServerEventSavePartChanges|EditicsServerEventUnSaveLock|EditicsServerEventDrop|
 *   EditicsServerEventWarning} EditicsServerEvent */

// ---------------------------------------------------------------------------
// Small byte <-> string helpers (the only "encoding" the translator does; it
// never touches base64 — that is the transport layer's job).
// ---------------------------------------------------------------------------

/**
 * UTF-8 encode a JS string into a `Uint8Array`. Pure (no `TextEncoder` global
 * dependency, so it works in PyMiniRacer which may not expose it).
 * @param {string} s
 * @returns {Uint8Array}
 */
function strToBytes(s) {
  // Encode UTF-8 manually. This is small and dependency-free, which keeps the
  // translator loadable in any JS runtime (V8 isolate / browser).
  const out = [];
  for (let i = 0; i < s.length; i++) {
    let c = s.charCodeAt(i);
    if (c < 0x80) {
      out.push(c);
    } else if (c < 0x800) {
      out.push(0xc0 | (c >> 6));
      out.push(0x80 | (c & 0x3f));
    } else if (c >= 0xd800 && c <= 0xdbff) {
      // Surrogate pair (U+10000..U+10FFFF).
      const next = s.charCodeAt(++i);
      c = 0x10000 + ((c - 0xd800) << 10) + (next - 0xdc00);
      out.push(0xf0 | (c >> 18));
      out.push(0x80 | ((c >> 12) & 0x3f));
      out.push(0x80 | ((c >> 6) & 0x3f));
      out.push(0x80 | (c & 0x3f));
    } else {
      out.push(0xe0 | (c >> 12));
      out.push(0x80 | ((c >> 6) & 0x3f));
      out.push(0x80 | (c & 0x3f));
    }
  }
  return new Uint8Array(out);
}

/**
 * UTF-8 decode a `Uint8Array` into a JS string. Pure.
 * @param {Uint8Array} bytes
 * @returns {string}
 */
function bytesToStr(bytes) {
  let s = '';
  let i = 0;
  while (i < bytes.length) {
    const b = bytes[i++];
    if (b < 0x80) {
      s += String.fromCharCode(b);
    } else if (b < 0xe0) {
      s += String.fromCharCode(((b & 0x1f) << 6) | (bytes[i++] & 0x3f));
    } else if (b < 0xf0) {
      s += String.fromCharCode(((b & 0x0f) << 12) | ((bytes[i++] & 0x3f) << 6) | (bytes[i++] & 0x3f));
    } else {
      const cp = ((b & 0x07) << 18) | ((bytes[i++] & 0x3f) << 12) | ((bytes[i++] & 0x3f) << 6) | (bytes[i++] & 0x3f);
      // Convert to a surrogate pair.
      const adj = cp - 0x10000;
      s += String.fromCharCode(0xd800 + (adj >> 10), 0xdc00 + (adj & 0x3ff));
    }
  }
  return s;
}

// ---------------------------------------------------------------------------
// EditicsTranslator
// ---------------------------------------------------------------------------

/**
 * @typedef {Object} EditicsTranslatorConfig
 * @property {string} workspaceId - WorkspaceID (VlobID) hex.
 * @property {string} vlobId - VlobID hex of the document.
 * @property {string} deviceIdHex - DeviceID hex of the client's device.
 * @property {string} [userId] - per-person userId to seed the participant table
 *   before the server-assigned `indexUser` arrives (editor calls
 *   `getParticipants` during init). Defaults to the deviceId hex.
 * @property {string} [userName] - display name for the provisional self seed.
 * @property {number} vlobVersion - loaded vlob version (RFC §1.2).
 * @property {number} editorType - 0=Word, 1=Spreadsheet, 2=Presentation, 3=Visio.
 * @property {EditicsCapabilities} capabilities
 */

class EditicsTranslator {
  /**
   * @param {EditicsTranslatorConfig} cfg
   */
  constructor(cfg) {
    this.config = cfg;
    this.capabilities = cfg.capabilities;

    // Provisional `indexUser`; overridden by the server-assigned index on the
    // `auth` reply. 0 is the local seed used by `getParticipants` before the
    // server replies.
    this.indexUser = 0;

    // `indexUser -> { deviceId, userName, userId }` participant table. Seeded
    // with a provisional self entry so the editor's initial `getParticipants`
    // call has someone to show before the server-assigned `indexUser` arrives.
    this._participants = new Map();
    this._participants.set(0, {
      deviceId: cfg.deviceIdHex,
      userName: cfg.userName || cfg.userId || cfg.deviceIdHex,
      userId: cfg.userId || cfg.deviceIdHex,
    });
  }

  // --- OnlyOffice connectMockServer queries (pure sync) ---------------------

  /**
   * @returns {{list:OOParticipantEntry[], index:number}}
   */
  getParticipants() {
    const list = [];
    let index = -1;
    this._participants.forEach((p, indexUser) => {
      list.push({
        id: p.userId + String(indexUser),
        idOriginal: p.userId,
        username: p.userName || p.deviceId,
        indexUser: indexUser,
        view: false,
      });
      if (indexUser === this.indexUser) index = list.length - 1;
    });
    return { list, index };
  }

  /**
   * @returns {Array}
   */
  getInitialChanges() {
    // No durable change log on the client side; the backlog is delivered by
    // the server as `authChanges` SSE events.
    return [];
  }

  /**
   * @returns {Promise<string>}
   */
  getImageURL() {
    // Image URL resolution is done client-side (no server involvement); the
    // host page wires the actual resolution. The translator returns an empty
    // URL (matches the previous client behavior).
    return Promise.resolve('');
  }

  // --- Client event translation (OnlyOffice editor -> editics server) ------

  /**
   * Convert an OnlyOffice client event (editor -> "server") into the editics
   * client event to send to the Parsec server, or `null` if the event has no
   * editics counterpart (RFC §2.2 "ignored" rows).
   *
   * @param {OOClientEvent} oo
   * @returns {Promise<EditicsClientEvent|null>}
   */
  async cookClientEvent(oo) {
    switch (oo && oo.type) {
      case 'auth':
        return {
          type: 'auth',
          indexUser: -1,
          editorType: this.config.editorType,
          vlobVersion: this.config.vlobVersion,
        };
      case 'authChangesAck':
        return { type: 'authChangesAck' };
      case 'message':
        return { type: 'message', encryptedMessage: this._encrypt(strToBytes(String(oo.message))) };
      case 'cursor':
        return { type: 'cursor', encryptedCursor: this._encrypt(strToBytes(String(oo.cursor))) };
      case 'getLock':
        return { type: 'getLock', block: oo.block };
      case 'isSaveLock':
        return { type: 'isSaveLock', syncChangesIndex: oo.syncChangesIndex };
      case 'saveChanges':
        return this._cookSaveChanges(oo);
      case 'unSaveLock':
        return { type: 'unSaveLock' };
      case 'unLockDocument':
        return {
          type: 'unLockDocument',
          isSave: !!oo.isSave,
          unlock: !!oo.unlock,
          deleteIndex: oo.deleteIndex,
          releaseLocks: !!oo.releaseLocks,
        };
      case 'close':
        return { type: 'close' };
      case 'saveDone':
        return { type: 'saveDone', savedUpToIndex: oo.savedUpToIndex, newVersion: oo.newVersion };
      default:
        // Ignored OO events (RFC §2.2 ❌ rows): getMessages, openDocument,
        // clientLog, extendSession, forceSaveStart, rpc, license-related.
        return null;
    }
  }

  /**
   * @param {OOClientEventSaveChanges} oo
   * @returns {EditicsClientEventSaveChanges}
   */
  _cookSaveChanges(oo) {
    // OnlyOffice sends `changes` as a JSON-encoded *string* in default (JSON)
    // mode, e.g. '["66;...","127;..."]' (an array of opaque op fragments).
    // Parse it into an array of fragment strings, then encrypt each fragment
    // independently as `encryptedChanges` (one entry per fragment, RFC §2.2).
    let fragments = [];
    if (oo.changes !== null && oo.changes !== undefined) {
      if (typeof oo.changes === 'string') {
        try {
          const parsed = JSON.parse(oo.changes);
          if (Array.isArray(parsed)) fragments = parsed;
        } catch (_e) {
          fragments = [oo.changes];
        }
      } else if (Array.isArray(oo.changes)) {
        fragments = oo.changes;
      } else {
        fragments = [oo.changes];
      }
    }
    const encryptedChanges = fragments.map((f) => this._encrypt(strToBytes(String(f))));

    // `excelAdditionalInfo` is split (RFC §2.2) into `encryptedCursor` (the
    // `CursorInfo` part, encrypted) and `excel_info` (the cleartext
    // `indexCols`/`indexRows` part). Step 2 keeps the previous passthrough
    // behavior: the whole opaque blob is encrypted as the cursor; `excel_info`
    // is null (real parsing is deferred to a later step).
    let encryptedCursor = null;
    if (oo.excelAdditionalInfo !== null && oo.excelAdditionalInfo !== undefined) {
      encryptedCursor = this._encrypt(strToBytes(String(oo.excelAdditionalInfo)));
    }
    return {
      type: 'saveChanges',
      encryptedChanges,
      startSaveChanges: !!oo.startSaveChanges,
      endSaveChanges: !!oo.endSaveChanges,
      deleteIndex: oo.deleteIndex,
      // `excel_info` mirrors the server pydantic field name (snake_case).
      // eslint-disable-next-line camelcase
      excel_info: null,
      encryptedCursor,
      releaseLocks: !!oo.releaseLocks,
    };
  }

  // --- Server event translation (editics server -> OnlyOffice editor) -------

  /**
   * Convert an editics server event (from the Parsec server) into the OnlyOffice
   * server event to push to the editor, or `null` if it should not be forwarded.
   *
   * @param {EditicsServerEvent} editics
   * @returns {Promise<OOServerEvent|null>}
   */
  async cookServerEvent(editics) {
    switch (editics && editics.type) {
      case 'auth':
        return this._cookServerAuth(editics);
      case 'waitAuth':
        return this._cookWaitAuth(editics);
      case 'connectState':
        return this._cookConnectState(editics);
      case 'authChanges':
        return this._cookAuthChanges(editics);
      case 'message':
        return this._cookMessage(editics);
      case 'cursor':
        return this._cookCursor(editics);
      case 'getLock':
        return this._cookGetLock(editics);
      case 'releaseLock':
        return this._cookReleaseLock(editics);
      case 'saveChanges':
        return this._cookServerSaveChanges(editics);
      case 'savePartChanges':
        return {
          type: 'savePartChanges',
          changesIndex: editics.changesIndex,
          syncChangesIndex: editics.syncChangesIndex,
        };
      case 'saveLock':
        return { type: 'saveLock', saveLock: !!editics.saveLock };
      case 'unSaveLock':
        return {
          type: 'unSaveLock',
          index: editics.index,
          time: editics.time,
          syncChangesIndex: editics.syncChangesIndex,
        };
      case 'drop':
        return { type: 'drop', code: editics.code, description: editics.description };
      case 'warning':
        return { type: 'warning', code: editics.code, message: editics.message };
      default:
        // Unknown server event: do not forward.
        return null;
    }
  }

  /**
   * On a successful `auth` (result: 1) the translator sets `indexUser`,
   * rebuilds the participant table from `data.participants` (resolving names
   * via `resolveUserName`) and produces an OO `connectState` event — the mock
   * server never sent an OO `auth` event; the editor gets participants via
   * `connectState`. On rejection (result: 0) the translator produces nothing
   * to forward (the rejection is handled by `main.js` / the test).
   * @param {EditicsServerEventAuth|EditicsServerEventAuthRejected} editics
   * @returns {Promise<OOServerEvent|null>}
   */
  async _cookServerAuth(editics) {
    if (editics.result !== 1) {
      return null;
    }
    this.indexUser = editics.indexUser;
    // The server's participant list is authoritative: drop the provisional
    // self seed and rebuild from it.
    this._participants.clear();
    await this._mergeParticipants(editics.participants || []);
    return {
      type: 'connectState',
      participantsTimestamp: Date.now(),
      participants: this._onlyofficeParticipants(),
      waitAuth: false,
    };
  }

  /**
   * @param {EditicsServerEventWaitAuth} editics
   * @returns {Promise<OOServerEventWaitAuth|null>}
   */
  async _cookWaitAuth(editics) {
    // Ensure the holder is in the participant table so `lockDocument` is
    // well-formed (the server may have broadcast a `connectState{waitAuth:true}`
    // carrying the holder before this RPC reply reached the newcomer).
    const holderIndex = editics.authLockedBy;
    if (!this._participants.has(holderIndex)) {
      await this._mergeParticipants([{ indexUser: holderIndex, deviceId: this._deviceId(holderIndex) }]);
    }
    const holder = this._participants.get(holderIndex);
    if (!holder) {
      return null;
    }
    return {
      type: 'waitAuth',
      lockDocument: this._onlyofficeParticipantEntry(holder, holderIndex),
    };
  }

  /**
   * @param {EditicsServerEventConnectState} editics
   * @returns {Promise<OOServerEventConnectState>}
   */
  async _cookConnectState(editics) {
    await this._mergeParticipants(editics.participants || []);
    return {
      type: 'connectState',
      participantsTimestamp: editics.participantsTimestamp,
      participants: this._onlyofficeParticipants(),
      waitAuth: !!editics.waitAuth,
    };
  }

  /**
   * @param {EditicsServerEventAuthChanges} editics
   * @returns {OOServerEventAuthChanges}
   */
  _cookAuthChanges(editics) {
    const docid = `${this.config.workspaceId}/${this.config.vlobId}`;
    const changes = (editics.changes || []).map((entry) => {
      const idx = entry[0];
      const blob = entry[1];
      const userId = this._userId(idx);
      return {
        docid,
        change: JSON.stringify(bytesToStr(this._decrypt(blob))),
        time: 0,
        user: userId + String(idx),
        useridoriginal: userId,
      };
    });
    return { type: 'authChanges', changes };
  }

  /**
   * @param {EditicsServerEventMessage} editics
   * @returns {OOServerEventMessage}
   */
  _cookMessage(editics) {
    const messages = (editics.messages || []).map((m) => {
      const userId = this._userId(m.authorIndexUser);
      return {
        message: bytesToStr(this._decrypt(m.encryptedMessage)),
        time: m.time,
        user: userId + String(m.authorIndexUser),
        useridoriginal: userId,
        username: this._userName(m.authorIndexUser),
      };
    });
    return { type: 'message', messages };
  }

  /**
   * @param {EditicsServerEventCursor} editics
   * @returns {OOServerEventCursor}
   */
  _cookCursor(editics) {
    const messages = (editics.messages || []).map((m) => {
      const userId = this._userId(m.authorIndexUser);
      return {
        cursor: bytesToStr(this._decrypt(m.encryptedCursor)),
        time: m.time,
        user: userId + String(m.authorIndexUser),
        useridoriginal: userId,
      };
    });
    return { type: 'cursor', messages };
  }

  /**
   * @param {EditicsServerEventGetLock} editics
   * @returns {OOServerEventGetLock}
   */
  _cookGetLock(editics) {
    const ooLocks = {};
    for (const key in editics.locks || {}) {
      const lock = editics.locks[key];
      const userId = this._userId(lock.user);
      ooLocks[key] = { time: lock.time, user: userId + String(lock.user), block: lock.block };
    }
    return { type: 'getLock', locks: ooLocks };
  }

  /**
   * @param {EditicsServerEventReleaseLock} editics
   * @returns {OOServerEventReleaseLock}
   */
  _cookReleaseLock(editics) {
    const locks = (editics.locks || []).map((lock) => {
      const userId = this._userId(lock.user);
      return { block: lock.block, user: userId + String(lock.user), time: lock.time, changes: null };
    });
    return { type: 'releaseLock', locks };
  }

  /**
   * @param {EditicsServerEventSaveChanges} editics
   * @returns {OOServerEventSaveChanges}
   */
  _cookServerSaveChanges(editics) {
    const docid = `${this.config.workspaceId}/${this.config.vlobId}`;
    const changes = (editics.changes || []).map((c) => {
      const userId = this._userId(c.authorIndexUser);
      return {
        docid,
        change: JSON.stringify(bytesToStr(this._decrypt(c.change))),
        time: c.time,
        user: userId + String(c.authorIndexUser),
        useridoriginal: userId,
      };
    });
    const locks = (editics.locks || []).map((lock) => {
      const userId = this._userId(lock.user);
      return { block: lock.block, user: userId + String(lock.user), time: lock.time, changes: lock.changes };
    });
    /** @type {OOServerEventSaveChanges} */
    const oo = {
      type: 'saveChanges',
      changes,
      changesIndex: editics.changesIndex,
      syncChangesIndex: editics.syncChangesIndex,
      endSaveChanges: !!editics.endSaveChanges,
      startSaveChanges: true,
      locks,
    };
    if (editics.encryptedCursor !== null && editics.encryptedCursor !== undefined) {
      oo.excelAdditionalInfo = bytesToStr(this._decrypt(editics.encryptedCursor));
    }
    return oo;
  }

  // --- Participant table helpers -------------------------------------------

  /**
   * Merge the server's participant entries into the local table, resolving
   * names/userIds via the injected capabilities (the server is NOT trusted for
   * names, RFC §3.3).
   * @param {EditicsParticipantEntry[]} participants
   * @returns {Promise<void>}
   */
  async _mergeParticipants(participants) {
    for (const p of participants) {
      if (!this._participants.has(p.indexUser)) {
        let userName = p.deviceId;
        let userId = p.deviceId;
        try {
          if (this.capabilities.resolveUserName) {
            const resolved = await this.capabilities.resolveUserName(p.deviceId);
            if (resolved) userName = resolved;
          }
        } catch (_e) {
          // Fall back to the device id (the server is not trusted for names).
        }
        try {
          if (this.capabilities.resolveUserId) {
            const resolved = await this.capabilities.resolveUserId(p.deviceId);
            if (resolved) userId = resolved;
          }
        } catch (_e) {
          // Fall back to the device id.
        }
        this._participants.set(p.indexUser, { deviceId: p.deviceId, userName, userId });
      } else {
        // Backfill the userId if it wasn't resolved the first time but is now.
        const existing = this._participants.get(p.indexUser);
        if (existing && (!existing.userId || existing.userId === existing.deviceId) && this.capabilities.resolveUserId) {
          try {
            const resolved = await this.capabilities.resolveUserId(p.deviceId);
            if (resolved) existing.userId = resolved;
          } catch (_e) {
            /* ignore */
          }
        }
      }
    }
  }

  /**
   * @returns {OOParticipantEntry[]}
   */
  _onlyofficeParticipants() {
    const list = [];
    this._participants.forEach((p, indexUser) => {
      list.push(this._onlyofficeParticipantEntry(p, indexUser));
    });
    return list;
  }

  /**
   * @param {{deviceId:string, userName:string, userId:string}} p
   * @param {number} indexUser
   * @returns {OOParticipantEntry}
   */
  _onlyofficeParticipantEntry(p, indexUser) {
    return {
      id: p.userId + String(indexUser),
      idOriginal: p.userId,
      username: p.userName || p.deviceId,
      indexUser: indexUser,
      view: false,
    };
  }

  /**
   * @param {number} indexUser
   * @returns {string}
   */
  _userName(indexUser) {
    const p = this._participants.get(indexUser);
    return p ? p.userName : String(indexUser);
  }

  /**
   * The per-person userId for `user`/`useridoriginal` fields (matches the
   * editor's `_userId = userId + indexUser`).
   * @param {number} indexUser
   * @returns {string}
   */
  _userId(indexUser) {
    const p = this._participants.get(indexUser);
    return p ? p.userId || p.deviceId : String(indexUser);
  }

  /**
   * Best-effort deviceId for a participant index that may not be in the table
   * yet (used to seed the auth-lock holder before its `connectState` arrives).
   * @param {number} indexUser
   * @returns {string}
   */
  _deviceId(indexUser) {
    const p = this._participants.get(indexUser);
    return p ? p.deviceId : String(indexUser);
  }

  // --- Encryption (sync, capability-injected) -------------------------------

  /** @param {Uint8Array} plain @returns {Uint8Array} */
  _encrypt(plain) {
    return this.capabilities.encrypt(plain);
  }

  /** @param {Uint8Array} cipher @returns {Uint8Array} */
  _decrypt(cipher) {
    return this.capabilities.decrypt(cipher);
  }
}

export { EditicsTranslator };
