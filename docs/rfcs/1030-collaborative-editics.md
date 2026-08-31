<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Collaborative editics

## 1 - Overview

This RFC introduce a mechanism to be able to edit documents (i.e. docx/xlsx/pptx/etc. )
from within the Parsec GUI.

This edition system (called "editics") works in two modes:

- Offline mode: the edition is done purely in local.
- Online mode: the Parsec server hosts an editics collaborative session on which
  all clients looking to view/edit the document connect to.

Online mode should be considered the default one (i.e. the client tries to use the
online mode and fall back in offline if it is not possible).

### 1.1 - Use of OnlyOffice

OnlyOffice is used to implement the actual document edition, however its integration
into Parsec is special since we are in an end-to-end encrypted system:

- Document conversion (done with the x2t tool) must run client side
- OnlyOffice server cannot be used (since document modifications flow trough it in clear text)

For this reason we use the OnlyOffice patched by Cryptpad to support end-to-end encryption:

- [onlyoffice-editor](https://github.com/cryptpad/onlyoffice-editor) has been patched to use a fake server class
  (that can be used to encrypt content before sending to an arbitrary server) instead of directly sending request
  to the OnlyOffice server.
- [x2t](https://github.com/cryptpad/onlyoffice-x2t-wasm) has been patched to compile in WASM in order to run
  client-side in a web browser.

> [!NOTE]
> `x2t` is a tool to convert the various document formats (docx/xlsx/pptx/etc.) into
> the internal format used by OnlyOffice (known as "bin format", in practice it
> basically corresponds to an unzipped Office Open XML with a specific magic header
> such as "DOCY" for a docx document).

Finally we need to re-implement in the Parsec server the communication system used to
exchange events between the clients connected to a same editics collaborative session
(e.g. to modify the document, update client's cursor location, etc.).

### 1.2 - Document load/modify/save lifecycle

Let's consider a workspace containing a document `/foo.docx` with vlob ID 42, Alice (OWNER),
Bob (CONTRIBUTOR) and Mallory (READER) have access to the workspace and want to access the document.

1. Alice wants to edit document `/foo.docx`.
    1.1. Alice's client resolves `/foo.docx` path and obtains vlob ID 42 with latest version 10.
    1.2. Alice's client uses `x2t` to convert the content of vlob ID 42 at version 10 in the
         bin format.
    1.3. Alice's client loads in the editics editor the content vlob ID 42 at version 10 converted
         in bin format.
    1.4. Alice's client connects to the editics session with ID 42 and specify she uses version 10.
         The session doesn't exist on the server, it is created.
2. Alice modifies the document twice. Her client sends two modification events to the editics session.
   Each modification is given an index ID by the server to ensure they are ordered (so index 1 and 2).
3. Bob wants to join the session.
    3.1. Same as step 1.1. but for Bob.
    3.2. Same as step 1.2. but for Bob.
    3.3. Same as step 1.3. but for Bob.
    3.4. Bob's client connects to the editics session with ID 42 and specify he uses version 10.
        The server accept the connection and pushes to the client all the modifications
        that occurred since the session was created.
4. Alice wants to save the document
    4.1. Alice's client takes a save lock on the session (ensure no other clients tries a concurrent save).
    4.2. Alice's client exports the document from the editics editor and save it as vlob ID 42 version 11.
    4.3. Alice's client release the save lock on the session and indicates that all
         modifications up to index 2 are contained in vlob ID 42 version 11.
5. Mallory wants to join the session
    5.1. Same as step 1.1. but for Mallory (and latest version for vlob ID 42 is now version 11).
    5.2. Same as step 1.2. but for Mallory (and loaded version for vlob ID 42 is version 11).
    5.3. Mallory's client loads in the editics editor the content vlob ID 42 at version 11 converted in bin format.
    5.4. Mallory's client connects to the editics session with ID 42 and specify she uses version 11.
         The server accept the connection and pushes no modification.

The server ensures the client can reach the correct document state by only allowing certain vlob versions:

- Version must be at least the initial version used when the session has been created.
- A version more recent that what the session may indicates two situations:

  - The document is currently being saved and the vlob has been modified but the
    save lock not yet released (this is an unlikely situation).
  - The document has been currently modified outside of the session, hence there
    is no guarantees on what the document contains at this version.

When rejecting the client, the server provides the latest allowed version so that the client can retry.

The fact each client has to load the document by itself (i.e. the server doesn't
provide to the client the document edited in the session but only patches to apply)
is more performance-hungry on the client (as `x2t` document conversion is done client-side),
but this prevents the client that created the session from controlling the initial
content of the document (this is an issue since a user with no write access in the
workspace can then modify a document by creating a session with his tempered document,
then wait for a user with write access to join and do the save for them...).

### 1.3 - Session ID vs document ID

Each OnlyOffice session is identified by an ID that is used by the clients to join the session.
We use the couple (workspace ID + document vlob ID) as the session ID.
This means:

- A document cannot change its vlob ID (i.e. the document is saved by creating new versions of this vlob)
- A document must exist in the workspace before editing it in OnlyOffice.
- Document path in the workspace is only used to resolve the vlob ID, which is done *before* OnlyOffice is involved.
  So OnlyOffice has no concept of document path and the document name is only given as informative (it may become
  inaccurate in the session if the document is moved/renamed in the workspace).
- A document can only have at most one session at a given time. This is important
  as it prevent most concurrency modifications.
- Concurrency modifications are still possible if the document is directly modified
  in the workspace (e.g. document modified
  from the workspace mountpoint).
- Since the editics session is considered the main way of editing a document and
  doesn't cause save conflicts by itself (i.e. the session act as a single source of truth),
  it is allowed to save to the workspace by just uploading a new vlob version without
  checking if its modifications conflicts with the last existing version[^1].

[^1]: While this means the conflict is silently resolved, the end user can still
access the overwritten version through the history if needed.

### 1.4 - Session participant identified by an index

Each participant in the OnlyOffice session (i.e. each client connected to the session,
in theory a device could connect multiple time to the session and hence be considered
as multiple participants) is identified by an index corresponding to the order they
have joined the session.

> [!NOTE]
> This a strong requirement from the OnlyOffice [client code that does arithmetic](
> https://github.com/cryptpad/onlyoffice-editor/blob/c1be39bb0042d82c0f52d420e2d668f866458611/sdkjs/common/docscoapi.js#L1553)
> with this index so we cannot just use an UUID here.

TODO: talk about user ID used in OnlyOffice server to decide to broadcast changes

### 1.5 - Multiple server instances vs sessions

OnlyOffice edition session lives in a single server instance and every session
participant hence must connect to this server instance.

This is in stark contrast with the [Twelve-Factor App](https://12factor.net/)
philosophy we follow in Parsec (i.e. server instances are disposable and are all
equivalent to each others).

However hosting each instance on a single server instance brings significant benefits:

- Better performances since the event can be processed and broadcasted without going
  trough the PostgreSQL server.
- Simple code since most of the state is store in memory.

So to keep those advantages while staying in a Twelve-Factor App philosophy we:

- Store the state of the session on the database (typically this can be done in
  a background job in order not to impact reactivity). This ensures the session
  survives the destruction of the server instance.
- The client has to send an initial query to the server to get the URL of the
  server to connect to access session:
  - Single server instance: it is the URL of the server itself.
  - Multiple server instance: each server instance has its own domain (e.g.
    `1.editics.parsec.cloud`), and the main server runs a sharding algorithm to
    dispatch the edition session according to the document ID.

### 1.5 - View vs editor participants

TODO

### 1.5 - End-to-end encryption

Not all event sent to the server has to be end-to-end encrypted. As a matter,
all events should have their name in clear text (so that the server can do access
control, know if they are ephemeral etc.) and only a small subset of the events'
content should be encrypted.

For instance: the `changes` field in `saveChanges` should be encrypted (since it
contains the actual content of the document).

Regarding how encryption is actually handled, the workspace encryption system will be used:

- The workspace's last key is used to encrypt.
- The key index is provided along with the encrypted payload (so a data encrypted
  with an old key can still be decrypted).
- The server reject the request if it doesn't use the lastest key index (this
  handles key rotation in case a user no longer as access to the workspace).

## 2 - The OnlyOffice communication protocol

The OnlyOffice protocol used between client and server is not clearly defined
(no spec, no schemas), it is instead implemented the yolo way directly in the code :/

The list of events used in the protocol can be obtained from the code:

- [Client->server events](https://github.com/ONLYOFFICE/server/blob/v9.3.1.11/DocService/sources/DocsCoServer.js#L1917-L2002)
- [Server->client events](https://github.com/ONLYOFFICE/sdkjs/blob/v9.3.1.11/common/docscoapi.js#L1854-L1931)

> [!NOTE]
> See [1030-collaborative-editics/oo-protocol-monitor/README.md] for an easy way
> to investigate the OnlyOffice protocol.
>
> Also see [./1030-collaborative-editics/oo_example_session.md] which contains
> the events of a typical edition session captured with this tool.

Those events are what uses the OnlyOffice client integrated into the Parsec GUI.
However we cannot implement as-is this protocol between the Parsec client and server
(i.e. the "editics protocol") since:

- The info related to the document's content must be end-to-end encrypted
  (e.g. document modification, cursor movement)
- The authentication system differs

### 2.1 - Session locks

Each OnlyOffice session has multiple locks:

- Save lock: Single mutex held by whichever client is currently in the middle of
  saving changes (it's taken when the client sends `saveChanges` event).
- Auth lock: Single mutex taken during auth handling: when the second non-view
  participant connects, the server takes this lock on the first participant's behalf
  and sends the newcomer a `waitAuth` event, making them wait for the established
  editor to finish its local save/switch to co-editing mode.
  The established editor is then expected to send an `unLockDocument` event with
  `unlock: true` once it's ready to let others in.
- Region lock: Multiple mutexes used by participants to lock on a document
  region (sheets/ranges in Excel, objects in Word/PPT, etc.) before editing it.
  Those locks are optimistically taken at edit time, and the edit is rolled back
  if the server denies the lock.

### 2.2 - Connection lifecycle overview

A client connection goes through a fixed sequence of events. Understanding the
individual events below requires seeing where they fit:

```raw
 WS open
   │
   ▼
 license (s→c)                    ← server pushes license/feature flags (very first msg)
   │
   ▼
 auth (c→s)                       ← client authenticates (docid, user, token, …)
   │                                this when the client actually join a session
   ▼
 ┌─────────────── waitAuth (s→c) ────────────────┐
 │  If the document auth-lock is held by another │
 │  editor (single-editor → co-editing switch):  │
 │  server sends waitAuth, then connectState     │
 │  (waitAuth:true) to everyone, and waits.      │
 │  The old editor must send unLockDocument      │
 │  {unlock:true} to release the auth lock,      │
 │  which lets the new editor's pending auth     │
 │  proceed (→ authChanges + auth).              │
 └───────────────────────────────────────────────┘
   │
   ▼
 authChanges (s→c) [0..N chunks]  ← backlog of changes since the doc was opened
   │                                (each chunk acked by the client with authChangesAck)
   ▼
 auth (s→c)                       ← server: session id, participants, locks, settings, jwt
   │
   ▼
 documentOpen (s→c)               ← server: where to fetch the document content (URL)
   │
   ▼
 …co-editing… (cursor, getLock, saveChanges, releaseLock, meta, connectState…)
   │
   ▼
 close (c→s) / drop / disconnectReason
```

The `auth` (c→s) is the gate: nothing else is allowed until it succeeds. The
server's reply ordering (`waitAuth`? → `authChanges` → `auth` (s→c) →
`documentOpen`) is enforced by `DocsCoServer.auth()`.

### 2.2 - Events

list of all OnlyOffice events:

| Event                | Origin | Kept in editics protocol |
|----------------------|--------|--------------------------|
| `auth`               | client |            ✅            |
| `auth`               | server |            ✅            |
| `waitAuth`           | server |            ✅            |
| `connectState`       | server |            ✅            |
| `authChangesAck`     | client |            ✅            |
| `authChanges`        | server |            ✅            |
| `message`            | client |            ✅            |
| `message`            | server |            ✅            |
| `getMessages`        | client |            ❌            |
| `cursor`             | client |            ✅            |
| `cursor`             | server |            ✅            |
| `getLock`            | client |            ✅            |
| `getLock`            | server |            ✅            |
| `releaseLock`        | server |            ✅            |
| `saveChanges`        | client |            ✅            |
| `saveChanges`        | server |            ✅            |
| `savePartChanges`    | server |            ✅            |
| `isSaveLock`         | client |            ✅            |
| `saveLock`           | server |            ✅            |
| `unSaveLock`         | client |            ✅            |
| `unSaveLock`         | server |            ✅            |
| `unLockDocument`     | client |            ✅            |
| `close`              | client |            ✅            |
| `drop`               | server |            ✅            |
| `warning`            | server |            ✅            |
| `license`            | server |            ❌            |
| `openDocument`       | client |            ❌            |
| `documentOpen`       | server |            ❌            |
| `clientLog`          | client |            ❌            |
| `extendSession`      | client |            ❌            |
| `session`            | server |            ❌            |
| `refreshToken`       | server |            ❌            |
| `expiredToken`       | server |            ❌            |
| `forceSaveStart`     | client |            ❌            |
| `forceSaveStart`     | server |            ❌            |
| `forceSave`          | server |            ❌            |
| `rpc`                | client |            ❌            |
| `rpc`                | server |            ❌            |
| `meta`               | server |            ❌            |
| `disconnectReason`   | server |            ❌            |
| `error`              | server |            ❌            |
| `updateVersion`      | server |            ❌            |

Detail for each command:

#### `auth` (client → server) & `auth`/`waitAuth` (server → client)

It is this event that authenticates the client (see `jwtOpen` field) and connect
it to an edition session according to the document ID its provides (see `docid` field).

> [!NOTE]
> The document ID is also present in the URL of the websocket (e.g.
> `wss://site.docs.onlyoffice.com/9.4.1/web-apps/apps/documenteditor/doc/<documentID>/c/?shardkey=<documentID>&EIO=4&transport=websocket`)
> but this is only used for sharding purpose (i.e. the reverse proxy acts as load
> balancer and uses this `shardkey` parameter to connect all clients editing a
> given document to the same server instance).
> This is needed since each edition session only lives in a single server instance's
> memory.

Quick reference, who sends what during a auth handshake:

```raw
New editor (Kate)                       Server                         Existing editor (John)
   │                                      │                                      │
   │── WS open ──────────────────────────►│                                      │
   │◄── license ──────────────────────────│                                      │
   │── auth (c→s) ───────────────────────►│                                      │
   │   (docid, user, token, supportAuthChangesAck:true)                          │
   │                                      │── connectState {waitAuth:true} ─────►│
   │◄── waitAuth {lockDocument: John} ────│                                      │
   │   (Kate is parked; John is nudged)   │                                      │
   │                                      │◄── unLockDocument {unlock:true} ─────│
   │                                      │   (John releases the auth lock)      │
   │◄── authChanges [chunk 1] ────────────│                                      │
   │── authChangesAck ───────────────────►│                                      │
   │◄── authChanges [chunk 2 if needed] ──│                                      │
   │── authChangesAck ───────────────────►│                                      │
   │◄── auth (s→c) ───────────────────────│                                      │
   │  (sessionId, participants[John,Kate],│                                      │
   │   locks, settings, jwt, openedAt)    │                                      │
   │◄── documentOpen ─────────────────────│                                      │
   │   (URLs to fetch the document)       │                                      │
   │                                      │── connectState {waitAuth:false} ────►│
   │                                      │   (now 2 editors; co-editing on)     │
```

The handshake is driven by the client's `ConnectionState` enum (defined in
`sdkjs/common/docscoapicommon.js`, client-side only, not a network message):

| State            | Meaning                                                                                          |
|------------------|--------------------------------------------------------------------------------------------------|
| `Reconnect` (-1) | transport lost, waiting to reconnect                                                              |
| `None` (0)       | not initialized                                                                                  |
| `WaitAuth` (1)   | socket open, `auth` (c→s) sent, waiting for the server `auth` (s→c) reply (or `waitAuth` first)   |
| `Authorized` (2)| `auth` (s→c) received, the client is a full participant                                          |
| `ClosedCoAuth` (3) | co-authoring disabled (e.g. demoted to viewer)                                                 |
| `ClosedAll` (4)  | connection fully closed                                                                         |
| `SaveChanges` (10) | client is in the middle of a save round (askSaveChanges/sendChanges handshake)                  |
| `AskSaveChanges` (11) | same, intermediate sub-state                                                                 |

The client moves to `WaitAuth` on socket open, then to `Authorized` once the
server `auth` (s→c) reply arrives. Nothing else (cursor, changes, locks…) is
allowed on the socket until `Authorized` is reached.

Format of the client → server `auth` event:

```json5
{
  "type": "auth",
  "payload": {
    "type": "auth",
    "docid": <string>,            // OnlyOffice session ID (= workspace_id + document vlob ID in editics)
    "token": <string>,            // Integrator-provided document token (also in the JWT below)
    "user": {
      "id": <string>,            // Integrator-provided user id (e.g. the device id)
      "username": <string>,
      "firstname": <string|null>,
      "lastname": <string|null>,
      "indexUser": <integer>     // -1 on first open, the client's participant index on reconnection
    },
    "editorType": <integer>,      // 0=Word, 1=Spreadsheet, 2=Presentation, 3=Visio
    "lastOtherSaveTime": <integer>, // last save timestamp the client knows about, -1 on first open
    "block": <array>,            // Region lock block ids the client believes it still owns (restore)
    "sessionId": <string|null>,  // null on first open, the session id on reconnection (marks this as a restore)
    "sessionTimeConnect": <integer|null>, // null on first open, server-provided connect time on reconnection
    "sessionTimeIdle": <integer>,// How long the client considers itself idle (ms)
    "documentFormatSave": <integer>, // Output format id the server should use when exporting (see forceSave)
    "isCloseCoAuthoring": <boolean>,
    "openCmd": <object|null>,     // The `open` command that triggered the session (contains doc url, format, etc.)
    "lang": <string>,
    "mode": <string>,            // "edit" | "view"
    "permissions": {
      "edit": <boolean>,
      "review": <boolean>
    },
    "encrypted": <boolean>,
    "IsAnonymousUser": <boolean>,
    "timezoneOffset": <integer>,
    "headingsColor": <string|null>,
    "coEditingMode": <string>,   // "fast" | "strict"
    "jwtOpen": <string>,         // Opening JWT token (proves authorization to open this document)
    "jwtSession": <string>,      // Session JWT token (proves a previously-established session), takes precedence over `jwtOpen`
    "time": <integer>,           // client-side `performance.now()`, for server-side timing logs
    "supportAuthChangesAck": <boolean> // capability flag: if true the client will ack `authChanges` chunks (modern clients)
  }
}
```

Server handling (`DocsCoServer.auth()`):

1. Verifies the JWT (`jwtSession` takes precedence over `jwtOpen`), fills the
   `data` object from it.
2. Parses `openCmd`, looks up the document row (`taskResult.select`).
3. Determines `bIsRestore`: a reconnection is detected when `sessionId` /
   `sessionTimeConnect` are present (i.e. the client is restoring a known
   session rather than opening fresh). On restore the server skips the
   auth-lock logic and the document-open flow.
4. Assigns `conn.sessionId`, `conn.user.indexUser`, registers the connection,
   builds the participant map, stores `conn.supportAuthChangesAck`.
5. Decides the **auth lock** (see `waitAuth` below): if this is the second
   non-view participant and the first one still holds the lock, the newcomer
   gets a `waitAuth` and the rest of the handshake is deferred.
6. Otherwise (or once the lock is released) sends `authChanges` (the change
   backlog) then the `auth` (s→c) reply.

Format of the server → client `auth` reply:

```json5
{
  "type": "auth",
  "payload": {
    "type": "auth",
    "result": <integer>,          // 1 = success
    "sessionId": <string>,        // The connection's session id (used on reconnection)
    "sessionTimeConnect": <integer>, // Server timestamp (ms) at connect (used on reconnection)
    "participants": <array>,      // Current participant map (same shape as in `connectState`)
    "messages": <array|undefined>, // Chat messages (in practice the server leaves this empty; see `getMessages`)
    "locks": <object|array>,      // Current region lock table (same shape as server `getLock` reply)
    "indexUser": <integer>,       // This connection's participant index
    "hasForgotten": <boolean>,    // Whether the document has unsaved "forgotten" changes
    "jwt": <string>,             // Fresh session JWT token (replaces `jwtSession`)
    "g_cAscSpellCheckUrl": <string>,
    "buildVersion": <string>,
    "buildNumber": <integer>,
    "licenseType": <integer>,
    "settings": <object>,        // Editor config: reconnection params, `binaryChanges`, `websocketMaxPayloadSize`, `maxChangesSize`, image limits, etc.
    "openedAt": <integer>        // Server timestamp (ms) passed through to `documentOpen`
  }
}
```

On the client, `_onAuth` distinguishes two cases:

- **First auth** (`data.result === 1`, `_isAuth` was false): stores the session
  id / index / connect time, applies reconnection settings, refreshes
  participants, applies any offline changes (`AscChanges`), then runs
  `_updateAuthChanges()` to apply the buffered `authChanges`, and finally
  `onFirstLoadChangesEnd(openedAt)` (which lets the editor know the document
  history is fully loaded).
- **Reconnection** (`_isAuth` already true): refreshes token, participants,
  messages and locks, and if a save was in flight when the connection dropped
  (`_isReSaveAfterAuth`) it re-issues `askSaveChanges` to retry it.

In both cases the client moves to `ConnectionState.Authorized`.

*Editics protocol changes*:

- Rename to `editics_auth`.
- Remove `jwtOpen` / `jwtSession` / `jwt`: Parsec has its own authentication
  (the SSE endpoint is already authenticated), so OnlyOffice JWTs are not needed.
- Remove `documentCallbackUrl`, `documentFormatSave`, `headingsColor`,
  `coEditingMode`, `IsAnonymousUser`, `sessionTimeIdle`: those are OnlyOffice-
  integrator specific and don't apply to Parsec.
- Remove `openCmd`: the document is loaded entirely client-side (see `documentOpen`).
- Remove `mode`/`permissions`: access control is enforced by the Parsec server
  (realm roles) before the SSE connection is even established.
- Remove `messages`, `g_cAscSpellCheckUrl`, `buildVersion`/`buildNumber`,
  `licenseType`, `settings`: those are OnlyOffice integrator features the Parsec
  server has no business providing.
- Remove `lastOtherSaveTime`: the save flow in the editics protocol relies on the
  change index, not on a save timestamp.
- Keep `docid` (the session id), `user.id` (the device id), `editorType`,
  `sessionId`/`sessionTimeConnect` (restore detection), `indexUser`,
  `supportAuthChangesAck`, and `time` (telemetry).
- The server `auth` (s→c) reply is replaced by the `bootstrap` SSE event (see
  §3.1) which carries the participant map and the encrypted change backlog in a
  single message; the separate `authChanges` flow is folded into it.

#### `waitAuth` (server → client)

This is the **single-editor auth lock** during initial open. It exists so that
when a second editor joins while the first one hasn't finished loading the
document, the newcomer waits until the first one has applied the full change
history and switched to co-editing mode — preventing both editors from
starting from divergent baselines.

The lock is taken in `DocsCoServer.auth()` the moment the **second non-view
participant** connects (condition: `2 === countNoView && !tmpUser.view`), on
behalf of the first participant (`firstParticipantNoView.id`). It is stored in
Redis (`editorData.lockAuth`), keyed by `docId`, owned by the first editor's
user id. A safety timer (`setLockDocumentTimer`, `tenExpLockDoc` × 2) force-
releases the lock and drops the first editor if it never unlocks.

While the lock is held, the newcomer is sent `waitAuth` **instead of** the
`authChanges` + `auth` (s→c) reply, and a `connectState { waitAuth: true }` is
broadcast to everyone (nudging the first editor to release the lock). The
deferred `authChanges` + `auth` (s→c) are delivered once the first editor sends
`unLockDocument { unlock: true }`, which triggers `checkEndAuthLock` →
`editorData.unlockAuth` → a published `auth` event that fans the handshake out
to all waiting participants.

Format:

```json5
{
  "type": "waitAuth",
  "payload": {
    "type": "waitAuth",
    // The participant currently holding the auth lock (i.e. the established
    // editor the newcomer must wait for). Same shape as a `participants` entry:
    //   { id, idOriginal, username, indexUser, view, connectionId,
    //     isCloseCoAuthoring, isLiveViewer, encrypted }
    "lockDocument": <object>
  }
}
```

On the client the `waitAuth` case in the dispatcher is a **no-op** by itself;
its effect comes through the `connectState` message that carries
`waitAuth: true`, which triggers `onStartCoAuthoring(isStartEvent=false,
isWaitAuth=true)` → `_unlockDocument(isWaitAuth)` (see `connectState`).

*Editics protocol changes*:

- Kept (renamed to `wait_auth`), it is needed to implement the
  exclusive → co-editing transition (see §1.4 and the `co_editing_ready` field
  of the `participants` SSE event in §3.1).
- `lockDocument` is replaced by the holder's `device_id` / participant index
  only (the client already knows the participant details from the `bootstrap`
  / `participants` events).

#### `connectState` (server → client)

Broadcast to **all** participants whenever the participant set changes (a
client joins, leaves, or is dropped). It is also the vehicle that carries the
`waitAuth` flag which nudges the established editor to release the auth lock.

Produced by `DocsCoServer.sendParticipantsState()`, published on every join
(after `auth()`) and every disconnect. Each message carries a monotonically
increasing `participantsTimestamp` so clients can ignore out-of-order/stale
broadcasts.

Format:

```json5
{
  "type": "connectState",
  "payload": {
    "type": "connectState",
    // Monotonic timestamp (ms) of this participant-set update. The client
    // keeps `_participantsTimestamp` and ignores any message whose timestamp
    // is older, so reordered deliveries don't clobber a newer state.
    "participantsTimestamp": <integer>,
    // Full replacement of the participant set (not a delta). Each entry:
    //   { id, idOriginal, username, indexUser, view, connectionId,
    //     isCloseCoAuthoring, isLiveViewer, encrypted }
    //  - `id`: `<userId><indexUser>` composite id used as the participant key.
    //  - `idOriginal`: integrator-provided user id (the device id in editics).
    //  - `indexUser`: the participant index (order of arrival in the session).
    //  - `view`: whether the participant is a viewer (read-only).
    //  - `connectionId`: the underlying connection's id (= `sessionId`).
    //  - `isLiveViewer`/`isCloseCoAuthoring`/`encrypted`: feature flags.
    "participants": <array>,
    // true while the document auth lock is held (see `waitAuth`). When true it
    // tells the established editor it must send `unLockDocument { unlock: true }`
    // once its document is loaded, to release the lock and let newcomers proceed.
    "waitAuth": <boolean>
  }
}
```

Client handling (`DocsCoApi._onConnectionStateChanged`):

1. Drops the message if `participantsTimestamp` is stale.
2. Updates `_participants` / `_countEditUsers`, computes the diff
   (`usersStateChanged`).
3. If the diff is non-empty and there is more than one edit user → calls
   `onStartCoAuthoring(isStartEvent=false, isWaitAuth)`, which on the editor
   side calls `_unlockDocument(isWaitAuth)` → sends `unLockDocument`.
4. Fires `onConnectionStateChanged` per changed user (join/leave notifications
   to the editor UI).

The `waitAuth` value and the `isWaitAuth` argument exist mostly as sanity
checks (the client logs a `changesError` if `waitAuth` is set but the
participant set didn't actually change or there aren't >1 edit users, which
would indicate a race condition).

*Editics protocol changes*:

- Rename to `participants` (this is exactly the `participants` SSE event in
  §3.1).
- Replace `participantsTimestamp` with a `DateTime timestamp` (same monotonic
  role).
- Replace the `participants` array with a `Dictionary<Integer, DeviceID>`
  (participant index → device id), since the client already knows the rest.
- Replace the `waitAuth` boolean with `co_editing_ready: Boolean` (inverted
  meaning: `true` once the established editor has released the auth lock and
  co-editing is fully on; corresponds to `waitAuth: false`).

#### `authChanges` (server → client)

Delivered during the auth handshake (after `waitAuth`, if any, and before the
`auth` (s→c) reply), this is the **backlog of document changes** that occurred
in the session before the newcomer joined. It lets a freshly-connecting client
reach the exact same document state as the existing editors.

The change set can be large, so the server sends it in **chunks** bounded by
`websocketMaxPayloadSize` (~1.5MB), and uses an **ack-based flow control** when
the client declared `supportAuthChangesAck: true`: the server sets
`conn.authChangesAck = true`, sends a chunk, then blocks (polling every 100ms,
up to 30s) until the client replies with `authChangesAck`. Only sent to
`needSendChanges` connections (editors and live viewers; pure viewers are
skipped).

Format:

```json5
{
  "type": "authChanges",
  "payload": {
    "type": "authChanges",
    // A slice of the document's change history, in order. Each entry is one
    // stored change:
    //   { docid, change, time, user, useridoriginal }
    //  - `change`: an opaque OnlyOffice change fragment (JSON or binary,
    //     depending on `settings.binaryChanges`).
    //  - `time`: server timestamp (ms) the change was stored.
    //  - `user`: composite `<userId><indexUser>` of the change's author.
    //  - `useridoriginal`: integrator-provided user id of the author.
    // The whole array is at most `websocketMaxPayloadSize`-worth of changes.
    "changes": <array>
  }
}
```

Client handling (`DocsCoApi._onAuthChanges`): the client **buffers** the chunk
into `_authChanges` and immediately replies with `authChangesAck` (so the
server can send the next chunk). The buffered changes are applied all at once
later, by `_updateAuthChanges()`, called from `_onAuth` right before
`onFirstLoadChangesEnd`. `_updateAuthChanges` also reconciles against
`_authOtherChanges` (changes that arrived via `saveChanges` broadcasts while the
auth handshake was in progress) by computing `changesIndex` diffs so nothing
is double-applied.

> [!NOTE]
> The `authChanges` chunks are not interleaved with the `auth` (s→c) reply:
> they are sent first (once the auth lock is clear), then the `auth` reply is
> sent, then `documentOpen`. The client only applies the buffered changes in
> `_updateAuthChanges()` after it has processed the `auth` reply (so it knows
> its session id, participant index, and the lock table).

*Editics protocol changes*:

- Folded into the `bootstrap` SSE event (§3.1): the change backlog is delivered
  as `encrypted_changes: List<(Index, Bytes)>` in the same message that
  acknowledges the join. No separate chunking / ack flow is needed since SSE
  is one-directional and the server controls the send rate (and the changes are
  end-to-end encrypted, so the server can't size-optimize them by inspecting
  content).
- The opaque `change` fragments are replaced by `(Index, Bytes)` tuples
  (change index + encrypted change blob).
- Drop the `docid`/`user`/`useridoriginal` fields: the author is conveyed by
  the `participant` field of the `change` SSE event, and the session is
  single-document.

#### `authChangesAck` (client → server)

Acknowledge one `authChanges` chunk so the server can send the next one (see
`authChanges`). This is the flow-control half of the chunked handshake.

Format:

```json5
{
  "type": "authChangesAck",
  "payload": {
    "type": "authChangesAck"
  }
}
```

Server handling (`DocsCoServer` dispatch): `delete conn.authChangesAck`, which
unblocks the chunk-sending loop (`sendAuthChangesByChunks`) so it can send the
next chunk or proceed to the `auth` (s→c) reply. If the ack doesn't arrive
within 30s the server gives up waiting (the `authChangesAck` flag is cleared
and the loop continues regardless).

*Editics protocol changes*:

- Removed: there is no chunked flow to ack since `authChanges` is folded into
  the single `bootstrap` SSE event (see `authChanges`).

#### `message` (client → server) & `message` (server → client)

Send a message in the chat.

Format:

```json5
{
  "type": "message",
  "payload": {
    "type": "message",
    "message": <string>  // Actual message
  }
}
```

*Editics protocol changes*:

- Rename to `send_message`.
- Encrypt `message` field.

This leads to the server to send its own `message` event to inform other users.

> [!NOTE]
>
> Sender's `message` is also send when the client sends a `getMessages` event.

Format:

```json5
{
  "type": "message",
  "payload": {
    "type": "message",
    "messages": [
      {
        "docid": <string>,
        "message": <string>,  // Actual message
        "time": <integer>,  // timestamp in ms
        "user": <string>,
        "useridoriginal": <string>,
        "username": <string>
      }
    ]
  }
}
```

*Editics protocol changes*:

- Remove field `docid` (as the session is only related to a single document).
- Replace fields `user`/`useridoriginal`/`username` by `device_id` (the client
  has already a single source of truth on those info).
- Encrypt `message` field.

#### `getMessages` (client → server)

Get all the chat messages for the session.

Format:

```json5
{
  "type": "getMessages",
  "payload": {
    "type": "getMessages"
  }
}
```

> [!NOTE]
> This even is not needed in theory: the client checks for a `messages` field in
> the server `auth` event.
> However in practice the server never set this `messages` field, hence why the
> client has to explicitly send a `getMessages` event.

*Editics protocol changes*: Ignore this event since the `auth` event sent by the server
always contains the session messages.

#### `cursor` (client → server) & `cursor` (server → client)

Move the cursor.

Format:

```json5
{
  "type": "cursor",
  "payload": {
    "type": "cursor",
    "cursor": <string> // Opaque string in OnlyOffice internal format
  }
}
```

*Editics protocol changes*:

- Rename to `move_cursor`.
- Encrypt `cursor` field.

This leads to the server to send its own `message` event to inform other users.

Format:

```json5
{
  "type": "cursor",
  "payload": {
    "type": "cursor",
    "messages": [
      {
        "cursor": <string>, // Opaque string in OnlyOffice internal format
        "time": <integer>,  // timestamp as ms
        "user": <string>,
        "useridoriginal": <string>
      }
    ]
  }
}
```

*Editics protocol changes*:

- Rename to `cursor_moved`.
- Encrypt `cursor` field.
- Replace fields `user`/`useridoriginal` by `device_id` (the client has already a
  single source of truth on those info).

#### `getLock` (client → server) & `getLock` (server → client)

Acquire a [region locks](#21---session-locks) to prevent concurrent edition on some
part of the document.

This event is used for heavy structural edit (insert/edit images, tables, headers/footers,
sheet ranges, slide objects, etc.).
For most edits (e.g. text edit) the region locks are instead optimisticly taken (i.e.
editing → isSaveLock → saveChanges with no getLock ever sent).

Client format:

```json5
{
  "type": "getLock",
  "payload": {
    "type": "getLock",
    // Array of block descriptors the client wants to acquire a lock on.
    // Shape depends on editor type:
    //  - Word: a plain string block id (a "guid").
    //  - Spreadsheet: { sheetId, type, rangeOrObjectId, guid }
    //  - Presentation/diagram: { type, val } or { type, slideId, objId }
    "block": <array>
  }
}
```

*Editics protocol changes*:

- Rename to `acquire_region_lock`

Then the server *to every clients* (including the one that send the `getLock` event) a `getLock` event.

Server format:

```json5
{
  "type": "getLock",
  "payload": {
    "type": "getLock",
    // Map of block-id → lock record, describing the full lock table as it
    // stands after the server attempted to acquire the requested blocks for
    // the requester. In other words: the locks the client just asked for are
    // now present in this map, attributed to the requester's `user` id (if
    // they were free); locks held by others appear with their `user` id. Each
    // record:
    //   { time, user, block }
    //  - `time`: server timestamp (ms) when the lock was taken/checked.
    //  - `user`: id of the user holding the lock. For the blocks the
    //     requester just acquired, this is the requester's own id.
    //  - `block`: the original block descriptor (same shape the client sent).
    //
    // For the Document editor the map is keyed by the plain block id and the
    // value's `block` is the string id. For Spreadsheet/Presentation/PDF the
    // client re-keys by `block.guid`.
    //
    // A record is never null in this event (null is only used in releaseLock
    // to signal "gone"). The `user` field tells each receiver whether the
    // lock is theirs or someone else's.
    "locks": <object | array>
  }
}
```

*Editics protocol changes*:

- Rename to `region_lock_acquired`

#### `releaseLock` (server → client)

Format:

```json5
```

*Editics protocol changes*:

#### `saveChanges` (client → server) & `saveChanges`/`savePartChanges` (server → client)

Send some modification in the document, this requires to have the lock on the
document first.

> [!NOTE]
>
> - *change ordering*: A global index is held to order each change.
>
> - *Chunking of changes*: A single save operation may be too large for one
>   WebSocket frame, so the client chunks the changes array across several
>   saveChanges messages. The `startSaveChanges` / `endSaveChanges` flags delimit
>   the chunked sequence.
>
> - *`changes` field format*: `saveChanges` contains a `changes` field that is an
>   array of modifications. By default this is represented as a JSON-serialized
>   array of raw op fragments (an OnlyOffice-internal binary/JSON change format
>   we never need to understand).
>
>   A typical example of `changes` field in default format:
>   `"[\"64;AgAAADEA//8BACxLuimoIAIApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///w4AAAAwAC4AMAAuADAALgAwAA==\",\"37;> CAAAADAAXwAyADQAAQAcAAEAAAAFAAAAAQAAAGkAAAAAAwAAAA==\"]"`
>
>   Default format is wasteful however so we want to use instead the binary format
>   (since the actual communication with the Parsec server is going to be done in
>   msgpack that supports binary data).
>
>   Switching to binary format can be done by setting `editorConfig.settings.binaryChanges`
>   to `true` > in `window.DocsAPI.DocEditor`'s config parameter.
>
> - *Spreadsheet Editor lock calculation*: `excelAdditionalInfo` field contains
>   aditional info that are used by the server to compute locks on cells.

Format:

```json5
{
  "type": "saveChanges",
  "payload": {
    "type": "saveChanges",
    // List of opaque OnlyOffice-internal change fragment, see the note about its format.
    "changes": <string | binary>,
    // true on the first chunk of this save operation.
    // The server only honors `deleteIndex` and establishes the new "save point"
    // when this is true.
    "startSaveChanges": <boolean>,
    // true on the last chunk of this save operation.
    // The server only finalizes the save when this is true: it then broadcasts
    // the changes to other participants, optionally releases locks, and sends
    // back an `unSaveLock` event. Intermediate chunks get a `savePartChanges`
    // ack instead.
    "endSaveChanges": <boolean>,
    // Whether more than one user is currently co-editing the document.
    // Maintained by the client (toggled by the `startCoAuthoring` /
    // `endCoAuthoring` server events). The server only uses it for the
    // spreadsheet editor, to gate the `excelAdditionalInfo` lock recalculation.
    "isCoAuthoring": <boolean>,
    // true if the editor is the Spreadsheet Editor.
    // The server uses it to decide whether to apply the `excelAdditionalInfo`
    // column/row lock recalculation. Other editor types ignore that field.
    "isExcel": <boolean>,
    // Ask the server to truncate changes up to (including) this index. This is
    // used to rollback when the user use the undo changes in the document.
    // Can by `null` or `-1` to indicate no truncate is needed.
    // Ignored if `startSaveChanges` is not set.
    "deleteIndex": <null|integer>,
    // JSON-serialized opaque blob for the Spreadsheet Editor:
    //   { "UserId": ..., "UserShortId": ..., "CursorInfo": ...,
    //     "indexCols": ..., "indexRows": ... }
    // - `CursorInfo` is broadcast as-is to other participants (cursor display).
    // - `indexCols` / `indexRows` describe inserted columns/rows; the server
    //   uses them to recalculate the locked ranges of other users so their
    //   locks follow the shifted cells. Only used for the spreadsheet editor.
    "excelAdditionalInfo": <null|string>,
    // Whether the server should release the document's auth lock (not the region locks !)
    // after this save.
    // This is only true when switch from solo to collaborative edition (note this
    // flag is also carride by an `unLockDocument` event since `saveChanges` is
    // only send if the document has been modified).
    "unlock": <boolean>,
    // Whether the server should release the region locks held by this user.
    "releaseLocks": <boolean>,
    // Set by the client when re-sending a failed save (server closes connection
    // or timeouts). Only used on the server for logging purpose.
    "reSave": <integer | undefined>
  }
}
```

*Editics protocol changes*:

- Rename to `save_changes`.
- Encrypt `changes` fields.
- Split `excelAdditionalInfo` into `cursor` (encrypted field) and `excel_info` (containing `index_cells` and `index_rows`)
- Switch to binary format for `changes` field.
- Remove `unlock` field: we rely on the fact `unLockDocument` is always send after and contains this field.
- Remove `reSave`
- Remove `isExcel`: instead set `excel_info` to null if not excel.
- Consider removing `isCoAuthoring`: the server should be aware of the state.

This leads to the server to send its own event to inform other users:

- To the client the original `saveChanges` is coming from: `unSaveLock` if `endSaveChanges: true`, `savePartChanges` otherwise.
- To all other clients: `saveChanges`

> [!NOTE]
> Server also sends a `unSaveLock` event if `endSaveChanges: true`.

Format:

 ```json5
{
  "type": "saveChanges",
  "payload": {
    "type": "saveChanges",
    // Array of { docid, change, time, user, useridoriginal }, with:
    // - `change`: the opaque change fragment
    // - `time`: server timestamp (ms) of the change.
    // - `user`: id of the user who made the change.
    // - `useridoriginal`: original (integrator-provided) user id.
    //
    // Null when the change set was too large to publish inline; in that case
    // the client refetches the missing changes (via the auth/changes flow).
    "changes": <array | null>,
    // New total number of changes stored for the document after this save
    "changesIndex": <integer>,

    // `syncChangesIndex` is the always-advancing sync point, while `changesIndex`
    // might be lower (in case of undo, see )
    // Same value as `changesIndex` here (= `puckerIndex`). Tracked separately
    // by the client as `syncChangesIndex` (the "always-advancing" sync point,
    // used in the `isSaveLock` / `unSaveLock` handshake). `changesIndex` can
    // lag behind it (it is reset to the save point on the saver's
    // `unSaveLock.index`), while `syncChangesIndex` always reflects the total.
    "syncChangesIndex": <integer>,

    // Mirrors the originator's `endSaveChanges`
    "endSaveChanges": <boolean>,

    // Locks released by the originator (only when its `releaseLocks` was true).
    // Each entry: { block, user, time, changes }. For spreadsheet/presentation/
    // pdf editors, `block` is an object with a `guid`; for the document editor
    // it's a plain block id. The receiver marks those locks as released and
    // notifies its lock manager (`onLocksReleased`).
    // Array of { block, user, time, changes }, with:
    // - `block`
    // - `user`
    // - `time`: server timestamp (ms) of the change.
    "locks": <array>,

    // The originator's `excelAdditionalInfo`, passed through unchanged.
    // Spreadsheet clients use it to recalculate their own lock ranges
    // (`onRecalcLocks`) and for cursor display; other editors ignore it
    // (or use only the cursor portion).
    "excelAdditionalInfo": <string | undefined>
  }
}
 ```

 Notes:
 - Unlike the client→server message, the server→client saveChanges does not carry startSaveChanges, isCoAuthoring, isExcel, deleteIndex, unlock, or releaseLocks — only the fields other participants need to apply the changes.
 - The saving client itself does not receive this broadcast; it receives unSaveLock (and savePartChanges for intermediate chunks) instead.

Format:

```json5
```

*Editics protocol changes*:

#### `savePartChanges` (server → client)

Format:

```json5
```

*Editics protocol changes*:


#### `isSaveLock` (client → server) & `saveLock` (server → client)

Try to take the lock to save the document.

 Format:

```json5
{
  "type": "isSaveLock",
  "payload": {
  "type": "isSaveLock",
    // The client's current change index (i.e. the total number of changes it
    // has observed for the document so far from the server's `saveChanges` and
    // `unSaveLock` events).
    // This is used by the server to detect a desynchronized client, in such
    // case the server keeps returning `saveLock: true` (denied) so that the
    // client should catch up before retrying.
    "syncChangesIndex": <integer>
  }
}
 ```

This leads to the server to send its own event to the client:

```json5
{
  "type": "saveLock",
  "payload": {
  "type": "saveLock",
    // true means somebody else already holds the lock (i.e. the lock wasn denied)
    "saveLock": <boolean>
  }
}
 ```

#### `unSaveLock` (client → server) & `unSaveLock` (server → client)

 Cancel an in-progress save operation and release the save lock without saving anything.

Under normal circumstances, the lock is automatically released when the client
sends a `saveChanges` (see `releaseLocks` field).

 Format:

 ```json5
{
  "type": "unSaveLock",
}
```

*Editics protocol changes*:

- Rename to `un_save_lock`

This leads to the server to send its own event to the client:

> [!NOTE]
> `unSaveLock` server event is used for two things:
>
> - Cancellation scenario: Response to `unSaveLock` client event
> - Success scenario: Response to a successful save (i.e. `saveChanges` event with `endSaveChanges: true`)

Format:

 ```json5
{
  "type": "unSaveLock",
  "payload": {
    "type": "unSaveLock",
    // The client's new save point (the absolute change index at which this
    // user's changes were committed) or `-1` in case of cancellation.
    "index": <integer>,
    // Server timestamp (ms) of the last change in this save or `-1` in case of cancellation.
    "time": <integer>,
    // The new total number of changes stored for the document or `-1` in case of cancellation.
    "syncChangesIndex": <integer>
  }
}
```

#### `unLockDocument` (client → server)

 Notify the server that the client is leaving active editing, or wants to drop its locks and/or the document auth lock. It is sent:
- on disconnect / closing the editor,
- when the user stops being the "exclusive" editor (single-editor → co-authoring transition),
- when explicitly releasing region locks (e.g. after a save that released locks server-side, or on undo).

Format:

```json5
{
  "type": "unLockDocument",
  "payload": {
    "type": "unLockDocument",
    // Indicates if the client is currently in a save operation. If so, its save
    // lock will be released and a `unSaveLock` event is sent by the server.
    "isSave": <boolean>,
    // Indi
    "unlock": <boolean>,  // TODO: document
    // Ignored if `null` or `-1`, otherwise inform the server that the changes
    // up to this index can be destroyed ().
    "deleteIndex": <integer|null>,  // TODO
    "releaseLocks": <boolean>  // TODO: document
  }
}
```






 It is a "fire-and-forget cleanup" that optionally combines three independent actions (auth unlock, lock release, save-lock release).

 Format:

 ```json5
   {
     "type": "unLockDocument",
     "payload": {
       "type": "unLockDocument",

       // Whether the client is currently IN A SAVE OPERATION (i.e. it holds the
       // save lock from a prior `isSaveLock`/`saveLock` handshake). If true, the
       // server releases that save lock by sending back an `unSaveLock` event
       // (with `index: -1, time: -1, syncChangesIndex: -1` — an EMERGENCY
       // withdrawal without saving).
       //
       // In practice the client passes `false` here for a normal cleanup
       // (the save lock is normally released by the `saveChanges` flow itself).
       // It is the emergency/companion path to `unSaveLock` (client→server),
       // which is used to CANCEL a save.
       "isSave": <boolean>,

       // Whether the server should release the document's AUTH lock for this user.
       // The auth lock is the "I am the current exclusive editor" lock (distinct
       // from per-region locks and the save lock). When true, the server calls
       // `editorData.unlockAuth`; if it succeeds (the lock was indeed this user's),
       // it publishes an `auth` event to all participants so another user can take
       // over the document. Maps to the client's `canUnlockDocument` flag.
       //
       // False during ordinary saves; set true when the client genuinely leaves
       // the document (disconnect, or handing off to another editor).
       "unlock": <boolean>,

       // Same semantics as the `deleteIndex` field of `saveChanges` (UNDO support):
       //  - null  → no truncation of change history.
       //  - -1    → no truncation (sentinel, same effect as null).
       //  - <int> → ABSOLUTE index; the server deletes all stored changes with
       //            index >= deleteIndex before doing anything else. Used so the
       //            client can throw away its last changes during an undo that
       //            coincides with leaving the document.
       "deleteIndex": <integer | null>,

       // Whether the server should release the REGION locks held by this user
       // (so other users can lock those areas). Maps to the client's
       // `canReleaseLocks` flag. When true, the server removes the user's locks
       // via `removeUserLocks`, sends a `releaseLock` event directly to the
       // client, AND broadcasts a `releaseLock` event to all other participants
       // (each lock: `{ block, user, time, changes }`).
       //
       // Note the difference from `saveChanges`'s `releaseLocks`: in `saveChanges`
       // the released locks are returned to OTHER participants embedded inside
       // the server→client `saveChanges` message (`locks` field). Here, in
       // `unLockDocument`, they are sent as a standalone `releaseLock` event.
       "releaseLocks": <boolean>
     }
   }
 ```




*Editics protocol changes*:

- Rename to `un_lock_document`

#### `close` (client → server)

Notify the server that the client is voluntarily leaving the session and closing
the connection (unlike `unLockDocument` that keeps the connection alive but drops
locks).

Format:

```json5
{
  "type": "close",
  "payload": {
    "type": "close"
  }
}
```

*Editics protocol changes*: Keep as-is.

#### `drop` (server → client)

Inform the client its editing session has been terminated by an external action (typically
the integrator has revoked the user's permissions).

Format:

```json5
{
  "type": "drop",
  "payload": {
    "type": "drop",
    // Always the DROP_CODE constant (4007), included for symmetry with `disconnectReason`.
    "code": 4007,
    // A free-form description string, usually empty.
    "description": <string>
  }
}
```

*Editics protocol changes*: Keep as-is.

#### `warning` (server → client)

Send a warning message to the client to be displayed to the end user.

Format:

```json5
{
    "type": "warning",
    "code": <integer>, // `-200`: FORCED_VIEW_MODE, `-201`: FILE_NOT_ASSEMBLED
    "message": <string>
}
```

*Editics protocol changes*:

- Rename to `warning_received`

#### `license` (server → client)

Event send right after the wesocket opens, the client uses it to
enable/disable editor features and to decide branding/customization.

Format:

```json5
{
  "type": "license",
  "payload": {
    "type": "license",

    // License descriptor. Fields:
    //  - type:               license type id (integer; e.g. 3 in the logs).
    //  - light:              legacy boolean, always false.
    //  - mode:               license mode (integer; 0 in the logs).
    //  - rights:             bitmask of rights (integer; 1 in the logs).
    //  - buildVersion:       server build version string (e.g. "9.4.1").
    //  - buildNumber:        server build number (integer; e.g. 15).
    //  - protectionSupport:  whether protected-file opening is supported.
    //  - isAnonymousSupport: whether anonymous users are supported.
    //  - liveViewerSupport:  whether the live viewer feature is supported.
    //  - branding:           whether branding/white-label is allowed.
    //  - customization:      whether UI customization is allowed.
    //  - advancedApi:        whether the advanced editor API is available.
    "license": {
      "type": <integer>,
      "light": <boolean>,
      "mode": <integer>,
      "rights": <integer>,
      "buildVersion": <string>,
      "buildNumber": <integer>,
      "protectionSupport": <boolean>,
      "isAnonymousSupport": <boolean>,
      "liveViewerSupport": <boolean>,
      "branding": <boolean>,
      "customization": <boolean>,
      "advancedApi": <boolean>
    },

    // AI plugin settings for the editor UI (may be absent). Forwarded to
    // the editor via `onAiPluginSettings` if the license init is happening.
    "aiPluginSettings": <object | undefined>
  }
}
```

*Editics protocol changes*: Ignore this event and use a static configuration instead.

#### `openDocument` (client → server) & `documentOpen` (server → client)

Those events are about having the server hosting a file the client needs.

The usecases for this:

1. During the initial opening opening of the document: server sends a `documentOpen` event
  (after the `auth` event) containing the URL to the document (the server has typically
  downloaded the document from the 3rd party storage and converted it to the bin format).
2. To change the document configuration (e.g. set/change/remove document password,
  or re-open with a different CSV delimiter): client sends `openDocument` with
  the config changes, and server replies with a `documentOpen` so that the client
  can update.
3. To open a password protected document: server sends a `documentOpen` event with
  `"status": "needpassword"`, client respond with a `openDocument` containing the
  password and server finally respond with a `documentOpen` containing the URL of
  the document to download (i.e. back to case 1).
4. When inserting an image from a URL: client sends a `documentOpen` event with
   `"c": "imgurls"`, server download the image on its own then responds with a
   `openDocument` containing the URL (this one pointing on the server) to get the
   image.

Client event format:

```json5
{
  "type": "openDocument",
  "payload": {
    "type": "openDocument",
    // Dispatch key is `message.c`, which can be:
    // - "reopen": re-open with advanced open options (TXT codepage,
    //   CSV delimiter, or a DRM password).
    // - "setpassword": set/change/remove the document password.
    // - "changedocinfo": change the user's display name.
    // - "imgurls"/"pathurl"/"pathurls": resolve image/asset URLs.
    "message": <object>
  }
}
```

Server event format:

```json5
{
  "type": "documentOpen",
  "payload": {
    "type": "documentOpen",
    "data": {
      "type": <string>, // "reopen"/"imgurls"/etc.
      "status": "ok", // Also "ok" when the original URL couldn't be fetched...
      "data": {
        "error": <integer>,  // `0`: no error
        "urls": [
          {
            // URL pointing on the server to download the file
            "url": <string>,
            // Path of the file, e.g. "media/563ce5bc334c20e2db00d7c8337df009_image1.jpg"
            "path": <string>,
          }
        ]
      }
    }
  }
}
```

*Editics protocol changes*: Ignored since server shouldn't deal with unencrypted document content.

> [!NOTE]
> On client-side, we should nevertheless support the client `openDocument` event:
>
> - By having the client downloading the images by itself.
> - By disabling password protection in the editor (`protectionSupport: false`
>   in the `license` server event)

#### `clientLog` (client → server)

Telemetry info to provide the timing for the following events:

- `onDownloadFile`
- `onOpenDocument`
- `onLoadFonts`
- `onDocumentContentReady` (also provides the memory usage)
- `onApplyChanges`

Format:

```json5
{
  "type": "clientLog",
  "payload": {
    "type": "clientLog",
    "level": <string>,  // e.g. "debug"
    "msg": <string>  // e.g. "onDownloadFile time:168"
  }
}
```

*Editics protocol changes*: Ignored since it is only for telemetry purpose.

#### `extendSession` (client → server) & `session` (server → client)

The server periodically check each connection to a session:

- If the connection has been idle for 1 hour
- If the connection is older than 30 days

In both case, the server send a `session` event:

```json5
{
    "type": "session",
    "messages": {
    "code": <integer>,      // 4002 = idle, 4003 = absolute
    "reason": <string>,    // "idle session expires" | "absolute session expires"
    "interval": <number>   // present only for idle (the idle threshold in ms)
    }
}
```

*Editics protocol changes*: Ignored for now since those connection times are unlikely.

The client is then expected to send a `extendSession` event within 2 minutes,
otherwise the server closes the connection.

Format:

```json5
{
    "type": "extendSession",
    // For how long the client considers itself idle, the server uses this to
    // determine when it should send the next `session` event.
    "idletime": <integer>  // timestamp in ms
}
```

#### `refreshToken`/`expiredToken` (server → client)

Provide a new JWT token to the client.

> [!NOTE]
> The initial JWT token is provided by the server's `auth` event.

While the JWT token have a 30 days lifetime, this event is not automatically
triggered, instead it fires when:

- an editor is demoted to viewer (role change).
- the user correctly enters a document password (i.e. `documentOpen` event with type `setpassword`).
- the user's display name / connection info changes (i.e. `documentOpen` event with type `changedocinfo`).

Format:

```json5
{
  "type": "refreshToken",
  "payload": {
    "type": "refreshToken",
    "messages": <string>  // Freshly signed session JWT token
  }
}
```

`expiredToken` is never send by the OnlyOffice server (but still handled client side,
legacy code ?): when the token expires, the server sends a `disconnectReason` instead.

*Editics protocol changes*: Ignored since Parsec has its own authentication system the editics protocol relies on.

#### `forceSaveStart` (client → server) and `forceSaveStart`/`forceSave` (server → client)

OnlyOffice is designed to be integrated with a 3rd party service responsible
for the long term storage of the documents (e.g. OwnCloud).

In practice this means the server periodically exports the document and sends
it to the 3rd party service for it to save (see `documentFormatSave` field from
`auth` event that specify in which format the document should be exported).

However the end user is also able to force a save, which is done by sending this
event to the server.

client → server format:

```json5
{
    "type": "forceSaveStart",
}
```

The server then replies with:

```json5
{
    "type": "forceSaveStart",
    "messages": {
        "code": <integer>,
        "time": <integer|null>, // timestamp in ms
        "inProgress": <boolean|undefined>,
        "url": <string|null|undefined>
    }
}
```

Later (on save completion or timeout) the server sends:

```json5
{
    "type": "forceSave",
    "messages": {
        "type": <integer>,   // e.g. 2 = Timeout
        "time": <integer>, // timestamp in ms
        "inProgress": <boolean|undefined>,
        "url": <string|null|undefined>,
        "success": <boolean|undefined>,
        "start": <boolean|undefined>
    }
}
```

*Editics protocol changes*: Ignore those events since document saving is entirely done client-side.

#### `rpc` (client → server) & `rpc` (server → client)

Generic client → server request/response envelope.

Operations using RPC:

- `pathurls`: Resolve relative image paths embedded in a change to absolute storage URLs,
  so the receiver of a change can fetch the images it references.
  Triggered when a change includes new images that need URL resolution.
- `saveRelativeFromChanges`: Save using an already-existing relative changes file (a re-save path).
- `wopi_RefreshFile`: Ask the server to re-read the file from the WOPI host (e.g. after an external change).
- `wopi_RenameFile`: Ask the server to rename the file on the WOPI host.
- `sendForm`: Produce/print a filled form (PDF).

Format:

```json5
{
    "type": "rpc",
    "responseKey": <integer>,  // Monotonic counter
    "data": {
        "type": <string>, // e.g. "pathurls"
        ...
    }
}
```

 ```json5
{
    "type": "rpc",
    "responseKey": <integer>,
    "data": <result>
}
 ```

*Editics protocol changes*: Ignore those events as they provide too much cleartext data to the server.

#### `meta` (server → client)

Notify participants that document metadata has changed (currently: the
document title, after a WOPI rename). Broadcast to all participants of the
document.

Format:

```json5
{
  "type": "meta",
  "payload": {
    "type": "meta",
    // Metadata object. Currently produced fields:
    //   { title: <string> }: new document title (WOPI rename).
    "messages": <object>
  }
}
```

*Editics protocol changes*: Ignore as server don't know about file paths.

> [!NOTE]
> It might be interesting to support file renaming detection in the future, however
> this would be done on the client side (so the OnlyOffice editor would get a `meta`
> without any involvement from the server).

#### `disconnectReason` (server → client)

 Tell a client that the server is kicking it out for a specific operational reason:
 server shutdown, connection timeout, expired JWT token, or because the auth lock was handed to another user.

 Unlike drop (which is triggered by an external integrator command targeting specific users), disconnectReason is triggered by server-internal lifecycle/policy decisions and is sent to the specific connection(s) affected.

 Format:

 ```json5
   {
     "type": "disconnectReason",
     "payload": {
       "type": "disconnectReason",
       // 4001  SHUTDOWN_CODE         "server shutdown"
       // 4002  SESSION_IDLE_CODE     "idle session expires"
       // 4003  SESSION_ABSOLUTE_CODE "absolute session expires"
       // 4004  ACCESS_DENIED_CODE    "access deny"
       // 4006  JWT_ERROR_CODE        "token:" + <jwt error message>
       // 4007  DROP_CODE             "drop" (also used for auth-lock-taken)
       "code": <integer>,
       // Human-readable reason (e.g. "server shutdown", "idle session expires")
       "description": <string>
     }
   }
 ```

*Editics protocol changes*: Ignored since Parsec has its own logic (based on
HTTP status code) for handling this.

#### `documentOpen` (server → client)

Event send by the server when handling client `auth` event, this instruct the client
on how to load the document the session is editing.

```json5
{
  "type": "documentOpen",
  "payload": {
    "type": "documentOpen",
    "data": {
      // TODO: type can also be set to reopen|imgurls|pathurl|pathurls|setpassword|changedocinfo
      "type": "open",
      "status": "ok",
      "data": {
        <string>: <string> // {<Document name>: <URL to fetch document content>}
      },
      "openedAt": <integer> // timestamp in ms
    }
  }
}
```

TODO: this event is also used as response to `openDocument` event...

*Editics protocol changes*: Ignore this event since document opening is entirely done client-side.

#### `error` (server → client)

It carries an error identifier and code, and on the
client it is treated as a **hard disconnect** (it is routed to the same
handler as `drop`).

Legacy error event. The comment in the client dispatch labels it "Old SDK version",
it predates the structured `disconnectReason`/`warning` events (kept for backward
compatibility?).

Format:

```json5
{
  "type": "error",
  "payload": {
    "type": "error",
    "description": <string>,
    "code": <integer>
  }
}
```

*Editics protocol changes*: Ignore this event as it is a legacy one.

#### `updateVersion` (server → client)

Inform the client it should reloads its document since its in-memory is staled.

This is to handle a corner case:

1. Alice starts an edition session and makes some changes
2. Alice has its connection unexpectedly drop, there is now no other participants
   in the session.
3. The server triggers a save in the 3rd party storage.
4. Alice reconnects but is not aware that the saved document has changed.

Format:

```json5
{
    "type": "updateVersion",
    "success": <boolean>
}
```

*Editics protocol changes*: Ignore this event since the server is not able to save on its side.

## 3 - Protocol changes

Basically stack works as follow:

- Iframe dedicated to OnlyOffice that communicates with the main GUI
- main GUI uses libparsec to communicate with the server
- libparsec handles encryption/decryption (e.g. cursor, changes) and uses RCP & SSE with the server


### 3.1 - Join session SSE endpoint

On the server, the editics session needs to both keep track of the connected clients and push them messages (i.e. cursor move, document modification etc.).

For this we introduce a new authenticated SSE endpoint dedicated to joining a session.

Since the server has to keep track of who is connected to an editics session, we

> [NOTE]
>
> - This endpoint is similar to the already existing `GET /authenticated/{raw_organization_id}/events`.
> - We choose to introduce a new SSE endpoint instead of modifying `/authenticated/{raw_organization_id}/events`.
>   This is to simplify tracking who is connected to the session (and establishing a dedicated SSE connection for
>   each session maps this very well).

`GET /authenticated/{organization_id}/editics/{workspace_id}/{document_id}/{version}`

```json5
[
    {
        "major_versions": [
            5
        ],
        "cmd": "editics_join_session",
        // Request is never used as this API is only meant to be used from SSE
        "req": {
            "fields": []
        },
        "reps": [
            {
                "status": "ok",
                "unit": "EditicsEvent"
            },
            {
                // Returned if:
                // - The server has disabled editics support
                // - The command is used through the regular rpc route instead of the SSE one
                "status": "not_available"
            },
            {
                // To reconstruct a document we need two things:
                // - The document as stored in the workspace (so the content of a vlob at given version)
                // - Additional changes that have been done to the document in the edition session
                //
                // So obviously both has to match otherwise we would apply changes to an unrelated document.
                // For this reason only the initial vlob version and the versions that have been created by
                // saving this session are allowed to be used as starting document.
                "status": "cannot_bootstrap_from_version",
                "fields": [
                    {
                        // Note this version is not necessary *the* latest (i.e. a newer version might exist
                        // that results from some modification done outside of the edition session).
                        "name": "latest_allowed_version",
                        "type": "Version"
                    }
                ]
            },
            {
                "status": "author_not_allowed"
            },
            {
                "status": "realm_not_found"
            },
            {
                "status": "realm_archived"
            },
            {
                "status": "realm_deleted"
            }
        ],
        "nested_types": [
            {
                "name": "EditicsEvent",
                "discriminant_field": "event",
                "variants": [
                    {
                        // First event sent automatically by the server when the connection starts
                        "name": "bootstrap",
                        "discriminant_value": "BOOTSTRAP",
                        "fields": [
                            {
                                "name": "timestamp",
                                "type": "DateTime"
                            },
                            {
                                "name": "participants",
                                "type": "Dictionary<UUID, DeviceID>"
                            },
                            {
                                // TODO: what if there is *a lot* of changes ? should we stream them across multiple events ?
                                // TODO: provide also the change index, however we need to patch Cryptpad's OnlyOffice (see `onlyoffice-editor/src/index.ts:147`) to handle it
                                "name": "encrypted_changes",
                                // List of (key_index, encrypted_change)
                                "type": "List<(Index, Bytes)>"
                            }
                        ]

                    },
                    {
                        "name": "participants",
                        "discriminant_value": "PARTICIPANTS",
                        "fields": [
                            {
                                "name": "timestamp",
                                "type": "DateTime"
                            },
                            {
                                "name": "participants",
                                // Each member of the session is identified by an index corresponding to the order they
                                // have joined the session (this a strong requirement from the OnlyOffice client code that
                                // does arithmetic with this index so we cannot just use an UUID here).
                                // see: https://github.com/cryptpad/onlyoffice-editor/blob/c1be39bb0042d82c0f52d420e2d668f866458611/sdkjs/common/docscoapi.js#L1553
                                "type": "Dictionary<Integer, DeviceID>"
                            },
                            {
                                // There is two mode for document edition:
                                // - Exclusive: Used when there is a single client connected to the session
                                //   (hence no need for broadcasting every change).
                                // - Co-editing: Used when multiple clients are connected to the session.
                                //
                                // The tricky part is when a new client joins a session in exclusive mode: the initial
                                // client must switch the session to co-editing mode (this is done by sending an `unLockDocument`)
                                // and the new client must wait in the meantime.
                                // Hence this boolean field that indicates when this operation is done.
                                "name": "co_editing_ready",
                                "type": "Boolean"
                            }
                        ]
                    },
                    {
                        "name": "cursor",
                        "discriminant_value": "CURSOR",
                        "fields": [
                            {
                                // Key index identifies which key in the realm's keys bundle has
                                // been used to encrypt the cursor data.
                                "name": "key_index",
                                "type": "Index"
                            },
                            {
                                "name": "encrypted_cursor",
                                "type": "Bytes"
                            },
                            {
                                "name": "timestamp",
                                "type": "DateTime"
                            },
                            {
                                "name": "participant",
                                "type": "Integer"
                            }
                        ]
                    },
                    {
                        "name": "change",
                        "discriminant_value": "CHANGE",
                        "fields": [
                            {
                                // Key index identifies which key in the realm's keys bundle has
                                // been used to encrypt the changes.
                                "name": "key_index",
                                "type": "Index"
                            },
                            {
                                "name": "encrypted_changes",
                                "type": "List<(Index, Bytes)>"
                            },
                            {
                                "name": "timestamp",
                                "type": "DateTime"
                            },
                            {
                                "name": "participant",
                                "type": "Integer"
                            }
                        ]
                    }
                ]
            }
        ]
    }
]
```

### 3.2 - Move cursor

authenticated API:

```json5
[
    {
        "major_versions": [
            5
        ],
        "cmd": "editics_move_cursor",
        "req": {
            "fields": [
                {
                    "name": "realm_id",
                    "type": "VlobID"
                },
                {
                    "name": "document_id",
                    "type": "VlobID"
                },
                {
                    // Key index identifies which key in the realm's keys bundle has
                    // been used to encrypt the cursor data.
                    "name": "key_index",
                    "type": "Index"
                },
                {
                    // OnlyOffice opaque cursor data encrypted with the realm's latest key
                    "name": "encrypted_cursor",
                    "type": "Bytes"
                }
            ]
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        "name": "timestamp",
                        "type": "DateTime"
                    }
                ]
            },
            {
                "status": "author_not_allowed"
            },
            {
                // If the `key_index` in the certificate is not currently the realm's last
                "status": "bad_key_index",
                "fields": [
                    {
                        "name": "last_realm_certificate_timestamp",
                        "type": "DateTime"
                    }
                ]
            },
            {
                "status": "realm_not_found"
            },
            {
                "status": "realm_archived"
            },
            {
                "status": "realm_deleted"
            }
        ]
    }
]
```

### 3.3 - Edit document

authenticated API:

```json5
[
    {
        "major_versions": [
            5
        ],
        "cmd": "editics_change_lock",
        "req": {
            "fields": [
                {
                    "name": "realm_id",
                    "type": "VlobID"
                },
                {
                    "name": "document_id",
                    "type": "VlobID"
                },
                {
                    "name": "encrypted_cursor",
                    "type": "Bytes"
                }
            ]
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        "name": "timestamp",
                        "type": "DateTime"
                    }
                ]
            },
            {
                "status": "author_not_allowed"
            },
            {
                // If the `key_index` in the certificate is not currently the realm's last
                "status": "bad_key_index",
                "fields": [
                    {
                        "name": "last_realm_certificate_timestamp",
                        "type": "DateTime"
                    }
                ]
            },
            {
                "status": "realm_not_found"
            },
            {
                "status": "realm_archived"
            },
            {
                "status": "realm_deleted"
            }
        ]
    }
]
```

### 3.5 - Save document

authenticated API:


### 4 - Server-side implementation of session members tracking

Since multiple instance of the server can be running, the tracking of the clients connected to a session cannot be done purely in the server memory.

Instead the server relies on the PostgreSQL database to periodically store the list of its connections

 ```sql
 -- Session information are only kept for the duration of the session.
CREATE TABLE editics_session (
    _id SERIAL PRIMARY KEY,
    -- An editics session is always related to a given vlob
    vlob_id UUID NOT NULL,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    -- TODO: save lock logic should go here
);

-- Modifications on the document has to be kept for the duration of
-- the session so that other clients can join the session.
CREATE TABLE editics_session_patch (
    _id SERIAL PRIMARY KEY,
    -- An editics session is always related to a given vlob
    vlob_id UUID NOT NULL,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    blob BYTEA NOT NULL,
    -- Strictly growing
    index INTEGER NOT NULL,

    UNIQUE (realm, vlob_id, index)
);

-- Since the session only stores patches, the client has to obtain by
-- itself the initial document on which to apply those patches.
-- Hence those join points that are all the allowed version of the vlobs:
-- - The initial version that has been used when the session has been created.
-- - All the versions that have been created when the document has been saved
--   during this session edit.
CREATE TABLE editics_session_join_point (
    _id SERIAL PRIMARY KEY,
    editics_session INTEGER REFERENCES editics_session (_id) NOT NULL,
    vlob_atom INTEGER REFERENCES vlob_atom (_id) NOT NULL,
    -- All patches with index lower than this one should be ignored for this join point
    -- (i.e. those older patches are supposed to be already contained in the vlob atom to use)
    first_patch_index INTEGER NOT NULL,
):

CREATE TABLE editics_session_presence (
    _id SERIAL PRIMARY KEY,
    editics_session INTEGER REFERENCES editics_session (_id) NOT NULL,
    device INTEGER REFERENCES device (_id) NOT NULL,
    -- Periodically updated by the server owning the SSE connection that
    -- represent this presence
    last_seen   TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (editics_session_id, user_id)
);
 ```

### 5 - Per server session handling

TODO: document that an edition session only lives in a single server instance,
      which means a client has to be able to connect to a specific server instance
      to join a session.

- Single server handling the session means trivial ordering of events and monotonic timestamps
- return the list of editics servers with `server_config` command ?
- provide a dedicate `editics_join_session` command that return the URL of the editics server to join ?
- What sharding algorith to use ?
- use the hash of the configuration (e.g. URLs of each available editics server) to detect that the configuration hasn't changed ? (in which case the sharding algorithm could - instruct to use the wrong editics server)
- Use a background job on the server to save to PostgreSQL the non-euphemeral events ? (faster communication with the clients)
