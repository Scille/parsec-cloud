# Oxidation in prod - CI test Workflow

From [ISSUE-2491](https://github.com/Scille/parsec-cloud/issues/2491)

depends on [RFC-0026](0026-rust-in-prod-poetry-maturin-based-pyproject.md) & [RFC-0027](0027-rust-in-prod-feature-flag-in-the-rust-extension-binding.md)

## Current situation

Currently CI works by:

1) building a wheel of the parsec project
2) using this wheel to install parsec on each platform
3) run the tests for each platform
4) In a separate Linux build: run the Rust tests, then install parsec with the Rust extension (i.e. the `libparsec` package) with pip and run the Python tests

This won't work once the Rust extension will be mandatory to run parsec (for instance once we have totally replaced the python schema code by the rust one).

## Target

### Installing

We should no longer rely on wheel building for testing.

Instead we can use poetry & maturin (see [RFC-0026](0026-rust-in-prod-poetry-maturin-based-pyproject.md)) to install the correct environment to test.

This approach is simple and should be fine enough (dependencies are locked, difference between a release and a debug build for Rust should not be an issue).

### Testing

So the idea is for each platform:

1) Run Rust-only tests
2) install the project with poetry&maturin
3) Run the fast Python tests
4) Run the slow Python tests

I guess the compilation artifacts generate in step 1) are going to be reused in 2), we should make sure of that (it's easy to have the wrong flag enabled that force a full rebuild...)
