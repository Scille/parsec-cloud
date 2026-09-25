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
│  │  Host iframe (i.e. `editics/offline.html`)                        │  │
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

## Packaging (see `vite.config.ts`)

The host page must stay standalone since it is loaded in a iframe.
Hence it lives outside the app's import graph (no shared chunks with the app
bundle, no `@/` imports).

- **Dev**: the Vite dev server serves `editics/offline.html` and transpiles
  `offline.ts` on the fly.
- **Release**: `scripts/vite_plugin_editics.ts` bundles each entry with
  esbuild and inlines the code into the emitted HTML, so each page ships as a
  single self-contained asset (e.g. `dist/editics/offline.html`).

On top of that, the OnlyOffice editor and x2t assets trees are exposed/copied
(using `viteStaticCopy` plugin) verbatim.
