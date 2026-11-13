// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

// cspell: words: Visio, useridoriginal, CAAAADEAMQAzADcAAAAAAA

// Here we define `EditicsTranslator` which works as an interface between the
// OnlyOffice client (speaking the OnlyOffice protocol) and the Parsec client/server
// communication (speaking the Editics protocol).
//
// The Editics protocol is basically the OnlyOffice protocol plus an encryption
// layer to protect the sensitive fields (e.g. document modification).
//
// Note the file is used in the server tests in order to replay real OnlyOffice
// records (using a Python-controlled embedded V8 runtime, see
// server/tests/editics/test_records.py).
// This is also why this code is in Javascript: we don't want Python tests to
// depend on a the Typescript transpiler.

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

const [textEncoder, textDecoder] = [new TextEncoder(), new TextDecoder()];

/**
 * UTF-8 encode a JS string into bytes (standard Web API: `TextEncoder`).
 * @param {string} s
 * @returns {Uint8Array}
 */
function bytesFromStr(s) {
  return textEncoder.encode(s);
}

/**
 * UTF-8 decode bytes into a JS string (standard Web API: `TextDecoder`).
 * @param {Uint8Array} bytes
 * @returns {string}
 */
function bytesToStr(bytes) {
  return textDecoder.decode(bytes);
}

// ---------------------------------------------------------------------------
// EditicsTranslator
// ---------------------------------------------------------------------------

/**
 * Result of a translation.
 *
 * Any occurring error is considered as not recoverable and the integrator is
 * expected to close the editics session and inform the end user.
 * @template T
 * @typedef {{ok: true, value: T} | {ok: false, error: string, kind: 'libparsec'|'translation'}} EditicsResult<T>
 */

/**
 * @template T
 * @typedef {{ok: true, value: T} | {ok: false, error: string, kind: 'libparsec'}} EditicsLibparsecResult<T>
 */

/**
 * The libparsec operations the translator depends on, provided by the integrator.
 * @typedef {Object} EditicsLibparsecDeps
 * @property {(deviceIdHex: string) => Promise<EditicsLibparsecResult<string>>} getHumanLabelFromDeviceId
 * @property {(cleartext: Uint8Array) => Promise<[number, Uint8Array]>} encrypt
 * @property {(authorDeviceId: string, keyIndex: number, ciphertext: Uint8Array) => Promise<EditicsLibparsecResult<Uint8Array>>} decrypt
 *
 * @typedef {Object} EditicsTranslatorConfig
 * @property {string} workspaceId - WorkspaceID (VlobID) hex.
 * @property {string} vlobId - VlobID hex of the document.
 * @property {string} deviceIdHex - DeviceID hex of the client's device.
 * @property {string} userId - Integrator user id (the same `user.id` the editor
 * config gets, so the composite user IDs match).
 * @property {string} userName - Integrator user display name.
 * @property {number} vlobVersion - loaded vlob version.
 * @property {number} editorType - 0=Word, 1=Spreadsheet, 2=Presentation, 3=Visio.
 */

/**
 * One OnlyOffice spreadsheet recalc record, as carried in the `indexCols`/`indexRows`
 * keys of the OnlyOffice `excelAdditionalInfo` blob (OnlyOffice `CRecalcIndexElement`
 * wire shape: the translator moves those records around, but never inspects them).
 * @typedef {{_recalcType: 1|2, _position: number, _count: number, m_bIsSaveIndex: boolean}} OORecalcIndexElement
 */

/**
 * The OnlyOffice `indexCols`/`indexRows` shape (only sent by the spreadsheet editor):
 * sheet id -> its recalc records.
 * @typedef {Record<string, {_arrElements: Array<OORecalcIndexElement>}>} OORecalcIndex
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
   * @returns {Promise<EditicsResult<string>>}
   */
  async getHumanLabelFromDeviceId(deviceIdHex) {
    const found = this._humanLabelFromDeviceIdCache.get(deviceIdHex);
    if (found !== undefined) {
      return { ok: true, value: found };
    }
    // Cache miss
    const result = await this.libparsec.getHumanLabelFromDeviceId(deviceIdHex);
    if (result.ok) {
      // Overwrite any concurrent insertion since the result should be always the same.
      this._humanLabelFromDeviceIdCache.set(deviceIdHex, result.value);
    }
    return result;
  }

  /**
   * Combine the results of parallel operations into a single result: the first
   * `{ok: false}` short-circuits (all the results have already been awaited,
   * since failures are not expected to be common enough to be worth cancelling).
   * @template T
   * @param {Array<EditicsResult<T>>} results
   * @returns {EditicsResult<T[]>}
   */
  _collectResults(results) {
    const values = [];
    for (const result of results) {
      if (!result.ok) {
        return result;
      }
      values.push(result.value);
    }
    return { ok: true, value: values };
  }

  /**
   * @param {EditicsParticipantIdentity} identity `(device id, index)` tuple
   * @returns {string}
   */
  _onlyofficeUserId(identity) {
    const [deviceId, indexUser] = identity;
    return `${deviceId}${indexUser}`;
  }

  // --- Client events translation -------------------------------------------

  /**
   * Convert a client event from OnlyOffice to Editics protocol.
   *
   * This is used when the OnlyOffice editor wants to communicate with the server,
   * and the returned Editics event should then be send to the Parsec server.
   *
   * Returns a `null` value if the event has no Editics counterpart (e.g. `rpc`).
   *
   * @param {OOClientEvent} oo
   * @returns {Promise<EditicsResult<EditicsClientEvent|null>>}
   */
  async cookClientEvent(oo) {
    return this._cookClientEvent(oo);
  }

  /**
   * @param {OOClientEvent} oo
   * @returns {Promise<EditicsResult<EditicsClientEvent|null>>}
   */
  async _cookClientEvent(oo) {
    switch (oo.type) {
      case 'auth': {
        /** @type {EditicsClientEventAuth} */
        const event = {
          type: 'auth',
          mode: oo.mode === 'edit' ? 'edit' : 'view',
          reconnect: null,
        };
        if (typeof oo.sessionId === 'string') {
          event.reconnect = {
            participantSessionId: oo.sessionId,
            lastOtherSaveTime: oo.lastOtherSaveTime !== -1 ? oo.lastOtherSaveTime : null,
          };
        }
        return { ok: true, value: event };
      }

      case 'close': {
        return { ok: true, value: { type: 'close' } };
      }

      case 'getMessages': {
        return {
          ok: true,
          value: {
            type: 'getMessages',
          },
        };
      }

      case 'authChangesAck': {
        return { ok: true, value: { type: 'authChangesAck' } };
      }

      case 'getLock': {
        // The OnlyOffice and editics block types are structurally compatible,
        // but the former is looser (hence the cast).
        return { ok: true, value: { type: 'getLock', block: /** @type {EditicsBlock[]} */ (oo.block) } };
      }

      case 'message': {
        const [msgKeyIndex, msgCipherText] = await this.libparsec.encrypt(bytesFromStr(oo.message));
        return {
          ok: true,
          value: {
            type: 'message',
            encryptedMessage: [msgKeyIndex, msgCipherText.toBase64()],
          },
        };
      }

      case 'cursor': {
        // OnlyOffice provides the cursor as a base64-encoded payload in a
        // custom format (e.g. `"16;CAAAADEAMQAzADcAAAAAAA=="`).
        // We consider this format opaque and encrypt it as-is.
        const [keyIndex, cipherText] = await this.libparsec.encrypt(bytesFromStr(oo.cursor));
        return {
          ok: true,
          value: {
            type: 'cursor',
            encryptedCursor: [keyIndex, cipherText.toBase64()],
          },
        };
      }

      case 'unLockDocument': {
        /** @type {EditicsClientEventUnLockDocument} */
        const event = {
          type: 'unLockDocument',
          isSave: oo.isSave,
          unlock: oo.unlock,
          // OnlyOffice uses `-1` as no-truncation marker, editics uses `null`.
          deleteIndex: typeof oo.deleteIndex === 'number' && oo.deleteIndex >= 0 ? oo.deleteIndex : null,
          releaseLocks: oo.releaseLocks ?? false,
        };
        // `unLockDocument` is supposed to be only send by editors, however the
        // spreadsheet editor always (i.e. also for viewers) send a all-false
        // `unLockDocument` at document load.
        //
        // Since an all-false `unLockDocument` is handled as a noop on the server,
        // we just skip it in order to simplify server's access control.
        if (!event.isSave && !event.unlock && event.deleteIndex === null && !event.releaseLocks) {
          return { ok: true, value: null };
        }
        return { ok: true, value: event };
      }

      case 'isSaveLock': {
        return { ok: true, value: { type: 'isSaveLock', syncChangesIndex: oo.syncChangesIndex } };
      }

      case 'saveChanges': {
        return { ok: true, value: await this._cookSaveChanges(oo) };
      }

      case 'unSaveLock': {
        return { ok: true, value: { type: 'unSaveLock' } };
      }

      // Ignored events
      case 'clientLog': {
        return { ok: true, value: null };
      }

      case 'extendSession': {
        return { ok: true, value: null };
      }

      case 'forceSaveStart': {
        return { ok: true, value: null };
      }

      case 'openDocument': {
        return { ok: true, value: null };
      }

      case 'rpc': {
        return { ok: true, value: null };
      }

      default: {
        return {
          ok: false,
          kind: 'translation',
          error: `Unknown OnlyOffice protocol event ${JSON.stringify(oo)}`,
        };
      }
    }
  }

  /**
   * @param {OOClientEventSaveChanges} oo
   * @returns {Promise<EditicsClientEventSaveChanges>}
   */
  async _cookSaveChanges(oo) {
    // OnlyOffice editor is always configured in JSON change mode (i.e.
    // `binaryChanges: false`), so `changes` is a JSON-serialized array of
    // `<cleartext length>;<base64(cleartext)>` fragments.
    const fragments = JSON.parse(/** @type string */ (oo.changes));
    /** @type {[number, string][]} */
    const encryptedChanges = [];
    for (const fragment of fragments) {
      // The length prefix is redundant with the base64 payload, only the
      // base64 part carries information.
      const bytes = Uint8Array.fromBase64(fragment.slice(fragment.indexOf(';') + 1));
      const [keyIndex, cipherText] = await this.libparsec.encrypt(bytes);
      encryptedChanges.push([keyIndex, cipherText.toBase64()]);
    }

    // In OnlyOffice protocol, the `excelAdditionalInfo` (despite its name) is
    // carried by all the editor types.
    // - The spreadsheet editor only sends it when there is something to carry:
    //   the `indexCols`/`indexRows` lock-shift info when columns/rows were
    //   inserted (the receivers use it to recalculate their locked ranges),
    //   and the `CursorInfo` cursor keys only when it has a document position.
    //   So a save can carry lock-shift info without any cursor part, and vice versa.
    // - The word/presentation editors always carry the cursor keys (and never the
    //   lock-shift info), with `CursorInfo` null when the editor is not in
    //   fast co-editing mode or has no document position.
    //
    // So editics protocol splits this into three fields:
    // - `excelAdditionalInfoIndex[Cols|Rows]` that contains the spreadsheet-specific infos.
    // - `encryptedCursorAdditionalInfo` that contains the cursor info use by all editors.
    /** @type {OORecalcIndex|null} */
    let excelAdditionalInfoIndexCols = null;
    /** @type {OORecalcIndex|null} */
    let excelAdditionalInfoIndexRows = null;
    /** @type {[number, string] | null} */
    let encryptedCursor = null;
    if (oo.excelAdditionalInfo !== null && oo.excelAdditionalInfo !== undefined) {
      const parsed = JSON.parse(oo.excelAdditionalInfo);
      excelAdditionalInfoIndexCols = parsed.indexCols ?? null;
      excelAdditionalInfoIndexRows = parsed.indexRows ?? null;
      /** @type {Record<string, any>} */
      const cursorPart = {};
      for (const key of Object.keys(parsed)) {
        if (key !== 'indexCols' && key !== 'indexRows') cursorPart[key] = parsed[key];
      }
      if (Object.keys(cursorPart).length > 0) {
        const [keyIndex, cipherText] = await this.libparsec.encrypt(bytesFromStr(JSON.stringify(cursorPart)));
        encryptedCursor = [keyIndex, cipherText.toBase64()];
      }
    }
    return {
      type: 'saveChanges',
      encryptedChanges,
      startSaveChanges: oo.startSaveChanges,
      endSaveChanges: oo.endSaveChanges,
      // OnlyOffice uses `-1` as no-truncation marker, editics uses `null`.
      deleteIndex: typeof oo.deleteIndex === 'number' && oo.deleteIndex >= 0 ? oo.deleteIndex : null,
      excelAdditionalInfoIndexCols,
      excelAdditionalInfoIndexRows,
      encryptedCursorAdditionalInfo: encryptedCursor,
      releaseLocks: oo.releaseLocks ?? false,
    };
  }

  // --- Sever events translation --------------------------------------------

  /**
   * Convert a server event from Editics to OnlyOffice protocol.
   *
   * This is used when the server wants to communicate with the client, and the
   * returned OnlyOffice event should then be ingested by the OnlyOffice client.
   *
   * @param {EditicsServerEvent} editics
   * @returns {Promise<EditicsResult<OOServerEvent>>}
   */
  async cookServerEvent(editics) {
    return this._cookServerEvent(editics);
  }

  /**
   * @param {EditicsServerEvent} editics
   * @returns {Promise<EditicsResult<OOServerEvent>>}
   */
  async _cookServerEvent(editics) {
    switch (editics.type) {
      case 'auth': {
        return await this._cookServerAuth(editics);
      }

      case 'waitAuth': {
        return await this._cookWaitAuth(editics);
      }

      case 'connectState': {
        return this._cookConnectState(editics);
      }

      case 'message': {
        return await this._cookMessage(editics);
      }

      case 'cursor': {
        return await this._cookCursor(editics);
      }

      case 'saveLock': {
        // Reply to our `isSaveLock`: the `saveLock` field is confusingly
        // named, `false` means the save lock is granted (i.e. we may now send
        // our `saveChanges`), `true` means it is held by somebody else.
        return { ok: true, value: { type: 'saveLock', saveLock: editics.lockedBySomebodyElse } };
      }

      case 'savePartChanges': {
        return {
          ok: true,
          value: {
            type: 'savePartChanges',
            changesIndex: editics.changesIndex,
            syncChangesIndex: editics.syncChangesIndex,
          },
        };
      }

      case 'unSaveLock': {
        return {
          ok: true,
          value: {
            type: 'unSaveLock',
            // `index`/`time`/`syncChangesIndex` fields gets the `-1` sentinel
            // value if the save have been canceled or did not establish a new
            // save point.
            index: editics.index ?? -1,
            time: editics.time ?? -1,
            syncChangesIndex: editics.syncChangesIndex ?? -1,
          },
        };
      }

      case 'saveChanges': {
        return await this._cookServerSaveChanges(editics);
      }

      case 'authChanges': {
        return await this._cookAuthChanges(editics);
      }

      case 'getLock': {
        return { ok: true, value: this._cookGetLock(editics) };
      }

      case 'releaseLock': {
        return { ok: true, value: this._cookReleaseLock(editics) };
      }

      case 'error': {
        // Reply to a refused session restoration (e.g. the document has been
        // modified while the participant was away, code 4010 "Restore error.
        // Document modified."): the editor reacts by closing the co-authoring
        // session.
        return {
          ok: true,
          value: {
            type: 'error',
            code: editics.code,
            description: editics.description,
          },
        };
      }

      case 'warning': {
        return { ok: true, value: { type: 'warning', code: editics.code, message: editics.message } };
      }

      case 'drop': {
        // `code` is always the DROP_CODE constant (4007) for a `drop` event,
        // OnlyOffice includes it only for symmetry with `disconnectReason`.
        return { ok: true, value: { type: 'drop', code: 4007, description: editics.description } };
      }

      case 'disconnectReason': {
        return {
          ok: true,
          value: {
            type: 'disconnectReason',
            code: editics.code,
            description: editics.description,
          },
        };
      }

      default: {
        return {
          ok: false,
          kind: 'translation',
          error: `Unknown Editics protocol event ${JSON.stringify(editics)}`,
        };
      }
    }
  }

  /**
   * @param {EditicsServerEventAuth} editics
   * @returns {Promise<EditicsResult<OOServerEventAuth>>}
   */
  async _cookServerAuth(editics) {
    const participants = await this._onlyofficeParticipants(editics.participants);
    if (!participants.ok) {
      return participants;
    }
    /** @type {OOServerEventAuth} */
    const event = {
      type: 'auth',
      result: 1,
      participants: participants.value,
      sessionId: editics.participantSessionId,
      indexUser: editics.participantIndexUser,
      sessionTimeConnect: editics.participantSessionTimeConnect,
      openedAt: editics.openedAt,
      locks: this._onlyofficeLockTable(editics.locks),
      hasForgotten: false,
      jwt: '',
      // eslint-disable-next-line camelcase -- OnlyOffice wire field name
      g_cAscSpellCheckUrl: '',
      buildVersion: '',
      buildNumber: 0,
      licenseType: 0,
      settings: {
        reconnection: { attempts: 50, delay: 2000 },
        // It would be tempting to use the binary change mode here (faster&more
        // convenient since we are going to encrypt the changes).
        // However this flag is never used in the original OnlyOffice editor (and
        // is in fact buggy!), so we stick with the battle-tested default mode
        // where changes are encoded as a `"<length>;<base64>"` strings.
        binaryChanges: false,
        websocketMaxPayloadSize: 1572864,
        maxChangesSize: 157286400,
        // eslint-disable-next-line camelcase -- OnlyOffice wire field name
        limits_image_size: 26214400,
        // eslint-disable-next-line camelcase -- OnlyOffice wire field name
        limits_image_types_upload: 'jpg;jpeg;jpe;png;gif;bmp;svg;tiff;tif;webp;heic;heif;avif',
      },
    };
    if (editics.messages.length !== 0) {
      const messages = await this._cookMessageEntries(editics.messages);
      if (!messages.ok) {
        return messages;
      }
      event.messages = messages.value;
    }
    return { ok: true, value: event };
  }

  /**
   * @param {EditicsServerEventWaitAuth} editics
   * @returns {Promise<EditicsResult<OOServerEventWaitAuth>>}
   */
  async _cookWaitAuth(editics) {
    const humanLabel = await this.getHumanLabelFromDeviceId(editics.authLockedBy.deviceId);
    if (!humanLabel.ok) {
      return humanLabel;
    }
    return {
      ok: true,
      value: {
        type: 'waitAuth',
        lockDocument: {
          id: `${editics.authLockedBy.deviceId}${editics.authLockedBy.indexUser}`,
          idOriginal: editics.authLockedBy.deviceId,
          username: humanLabel.value,
          indexUser: editics.authLockedBy.indexUser,
          view: editics.authLockedBy.view,
          // Field never used by the editor (OnlyOffice server leaks internal data)
          connectionId: '',
          // Unused remaining fields, always false
          isCloseCoAuthoring: false,
          isLiveViewer: false,
          encrypted: false,
        },
      },
    };
  }

  /**
   * @param {EditicsServerEventConnectState} editics
   * @returns {Promise<EditicsResult<OOServerEventConnectState>>}
   */
  async _cookConnectState(editics) {
    const participants = await this._onlyofficeParticipants(editics.participants);
    if (!participants.ok) {
      return participants;
    }
    return {
      ok: true,
      value: {
        type: 'connectState',
        participantsTimestamp: editics.participantsTimestamp,
        participants: participants.value,
        waitAuth: editics.waitAuth,
      },
    };
  }

  /**
   * @param {EditicsServerEventAuthChanges} editics
   * @returns {Promise<EditicsResult<OOServerEventAuthChanges>>}
   */
  async _cookAuthChanges(editics) {
    const changes = this._collectResults(
      await Promise.all(
        editics.changes.map(async (c) => {
          const [authorDeviceId, _authorIndexUser] = c.author;
          const [keyIndex, b64CipherText] = c.encryptedChange;
          const cleartext = await this.libparsec.decrypt(authorDeviceId, keyIndex, Uint8Array.fromBase64(b64CipherText));
          if (!cleartext.ok) {
            return cleartext;
          }
          return {
            ok: true,
            value: {
              docid: this.docId,
              // OnlyOffice editor is always configured in JSON change mode (i.e.
              // `binaryChanges: false`), so `changes` is a JSON-serialized array of
              // `<cleartext length>;<base64(cleartext)>` fragments.
              change: JSON.stringify(`${cleartext.value.byteLength};${cleartext.value.toBase64()}`),
              time: c.time,
              user: this._onlyofficeUserId(c.author),
              useridoriginal: authorDeviceId,
            },
          };
        }),
      ),
    );
    if (!changes.ok) {
      return changes;
    }
    return { ok: true, value: { type: 'authChanges', changes: changes.value } };
  }

  /**
   * @param {EditicsServerEventMessage['messages']} messages
   * @returns {Promise<EditicsResult<Array<{
   *     docid: string,
   *     message: string,
   *     time: number,
   *     user: string,
   *     useridoriginal: string,
   *     username: string,
   *   }>>>}
   */
  async _cookMessageEntries(messages) {
    return this._collectResults(
      await Promise.all(
        messages.map(async (m) => {
          const [authorDeviceId, _authorIndexUser] = m.author;
          const [keyIndex, b64CipherText] = m.encryptedMessage;
          const username = await this.getHumanLabelFromDeviceId(authorDeviceId);
          if (!username.ok) {
            return username;
          }
          const cleartext = await this.libparsec.decrypt(authorDeviceId, keyIndex, Uint8Array.fromBase64(b64CipherText));
          if (!cleartext.ok) {
            return cleartext;
          }
          return {
            ok: true,
            value: {
              docid: this.docId,
              message: bytesToStr(cleartext.value),
              time: m.time,
              user: this._onlyofficeUserId(m.author),
              useridoriginal: authorDeviceId,
              username: username.value,
            },
          };
        }),
      ),
    );
  }

  /**
   * @param {EditicsServerEventMessage} editics
   * @returns {Promise<EditicsResult<OOServerEventMessage>>}
   */
  async _cookMessage(editics) {
    /** @type {OOServerEventMessage} */
    const event = { type: 'message' };
    if (editics.messages.length !== 0) {
      const messages = await this._cookMessageEntries(editics.messages);
      if (!messages.ok) {
        return messages;
      }
      event.messages = messages.value;
    }
    return { ok: true, value: event };
  }

  /**
   * @param {EditicsServerEventCursor} editics
   * @returns {Promise<EditicsResult<OOServerEventCursor>>}
   */
  async _cookCursor(editics) {
    const cleartext = await this.libparsec.decrypt(
      editics.author[0],
      editics.encryptedCursor[0],
      Uint8Array.fromBase64(editics.encryptedCursor[1]),
    );
    if (!cleartext.ok) {
      return cleartext;
    }
    return {
      ok: true,
      value: {
        type: 'cursor',
        // The editics protocol provides a single cursor movement, while the
        // OnlyOffice protocol expects a (single-element) list of messages.
        messages: [
          {
            cursor: bytesToStr(cleartext.value),
            time: editics.time,
            user: this._onlyofficeUserId(editics.author),
            useridoriginal: editics.author[0],
          },
        ],
      },
    };
  }

  /**
   * @param {EditicsServerEventGetLock} editics
   * @returns {OOServerEventGetLock}
   */
  _cookGetLock(editics) {
    /** @type {Record<string, OOLockEntry>} */
    const ooLocks = {};
    for (const key in editics.locks) {
      const lock = editics.locks[key];
      ooLocks[key] = {
        time: lock.time,
        user: this._onlyofficeUserId(lock.holder),
        block: lock.block,
      };
    }
    return { type: 'getLock', locks: ooLocks };
  }

  /**
   * @param {EditicsServerEventReleaseLock} editics
   * @returns {OOServerEventReleaseLock}
   */
  _cookReleaseLock(editics) {
    const locks = editics.locks.map((lock) => {
      return {
        block: lock.block,
        user: this._onlyofficeUserId(lock.holder),
        time: lock.time,
        changes: lock.changes,
      };
    });
    return { type: 'releaseLock', locks };
  }

  /**
   * @param {EditicsServerEventAuth['locks']} locks
   * @returns {Object<string, OOLockEntry>|Array<OOLockEntry>}
   */
  _onlyofficeLockTable(locks) {
    const records = locks.map((lock) => {
      return {
        time: lock.time,
        user: this._onlyofficeUserId(lock.holder),
        block: lock.block,
      };
    });
    if (this.config.editorType === 0) {
      /** @type {Record<string, OOLockEntry>} */
      const table = {};
      for (const record of records) {
        // In the word editor the block is the lock table key itself.
        table[/** @type {string} */ (record.block)] = record;
      }
      return table;
    } else {
      return records;
    }
  }

  /**
   * @param {EditicsServerEventSaveChanges} editics
   * @returns {Promise<EditicsResult<OOServerEventSaveChanges>>}
   */
  async _cookServerSaveChanges(editics) {
    const author = editics.author;
    const authorDeviceId = author[0];
    const changes = this._collectResults(
      await Promise.all(
        editics.changes.map(async (c) => {
          const [keyIndex, b64CipherText] = c.encryptedChange;
          const cleartext = await this.libparsec.decrypt(authorDeviceId, keyIndex, Uint8Array.fromBase64(b64CipherText));
          if (!cleartext.ok) {
            return cleartext;
          }
          return {
            ok: true,
            value: {
              docid: this.docId,
              // OnlyOffice editor is always configured in JSON change mode (i.e.
              // `binaryChanges: false`), so `changes` is a JSON-serialized array of
              // `<cleartext length>;<base64(cleartext)>` fragments.
              change: JSON.stringify(`${cleartext.value.byteLength};${cleartext.value.toBase64()}`),
              time: c.time,
              user: this._onlyofficeUserId(author),
              useridoriginal: authorDeviceId,
            },
          };
        }),
      ),
    );
    if (!changes.ok) {
      return changes;
    }
    const locks = editics.locks.map((lock) => {
      return {
        block: lock.block,
        user: this._onlyofficeUserId(lock.holder),
        time: lock.time,
        changes: lock.changes,
      };
    });
    /** @type {OOServerEventSaveChanges} */
    const oo = {
      type: 'saveChanges',
      changes: changes.value,
      changesIndex: editics.changesIndex,
      syncChangesIndex: editics.syncChangesIndex,
      endSaveChanges: editics.endSaveChanges,
      locks,
    };
    // Reassemble the OnlyOffice blob from its editics parts (the mirror of the
    // client -> server split): the spreadsheet lock-shift keys first (when the
    // save carries them), then the decrypted cursor keys.
    let hasExcelAdditionalInfo = false;
    let excelObj = {};
    const hasIndexCols = editics.excelAdditionalInfoIndexCols !== null;
    const hasIndexRows = editics.excelAdditionalInfoIndexRows !== null;
    if (hasIndexCols || hasIndexRows) {
      // On a structural-change save the OnlyOffice blob always carries both
      // keys (the unchanged axis as `null`), re-emit it as-is.
      excelObj.indexCols = editics.excelAdditionalInfoIndexCols;
      excelObj.indexRows = editics.excelAdditionalInfoIndexRows;
      hasExcelAdditionalInfo = true;
    }
    if (editics.encryptedCursorAdditionalInfo !== null) {
      const [keyIndex, b64CipherText] = editics.encryptedCursorAdditionalInfo;
      const cleartext = await this.libparsec.decrypt(authorDeviceId, keyIndex, Uint8Array.fromBase64(b64CipherText));
      if (!cleartext.ok) {
        return cleartext;
      }
      excelObj = Object.assign(excelObj, JSON.parse(bytesToStr(cleartext.value)));
      hasExcelAdditionalInfo = true;
    }
    if (hasExcelAdditionalInfo) {
      oo.excelAdditionalInfo = JSON.stringify(excelObj);
    }
    return { ok: true, value: oo };
  }

  // --- Participant entries helpers -------------------------------------------

  /**
   * @param {EditicsParticipantEntry[]} entries
   * @returns {Promise<EditicsResult<OOParticipantEntry[]>>}
   */
  async _onlyofficeParticipants(entries) {
    return this._collectResults(await Promise.all(entries.map((entry) => this._onlyofficeParticipantEntry(entry))));
  }

  /**
   * @param {EditicsParticipantEntry} entry
   * @returns {Promise<EditicsResult<OOParticipantEntry>>}
   */
  async _onlyofficeParticipantEntry(entry) {
    const humanLabel = await this.getHumanLabelFromDeviceId(entry.deviceId);
    if (!humanLabel.ok) {
      return humanLabel;
    }
    return {
      ok: true,
      value: {
        id: `${entry.deviceId}${entry.indexUser}`,
        idOriginal: entry.deviceId,
        username: humanLabel.value,
        indexUser: entry.indexUser,
        // Field never used by the editor (OnlyOffice server leaks internal data)
        connectionId: '',
        view: entry.view,
        // The editics protocol has no separate flag: a viewer of a live session
        // is a "live viewer" (the OnlyOffice editor uses this to know it must
        // relay the changes to the participant).
        isLiveViewer: entry.view,
        isCloseCoAuthoring: false,
        encrypted: false,
      },
    };
  }
}

export { EditicsTranslator };

// ---------------------------------------------------------------------------
// Editics protocol types
// ---------------------------------------------------------------------------

// /!\ Auto-generated code (see `misc/gen_client_editics_protocol.py`), any modification will be lost ! /!\
// The editics protocol API version spoken by this client: sent as the value of
// the `Api-Version` HTTP header on every editics request (mirror of
// `EDITICS_API_VERSION` in `server/parsec/editics_protocol.py`).
export const EDITICS_API_VERSION = '1.0';

/* eslint-disable max-len */

/**
 * @typedef {number} EditicsParticipantIndex
 *
 * @typedef {string} EditicsParticipantSessionID
 *
 * @typedef {[string, EditicsParticipantIndex]} EditicsParticipantIdentity
 *
 * @typedef {number} EditicsTimestampMs
 *
 * @typedef {string} EditicsRegionLockID
 *
 * @typedef {Object} EditicsExcelRangeOrObjectID
 * @property {number} c1
 * @property {number} r1
 * @property {number} c2
 * @property {number} r2
 *
 * @typedef {Object} EditicsExcelBlock
 * @property {EditicsRegionLockID} guid
 * @property {string} sheetId
 * @property {number} type
 * @property {number|null} subType
 * @property {EditicsExcelRangeOrObjectID} rangeOrObjectId
 *
 * @typedef {Object} EditicsPresentationBlock
 * @property {EditicsRegionLockID} guid
 * @property {number} type
 * @property {string} slideId
 * @property {string} objId
 *
 * @typedef {EditicsRegionLockID|EditicsExcelBlock|EditicsPresentationBlock} EditicsBlock
 *
 * @typedef {string} EditicsBase64Ciphertext
 *
 * @typedef {number} EditicsKeyIndex
 *
 * @typedef {[EditicsKeyIndex, EditicsBase64Ciphertext]} EditicsEncryptedContent
 *
 * @typedef {Object} EditicsParticipantEntry
 * @property {EditicsParticipantIndex} indexUser
 * @property {string} deviceId
 * @property {boolean} view
 *
 * @typedef {Object} EditicsExcelRecalcIndexElement
 * @property {1|2} _recalcType
 * @property {number} _position
 * @property {number} _count
 * @property {boolean} m_bIsSaveIndex
 *
 * @typedef {Object} EditicsExcelSheetRecalcIndex
 * @property {EditicsExcelRecalcIndexElement[]} _arrElements
 *
 * @typedef {Record<string, EditicsExcelSheetRecalcIndex>} EditicsExcelRecalcIndex
 *
 * @typedef {Object} EditicsClientEventAuthReconnect
 * @property {EditicsParticipantSessionID} participantSessionId
 * @property {EditicsTimestampMs|null} lastOtherSaveTime
 *
 * @typedef {Object} EditicsClientEventAuth
 * @property {'auth'} type
 * @property {EditicsClientEventAuthReconnect|null} reconnect
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
 * @property {EditicsEncryptedContent} encryptedMessage
 *
 * @typedef {Object} EditicsClientEventCursor
 * @property {'cursor'} type
 * @property {EditicsEncryptedContent} encryptedCursor
 *
 * @typedef {Object} EditicsClientEventGetLock
 * @property {'getLock'} type
 * @property {EditicsBlock[]} block
 *
 * @typedef {Object} EditicsClientEventIsSaveLock
 * @property {'isSaveLock'} type
 * @property {number} syncChangesIndex
 *
 * @typedef {Object} EditicsClientEventSaveChanges
 * @property {'saveChanges'} type
 * @property {EditicsEncryptedContent[]} encryptedChanges
 * @property {boolean} startSaveChanges
 * @property {boolean} endSaveChanges
 * @property {number|null} deleteIndex
 * @property {EditicsExcelRecalcIndex|null} excelAdditionalInfoIndexCols
 * @property {EditicsExcelRecalcIndex|null} excelAdditionalInfoIndexRows
 * @property {EditicsEncryptedContent|null} encryptedCursorAdditionalInfo
 * @property {boolean} releaseLocks
 *
 * @typedef {Object} EditicsClientEventUnSaveLock
 * @property {'unSaveLock'} type
 *
 * @typedef {Object} EditicsClientEventUnLockDocument
 * @property {'unLockDocument'} type
 * @property {boolean} isSave
 * @property {boolean} unlock
 * @property {number|null} deleteIndex
 * @property {boolean} releaseLocks
 *
 * @typedef {Object} EditicsClientEventClose
 * @property {'close'} type
 *
 * @typedef {EditicsClientEventAuth|EditicsClientEventAuthChangesAck|EditicsClientEventMessage|EditicsClientEventGetMessages|EditicsClientEventCursor|EditicsClientEventGetLock|EditicsClientEventIsSaveLock|EditicsClientEventSaveChanges|EditicsClientEventUnSaveLock|EditicsClientEventUnLockDocument|EditicsClientEventClose} EditicsClientEvent
 *
 * @typedef {Object} EditicsServerEventAuth
 * @property {'auth'} type
 * @property {EditicsParticipantEntry[]} participants
 * @property {EditicsParticipantSessionID} participantSessionId
 * @property {EditicsParticipantIndex} participantIndexUser
 * @property {EditicsTimestampMs} participantSessionTimeConnect
 * @property {EditicsTimestampMs} openedAt
 * @property {Array<{time:EditicsTimestampMs, holder:EditicsParticipantIdentity, block:EditicsBlock}>} locks
 * @property {Array<{time:EditicsTimestampMs, author:EditicsParticipantIdentity, encryptedMessage:EditicsEncryptedContent}>} messages
 * @property {boolean} initialContentUploaded
 *
 * @typedef {Object} EditicsServerEventConnectState
 * @property {'connectState'} type
 * @property {EditicsTimestampMs} participantsTimestamp
 * @property {EditicsParticipantEntry[]} participants
 * @property {boolean} waitAuth
 *
 * @typedef {Object} EditicsServerEventAuthChanges
 * @property {'authChanges'} type
 * @property {Array<{index:number, author:EditicsParticipantIdentity, time:EditicsTimestampMs, encryptedChange:EditicsEncryptedContent}>} changes
 *
 * @typedef {Object} EditicsServerEventWaitAuth
 * @property {'waitAuth'} type
 * @property {EditicsParticipantEntry} authLockedBy
 *
 * @typedef {Object} EditicsServerEventMessage
 * @property {'message'} type
 * @property {Array<{time:EditicsTimestampMs, author:EditicsParticipantIdentity, encryptedMessage:EditicsEncryptedContent}>} messages
 *
 * @typedef {Object} EditicsServerEventCursor
 * @property {'cursor'} type
 * @property {EditicsTimestampMs} time
 * @property {EditicsParticipantIdentity} author
 * @property {EditicsEncryptedContent} encryptedCursor
 *
 * @typedef {Object} EditicsServerEventGetLock
 * @property {'getLock'} type
 * @property {Record<string, {time:EditicsTimestampMs, holder:EditicsParticipantIdentity, block:EditicsBlock}>} locks
 *
 * @typedef {Object} EditicsServerEventReleaseLock
 * @property {'releaseLock'} type
 * @property {Array<{block:EditicsBlock, holder:EditicsParticipantIdentity, time:EditicsTimestampMs, changes:null}>} locks
 *
 * @typedef {Object} EditicsServerEventSaveLock
 * @property {'saveLock'} type
 * @property {boolean} lockedBySomebodyElse
 *
 * @typedef {Object} EditicsServerEventSaveChanges
 * @property {'saveChanges'} type
 * @property {EditicsParticipantIdentity} author
 * @property {Array<{time:EditicsTimestampMs, encryptedChange:EditicsEncryptedContent}>} changes
 * @property {number} changesIndex
 * @property {number} syncChangesIndex
 * @property {boolean} endSaveChanges
 * @property {Array<{block:EditicsBlock, holder:EditicsParticipantIdentity, time:EditicsTimestampMs, changes:null}>} locks
 * @property {EditicsExcelRecalcIndex|null} excelAdditionalInfoIndexCols
 * @property {EditicsExcelRecalcIndex|null} excelAdditionalInfoIndexRows
 * @property {EditicsEncryptedContent|null} encryptedCursorAdditionalInfo
 *
 * @typedef {Object} EditicsServerEventSavePartChanges
 * @property {'savePartChanges'} type
 * @property {number} changesIndex
 * @property {number} syncChangesIndex
 *
 * @typedef {Object} EditicsServerEventUnSaveLock
 * @property {'unSaveLock'} type
 * @property {number|null} index
 * @property {EditicsTimestampMs|null} time
 * @property {number|null} syncChangesIndex
 *
 * @typedef {Object} EditicsServerEventDisconnectReason
 * @property {'disconnectReason'} type
 * @property {4004} code
 * @property {string} description
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
 * @typedef {Object} EditicsServerEventParsecInitialContentUploaded
 * @property {'parsecInitialContentUploaded'} type
 *
 * @typedef {EditicsServerEventAuth|EditicsServerEventConnectState|EditicsServerEventAuthChanges|EditicsServerEventWaitAuth|EditicsServerEventMessage|EditicsServerEventCursor|EditicsServerEventGetLock|EditicsServerEventReleaseLock|EditicsServerEventSaveLock|EditicsServerEventSaveChanges|EditicsServerEventSavePartChanges|EditicsServerEventUnSaveLock|EditicsServerEventDisconnectReason|EditicsServerEventDrop|EditicsServerEventWarning|EditicsServerEventError|EditicsServerEventParsecInitialContentUploaded} EditicsServerEvent
 */

/* eslint-enable max-len */
// /!\ End of auto-generated code /!\
