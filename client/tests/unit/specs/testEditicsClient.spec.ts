// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Tests for the editics protocol translator (`client/editics/protocol.js`).
// Verifies the OnlyOffice <-> editics protocol mapping for `auth`, `saveChanges`,
// `authChanges`, `connectState`, `isSaveLock`/`saveLock`, `getLock`, `releaseLock`,
// `unSaveLock` and the participant `id`/`idOriginal` shape, against the real
// OnlyOffice wire shapes captured in
// docs/rfcs/1030-collaborative-editics/oo_example_session.md.
//
// The translator is a pure ES module (no I/O, no globals); it is loaded by
// importing the source file directly (no build step — todo step_2 §2.3).

import { readFileSync } from 'fs';
import * as vm from 'node:vm';
import { resolve } from 'path';
import { describe, expect, it } from 'vitest';

// `protocol.js` is plain JS with an ESM `export`. happy-dom/vitest don't load
// `.js` files through the TS pipeline, so evaluate the module source in a tiny
// sandbox that captures the `export` (we rewrite `export { … }` into an
// assignment). This keeps the test path build-free (todo step_2 §2.1).
const source = readFileSync(resolve(import.meta.dirname, '../../../editics/protocol.js'), 'utf8');
const moduleNs: { exports: { EditicsTranslator: unknown } } = { exports: { EditicsTranslator: undefined } };
const wrapped = source.replace(/^export \{ EditicsTranslator \};$/m, '__editicsExports.EditicsTranslator = EditicsTranslator;');
const context = vm.createContext({ __editicsExports: moduleNs.exports });
vm.runInContext(wrapped, context, { filename: 'protocol.js' });

// Minimal structural interface for the pure translator (the source is plain JS
// with JSDoc; we type only what the test exercises).
interface EditicsTranslatorLike {
  indexUser: number;
  _participants: Map<number, { deviceId: string; userName: string; userId: string }>;
  getParticipants(): { list: unknown[]; index: number };
  cookClientEvent(oo: any): Promise<any>;
  cookServerEvent(editics: any): Promise<any>;
  _mergeParticipants(participants: any[]): Promise<void>;
}
const EditicsTranslator = moduleNs.exports.EditicsTranslator as { new (cfg: unknown): EditicsTranslatorLike };

const DEVICE_A = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa';
const DEVICE_B = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb';

function makeClient(overrides = {}) {
  const sent: any[] = [];
  const capabilities = {
    resolveUserName: async (d: string) => (d === DEVICE_A ? 'Alice' : undefined),
    resolveUserId: async (d: string) => (d === DEVICE_A ? 'F89d8069ba2b' : undefined),
    encrypt: (plain: Uint8Array) => plain,
    decrypt: (cipher: Uint8Array) => cipher,
  };
  const translator = new EditicsTranslator({
    workspaceId: '00000000000000000000000000000001',
    vlobId: '00000000000000000000000000000002',
    deviceIdHex: DEVICE_A,
    userId: 'F89d8069ba2b',
    userName: 'Alice',
    vlobVersion: 1,
    editorType: 0,
    capabilities,
    ...overrides,
  } as any);
  return { translator, sent };
}

describe('editics translator', () => {
  it('cookClientEvent strips the OO auth event to the editics shape', async () => {
    const { translator } = makeClient();
    const ooAuth = {
      type: 'auth',
      docid: 'doc-uuid',
      token: 'tok',
      user: { id: 'F89d8069ba2b', username: 'Alice', firstname: null, lastname: null, indexUser: -1 },
      editorType: 0,
      jwtOpen: 'eyJ...',
      openCmd: { c: 'open' },
    } as any;
    const cooked = await translator.cookClientEvent(ooAuth);
    expect(cooked).toEqual({ type: 'auth', indexUser: -1, editorType: 0, vlobVersion: 1 });
  });

  it('cookClientEvent returns null for ignored OO events', async () => {
    const { translator } = makeClient();
    expect(await translator.cookClientEvent({ type: 'getMessages' } as any)).toBeNull();
    expect(await translator.cookClientEvent({ type: 'openDocument', message: {} } as any)).toBeNull();
    expect(await translator.cookClientEvent({ type: 'clientLog', level: 'debug', msg: 'x' } as any)).toBeNull();
    expect(await translator.cookClientEvent({ type: 'extendSession', idletime: 0 } as any)).toBeNull();
    expect(await translator.cookClientEvent({ type: 'forceSaveStart' } as any)).toBeNull();
    expect(await translator.cookClientEvent({ type: 'rpc', responseKey: 1, data: {} } as any)).toBeNull();
  });

  it('maps a saveChanges (JSON-string changes) to encryptedChanges per fragment', async () => {
    const { translator } = makeClient();
    const fragments = ['66;AgAAA', '37;CAAA'];
    const ooSaveChanges = {
      type: 'saveChanges',
      changes: JSON.stringify(fragments),
      startSaveChanges: true,
      endSaveChanges: true,
      deleteIndex: null,
      excelAdditionalInfo: null,
      releaseLocks: false,
    } as any;
    const cooked = (await translator.cookClientEvent(ooSaveChanges)) as any;
    expect(cooked.type).toBe('saveChanges');
    expect(cooked.startSaveChanges).toBe(true);
    expect(cooked.endSaveChanges).toBe(true);
    // One entry per fragment (2 here), each the (passthrough) bytes of the fragment.
    expect(cooked.encryptedChanges.length).toBe(2);
    expect(cooked.encryptedChanges.map((b: Uint8Array) => Buffer.from(b).toString('utf8'))).toEqual(fragments);
    // `excel_info` is mandated by the editics protocol (mirrors the server
    // pydantic field name); keep the snake_case wire name on purpose.
    expect(cooked.excel_info).toBeNull();
  });

  it('maps a server saveChanges broadcast back to the OnlyOffice record shape', async () => {
    const { translator } = makeClient();
    // Seed the participant table so `user`/`useridoriginal` resolve.
    await translator._mergeParticipants([{ indexUser: 1, deviceId: DEVICE_A }]);
    translator.indexUser = 1;

    const fragment = '66;AgAAA';
    const serverEvent = {
      type: 'saveChanges',
      changes: [{ time: 1787565260000, authorIndexUser: 1, change: new TextEncoder().encode(fragment) }],
      changesIndex: 1,
      syncChangesIndex: 1,
      endSaveChanges: true,
      locks: [],
      // `excel_info` mirrors the server pydantic field name (snake_case).
      // eslint-disable-next-line camelcase
      excel_info: null,
      encryptedCursor: null,
    } as any;
    const oo = (await translator.cookServerEvent(serverEvent)) as any;
    expect(oo.type).toBe('saveChanges');
    expect(oo.endSaveChanges).toBe(true);
    expect(oo.startSaveChanges).toBe(true);
    expect(oo.changesIndex).toBe(1);
    expect(oo.syncChangesIndex).toBe(1);
    expect(oo.changes.length).toBe(1);
    const rec = oo.changes[0];
    expect(JSON.parse(rec.change)).toBe(fragment);
    expect(rec.user).toBe('F89d8069ba2b1');
    expect(rec.useridoriginal).toBe('F89d8069ba2b');
    expect(rec.docid).toBe('00000000000000000000000000000001/00000000000000000000000000000002');
  });

  it('maps an authChanges backlog to the OnlyOffice record shape', async () => {
    const { translator } = makeClient();
    await translator._mergeParticipants([{ indexUser: 1, deviceId: DEVICE_A }]);
    const serverEvent = {
      type: 'authChanges',
      changes: [
        [1, new TextEncoder().encode('66;AgAAA')],
        [2, new TextEncoder().encode('37;CAAA')],
      ] as Array<[number, Uint8Array]>,
    } as any;
    const oo = (await translator.cookServerEvent(serverEvent)) as any;
    expect(oo.type).toBe('authChanges');
    expect(oo.changes.length).toBe(2);
    expect(oo.changes.map((c: any) => JSON.parse(c.change))).toEqual(['66;AgAAA', '37;CAAA']);
    expect(oo.changes[0].user).toBe('F89d8069ba2b1');
    expect(oo.changes[0].useridoriginal).toBe('F89d8069ba2b');
  });

  it('maps isSaveLock -> isSaveLock, saveLock -> saveLock', async () => {
    const { translator } = makeClient();
    const cooked = await translator.cookClientEvent({ type: 'isSaveLock', syncChangesIndex: 0 } as any);
    expect(cooked).toEqual({ type: 'isSaveLock', syncChangesIndex: 0 });
    const oo = await translator.cookServerEvent({ type: 'saveLock', saveLock: true } as any);
    expect(oo).toEqual({ type: 'saveLock', saveLock: true });
  });

  it('translates a getLock SSE broadcast: key by plain block id, user = userId+index', async () => {
    const { translator } = makeClient();
    await translator._mergeParticipants([{ indexUser: 1, deviceId: DEVICE_A }]);
    const serverEvent = { type: 'getLock', locks: { K: { time: 1, user: 1, block: 'K' } } } as any;
    const oo = (await translator.cookServerEvent(serverEvent)) as any;
    expect(oo.type).toBe('getLock');
    expect(Object.keys(oo.locks)).toEqual(['K']);
    expect(oo.locks.K.user).toBe('F89d8069ba2b1');
    expect(oo.locks.K.block).toBe('K');
  });

  it('translates a releaseLock: user = userId+index', async () => {
    const { translator } = makeClient();
    await translator._mergeParticipants([{ indexUser: 1, deviceId: DEVICE_A }]);
    const serverEvent = { type: 'releaseLock', locks: [{ block: 'K', user: 1, time: 1, changes: null }] } as any;
    const oo = (await translator.cookServerEvent(serverEvent)) as any;
    expect(oo.type).toBe('releaseLock');
    expect(oo.locks[0].user).toBe('F89d8069ba2b1');
    expect(oo.locks[0].block).toBe('K');
  });

  it('maps unSaveLock reply to the editor', async () => {
    const { translator } = makeClient();
    const oo = await translator.cookServerEvent({ type: 'unSaveLock', index: 1, time: 1234, syncChangesIndex: 1 } as any);
    expect(oo).toEqual({ type: 'unSaveLock', index: 1, time: 1234, syncChangesIndex: 1 });
  });

  it('on auth success sets indexUser and emits a connectState with composite ids', async () => {
    const { translator } = makeClient();
    const oo = (await translator.cookServerEvent({
      type: 'auth',
      result: 1,
      participants: [{ indexUser: 1, deviceId: DEVICE_A }],
      indexUser: 1,
      sessionId: 'sess',
      sessionTimeConnect: 1,
    } as any)) as any;
    expect(translator.indexUser).toBe(1);
    expect(oo.type).toBe('connectState');
    expect(oo.waitAuth).toBe(false);
    expect(oo.participants.length).toBe(1);
    expect(oo.participants[0]).toMatchObject({ id: 'F89d8069ba2b1', idOriginal: 'F89d8069ba2b', indexUser: 1 });
  });

  it('on auth rejection emits nothing to forward', async () => {
    const { translator } = makeClient();
    const oo = await translator.cookServerEvent({
      type: 'auth',
      result: 0,
      latestAllowedVersion: 11,
    } as any);
    expect(oo).toBeNull();
  });

  it('on waitAuth emits an OO waitAuth with lockDocument rebuilt from the table', async () => {
    const { translator } = makeClient();
    await translator._mergeParticipants([{ indexUser: 1, deviceId: DEVICE_A }]);
    const oo = (await translator.cookServerEvent({ type: 'waitAuth', authLockedBy: 1 } as any)) as any;
    expect(oo.type).toBe('waitAuth');
    expect(oo.lockDocument).toMatchObject({ id: 'F89d8069ba2b1', idOriginal: 'F89d8069ba2b', indexUser: 1 });
  });

  it('produces connectState participants with composite id = userId+indexUser', async () => {
    const { translator } = makeClient();
    // Drop the provisional self-seed (index 0): the server's participant list is
    // authoritative, same as the `auth` success path which clears the table.
    translator._participants.clear();
    await translator._mergeParticipants([
      { indexUser: 1, deviceId: DEVICE_A },
      { indexUser: 2, deviceId: DEVICE_B },
    ]);
    const oo = (await translator.cookServerEvent({
      type: 'connectState',
      participantsTimestamp: 1,
      participants: [
        { indexUser: 1, deviceId: DEVICE_A, view: false },
        { indexUser: 2, deviceId: DEVICE_B, view: false },
      ],
      waitAuth: false,
    } as any)) as any;
    expect(oo.participants.length).toBe(2);
    expect(oo.participants[0]).toMatchObject({ id: 'F89d8069ba2b1', idOriginal: 'F89d8069ba2b', indexUser: 1 });
    // The second participant's userId falls back to its deviceId (the resolver
    // returns a fixed value only for the first device); still a composite id.
    expect(oo.participants[1].indexUser).toBe(2);
  });
});
