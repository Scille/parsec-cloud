# client — Agent Guide

TypeScript & Vue 3 frontend (`client-vite`), built with Vite and deployed to Web, Desktop
(`client/electron`), and Mobile (Capacitor + Ionic Framework). See also `client/README.md`.

## Commands

- Node version: enforce `.nvmrc`.
- Dev server (web): `cd client && npm run dev` (or `npm run web:open`)
- Dev desktop app (Electron): `cd client && npm run electron:open`
- Typecheck & lint (zero warnings allowed): `cd client && npm run lint`
  (runs `eslint . --max-warnings=0` and `vue-tsc --noEmit`)
- Format check & unused code check: `cd client && npm run prettier:check && npm run knip`
- Unit tests (Vitest): `cd client && npm run test:unit -- <path/to/test.spec.ts>`
- E2E tests (Playwright): start a testbed first (`python make.py rts`), then
  `cd client && npm run test:e2e:headless` (or `npm run test:e2e:important`)

## Rules

- **Imports:** **Never** use relative parent imports (`../`); `eslint-plugin-no-relative-import-paths`
  is enforced with `--max-warnings=0`. Use the configured path aliases (`@/*`, `@tests/*`).
- **Dead code:** `knip` is configured in `package.json` — remove unused exports, files, and
  dependencies rather than leaving dead code.
- **Platform separation (Web vs. Electron vs. Capacitor/Ionic):**
  - **Never** import Electron/Node APIs directly inside shared Vue components; use platform
    abstraction layers (e.g. `window.getPlatform()` in `client/src/main.ts`, or the
    `@/parsec/environment.ts` helpers) so Web and Capacitor mobile builds remain intact.
  - Rebuilding native/web Rust bindings requires `python make.py er` (Electron) or
    `python make.py wr` (Web).
- **Vite version warning:** Do not bump `vite` without updating `BUILD_TARGET` in
  `client/vite.config.ts`.
