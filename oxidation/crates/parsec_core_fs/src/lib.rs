// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod error;
mod extensions;
mod remote_loader;
mod schema;
mod storage;
mod workspacefs;

#[macro_use]
extern crate diesel;

pub enum Language {
    En,
    Fr,
}
