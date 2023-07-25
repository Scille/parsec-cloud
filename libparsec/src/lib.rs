// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[cfg(feature = "python-bindings-support")]
pub mod low_level {
    // LowLevel stuff only exposed for Python bindings
    pub use libparsec_crypto as crypto;
    pub use libparsec_platform_async as platform_async;
    pub use libparsec_protocol as protocol;
    pub use libparsec_serialization_format as serialization_format;
    #[cfg(feature = "test-utils")]
    pub use libparsec_testbed as testbed;
    pub use libparsec_types as types;
}

pub use libparsec_client_high_level_api::*;
#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_platform_async::Runtime as TokioRuntime;
