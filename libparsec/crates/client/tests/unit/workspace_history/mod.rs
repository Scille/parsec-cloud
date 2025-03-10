// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod fd_close;
mod fd_read;
mod fd_stat;
mod open_file;
mod open_file_by_id;
// Realm export database support is not available on web.
#[cfg(not(target_arch = "wasm32"))]
mod realm_export_access_sequester_decryptor;
mod start;
mod stat_entry;
mod stat_entry_by_id;
mod stat_folder_children;
mod stat_folder_children_by_id;
mod timestamp_of_interest;
mod utils;
