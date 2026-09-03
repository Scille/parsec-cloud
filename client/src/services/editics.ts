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
  // OnlyOffice `authChanges`. Name kept. Defined for completeness; NOT sent
  // in step 0 (fresh session -> empty backlog).
  type: 'authChanges';
  // Each entry: (change index, base64 change blob). JSON cannot carry raw
  // bytes, so the server serializes `bytes` as base64 when it eventually sends
  // `authChanges`. Placeholder to be revisited when the backlog is implemented.
  changes: Array<[number, string /* base64 bytes */]>;
}

export type ServerEvent = AuthServer | ConnectState | AuthChanges;

// --- Client -> server events ------------------------------------------------

export interface AuthClient {
  // OnlyOffice client `auth` event, trimmed. Name kept.
  type: 'auth';
  indexUser: number; // -1 on first open
  editorType: number; // 0=Word, 1=Spreadsheet, 2=Presentation, 3=Visio
  vlobVersion: number; // the vlob version the client has loaded locally
}

export type ClientEvent = AuthClient;

// --- Rejection response (RPC reply) -----------------------------------------

// On rejection the RPC returns this instead of `AuthServer` (todo §7.1).
export interface AuthRejected {
  type: 'auth';
  result: number; // 0 = rejected (OnlyOffice: non-1 = failure)
  // RFC §1.2: the version the client should reload to before retrying.
  latestAllowedVersion: number;
}
