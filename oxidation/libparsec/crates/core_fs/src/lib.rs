// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

#[cfg(test)]
mod conftest;
mod error;
mod extensions;
// TODO: Wait a fix from diesel
#[allow(clippy::extra_unused_lifetimes)]
mod storage;
mod workspacefs;

pub use error::*;
pub use storage::*;
pub use workspacefs::*;

/// Translations used to for File/Folder conflict renaming
pub enum Language {
    En,
    Fr,
}
