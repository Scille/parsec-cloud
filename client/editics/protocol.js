// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Here we define `EditicsTranslator` which works as an interface between the
// OnlyOffice client (speaking the OnlyOffice protocol) and the Parsec client/server
// communication (speaking the Editics protocol).
//
// The Editics protocols is basically the OnlyOffice protocol plus an encryption
// layer to protect the sensitive fields (e.g. document modification).
//
// Note this file is tested in the server tests (see server/tests/editics/test_records.py).
// This is done by loading it in a nodejs VM controlled from Python (using PyMiniRacer)
// so that the test can simulate the behavior of an actual OnlyOffice client sending
// and receiving OnlyOffice protocol events that are then translated by this file and
// send to the server.

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
 * @typedef {Object} LibparsecCapabilities
 *    Injected capabilities that are expected to be provided by libparsec.
 * @property {(deviceIdHex: string) => Promise<string | undefined>} getHumanLabelFromDeviceId
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
   * @returns {{list:OOParticipantEntry[], index:number}}
   * This callback comes from a modification done by Cryptpad in their OnlyOffice
   * fork (that we are based on), it is called once when the OnlyOffice client
   * process the server `auth` event and should return the initial participant
   * specified in the server `auth` event we have just received.
   *
   * see https://github.com/cryptpad/onlyoffice-editor/blob/b5d78add4608a76b28d14467d44c5c001da768db/onlyoffice-editor/src/index.ts#L133
   */
  getParticipants() {
    const initialParticipants = this._initialParticipants;
    this._initialParticipants = undefined;
    if (initialParticipants === undefined) {
      // Unexpected: this callback should only be called once right after we have
      // handled a server `auth` event (where we have set `this._initialParticipants`).
      throw new Error('`initialParticipants` undefined');
    }
    return initialParticipants;
    // const list = [];
    // let index = -1;
    // this._participants.forEach((p, indexUser) => {
    //   list.push({
    //     id: p.userId + String(indexUser),
    //     idOriginal: p.userId,
    //     username: p.userName || p.deviceId,
    //     indexUser: indexUser,
    //     view: false,
    //   });
    //   if (indexUser === this.indexUser) index = list.length - 1;
    // });
    // return { list, index };
  }

  /**
   * @returns {Array}
   * This callback comes from a modification done by Cryptpad in their OnlyOffice
   * fork (that we are based on), it is called once when the OnlyOffice client
   * process the server `auth` event and should return the initial changes
   * specified in the server `auth` event we have just received.
   *
   * see: https://github.com/cryptpad/onlyoffice-editor/blob/b5d78add4608a76b28d14467d44c5c001da768db/onlyoffice-editor/src/index.ts#L131
   */
  getInitialChanges() {
    const initialChanges = this._initialChanges;
    this._initialChanges = undefined;
    if (initialChanges === undefined) {
      // Unexpected: this callback should only be called once right after we have
      // handled a server `auth` event (where we have set `this._initialChanges`).
      throw new Error('`initialChanges` undefined');
    }
    return initialChanges;
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
    switch (oo && oo.type) {
      case 'auth':
        return {
          type: 'auth',
          indexUser: -1,
          editorType: this.config.editorType,
          vlobVersion: this.config.vlobVersion,
        };

      case 'getMessages':
        return {
          type: 'getMessages',
        };

      // TODO

      // case 'authChangesAck':
      //   return { type: 'authChangesAck' };

      // case 'message':
      //   return { type: 'message', encryptedMessage: this._encrypt(strToBytes(String(oo.message))) };

      // case 'cursor':
      //   return { type: 'cursor', encryptedCursor: this._encrypt(strToBytes(String(oo.cursor))) };

      // case 'getLock':
      //   return { type: 'getLock', block: oo.block };

      // case 'isSaveLock':
      //   return { type: 'isSaveLock', syncChangesIndex: oo.syncChangesIndex };

      // case 'saveChanges':
      //   return this._cookSaveChanges(oo);

      // case 'unSaveLock':
      //   return { type: 'unSaveLock' };

      // case 'unLockDocument':
      //   return {
      //     type: 'unLockDocument',
      //     isSave: !!oo.isSave,
      //     unlock: !!oo.unlock,
      //     deleteIndex: oo.deleteIndex,
      //     releaseLocks: !!oo.releaseLocks,
      //   };

      // case 'close':
      //   return { type: 'close' };

      // case 'saveDone':
      //   return { type: 'saveDone', savedUpToIndex: oo.savedUpToIndex, newVersion: oo.newVersion };

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
    switch (editics && editics.type) {
      case 'auth':
        return this._cookServerAuth(editics);

      // TODO

      // case 'waitAuth':
      //   return this._cookWaitAuth(editics);

      // case 'connectState':
      //   return this._cookConnectState(editics);

      // case 'authChanges':
      //   return this._cookAuthChanges(editics);

      case 'message':
        return this._cookMessage(editics);

      // case 'cursor':
      //   return this._cookCursor(editics);

      // case 'getLock':
      //   return this._cookGetLock(editics);

      // case 'releaseLock':
      //   return this._cookReleaseLock(editics);

      // case 'saveChanges':
      //   return this._cookServerSaveChanges(editics);

      // case 'savePartChanges':
      //   return {
      //     type: 'savePartChanges',
      //     changesIndex: editics.changesIndex,
      //     syncChangesIndex: editics.syncChangesIndex,
      //   };

      // case 'saveLock':
      //   return { type: 'saveLock', saveLock: !!editics.saveLock };

      // case 'unSaveLock':
      //   return {
      //     type: 'unSaveLock',
      //     index: editics.index,
      //     time: editics.time,
      //     syncChangesIndex: editics.syncChangesIndex,
      //   };

      // case 'drop':
      //   return { type: 'drop', code: editics.code, description: editics.description };

      // case 'warning':
      //   return { type: 'warning', code: editics.code, message: editics.message };

      default:
        console.warn(`Unknown Editics protocol event ${JSON.stringify(editics)}`);
        return null;
    }
  }

  /**
   * On a successful `auth` (result: 1) the translator sets `indexUser`,
   * rebuilds the participant table from `data.participants` (resolving names
   * via `resolveUserName`) and produces an OO `auth` (server→client) event —
   * the editor's handshake expects this as the positive reply to its own
   * `auth` (c→s). On rejection (result: 0) the translator produces nothing
   * to forward (the rejection is handled by `main.js` / the test).
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
    /** @type {OOParticipantEntry[]} */
    let participants = [];
    for (const editicsParticipant of editics.participants) {
      const humanLabel = this.config.capabilities.getHumanLabelFromDeviceId(editicsParticipant.deviceId) || '<unknown>';
      participants.push({
        // TODO: what id/idOriginal/username stand for ? is it the connection/device/user ?
        id: editicsParticipant.deviceId + editicsParticipant.indexUser,
        idOriginal: editicsParticipant.deviceId,
        indexUser: editicsParticipant.indexUser,
        // Dummy value since `connectionId` is never actually used by the client
        connectionId: '',
        username: humanLabel,
        view: editicsParticipant.view,
        isCloseCoAuthoring: false, // TODO: needed in editics event ?
        isLiveViewer: false, // TODO: needed in editics event ?
        encrypted: false,
      });
    }

    return {
      type: 'auth',
      result: 1,
      sessionId: editics.sessionId,
      sessionTimeConnect: editics.sessionTimeConnect,
      participants,
      locks: {},
      indexUser: editics.indexUser,
      hasForgotten: false,
      jwt: '',
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
        limits_image_size: 26214400,
        limits_image_types_upload: 'jpg;jpeg;jpe;png;gif;bmp;svg;tiff;tif;webp;heic;heif;avif',
      },
      openedAt: editics.sessionTimeConnect,
    };
  }

  // /**
  //  * @param {EditicsServerEventWaitAuth} editics
  //  * @returns {Promise<OOServerEventWaitAuth|null>}
  //  */
  // async _cookWaitAuth(editics) {
  //   // Ensure the holder is in the participant table so `lockDocument` is
  //   // well-formed (the server may have broadcast a `connectState{waitAuth:true}`
  //   // carrying the holder before this RPC reply reached the newcomer).
  //   const holderIndex = editics.authLockedBy;
  //   if (!this._participants.has(holderIndex)) {
  //     await this._mergeParticipants([{ indexUser: holderIndex, deviceId: this._deviceId(holderIndex) }]);
  //   }
  //   const holder = this._participants.get(holderIndex);
  //   if (!holder) {
  //     return null;
  //   }
  //   return {
  //     type: 'waitAuth',
  //     lockDocument: this._onlyofficeParticipantEntry(holder, holderIndex),
  //   };
  // }

  // /**
  //  * @param {EditicsServerEventConnectState} editics
  //  * @returns {Promise<OOServerEventConnectState>}
  //  */
  // async _cookConnectState(editics) {
  //   // The server's participant list is authoritative: drop the provisional
  //   // self-seed (index 0) and any participant no longer present, the first
  //   // time an authoritative list arrives. Mirrors `_cookServerAuth` which
  //   // clears the table before merging the auth reply.
  //   const incoming = editics.participants || [];
  //   const incomingIdx = new Set(incoming.map((p) => p.indexUser));
  //   if (incoming.length > 0) {
  //     for (const idx of [...this._participants.keys()]) {
  //       if (!incomingIdx.has(idx)) this._participants.delete(idx);
  //     }
  //   }
  //   await this._mergeParticipants(incoming);
  //   return {
  //     type: 'connectState',
  //     participantsTimestamp: editics.participantsTimestamp,
  //     participants: this._onlyofficeParticipants(),
  //     waitAuth: !!editics.waitAuth,
  //   };
  // }

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
    let event = { type: 'message' };
    if (editics.messages && editics.messages.length != 0) {
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

  // // --- Participant table helpers -------------------------------------------

  // /**
  //  * Merge the server's participant entries into the local table, resolving
  //  * names/userIds via the injected capabilities (the server is NOT trusted for
  //  * names, RFC §3.3).
  //  * @param {EditicsParticipantEntry[]} participants
  //  * @returns {Promise<void>}
  //  */
  // async _mergeParticipants(participants) {
  //   for (const p of participants) {
  //     if (!this._participants.has(p.indexUser)) {
  //       let userName = p.deviceId;
  //       let userId = p.deviceId;
  //       const resolved = await this.capabilities.resolveUser(p.deviceId);
  //       if (resolved) {
  //         [userId, userName] = resolved;
  //       }
  //       this._participants.set(p.indexUser, { deviceId: p.deviceId, userName, userId });
  //     } else {
  //       // Backfill the userId if it wasn't resolved the first time but is now.
  //       const existing = this._participants.get(p.indexUser);
  //       if (existing && (!existing.userId || existing.userId === existing.deviceId) && this.capabilities.resolveUserId) {
  //         try {
  //           const resolved = await this.capabilities.resolveUserId(p.deviceId);
  //           if (resolved) existing.userId = resolved;
  //         } catch (_e) {
  //           /* ignore */
  //         }
  //       }
  //     }
  //   }
  // }

  // /**
  //  * @returns {OOParticipantEntry[]}
  //  */
  // _onlyofficeParticipants() {
  //   const list = [];
  //   this._participants.forEach((p, indexUser) => {
  //     list.push(this._onlyofficeParticipantEntry(p, indexUser));
  //   });
  //   return list;
  // }

  // /**
  //  * @param {{deviceId:string, userName:string, userId:string}} p
  //  * @param {number} indexUser
  //  * @returns {OOParticipantEntry}
  //  */
  // _onlyofficeParticipantEntry(p, indexUser) {
  //   return {
  //     id: p.userId + String(indexUser),
  //     idOriginal: p.userId,
  //     username: p.userName,
  //     indexUser: indexUser,
  //     // TODO: server should return connection ID
  //     connectionId: "",
  //     // TODO: do we need view/isLiveViewer/isCloseCoAuthoring ?
  //     view: false,
  //     isLiveViewer: false,
  //     isCloseCoAuthoring: false,
  //     // Never used
  //     encrypted: false,
  //   };
  // }

  // /**
  //  * @param {number} indexUser
  //  * @returns {string}
  //  */
  // _userName(indexUser) {
  //   const p = this._participants.get(indexUser);
  //   return p ? p.userName : String(indexUser);
  // }

  // /**
  //  * The per-person userId for `user`/`useridoriginal` fields (matches the
  //  * editor's `_userId = userId + indexUser`).
  //  * @param {number} indexUser
  //  * @returns {string}
  //  */
  // _userId(indexUser) {
  //   const p = this._participants.get(indexUser);
  //   return p ? p.userId || p.deviceId : String(indexUser);
  // }

  // /**
  //  * Best-effort deviceId for a participant index that may not be in the table
  //  * yet (used to seed the auth-lock holder before its `connectState` arrives).
  //  * @param {number} indexUser
  //  * @returns {string}
  //  */
  // _deviceId(indexUser) {
  //   const p = this._participants.get(indexUser);
  //   return p ? p.deviceId : String(indexUser);
  // }

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
// OnlyOffice protocol types
// ---------------------------------------------------------------------------

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
 *
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
 *
 * @typedef {Object} OOClientEventMessage
 * @property {'message'} type
 * @property {string} message
 *
 * @typedef {Object} OOClientEventCursor
 * @property {'cursor'} type
 * @property {string} cursor - opaque OnlyOffice internal string.
 *
 * @typedef {Object} OOClientEventGetLock
 * @property {'getLock'} type
 * @property {Array} block - opaque block descriptors (shape depends on editor).
 *
 * @typedef {Object} OOClientEventIsSaveLock
 * @property {'isSaveLock'} type
 * @property {number} syncChangesIndex
 *
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
 *
 * @typedef {Object} OOClientEventUnSaveLock
 * @property {'unSaveLock'} type
 *
 * @typedef {Object} OOClientEventUnLockDocument
 * @property {'unLockDocument'} type
 * @property {boolean} isSave
 * @property {boolean} unlock
 * @property {number|null} [deleteIndex]
 * @property {boolean} [releaseLocks]
 *
 * @typedef {Object} OOClientEventClose
 * @property {'close'} type
 *
 * @typedef {Object} OOClientEventAuthChangesAck
 * @property {'authChangesAck'} type
 *
 * @typedef {Object} OOClientEventGetMessages
 * @property {'getMessages'} type
 *
 * @typedef {Object} OOClientEventOpenDocument
 * @property {'openDocument'} type
 * @property {Object} message
 *
 * @typedef {Object} OOClientEventClientLog
 * @property {'clientLog'} type
 * @property {string} level
 * @property {string} msg
 *
 * @typedef {Object} OOClientEventExtendSession
 * @property {'extendSession'} type
 * @property {number} idletime
 *
 * @typedef {Object} OOClientEventForceSaveStart
 * @property {'forceSaveStart'} type
 *
 * @typedef {Object} OOClientEventRpc
 * @property {'rpc'} type
 * @property {number} responseKey
 * @property {Object} data
 *
 * @typedef {Object} OOClientEventSaveDone
 *   Editics addition (no OO equivalent): the host page posts it after a vlob
 *   upload so the server bumps the session's allowed vlob version.
 * @property {'saveDone'} type
 * @property {number} savedUpToIndex
 * @property {number} newVersion
 *
 * @typedef {OOClientEventAuth|OOClientEventMessage|OOClientEventCursor|OOClientEventGetLock|OOClientEventIsSaveLock|OOClientEventSaveChanges|OOClientEventUnSaveLock|OOClientEventUnLockDocument|OOClientEventClose|OOClientEventAuthChangesAck|OOClientEventGetMessages|OOClientEventOpenDocument|OOClientEventClientLog|OOClientEventExtendSession|OOClientEventForceSaveStart|OOClientEventRpc|OOClientEventSaveDone} OOClientEvent
 *
 * @typedef {Object} OOMessageEntry
 * @property {string} docid
 * @property {string} message
 * @property {number} time
 * @property {string} user
 * @property {string} useridoriginal
 * @property {string} username
 *
 * @typedef {Object} OOServerEventAuth
 * @property {'auth'} type
 * @property {number} result - 1 = success
 * @property {string} sessionId
 * @property {number} sessionTimeConnect
 * @property {OOParticipantEntry[]} participants
 * @property {Array<OOMessageEntry> | undefined} messages
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
 *
 * @typedef {Object} OOServerEventWaitAuth
 * @property {'waitAuth'} type
 * @property {OOParticipantEntry} lockDocument - the established editor holding
 *   the auth lock (the newcomer must wait for it to release).
 *
 * @typedef {Object} OOServerEventConnectState
 * @property {'connectState'} type
 * @property {number} participantsTimestamp
 * @property {OOParticipantEntry[]} participants
 * @property {boolean} waitAuth
 *
 * @typedef {Object} OOServerEventAuthChanges
 * @property {'authChanges'} type
 * @property {Array<{docid:string, change:string, time:number, user:string, useridoriginal:string}>} changes
 *
 * @typedef {Object} OOServerEventMessage
 * @property {'message'} type
 * @property {Array<{docid:string, message:string, time:number, user:string, useridoriginal:string, username:string}>} messages
 *
 * @typedef {Object} OOServerEventCursor
 * @property {'cursor'} type
 * @property {Array<{cursor:string, time:number, user:string, useridoriginal:string}>} messages
 *
 * @typedef {Object} OOServerEventGetLock
 * @property {'getLock'} type
 * @property {Record<string, {time:number, user:string, block:*}>} locks
 *
 * @typedef {Object} OOServerEventReleaseLock
 * @property {'releaseLock'} type
 * @property {Array<{block:*, user:string, time:number, changes:null}>} locks
 *
 * @typedef {Object} OOServerEventSaveChanges
 * @property {'saveChanges'} type
 * @property {Array<{docid:string, change:string, time:number, user:string, useridoriginal:string}>|null} changes
 * @property {number} changesIndex
 * @property {number} syncChangesIndex
 * @property {boolean} endSaveChanges
 * @property {Array<{block:*, user:string, time:number, changes:*}>} [locks]
 * @property {string} [excelAdditionalInfo]
 *
 * @typedef {Object} OOServerEventSavePartChanges
 * @property {'savePartChanges'} type
 * @property {number} changesIndex
 * @property {number} syncChangesIndex
 *
 * @typedef {Object} OOServerEventSaveLock
 * @property {'saveLock'} type
 * @property {boolean} saveLock
 *
 * @typedef {Object} OOServerEventUnSaveLock
 * @property {'unSaveLock'} type
 * @property {number} index
 * @property {number} time
 * @property {number} syncChangesIndex
 *
 * @typedef {Object} OOServerEventDrop
 * @property {'drop'} type
 * @property {number} code
 * @property {string} description
 *
 * @typedef {Object} OOServerEventWarning
 * @property {'warning'} type
 * @property {number} code
 * @property {string} message
 *
 * @typedef {OOServerEventAuth|OOServerEventWaitAuth|OOServerEventConnectState|OOServerEventAuthChanges|OOServerEventMessage|OOServerEventCursor|OOServerEventGetLock|OOServerEventReleaseLock|OOServerEventSaveChanges|OOServerEventSavePartChanges|OOServerEventSaveLock|OOServerEventUnSaveLock|OOServerEventDrop|OOServerEventWarning} OOServerEvent
 */

// ---------------------------------------------------------------------------
// Editics protocol types
// ---------------------------------------------------------------------------

/**
 * @typedef {Object} EditicsParticipantEntry
 * @property {number} indexUser
 * @property {string} deviceId - DeviceID hex
 * @property {boolean} view
 *
 * @typedef {Object} EditicsClientEventAuth
 * @property {'auth'} type
 * @property {number} indexUser - -1 on first open
 * @property {number} editorType
 * @property {number} vlobVersion
 *
 * @typedef {Object} EditicsClientEventAuthChangesAck
 * @property {'authChangesAck'} type
 *
 * @typedef {Object} EditicsClientEventGetMessages
 * @property {'message'} type
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
 * @property {Array} block
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
 * @property {Array<{docid:string, message:string, time:number, deviceId: string}>} messages
 * @property {number} indexUser
 * @property {string} sessionId
 * @property {number} sessionTimeConnect
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
