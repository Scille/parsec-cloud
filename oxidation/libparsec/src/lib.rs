// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_platform_native::{create_context, RuntimeContext};

#[cfg(target_arch = "wasm32")]
pub use libparsec_platform_web::{create_context, RuntimeContext};

pub use api_crypto;
