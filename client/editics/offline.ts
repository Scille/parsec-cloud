// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Offline edition mode host page (see docs/rfcs/1030-collaborative-editics.md).
//
// The whole edition is done locally: the document is loaded from the bytes
// provided by the parent window (see `common.ts` for the parts shared with
// the collaborative mode), and the OnlyOffice editor is driven by the
// `OfflineMockServer` below, which answers its co-authoring protocol by
// itself (single user, no server involved).
//
// Document saving is delegated to the parent window: it is the only side
// with access to libparsec, so it is the one writing the document back into
// the Parsec workspace (see the `oo-save` message in `parent_host_api.ts`).

import type OO from 'onlyoffice-editor';
import { openEditor, startHostPage } from './common';
import type { EditicsOpenOptions } from './parent_host_api';

// --- OfflineMockServer -----------------------------------------------------

// The OnlyOffice editor still runs its co-authoring state machine over the
// wrapper's message bridge, so it expects a "server" to answer the `auth`
// handshake, the save-lock requests and the element (block) locks.
class OfflineMockServer implements OO.MockServer {
  editor: OO.DocEditor;
  userId: string;
  username: string;
  // Server-assigned participant index (see `answerAuth` event). The OnlyOffice
  // protocol identifies users by the composite `<idOriginal><indexUser>` ID.
  indexUser: number;
  compositeUserId: string;
  // Monotonic change counter (the OnlyOffice "puckerIndex"): the total number
  // of changes the server has stored. Used to answer the saveChanges flow
  // (`savePartChanges`/`unSaveLock`).
  syncChangesIndex: number;
  // Absolute change index at which the in-progress save starts (the
  // "save point" reported back in the final `unSaveLock`).
  saveChangesIndex: number;

  constructor(editor: OO.DocEditor, options?: { user?: { id: string; username: string } }) {
    this.editor = editor;
    const u = options?.user ?? { id: 'uid-local', username: 'Local user' };
    this.userId = u.id;
    this.username = u.username;
    this.indexUser = 1;
    this.compositeUserId = this.userId + this.indexUser;
    this.syncChangesIndex = 0;
    this.saveChangesIndex = 0;
  }

  onMessage(msg: OO.OOClientEvent): void {
    switch (msg.type) {
      case 'auth':
        const participant = {
          id: this.compositeUserId,
          idOriginal: this.userId,
          username: this.username,
          indexUser: this.indexUser,
          connectionId: 'conn-local',
          isCloseCoAuthoring: false,
          view: false,
        };

        this.editor.sendMessageToOO({ type: 'authChanges', changes: [] } satisfies OO.OOServerEventAuthChanges);

        this.editor.sendMessageToOO({
          type: 'auth',
          result: 1,
          sessionId: 'local-session',
          participants: [participant],
          locks: [],
          changesIndex: 0,
          indexUser: this.indexUser,
          buildVersion: '5.2.6',
          buildNumber: 2,
          licenseType: 3,
          settings: {
            binaryChanges: true,
          },
        } satisfies OO.OOServerEventAuth);
        break;

      case 'isSaveLock':
        // Grant the save lock immediately so `asc_Save`'s `askSaveChanges`
        // resolves without the ~10 s timeout.
        this.editor.sendMessageToOO({ type: 'saveLock', saveLock: false } satisfies OO.OOServerEventSaveLock);
        break;

      case 'unSaveLock':
        // Cancellation of an in-progress save: release the save lock and
        // acknowledge with -1 indices (RFC 1030 §2.2).
        this.editor.sendMessageToOO({ type: 'unSaveLock', index: -1, time: -1, syncChangesIndex: -1 } satisfies OO.OOServerEventUnSaveLock);
        break;

      case 'saveChanges':
        // Only need to drive the client's save state machine: chunk acks + final unSaveLock

        if (msg.startSaveChanges) {
          // The save point of this save is the change index it starts at
          // (the previous total).
          this.saveChangesIndex = this.syncChangesIndex;
        }
        // Since we have enabled `binaryChanges` (see `auth` event), the changes
        // are provided as an array instead of being JSON-serialized.
        this.syncChangesIndex += (msg.changes as Array<Uint8Array>).length;
        if (msg.endSaveChanges) {
          this.editor.sendMessageToOO({
            type: 'unSaveLock',
            index: this.saveChangesIndex,
            time: Date.now(),
            syncChangesIndex: this.syncChangesIndex,
          } satisfies OO.OOServerEventUnSaveLock);
        } else {
          // Acknowledge the chunk so the client emits the next one (the -1
          // index leaves the client's save point unchanged).
          this.editor.sendMessageToOO({
            type: 'savePartChanges',
            changesIndex: msg.startSaveChanges ? this.saveChangesIndex : -1,
            syncChangesIndex: this.syncChangesIndex,
          } satisfies OO.OOServerEventSavePartChanges);
        }
        break;

      case 'getLock':
        // Element locks (cell ranges, slide objects, ...) are requested by the
        // editors even in single-user mode (the edit is applied optimistically
        // and the reply updates the lock table).

        const blocks = (typeof msg.block === 'string' ? [msg.block] : msg.block) || [];
        const locks: { [key: string]: { user: string; time: number; block: unknown } } = {};
        for (let i = 0; i < blocks.length; i++) {
          const b = blocks[i];
          const key = typeof b === 'object' && b !== null ? b.guid : b;
          if (key !== undefined) {
            locks[key] = {
              user: this.compositeUserId,
              time: typeof b === 'object' && b !== null && b.time !== undefined ? b.time : Date.now(),
              block: b,
            };
          }
        }
        this.editor.sendMessageToOO({ type: 'getLock', locks: locks } satisfies OO.OOServerEventGetLock);
        break;

      case 'unLockDocument':
        // Fire-and-forget cleanup (there is no other participant to notify
        // and no auth lock to manage in single-user offline mode); only an
        // in-progress save needs an answer (RFC 1030 §2.2).
        if (msg.isSave) {
          this.editor.sendMessageToOO({
            type: 'unSaveLock',
            index: -1,
            time: -1,
            syncChangesIndex: -1,
          } satisfies OO.OOServerEventUnSaveLock);
        }
        break;

      case 'openDocument':
        console.log('openDocument', msg);
        // cspell: ignore imgurls
        if (msg.message.c === 'imgurls') {
          // e.g.:
          // {
          //   "type": "openDocument",
          //   "message": {
          //     "id": "778b08ca879aa0da2fed",
          //     "c": "imgurls",
          //     "userid": "a11cec00100000000000000000000000",
          //     "saveindex": 18,
          //     "data": [
          //       "http://foo.com/bar/spam.png"
          //     ]
          //   }
          // }

          const urls = msg.message.data as string[];
          if (urls.length !== 1) {
            console.error('Expected a single URL in imgurls');
          }
          const url = urls[0];
          const fileName = url.split('/').pop();
          fetch(url).then(
            async (success) => {
              console.log('openDocument fetched ok', success);

              if (!success.ok) {
                this.editor.sendMessageToOO({
                  type: 'documentOpen',
                  status: 'ok', // Also "ok" when the original URL couldn't be fetched...
                  data: {
                    // @ts-expect-error
                    error: 1,
                    // TODO: how to pass the error message to the GUI ?
                  },
                } satisfies OO.OOServerEventDocumentOpen);
                return;
              }

              const content = await success.blob();
              this.editor.sendMessageToOO({
                type: 'documentOpen',
                status: 'ok', // Also "ok" when the original URL couldn't be fetched...
                data: {
                  // @ts-expect-error
                  error: 0, // `0`: no error
                  urls: [
                    {
                      url: URL.createObjectURL(content),
                      path: `media/${fileName}`,
                    },
                  ],
                },
              } satisfies OO.OOServerEventDocumentOpen);
            },

            (error) => {
              console.log('openDocument fetched error', error);
              this.editor.sendMessageToOO({
                type: 'documentOpen',
                status: 'ok', // Also "ok" when the original URL couldn't be fetched...
                data: {
                  // @ts-expect-error
                  error: 1,
                  // TODO: how to pass the error message to the GUI ?
                },
              } satisfies OO.OOServerEventDocumentOpen);
            },
          );
        }
        break;

      default:
        // Other message types (cursor, meta, authChangesAck, ...) are not
        // relevant in single-user offline mode: ignore them.
        console.debug('ignored message', msg);
        break;
    }
  }
}

// --- open ---------------------------------------------------------------

// `openDocument` is expected to be called only once since it loads OnlyOffice
// scripts and set global variables (e.g. `window.APP`, `window.ooDocEditor`).
async function openDocument(options: EditicsOpenOptions, documentContent: Uint8Array): Promise<void> {
  await openEditor(options, documentContent, (editor) => {
    if (options.collaborative !== undefined) {
      // Defense in depth: the parent is expected to serve the collaborative
      // host page when a collaborative session is provided (see
      // `src/services/editics.ts`).
      throw new Error('Collaborative session info provided to the offline host page');
    }
    return new OfflineMockServer(editor, {
      user: { id: options.userId, username: options.userName },
    });
  });
}

startHostPage({ onOpen: openDocument });
