// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod user_storage;
mod version;
mod workspace_storage;

pub use user_storage::{user_storage_non_speculative_init, UserStorageSpecialized};
pub use workspace_storage::{
    workspace_storage_non_speculative_init, LocalFileOrFolderManifest, WorkspaceStorageSpecialized,
    DEFAULT_CHUNK_VACUUM_THRESHOLD, DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE, FAILSAFE_PATTERN_FILTER,
};
