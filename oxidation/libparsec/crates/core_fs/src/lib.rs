// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod error;
mod extensions;
mod storage;
mod workspacefs;

pub use error::*;
pub use storage::*;

/// Translations used to for File/Folder conflict renaming
pub enum Language {
    En,
    Fr,
}
