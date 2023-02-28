# Oxidation in prod - Feature flag in the Rust extension bindings

From [ISSUE-2490](https://github.com/Scille/parsec-cloud/issues/2490)

Depends on [RFC-0026](0026-rust-in-prod-poetry-maturin-based-pyproject.md)

## Overview

We want to be able to have feature flag to choose to only use a subset of the Rust extension.

This is required because we want to start small by only shipping the schemas in Rust (so we want to disable all the other components in the Rust extension)

This is probably going to be useful when rewriting other complex part of the application where we want to have it merged on master while not using it in production for some time.

## Architecture

Currently the Python code tries to individually each element (e.g. `UserCertificate`) that might be oxidized.
So feature flag can be achieved by making this import failed

So for the feature flag there is two approaches:

1) implement the flag as a feature in the Cargo.toml, hence the extension module would only contains the elements that are to be used
2) implement the flag in Python, hence there would be a check to determine for a given item if it should be imported from the rust extension or from regular Python code

Approach 1) should bring a faster compilation (less stuff to compile) and a smaller binary size (while this is most likely pretty puny, so I think we can ignore this). However it requires two separate compilation + pip install steps if we want to test in the CI both configurations...

So I guess approach 2) is more flexible and is the way to go.

Considering how to config should be provided, environment variable seems the obvious way to go, typically having a `PARSEC_ENABLE_UNSTABLE_OXIDATIONS` (I don't think a fine grain for enabling only a subset of oxidized element is needed, and this would add complexity)

On top of that we most likely want a `--enable-unstable-oxidations` flag in pytest that modify `os.environ` to simplify running the tests
