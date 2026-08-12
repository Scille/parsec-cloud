# OnlyOffice client/server protocol — investigation (step 3.2)

Goal: figure out what the OnlyOffice client actually sends/expects from "the server" before designing
Parsec's own protocol in step 3.3. Two sources were used:

1. **Reading CryptPad's own source** (`cryptpad/cryptpad` on GitHub, `www/common/onlyoffice/{inner,main}.js`).
   CryptPad is the only other project that re-implements the OnlyOffice server-side protocol for an
   end-to-end-encrypted context (the same reason we can't use OnlyOffice's own server), so its code is
   effectively a worked answer key.
2. **Actually running it**: `client/public/onlyoffice-mock-server.js` (new) replaces the do-nothing
   `connectMockServer` stub from step 1/2 with a real (if minimal) mock "server" — implemented purely
   with `localStorage` + `BroadcastChannel` (same-origin, zero network) — plus an on-page debug panel
   that logs every message crossing the client/server boundary. Two browser tabs opening the same file
   join the same simulated session, so genuine multi-client traffic (not hand-waved) could be observed.

## The message vocabulary

The OnlyOffice client (`sdkjs`/`web-apps`) talks to "the server" (normally socket.io, here the mock) in
one `type`-tagged JSON message at a time. There is no discovery mechanism — the vocabulary is small and
fixed. Every message CryptPad's own switch statement handles, and everything actually observed from the
mock:

| type | direction | purpose |
|---|---|---|
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

### Is there a flag that says "this message is ephemeral"? — No, and you don't need one

There is no explicit flag. But the vocabulary above is small and closed, and **exactly one type carries an
actual document mutation that has to survive a page reload or a new client joining**: `saveChanges` (its
`changes` field — see format below). Everything else is session bootstrap, presence, lock coordination, or
an acknowledgement: safe to lose if the mock/server restarts, without corrupting the document. So the rule
implemented in `onlyoffice-mock-server.js` (`classify()`) is simply:

```js
type === 'saveChanges' ? 'persistent' : 'ephemeral'
```

This was verified live: the debug panel tags every message, and only `saveChanges` ever shows the red
`[persistent]` tag (see screenshots taken during testing — typing "Hello world" produced exactly one
`isSaveLock → saveLock → saveChanges[persistent] → unSaveLock` sequence).

### The `saveChanges` wire format

`msg.changes` as sent by the client is a JSON **string** encoding an array of raw op fragments (an
OnlyOffice-internal binary/JSON change format we never need to understand the internals of — we just
relay it opaquely). Critically, what goes back out to *any* OnlyOffice engine (the sender itself, echoed
back through the ordered log, or another client) is **not** that raw string — it must be wrapped, exactly
as CryptPad's `parseChanges()` does:

```js
JSON.parse(rawChangesJson).map(change => ({
  docid: 'fresh',
  change: '"' + change + '"',
  time: <submission time>,
  user: <submitting user's id>,
  useridoriginal: <submitting user's id>,
}))
```

`onlyoffice-mock-server.js` (`_parseChanges`) does this once, at submission time, and stores/relays the
wrapped form from then on — matching where CryptPad does the same conversion (client-side, before handing
off to its realtime layer).

## Joining a new vs. an existing session

There's no separate "does a session exist" message. The `auth` handshake always gets an `authChanges`
reply containing every non-ephemeral change accumulated so far (empty array for a brand new document).
This is why the CLAUDE.md architecture point (*"How the client detects a session already exists, checked
before reading the document from storage"*) matters: in CryptPad's model, "does a session exist" and "does
the document have prior edits" are the same question, answered by the *same* piece of storage — whatever
holds the append-only change log. A server implementation should check "is there a log for this
document?" before paying for a separate document-storage read, not the other way around.

In the mock, `getInitialChanges()` reads the full accumulated log (`localStorage`, keyed by
`documentId`) and flattens every entry's `changes` into the `authChanges` reply — exactly mirroring
CryptPad's `ooChannel.queue` flush. **This was verified with two real browser tabs**: tab 1 (Alice) typed
"Hello world" then hit "Replay last edit as Bob" (2 persisted changes total); tab 2 (Bob), opening the
same document fresh, rendered "Hello worldHello world" immediately on load — i.e. joining replayed the
whole history correctly, without re-downloading/re-converting the source file.

## Snapshots / checkpoints

CryptPad periodically (`CHECKPOINT_INTERVAL = 100` patches, or forced every `FORCE_CHECKPOINT_INTERVAL =
10000`) has one client export the current document, upload it as a new base snapshot, and — implicitly —
lets clients joining afterward start from that snapshot instead of replaying the entire history. This
is *not* a distinct OnlyOffice message type: it's purely a server/storage-side optimization sitting
"underneath" the same `auth` → `authChanges` flow (a smarter `getInitialChanges()` would return "snapshot
URL + changes since the snapshot" instead of "changes since document creation"). The mock doesn't
implement real snapshotting (out of scope until the save strategy from step 3.1 is settled — snapshotting
*is* the export/flow-way question), but logs a visible marker every `CHECKPOINT_WARN_INTERVAL` (20, a
POC-scale stand-in for 100) persisted changes, so the trigger condition is at least visible.

## Ordering

There's no explicit sequence number the client picks — `changesIndex` on an outgoing `saveChanges` is the
client's *local* view of how many changes have been applied so far, not a claim about serverside order.
The server is the single source of truth: it appends whatever `saveChanges` it receives to one ordered,
append-only log per document (`_appendLog` — `entry.seq = log.length`) and rebroadcasts the wrapped
`changes` **plus its own `changesIndex: entry.seq`** to every connected client, unconditionally, including
back to the original sender. There's no rejection/merge logic: OnlyOffice's own co-authoring engine is
expected to reconcile concurrent edits given the (server-assigned) global order — the server's job is
purely "pick *an* order, and it's the same order for everyone." This matches CryptPad's `EV_OO_EVENT`
`MESSAGE` handler, which does exactly `ooChannel.send(obj.data.msg)` for every message regardless of
sender.

Locks (`getLock`/`releaseLock`) are the concurrency-avoidance mechanism *underneath* that: they don't
change how ordering works, they just make simultaneous edits to the same region rare in practice.

## Participant identity: `id` vs `idOriginal` (a real bug found and fixed)

Every participant descriptor (in the `auth` reply and in `connectState`) has *two* identity fields, and
getting this wrong silently breaks both the "who's editing" widget and foreign cursors — with no error,
so it's easy to miss:

- `id` — unique per **connection**. The client's participant map (`asc_CUser`, in the vendored
  `sdkjs/word/sdk-all-min.js`) is keyed by this field, so it must be unique per browser
  tab/session, not per person.
- `idOriginal` — unique per **person**. This is what the "Users who are editing the file" widget
  actually groups/colors by (`getUserColorById(idOriginal, ...)`), and what a foreign cursor's `user`/
  `useridoriginal` fields are checked against to find a known participant to attach a color/label to.

The first version of `onlyoffice-mock-server.js` only ever sent `id` (reusing the Parsec user id for
real participants, a fake string like `fake-bob-1` for simulated ones) and never sent `idOriginal` at
all. Every participant therefore had `idOriginal === undefined`, and the client's grouping logic (which
keys on `idOriginal`) treated *every* participant — Alice and any simulated "Bob" alike — as the same
person with multiple connections: the "Users editing" widget showed a single merged row
("Alicey McAliceFace **(2)**") instead of two separate ones, and a cursor sent for an "unknown" identity
was silently dropped rather than rendered. This confirms the exact failure mode: **not** a message-delivery
problem (traced and confirmed messages reach `Common.Gateway` → `DocsCoApi._onServerMessage` →
`_onConnectionStateChanged`/`_onCursor` correctly in every case) but an **identity** one — the client
does track connected users, precisely, but only by `idOriginal`.

Fix: `getParticipants()` now sends `id` = the connection id (the tab's own random `clientId` for real
participants; the simulated user's own id for fakes, since a simulated "join" is a single connection) and
`idOriginal` = the actual person id (Parsec user id, or the simulated user id again). Cursor messages
were adjusted the same way (`useridoriginal` must carry the *person* id, not the connection id). With that
fixed, "Simulate user joining" produces a correctly separate row in the widget, and "Simulate cursor move"
renders a distinctly colored, correctly labelled foreign cursor (verified visually: a second, differently
colored caret appears at the simulated position while the real local caret is elsewhere).

One more wrinkle specific to `cursor`: the position value itself (e.g. `"14;BgAAADkAMAAxAAsAAAA="`) is
an opaque OnlyOffice-internal encoding, same as `saveChanges`' `changes` payload — not something to
hand-craft. The "Simulate cursor move" button therefore reuses the local session's own last *real*
captured cursor value (mirroring "Replay last edit as Bob" for `saveChanges`) rather than a fabricated
placeholder string, which is why it needed the local session to have moved its own cursor/typed at least
once first.

## `getLock` needs a direct reply to the requester, not just a broadcast (a real bug found and fixed)

`saveChanges` has a dedicated ack (`unSaveLock`) that CryptPad sends straight back to the submitting
client, separately from the ordered broadcast that (eventually) reaches everyone including the sender —
this is documented above under "Ordering". `getLock` has **no separate ack message**: the grant *is*
the ack, and OnlyOffice's client blocks further local edits in that region until it receives one back.

The mock's `_broadcast()` is a thin wrapper over `BroadcastChannel.postMessage()`, and **`BroadcastChannel`
never delivers a message back to the tab that sent it** (this is standard, spec'd behavior, not a bug in
the browser). The first version of the `getLock`/`unLockDocument` handlers only ever called `_broadcast()`
— so another tab would correctly receive the grant, but the *requesting* tab never got its own answer.
Once the client started requesting locks at all (empirically, this only happens once it believes it's in
a real multi-participant session — a lone editor never sent a single `getLock` in testing, only after
"Simulate user joining" registered a second participant), every subsequent edit attempt would silently
hang waiting for a grant that was never coming, until whatever internal timeout OnlyOffice uses gave up
— matching exactly the reported symptom ("Alice has to wait ... before being able to edit again").

Fix: `getLock` and `unLockDocument` now call `_sendToClient(...)` directly (an immediate self-reply, like
`saveChanges`'s `unSaveLock`) *in addition to* `_broadcast(...)` for other tabs. Also fixed while at it:
the grant's `locks` field must be an id-keyed **map** of `{time, user, block}` entries (per CryptPad's own
`getLock()`), not the raw `block` array a *request* carries — the previous code relayed the request's
array back as if it were already in reply shape.

General lesson for 3.3: **any message type that isn't purely "broadcast and forget" needs its own explicit
delivery path back to the sender** - a same-origin `BroadcastChannel`, and by extension a real server's
"broadcast to everyone in the channel" primitive, is not guaranteed (and here, is guaranteed *not*) to
loop back to its own publisher. A real server has to decide per-message-type whether the sender needs a
distinct ack, mirroring what CryptPad does for `saveChanges` (ack) vs. what it *doesn't* do for `getLock`
(the grant flowing back to the sender through the *same* ordered broadcast a real multi-client server
would provide, which this synchronous two-call mock approximates).

## An OnlyOffice client-side edge case: a foreign cursor at your *exact* position blocks local typing

Reported: after "Simulate user joining" + "Simulate cursor move", Alice becomes unable to type - *only*
when the simulated cursor lands on her own exact current character offset (confirmed: moving her own
cursor even one character away fixes it immediately; the "fix" that was found by trial and error was
switching the browser tab away and back *and* moving the cursor - switching tabs alone wasn't enough).

The critical diagnostic: while stuck, **the debug panel shows zero new log entries** for the keystrokes
being typed. Nothing was hanging waiting for a reply inside our mock (that would show up as, e.g., an
`isSaveLock` with no `saveLock` back) - the vendored OnlyOffice engine (`sdkjs`) isn't even attempting to
send anything. Whatever gate this is lives entirely client-side, upstream of `onMessage`/`connectMockServer`,
inside heavily-minified code we don't control (not CryptPad's wrapper, not our mock - genuine stock
OnlyOffice `sdkjs` behavior for a foreign cursor occupying your own position). It wasn't reproducible via
automated testing here (tried Chromium and Firefox via Playwright, exact-position clicks, several-pixel
offsets, zero-delay attempts, repeated "Simulate cursor move" clicks, real multi-tab focus/blur cycling,
and human-like multi-step mouse movement instead of instant clicks - all typed cleanly, no stall), which
points at something timing- or input-event-precision-sensitive that synthetic automation doesn't trigger
reliably, consistent with it being a real (if obscure) client-side interaction bug rather than anything in
the message protocol itself.

Since this lives inside the vendored engine, the fix isn't to patch OnlyOffice's behavior (out of reach)
but to stop `onlyoffice-mock-server.js` from manufacturing the exact-overlap case in the first place: a
genuine second user's cursor essentially never lands on your precise current character offset, that's
purely an artifact of "Simulate cursor move" naively reusing your own *latest* real cursor value. It now
keeps a short history (`_cursorHistory`, last 8 real values from this session) and uses the *oldest*
available one instead of the latest, which is very unlikely to still coincide with wherever your cursor
currently is. See the `CURSOR_COLLISION` comment at that button's handler.

Worth flagging for 3.3 even though it's not our bug: if this same client-side behavior exists for *real*
concurrent users (not just our simulation), two people genuinely typing at the same character offset at
the same instant could hit it too, and would have no way to "fix" it themselves without knowing the
tab-refocus trick - possibly worth a support/UX note, or investigating further once real cross-network
multi-user testing is possible (a real second client’s cursor updates arrive with genuine network jitter,
unlike two same-machine tabs, which may also matter for whether this triggers in practice).

## A real limitation found while testing (worth carrying into 3.3)

With two real tabs open on the same document, genuine incremental `saveChanges` messages **were**
delivered, logged, and counted correctly on the other tab (the log length / "N persisted change(s)"
status matched exactly) — but they were **not visually merged into the second tab's already-open
document**. The exact same `changes` payload, delivered as part of the *bulk* `authChanges` reply at
`auth` time (i.e. to a client that hadn't loaded the document yet), rendered correctly.

That's a real, reproducible difference between "apply changes while bootstrapping" and "apply changes to
an already-running session", and it means simply relaying the raw `changes` payload (as CryptPad's own
`ooChannel.send(obj.data.msg)` does, and as this mock does too) isn't sufficient by itself for a
fully-working live experience — something about per-client-relative versioning/expectations is missing
that CryptPad's fuller implementation (chainpad OT, checkpoint reload, image loading, `content.locks`
sync) evidently satisfies but this minimal mock doesn't reproduce. Worth a closer look before finalizing
the live-broadcast design in 3.3; it doesn't block the architecture decisions above, which only depend on
the message *shapes*, not on getting live merge pixel-perfect.

## How to see this yourself

```sh
cd client
TESTBED_SERVER='parsec3://127.0.0.1:6770?no_ssl=true' npm run web:open -- --port 8083
```

Open `http://localhost:8083/1/workspaces`, log in as Alice, create/open a `.docx` in `wksp1`. A panel
titled "OnlyOffice protocol (mocked server)" appears bottom-right inside the editor, showing every
message live plus buttons to simulate another user joining/leaving/moving their cursor, and a
"replay last edit as Bob" button (see its tooltip for why it replays rather than fabricates — OnlyOffice's
internal patch format isn't something we hand-craft for this POC). For a genuine second user, open the
same document in a second tab: presence, cursors, and (per the caveat above) edits sync live via
`localStorage` + `BroadcastChannel`, with zero network requests and nothing written back to Parsec.

## Files touched

- `client/public/onlyoffice-mock-server.js` (new) — the mock server + debug panel.
- `client/public/onlyoffice-host.html` — wires the mock server in instead of the step-1/2 no-op stub.
- `client/src/services/onlyoffice.ts`, `client/src/views/files/handler/editor/FileEditor.vue` — thread a
  stable `documentId` (`${workspaceId}:${path}`, not the per-open-random OnlyOffice `key`) through so two
  tabs/users opening the same file resolve to the same simulated session.
