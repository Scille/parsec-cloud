// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

#[cfg(not(target_arch = "wasm32"))]
pub use platform_native::{create_context, RuntimeContext};

#[cfg(target_arch = "wasm32")]
pub use platform_web::{create_context, RuntimeContext};

#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_client_types as client_types;
#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_core as core;
#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_core_fs as core_fs;
#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_protocol as protocol;
#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_types as types;

pub use libparsec_crypto as crypto;
