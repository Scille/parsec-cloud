// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

#[cfg(not(target_arch = "wasm32"))]
pub use platform_native::{create_context, RuntimeContext};

#[cfg(target_arch = "wasm32")]
pub use platform_web::{create_context, RuntimeContext};

pub use client_types;
pub use core;
pub use core_fs;
pub use crypto;
pub use protocol;
pub use types;
