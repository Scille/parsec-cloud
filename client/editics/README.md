# Editics

Editics is the integration of [OnlyOffice](https://www.onlyoffice.com) into the
Parsec GUI, to edit office documents (text documents, spreadsheets, slides)
directly from a workspace.

The feature is gated by the `PARSEC_APP_ENABLE_EDITICS` environ variable.

## Frame nesting

The whole system is a stack of three nested windows:

```
┌─────────────────────────────────────────────────────────────────────────┐
│  Parsec GUI parent page (i.e. `FileEditor.vue`)                         │
│                                                                         │
│  - libparsec lives here and is not exposed to the iframes below         │
│  - Contains an `<iframe ref="editorFrame">` that is initialized by      │
│    `src/services/editics.ts` to create & control the host iframe        │
│  - Communicates with the host iframe by `postMessage` to handle the     │
|    operations involving libparsec (e.g. saving the document back in     |
|    the workspace)                                                       |
│                                                                         │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │  Host iframe (offline or collaborative, see below)                │  │
│  │                                                                   │  │
│  │  - Loads OnlyOffice x2t to handle conversions from/into           |  |
|  |    OnlyOffice's .bin internal format                              |  |
|  |  - Loads OnlyOffice API (i.e.                                     │  │
|  |    `onlyoffice/web-apps/apps/api/documents/api.js`) and starts    │  │
|  |    the editor                                                     │  │
|  |  - Runs a `OOMockServer` instance that handles events (e.g.       |  |
|  |    cursor movement) going in out of the OnlyOffice editor         |  |
│  │                                                                   │  │
│  │  ┌─────────────────────────────────────────────────────────────┐  │  │
│  │  │  Editor iframe                                              │  │  │
│  │  │                                                             │  │  │
│  │  │  - The actual OnlyOffice document editor (word/cell/slide)  │  │  │
│  │  │                                                             │  │  │
│  │  └─────────────────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────┘
```

## Origin isolation

The host page should always be served from a different origin than the Parsec GUI
parent page, so the OnlyOffice editor it contains is sandboxed away from the
app's data (e.g. `libparsec`, its storage and its credentials). The exact
origin depends on how the app is deployed (see `getEditicsOrigin` in
`src/services/editics.ts`):

| Deployment                   | Parent page origin             | Host page origin                                                             |
| ---------------------------- | ------------------------------ | ---------------------------------------------------------------------------- |
| Electron                     | `parsec-desktop://-`           | `parsec-editics://-`                                                         |
| Web dev (Vite dev server)    | `http://localhost:8080`        | `http://editics.localhost:8080`                                              |
| Web release with `serverUrl` | e.g. `https://app.example.com` | ``serverUrl`used as-is, e.g.`https://editics0.example.com`                   |
| Web release without prefix   | e.g. `https://app.example.com` | `editics.` always used as the prefix, e.g. `https://editics.app.example.com` |

Electron uses a custom scheme for editics (`parsec-editics:`) as it ensures a high level of isolation automatically.

## Edition modes

Editics works in two modes, with one host page per mode (the parent picks the
one to serve in the iframe, see `src/services/editics.ts`):

- **Offline** (`editics/offline.html`, logic in `offline.ts`): the edition is
  done purely locally, and the OnlyOffice editor's co-authoring protocol is
  answered by the `OfflineMockServer` (single user, no server involved).
- **Collaborative** (`editics/collaborative.html`, logic in `collaborative.ts`):
  the co-authoring protocol events are translated between the OnlyOffice
  protocol and the editics protocol by `EditicsTranslator` (see
  `collaborative_protocol.js`), and exchanged with the Parsec server hosting
  the session (SSE stream for server→client, HTTP POST for client→server,
  see RFC 1030).

Both modes share their host page machinery (x2t conversions, document media,
save flow, communication with the parent window, ...) through `common.ts`.

### Mode selection (see `FileEditor.vue` & `src/services/editics.ts`)

The collaborative mode is the default one (RFC 1030 §1): when opening a
document, the editor component asks the service to resolve the collaborative
session info (see `getEditicsCollaborativeSession`), which requires:

- the client to be online (`ClientInfo.isServerOnline`): the session is hosted
  by the Parsec server, hence cannot be joined while offline. In that case a
  dialog informs the user that the collaborative edition is not possible and
  lets him either go back or open the document in offline mode;
- the document to be opened at its current version (a file displayed from the
  workspace history is always opened in offline mode);
- an editics authentication token to be available. The genuine token is
  signed with the device's signing key, which the libparsec bindings do not
  expose yet: until then the client uses the `trust_me_bro` placeholder
  signature the server accepts for that exact purpose (see
  `generateEditicsAuthenticationToken` and the `TODO: remove me when the
client supports token signature!` in `server/parsec/components/auth.py`).

When any of those is not met, the offline host page is served instead (and
`options.collaborative` is left unset, which the host pages double-check).

### Document loading in collaborative mode (see `collaborative.ts`)

All the participants of an edition session must edit against a single
**original content** — the converted document (OnlyOffice `.bin` + the media
files extracted by x2t), uploaded to the session by its first editor and
sealed with the workspace key (the server only sees an opaque blob, RFC 1030
§1.2). Otherwise the op patches produced by a client against its own `x2t`
conversion would not apply cleanly on the others (two `x2t` versions produce
different `.bin` bases).

The load is driven by the `auth` handshake (the editor sends its `auth` during
its boot, before any document is loaded):

- the reply says the session already has a content
  (`originalContentUploaded: true`) → the host **fetches** it
  (`parsecFetchOriginalContent`), asks the parent to decrypt it and loads it
  into the editor (no x2t conversion at all);
- the reply says it hasn't (`false`) → the host converts the document locally
  with x2t, then:
  - as an **editor**, it asks the parent to encrypt the result and **uploads**
    it (`parsecUploadOriginalContent`) _before_ loading it into the editor —
    this ordering is what guarantees the server-side invariant that no
    document modification is accepted before an original content exists
    (first-upload-wins: if a concurrent upload won, the reply says
    `alreadyUploaded: true` and the host fetches the winner's content
    instead);
  - as a **viewer** (which cannot upload, it would be kicked like for any
    editor-only event), it loads its own conversion and waits for the
    `updateVersion` broadcast.

The content crypto (msgpack archive + workspace-key sealing) is provided by
libparsec (`workspaceEncryptEditicsOriginalContent` /
`workspaceDecryptEditicsOriginalContent`), which only the parent can call: the
host asks for it through the `oo-encrypt-original-content` /
`oo-decrypt-original-content` messages (see `parent_host_api.ts`).

A viewer holding its local conversion **reloads the whole host page** when the
content becomes available (`updateVersion`, broadcast to the connected
viewers when the first editor's upload lands): the fork's client cannot
re-open a document in place, so the page reboots from scratch — fresh
participant, fresh `auth` whose reply now says the content is uploaded →
fetch → load. The parent re-sends `oo-open` on every `oo-host-ready` for that
purpose, and resets its save bookkeeping accordingly. The same reload applies
to anyone whose `auth` reply says the content is uploaded while a local
conversion is loaded (the restore case: the participant was disconnected
when the upload happened and missed the broadcast).

### Saving (see `onSaveRequest` in `common.ts`)

The persistence of the workspace file differs between the modes:

- **Offline**: the editor's SDK autosave loop is disabled (`autosave: 0`), so
  the document is only persisted when the parent asks for it (leaving the
  editor, see `FileEditor.vue`'s save-on-close path): the host serializes the
  document, converts it back to the workspace file's format with x2t and asks
  the parent to write it.
- **Collaborative**: the changes flow to the session continuously (the SDK
  autosave loop drives `isSaveLock` -> `saveChanges`, see above), so the
  workspace file is only persisted on a manual save (when the session has
  accumulated changes, tracked by the host page). The wrapper's per-`asc_Save`
  hook (full serialize + x2t + write on every autosave tick!) is deliberately
  NOT installed in this mode: it would flood the workspace with writes while
  typing and hold the paragraph locks for the whole conversion.

Note the e2e tests keep the editor in offline mode by default (so the suite
doesn't depend on the still work in progress collaborative stack): a test can
opt in by setting the `TESTING_ENABLE_EDITICS_COLLABORATIVE` window flag
(like the other `TESTING_*` flags, see `tests/e2e/helpers/fixtures.ts`).

Note the collaborative host page reaches the Parsec server directly for the
session (SSE stream + HTTP POST):

- in web release, the app is served by the Parsec server itself and the
  session is reached on the editics origin (same server, as constrained by
  the editics pages content security policy, see `EditicsHostIsolationMiddleware`
  in `server/parsec/asgi/__init__.py`);
- in web dev, the host page is served by the Vite dev server while the session
  lives on the actual Parsec server (cross-origin): the server must be
  configured to allow the editics origin for CORS (see the
  `--cors-allow-origins` option of `parsec run`, e.g.
  `--cors-allow-origins=http://editics.localhost:8080`);
- in Electron, the host page's content security policy allows reaching the
  Parsec server (see `electron/src/cspRules.ts`).

## Packaging (see `vite.config.ts`)

The host pages must stay standalone since they are loaded in an iframe.
Hence they live outside the app's import graph (no shared chunks with the app
bundle, no `@/` imports).

- **Dev**: the Vite dev server serves `editics/offline.html` &
  `editics/collaborative.html` and transpiles their entries on the fly.
- **Release**: `scripts/vite_plugin_editics.ts` bundles each typescript entry
  with esbuild and inlines it in its host page, which is emitted with a
  content hash (e.g. `dist/editics/offline-8bdb1067.html`) for cache busting.
  The actual name of each host page is advertised to the main app through a
  meta tag injected in the main app's `index.html` (i.e. `editics-offline-page`
  & `editics-collaborative-page`).

On top of that, the OnlyOffice editor and x2t assets trees are exposed/copied
(using `viteStaticCopy` plugin) verbatim, but into versioned folders inside the
editics folder (e.g. `editics/onlyoffice/<version>/`, since they are only used
by editics) in order to handle cache busting when upgrading the dependencies
(see `scripts/vite_plugin_editics.ts`).
