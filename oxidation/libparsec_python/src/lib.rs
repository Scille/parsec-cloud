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
mod protocol;
mod storage;
mod time;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _libparsec(_py: Python, m: &PyModule) -> PyResult<()> {
    // Events
    m.add_class::<protocol::EventsListenReq>()?;
    m.add_class::<protocol::EventsListenRep>()?;
    m.add_class::<protocol::EventsSubscribeReq>()?;
    m.add_class::<protocol::EventsSubscribeRep>()?;

    // User
    m.add_class::<protocol::UserGetReq>()?;
    m.add_class::<protocol::UserGetRep>()?;
    m.add_class::<protocol::UserCreateReq>()?;
    m.add_class::<protocol::UserCreateRep>()?;
    m.add_class::<protocol::UserRevokeReq>()?;
    m.add_class::<protocol::UserRevokeRep>()?;
    m.add_class::<protocol::DeviceCreateReq>()?;
    m.add_class::<protocol::DeviceCreateRep>()?;
    m.add_class::<protocol::HumanFindReq>()?;
    m.add_class::<protocol::HumanFindRep>()?;
    m.add_class::<protocol::Trustchain>()?;
    m.add_class::<protocol::HumanFindResultItem>()?;
    // Storage
    m.add_class::<storage::WorkspaceStorage>()?;
    Ok(())
}
