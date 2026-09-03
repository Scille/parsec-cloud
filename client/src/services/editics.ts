// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Editics client/server protocol types (step 0: auth subset).
//
// These mirror the Pydantic server-side types in `server/parsec/components/editics/`
// (see `docs/rfcs/1030-collaborative-editics.md` and `todo/step_0.md` §4.2). The
// client defines the same shapes for server events it expects, but does NOT
// strictly validate them: incoming SSE `data:` JSON is parsed and passed through
// to OnlyOffice internals. A discriminated union on `type` is provided for
// readability and to document intent, but runtime validation is best-effort (a
// malicious server can still push a structurally-invalid event and the client
// will accept it — by design, see `todo/step_0.md` §2). OnlyOffice event/field
// names are kept verbatim and documented here, they are *not* renamed even when
// they are known-bad (this keeps the client-side translation layer thin).

// `indexUser`: 1-based, monotonic per session, assigned by order of join.
// OnlyOffice client code does arithmetic with this index (RFC §1.4), so it
// must stay a plain number.
export type IndexUser = number;

export interface ParticipantEntry {
  // OnlyOffice name `indexUser` kept (bad name documented, not renamed).
  indexUser: IndexUser;
  // Editics addition (replaces OnlyOffice's idOriginal/username/etc.).
  // The server is NOT trusted for user names: the client resolves
  // deviceId -> user_name through libparsec and keeps its own
  // [indexUser, deviceId, user_name] table (RFC §3.3 / todo §2).
  deviceId: string; // DeviceID hex
  // OnlyOffice name `view` kept. Step 1 addition (forward-compat, todo
  // step_1 §4.1 / §2.5): whether the participant is a viewer (read-only).
  // Always false in step 1 (all participants are treated as editors; real
  // realm-role-based access control is deferred).
  view: boolean;
}

// --- Server -> client events ------------------------------------------------

export interface AuthServer {
  // OnlyOffice server `auth` reply, trimmed. Name kept.
  type: 'auth';
  result: number; // 1 = success (OnlyOffice convention)
  participants: ParticipantEntry[]; // current participant map
  indexUser: IndexUser; // this connection's assigned index
  // Reconnect info (forward-compat; unused in step 0 but kept).
  sessionId: string;
  sessionTimeConnect: number; // server timestamp (ms) at connect
  // NOTE: the change backlog is NOT here. It is delivered as a separate
  // `authChanges` SSE event (RFC §2.2). For a fresh session the backlog is
  // empty -> no `authChanges` is sent in step 0.
}

export interface ConnectState {
  // OnlyOffice `connectState`, trimmed. Name kept.
  type: 'connectState';
  // Monotonic ms timestamp of this participant-set update (OnlyOffice name).
  participantsTimestamp: number;
  participants: ParticipantEntry[];
  waitAuth: boolean; // always false in step 0 (no auth lock)
}

export interface AuthChanges {
  // OnlyOffice `authChanges`. Name kept. Delivered to a joining client as
  // the backlog of changes since the session was created (RFC §1.2 step 3.4).
  type: 'authChanges';
  // Each entry: (change index, base64 change blob). JSON cannot carry raw
  // bytes, so the server serializes `bytes` as base64.
  changes: Array<[number, string /* base64 bytes */]>;
}

// OnlyOffice `waitAuth` (s->c). Name kept. Per RFC §2.2 editics changes,
// `lockDocument` is replaced by `authLockedBy`. Sent to a joining non-view
// participant when the auth lock is held (todo step_1 §6.2), as the RPC reply.
export interface WaitAuth {
  type: 'waitAuth';
  authLockedBy: IndexUser;
}

// --- Server -> client events: chat / cursor / locks / save ----------------

// One record in a `message` event's `messages` array.
export interface MessageRecord {
  time: number; // server timestamp (ms)
  authorIndexUser: IndexUser;
  encryptedMessage: string; // base64 (§2.4); opaque to the server.
}

// OnlyOffice `message` (s->c). Name kept. Broadcast to all participants
// (including the sender, §6.4).
export interface MessageServer {
  type: 'message';
  // OnlyOffice wraps the payload in `messages: [...]`; we keep the array shape
  // (bad name documented) for translation-layer symmetry.
  messages: MessageRecord[];
}

// One record in a `cursor` event's `messages` array.
export interface CursorRecord {
  time: number;
  authorIndexUser: IndexUser;
  encryptedCursor: string; // base64 (§2.4); opaque to the server.
}

// OnlyOffice `cursor` (s->c). Name kept. Broadcast to other participants.
export interface CursorServer {
  type: 'cursor';
  messages: CursorRecord[];
}

// OnlyOffice `getLock` (s->c). Name kept. Broadcast to all participants.
// `locks` is keyed by the block key; each record is { time, user, block }.
export interface GetLockServer {
  type: 'getLock';
  locks: Record<string, { time: number; user: IndexUser; block: unknown }>;
}

// One record in a `releaseLock` event / a `saveChanges` `locks` field.
export interface ReleaseLockRecord {
  block: unknown; // opaque block descriptor (re-broadcast as-is)
  user: IndexUser; // holder who released (OnlyOffice bad name kept)
  time: number;
  changes: null; // always null here (OnlyOffice shape)
}

// OnlyOffice `releaseLock` (s->c). Name kept. Broadcast to others.
export interface ReleaseLockServer {
  type: 'releaseLock';
  locks: ReleaseLockRecord[];
}

// OnlyOffice `saveLock` (s->c). Name kept. Reply to `isSaveLock` (c->s).
export interface SaveLockServer {
  type: 'saveLock';
  saveLock: boolean; // true = denied, false = granted
}

// One record in a `saveChanges` broadcast's `changes` array.
export interface SaveChangeRecord {
  time: number;
  authorIndexUser: IndexUser;
  change: string; // base64 (§2.4); opaque to the server.
}

// OnlyOffice `saveChanges` (s->c, broadcast to *other* participants). Name kept.
export interface SaveChangesServer {
  type: 'saveChanges';
  changes: SaveChangeRecord[];
  changesIndex: number; // new save point after this save (§2.2)
  syncChangesIndex: number; // always-advancing total (§2.2)
  endSaveChanges: boolean; // mirrors the originator's flag
  locks: ReleaseLockRecord[]; // locks released by the originator
  excel_info: Record<string, unknown> | null;
  encryptedCursor: string | null; // base64 (§2.4)
}

// OnlyOffice `savePartChanges` (s->c, reply to the saver for intermediate
// chunks). Name kept.
export interface SavePartChangesServer {
  type: 'savePartChanges';
  changesIndex: number; // -1 except first non-truncating chunk
  syncChangesIndex: number; // always-advancing total
}

// OnlyOffice `unSaveLock` (s->c). Name kept. Reply to `unSaveLock` (c->s,
// cancellation) or a final `saveChanges` chunk (success).
export interface UnSaveLockServer {
  type: 'unSaveLock';
  index: number; // save point, or -1 on cancel
  time: number; // last change time, or -1 on cancel
  syncChangesIndex: number; // new total, or -1 on cancel
}

// OnlyOffice `drop` (s->c). Name kept. Force-remove a participant.
export interface DropServer {
  type: 'drop';
  code: number; // OnlyOffice DROP_CODE constant (4007)
  description: string;
}

// OnlyOffice `warning` (s->c). Name kept. Shape only in step 1.
export interface WarningServer {
  type: 'warning';
  code: number;
  message: string;
}

export type ServerEvent =
  | AuthServer
  | ConnectState
  | AuthChanges
  | WaitAuth
  | MessageServer
  | CursorServer
  | GetLockServer
  | ReleaseLockServer
  | SaveLockServer
  | SaveChangesServer
  | SavePartChangesServer
  | UnSaveLockServer
  | DropServer
  | WarningServer;

// --- Client -> server events ------------------------------------------------

export interface AuthClient {
  // OnlyOffice client `auth` event, trimmed. Name kept.
  type: 'auth';
  indexUser: number; // -1 on first open
  editorType: number; // 0=Word, 1=Spreadsheet, 2=Presentation, 3=Visio
  vlobVersion: number; // the vlob version the client has loaded locally
}

// OnlyOffice `authChangesAck` (c->s). Name kept.
export interface AuthChangesAckClient {
  type: 'authChangesAck';
}

// OnlyOffice `message` (c->s). Name kept. `encryptedMessage` is base64.
export interface MessageClient {
  type: 'message';
  encryptedMessage: string; // base64 (§2.4)
}

// OnlyOffice `cursor` (c->s). Name kept. `encryptedCursor` is base64.
export interface CursorClient {
  type: 'cursor';
  encryptedCursor: string; // base64 (§2.4)
}

// OnlyOffice `getLock` (c->s). Name kept.
export interface GetLockClient {
  type: 'getLock';
  block: unknown[]; // opaque block descriptors
}

// OnlyOffice `isSaveLock` (c->s). Name kept.
export interface IsSaveLockClient {
  type: 'isSaveLock';
  syncChangesIndex: number;
}

// OnlyOffice `saveChanges` (c->s). Name kept. See RFC §2.2 editics changes.
export interface SaveChangesClient {
  type: 'saveChanges';
  encryptedChanges: string[]; // each: base64 (§2.4); one per fragment
  startSaveChanges: boolean;
  endSaveChanges: boolean;
  deleteIndex: number | null;
  excel_info: Record<string, unknown> | null;
  encryptedCursor: string | null; // base64 (§2.4)
  releaseLocks: boolean;
}

// OnlyOffice `unSaveLock` (c->s). Name kept.
export interface UnSaveLockClient {
  type: 'unSaveLock';
}

// OnlyOffice `unLockDocument` (c->s). Name kept.
export interface UnLockDocumentClient {
  type: 'unLockDocument';
  isSave: boolean;
  unlock: boolean;
  deleteIndex: number | null;
  releaseLocks: boolean;
}

// OnlyOffice `close` (c->s). Name kept.
export interface CloseClient {
  type: 'close';
}

// Editics addition (no OnlyOffice equivalent). Bumps the session's
// `latest_allowed_version` after a vlob upload (RFC §1.2 step 4.3).
export interface SaveDoneClient {
  type: 'saveDone';
  savedUpToIndex: number;
  newVersion: number;
}

export type ClientEvent =
  | AuthClient
  | AuthChangesAckClient
  | MessageClient
  | CursorClient
  | GetLockClient
  | IsSaveLockClient
  | SaveChangesClient
  | UnSaveLockClient
  | UnLockDocumentClient
  | CloseClient
  | SaveDoneClient;

// --- Rejection response (RPC reply) -----------------------------------------

// On rejection the RPC returns this instead of `AuthServer` (todo §7.1).
export interface AuthRejected {
  type: 'auth';
  result: number; // 0 = rejected (OnlyOffice: non-1 = failure)
  // RFC §1.2: the version the client should reload to before retrying.
  latestAllowedVersion: number;
}
