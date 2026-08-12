# Parsec/editics protocol — schema design (step 3.3/3.4)

Design for the Parsec-side reimplementation of the OnlyOffice server, following the vocabulary and
field-level shapes established in `notes/onlyoffice_protocol_types.md` (read that first) and the
message-classification/bug findings in `notes/communication_protocol.md`.

Two wire primitives, per `CLAUDE.md`:

- **One RPC command**, `editics_session_do`, used for every C→S message (`cursor`, `getLock`,
  `saveChanges`, …).
- **One SSE endpoint**, `GET /authenticated/{organization_id}/editics/{workspace_id}/{document_id}/join`,
  used for every S→C push (`connectState`, `getLock` grants, `saveChanges` broadcasts, …).

Schemas are written in the same JSON5 dialect as `libparsec/crates/protocol/schema/authenticated_cmds/`
(see `events_listen.json5` for the SSE-nested-type convention and `vlob_update.json5`/
`cryptpad_register_session.json5` for the encrypted-payload/`key_index` convention this reuses).

## Architecture decisions

### 1. `auth` never needs a wire counterpart

Per `onlyoffice_protocol_types.md`, CryptPad's own integration answers the OO client's `auth` message
**entirely locally**, without ever putting it on the wire — everything it carries (user identity, doc
type, view/edit mode, locale) is already known to the embedding page before the editor even starts.
Parsec is in the same position: the GUI already knows the caller's identity, the workspace role (hence
view/edit mode), the document type, and the locale, exactly as today's `FileEditor.vue`/`onlyoffice.ts`
construct `OpenDocumentOptions` without any network round-trip (step 1/2, already implemented). So the
"session bootstrap" the Parsec server actually needs to provide is narrower than the full stock `auth`
handshake: just the *durable state* the browser tab can't already know — the participant list and the
change log accumulated so far. That's exactly what `SessionJoined` (below) carries, delivered as the
first SSE event on the `join` connection instead of as a reply to an `auth` RPC.

Likewise, `documentOpen`'s "bootstrap" use (telling the client where to download the doc body from,
see `onlyoffice_protocol_types.md`) isn't needed either: the GUI already fetches the file's content via
the regular `vlob_read_batch`/`getFileContent` path *before* calling `openDocument()` (unchanged from
step 1/2). The `join` SSE connection is opened only once that content is already in hand — it exists
purely to bootstrap the *collaboration* state (who else is here, what's been edited since), not the
document body itself.

### 2. Self-broadcast eliminates the two documented real bugs

`communication_protocol.md` found two bugs while building the mock, both the same root cause: a
"broadcast to everyone" primitive (`BroadcastChannel`) that, by spec, never delivers back to its own
sender, so the sender needs a separate explicit self-reply for any message that isn't purely
fire-and-forget (`getLock`'s grant, `saveChanges`'s `unSaveLock`).

**A real SSE connection doesn't have this asymmetry**: the sender's own `join` stream is just another
subscriber. So the design here is: `editics_session_do` **never carries the OnlyOffice-level reply
in its HTTP response** — the HTTP response is a thin `ok`/error ack that the request was accepted (needed
because SSE has no per-request error channel). The actual reply data (`getLock` grant, `saveLock`,
`unSaveLock`/`savePartChanges`, `connectState`, the relayed `saveChanges`) is **always** delivered via
the `join` SSE stream, fanned out to every connected participant **including the requester**. This
removes the entire "does this message type need an explicit self-delivery path" judgment call the mock
had to make per-message-type — every message answers the same way, always.

Trade-off: the requester now waits for its own SSE round-trip instead of getting an immediate synchronous
reply, adding one hop of latency. Acceptable for a collaborative editor already built around eventual
convergence (see `communication_protocol.md`'s "Ordering" section — there's no synchronous
request/response guarantee in the underlying protocol design anyway, only "the server picks *an* order").

### 3. Locks always use the map shape

Per `onlyoffice_protocol_types.md`, the real client's `_onGetLock` iterates locks with `for (key in ...)`,
which is agnostic to array-vs-map — CryptPad only used an array for spreadsheets as an implementation
detail. The schema here always uses `Map<String, LockEntry>`, avoiding doc-type branching server-side.

### 4. Binary changes, always

`editorConfig.settings.binaryChanges: true` (per `CLAUDE.md`) is assumed throughout: `saveChanges`'
content is `Bytes`, not a JSON string. This also determines the encryption boundary — see below.

### 5. Presence doesn't need a heartbeat/staleness hack

`onlyoffice-mock-server.js` needed a 5s heartbeat + 20s staleness prune because `localStorage` +
`BroadcastChannel` has no notion of "this tab is gone". A real SSE connection's lifecycle *is* that
signal: when a participant's `join` stream closes (tab closed, network drop, navigation away), the
server observes the disconnect directly and broadcasts an updated `ParticipantsChanged` to whoever's
left. No polling, no TTL.

### 6. What gets encrypted

Per `CLAUDE.md`: message type stays in clear (needed for server-side classification/access control),
only actual document content is encrypted, using the workspace's latest key with its `key_index`
attached (same convention as `vlob_update`/`cryptpad_register_session`). The only field carrying
document content is `saveChanges`' `changes` — and, since it can also carry cell data,
`excelAdditionalInfo` — everything else (cursor position, lock descriptors, participant names) is
either already known to the server (names) or too structurally shallow to bother encrypting (an opaque
lock/cursor position leaks far less than the document text itself, and — unlike `saveChanges` — isn't
durably persisted).

## New identifier type

`EditicsConnectionID` — a fresh UUID-backed newtype (same style as `GreetingAttemptID`,
`SequesterServiceID`, etc. — would need to be defined in `libparsec_types` alongside those, out of scope
for this schema doc), minted by the server when a `join` SSE connection is established. Maps directly
onto the OnlyOffice protocol's *connection-scoped* identity (`Participant.id` / `ChangeEntry.user` /
`cursor.messages[].user` in `onlyoffice_protocol_types.md`) — as opposed to `UserID`, which maps onto
the *person-scoped* identity (`Participant.idOriginal` / `useridoriginal`). Minting a fresh id per
connection (rather than reusing `DeviceID`) is deliberate: it correctly gives two tabs on the *same*
device two distinct rows in the participant list, which a `DeviceID`-keyed identity would collide on —
the exact class of bug the "id vs idOriginal" writeup in `communication_protocol.md` warns about.

`document_id` and `realm_id` reuse `VlobID` throughout, following `cryptpad_register_session.json5`'s
precedent (the target file's own vlob id identifies the editing session 1:1).

## The SSE endpoint: `GET /authenticated/{organization_id}/editics/{workspace_id}/{document_id}/join`

Modeled on `authenticated_cmds/events_listen.json5`'s nested-discriminated-union convention, but bound
to its own dedicated route (not the generic `/authenticated/{organization_id}/events`) since it's scoped
to one workspace+document and needs its own connection-lifecycle (participant join/leave) semantics that
the generic event stream doesn't have. `workspace_id`/`document_id` come from the URL path, exactly like
`{raw_organization_id}` does for the existing routes in `server/parsec/asgi/rpc.py`; the schema's `req`
is consequently empty, same as `events_listen`. Reuses the existing authenticated+SSE handshake
(`with_sse_headers=True`, `Accept: text/event-stream`) and the existing `Last-Event-ID` reconnect
mechanism (`parsed.last_event_id` in `rpc.py`) for **reconnects mid-session** — distinct from
`SessionJoined.durable_changes`, which handles a **first-ever** join needing the full history. Each
`EditicsEvent` should get an SSE `id:` equal to its `change_index` (for `SaveChanges`) or a per-event
counter (for everything else), so a reconnecting client's `Last-Event-ID` correctly resumes without
replaying the whole log.

```json5
[
    {
        "major_versions": [5],
        "cmd": "editics_session_join",
        "introduced_in": "5.7",
        "req": {
            // workspace_id and document_id come from the URL path
            // (GET /authenticated/{organization_id}/editics/{workspace_id}/{document_id}/join),
            // not from a request body -- this is a GET/SSE route, same as `events_listen`.
            "fields": []
        },
        "reps": [
            {
                "status": "ok",
                "unit": "EditicsEvent"
            },
            {
                // Returned if the command is used through the regular rpc route instead of the SSE one
                "status": "not_available"
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
            },
            {
                // `document_id` doesn't (yet) exist as a vlob in the realm. Allowed to still open a
                // session for it -- mirrors `cryptpad_register_session`'s handling of a not-yet-
                // synchronized document -- but the server can't return `SessionJoined.durable_changes`
                // seeded from a file it doesn't know, so it just starts an empty session.
                "status": "document_not_found"
            }
        ],
        "nested_types": [
            {
                "name": "EditicsEvent",
                "discriminant_field": "kind",
                "variants": [
                    {
                        // Always the first event sent on a freshly-opened `join` connection.
                        // Replaces the OnlyOffice `authChanges` + `auth` (reply) + `documentOpen`
                        // (bootstrap) sequence -- see "auth never needs a wire counterpart" above.
                        "name": "SessionJoined",
                        "discriminant_value": "SESSION_JOINED",
                        "fields": [
                            {
                                // This connection's own freshly-minted identity. Used as `connection_id`
                                // in every subsequent `editics_session_do` call on this connection.
                                "name": "connection_id",
                                "type": "EditicsConnectionID"
                            },
                            {
                                // This participant's own seat index, mirrors OnlyOffice's `indexUser`.
                                "name": "index_user",
                                "type": "Integer"
                            },
                            {
                                "name": "participants",
                                "type": "List<Participant>"
                            },
                            {
                                // The full durable change log accumulated so far for this document,
                                // in application order. Empty for a brand new/never-edited document.
                                // Mirrors OnlyOffice's `authChanges`.
                                "name": "durable_changes",
                                "type": "List<PersistedChange>"
                            }
                        ]
                    },
                    {
                        // Mirrors OnlyOffice's `connectState`. Full list, not a diff -- simpler for
                        // clients and matches what the OO engine already expects to receive.
                        "name": "ParticipantsChanged",
                        "discriminant_value": "PARTICIPANTS_CHANGED",
                        "fields": [
                            {
                                "name": "participants",
                                "type": "List<Participant>"
                            }
                        ]
                    },
                    {
                        // Ephemeral, never persisted. Mirrors OnlyOffice's `cursor` broadcast.
                        "name": "Cursor",
                        "discriminant_value": "CURSOR",
                        "fields": [
                            {
                                "name": "from",
                                "type": "EditicsConnectionID"
                            },
                            {
                                "name": "user",
                                "type": "UserID"
                            },
                            {
                                // Opaque OnlyOffice-internal position encoding, relayed verbatim.
                                "name": "cursor",
                                "type": "String"
                            }
                        ]
                    },
                    {
                        // Reply to a `GetLock` request AND the self-delivery of the requester's own
                        // grant -- see "self-broadcast eliminates the two documented real bugs" above.
                        "name": "LockAcquired",
                        "discriminant_value": "LOCK_ACQUIRED",
                        "fields": [
                            {
                                "name": "locks",
                                "type": "Map<String, LockEntry>"
                            }
                        ]
                    },
                    {
                        "name": "LockReleased",
                        "discriminant_value": "LOCK_RELEASED",
                        "fields": [
                            {
                                "name": "locks",
                                "type": "Map<String, LockEntry>"
                            }
                        ]
                    },
                    {
                        // Whether a checkpoint/save is currently in progress for this document.
                        // Reply to `IsSaveLock`.
                        "name": "SaveLockStatus",
                        "discriminant_value": "SAVE_LOCK_STATUS",
                        "fields": [
                            {
                                "name": "save_lock",
                                "type": "Boolean"
                            }
                        ]
                    },
                    {
                        // The one durable mutation. Delivered to every participant, including the
                        // author -- see design decision #2. Reassembly of large/chunked saves
                        // (`start_save_changes`/`end_save_changes`) is the receiving client's own
                        // responsibility, same as stock OnlyOffice -- see
                        // notes/onlyoffice_protocol_types.md, "Large saves are chunked".
                        "name": "SaveChanges",
                        "discriminant_value": "SAVE_CHANGES",
                        "fields": [
                            {
                                // Server-assigned, strictly increasing per document. Doubles as the
                                // SSE event id for `Last-Event-ID` resumption.
                                "name": "change_index",
                                "type": "Integer"
                            },
                            {
                                "name": "author",
                                "type": "UserID"
                            },
                            {
                                "name": "timestamp",
                                "type": "DateTime"
                            },
                            {
                                "name": "start_save_changes",
                                "type": "Boolean"
                            },
                            {
                                "name": "end_save_changes",
                                "type": "Boolean"
                            },
                            {
                                // The key index used to encrypt `encrypted_changes` (and
                                // `encrypted_excel_additional_info`, if present). Provided so a client
                                // catching up on history (`SessionJoined.durable_changes`) can still
                                // decrypt entries written under an older (but not-yet-rotated-out) key.
                                "name": "key_index",
                                "type": "Index"
                            },
                            {
                                "name": "encrypted_changes",
                                "type": "Bytes"
                            },
                            {
                                "name": "encrypted_excel_additional_info",
                                "type": "RequiredOption<Bytes>"
                            },
                            {
                                // Locks implicitly released by this save.
                                "name": "locks",
                                "type": "RequiredOption<Map<String, LockEntry>>"
                            }
                        ]
                    },
                    {
                        // Informational only, a hook for the eventual checkpoint/export mechanism
                        // (see notes/save_strategy.md) -- out of scope for this POC's core save path.
                        "name": "ForceSaveStart",
                        "discriminant_value": "FORCE_SAVE_START",
                        "fields": [
                            {
                                "name": "requested_by",
                                "type": "UserID"
                            }
                        ]
                    }
                ]
            },
            {
                "name": "Participant",
                "fields": [
                    {
                        "name": "id",
                        "type": "EditicsConnectionID"
                    },
                    {
                        "name": "id_original",
                        "type": "UserID"
                    },
                    {
                        "name": "username",
                        "type": "String"
                    },
                    {
                        "name": "index_user",
                        "type": "Integer"
                    },
                    {
                        // View-only (READER role) vs. edit-capable participant.
                        "name": "view",
                        "type": "Boolean"
                    }
                ]
            },
            {
                "name": "LockEntry",
                "fields": [
                    {
                        "name": "time",
                        "type": "DateTime"
                    },
                    {
                        "name": "user",
                        "type": "EditicsConnectionID"
                    },
                    {
                        // Opaque, doc-type-shaped lock descriptor -- see
                        // notes/onlyoffice_protocol_types.md, "Lock descriptors are doc-type-shaped".
                        // JSON-serialized by the client before sending, relayed verbatim.
                        "name": "block",
                        "type": "String"
                    }
                ]
            },
            {
                // The durable-history counterpart of the `SaveChanges` event above, used in
                // `SessionJoined.durable_changes`. Same fields except no `locks` (irrelevant once
                // already merged into history) and no doc-author-mismatch corner case.
                "name": "PersistedChange",
                "fields": [
                    {
                        "name": "change_index",
                        "type": "Integer"
                    },
                    {
                        "name": "author",
                        "type": "UserID"
                    },
                    {
                        "name": "timestamp",
                        "type": "DateTime"
                    },
                    {
                        "name": "key_index",
                        "type": "Index"
                    },
                    {
                        "name": "encrypted_changes",
                        "type": "Bytes"
                    },
                    {
                        "name": "encrypted_excel_additional_info",
                        "type": "RequiredOption<Bytes>"
                    }
                ]
            }
        ]
    }
]
```

## The RPC command: `editics_session_do`

Every C→S message goes through this one command. `connection_id` (obtained from `SessionJoined`) scopes
the request to a specific workspace/document/participant, so — unlike `editics_session_join` — no
separate `workspace_id`/`document_id` fields are needed on the request; the server looks up the
session's realm/document from `connection_id` itself, which also avoids a client being able to claim a
`saveChanges` applies to a document it never actually joined.

`auth`, `authChangesAck`, `clientLog`, and the built-in chat (`getMessages`/`message`) are intentionally
**not** represented here — see `onlyoffice_protocol_types.md`'s "out of scope" sections for why each is
either handled entirely client-side or silently droppable. `openDocument` (imgurls) is included as a
placeholder variant returning `not_available`, since it's a real (if rare) protocol need not yet
designed.

```json5
[
    {
        "major_versions": [5],
        "cmd": "editics_session_do",
        "introduced_in": "5.7",
        "req": {
            "fields": [
                {
                    // From `EditicsEvent::SessionJoined.connection_id` on this client's own `join`
                    // SSE connection.
                    "name": "connection_id",
                    "type": "EditicsConnectionID"
                },
                {
                    "name": "message",
                    "type": "EditicsClientMessage"
                }
            ]
        },
        "reps": [
            {
                // Thin ack only -- the actual OnlyOffice-level reply always arrives over the
                // `join` SSE stream, fanned out to every participant including the sender.
                // See "self-broadcast eliminates the two documented real bugs" above.
                "status": "ok"
            },
            {
                // `connection_id` doesn't match a currently-open `join` connection (never joined,
                // already disconnected, or belongs to a different document).
                "status": "connection_not_found"
            },
            {
                // Caller's role on the workspace doesn't allow this message (e.g. a READER sending
                // `GetLock`/`SaveChanges`/`UnLockDocument`/`ForceSaveStart` -- `Cursor` and
                // `IsSaveLock` remain allowed for view-only participants).
                "status": "author_not_allowed"
            },
            {
                "status": "realm_archived"
            },
            {
                "status": "realm_deleted"
            },
            {
                // Only relevant for `SaveChanges`: the `key_index` used doesn't match the realm's
                // current latest key. Mirrors `vlob_update`'s status of the same name.
                "status": "bad_key_index",
                "fields": [
                    {
                        "name": "last_realm_certificate_timestamp",
                        "type": "DateTime"
                    }
                ]
            }
        ],
        "nested_types": [
            {
                "name": "EditicsClientMessage",
                "discriminant_field": "kind",
                "variants": [
                    {
                        "name": "Cursor",
                        "discriminant_value": "CURSOR",
                        "fields": [
                            {
                                "name": "cursor",
                                "type": "String"
                            }
                        ]
                    },
                    {
                        "name": "GetLock",
                        "discriminant_value": "GET_LOCK",
                        "fields": [
                            {
                                // Each element JSON-serialized client-side, see LockEntry.block.
                                "name": "block",
                                "type": "List<String>"
                            }
                        ]
                    },
                    {
                        "name": "UnLockDocument",
                        "discriminant_value": "UN_LOCK_DOCUMENT",
                        "fields": [
                            {
                                "name": "is_save",
                                "type": "Boolean"
                            },
                            {
                                "name": "release_locks",
                                "type": "Boolean"
                            }
                        ]
                    },
                    {
                        "name": "IsSaveLock",
                        "discriminant_value": "IS_SAVE_LOCK",
                        "fields": []
                    },
                    {
                        // See "Large saves are chunked" in notes/onlyoffice_protocol_types.md: a
                        // client sends one `SaveChanges` per chunk; the server acks each one via
                        // `SaveChanges`/`SaveLockStatus`-equivalent SSE events, same as stock
                        // OnlyOffice's `savePartChanges`/`unSaveLock` pair.
                        "name": "SaveChanges",
                        "discriminant_value": "SAVE_CHANGES",
                        "fields": [
                            {
                                "name": "key_index",
                                "type": "Index"
                            },
                            {
                                "name": "encrypted_changes",
                                "type": "Bytes"
                            },
                            {
                                "name": "encrypted_excel_additional_info",
                                "type": "RequiredOption<Bytes>"
                            },
                            {
                                "name": "start_save_changes",
                                "type": "Boolean"
                            },
                            {
                                "name": "end_save_changes",
                                "type": "Boolean"
                            },
                            {
                                "name": "release_locks",
                                "type": "Boolean"
                            }
                        ]
                    },
                    {
                        "name": "ForceSaveStart",
                        "discriminant_value": "FORCE_SAVE_START",
                        "fields": []
                    },
                    {
                        // Placeholder -- see the doc comment above the `nested_types` block.
                        // Always answered with `not_available` for now.
                        "name": "OpenDocumentImageUrls",
                        "discriminant_value": "OPEN_DOCUMENT_IMAGE_URLS",
                        "fields": [
                            {
                                "name": "image_refs",
                                "type": "List<String>"
                            }
                        ]
                    }
                ]
            }
        ]
    }
]
```

## Fields intentionally dropped from the OnlyOffice shapes

Cross-referencing `onlyoffice_protocol_types.md` field-by-field, the following are deliberately *not*
represented anywhere above, with the reason:

| OnlyOffice field | Why dropped |
|---|---|
| `Participant.connectionId`, `.isCloseCoAuthoring` | dead fields — the real client (`asc_CUser._setUser`) never reads them. |
| `LockEntry.changes` | never populated by CryptPad's own simulation; unclear stock-only usage, safe to omit for the POC. |
| `connectState.waitAuth` | only feeds the OO engine's own internal consistency logging, no functional effect — the GUI-side shim can hardcode `false` when forwarding `ParticipantsChanged` into the iframe. |
| `saveChanges.deleteIndex`, `.isCoAuthoring`, `.isExcel`, `.unlock`, `.reSave` | these steer the *client engine's own* local bookkeeping (co-authoring mode, undo/redo indexing, retry classification) rather than carrying information the server needs to act on; the GUI-side shim reconstructs them locally from `EditicsEvent`/context when handing a reply back to the iframe, the same way it already reconstructs `docid`/`change` wrapping today in `onlyoffice-mock-server.js`'s `_parseChanges`. |
| `auth`'s JWT/WOPI/licensing fields (`token`, `jwtOpen`, `jwtSession`, `documentCallbackUrl`, `wopiSrc`, `shardKey`, `userSessionId`, `sessionTimeIdle`, `headingsColor`, `coEditingMode`) | not applicable — see architecture decision #1, `auth` never reaches the server at all. |
| `auth` reply's `settings.reconnection`, `settings.websocketMaxPayloadSize`, `g_cAscSpellCheckUrl`, `hasForgotten`, `openedAt`, `jwt`, `locks`, `changes`/`changesIndex` | stock-server niceties CryptPad's own simplified `handleAuth` already confirmed are safe to omit (see `onlyoffice_protocol_types.md`'s `auth` (reply) table). |
| `documentOpen` (bootstrap case) | superseded entirely — the document body is fetched via `vlob_read_batch` before `join` is even opened. See architecture decision #1. |
| `getMessages`/`message` (chat), `clientLog`, `authChangesAck`, `rpc`, `updateVersion`, `extendSession`, `close`, `license` | out of scope, see `onlyoffice_protocol_types.md`'s "out of scope entirely" section. |

## Open questions for step 4

- Where `PersistedChange`/`SaveChanges` events actually get stored (a dedicated append-only table keyed
  by `document_id`, vs. reusing the vlob/version mechanism some other way) is the "Flow way" question from
  `notes/save_strategy.md` — this schema is agnostic to that choice; it only fixes the *wire* shape.
- Checkpoint/snapshot policy (`communication_protocol.md`'s "Snapshots / checkpoints" section) — when a
  server-side snapshot trims `SessionJoined.durable_changes`, and how `ForceSaveStart` ties into it —
  is deliberately left as a hook (`SaveLockStatus`, `ForceSaveStart`) rather than designed here.
