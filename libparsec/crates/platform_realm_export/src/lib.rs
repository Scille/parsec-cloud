// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Given a realm export comes as a file database (of typically multiple Go), it
// can only be accessed from the native platform.
#[cfg(target_arch = "wasm32")]
compile_error!("Realm export is not supported on web !");

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use native as platform;

pub use platform::*;
