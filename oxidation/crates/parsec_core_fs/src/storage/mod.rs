// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

#[allow(dead_code)]
mod chunk_storage;
#[allow(dead_code)]
mod local_database;
#[allow(dead_code)]
mod manifest_storage;
#[allow(dead_code)]
mod user_storage;
#[allow(dead_code)]
mod workspace_storage;

pub(crate) use workspace_storage::WorkspaceStorage;
