# Parsec Cloud — Agent Guide

Parsec is a zero-trust, client-side end-to-end encrypted, cloud file-sharing platform. This monorepo contains the core cryptographic & business logic library (`libparsec`), server, desktop/web clients, CLI, language bindings, and documentation.

## Components

Read the local `README.md` and `AGENTS.md` (when present) before modifying files in a specific component:

- `libparsec/` Core business logic Rust library (cryptography, sync engine, rpc protocol, mountpoint logic)
- `bindings/` libparsec bindings exposed to clients (electron, web, android)
- `server/` Backend API & meta-data server (Python, packaged with maturin-bound Rust)
- `client/` GUI for desktop & web apps (Vue 3, Capacitor, Electron)
- `cli/` CLI client (Rust, parsec-cli cargo package)
- `json_schema/` Schemas for `libparsec/crates/protocol/schema/` & libparsec/crates/types/schema/
- `docs/` Comprehensive documentation (Sphinx, ReadTheDocs)

Cross-cutting docs live in `docs/development/` (workflow, testbed, adding RPC commands,
adding types with bindings, glossary). Start at `docs/development/README.md`.

## Toolchain & commands

The dev environment is assumed to be already configured. Tool versions are pinned by the
following files: `@rust-toolchain.toml` (Rust), `@.nvmrc` (Node), and `uv` for Python
(server venv in `server/.venv`).

`make.py` at the root wraps common tasks (run `python make.py --help`); notably
`python make.py run-testbed-server` starts the testbed server.

### Build / test per component

- **libparsec / Rust**: `cargo build --workspace`; tests with `cargo nextest run -p <crate>`
  (e.g. `cargo nextest run -p libparsec_protocol`). CLI: `cargo build/run --package parsec-cli`,
  tests `cargo nextest run --package parsec-cli`.
- **server**: `python ./make.py python-dev-install` once, then `uv --project server run parsec`;
  tests from `server/`: `uv run pytest tests` (options: `-n 4`, `-x`, `-k EXPR`, `--postgresql`).
- **client**: `npm install`; unit tests `npm run test:unit`; lint `npm run lint` (eslint +
  vue-tsc); e2e `npm run test:e2e:headless` (Playwright); web dev `npm run web:open`;
  desktop dev `npm run electron:open`.

### Testbed server

CLI tests, client e2e tests, dev-mode clients, and Rust tests annotated with
`#[parsec_test(testbed = "...", with_server)]` (used in `libparsec_client`,
`libparsec_client_connection`, `libparsec_account`, `libparsec_openbao`, …) all need a
testbed server:

```shell
python make.py run-testbed-server        # or via Docker
export TESTBED_SERVER='parsec3://127.0.0.1:6770?no_ssl=true'
```

The `testbed` crate itself can also spawn one from `TESTBED_SERVER` (or start its own).

## Critical rules (Must / Never)

- **Security & cryptography**: **Never** weaken, bypass, or alter cryptographic primitives,
  signature verification, or key-management flows in `libparsec/` without explicit instruction.
  Parsec is ANSSI CSPN-certified; treat all crypto/auth changes as high-risk.
- **Dependencies & licenses**: **Never** add new Rust crates without running `cargo deny check`
  (enforces `deny.toml` license/security policies). Respect dual-licensing
  (`LICENSE-AGPL3` / `LICENSE-BUSL`).
- **Generated & lock files**: **Never** hand-edit `Cargo.lock`, `package-lock.json` or compiled
  binding artifacts; use the appropriate package manager or `make.py` target. Bindings are
  regenerated with `python make.py r` (Python), `er` (Electron), `wr` (Web).

## Conventions

- **Commits**: conventional commits (`type(scope): subject`) enforced by commitlint — allowed
  types, scopes and rules are configured in `@.commitlintrc.yml`; check that file for the
  up-to-date list.
- **Changelog**: every user-facing change needs a newsfragment in `newsfragments/`
  named `<issue>.<type>.rst` (types: `feature`, `bugfix`, `misc`, `removal`, …).
  Note: newsfragments will be replaced by `git-cliff` in the near future.
- **Pre-commit**: hooks run on commit via `pre-commit` (or `prek`) — license headers via
  `misc/license_headers.py`, trailing whitespace/EOL fixes, commitlint, ruff, taplo, yamllint,
  cspell, … — see `@.pre-commit-config.yaml`. Run `pre-commit run --all-files` (or
  `prek run --all-files`) before pushing if in doubt.
- **Licensing**: new source files (Python/Rust/Vue/…) need the BUSL-1.1 copyright header — the
  pre-commit hook adds it automatically (`python misc/license_headers.py add`).
- **Branches/PRs**: no strict branch naming convention, but we recommend prefixing the branch
  with the issue ID (e.g. `1234-my-fix`); rebase on `master`, no merge commits; keep PRs small
  and link the issue (see `docs/development/guide-development-workflow.md`).
- **Editor**: respect `@.editorconfig`.

## Quick orientation

- Architecture & design docs: `docs/development/design-*.md`, RFPs in `docs/rfcs/`.
- How-to guides (adding an RPC command, adding a type with bindings, SQL, benchmarks…):
  `docs/development/how-to-*.md`.
- Glossary (domain terms like workspace/realm/certificate/manifest): `docs/development/glossary-EN.md`.
- Protocol typings generation: `@misc/gen_protocol_typings.py` generates the Python
  protocol bindings used by the server (`server/parsec/_parsec_pyi/protocol`); the
  clients (web/Electron) get their TypeScript definitions from `@bindings/generator/generate.py`.
