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

## Packaging (see `vite.config.ts`)

The host page must stay standalone since it is loaded in a iframe.
Hence it lives outside the app's import graph (no shared chunks with the app
bundle, no `@/` imports).

- **Dev**: the Vite dev server serves `editics/offline.html` and transpiles
  `offline.ts` on the fly.
- **Release**: `scripts/vite_plugin_editics.ts` copies as-is `editics/offline.html`
  and bundles each typescript entry with esbuild to emit `offline-<hash>.js`.

On top of that, the OnlyOffice editor and x2t assets trees are exposed/copied
(using `viteStaticCopy` plugin) verbatim, but into versioned folders inside the
editics folder (e.g. `editics/onlyoffice/<version>/`, since they are only used
by editics) in order to handle cache busting when upgrading the dependencies
(see `scripts/vite_plugin_editics.ts`).
