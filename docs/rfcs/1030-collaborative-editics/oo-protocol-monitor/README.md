<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# OnlyOffice co-editing protocol monitor

A small investigation harness that watches the live co-editing client↔server
messages in a draggable panel — **and can hold/release individual frames** to
craft concurrent-operation scenarios (e.g. one client takes a lock, another
tries to take the same lock *before* receiving the first client's lock event).

The harness can run in two mode:

- `npm run demo`: (aka Quick&dirty mode): use the public [OnlyOffice "see-it-in-action" demo](https://www.onlyoffice.com/see-it-in-action)
- `npm run with-docker`: (aka Correct but slow mode): use docker to have the version of OnlyOffice used in Parsec

In `with-docker` mode, the harness page opens the clients under their Parsec
test fixtures names/IDs (Alice / Bob / Mallory — see `docker/index.html`), so a
capture can be used as a test record (`server/tests/editics/records/`)
without running `cook_record.py` on it: only `sessionId` still needs a
fixup, done where the record is parsed (`server/tests/editics/test_records.py`).

## How it works

A local **WebSocket man-in-the-middle** (`proxy.js`) is started first. The
in-page script (`monitor.js`) patches `window.WebSocket` only to **redirect**
OnlyOffice's co-editing socket URL (`wss://…/doc/{id}/c/EIO=4`) to
`ws://127.0.0.1:PORT/oo`. Everything else about the socket stays native, so
socket.io/engine.io sees a real `WebSocket` with real `readyState` /
`MessageEvent`s — no fragile proxy-quacking.

The proxy terminates both legs (browser↔proxy plain `ws`, proxy↔OO real `wss`),
decodes every Engine.IO v4 / Socket.IO v4 frame in Node, and pipes both
directions through a **gate**:

- An **`auto-flow`** / **`manual-flow`** toggle (next to copy/clear) controls
  the gate. In auto-flow (default) frames pass through freely and are shown as
  normal rows. In manual-flow every non-noise frame is **held** at the proxy
  and shown as a yellow ⏸ row with a **`send`** button; clicking `send` releases
  that one frame (recv → delivered to the browser's socket.io; send →
  forwarded to OnlyOffice). Each held frame produces exactly one row (no
  duplicate).
- Keepalive frames (`ping`/`pong`/`noop`) are never gated — holding them would
  make engine.io time the connection out.

Because the proxy owns the bytes on the wire, **recv can actually be paused**:
the message cannot reach a client's socket.io until the proxy forwards it.
This is the only approach that can gate recv; observing alone (e.g.
Playwright's `page.on('websocket')`) can't prevent delivery.

The panel lives in the top frame and is fed by the proxy over a control
WebSocket (`/ctl`). Editor identity (John Smith / Kate Cage / Anonymous on the
demo page, Alice / Bob / Mallory with docker) is resolved in the proxy from
the `auth` event.

## Run

```bash
npm install               # installs playwright + ws
npm run demo       # Quick&dirty: use OnlyOffice public demo page (i.e. www.onlyoffice.com/see-it-in-action)
npm run with-docker       # Correct: use docker to have the version of OnlyOffice used in Parsec
```

A draggable "OO Protocol Monitor" panel appears bottom-right. Click
**auto-flow** to switch to **manual-flow**: frames are then held and each row
shows a **`send`** button to release it.

The browser is launched with `--allow-running-insecure-content` /
`--disable-web-security` and `bypassCSP: true` so the https OnlyOffice page can
connect to the local `ws://127.0.0.1` proxy (mixed content + CSP would
otherwise block it). If the document doesn't load and you only see `poll` /
`unknown` events, check the run terminal for `[proxy] /oo connection:` and
`[proxy] upstream connected` lines — if they're absent, the redirect isn't
reaching the proxy; run with `PROXY_VERBOSE=1 npm run demo` for full proxy logs.
If the proxy is unreachable, the in-page script logs `[OO Monitor] proxy NOT
reachable...` to the page console and falls back to passive mode (OO loads
normally, no gating).

### Crafting a concurrent scenario

1. Switch to **Collaborate** (demo) / edit solo in the first client's editor
   (John on the demo page, Alice with docker).
2. Click **Start …'s editor** on a deferred client's overlay (Kate on the
   demo page, Bob / Mallory with docker), OR click **Re-start …'s editor in
   manual-flow** (on the first deferred overlay, labeled after the first
   client) to reload the whole page with manual-flow armed so the first
   client's whole socket handshake can be stepped through frame by frame from
   a clean state — useful to test concurrent joins.
3. In manual-flow, drive the action (e.g. the first client takes a lock).
   Frames now pile up as ⏸ held rows; click each row's **`send`** to release it
   in the order you want to test (e.g. release the second client's lock send
   *before* releasing the first client's lock-grant recv to it) to force a
   race.

## Files

Two separate code bases, one per mode:

- `demo/` — monitor against the public onlyoffice.com demo page (whatever
  OnlyOffice version it happens to run today):
  - `run.js` — Playwright launcher: starts the proxy, injects the monitor,
    opens the demo page, switches to Collaborate.
  - `proxy.js` — local WebSocket MITM (`/oo` traffic, `/ctl` control channel),
    decode + hold/release logic.
  - `monitor.js` — in-page script: `WebSocket` URL redirect + the panel
    (scenario chips, Kate-deferral overlay, "Re-start John's editor in
    manual-flow").
- `docker/` — monitor against a **pinned local** OnlyOffice Document Server
  (9.3.0, the release the project's bundled editor is built from), started on
  demand via podman/docker and destroyed on exit:
  - `run.js` — launcher: container lifecycle + healthcheck, harness page
    server, then the same proxy + monitor flow as the demo mode.
  - `proxy.js` — same WebSocket MITM as `demo/proxy.js`.
  - `monitor.js` — same panel as `demo/monitor.js`, minus the demo-page
    specifics (scenario-tab observation); its deferral is per-editor and
    covers both manual clients (Bob / Mallory).
  - `index.html` — the harness page (Alice / Bob / Mallory editors on one
    document key); the opened document comes from
    `client/tests/e2e/data/imports/` (`OO_DOC` to pick another file,
    default `document.docx`).
- `package.json` — the `npm run demo` / `npm run with-docker` scripts.
