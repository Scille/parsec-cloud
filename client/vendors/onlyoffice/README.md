# Vendored OnlyOffice client

This directory holds CryptPad's end-to-end-encrypted fork of the OnlyOffice client-side code
(sdkjs, web-apps, dictionaries, fonts), used to integrate a document editor directly into Parsec
without going through CryptPad itself (see the root `CLAUDE.md` for the overall plan).

`editor/` is **not checked into git** (it's ~1.1GB) and isn't fetched automatically. To (re)create
it, download the latest release from https://github.com/cryptpad/onlyoffice-editor/releases and
extract it here, so that e.g. `editor/web-apps/apps/api/documents/api.js` exists:

```sh
cd client/vendors/onlyoffice
curl -L -o onlyoffice-editor.zip https://github.com/cryptpad/onlyoffice-editor/releases/latest/download/onlyoffice-editor.zip
unzip onlyoffice-editor.zip -d editor
rm onlyoffice-editor.zip
```

Building this repo from source requires Docker and is a pain — use a release instead.

## How it's wired up

- `vite.config.ts` serves `editor/*` at `/onlyoffice/*` in dev mode (`serve-onlyoffice-vendor`
  plugin) and copies it to `dist/onlyoffice/*` on build (`viteStaticCopy` target).
- `client/public/onlyoffice-host.html` is a plain (non-Vue) page loaded in an iframe by
  `src/services/onlyoffice.ts` / `FileEditor.vue`. It loads `/onlyoffice/web-apps/apps/api/documents/api.js`
  and wires it to the parent window with postMessage.
- There's no collaborative server yet (see the root `CLAUDE.md`, steps 2 and 3): the host page uses
  CryptPad's `connectMockServer` API (their replacement for OnlyOffice's usual socket.io-based
  server communication, added on top of the stock `api.js`) with handlers that keep everything
  local to the current tab. Nothing is sent back to Parsec.
- Only files already in OnlyOffice's own native binary format (as used internally during a
  collaborative session, not `.docx`/`.xlsx`/`.pptx`) can be opened — no x2t conversion yet. See
  `common/fileTypes.ts` (`.bin` extension) and `services/onlyoffice.ts`
  (`BLANK_TEMPLATE_URLS`/`getEditableContent`, used to bootstrap an empty file).

## x2t (not yet integrated)

https://github.com/cryptpad/onlyoffice-x2t-wasm builds the document converter (`.docx` <-> native
format) as wasm. Not used yet (see step 2 in the root `CLAUDE.md`); release assets at
https://github.com/cryptpad/onlyoffice-x2t-wasm/releases.
