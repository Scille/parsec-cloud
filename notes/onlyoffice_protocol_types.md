# OnlyOffice client/server wire protocol — field-level types

This extends `notes/communication_protocol.md` (which established *which* message types exist and
why) with the exact **field-level shape** of every message, typed precisely enough to drive the
Parsec protocol schemas in `notes/parsec_protocol_schemas.md`.

## Sources

1. **`notes/communication_protocol.md`** — the message vocabulary table and the two real bugs found
   while building `client/public/onlyoffice-mock-server.js` (participant `id` vs `idOriginal`, and
   `getLock`'s need for a direct reply to the requester). Read that first; this document doesn't repeat
   the reasoning, only the shapes.
2. **CryptPad's own server-side simulation**, `www/common/onlyoffice/inner.js` (found on this machine at
   `/home/agent/projects/cryptpad-saas-deploy/cryptpad-server/cryptpad/www/common/onlyoffice/inner.js`,
   4043 lines) — this is CryptPad's equivalent of `onlyoffice-mock-server.js`: a browser-side JS module
   that answers the OnlyOffice iframe's messages without ever hitting a real OnlyOffice server. Functions
   referenced below (`handleAuth`, `handleLock`, `handleChanges`, `parseChanges`, `getParticipants`,
   `getUserLock`) all live here.
3. **The actual client-side protocol engine**, vendored (unminified enough to read — `DocsCoApi`/
   `CDocsCoApi` classes keep their real names and mostly one-statement-per-line formatting despite being
   a production build) at
   `www/common/onlyoffice/dist/v9/sdkjs/word/sdk-all-min.js` in the same CryptPad checkout. This is the
   class that actually builds and parses every wire message — CryptPad's `inner.js` only reacts to
   whatever this class already decided to send, so where the two disagree on a field, **this is the
   ground truth**. Method names below (`DocsCoApi.prototype.X`) refer to this file. It's identical in
   spirit across `cell`, `slide`, `visio` builds; doc-type-specific differences (spreadsheet lock shape,
   `isExcel`) are called out explicitly where they occur.

Every message is a single flat JSON object with a `"type"` field selecting the shape (there is no
envelope/versioning). Directions: **C→S** (client sends via `DocsCoApi.prototype._send`, i.e. socket.io
`"message"` event), **S→C** (server pushes, arrives at `DocsCoApi.prototype._onServerMessage`).

## Shared nested shapes

### `Participant`

Sent inside `auth` (reply) and `connectState`. Consumed by `asc_CUser` (`_setUser`, same sdk-all-min.js):

| field | type | notes |
|---|---|---|
| `id` | `String` | Unique per **connection**, not per person. `asc_CUser` is keyed by this. See the "id vs idOriginal" bug in `communication_protocol.md`. |
| `idOriginal` | `String` | Unique per **person**. Drives the "who's editing" widget grouping/coloring (`getUserColorById(idOriginal, ...)`) and is what a `cursor` broadcast's `useridoriginal` is matched against. |
| `username` | `String` | Display name. |
| `indexUser` | `Integer` | Numeric seat index (0, 1, 2, …); the color/label picker also uses this as a fallback key. `-1` = read-only/unassigned (`READ_ONLY_INDEX_USER` in CryptPad). |
| `view` | `Boolean` | `true` = viewer/read-only participant. |
| `isLiveViewer` | `Boolean` | optional; read into `asc_CUser.live`. Not used by CryptPad's own participants (always constructs without it ⇒ `undefined`/falsy). |

CryptPad's `getParticipants()` additionally sends `connectionId` and `isCloseCoAuthoring` on each entry —
**dead fields**: `_setUser` above only reads `id`/`idOriginal`/`username`/`indexUser`/`view`/`isLiveViewer`,
so these two are ignored by the real client. Don't carry them into the Parsec schema.

### `LockEntry`

One granted/released lock, as stored server-side and pushed to clients (`getLock`/`releaseLock`/the
`locks` field on a relayed `saveChanges`):

| field | type | notes |
|---|---|---|
| `time` | `Integer` | epoch-ms when the lock was taken. |
| `user` | `String` | the **connection** id (`asc_CUser.id`-shaped) of the lock holder — compared against `this._userId` client-side to decide "is this my own lock". |
| `block` | opaque | see "Lock descriptors are doc-type-shaped" below. |
| `changes` | opaque, optional | only populated in the `releaseLock`/`saveChanges.locks` path (`_onReleaseLock`, `_onSaveChanges`); not populated by CryptPad's own simulation (always `undefined`). Unclear stock-server-only usage; safe to omit. |

### `ChangeEntry` (the wrapped form of a persisted change)

What every client's engine actually consumes, whether from the initial `authChanges` bulk dump or a
live `saveChanges` broadcast (`DocsCoApi.prototype._updateChanges`, reading `change["change"]`,
`change["user"]`, `change["useridoriginal"]`, `change["time"]`):

| field | type | notes |
|---|---|---|
| `docid` | `String` | CryptPad always uses the literal `"fresh"`; the real client never reads this field back (`_updateChanges` doesn't touch it) — vestigial, safe to drop. |
| `change` | `String` (default mode) or `Bytes` (binary mode) | the actual opaque op fragment. In default mode it's JSON-stringified *again* (`'"' + change + '"'` — a string containing a quoted string) — an artifact of `parseChanges` wrapping raw fragments that were already individually JSON-encoded by the sender; in binary mode (`binaryChanges: true`, see below) it's read directly as a `Uint8Array`. |
| `time` | `Integer` | epoch-ms. |
| `user` | `String` | connection id of the author (compared to `this._userId` to decide "is this my own change" for `lastOtherSaveTime` bookkeeping). |
| `useridoriginal` | `String` | person id of the author — this is the field surfaced to `onSaveChanges(changesOneUser, change["useridoriginal"], bFirstLoad)`, i.e. what actually attributes the edit to a user in the UI. |

## Messages, C→S (client → server)

### `auth`

Built by `DocsCoApi.prototype.getAuthCommand`. **Important finding: this message never actually leaves
the browser tab in CryptPad's integration.** `inner.js`'s dispatch switch has `case "auth": // Handled by
onlyoffice-editor now — break;` — i.e. CryptPad's own `www/common/onlyoffice/main.js`/embedding layer
intercepts `auth` before it reaches `inner.js`'s relay logic and answers it 100% locally and
synchronously (see `handleAuth`, which is *called directly*, not in response to a relayed message). Our
own `onlyoffice-mock-server.js` does the same (`onAuth()` is called directly by the CryptPad-style
wrapper, `auth` is logged but has an empty case in `onMessage`'s switch). **This has a direct
architectural consequence for step 3.3: `auth` doesn't need a Parsec RPC/SSE counterpart at all** — see
`notes/parsec_protocol_schemas.md`.

Full field list, for completeness (most are stock-OnlyOffice-server concepts — JWT auth, WOPI, licensing
— that don't apply once the message is handled locally; kept here so nothing is silently lost):

| field | type | Parsec-relevant? |
|---|---|---|
| `docid` | `String` | no — replaced by Parsec's own `realm_id`/`document_id`. |
| `documentCallbackUrl` | `String` | no — stock-server export callback, not used by CryptPad-style integrations. |
| `token` | `String` | no — JWT. |
| `user` | `{id, username, firstname, lastname, indexUser}` | no — Parsec already knows the caller's identity from the RPC/SSE auth headers. |
| `editorType` | `Integer` (`c_oEditorId`: `Word=0, Spreadsheet=1, Presentation=2, Visio=3`) | superseded by our own `OnlyOfficeDocumentType` enum (`client/src/services/onlyoffice.ts`), known before the editor even opens. |
| `lastOtherSaveTime` | `Integer` | no. |
| `block` | `Array` (locks the client believes it still owns, for re-auth after reconnect) | maybe, for reconnect UX — out of scope for the POC. |
| `sessionId` | `String \| null` | replaced by our own session/connection id (see SSE design doc). |
| `sessionTimeConnect` / `sessionTimeIdle` | `Integer` | no. |
| `documentFormatSave` | ? | no — export-format concept, irrelevant to Flow-way. |
| `isCloseCoAuthoring` | `Boolean` | no. |
| `openCmd` | `{url, ...}` | no — see `documentOpen` below; this is the client telling itself where its own configured `document.url` is. |
| `lang` | `String` | no — set from `editorConfig` at open time, already client-local. |
| `mode` | `Integer` | no — view/edit, already known from Parsec's own role check. |
| `permissions` | `Object` | no — same. |
| `encrypted` | `Boolean` | no — always true for us, not negotiated. |
| `IsAnonymousUser` | `Boolean` | no. |
| `timezoneOffset` | `Integer` | no. |
| `headingsColor` | ? | no. |
| `coEditingMode` | ? | no. |
| `jwtOpen` / `jwtSession` | `String` | no — JWT. |
| `time` | `Integer` | no. |
| `supportAuthChangesAck` | `Boolean` (always `true`) | no — see `authChangesAck` below. |

### `isSaveLock`

"Is anyone mid-checkpoint for this document right now?" (`DocsCoApi.prototype.askSaveChanges`)

| field | type |
|---|---|
| `syncChangesIndex` | `Integer` |

### `cursor`

`DocsCoApi.prototype.sendCursor`. Only sent if `cursor` is already a string (the engine's own internal
guard).

| field | type | notes |
|---|---|---|
| `cursor` | `String` | opaque OnlyOffice-internal encoding of a selection/position (e.g. `"14;BgAAADkAMAAxAAsAAAA="`) — never hand-craft, always relay verbatim. |

### `getLock`

`DocsCoApi.prototype.askLock`, sent as `{"type":"getLock","block":arrayBlockId}`.

| field | type | notes |
|---|---|---|
| `block` | `Array<opaque>` | **Lock descriptors are doc-type-shaped**, see below. |

#### Lock descriptors are doc-type-shaped

For `Word`/generic documents, each element of `block` (and the `block` field inside a `LockEntry`) is an
arbitrary opaque JSON value (commonly a paragraph/range id). For `Spreadsheet`/`Presentation`/`Visio` (i.e.
`this._isExcel || this._isPresentation || this._isPDF`), each element is instead an **object with a
required `guid` field** — the engine explicitly does `arrayBlockId[i]["guid"]` to compute its internal
lock key in that case. **The server must never interpret this value** — just store/relay whatever byte
shape it's given, keyed however the client-observable identity requires (see the map-vs-array note
under `LockEntry`'s container below).

### `unLockDocument`

`DocsCoApi.prototype.unLockDocument`.

| field | type | notes |
|---|---|---|
| `isSave` | `Boolean` | true when this unlock accompanies a save completion. |
| `unlock` | `Boolean` | = the engine's own `canUnlockDocument` flag. |
| `deleteIndex` | `Integer \| null` | |
| `releaseLocks` | `Boolean` | = `canReleaseLocks`. |

### `saveChanges`

**The one message that mutates the document.** `DocsCoApi.prototype.saveChanges`. Note there is **no
`changesIndex` field on the client's outbound message** — despite CryptPad's own `handleChanges` adding
one when it *relays* the change server-side (see `saveChanges` S→C below), the wire message the real
engine sends never includes it; ordering is entirely server-assigned.

| field | type | notes |
|---|---|---|
| `changes` | `String` (default) or raw binary array (binary mode) | JSON-stringified array of opaque op fragments in default mode; see "Binary changes mode" below. |
| `startSaveChanges` | `Boolean` | true iff this is the first chunk of a (possibly multi-part) save. |
| `endSaveChanges` | `Boolean` | true iff this is the last chunk — see "Large saves are chunked" below. |
| `isCoAuthoring` | `Boolean` | |
| `isExcel` | `Boolean` | |
| `deleteIndex` | `Integer \| null` | |
| `excelAdditionalInfo` | `String \| null` | JSON-stringified extra payload, spreadsheet-only. |
| `unlock` | `Boolean` | = `canUnlockDocument`. |
| `releaseLocks` | `Boolean` | = `canReleaseLocks`. |
| `reSave` | `Integer \| undefined` | set only when this save is a client-driven retry after a timeout (`1`) or after a failed ack (`2`); absent on a normal save. |

#### Binary changes mode

`DocsCoApi.prototype.setBinaryChanges(binaryChanges)` toggles `this.binaryChanges`; when `true`,
`saveChanges` sends `arrayChanges.slice(...)` (a raw array, msgpack-friendly) instead of
`JSON.stringify(arrayChanges.slice(...))`. On receipt, `_updateChanges` reads each entry's `change` as
`new Uint8Array(change["change"])` instead of `JSON.parse(change["change"])`. Set via
`editorConfig.settings.binaryChanges: true` per `CLAUDE.md` — **this is the mode Parsec should use**,
since msgpack (used for our RPC bodies) natively supports binary, avoiding the wasteful base64-in-JSON
default. CryptPad itself doesn't use this mode (its own vendored fork never toggles `binaryChanges`), so
this is a deliberate improvement over CryptPad's approach, not something to copy verbatim.

#### Large saves are chunked (`startSaveChanges`/`endSaveChanges`, `savePartChanges`)

`saveChanges` picks `endIndex` by accumulating fragment byte-lengths up to
`this.websocketMaxPayloadSize`; if the full `arrayChanges` doesn't fit in one message, it sends only
`[startIndex, endIndex)` with `endSaveChanges: false`, and the *reply* to that partial send
(`savePartChanges`, see below) triggers the client to call `saveChanges` again for the next slice
(`this.saveChanges(this.arrayChanges, this.currentIndexEnd)`), repeating until a chunk has
`endSaveChanges: true`. **This is a real protocol feature, not a CryptPad simplification** — CryptPad's
own `handleChanges` forwards `obj.startSaveChanges`/`obj.endSaveChanges` through unchanged and branches
on `!obj.endSaveChanges` to emit `savePartChanges` instead of `unSaveLock`. A from-scratch Parsec server
must implement this chunk/continue loop if it wants to support large documents/edits, even though the
POC-scale mock never exercises it.

### `forceSaveStart`

`DocsCoApi.prototype.forceSave`. Manual save trigger (Ctrl+S). No fields.

### `openDocument` (image URL lookup, `message.c === "imgurls"`)

`DocsCoApi.prototype.openDocument(data)` → `{"type":"openDocument","message":data}`. Used when
duplicating a slide/pasting content that references images by name; asks the server-side wrapper to
resolve those names/data-URIs to fetchable URLs.

| field | type |
|---|---|
| `message.c` | `String`, literal `"imgurls"` for this use case (the field is a generic RPC-style command selector shared with unrelated built-in commands — not otherwise used by CryptPad or by us). |
| `message.data` | `Array<String>` — image names, or `data:image/...` URIs for inline pasted images. |

### `clientLog`

`DocsCoApi.prototype.sendClientLog(level, msg)` → `{"type":"clientLog","level":level,"msg":msg}`.
Diagnostic/telemetry, fired by the engine's own error-detection heuristics (see e.g. the
`waitAuth`-consistency checks inside `_onConnectionStateChanged`/`_onStartCoAuthoring`, which call this
on internal inconsistency). Not in CryptPad's dispatch switch (falls through, ignored) and not needed by
ours either.

| field | type |
|---|---|
| `level` | `String` (e.g. `"error"`) |
| `msg` | `String` |

### `authChangesAck`

`DocsCoApi.prototype._onAuthChanges` sends this immediately upon receiving `authChanges` —
`{"type":"authChangesAck"}`, no payload. Purely an internal handshake step of the *stock* protocol
(hence `supportAuthChangesAck: true` advertised in `auth`); CryptPad ignores it entirely (not in the
dispatch switch) and so does our mock.

### Out of scope entirely (present in `DocsCoApi` but never used by CryptPad's integration or ours)

Observed in the class but with no corresponding handling anywhere in CryptPad's `inner.js`: `rpc`
(`callPRC`, a generic request/response escape hatch), `updateVersion`, `extendSession`, `close`,
`getMessages`/`message` (OnlyOffice's built-in chat — `sendMessage`/`getMessages`), and license
negotiation (a separate `type: "license"` message CryptPad's mock never sends; it inlines
`buildVersion`/`buildNumber`/`licenseType` directly into its `auth` reply instead, see below — a
simplification confirmed to work, worth keeping).

## Messages, S→C (server → client)

### `authChanges`

Bulk dump of every durable change accumulated so far, answering `auth` (see above: in CryptPad's
integration this is generated and consumed **entirely client-side**, never touching a real network
in the current architecture — but it's still a distinct logical message our SSE `join` bootstrap must
reproduce).

| field | type |
|---|---|
| `changes` | `Array<ChangeEntry>` |

Client-side, `_onAuthChanges` doesn't apply these changes immediately — it buffers them
(`this._authChanges.push(...)`) and only merges them via `_updateAuthChanges()` once the *next* `auth`
reply lands (interleaving late-arriving out-of-band changes correctly by index diff). This ordering
(`authChanges` always arrives, then `auth`, then `documentOpen`) must be preserved by any replacement
bootstrap sequence.

### `auth` (reply)

Answers a `handleAuth`-equivalent. Real field set per `_onAuth`, cross-checked with CryptPad's own
`handleAuth`/`connectMockServer.handleAuth`:

| field | type | notes |
|---|---|---|
| `result` | `Integer` | `1` = success; anything else is treated as "not yet authorized" (the engine just returns without erroring — no other documented value observed). |
| `sessionId` | `String` | CryptPad hardcodes `"session-id"`/`"sessionId"` — the real value is never actually read back for anything beyond `getSessionId()` (no protocol-level consequence to picking any unique string). |
| `participants` | `Array<Participant>` | |
| `locks` | `Array` | CryptPad always sends `[]`; real field would be pre-existing locks the newly-joined client should immediately respect. |
| `changes` | `Array<ChangeEntry>` | CryptPad always sends `[]` here — the actual durable dump goes through the separate `authChanges` message; this field is vestigial/legacy on the `auth` reply itself. |
| `changesIndex` | `Integer` | CryptPad hardcodes `0`. |
| `indexUser` | `Integer` | the caller's own seat index (from `getParticipants().index`). |
| `sessionTimeConnect` | `Integer` | not sent by CryptPad's simplified `handleAuth` (undefined-safe on the client). |
| `settings.reconnection.{attempts,delay}` | `Integer` | optional, stock reconnection tuning — not sent by CryptPad. |
| `settings.websocketMaxPayloadSize` | `Integer` | optional — governs the large-save chunk size described above; not sent by CryptPad (client falls back to its own default). |
| `g_cAscSpellCheckUrl` | `String` | optional, spellchecker endpoint — not sent by CryptPad (commented out in `handleAuth`). |
| `hasForgotten` | `Boolean` | optional. |
| `openedAt` | `Integer/String` | optional, surfaced via `onFirstLoadChangesEnd`. |
| `jwt` | `String` | optional session-refresh token — not applicable (no JWT). |
| `buildVersion` | `String` | CryptPad sends `"5.2.6"`. |
| `buildNumber` | `Integer` | CryptPad sends `2`. |
| `licenseType` | `Integer` | CryptPad sends `3`. |

`buildVersion`/`buildNumber`/`licenseType` are, in the *stock* protocol, actually part of a separate
`{"type":"license","license":{...}}` message (seen constructed in `_initSocksJs`, `var license = {...}`)
— CryptPad's inlining them into `auth` instead is a confirmed-working simplification, safe to reuse.

### `documentOpen`

Two distinct uses, both under the same `type`:

1. **Bootstrap** (third reply to `auth`, tells the client where to fetch the document body from):
   ```
   { "type": "documentOpen", "data": { "type": "open", "status": "ok", "data": { "<fileName>": "<url>" } } }
   ```
   `data.data` is a single-key object mapping the file name (e.g. `"Editor.bin"`) to a URL string. In our
   own step-1/2 integration this responsibility is already handled differently (content is fetched via
   Parsec's own `getFileContent` and handed to the editor directly) — see the architecture note in
   `notes/parsec_protocol_schemas.md` for why this sub-message doesn't need a Parsec wire counterpart.
2. **Image URL reply** (answers the `openDocument`/`imgurls` request above):
   ```
   { "type": "documentOpen", "data": { "type": "imgurls", "status": "ok", "data": { "urls": [...], "error": 0 } } }
   ```
   `urls` is `Array<String | {path: String, url: String}>` — a flat URL for a pasted `data:image/...`
   source, or a `{path, url}` pair for a named media reference.

### `connectState`

Pushed whenever the participant list changes (join/leave/role change). `_onConnectionStateChanged`:

| field | type | notes |
|---|---|---|
| `waitAuth` | `Boolean` | used only for the engine's own internal consistency logging (`sendClientLog` on mismatch) — not required for correct behavior; CryptPad always sends `false`. |
| `participantsTimestamp` | `Integer` | epoch-ms; the client only applies an update if `participantsTimestamp` is `>=` the last one seen — **must be monotonically non-decreasing** or updates get silently dropped. |
| `participants` | `Array<Participant>` | full list, not a diff. |

### `cursor`

| field | type |
|---|---|
| `messages` | `Array<{cursor: String, time: Integer, user: String, useridoriginal: String}>` |

`user`/`useridoriginal` follow the same connection-id/person-id split as `Participant.id`/`idOriginal` —
`useridoriginal` is what's checked against known participants to attach a color/label (see the real bug
writeup in `communication_protocol.md`).

### `getLock` (grant broadcast)

| field | type |
|---|---|
| `locks` | keyed collection of `LockEntry`, see below |

**Map vs. array**: `_onGetLock` client-side iterates with `for (key in data["locks"])`, which works
identically whether `locks` is a JS object (arbitrary string keys) or an array (numeric-index keys) —
this is why CryptPad's server-side `getUserLock()` can get away with returning an **array** for
spreadsheet documents and a **map** for everything else (`type === "sheet" ? array : map`) without the
client caring. **Recommendation for the Parsec schema: always use a map.** There's no protocol
requirement to preserve the array shape, and a single consistent shape avoids doc-type branching in the
server implementation. See `LockEntry` above for the value shape.

### `releaseLock`

| field | type |
|---|---|
| `locks` | same keyed-collection-of-`LockEntry` shape as `getLock`. |

### `saveChanges` (broadcast relay of someone's persisted change)

This is the *other* direction of the same `type` string used C→S — a different shape, matched by
`_onSaveChanges(data, useEncryption)`:

| field | type | notes |
|---|---|---|
| `changes` | `Array<ChangeEntry>` | the wrapped form — see `parseChanges`/`ChangeEntry` above. |
| `changesIndex` | `Integer` | server-assigned global sequence number for this batch. |
| `syncChangesIndex` | `Integer`, optional | |
| `startSaveChanges` / `endSaveChanges` | `Boolean` | **relayed from the originating client's own save** — the *receiving* client also reassembles multi-chunk saves via these two flags (buffers `changes` across messages with `endSaveChanges: false` until one arrives with `true`). |
| `locks` | keyed collection of `LockEntry`, optional | locks implicitly released by this save (`unlock`/`releaseLocks` on the originating `saveChanges`). |

### `unSaveLock`

Direct ack that a `saveChanges` was durably stored — unblocks the *sender's* further local edits.
`_onUnSaveLock`:

| field | type | notes |
|---|---|---|
| `index` | `Integer` | `-1` = sentinel/no-op (checked with `if (-1 !== data["index"])` before applying). |
| `time` | `Integer` | epoch-ms; `-1` = sentinel/no-op, same pattern. |
| `syncChangesIndex` | `Integer`, optional | `-1`/`undefined` = sentinel/no-op. |

### `savePartChanges`

Ack for a **non-final** chunk of a large save — tells the client to send the next chunk.
`_onSavePartChanges`:

| field | type |
|---|---|
| `changesIndex` | `Integer` (`-1` = sentinel/no-op) |
| `syncChangesIndex` | `Integer`, optional |

### `saveLock`

Reply to `isSaveLock`. `_onSaveLock`:

| field | type | notes |
|---|---|---|
| `saveLock` | `Boolean` | `true` = someone else is mid-checkpoint, client should back off. |
| `error` | any, optional | if present alongside a truthy `saveLock`, still unblocks local state (`_onSaveLock` treats `error` the same as `saveLock: true` for state-machine purposes). |

### `message`

Reply to `getMessages` (built-in chat, unused): `{"type":"message"}`. CryptPad always replies with an
empty message and never wires up the chat UI. Not needed for Parsec.

### `forceSaveStart` / `forceSave`

Server-side ack/progress for the Ctrl+S trigger. `_onForceSaveStart`/`_onForceSave`:

| field | type | notes |
|---|---|---|
| `code` | `Integer` (`c_oAscServerCommandErrors` enum: `NoError=0`, …) | |
| `time` | `Integer` | correlates a `forceSaveStart` ack back to the specific button-press that triggered it (`this._lastForceSaveButtonTime`). |
| `type` | `Integer` (`c_oAscForceSaveTypes` enum) | |
| `start` | `Boolean`, optional | |
| `success` | `Boolean`, optional | |

Out of scope for the POC's core save strategy (which is about durable persistence of `saveChanges`, not
this manual-trigger UX), but a natural hook point for whatever checkpoint/export mechanism the "Flow
way"/"Export way" decision in `notes/save_strategy.md` lands on.

## Summary: what a from-scratch server must actually implement

Cross-referencing the "out of scope" callouts above against the original vocabulary table in
`communication_protocol.md`, the **minimum viable vocabulary** for a real Parsec-backed server is:

- Bootstrap: `authChanges` + `auth` (reply) + `documentOpen` (open case) — collapsed into one SSE
  "session joined" event in the Parsec design, since `auth` (request) never has to leave the browser tab.
- Presence: `connectState`.
- Ephemeral coordination: `cursor`, `getLock`/`releaseLock` (+ their C→S request forms), `isSaveLock`/
  `saveLock`.
- The one durable mutation: `saveChanges` (both directions) + its acks `unSaveLock`/`savePartChanges`.
- Everything else (`clientLog`, `authChangesAck`, `openDocument`/imgurls, `forceSaveStart`, chat) is
  either silently droppable (matching CryptPad's own behavior) or explicitly out of scope for this POC.
