// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

pub use libparsec_client_types as client_types;
pub use libparsec_core as core;
// TODO: fix compatibility with web build
#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_core_fs as core_fs;
pub use libparsec_protocol as protocol;
pub use libparsec_types as types;

pub use libparsec_crypto as crypto;

// TODO: replace me by the high-level API here ;-)
pub enum HelloError {
    EmptySubject,
    YouAreADog { hello: String },
}
pub fn hello_world(subject: &str) -> Result<String, HelloError> {
    match subject {
        "" => Err(HelloError::EmptySubject),
        "Scruffy" => Err(HelloError::YouAreADog {
            hello: "Woof !".to_owned(),
        }),
        _ => Ok(format!("Hellow {subject}")),
    }
}
