<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Collaborative editics

## 1 - Overview

This RFC introduce a mechanism to be able to edit documents (i.e. docx/xlsx/pptx/etc. )
from within the Parsec GUI.

This edition system (called "editics") works in two modes:

- Offline mode: the edition is done purely in local.
- Online mode: the Parsec server hosts an editics collaborative session on which
  all clients looking to view/edit the document connect to.

### 1.1 - Offline vs Online modes

Online mode should be considered the default one (i.e. the client tries to use the
online mode and fall back in offline if it is not possible).

Note Offline mode is used in two cases:

- The client is offline (i.e. Parsec server cannot be reached)
- If the document being edited doesn't have a collaborative session running and the user
  trying to open it doesn't have write access to the workspace containing the document.
  This is because a user without write access is not allowed to send modification in the
  collaborative session, and hence cannot send the initial state of the document.

TODO: reader CAN start online session since the initial document is never stored in the
session (each joining client generates it independently).

### 1.2 - Use of OnlyOffice

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

Finally we need to re-implement in the Parsec server the communication system used to
exchange events between the clients connected to a same editics collaborative session
(e.g. to modify the document, update client's cursor location, etc.).

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

### 1.6 - Document load/modify/save lifecycle

Let's consider a workspace containing a document `/foo.docx` with vlob ID 42, Alice (OWNER),
Bob (CONTRIBUTOR) and Mallory (READER) have access to the workspace and want to access the document.

1. Alice wants to edit document `/foo.docx`.
    1.1. Alice's client resolves `/foo.docx` path and obtains vlob ID 42 with latest version 10.
    1.2. Alice's client loads in the editics editor the content of vlob ID 42 at version 10.
    1.3. Alice's client connects to the editics session with ID 42 and specify she uses version 10.
         The session doesn't exist on the server, it is created.
2. Alice modifies the document twice. Her client sends two modification events to the editics session.
   Each modification is given an index ID by the server to ensure they are ordered (so index 1 and 2).
3. Bob wants to join the session.
    3.1. Same as step 1.1. but for Bob
    3.2. Same as step 1.2. but for Bob
    3.3. Bob's client connects to the editics session with ID 42 and specify he uses version 10.
        The server accept the connection and pushes to the client all the modifications
        that occurred since the session was created.
4. Alice wants to save the document
    4.1. Alice's client takes a save lock on the session (ensure no other clients tries a concurrent save).
    4.2. Alice's client exports the document from the editics editor and save it as vlob ID 42 version 11.
    4.3. Alice's client release the save lock on the session and indicates that all
         modifications up to index 2 are contained in vlob ID 42 version 11.
5. Mallory wants to join the session
    5.1. Same as step 1.1. but for Mallory
    5.2. Mallory's client loads in the editics editor the content of vlob ID 42 at version 11.
    5.3. Mallory's client connects to the editics session with ID 42 and specify she uses version 11.
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
is more performance-hungry on the client, but this prevents the client that created
the session from controlling the initial content of the document (this is an issue
since a user with no write access in the workspace can then modify a document by
creating a session with his tempered document, then wait for a user with write
access to join and do the save for them...).

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

### 2.2 - Events

list of all OnlyOffice events:

| Event                | Origin | Kept in editics protocol |
|----------------------|--------|--------------------------|
| `auth`               | client |            TODO          |
| `message`            | client |            ✅            |
| `cursor`             | client |            ✅            |
| `getLock`            | client |            ✅            |
| `saveChanges`        | client |            ✅            |
| `isSaveLock`         | client |            ✅            |
| `unSaveLock`         | client |            ✅            |
| `getMessages`        | client |            ❌            |
| `unLockDocument`     | client |            ✅            |
| `close`              | client |            ✅            |
| `openDocument`       | client |            ❌            |
| `clientLog`          | client |            ❌            |
| `extendSession`      | client |            ❌            |
| `forceSaveStart`     | client |            ❌            |
| `rpc`                | client |            ❌            |
| `authChangesAck`     | client |            TODO          |
| `auth`               | server |            TODO          |
| `message`            | server |            ✅            |
| `cursor`             | server |            ✅            |
| `meta`               | server |            TODO          |
| `getLock`            | server |            ✅            |
| `releaseLock`        | server |            ✅            |
| `connectState`       | server |            TODO          |
| `saveChanges`        | server |            ✅            |
| `authChanges`        | server |            TODO          |
| `saveLock`           | server |            ✅            |
| `unSaveLock`         | server |            TODO          |
| `savePartChanges`    | server |            TODO          |
| `drop`               | server |            TODO          |
| `disconnectReason`   | server |            TODO          |
| `waitAuth`           | server |            TODO          |
| `error`              | server |            TODO          |
| `documentOpen`       | server |            ❌            |
| `warning`            | server |            ✅            |
| `license`            | server |            ❌            |
| `session`            | server |            ❌            |
| `refreshToken`       | server |            ❌            |
| `expiredToken`       | server |            ❌            |
| `forceSaveStart`     | server |            ❌            |
| `forceSave`          | server |            ❌            |
| `rpc`                | server |            ❌            |
| `updateVersion`      | server |            ❌            |

Detail for each command:

#### `auth` (client → server)

Handshake: "I want to open document X as user Y"

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

TODO: do we need it ?

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

Ask the server to obtain (i.e. the server sends a `getLock` event) the list of
all [region locks](#21---session-locks) currently held.

This is event is rarely used since region locks are optimisticly taken (i.e.
editing → isSaveLock → saveChanges with no getLock ever sent).

The operations that actually require a server lock are the "heavy" structural ones
(e.g. inserting/editing images, tables, headers/footers, document settings etc.).

```json5
{
    "type": "getLock"
}
```

*Editics protocol changes*:

- Rename to `list_region_locks`

Then the server sends:

```json5
{
    "type": "getLock",
    "locks": [
        {
            TODO
        }
    ]
}
```

*Editics protocol changes*:

- Rename to `list_region_locks_rep`

#### `saveChanges` (client → server)

Send some modification in the document, this requires to have the lock on the
document first.

Format:

```json5
{
  "type": "saveChanges",
  "payload": {
    "type": "saveChanges",
    // String of JSON serialized array of changes,
    // each change itself being an opaque string in OnlyOffice internal format
    "changes": <string>,
    "startSaveChanges": <boolean>,  // TODO: document
    "endSaveChanges": <boolean>,  // TODO: document
    "isCoAuthoring": <boolean>,  // TODO: document
    "isExcel": <boolean>,  // TODO: document
    "deleteIndex": <null|integer>,  // TODO: document
    "excelAdditionalInfo": <string>,  // Opaque string in OnlyOffice internal format
    "unlock": <boolean>,  // TODO: document
    "releaseLocks": <boolean>  // TODO: document
  }
}
```

> [!NOTE]
> *The `saveChanges` wire format*
>
> `saveChanges` contains a `changes` field that is an array of modifications.
> By default this is represented as a JSON-serialized array of raw op fragments
> (an OnlyOffice-internal binary/JSON change format we never need to understand).
>
> A typical example of `changes` field in default format:
> `"[\"64;AgAAADEA//8BACxLuimoIAIApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///w4AAAAwAC4AMAAuADAALgAwAA==\",\"37;> CAAAADAAXwAyADQAAQAcAAEAAAAFAAAAAQAAAGkAAAAAAwAAAA==\"]"`
>
> Default format is wasteful however so we want to use instead the binary format
> (since the actual communication with the Parsec server is going to be done in
> msgpack that supports binary data).
>
> Switching to binary format can be done by setting `editorConfig.settings.binaryChanges`
> to `true` > in `window.DocsAPI.DocEditor`'s config parameter.

*Editics protocol changes*:

- Rename to `save_changes`.
- Encrypt `changes` and `excelAdditionalInfo` fields.
- Switch to binary format for `changes` field.

#### `isSaveLock` (client → server)

Try to take the lock to save the document. This leads to the server sending a
`saveLock` event.

Format:

```json5
{
  "type": "isSaveLock",
  "payload": {
    "type": "isSaveLock",
    "syncChangesIndex": <integer> // TODO: document
  }
}
```

*Editics protocol changes*:

- Rename to `is_save_lock`.

#### `unSaveLock` (client → server)

This is used be the client to cancel a save operation.
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

#### `unLockDocument` (client → server)

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

*Editics protocol changes*:

- Rename to `un_lock_document`

#### `close` (client → server)

Format:

```json5
```

*Editics protocol changes*:

#### `openDocument` (client → server)

: Ignored since the client deals alone with document opening due to e2e encryption

Format:

```json5
```

*Editics protocol changes*:

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

#### `extendSession` (client → server) & `session`/`refreshToken`/`expiredToken` (server → client)

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

Format:

```json5
{
    "type": "extendSession",
    "idletime": <integer>  // timestamp in ms
}
```

*Editics protocol changes*: Ignored for now since those connection times are unlikely.

TODO: what to do with refreshToken/expiredToken

The client is then expected to send a `extendSession` event within 2 minutes,
otherwise the server closes the connection.

Ask the server to provide a new JWT token (i.e. server sends `refreshToken` event)
to stay authenticated.

If `extendSession` is not used, the server eventually sends a `expiredToken`
event to the client to inform it that its JWT token has expired.

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

#### `authChangesAck` (client → server)

Format:

```json5
```

*Editics protocol changes*:

#### `auth` (server → client)

Format:

```json5
```

*Editics protocol changes*:

#### `meta` (server → client)

Format:

```json5
```

*Editics protocol changes*:

#### `releaseLock` (server → client)

Format:

```json5
```

*Editics protocol changes*:

#### `connectState` (server → client)

Format:

```json5
```

*Editics protocol changes*:

#### `saveChanges` (server → client)

Format:

```json5
```

*Editics protocol changes*:

#### `authChanges` (server → client)

Format:

```json5
```

*Editics protocol changes*:

#### `saveLock` (server → client)

Format:

```json5
```

*Editics protocol changes*:

#### `unSaveLock` (server → client)

```json5
{
  "type": "unSaveLock",
  "payload": {
    "type": "unSaveLock",
    "index": <integer>,
    "time": <integer>,  // timestamp in ms
    "syncChangesIndex": <integer>
  }
}
```

#### `savePartChanges` (server → client)

Format:

```json5
```

*Editics protocol changes*:

#### `drop` (server → client)

Format:

```json5
```

*Editics protocol changes*:

#### `disconnectReason` (server → client)

Format:

```json5
```

*Editics protocol changes*:

#### `waitAuth` (server → client)

Format:

```json5
```

*Editics protocol changes*:

#### `error` (server → client)

Format:

```json5
```

*Editics protocol changes*:

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

TODO: document

Format:

```json5
{
  "type": "license",
  "payload": {
    "type": "license",
    "license": {
      "type": <integer>,
      "light": <boolean>,
      "mode": <integer>,
      "rights": <integer>,
      "buildVersion": <string>,  // e.g. "9.4.1"
      "buildNumber": <integer>,
      "protectionSupport": <boolean>,
      "isAnonymousSupport": <boolean>,
      "liveViewerSupport": <boolean>,
      "branding": <boolean>,
      "customization": <boolean>,
      "advancedApi": <boolean>
    }
  }
}
```

*Editics protocol changes*: Ignore this event and use a static configuration instead.

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
