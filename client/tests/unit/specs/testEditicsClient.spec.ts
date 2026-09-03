// Tests for the editics client translation layer (onlyoffice-editics-client.js).
// Verifies the OnlyOffice <-> editics protocol mapping for `saveChanges`,
// `authChanges`, `connectState`, `isSaveLock`/`saveLock` and the participant
// `id`/`idOriginal` shape, against the real OnlyOffice wire shapes captured in
// docs/rfcs/1030-collaborative-editics/oo_example_session.md.

import { describe, it, expect, beforeEach, vi } from 'vitest';

// The editics client is a browser IIFE that attaches to `window`. Load it by
// evaluating the source in the test's global scope (happy-dom provides
// `window`, `btoa`, `atob`, `EventSource`, `fetch`).
import { readFileSync } from 'fs';
import { resolve } from 'path';

const source = readFileSync(
  resolve(import.meta.dirname, '../../../public/onlyoffice-editics-client.js'),
  'utf8',
);

// Evaluate the IIFE in the current global scope so `window` is the test window.
// eslint-disable-next-line no-eval
eval(source);

function makeClient(overrides = {}) {
  const sentToOO: any[] = [];
  const posted: any[] = [];
  const fakeDocEditor = { sendMessageToOO: (m: any) => sentToOO.push(m) };
  const client = (window as any).OnlyOfficeEditicsClient.create({
    baseUrl: 'http://parsec.invalid',
    organizationId: 'TestbedOrg1',
    workspaceId: '00000000000000000000000000000001',
    vlobId: '00000000000000000000000000000002',
    deviceIdHex: 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
    vlobVersion: 1,
    editorType: 0,
    userId: 'F89d8069ba2b',
    userName: 'Alice',
    mode: 'edit',
    resolveUserName: async () => 'Alice',
    resolveUserId: async () => 'F89d8069ba2b',
    ...overrides,
  });
  client.setEditor(fakeDocEditor);
  // Capture outgoing RPCs instead of hitting the network.
  (client as any)._post = async (body: any) => {
    posted.push(body);
  };
  return { client, sentToOO, posted };
}

describe('editics client translation', () => {
  it('maps a saveChanges (JSON-string changes) to encryptedChanges per fragment', async () => {
    const { client, posted } = makeClient();
    // The editor sends `changes` as a JSON-encoded string (default JSON mode),
    // an array of opaque op fragments (see oo_example_session.md).
    const fragments = ['66;AgAAA', '37;CAAA'];
    const ooSaveChanges = {
      type: 'saveChanges',
      changes: JSON.stringify(fragments),
      startSaveChanges: true,
      endSaveChanges: true,
      deleteIndex: null,
      excelAdditionalInfo: null,
      releaseLocks: false,
    };
    (client as any).onMessage(ooSaveChanges);
    await Promise.resolve();
    expect(posted.length).toBe(1);
    const body = posted[0];
    expect(body.type).toBe('saveChanges');
    expect(body.startSaveChanges).toBe(true);
    expect(body.endSaveChanges).toBe(true);
    // One base64 entry per fragment (2 here).
    expect(body.encryptedChanges.length).toBe(2);
    // Each decodes back to the original fragment string.
    expect(body.encryptedChanges.map((b: string) => atob(b))).toEqual(fragments);
  });

  it('maps a server saveChanges broadcast back to the OnlyOffice record shape', async () => {
    const { client, sentToOO } = makeClient();
    // Seed the participant table (indexUser 1 = this user) so `user`/`useridoriginal`
    // resolve to the person's userId (matches the editor's `_userId = userId+indexUser`).
    await (client as any)._mergeParticipants([
      { indexUser: 1, deviceId: 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa' },
    ]);
    (client as any).indexUser = 1;

    const fragment = '66;AgAAA';
    const serverEvent = {
      type: 'saveChanges',
      changes: [
        {
          time: 1787565260000,
          authorIndexUser: 1,
          change: btoa(fragment), // base64 of the fragment
        },
      ],
      changesIndex: 1,
      syncChangesIndex: 1,
      endSaveChanges: true,
      locks: [],
      excel_info: null,
      encryptedCursor: null,
    };
    await (client as any)._onSseData(JSON.stringify(serverEvent));
    expect(sentToOO.length).toBe(1);
    const msg = sentToOO[0];
    expect(msg.type).toBe('saveChanges');
    expect(msg.endSaveChanges).toBe(true);
    expect(msg.startSaveChanges).toBe(true);
    expect(msg.changesIndex).toBe(1);
    expect(msg.syncChangesIndex).toBe(1);
    expect(msg.changes.length).toBe(1);
    const rec = msg.changes[0];
    // `change` must be a JSON string of the fragment so the editor's
    // `JSON.parse(change["change"])` recovers the fragment.
    expect(JSON.parse(rec.change)).toBe(fragment);
    // `user` = `<userId><indexUser>` and `useridoriginal` = userId.
    expect(rec.user).toBe('F89d8069ba2b1');
    expect(rec.useridoriginal).toBe('F89d8069ba2b');
    expect(rec.docid).toBe('00000000000000000000000000000001/00000000000000000000000000000002');
  });

  it('maps an authChanges backlog to the OnlyOffice record shape', async () => {
    const { client, sentToOO } = makeClient();
    await (client as any)._mergeParticipants([
      { indexUser: 1, deviceId: 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa' },
    ]);
    const serverEvent = {
      type: 'authChanges',
      changes: [[1, btoa('66;AgAAA')], [2, btoa('37;CAAA')]] as Array<[number, string]>,
    };
    await (client as any)._onSseData(JSON.stringify(serverEvent));
    expect(sentToOO.length).toBe(1);
    const msg = sentToOO[0];
    expect(msg.type).toBe('authChanges');
    expect(msg.changes.length).toBe(2);
    expect(msg.changes.map((c: any) => JSON.parse(c.change))).toEqual(['66;AgAAA', '37;CAAA']);
    expect(msg.changes[0].user).toBe('F89d8069ba2b1');
    expect(msg.changes[0].useridoriginal).toBe('F89d8069ba2b');
  });

  it('maps isSaveLock/saveLock round-trip', async () => {
    const { client, sentToOO } = makeClient();
    (client as any).onMessage({ type: 'isSaveLock', syncChangesIndex: 0 });
    // Apply the server's saveLock reply (denied).
    (client as any)._applyReply('isSaveLock', { type: 'saveLock', saveLock: true });
    expect(sentToOO.length).toBe(1);
    expect(sentToOO[0]).toEqual({ type: 'saveLock', saveLock: true });
  });

  it('translates a getLock reply: key by plain block id, user = userId+index', async () => {
    const { client, sentToOO } = makeClient();
    await (client as any)._mergeParticipants([
      { indexUser: 1, deviceId: 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa' },
    ]);
    const serverEvent = {
      type: 'getLock',
      // The server keys word blocks by the plain string and uses int indexUser.
      locks: { K: { time: 1, user: 1, block: 'K' } },
    };
    await (client as any)._onSseData(JSON.stringify(serverEvent));
    expect(sentToOO.length).toBe(1);
    const msg = sentToOO[0];
    expect(msg.type).toBe('getLock');
    // Key preserved (plain string "K").
    expect(Object.keys(msg.locks)).toEqual(['K']);
    // `user` translated to `<userId><indexUser>`.
    expect(msg.locks.K.user).toBe('F89d8069ba2b1');
    expect(msg.locks.K.block).toBe('K');
  });

  it('translates a releaseLock: user = userId+index', async () => {
    const { client, sentToOO } = makeClient();
    await (client as any)._mergeParticipants([
      { indexUser: 1, deviceId: 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa' },
    ]);
    const serverEvent = {
      type: 'releaseLock',
      locks: [{ block: 'K', user: 1, time: 1, changes: null }],
    };
    await (client as any)._onSseData(JSON.stringify(serverEvent));
    expect(sentToOO.length).toBe(1);
    expect(sentToOO[0].type).toBe('releaseLock');
    expect(sentToOO[0].locks[0].user).toBe('F89d8069ba2b1');
    expect(sentToOO[0].locks[0].block).toBe('K');
  });

  it('maps unSaveLock reply to the editor', async () => {
    const { client, sentToOO } = makeClient();
    (client as any)._applyReply('saveChanges', {
      type: 'unSaveLock',
      index: 1,
      time: 1234,
      syncChangesIndex: 1,
    });
    expect(sentToOO[0]).toEqual({ type: 'unSaveLock', index: 1, time: 1234, syncChangesIndex: 1 });
  });

  it('produces connectState participants with composite id = userId+indexUser', async () => {
    const { client, sentToOO } = makeClient();
    // Drop the provisional self-seed (the real `_sendAuth` does this before
    // merging the server's authoritative participant list).
    (client as any)._participants.clear();
    await (client as any)._mergeParticipants([
      { indexUser: 1, deviceId: 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa' },
      { indexUser: 2, deviceId: 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb' },
    ]);
    const serverEvent = {
      type: 'connectState',
      participantsTimestamp: 1,
      participants: [
        { indexUser: 1, deviceId: 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa', view: false },
        { indexUser: 2, deviceId: 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb', view: false },
      ],
      waitAuth: false,
    };
    await (client as any)._onSseData(JSON.stringify(serverEvent));
    expect(sentToOO.length).toBe(1);
    const list = sentToOO[0].participants;
    expect(list[0]).toMatchObject({ id: 'F89d8069ba2b1', idOriginal: 'F89d8069ba2b', indexUser: 1 });
    // The second participant's userId falls back to its deviceId (the resolver
    // returns a fixed value only for the first device); still a composite id.
    expect(list[1].indexUser).toBe(2);
  });
});
