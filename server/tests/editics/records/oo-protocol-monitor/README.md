<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# OnlyOffice co-editing protocol monitor

> ![WARNING]
>
> ⚠️ VIBE CODE DISCLAIMER ⚠️
>
> This code has been vibe coded without deep review.
>
> This is considered okay since it is a self-contained helper for generating
> tests (hence its blast radius is very small and never impact production).

Starts a local OnlyOffice Document Server (in version 9.3.0, the same we integrate
in Parsec) and opens a web browser contaning 3 OnlyOffice editors (Alice, Bob, Mallory)
along with a panel showing all OnlyOffice events flowing from/to the server.

## Run

```bash
npm install
npm start
```

This starts the document server container, then opens a chromium window with
three OnlyOffice editors and the monitor panel. Both always listen on the
same ports (document server: `8080`, harness page: `8081`), so the harness
URL — `http://127.0.0.1:8081/` — is stable across runs.

From there the harness is self-explaining: the panel shows every frame, each
editor pane has a control bar with disconnect/reconnection primitives, and
the page offers document type buttons (word / cell / slide / pdf).

## Generating test records

1. Run the scenario in the editors (joins, edits, disconnects… — the
   harness's UI controls cover the whole protocol surface).
2. Click the `copy` button in the "OO Protocol Monitor" panel.
3. Paste the result as a new file in `server/tests/editics/records/`.

If an existing test record needs to be regenerated (e.g. because OnlyOffice
behaviour changed), don't script its replay: the harness's UI controls are
already enough to reproduce the scenario. Simply describe the scenario to an
LLM (or follow the description contained in the record itself) and drive the
harness manually to capture the new record.

## Document server configuration

The image's own `/etc/onlyoffice/documentserver/local.json` is used as-is: the
only tweak is applied **from within the container**, just before the document
server's entrypoint starts. `run.js` overrides the container's entrypoint with
`/bin/bash -c '<json edit> && exec /app/ds/run-document-server.sh'`, where the
edit rewrites one key using the image's own `json` tool (the one the entrypoint
itself uses for its rewrites).

Doing it this way — instead of mounting a whole `local.json` over the image's —
matters: the entrypoint rewrites `local.json` on boot (that is how
`JWT_ENABLED=false` turns the token checks off), and a read-only mounted config
defeats those rewrites, leaving JWT on so every editor dies with
"The document security token is not correctly formed.".

The tweak lowers `services.CoAuthoring.editor.websocketMaxPayloadSize` from
the stock `1.5MB` to `4KB`. The co-editing client splits a save into several
`saveChanges` chunks once the accumulated change bytes reach that size, so the
small value is what makes the multi-chunk save path (`savePartChanges`, and the
`endSaveChanges`-gated lock recalculation) reachable at all — with the stock
value it would take a >1.5MB change payload.

Set `OO_WS_PAYLOAD=` (empty) to keep the stock `1.5MB`.

## Scripted record generation

The interactive harness above is the normal way to capture a record. For
scenarios that are hard to drive by hand (or that must be reproducible), use
`harness/drive.js`: it reuses an **already-running** document server instead of
starting its own container, and runs a JS scenario against the page.

```bash
# with a document server already listening on 127.0.0.1:8080
HEADLESS=1 OO_PAGE_PORT=8091 \
  node harness/drive.js scenarios/71_lock_recalc_own_lock_xlsx.js \
  --doc=cell --out=/tmp/record.md
```

- `--doc=word|cell|slide` picks the sample document (default `word`).
- `--out=FILE` writes the record; without it the record is printed on stdout.
- `OO_PAGE_PORT` defaults to `OO_DS_PORT + 1` (run.js's choice); override it to
  coexist with an interactive harness already bound to that port.

Scenarios live in `scenarios/` and export `async function run(ctx)`, where `ctx`
exposes `page` (Playwright), `sleep`, `startEditor(who, {view})`,
`editorFrame(who)`, `waitQuiescence(ms)` and `exportRecord()`. `scenarios/_lib.js`
holds the shared editor-driving helpers (clicking a cell, typing, committing an
edit, forcing a save).

The exported text is produced by the monitor's own `exportText()` (the very one
behind the `copy` button), so it cannot drift from the interactive capture.
Remember to fill in the `Scenario:` block of the generated record.
