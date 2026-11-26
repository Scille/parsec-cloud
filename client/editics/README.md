# Editics

Editics is the integration of [OnlyOffice](https://www.onlyoffice.com) into the
Parsec GUI, to edit office documents (text documents, spreadsheets, slides)
directly from a workspace.

Editics works in two modes:

- **Offline**: the edition is done purely locally.
- **Collaborative**: the co-authoring protocol events are translated between the
  OnlyOffice protocol and the editics protocol, and exchanged with the Parsec
  server hosting the session.

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
│  │  Host iframe (i.e. `editics/index.html`)                          │  │
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
origin depends on how the app is deployed (see `getEditicsIframeOrigin` in
`src/services/editics.ts`):

| Deployment                | Parent page origin             | Host page origin                                                                       |
| ------------------------- | ------------------------------ | -------------------------------------------------------------------------------------- |
| Electron                  | `parsec-desktop://-`           | `parsec-editics://-`                                                                   |
| Web dev (Vite dev server) | `http://localhost:8080`        | `http://editics.localhost:8080`                                                        |
| Web release               | e.g. `https://app.example.com` | the editics server hosting the document's session, e.g. `https://editics0.example.com` |

Electron uses a custom scheme for editics (`parsec-editics:`) as it ensures a high level of isolation automatically.

In web release mode the host page is always fetched from the editics server
hosting the document's collaborative session: the editics pages are served
with a Content-Security-Policy only allowing them to connect to their own
origin. Hence editics cannot be used at all in web release mode if no editics
server is available (not even in offline mode), unlike on Electron where the
app itself provides the editics pages.

## Packaging (see `vite.config.ts`)

The host page must stay standalone since it is loaded in a iframe.
Hence it lives outside the app's import graph (no shared chunks with the app
bundle, no `@/` imports).

- **Dev**: the Vite dev server serves `editics/index.html` and transpiles
  `start.ts` on the fly.
- **Release**: `scripts/vite_plugin_editics.ts` bundles the typescript entry
  with esbuild and inlines it in the host page, which is emitted with a
  content hash (e.g. `dist/editics/index-8bdb1067.html`) for cache busting.
  The actual name of the host page is advertised to the main app through a meta
  tag injected in the main app's `index.html` (i.e. `editics-page`).

On top of that, the OnlyOffice editor and x2t assets trees are exposed/copied
(using `viteStaticCopy` plugin) verbatim, but into versioned folders inside the
editics folder (e.g. `editics/onlyoffice/<version>/`, since they are only used
by editics) in order to handle cache busting when upgrading the dependencies
(see `scripts/vite_plugin_editics.ts`).
