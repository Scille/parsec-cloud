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
 *   OOLockEntry,
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
 *   OOServerEventWaitAuth,
 * } from 'onlyoffice-editor'
 */

// ---------------------------------------------------------------------------
// Web API wrapper functions
// ---------------------------------------------------------------------------

// The V8 runtime provided by PyMiniRacer for the tests comes with none of the
// standard Web APIs. So we only use those APIs in the wrapper functions defined
// here (and that are going to be mocked during the tests).

/**
 * UTF-8 encode a JS string into bytes (standard Web API: `TextEncoder`).
 * @param {string} s
 * @returns {Uint8Array}
 */
function bytesFromStr(s) {
  return new TextEncoder().encode(s);
}

/**
 * UTF-8 decode bytes into a JS string (standard Web API: `TextDecoder`).
 * @param {Uint8Array} bytes
 * @returns {string}
 */
function bytesToStr(bytes) {
  return new TextDecoder().decode(bytes);
}

// ---------------------------------------------------------------------------
// EditicsTranslator
// ---------------------------------------------------------------------------

// TODO: Rework documentation
// TODO: Rework error handling for getHumanLabelFromDeviceId/encrypt/decrypt
/**
 * @typedef {Object} EditicsLibparsecDeps
 * @property {(deviceIdHex: string) => Promise<string | undefined>} getHumanLabelFromDeviceId
 * @property {(cleartext: Uint8Array) => Promise<[number, Uint8Array]>} encrypt
 * @property {(keyIndex: number, ciphertext: Uint8Array) => Promise<Uint8Array>} decrypt
 *
 * @typedef {Object} EditicsTranslatorConfig
 * @property {string} workspaceId - WorkspaceID (VlobID) hex.
 * @property {string} vlobId - VlobID hex of the document.
 * @property {string} deviceIdHex - DeviceID hex of the client's device.
 * @property {string} [userId] - unused (the translator resolves the display
 *   names from the device ids through the `getHumanLabelFromDeviceId`
 *   capability). Kept for config compat, to be removed.
 * @property {string} [userName] - unused, see `userId`.
 * @property {number} vlobVersion - loaded vlob version (RFC §1.2).
 * @property {number} editorType - 0=Word, 1=Spreadsheet, 2=Presentation, 3=Visio.
 * @property {EditicsLibparsecDeps} capabilities
 */

/**
 * Side-effect-free translation layer between OnlyOffice and Editics protocols.
 */
class EditicsTranslator {
  /**
   * @param {EditicsTranslatorConfig} config
   * @param {EditicsLibparsecDeps} libparsec
   */
  constructor(config, libparsec) {
    /** @type {string} */
    this.docId = `${config.workspaceId}/${config.vlobId}`;

    /** @type {EditicsTranslatorConfig} */
    this.config = config;
    /** @type {EditicsLibparsecDeps} */
    this.libparsec = libparsec;

    // The translator is stateless, except for this cache. But this is transparent
    // since the human label never changes for a given device ID.
    /** @type {Map<string, string>} */
    this._humanLabelFromDeviceIdCache = new Map();
  }

  /**
   * @param {string} deviceIdHex
   * @returns {Promise<string | undefined>}
   */
  async getHumanLabelFromDeviceId(deviceIdHex) {
    const found = this._humanLabelFromDeviceIdCache.get(deviceIdHex);
    if (found) {
      return found;
    } else {
      // Cache miss
      const result = await this.libparsec.getHumanLabelFromDeviceId(deviceIdHex);
      if (result) {
        // Overwrite any concurrent insertion since the result should be always the same.
        this._humanLabelFromDeviceIdCache.set(deviceIdHex, result);
      }
      return result;
    }
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
    //   case 'saveDone':
    //     return { type: 'saveDone', savedUpToIndex: oo.savedUpToIndex, newVersion: oo.newVersion };

    switch (oo.type) {
      case 'auth': {
        /** @type {EditicsClientEventAuth} */
        const event = {
          type: 'auth',
          vlobVersion: this.config.vlobVersion,
          mode: oo.mode === 'edit' ? 'edit' : 'view',
          // `null` for a fresh join: this is what tells the server this is not
          // the restoration of a previous logical session.
          reconnect: null,
        };
        // Reconnection (i.e. transport-level restore): the editor echoes the
        // session ID the server assigned in its `auth` reply (which the editics
        // protocol uses as the participant ID) along with the connect time and
        // the time spent idle since the connection was lost.
        if (oo.sessionId !== null && oo.sessionId !== undefined) {
          event.reconnect = oo.sessionId;
        }
        return event;
      }

      case 'close':
        /** @type {EditicsClientEventClose} */
        return { type: 'close' };

      case 'getMessages':
        /** @type {EditicsClientEventGetMessages} */
        return {
          type: 'getMessages',
        };

      case 'authChangesAck':
        /** @type {EditicsClientEventAuthChangesAck} */
        return { type: 'authChangesAck' };

      case 'getLock':
        /** @type {EditicsClientEventGetLock} */
        return { type: 'getLock', block: oo.block };

      case 'message':
        // The chat message is encrypted as an opaque blob (RFC §2.2)
        const [msgKeyIndex, msgCipherText] = await this.libparsec.encrypt(bytesFromStr(String(oo.message)));
        /** @type {EditicsClientEventMessage} */
        return {
          type: 'message',
          encryptedMessage: [msgKeyIndex, msgCipherText.toBase64()],
        };

      case 'cursor':
        // OnlyOffice provides the cursor as a base64-encoded payload in a
        // custom format (e.g. `"16;CAAAADEAMQAzADcAAAAAAA=="`).
        // We consider this format opaque and encrypt it as-is.
        const [keyIndex, cipherText] = await this.libparsec.encrypt(bytesFromStr(oo.cursor));
        /** @type {EditicsClientEventCursor} */
        return {
          type: 'cursor',
          encryptedCursor: [keyIndex, cipherText.toBase64()],
        };

      case 'unLockDocument':
        /** @type {EditicsClientEventUnLockDocument} */
        return {
          type: 'unLockDocument',
          isSave: oo.isSave,
          unlock: oo.unlock,
          deleteIndex: oo.deleteIndex ?? null,
          releaseLocks: oo.releaseLocks ?? false,
        };

      case 'isSaveLock':
        /** @type {EditicsClientEventIsSaveLock} */
        return { type: 'isSaveLock', syncChangesIndex: oo.syncChangesIndex };

      case 'saveChanges':
        return await this._cookSaveChanges(oo);

      case 'unSaveLock':
        /** @type {EditicsClientEventUnSaveLock} */
        return { type: 'unSaveLock' };

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

  /**
   * @param {OOClientEventSaveChanges} oo
   * @returns {Promise<EditicsClientEventSaveChanges>}
   */
  async _cookSaveChanges(oo) {
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
        // `binaryChanges` mode: already a real array of fragments
        fragments = oo.changes;
      } else {
        fragments = [oo.changes];
      }
    }
    /** @type {[number, string][]} */
    const encryptedChanges = [];
    for (const fragment of fragments) {
      const [keyIndex, cipherText] = await this.libparsec.encrypt(bytesFromStr(String(fragment)));
      encryptedChanges.push([keyIndex, cipherText.toBase64()]);
    }

    // `excelAdditionalInfo` is split (RFC §2.2) into `encryptedCursor` (the
    // `CursorInfo` part, encrypted) and `excelAdditionalInfo` (the cleartext
    // `indexCols`/`indexRows` part). For now the whole opaque blob is
    // encrypted as the cursor; `excelAdditionalInfo` is null (real parsing is deferred
    // to a later step).
    /** @type {[number, string] | null} */
    let encryptedCursor = null;
    if (oo.excelAdditionalInfo !== null && oo.excelAdditionalInfo !== undefined) {
      const [keyIndex, cipherText] = await this.libparsec.encrypt(bytesFromStr(String(oo.excelAdditionalInfo)));
      encryptedCursor = [keyIndex, cipherText.toBase64()];
    }
    return {
      type: 'saveChanges',
      encryptedChanges,
      startSaveChanges: oo.startSaveChanges,
      endSaveChanges: oo.endSaveChanges,
      deleteIndex: oo.deleteIndex ?? null,
      // `excelAdditionalInfo` mirrors the server pydantic field name (snake_case).

      excelAdditionalInfo: null,
      encryptedCursorAdditionalInfo: encryptedCursor,
      releaseLocks: oo.releaseLocks,
    };
  }

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
        return await this._cookServerAuth(editics);

      case 'waitAuth':
        return await this._cookWaitAuth(editics);

      case 'connectState':
        return this._cookConnectState(editics);

      case 'message':
        return await this._cookMessage(editics);

      case 'cursor':
        return await this._cookCursor(editics);

      case 'saveLock':
        // Reply to our `isSaveLock`: the `saveLock` field is confusingly
        // named, `false` means the save lock is granted (i.e. we may now send
        // our `saveChanges`), `true` means it is held by somebody else.
        return { type: 'saveLock', saveLock: editics.lockedBySomebodyElse };

      case 'savePartChanges':
        // Acknowledgment of an intermediate `saveChanges` chunk (i.e. a save
        // too big for a single event).
        return {
          type: 'savePartChanges',
          changesIndex: editics.changesIndex,
          syncChangesIndex: editics.syncChangesIndex,
        };

      case 'unSaveLock':
        // Either the save has completed (real `index`/`time`/
        // `syncChangesIndex` values) or has been canceled (all `-1`).
        return {
          type: 'unSaveLock',
          index: editics.index,
          time: editics.time,
          syncChangesIndex: editics.syncChangesIndex,
        };

      case 'saveChanges':
        return await this._cookServerSaveChanges(editics);

      case 'authChanges':
        return await this._cookAuthChanges(editics);

      case 'getLock':
        return this._cookGetLock(editics);

      case 'releaseLock':
        return this._cookReleaseLock(editics);

      case 'error':
        // Reply to a refused session restoration (e.g. the document has been
        // modified while the participant was away, code 4010 "Restore error.
        // Document modified."): the editor reacts by closing the co-authoring
        // session.
        return {
          type: 'error',
          code: editics.code,
          description: editics.description,
        };

      case 'warning':
        return { type: 'warning', code: editics.code, message: editics.message };

      case 'drop':
        // `code` is always the DROP_CODE constant (4007) for a `drop` event,
        // OnlyOffice includes it only for symmetry with `disconnectReason`.
        return { type: 'drop', code: 4007, description: editics.description };

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
    /** @type {OOServerEventAuth} */
    const event = {
      type: 'auth',
      result: 1,
      // Editics protocol merge together the session and participant concepts
      sessionId: editics.participantId,
      // TODO: Check the relashionship between sessionTimeConnect and openedAt.
      sessionTimeConnect: editics.participantTimeConnect,
      participants: await this._onlyofficeParticipants(editics.participants),
      locks: this._onlyofficeLockTable(editics.locks),
      indexUser: editics.indexUser,
      hasForgotten: false,
      jwt: '',
      // eslint-disable-next-line camelcase -- OnlyOffice wire field name
      g_cAscSpellCheckUrl: '',
      buildVersion: '',
      buildNumber: 0,
      licenseType: 0,
      settings: {
        reconnection: { attempts: 50, delay: 2000 },
        binaryChanges: false,
        websocketMaxPayloadSize: 1572864,
        maxChangesSize: 157286400,
        // eslint-disable-next-line camelcase -- OnlyOffice wire field name
        limits_image_size: 26214400,
        // eslint-disable-next-line camelcase -- OnlyOffice wire field name
        limits_image_types_upload: 'jpg;jpeg;jpe;png;gif;bmp;svg;tiff;tif;webp;heic;heif;avif',
      },
      openedAt: editics.openedAt,
    };
    // The OnlyOffice `auth` event carries the chat history, which the editics
    // protocol keeps in the server event (the editor only renders the field
    // when non-empty, like for the `message` event).
    if (editics.messages && editics.messages.length !== 0) {
      event.messages = await this._cookMessageEntries(editics.messages);
    }
    return event;
  }

  /**
   * The server parks us behind the auth lock holder (single-editor to
   * co-editing transition, RFC §6.2): the editor must wait for the
   * established editor to release the lock with `unLockDocument{unlock:true}`.
   * @param {EditicsServerEventWaitAuth} editics
   * @returns {Promise<OOServerEventWaitAuth|null>}
   */
  async _cookWaitAuth(editics) {
    // `waitAuth` occurs *before*
    // The holder is necessarily in the participant table: the `connectState`
    // broadcast that announced our arrival carried it, and the SSE channel
    // preserves the ordering.

    let authLockedByhumanLabel = await this.getHumanLabelFromDeviceId(editics.authLockedBy.deviceId);
    if (!authLockedByhumanLabel) {
      // TODO: raise an error here instead ?
      //       `getHumanLabelFromDeviceId` should poll for new certificates to
      //       ensure the device ID exists; then, if not found, we should error out.
      console.warn(`waitAuth: unknown auth lock holder ${editics.authLockedBy}`);
      authLockedByhumanLabel = '<unknown>';
    }
    return {
      type: 'waitAuth',
      lockDocument: {
        id: editics.authLockedBy.id,
        idOriginal: editics.authLockedBy.deviceId,
        username: authLockedByhumanLabel,
        indexUser: editics.authLockedBy.indexUser,
        view: editics.authLockedBy.view,
        // Never provided by the editics protocol, and unused by the editor
        connectionId: '',
        // Unused remaining fields, always false
        isCloseCoAuthoring: false,
        isLiveViewer: false,
        encrypted: false,
      },
    };
  }

  /**
   * @param {EditicsServerEventConnectState} editics
   * @returns {Promise<OOServerEventConnectState>}
   */
  async _cookConnectState(editics) {
    return {
      type: 'connectState',
      participantsTimestamp: editics.participantsTimestamp,
      participants: await this._onlyofficeParticipants(editics.participants),
      waitAuth: editics.waitAuth,
    };
  }

  /**
   * The backlog of changes a newcomer must catch up on before its `auth`
   * reply completes the handshake (RFC §2.2).
   * @param {EditicsServerEventAuthChanges} editics
   * @returns {Promise<OOServerEventAuthChanges>}
   */
  async _cookAuthChanges(editics) {
    const changes = await Promise.all(
      (editics.changes || []).map(async (c) => {
        // The author's identity is carried by the record (`(participant id,
        // device id)` tuple): the backlog can contain changes from participants
        // that left the session before we joined (so they cannot be resolved
        // through the participants list).
        const [authorParticipantId, authorDeviceId] = c.author;
        const [keyIndex, b64CipherText] = c.encryptedChange;
        return {
          docid: this.docId,
          change: JSON.stringify(bytesToStr(await this.libparsec.decrypt(keyIndex, Uint8Array.fromBase64(b64CipherText)))),
          time: c.time,
          user: authorParticipantId,
          useridoriginal: authorDeviceId,
        };
      }),
    );
    return { type: 'authChanges', changes };
  }

  /**
   * @param {EditicsServerEventMessage['messages']} messages
   * @returns {Promise<Array<{docid:string, message:string, time:number, user:string, useridoriginal:string, username:string}>>}
   */
  async _cookMessageEntries(messages) {
    return await Promise.all(
      messages.map(async (m) => {
        // The author's identity is carried by the record (`(participant id,
        // device id)` tuple): a message can be replayed to a client that never
        // saw its author (the author left the session before it joined).
        const [authorParticipantId, authorDeviceId] = m.author;
        const [keyIndex, b64CipherText] = m.encryptedMessage;
        const username = (await this.getHumanLabelFromDeviceId(authorDeviceId)) || '<unknown>';
        return {
          docid: this.docId,
          message: bytesToStr(await this.libparsec.decrypt(keyIndex, Uint8Array.fromBase64(b64CipherText))),
          time: m.time,
          // The `user`/`useridoriginal` fields correspond to the author's
          // participant id / device id (like the `cursor` server event).
          user: authorParticipantId,
          useridoriginal: authorDeviceId,
          username,
        };
      }),
    );
  }

  /**
   * @param {EditicsServerEventMessage} editics
   * @returns {Promise<OOServerEventMessage>}
   */
  async _cookMessage(editics) {
    /** @type {OOServerEventMessage} */
    const event = { type: 'message' };
    if (editics.messages && editics.messages.length !== 0) {
      event.messages = await this._cookMessageEntries(editics.messages);
    }
    return event;
  }

  /**
   * The `user`/`useridoriginal` fields of the OnlyOffice protocol correspond
   * to the author's participant id / device id (i.e. the same values as the
   * `id`/`idOriginal` fields of the participants in the `auth`/`connectState`
   * events).
   * @param {EditicsServerEventCursor} editics
   * @returns {Promise<OOServerEventCursor>}
   */
  async _cookCursor(editics) {
    const messages = await Promise.all(
      (editics.messages || []).map(async (m) => {
        // The author's identity is carried by the record, like for a `message`.
        const [authorParticipantId, authorDeviceId] = m.author;
        const [keyIndex, b64CipherText] = m.encryptedCursor;
        return {
          cursor: bytesToStr(await this.libparsec.decrypt(keyIndex, Uint8Array.fromBase64(b64CipherText))),
          time: m.time,
          user: authorParticipantId,
          useridoriginal: authorDeviceId,
        };
      }),
    );
    return { type: 'cursor', messages };
  }

  /**
   * The lock table resulting from the server trying to acquire the requested
   * blocks for us (RFC §2.2).
   * @param {EditicsServerEventGetLock} editics
   * @returns {OOServerEventGetLock}
   */
  _cookGetLock(editics) {
    /** @type {Record<string, OOLockEntry>} */
    const ooLocks = {};
    for (const key in editics.locks || {}) {
      const lock = editics.locks[key];
      // The holder's identity is carried by the record (`(participant id,
      // device id)` tuple): the holder may have left the session, e.g. its
      // lock was released on disconnect.
      const [holderParticipantId] = lock.holder;
      ooLocks[key] = {
        time: lock.time,
        user: holderParticipantId,
        block: lock.block,
      };
    }
    return { type: 'getLock', locks: ooLocks };
  }

  /**
   * Region locks released by a participant outside of a `saveChanges`
   * (RFC §2.2).
   * @param {EditicsServerEventReleaseLock} editics
   * @returns {OOServerEventReleaseLock}
   */
  _cookReleaseLock(editics) {
    const locks = (editics.locks || []).map((lock) => {
      // The holder's identity is carried by the record, like in `getLock`.
      const [holderParticipantId] = lock.holder;
      return {
        block: lock.block,
        user: holderParticipantId,
        time: lock.time,
        changes: lock.changes,
      };
    });
    return { type: 'releaseLock', locks };
  }

  /**
   * OnlyOffice carries the lock table as an object keyed by block id in the
   * document editor, and as a plain array in the other editors.
   * @param {EditicsServerEventAuth['locks']} locks
   * @returns {Object<string, OOLockEntry>|Array<OOLockEntry>}
   */
  _onlyofficeLockTable(locks) {
    const records = (locks || []).map((lock) => {
      // The holder's identity is carried by the record, like in `getLock`.
      const [holderParticipantId] = lock.holder;
      return {
        time: lock.time,
        user: holderParticipantId,
        block: lock.block,
      };
    });
    if (this.config.editorType === 0) {
      /** @type {Record<string, OOLockEntry>} */
      const table = {};
      for (const record of records) {
        table[record.block] = record;
      }
      return table;
    } else {
      return records;
    }
  }

  /**
   * The `saveChanges` broadcast (RFC §2.2): another participant has saved
   * some changes. The saving participant itself doesn't receive this event
   * (it gets `unSaveLock`/`savePartChanges` replies instead).
   * @param {EditicsServerEventSaveChanges} editics
   * @returns {Promise<OOServerEventSaveChanges>}
   */
  async _cookServerSaveChanges(editics) {
    const changes = await Promise.all(
      (editics.changes || []).map(async (c) => {
        // The author's identity is carried by the record (`(participant id,
        // device id)` tuple), like for the `authChanges` entries: the author
        // may have left the session before we joined.
        const [authorParticipantId, authorDeviceId] = c.author;
        const [keyIndex, b64CipherText] = c.encryptedChange;
        return {
          docid: this.docId,
          // Each change fragment is an opaque string that OnlyOffice expects
          // JSON-encoded (e.g. `'"66;..."'`).
          change: JSON.stringify(bytesToStr(await this.libparsec.decrypt(keyIndex, Uint8Array.fromBase64(b64CipherText)))),
          time: c.time,
          // The `user`/`useridoriginal` fields correspond to the author's
          // participant id / device id (like the `cursor` server event).
          user: authorParticipantId,
          useridoriginal: authorDeviceId,
        };
      }),
    );
    const locks = (editics.locks || []).map((lock) => {
      // The holder's identity is carried by the record, like in `getLock`.
      const [holderParticipantId] = lock.holder;
      return {
        block: lock.block,
        user: holderParticipantId,
        time: lock.time,
        changes: lock.changes,
      };
    });
    /** @type {OOServerEventSaveChanges} */
    const oo = {
      type: 'saveChanges',
      changes,
      changesIndex: editics.changesIndex,
      syncChangesIndex: editics.syncChangesIndex,
      endSaveChanges: !!editics.endSaveChanges,
      locks,
    };
    if (editics.encryptedCursorAdditionalInfo !== null && editics.encryptedCursorAdditionalInfo !== undefined) {
      const [keyIndex, b64CipherText] = editics.encryptedCursorAdditionalInfo;
      oo.excelAdditionalInfo = bytesToStr(await this.libparsec.decrypt(keyIndex, Uint8Array.fromBase64(b64CipherText)));
    }
    return oo;
  }

  // --- Participant entries helpers -------------------------------------------

  /**
   * Convert the server's participant entries into the OnlyOffice participants
   * list, resolving the display names through the injected capabilities (the
   * server is NOT trusted for names, RFC §3.3).
   * @param {EditicsParticipantEntry[]} entries
   * @returns {Promise<OOParticipantEntry[]>}
   */
  async _onlyofficeParticipants(entries) {
    return await Promise.all(entries.map((entry) => this._onlyofficeParticipantEntry(entry)));
  }

  /**
   * @param {EditicsParticipantEntry} entry
   * @returns {Promise<OOParticipantEntry>}
   */
  async _onlyofficeParticipantEntry(entry) {
    const humanLabel = (await this.getHumanLabelFromDeviceId(entry.deviceId)) || '<unknown>';
    return {
      id: entry.id,
      idOriginal: entry.deviceId,
      username: humanLabel,
      indexUser: entry.indexUser,
      // Never provided by the editics protocol, and unused by the editor
      connectionId: '',
      view: !!entry.view,
      // The editics protocol has no separate flag: a viewer of a live session
      // is a "live viewer" (the OnlyOffice editor uses this to know it must
      // relay the changes to the participant).
      isLiveViewer: !!entry.view,
      isCloseCoAuthoring: false,
      encrypted: false,
    };
  }
}

export { EditicsTranslator };

// ---------------------------------------------------------------------------
// Editics protocol types
// ---------------------------------------------------------------------------

// /!\ Auto-generated code (see `misc/check_editics_protocol_consistency.py`), any modification will be lost ! /!\
/* eslint-disable max-len */

/**
 * @typedef {number} EditicsParticipantIndex
 *
 * @typedef {string} EditicsParticipantID
 *
 * @typedef {Object} EditicsParticipantEntry
 * @property {EditicsParticipantID} id
 * @property {EditicsParticipantIndex} indexUser
 * @property {string} deviceId
 * @property {boolean} view
 *
 * @typedef {Object} EditicsClientEventAuth
 * @property {'auth'} type
 * @property {number} vlobVersion
 * @property {EditicsParticipantID|null} reconnect
 * @property {'view'|'edit'} mode
 *
 * @typedef {Object} EditicsClientEventAuthChangesAck
 * @property {'authChangesAck'} type
 *
 * @typedef {Object} EditicsClientEventGetMessages
 * @property {'getMessages'} type
 *
 * @typedef {Object} EditicsClientEventMessage
 * @property {'message'} type
 * @property {[number, string]} encryptedMessage
 *
 * @typedef {Object} EditicsClientEventCursor
 * @property {'cursor'} type
 * @property {[number, string]} encryptedCursor
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
 * @property {Array<[number, string]>} encryptedChanges
 * @property {boolean} startSaveChanges
 * @property {boolean} endSaveChanges
 * @property {number|null} deleteIndex
 * @property {Object<string,*>|null} excelAdditionalInfo
 * @property {[number, string]|null} encryptedCursorAdditionalInfo
 * @property {boolean} [releaseLocks]
 *
 * @typedef {Object} EditicsClientEventUnSaveLock
 * @property {'unSaveLock'} type
 *
 * @typedef {Object} EditicsClientEventUnLockDocument
 * @property {'unLockDocument'} type
 * @property {boolean} [isSave]
 * @property {boolean} [unlock]
 * @property {number|null} deleteIndex
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
 * @typedef {EditicsClientEventAuth|EditicsClientEventAuthChangesAck|EditicsClientEventMessage|EditicsClientEventGetMessages|EditicsClientEventCursor|EditicsClientEventGetLock|EditicsClientEventIsSaveLock|EditicsClientEventSaveChanges|EditicsClientEventUnSaveLock|EditicsClientEventUnLockDocument|EditicsClientEventClose|EditicsClientEventSaveDone} EditicsClientEvent
 *
 * @typedef {Object} EditicsServerEventAuth
 * @property {'auth'} type
 * @property {EditicsParticipantEntry[]} participants
 * @property {EditicsParticipantID} participantId
 * @property {EditicsParticipantIndex} indexUser
 * @property {number} participantTimeConnect
 * @property {number} openedAt
 * @property {Array<{time:number, holder:[EditicsParticipantID, string], block:*}>} [locks]
 * @property {Array<{time:number, author:[EditicsParticipantID, string], encryptedMessage:[number, string]}>} [messages]
 *
 * @typedef {Object} EditicsServerEventConnectState
 * @property {'connectState'} type
 * @property {number} participantsTimestamp
 * @property {EditicsParticipantEntry[]} participants
 * @property {boolean} waitAuth
 *
 * @typedef {Object} EditicsServerEventAuthChanges
 * @property {'authChanges'} type
 * @property {Array<{index:number, author:[EditicsParticipantID, string], time:number, encryptedChange:[number, string]}>} [changes]
 *
 * @typedef {Object} EditicsServerEventWaitAuth
 * @property {'waitAuth'} type
 * @property {EditicsParticipantEntry} authLockedBy
 *
 * @typedef {Object} EditicsServerEventMessage
 * @property {'message'} type
 * @property {Array<{time:number, author:[EditicsParticipantID, string], encryptedMessage:[number, string]}>} messages
 *
 * @typedef {Object} EditicsServerEventCursor
 * @property {'cursor'} type
 * @property {Array<{time:number, author:[EditicsParticipantID, string], encryptedCursor:[number, string]}>} messages
 *
 * @typedef {Object} EditicsServerEventGetLock
 * @property {'getLock'} type
 * @property {Record<string, {time:number, holder:[EditicsParticipantID, string], block:*}>} locks
 *
 * @typedef {Object} EditicsServerEventReleaseLock
 * @property {'releaseLock'} type
 * @property {Array<{block:*, holder:[EditicsParticipantID, string], time:number, changes:null}>} locks
 *
 * @typedef {Object} EditicsServerEventSaveLock
 * @property {'saveLock'} type
 * @property {boolean} lockedBySomebodyElse
 *
 * @typedef {Object} EditicsServerEventSaveChanges
 * @property {'saveChanges'} type
 * @property {Array<{time:number, author:[EditicsParticipantID, string], encryptedChange:[number, string]}>} changes
 * @property {number} changesIndex
 * @property {number} syncChangesIndex
 * @property {boolean} endSaveChanges
 * @property {Array<{block:*, holder:[EditicsParticipantID, string], time:number, changes:null}>} [locks]
 * @property {Object<string,*>|null} excelAdditionalInfo
 * @property {[number, string]|null} encryptedCursorAdditionalInfo
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
 * @property {string} description
 *
 * @typedef {Object} EditicsServerEventWarning
 * @property {'warning'} type
 * @property {number} code
 * @property {string} message
 *
 * @typedef {Object} EditicsServerEventError
 * @property {'error'} type
 * @property {number} code
 * @property {string} description
 *
 * @typedef {EditicsServerEventAuth|EditicsServerEventConnectState|EditicsServerEventAuthChanges|EditicsServerEventWaitAuth|EditicsServerEventMessage|EditicsServerEventCursor|EditicsServerEventGetLock|EditicsServerEventReleaseLock|EditicsServerEventSaveLock|EditicsServerEventSaveChanges|EditicsServerEventSavePartChanges|EditicsServerEventUnSaveLock|EditicsServerEventDrop|EditicsServerEventWarning|EditicsServerEventError} EditicsServerEvent
 */

/* eslint-enable max-len */
// /!\ End of auto-generated code /!\
