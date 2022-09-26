// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

//! This crate implement binding for our python front, and those will never be compile on the arch `wasm32`.
//! Trying to compile this crate on the target `wasm32-*` will result in a crash of the `pyo3` build script.
#![cfg(not(target_arch = "wasm32"))]
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

mod addrs;
mod api_crypto;
mod binding_utils;
mod ids;
mod invite;
mod local_device;
mod local_manifest;
mod manifest;
mod storage;
mod time;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _libparsec(_py: Python, m: &PyModule) -> PyResult<()> {
    // Storage
    m.add_class::<storage::WorkspaceStorage>()?;
    Ok(())
}
