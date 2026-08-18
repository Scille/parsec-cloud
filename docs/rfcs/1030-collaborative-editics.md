<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Collaborative editics

## 1 - Overview

This RFC introduce a mechanism to be able to edit documents (i.e. docx/xlsx/pptx/etc. ) from within the Parsec GUI.

This edition system (called "editics") works in two modes:

- Offline mode: the edition is done purely in local.
- Online mode: the Parsec server hosts an editics collaborative session on which all clients
  looking to view/edit the document connect to.

### 1.1 - Offline vs Online modes

Online mode should be considered the default one (i.e. the client tries to use the online mode and fall back in offline if it is not possible).

Note Offline mode is used in two cases:

- The client is offline (i.e. Parsec server cannot be reached)
- If the document being edited doesn't have a collaborative session running and the user
  trying to open it doesn't have write access to the workspace containing the document.
  This is because a user without write access is not allowed to send modification in the
  collaborative session, and hence cannot send the initial state of the document.

TODO: reader CAN start online session since the initial document is never stored in the session (each joining client generates it independently).

### 1.2 - Use of OnlyOffice

OnlyOffice is used to implement the actual document edition, however its integration into Parsec is special since we are in an end-to-end encrypted system:

- Document conversion (done with the x2t tool) must run client side
- OnlyOffice server cannot be used (since document modifications flow trough it in clear text)

For this reason we use the OnlyOffice patched by Cryptpad to support end-to-end encryption:

- [onlyoffice-editor](https://github.com/cryptpad/onlyoffice-editor) has been patched to use a fake server class
  (that can be used to encrypt content before sending to an arbitrary server) instead of directly sending request
  to the OnlyOffice server.
- [x2t](https://github.com/cryptpad/onlyoffice-x2t-wasm) has been patched to compile in WASM in order to run
  client-side in a web browser.

Finally we need to re-implement in the Parsec server the communication system used to exchange messages between the
clients connected to a same editics collaborative session (e.g. to modify the document, update client's cursor location, etc.).

### 1.3 - Session ID vs document ID

Each OnlyOffice session is identified by an ID that is used by the clients to join the session.
We use the couple (workspace ID + document vlob ID) as the session ID.
This means:

- A document cannot change its vlob ID (i.e. the document is saved by creating new versions of this vlob)
- A document must exist in the workspace before editing it in OnlyOffice.
- Document path in the workspace is only used to resolve the vlob ID, which is done *before* OnlyOffice is involved.
  So OnlyOffice has no concept of document path and the document name is only given as informative (it may become
  inaccurate in the session if the document is moved/renamed in the workspace).
- A document can only have at most one session at a given time. This is important as it prevent most concurrency modifications.
- Concurrency modifications are still possible if the document is directly modified in the workspace (e.g. document modified
  from the workspace mountpoint).
- Since the editics session is considered the main way of editing a document and doesn't cause save conflicts by itself (i.e.
  the session act as a single source of truth), it is allowed to save to the workspace by just uploading a new vlob version
  without checking if its modifications conflicts with the last existing version[^1].

[^1]: While this means the conflict is silently resolved, the end user can still access the overwritten version through the history if needed.

### 1.4 - Document load/modify/save lifecycle

Let's consider a workspace containing a document `/foo.docx` with vlob ID 42, Alice (OWNER) Bob (CONTRIBUTOR)
and Mallory (READER) have access to the workspace and want to access the document.

1. Alice wants to edit document `/foo.docx`.
    1.1. Alice's client resolves `/foo.docx` path and obtains vlob ID 42 with latest version 10.
    1.2. Alice's client loads in the editics editor the content of vlob ID 42 at version 10.
    1.3. Alice's client connects to the editics session with ID 42 and specify she uses version 10.
         The session doesn't exist on the server, it is created.
2. Alice modifies the document twice. Her client sends two modification message to the editics session.
   Each modification is given an index ID by the server to ensure they are ordered (so index 1 and 2).
3. Bob wants to join the session.
    3.1. Same as step 1.1. but for Bob
    3.2. Same as step 1.2. but for Bob
    3.3. Bob's client connects to the editics session with ID 42 and specify he uses version 10.
        The server accept the connection and pushes to the client all the modifications that occurred since the session was created.
4. Alice wants to save the document
    4.1. Alice's client takes a save lock on the session (ensure no other clients tries a concurrent save).
    4.2. Alice's client exports the document from the editics editor and save it as vlob ID 42 version 11.
    4.3. Alice's client release the save lock on the session and indicates that all modifications up to index 2 are contained in vlob ID 42 version 11.
5. Mallory wants to join the session
    5.1. Same as step 1.1. but for Mallory
    5.2. Mallory's client loads in the editics editor the content of vlob ID 42 at version 11.
    5.3. Mallory's client connects to the editics session with ID 42 and specify she uses version 11.
        The server accept the connection and pushes no modification.

The server ensures the client can reach the correct document state by only allowing certain vlob versions:

- Version must be at least the initial version used when the session has been created.
- A version more recent that what the session may indicates two situations:

  - The document is currently being saved and the vlob has been modified but the save lock not yet released (this is an unlikely situation).
  - The document has been currently modified outside of the session, hence there is no guarantees on what the document contains at this version.

When rejecting the client, the server provides the latest allowed version so that the client can retry.

The fact each client has to load the document by itself (i.e. the server doesn't provide to the client the document edited in the session but only patches to apply)
is more performance-hungry on the client, but this prevents the client that created the session from controlling the initial content of the document (this is
an issue since a user with no write access in the workspace can then modify a document by creating a session with his tempered document, then wait for a
user with write access to join and do the save for them...).

## 2 - The OnlyOffice communication protocol

| type | direction | purpose |
| --- | --- | --- |
| `auth` | client → server | handshake: "I want to open document X as user Y" |
| `authChanges` | server → client | reply #1 to `auth`: the durable changes accumulated since the session/document was created (see "joining an existing session" below) |
| `auth` (reply) | server → client | reply #2 to `auth`: session bootstrap — participant list, `sessionId`, build version, etc. |
| `documentOpen` | server → client | reply #3 to `auth`: where to download the document from |
| `authChangesAck` | client → server | **not in CryptPad's code** — client-side ack that it processed `authChanges`. Observed live; CryptPad silently ignores it (falls through its `switch` with no `case`), and so do we. |
| `clientLog` | client → server | **not in CryptPad's code either** — diagnostic/telemetry messages the client fires on startup. Observed live (multiple per session), ignored by both CryptPad and our mock. |
| `connectState` | server → client | participant list changed (join/leave) |
| `cursor` | both | cursor/selection position — sent by whoever moved it, echoed to everyone else |
| `getLock` | client → server, echoed back | "I'm about to edit this paragraph/cell, reserve it" |
| `getLock` (broadcast) | server → client | tells every client (including the requester) which region is now locked |
| `unLockDocument` / `releaseLock` | both | release of the above |
| `isSaveLock` | client → server | "is anyone mid-checkpoint right now?" |
| `saveLock` | server → client | reply to the above (`true`/`false`) |
| `saveChanges` | client → server | **the actual edit** — see below |
| `savePartChanges` | server → client | ack when a save was too big and got split into multiple `saveChanges` calls |
| `unSaveLock` | server → client | ack that a `saveChanges` was durably stored; unblocks further local edits |
| `forceSaveStart` | client → server | manual save trigger (Ctrl+S) — a signal, carries no content of its own |
| `getMessages` / `message` | client → server / reply | OnlyOffice's built-in chat; unused here |
| `openDocument` (`message.c === "imgurls"`) | client → server | asks for the URLs of images referenced by pasted/duplicated content |


Events recognized by the OnlyOffice server (cf. [EuroOffice source code](https://github.com/Euro-Office/server/blob/c438fd3d336497b2acf7bfe6b4f9ee4fde1fcfbf/DocService/sources/DocsCoServer.js#L1904-L1990))
TODO
| `auth` |
| `message` |
| `cursor` |
| `getLock` |
| `saveChanges` |
| `isSaveLock` |
| `unSaveLock` |
| `getMessages` |
| `unLockDocument` |
| `close` |
| `openDocument` |
| `clientLog` |
| `extendSession` |
| `forceSaveStart` |
| `rpc` |
| `authChangesAck` |

### 2.1 - Examples of typical messages

- Create a new session
  - `connectState`
  - `auth`
  - `authChangesAck`
- Join an existing session
  - `connectState`
  - `auth`
  - `authChangesAck`
  - `unLockDocument`
  - `releaseLock`
  - `cursor`
  - `releaseLock`
  - `cursor`
- 


### The `saveChanges` wire format

`saveChanges` contains a `changes` field that is an array of modifications.
By default this is represented as a JSON-serialized array
of raw op fragments (an OnlyOffice-internal binary/JSON change format we never need to understand the
internals of — we just relay it opaquely).

A typical example of `changes` field in default format: `"[\"64;AgAAADEA//8BACxLuimoIAIApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///w4AAAAwAC4AMAAuADAALgAwAA==\",\"37;CAAAADAAXwAyADQAAQAcAAEAAAAFAAAAAQAAAGkAAAAAAwAAAA==\"]"`

Default format is wasteful however so we want to use instead the binary format (since the actual communication with the Parsec server is going to be done in msgpack that supports binary data).

Switching to binary format can be done by setting `editorConfig.settings.binaryChanges` to `true` in `window.DocsAPI.DocEditor`'s config parameter.

### End-to-end encryption

Not all messages as to be end-to-end encrypted. As a matter, all messages should have their command name in clear text (so that the server can do access control, know if they are ephemeral etc.) and only a small subset of the message content should be encrypted.
For instance: the `changes` field in `saveChanges` should be encrypted (since it contains the actual content of the document).

Regarding how encryption is actually handled, the workspace encryption system will be used:

- The workspace's last key is used to encrypt.
- The key index is provided along with the encrypted payload (so a data encrypted with an old key can still be decrypted).
- The server reject the request if it doesn't use the lastest key index (this handles key rotation in case a user no longer as access to the workspace).

## 3 - Protocol changes

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

`GET /authenticated/{organization_id}/editics/{workspace_id}/{document_id}`

```json
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
                // Returned if the command is used through the regular rpc route
                // instead of the SSE one
                "status": "not_available"
            }
        ],
        "nested_types": [
            {
                "name": "EditicsEvent",
                "discriminant_field": "event",
                "variants": [
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
                                "type": "Dictionary<UUID, DeviceID>"
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
                                "type": "UUID"
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
                                "type": "List<Bytes>"
                            },
                            {
                                "name": "timestamp",
                                "type": "DateTime"
                            },
                            {
                                "name": "participant",
                                "type": "UUID"
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
