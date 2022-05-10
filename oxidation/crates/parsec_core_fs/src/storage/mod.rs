// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

#[allow(dead_code)]
mod chunk_storage;
#[allow(dead_code)]
mod local_database;
#[allow(dead_code)]
mod manifest_storage;
#[allow(dead_code)]
mod user_storage;
mod version;
mod workspace_storage;

pub use chunk_storage::BlockStorage;
pub use manifest_storage::ChunkOrBlockID;
pub use workspace_storage::*;
