# Vendored OnlyOffice client

This directory holds CryptPad's end-to-end-encrypted fork of the OnlyOffice client-side code
(sdkjs, web-apps, dictionaries, fonts, and the x2t document converter), used to integrate a
document editor directly into Parsec without going through CryptPad itself (see the root
`CLAUDE.md` for the overall plan).

Neither `editor/` nor `x2t/` are **checked into git** (~1.1GB and ~36MB respectively) and aren't
fetched automatically. To (re)create them:

```sh
cd client/vendors/onlyoffice

curl -L -o onlyoffice-editor.zip https://github.com/cryptpad/onlyoffice-editor/releases/latest/download/onlyoffice-editor.zip
unzip onlyoffice-editor.zip -d editor
rm onlyoffice-editor.zip

curl -L -o x2t.zip https://github.com/cryptpad/onlyoffice-x2t-wasm/releases/latest/download/x2t.zip
unzip x2t.zip -d x2t
rm x2t.zip
```

So that e.g. `editor/web-apps/apps/api/documents/api.js` and `x2t/x2t.js` exist.

Building either repo from source requires Docker and is a pain — use a release instead.

## How it's wired up

- `vite.config.ts` serves `editor/*` at `/onlyoffice/*` and `x2t/*` at `/onlyoffice-x2t/*` in dev
  mode (`serve-onlyoffice-vendor` plugin), and copies both to `dist/onlyoffice/*` /
  `dist/onlyoffice-x2t/*` on build (`viteStaticCopy` targets).
- `client/public/onlyoffice-host.html` is a plain (non-Vue) page loaded in an iframe by
  `src/services/onlyoffice.ts` / `FileEditor.vue`. It loads `/onlyoffice/web-apps/apps/api/documents/api.js`
  and wires it to the parent window with postMessage.
- There's no collaborative server yet (see the root `CLAUDE.md`, steps 3 and 4): the host page uses
  CryptPad's `connectMockServer` API (their replacement for OnlyOffice's usual socket.io-based
  server communication, added on top of the stock `api.js`) with handlers that keep everything
  local to the current tab. Nothing is sent back to Parsec, and there's no "save" yet either.
- `src/services/x2t.ts` runs x2t (an Emscripten/wasm CLI tool, driven through its in-memory
  filesystem rather than a JS-friendly API) directly in the main app window to convert an office
  document (`docx`/`xlsx`/`pptx`/`odt`/`ods`/`odp`) into OnlyOffice's native binary format before
  handing it to the editor — mirrors CryptPad's own `www/common/outer/x2t.js`. Only files already
  in that native format (`.bin`) could be opened before this; see `common/fileTypes.ts` for the
  recognized extensions and `services/onlyoffice.ts` (`prepareDocumentContent`) for where
  conversion and the blank-template fallback (for empty files) are decided between.
- Exporting back to `docx`/etc. (needed for saving) isn't implemented yet, nor is passing fonts or
  images through to x2t for the conversion (CryptPad only does that once an editor session already
  exists to source them from — this is the first conversion, before that exists).
