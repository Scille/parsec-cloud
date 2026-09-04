<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# OnlyOffice co-editing protocol monitor

A small investigation harness that opens the public
[OnlyOffice "see-it-in-action" demo](https://www.onlyoffice.com/see-it-in-action)
and watches the live co-editing client↔server messages in a draggable panel —
**and can hold/release individual frames** to craft concurrent-operation
scenarios (e.g. John takes a lock, Kate tries to take the same lock *before*
receiving John's lock event).

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
the message cannot reach Kate's socket.io until the proxy forwards it. This is
the only approach that can gate recv; observing alone (e.g. Playwright's
`page.on('websocket')`) can't prevent delivery.

The panel lives in the top frame and is fed by the proxy over a control
WebSocket (`/ctl`). Editor identity (John Smith / Kate Cage / Anonymous) is
resolved in the proxy from the `auth` event. The panel auto-filters by the
active scenario tab: Collaborate → John & Kate; others → Anonymous.

## Run

```bash
npm install   # installs playwright + ws
npm run demo
```

A draggable "OO Protocol Monitor" panel appears bottom-right. Click
**auto-flow** to switch to **manual-flow**: frames are then held and each row
shows a **`send`** button to release it.

The browser is launched with `--allow-running-insecure-content` /
`--disable-web-security` and `bypassCSP: true` so the https OnlyOffice page can
connect to the local `ws://127.0.0.1` proxy (mixed content + CSP would
otherwise block it). If the document doesn't load and you only see `poll` /
`unknown` events, check the run-demo terminal for `[proxy] /oo connection:` and
`[proxy] upstream connected` lines — if they're absent, the redirect isn't
reaching the proxy; run with `PROXY_VERBOSE=1 npm run demo` for full proxy logs.
If the proxy is unreachable, the in-page script logs `[OO Monitor] proxy NOT
reachable...` to the page console and falls back to passive mode (OO loads
normally, no gating).

### Crafting a concurrent scenario

1. Switch to **Collaborate**, edit solo in John's editor.
2. Click **Start Kate's editor** (Kate's editor is deliberately deferred), OR
   click **Re-start John's editor in manual-flow** (under "Start Kate's
   editor") to tear John down, arm manual-flow, and rebuild him so his whole
   socket handshake can be stepped through frame by frame — useful to test
   concurrent joins.
3. In manual-flow, drive the action (e.g. John takes a lock). Frames now pile
   up as ⏸ held rows; click each row's **`send`** to release it in the order
   you want to test (e.g. release Kate's lock send *before* releasing John's
   lock-grant recv to Kate) to force a race.

## Files

- `proxy.js` — local WebSocket MITM (`/oo` traffic, `/ctl` control channel),
  decode + hold/release logic. Started by run-demo.js.
- `monitor.js` — in-page script: one-line `WebSocket` URL redirect + the panel
  (control-channel client, event rendering, `send` release buttons, scenario
  filter, Kate-deferral overlay + "Re-start John's editor in manual-flow").
- `run-demo.js` — Playwright launcher: starts the proxy, injects the monitor
  with `window.__OO_PROXY_PORT` set, opens the demo, switches to Collaborate.
