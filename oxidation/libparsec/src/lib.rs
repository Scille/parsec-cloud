// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

#[cfg(not(any(feature = "native", feature = "web")))]
compile_error!("Feature \"native\" or \"web\" must be enabled for this crate.");

#[cfg(all(feature = "native", feature = "web"))]
compile_error!("Features \"native\" and \"web\" cannot be enabled at the same time.");

#[cfg(feature = "native")]
pub use libparsec_platform_native::{create_context, RuntimeContext};

#[cfg(feature = "web")]
pub use libparsec_platform_web::{create_context, RuntimeContext};

pub use parsec_api_crypto::*;
