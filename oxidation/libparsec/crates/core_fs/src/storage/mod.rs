// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod chunk_storage;
mod local_database;
mod manifest_storage;
mod types;
// Not used for the moment
#[allow(dead_code)]
mod user_storage;
mod version;
mod workspace_storage;

pub use manifest_storage::ChunkOrBlockID;
pub use workspace_storage::*;
