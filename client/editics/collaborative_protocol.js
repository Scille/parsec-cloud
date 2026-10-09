// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Here we define `EditicsTranslator` which works as an interface between the
// OnlyOffice client (speaking the OnlyOffice protocol) and the Parsec client/server
// communication (speaking the Editics protocol).
//
// The Editics protocol is basically the OnlyOffice protocol plus an encryption
// layer to protect the sensitive fields (e.g. document modification).
//
// Note this file is tested on both sides:
// - the client unit tests load it into a Node VM (see
//   client/tests/unit/specs/testEditicsClient.spec.ts),
// - the server tests load it into an embedded V8 runtime controlled from Python
//   (using PyMiniRacer, see server/tests/editics/test_records.py), so that they can
//   simulate the behavior of an actual OnlyOffice client sending and receiving
//   OnlyOffice protocol events that are then translated by this file and sent to
//   the server.

/**
 * @import {
 *   OOClientEvent,
 *   OOClientEventSaveChanges,
 *   OOParticipantEntry,
 *   OOServerEvent,
 *   OOServerEventAuth,
 *   OOServerEventAuthChanges,
 *   OOServerEventConnectState,
 *   OOServerEventCursor,
 *   OOServerEventGetLock,
 *   OOServerEventMessage,
 *   OOServerEventReleaseLock,
 *   OOServerEventSaveChanges,
 *   OOServerEventWaitAuth
 * } from 'onlyoffice-editor'
 */

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
// Used by the translation cases currently disabled below (re-enabled as the
// server side of the collaborative mode lands).
// eslint-disable-next-line @typescript-eslint/no-unused-vars
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
 * @typedef {Object} LibparsecCapabilities
 *    Injected capabilities that are expected to be provided by libparsec.
 * @property {(deviceIdHex: string) => string | Promise<string | undefined>} getHumanLabelFromDeviceId
 *    Resolve the display name of a device id (sync or async). Used to build
 *    the OnlyOffice participant entries (the server is NOT trusted for
 *    names).
 * @property {(plain: Uint8Array) => Uint8Array} encrypt
 *    Encrypt a cleartext payload into an opaque blob. SYNC, deterministic.
 *    The browser injects real libparsec sealing later; tests inject a
 *    key-prefix fake. Operates on `Uint8Array` (no base64 here — base64 is the
 *    transport encoding handled by the connection layer / pydantic).
 * @property {(cipher: Uint8Array) => Uint8Array} decrypt
 *    Inverse of `encrypt`. SYNC.
 *
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
 * @property {LibparsecCapabilities} capabilities
 */

/**
 * Side-effect-free translation layer between OnlyOffice and Editics protocols.
 */
class EditicsTranslator {
  /**
   * @param {EditicsTranslatorConfig} cfg
   */
  constructor(cfg) {
    /** @type {EditicsTranslatorConfig} */
    this.config = cfg;
    /** @type {LibparsecCapabilities} */
    this.capabilities = cfg.capabilities;

    /**
     * Set when receiving the server `auth` event, and directly consumed
     * (through the `getParticipants` callback) when the OnlyOffice client
     * handles this server `auth` event.
     * This weird behavior is due to a modification done by Cryptpad in their
     * OnlyOffice fork (that we are based on).
     * @type {undefined | { list: OOParticipantEntry[], index: number }}
     */
    this._initialParticipants = undefined;
    /**
     * Set when receiving the server `auth` event, and directly consumed
     * (through the `getInitialChanges` callback) when the OnlyOffice client
     * handles this server `auth` event.
     * This weird behavior is due to a modification done by Cryptpad in their
     * OnlyOffice fork (that we are based on).
     * @type {undefined | { list: OOParticipantEntry[], index: number }}
     */
    // TODO: type
    this._initialChanges = undefined;

    // // Provisional `indexUser`; overridden by the server-assigned index on the
    // // `auth` reply. 0 is the local seed used by `getParticipants` before the
    // // server replies.
    // this.indexUser = 0;

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
   * Convert a client event from OnlyOffice to Editics protocol.
   *
   * This is used when the OnlyOffice editor wants to communicate with the server,
   * and the returned Editics event should then be send to the Parsec server.
   *
   * Returns `null` if the event has no Editics countepart (e.g. `rpc`).
   *
   * @param {OOClientEvent} oo
   * @returns {Promise<EditicsClientEvent|null>}
   */
  async cookClientEvent(oo) {
    // Still to be translated (disabled until the server side of the
    // collaborative mode is finished; move back into the switch below to
    // enable, see the matching server events in the RFC):
    //
    //   case 'authChangesAck':
    //     return { type: 'authChangesAck' };
    //
    //   case 'message':
    //     return { type: 'message', encryptedMessage: this._encrypt(strToBytes(String(oo.message))) };
    //
    //   case 'cursor':
    //     return { type: 'cursor', encryptedCursor: this._encrypt(strToBytes(String(oo.cursor))) };
    //
    //   case 'getLock':
    //     return { type: 'getLock', block: oo.block };
    //
    //   case 'isSaveLock':
    //     return { type: 'isSaveLock', syncChangesIndex: oo.syncChangesIndex };
    //
    //   case 'saveChanges':
    //     return this._cookSaveChanges(oo);
    //
    //   case 'unSaveLock':
    //     return { type: 'unSaveLock' };
    //
    //   case 'close':
    //     return { type: 'close' };
    //
    //   case 'saveDone':
    //     return { type: 'saveDone', savedUpToIndex: oo.savedUpToIndex, newVersion: oo.newVersion };

    switch (oo.type) {
      case 'auth':
        return {
          type: 'auth',
          indexUser: -1,
          editorType: this.config.editorType,
          vlobVersion: this.config.vlobVersion,
          reconnect: null,
        };

      case 'getMessages':
        return {
          type: 'getMessages',
        };

      case 'unLockDocument':
        return {
          type: 'unLockDocument',
          isSave: oo.isSave,
          unlock: oo.unlock,
          deleteIndex: oo.deleteIndex,
          releaseLocks: oo.releaseLocks,
        };

      // Ignored events
      case 'clientLog':
      case 'extendSession':
      case 'forceSaveStart':
      case 'openDocument':
      case 'rpc':
        return null;

      default:
        console.warn(`Unknown OnlyOffice protocol event ${JSON.stringify(oo)}`);
        return null;
    }
  }

  // /**
  //  * @param {OOClientEventSaveChanges} oo
  //  * @returns {EditicsClientEventSaveChanges}
  //  */
  // _cookSaveChanges(oo) {
  //   // OnlyOffice sends `changes` as a JSON-encoded *string* in default (JSON)
  //   // mode, e.g. '["66;...","127;..."]' (an array of opaque op fragments).
  //   // Parse it into an array of fragment strings, then encrypt each fragment
  //   // independently as `encryptedChanges` (one entry per fragment, RFC §2.2).
  //   let fragments = [];
  //   if (oo.changes !== null && oo.changes !== undefined) {
  //     if (typeof oo.changes === 'string') {
  //       try {
  //         const parsed = JSON.parse(oo.changes);
  //         if (Array.isArray(parsed)) fragments = parsed;
  //       } catch (_e) {
  //         fragments = [oo.changes];
  //       }
  //     } else if (Array.isArray(oo.changes)) {
  //       fragments = oo.changes;
  //     } else {
  //       fragments = [oo.changes];
  //     }
  //   }
  //   const encryptedChanges = fragments.map((f) => this._encrypt(strToBytes(String(f))));

  //   // `excelAdditionalInfo` is split (RFC §2.2) into `encryptedCursor` (the
  //   // `CursorInfo` part, encrypted) and `excel_info` (the cleartext
  //   // `indexCols`/`indexRows` part). Step 2 keeps the previous passthrough
  //   // behavior: the whole opaque blob is encrypted as the cursor; `excel_info`
  //   // is null (real parsing is deferred to a later step).
  //   let encryptedCursor = null;
  //   if (oo.excelAdditionalInfo !== null && oo.excelAdditionalInfo !== undefined) {
  //     encryptedCursor = this._encrypt(strToBytes(String(oo.excelAdditionalInfo)));
  //   }
  //   return {
  //     type: 'saveChanges',
  //     encryptedChanges,
  //     startSaveChanges: !!oo.startSaveChanges,
  //     endSaveChanges: !!oo.endSaveChanges,
  //     deleteIndex: oo.deleteIndex,
  //     // `excel_info` mirrors the server pydantic field name (snake_case).
  //     // eslint-disable-next-line camelcase
  //     excel_info: null,
  //     encryptedCursor,
  //     releaseLocks: !!oo.releaseLocks,
  //   };
  // }

  /**
   * Convert a server event from Editics to OnlyOffice protocol.
   *
   * This is used when the server wants to communicate with the client, and the
   * returned OnlyOffice event should then be ingested by the OnlyOffice client.
   *
   * Returns `null` if the event has no OnlyOffice countepart (unexpected though,
   * typically due to a bug).
   *
   * @param {EditicsServerEvent} editics
   * @returns {Promise<OOServerEvent|null>}
   */
  async cookServerEvent(editics) {
    switch (editics.type) {
      case 'auth':
        return this._cookServerAuth(editics);

      case 'waitAuth':
        return this._cookWaitAuth(editics);

      case 'connectState':
        return this._cookConnectState(editics);

      case 'message':
        return this._cookMessage(editics);

        // Still to be translated (disabled until the server side of the
        // collaborative mode is finished; move back into the switch below to
        // enable):
        //
        //   case 'authChanges':
        //     return this._cookAuthChanges(editics);
        //
        //   case 'cursor':
        //     return this._cookCursor(editics);
        //
        //   case 'getLock':
        //     return this._cookGetLock(editics);
        //
        //   case 'releaseLock':
        //     return this._cookReleaseLock(editics);
        //
        //   case 'saveChanges':
        //     return this._cookServerSaveChanges(editics);
        //
        //   case 'savePartChanges':
        //     return {
        //       type: 'savePartChanges',
        //       changesIndex: editics.changesIndex,
        //       syncChangesIndex: editics.syncChangesIndex,
        //     };
        //
        //   case 'saveLock':
        //     return { type: 'saveLock', saveLock: !!editics.saveLock };
        //
        //   case 'unSaveLock':
        //     return {
        //       type: 'unSaveLock',
        //       index: editics.index,
        //       time: editics.time,
        //       syncChangesIndex: editics.syncChangesIndex,
        //     };
        //
        //   case 'drop':
        //     return { type: 'drop', code: editics.code, description: editics.description };
        //
        //   case 'warning':
        //     return { type: 'warning', code: editics.code, message: editics.message };

      default:
        console.warn(`Unknown Editics protocol event ${JSON.stringify(editics)}`);
        return null;
    }
  }

  /**
   * Maps a successful server `auth` event (which the server only sends on
   * acceptance: rejections are handled by the connection layer, see
   * `collaborative.js`) into the OO `auth` (server→client) event the editor's
   * handshake expects as the positive reply to its own `auth` (c→s),
   * resolving participant names via `getHumanLabelFromDeviceId`.
   *
   * OnlyOffice's `auth` (s→c) carries many integrator-specific fields (jwt,
   * build info, settings, …) that the Parsec server doesn't provide (RFC §2.2
   * editics changes drop them). The translator re-injects sensible defaults
   * so the editor's state machine gets the shape it expects; their values
   * are opaque to the editor's collaboration logic (only `result`,
   * `indexUser`, `participants` and `sessionId` are meaningful).
   * @param {EditicsServerEventAuth} editics
   * @returns {Promise<OOServerEventAuth|null>}
   */
  async _cookServerAuth(editics) {
    await this._setParticipants(editics.participants);
    const participants = this._onlyofficeParticipants();

    return {
      type: 'auth',
      result: 1,
      // Editics protocol merge together the session and participant concepts
      sessionId: editics.participantId,
      // TODO: Check the relashionship between sessionTimeConnect and openedAt.
      sessionTimeConnect: editics.participantTimeConnect,
      participants,
      locks: {},
      indexUser: editics.indexUser,
      hasForgotten: false,
      jwt: '',
      // eslint-disable-next-line camelcase -- OnlyOffice wire field name
      g_cAscSpellCheckUrl: '',
      buildVersion: '',
      buildNumber: 0,
      licenseType: 0,
      settings: {
        spellcheckerUrl: '',
        reconnection: { attempts: 50, delay: 2000 },
        binaryChanges: false,
        websocketMaxPayloadSize: 1572864,
        maxChangesSize: 157286400,
        // eslint-disable-next-line camelcase -- OnlyOffice wire field name
        limits_image_size: 26214400,
        // eslint-disable-next-line camelcase -- OnlyOffice wire field name
        limits_image_types_upload: 'jpg;jpeg;jpe;png;gif;bmp;svg;tiff;tif;webp;heic;heif;avif',
      },
      openedAt: editics.participantTimeConnect,
    };
  }

  /**
   * The server parks us behind the auth lock holder (single-editor to
   * co-editing transition, RFC §6.2): the editor must wait for the
   * established editor to release the lock with `unLockDocument{unlock:true}`.
   * @param {EditicsServerEventWaitAuth} editics
   * @returns {OOServerEventWaitAuth|null}
   */
  _cookWaitAuth(editics) {
    // The holder is necessarily in the participant table: the `connectState`
    // broadcast that announced our arrival carried it, and the SSE channel
    // preserves the ordering.
    const holder = this._participants.get(editics.authLockedBy);
    if (!holder) {
      console.warn(`waitAuth: unknown auth lock holder ${editics.authLockedBy}`);
      return null;
    }
    return {
      type: 'waitAuth',
      lockDocument: this._onlyofficeParticipantEntry(holder, editics.authLockedBy),
    };
  }

  /**
   * @param {EditicsServerEventConnectState} editics
   * @returns {Promise<OOServerEventConnectState>}
   */
  async _cookConnectState(editics) {
    // The server's participant list is authoritative: it replaces the whole
    // table (dropping the provisional self seed and the departed participants).
    await this._setParticipants(editics.participants || []);
    return {
      type: 'connectState',
      participantsTimestamp: editics.participantsTimestamp,
      participants: this._onlyofficeParticipants(),
      waitAuth: !!editics.waitAuth,
    };
  }

  // /**
  //  * @param {EditicsServerEventAuthChanges} editics
  //  * @returns {OOServerEventAuthChanges}
  //  */
  // _cookAuthChanges(editics) {
  //   const docid = `${this.config.workspaceId}/${this.config.vlobId}`;
  //   const changes = (editics.changes || []).map((entry) => {
  //     const idx = entry[0];
  //     const blob = entry[1];
  //     const userId = this._userId(idx);
  //     return {
  //       docid,
  //       change: JSON.stringify(bytesToStr(this._decrypt(blob))),
  //       time: 0,
  //       user: userId + String(idx),
  //       useridoriginal: userId,
  //     };
  //   });
  //   return { type: 'authChanges', changes };
  // }

  /**
   * @param {EditicsServerEventMessage} editics
   * @returns {OOServerEventMessage}
   */
  _cookMessage(editics) {
    /** @type {OOServerEventMessage} */
    const event = { type: 'message' };
    if (editics.messages && editics.messages.length !== 0) {
      // TODO: when the author of the comment has left we won't be able to use `indexUser`
      //       so we should only rely on the deviceID (and hence can we remove the other fields ?)
      event.messages = editics.messages.map((m) => {
        const userId = this._userId(m.authorIndexUser);
        return {
          message: bytesToStr(this._decrypt(m.encryptedMessage)),
          time: m.time,
          user: userId + String(m.authorIndexUser),
          useridoriginal: userId,
          username: this._userName(m.authorIndexUser),
        };
      });
    }
    return event;
  }

  // /**
  //  * @param {EditicsServerEventCursor} editics
  //  * @returns {OOServerEventCursor}
  //  */
  // _cookCursor(editics) {
  //   const messages = (editics.messages || []).map((m) => {
  //     const userId = this._userId(m.authorIndexUser);
  //     return {
  //       cursor: bytesToStr(this._decrypt(m.encryptedCursor)),
  //       time: m.time,
  //       user: userId + String(m.authorIndexUser),
  //       useridoriginal: userId,
  //     };
  //   });
  //   return { type: 'cursor', messages };
  // }

  // /**
  //  * @param {EditicsServerEventGetLock} editics
  //  * @returns {OOServerEventGetLock}
  //  */
  // _cookGetLock(editics) {
  //   const ooLocks = {};
  //   for (const key in editics.locks || {}) {
  //     const lock = editics.locks[key];
  //     const userId = this._userId(lock.user);
  //     ooLocks[key] = { time: lock.time, user: userId + String(lock.user), block: lock.block };
  //   }
  //   return { type: 'getLock', locks: ooLocks };
  // }

  // /**
  //  * @param {EditicsServerEventReleaseLock} editics
  //  * @returns {OOServerEventReleaseLock}
  //  */
  // _cookReleaseLock(editics) {
  //   const locks = (editics.locks || []).map((lock) => {
  //     const userId = this._userId(lock.user);
  //     return { block: lock.block, user: userId + String(lock.user), time: lock.time, changes: null };
  //   });
  //   return { type: 'releaseLock', locks };
  // }

  // /**
  //  * @param {EditicsServerEventSaveChanges} editics
  //  * @returns {OOServerEventSaveChanges}
  //  */
  // _cookServerSaveChanges(editics) {
  //   const docid = `${this.config.workspaceId}/${this.config.vlobId}`;
  //   const changes = (editics.changes || []).map((c) => {
  //     const userId = this._userId(c.authorIndexUser);
  //     return {
  //       docid,
  //       change: JSON.stringify(bytesToStr(this._decrypt(c.change))),
  //       time: c.time,
  //       user: userId + String(c.authorIndexUser),
  //       useridoriginal: userId,
  //     };
  //   });
  //   const locks = (editics.locks || []).map((lock) => {
  //     const userId = this._userId(lock.user);
  //     return { block: lock.block, user: userId + String(lock.user), time: lock.time, changes: lock.changes };
  //   });
  //   /** @type {OOServerEventSaveChanges} */
  //   const oo = {
  //     type: 'saveChanges',
  //     changes,
  //     changesIndex: editics.changesIndex,
  //     syncChangesIndex: editics.syncChangesIndex,
  //     endSaveChanges: !!editics.endSaveChanges,
  //     startSaveChanges: true,
  //     locks,
  //   };
  //   if (editics.encryptedCursor !== null && editics.encryptedCursor !== undefined) {
  //     oo.excelAdditionalInfo = bytesToStr(this._decrypt(editics.encryptedCursor));
  //   }
  //   return oo;
  // }

  // --- Participant table helpers -------------------------------------------

  /**
   * Replace the participant table with the server's (authoritative) entries,
   * resolving the display names through the injected capabilities (the
   * server is NOT trusted for names, RFC §3.3).
   * @param {EditicsParticipantEntry[]} entries
   * @returns {Promise<void>}
   */
  async _setParticipants(entries) {
    const participants = new Map();
    for (const entry of entries) {
      const humanLabel = await this.config.capabilities.getHumanLabelFromDeviceId(entry.deviceId);
      participants.set(entry.indexUser, {
        deviceId: entry.deviceId,
        userName: humanLabel || '<unknown>',
        // The per-person user id the `<userId><indexUser>` composite ids of
        // the OnlyOffice protocol are built from.
        userId: entry.deviceId,
        view: !!entry.view,
      });
    }
    this._participants = participants;
  }

  /**
   * @returns {OOParticipantEntry[]}
   */
  _onlyofficeParticipants() {
    const list = /** @type {OOParticipantEntry[]} */ ([]);
    this._participants.forEach((p, indexUser) => {
      list.push(this._onlyofficeParticipantEntry(p, indexUser));
    });
    return list;
  }

  /**
   * @param {{deviceId: string, userName: string, userId: string, view: boolean}} p
   * @param {number} indexUser
   * @returns {OOParticipantEntry}
   */
  _onlyofficeParticipantEntry(p, indexUser) {
    return {
      id: p.userId + String(indexUser),
      idOriginal: p.userId,
      username: p.userName,
      indexUser: indexUser,
      // Never provided by the editics protocol, and unused by the editor
      connectionId: '',
      view: !!p.view,
      isLiveViewer: false,
      isCloseCoAuthoring: false,
      encrypted: false,
    };
  }

  /**
   * Display name of a participant index (falls back to the index itself when
   * the participant is not in the table yet).
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

// ---------------------------------------------------------------------------
// Editics protocol types
// ---------------------------------------------------------------------------
/* eslint-disable max-len */

/**
 * @typedef {string} EditicsParticipantID
 *
 * @typedef {Object} EditicsParticipantEntry
 * @property {EditicsParticipantID} id
 * @property {number} indexUser
 * @property {string} deviceId - DeviceID hex
 * @property {boolean} view
 *
 * @typedef {Object} EditicsReconnect
 * @property {EditicsParticipantID} participantId
 * @property {number} participantTimeConnect
 * @property {number} timeIdle
 *
 * @typedef {Object} EditicsClientEventAuth
 * @property {'auth'} type
 * @property {number} indexUser - -1 on first open
 * @property {number} editorType
 * @property {number} vlobVersion
 * @property {EditicsReconnect|null} reconnect
 *
 * @typedef {Object} EditicsClientEventAuthChangesAck
 * @property {'authChangesAck'} type
 *
 * @typedef {Object} EditicsClientEventGetMessages
 * @property {'getMessages'} type
 *
 * @typedef {Object} EditicsClientEventMessage
 * @property {'message'} type
 * @property {Uint8Array} encryptedMessage
 *
 * @typedef {Object} EditicsClientEventCursor
 * @property {'cursor'} type
 * @property {Uint8Array} encryptedCursor
 *
 * @typedef {Object} EditicsClientEventGetLock
 * @property {'getLock'} type
 * @property {Array<*>} block
 *
 * @typedef {Object} EditicsClientEventIsSaveLock
 * @property {'isSaveLock'} type
 * @property {number} syncChangesIndex
 *
 * @typedef {Object} EditicsClientEventSaveChanges
 * @property {'saveChanges'} type
 * @property {Uint8Array[]} encryptedChanges
 * @property {boolean} startSaveChanges
 * @property {boolean} endSaveChanges
 * @property {number|null} [deleteIndex]
 * @property {Object<string,*>|null} [excel_info]
 * @property {Uint8Array|null} [encryptedCursor]
 * @property {boolean} [releaseLocks]
 *
 * @typedef {Object} EditicsClientEventUnSaveLock
 * @property {'unSaveLock'} type
 *
 * @typedef {Object} EditicsClientEventUnLockDocument
 * @property {'unLockDocument'} type
 * @property {boolean} isSave
 * @property {boolean} unlock
 * @property {number|null} [deleteIndex]
 * @property {boolean} [releaseLocks]
 *
 * @typedef {Object} EditicsClientEventClose
 * @property {'close'} type
 *
 * @typedef {Object} EditicsClientEventSaveDone
 * @property {'saveDone'} type
 * @property {number} savedUpToIndex
 * @property {number} newVersion
 *
 * @typedef {EditicsClientEventAuth|EditicsClientEventAuthChangesAck|EditicsClientEventGetMessages|EditicsClientEventMessage|EditicsClientEventCursor|EditicsClientEventGetLock|EditicsClientEventIsSaveLock|EditicsClientEventSaveChanges|EditicsClientEventUnSaveLock|EditicsClientEventUnLockDocument|EditicsClientEventClose|EditicsClientEventSaveDone} EditicsClientEvent
 *
 * @typedef {Object} EditicsServerEventAuth
 * @property {'auth'} type
 * @property {EditicsParticipantEntry[]} participants
 * @property {number} indexUser
 * @property {EditicsParticipantID} participantId
 * @property {number} participantTimeConnect
 *
 * @typedef {Object} EditicsServerEventConnectState
 * @property {'connectState'} type
 * @property {number} participantsTimestamp
 * @property {EditicsParticipantEntry[]} participants
 * @property {boolean} waitAuth
 *
 * @typedef {Object} EditicsServerEventAuthChanges
 * @property {'authChanges'} type
 * @property {Array<[number, Uint8Array]>} changes - (index, encrypted blob)
 *
 * @typedef {Object} EditicsServerEventWaitAuth
 * @property {'waitAuth'} type
 * @property {number} authLockedBy - the indexUser holding the auth lock
 *
 * @typedef {Object} EditicsServerEventMessage
 * @property {'message'} type
 * @property {Array<{time:number, authorIndexUser:number, encryptedMessage:Uint8Array}>} messages
 *
 * @typedef {Object} EditicsServerEventCursor
 * @property {'cursor'} type
 * @property {Array<{time:number, authorIndexUser:number, encryptedCursor:Uint8Array}>} messages
 *
 * @typedef {Object} EditicsServerEventGetLock
 * @property {'getLock'} type
 * @property {Record<string, {time:number, user:number, block:*}>} locks
 *
 * @typedef {Object} EditicsServerEventReleaseLock
 * @property {'releaseLock'} type
 * @property {Array<{block:*, user:number, time:number, changes:null}>} locks
 *
 * @typedef {Object} EditicsServerEventSaveLock
 * @property {'saveLock'} type
 * @property {boolean} saveLock
 *
 * @typedef {Object} EditicsServerEventSaveChanges
 * @property {'saveChanges'} type
 * @property {Array<{time:number, authorIndexUser:number, change:Uint8Array}>} changes
 * @property {number} changesIndex
 * @property {number} syncChangesIndex
 * @property {boolean} endSaveChanges
 * @property {Array<{block:*, user:number, time:number, changes:null}>} [locks]
 * @property {Object<string,*>|null} [excel_info]
 * @property {Uint8Array|null} [encryptedCursor]
 *
 * @typedef {Object} EditicsServerEventSavePartChanges
 * @property {'savePartChanges'} type
 * @property {number} changesIndex
 * @property {number} syncChangesIndex
 *
 * @typedef {Object} EditicsServerEventUnSaveLock
 * @property {'unSaveLock'} type
 * @property {number} index
 * @property {number} time
 * @property {number} syncChangesIndex
 *
 * @typedef {Object} EditicsServerEventDrop
 * @property {'drop'} type
 * @property {number} code
 * @property {string} description
 *
 * @typedef {Object} EditicsServerEventWarning
 * @property {'warning'} type
 * @property {number} code
 * @property {string} message
 *
 * @typedef {EditicsServerEventAuth|EditicsServerEventConnectState|EditicsServerEventAuthChanges|EditicsServerEventWaitAuth|EditicsServerEventMessage|EditicsServerEventCursor|EditicsServerEventGetLock|EditicsServerEventReleaseLock|EditicsServerEventSaveLock|EditicsServerEventSaveChanges|EditicsServerEventSavePartChanges|EditicsServerEventUnSaveLock|EditicsServerEventDrop|EditicsServerEventWarning} EditicsServerEvent
 */
/* eslint-enable max-len */
