// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod error;
#[allow(dead_code)]
mod extensions;
mod remote_loader;
mod schema;
mod storage;
mod workspacefs;

#[macro_use]
extern crate diesel;

pub use error::*;
pub use storage::*;

pub enum Language {
    En,
    Fr,
}
