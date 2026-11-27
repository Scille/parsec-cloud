# server — Agent Guide

Python backend (FastAPI, Starlette, Uvicorn), Click administrative CLI, and the PyO3
Rust-to-Python extension (`server` workspace member). See also `server/README.md`.

## Commands

- Initial install / compile PyO3 extension: `python make.py python-dev-install` (alias: `i`)
- Rebuild PyO3 extension after Rust changes: `python make.py python-dev-rebuild` (alias: `r`)
- Run the server: `uv --project server run parsec`
- Run tests (from `server/`): `uv run pytest tests` (options: `-n 4`, `-x`, `-k EXPR`, `--postgresql`)
- Lint & format: `ruff check server/` and `ruff format --check server/` (via `uv --project server run --only-dev`)
- Start local testbed server: `python make.py run-testbed-server` (alias: `rts`) — runs
  `uv run python -m parsec testbed` from `server/`

## Rules

- **Package manager:** Always use `uv` (`uv run`, `uv sync --locked`); never use raw `pip` or
  system Python.
- **PyO3 / Maturin:** Never hand-edit generated PyO3 stubs (`server/parsec/_parsec_pyi/`) or
  build artifacts; run `python make.py r` whenever `server/` Rust bindings or `libparsec/`
  change.
- **FastAPI / Uvicorn async discipline:** Keep FastAPI/Starlette endpoints non-blocking; never
  run synchronous blocking I/O or heavy CPU tasks directly on the Uvicorn event loop.
- **Click CLI:** Keep Click commands (`parsec` CLI) thin; delegate all domain logic to testable
  service functions.
