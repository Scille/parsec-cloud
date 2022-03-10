#[cfg(not(any(feature = "native", feature = "web")))]
compile_error!("Feature \"native\" or \"web\" must be enabled for this crate.");

#[cfg(all(feature = "native", feature = "web"))]
compile_error!("Features \"native\" and \"web\" cannot be enabled at the same time.");

#[cfg(feature = "native")]
pub use libparsec_platform_native::{RuntimeContext, create_context};

#[cfg(feature = "web")]
pub use libparsec_platform_web::{RuntimeContext, create_context};

pub use libparsec_crypto::*;
