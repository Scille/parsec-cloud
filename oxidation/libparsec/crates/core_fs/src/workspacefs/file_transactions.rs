// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use parsec_api_types::EntryID;
use parsec_client_types::LocalDevice;

use crate::storage::WorkspaceStorage;
use crate::Language;

pub struct FileTransactions {
    /// A stateless class to centralize all file transactions.

    /// The actual state is stored in the local storage and file transactions
    /// have access to the remote loader to download missing resources.

    /// The exposed transactions all take a file descriptor as first argument.
    /// The file descriptors correspond to an entry id which points to a file
    /// on the file system (i.e. a file manifest).

    /// The corresponding file is locked while performing the change (i.e. between
    /// the reading and writing of the corresponding manifest) in order to avoid
    /// race conditions and data corruption.

    /// The table below lists the effects of the 6 file transactions:
    /// - close    -> remove file descriptor from local storage
    /// - write    -> affects file content and possibly file size
    /// - truncate -> affects file size and possibly file content
    /// - read     -> no side effect
    /// - flush    -> no-op
    workspace_id: EntryID,
    device: LocalDevice,
    local_storage: WorkspaceStorage,
    prefered_language: Language,
}
