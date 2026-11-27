# cli — Agent Guide

Standalone end-user command-line client written in Rust (`parsec-cli`), built on top of
`libparsec` with `clap`, `tokio`, `dialoguer`, and `assert_cmd`. See also `cli/README.md`.

## Commands

- Build: `cargo build --package parsec-cli`
- Run: `cargo run --package parsec-cli`
- Run unit tests: `cargo test --package parsec-cli --lib`
- Run integration tests: `cargo test --package parsec-cli --test integration <test_name>`
  (or `cargo nextest run --package parsec-cli`)
- Run benchmarks (Criterion): `cargo bench --package parsec-cli --bench <minimal_short_id|upload_speed|fetch_children>`
- Lint & format: `cargo clippy --package parsec-cli --all-targets` and `cargo fmt --check --all`
- Integration tests run against a testbed server — start one first with
  `python make.py run-testbed-server` (see root `AGENTS.md`).

## Rules

- **Workspace lints:** Inherits root `Cargo.toml` lints (`unwrap_used = "deny"`,
  `undocumented_unsafe_blocks = "deny"`). **Never** use `.unwrap()` in non-test code.
- **Integration test registration:** `autotests = false` is set in `cli/Cargo.toml`. All
  integration tests must be wired into `cli/tests/integration/mod.rs` (new standalone `.rs`
  files in `cli/tests/` will not run automatically).
- **Test environment feature:** Integration/CLI tests rely on the `testenv` feature (enabled via
  self dev-dependency); keep test-only hooks gated behind `#[cfg(feature = "testenv")]`.
- **Cross-platform constraints:**
  - `rexpect` (interactive PTY tests) is **Unix-only** (`#[cfg(target_family = "unix")]`);
    gate any `rexpect` test helpers so Windows builds do not break.
  - Maintain compatibility with `manpage` (`clap_mangen`) and `shell-completion`
    (`clap_complete`) default features when modifying `clap` commands or arguments.
- **Thin CLI layer:** Keep sync, crypto, and protocol state machines in `libparsec/`; restrict
  `parsec-cli` to `clap` argument parsing, interactive prompts (`dialoguer`/`rpassword`),
  output formatting, and exit codes.
