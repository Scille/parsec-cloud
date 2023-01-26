// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

pub use libparsec_types::{
    BackendOrganizationFileLinkAddr, DateTime, EntryID, EntryName, RealmRole, UserID,
};

use crate::ClientHandle;

// TODO
pub type FsPath = String;

//
// Workspace basics
//

// TODO: should we panic if the client handle is invalid ? (and same question for the workspace id ?)
// This make sens given there is not much to do in this case (there is obviously a bug)
// and the caller will most likely panic itself which will end up with a Js exception.
// The only important thing here is to make sure Sentry is able to log this.
//
// The drawback of panic on bad client handle is to handle concurrency on teardown:
// a call may be done a client that have been closed.
pub enum ClientListWorkspaceError {
    UnknownClient,
}

#[allow(unused)]
pub async fn client_list_workspaces(
    client: ClientHandle,
) -> Result<Vec<(EntryID, EntryName)>, ClientListWorkspaceError> {
    unimplemented!();
}

pub enum ClientCreateWorkspaceError {
    UnknownClient,
}
#[allow(unused)]
pub async fn client_create_workspace(
    client: ClientHandle,
    name: EntryName,
) -> Result<EntryID, ClientCreateWorkspaceError> {
    unimplemented!();
}

pub enum ClientWorkspaceRenameError {
    UnknownClient,
    UnknownWorkspace,
}
#[allow(unused)]
pub async fn client_workspace_rename(
    client: ClientHandle,
    workspace: EntryID,
    new_name: EntryName,
) -> Result<(), ClientWorkspaceRenameError> {
    unimplemented!();
}

pub enum ClientWorkspaceShareError {
    UnknownClient,
    UnknownWorkspace,
    // TODO: server default errors
}
#[allow(unused)]
pub async fn client_workspace_share(
    client: ClientHandle,
    workspace: EntryID,
    recipient: UserID,
    role: Option<RealmRole>,
) -> Result<(), ClientWorkspaceShareError> {
    unimplemented!();
}

pub enum ClientWorkspaceGetRolesError {
    UnknownClient,
    UnknownWorkspace,
}
#[allow(unused)]
pub fn client_workspace_get_roles(
    client: ClientHandle,
    workspace: EntryID,
) -> Result<Vec<(UserID, RealmRole)>, ClientWorkspaceGetRolesError> {
    unimplemented!();
}

//
// Workspace access
//

#[allow(unused)]
pub async fn client_wfs_exists(client: ClientHandle, workspace: EntryID, path: FsPath) -> bool {
    unimplemented!();
}

#[allow(unused)]
pub async fn client_wfs_is_dir(client: ClientHandle, workspace: EntryID, path: FsPath) -> bool {
    unimplemented!();
}

#[allow(unused)]
pub async fn client_wfs_is_file(client: ClientHandle, workspace: EntryID, path: FsPath) -> bool {
    unimplemented!();
}

pub enum FsEntryInfo {
    File { size: u64 },
    Folder { children: Vec<(EntryName, EntryID)> },
}
pub enum ClientWorkspaceFsPathInfoError {
    // TODO: common server errors
    UnknownPath,
}
#[allow(unused)]
pub async fn client_wfs_path_info(
    client: ClientHandle,
    workspace: EntryID,
    path: FsPath,
) -> Result<FsEntryInfo, ClientWorkspaceFsPathInfoError> {
    unimplemented!();
}

pub enum ClientWorkspaceFsListdirError {
    UnknownPath,
    NotAfolder,
}
pub enum FsListDirItem {
    Available { info: FsEntryInfo },
    // This entry is not present on the local cache, `client_wfs_path_info`
    // should be used to retreive it info.
    // We don't try to fetch it info in listdir to return fast so that the GUI
    // can first display what is locally available, then load the rest and
    // update the GUI asynchronously.
    ServerFetchNeeded { name: EntryName },
}
#[allow(unused)]
pub async fn client_wfs_listdir(
    client: ClientHandle,
    workspace: EntryID,
    path: FsPath,
) -> Result<Vec<FsListDirItem>, ClientWorkspaceFsListdirError> {
    unimplemented!();
}

pub enum ClientWorkspaceFsRenameError {
    UnknownPath,
    AlreadyExists,
}
#[allow(unused)]
pub async fn client_wfs_rename(
    client: ClientHandle,
    workspace: EntryID,
    path: FsPath,
    new_name: EntryName,
    overwrite: bool,
) -> Result<(), ClientWorkspaceFsRenameError> {
    unimplemented!();
}

pub enum ClientWorkspaceFsMkdirError {
    UnknownPath,
    AlreadyExists,
}
#[allow(unused)]
pub async fn client_wfs_mkdir(
    client: ClientHandle,
    workspace: EntryID,
    path: FsPath,
    parents: bool,
    exists_ok: bool,
) -> Result<(), ClientWorkspaceFsMkdirError> {
    unimplemented!();
}

pub enum ClientWorkspaceFsRmdirError {
    UnknownPath,
}
#[allow(unused)]
pub async fn client_wfs_rmdir(
    client: ClientHandle,
    workspace: EntryID,
    path: FsPath,
) -> Result<(), ClientWorkspaceFsRmdirError> {
    unimplemented!();
}

pub enum ClientWorkspaceFsTouchError {
    UnknownPath,
    AlreadyExists,
}
#[allow(unused)]
pub async fn client_wfs_touch(
    client: ClientHandle,
    workspace: EntryID,
    path: FsPath,
    exists_ok: bool,
) -> Result<(), ClientWorkspaceFsTouchError> {
    unimplemented!();
}

pub enum ClientWorkspaceFsUnlinkError {
    UnknownPath,
    NotAFile,
}
#[allow(unused)]
pub async fn client_wfs_unlink(
    client: ClientHandle,
    workspace: EntryID,
    path: FsPath,
) -> Result<(), ClientWorkspaceFsUnlinkError> {
    unimplemented!();
}

pub enum ClientWorkspaceFsTruncateError {
    UnknownPath,
    NotAFile,
}
#[allow(unused)]
pub async fn client_wfs_truncate(
    client: ClientHandle,
    workspace: EntryID,
    path: FsPath,
    length: u64,
) -> Result<(), ClientWorkspaceFsTruncateError> {
    unimplemented!();
}

// We don't provide a file descriptor based API here (i.e. open/close/read/write)
// reading/writing the whole file should be enough for most GUI (and testing)
// usecases.
// If anything, a more advanced API should instead introduce a way to ensure
// no modification occurs when reading/writing part of the file. This is
// something that is not covered by file descriptor but which corrupt the file
// when it occurs !

pub enum ClientWorkspaceFsReadError {
    UnknownPath,
    NotAFile,
    FileTooBig, // If max_size is reached
}
#[allow(unused)]
pub async fn client_wfs_read(
    client: ClientHandle,
    workspace: EntryID,
    path: FsPath,
    max_size: Option<u64>,
) -> Result<Vec<u8>, ClientWorkspaceFsReadError> {
    unimplemented!();
}

pub enum ClientWorkspaceFsWriteError {
    UnknownPath,
    NotAFile,
}
#[allow(unused)]
pub async fn client_wfs_write(
    client: ClientHandle,
    workspace: EntryID,
    path: FsPath,
    data: &[u8],
) -> Result<Vec<u8>, ClientWorkspaceFsWriteError> {
    unimplemented!();
}

pub enum ClientWorkspaceFsMoveError {
    UnknownSourcePath,
    UnknownDestinationPath,
    DestinationAlreadyExists,
}
#[allow(unused)]
pub async fn client_wfs_move(
    client: ClientHandle,
    workspace: EntryID,
    source: FsPath,
    destination: FsPath,
) -> Result<(), ClientWorkspaceFsMoveError> {
    unimplemented!();
}

pub enum ClientWorkspaceFsCopytreeError {
    UnknownSourcePath,
    UnknownDestinationPath,
    DestinationAlreadyExists,
}
#[allow(unused)]
pub async fn client_wfs_copytree(
    client: ClientHandle,
    workspace: EntryID,
    source: FsPath,
    destination: FsPath,
) -> Result<(), ClientWorkspaceFsCopytreeError> {
    unimplemented!();
}

pub enum ClientWorkspaceFsCopyfileError {
    NotAFile,
    UnknownSourcePath,
    UnknownDestinationPath,
    DestinationAlreadyExists,
}
#[allow(unused)]
pub async fn client_wfs_copyfile(
    client: ClientHandle,
    workspace: EntryID,
    source: FsPath,
    destination: FsPath,
    buffer_size: Option<u64>,
    exists_ok: bool,
) -> Result<(), ClientWorkspaceFsCopyfileError> {
    unimplemented!();
}

pub enum ClientWorkspaceFsRmtreeError {
    UnknownPath,
    NotAFile,
}
#[allow(unused)]
pub async fn client_wfs_rmtree(
    client: ClientHandle,
    workspace: EntryID,
    path: FsPath,
) -> Result<(), ClientWorkspaceFsRmtreeError> {
    unimplemented!();
}

pub enum ClientWorkspaceFsGenerateFileLinkError {
    UnknownPath,
}
#[allow(unused)]
pub async fn client_wfs_generate_file_link(
    client: ClientHandle,
    workspace: EntryID,
    path: FsPath,
    timestamp: Option<DateTime>,
) -> Result<BackendOrganizationFileLinkAddr, ClientWorkspaceFsGenerateFileLinkError> {
    unimplemented!();
}

pub enum ClientWorkspaceFsDecryptFileLinkPathError {
    UnknownWorkspace,
    DecryptionError,
}
#[allow(unused)]
pub async fn client_wfs_decrypt_file_link(
    client: ClientHandle,
    addr: BackendOrganizationFileLinkAddr,
) -> Result<(FsPath, Option<DateTime>), ClientWorkspaceFsDecryptFileLinkPathError> {
    unimplemented!();
}

//
// Workspace reencryption
//

// Similarly to the get_users function, this can be turned sync if we
// always have up to date all the certificates

pub struct ReencryptionNeed {
    pub user_revoked: Vec<UserID>,
    pub role_revoked: Vec<UserID>,
    pub reencryption_already_in_progress: bool,
}

#[allow(unused)]
pub fn client_workspace_get_reencryption_need(
    client: ClientHandle,
    workspace: EntryID,
) -> Option<ReencryptionNeed> {
    unimplemented!();
}

pub type WorkspaceReencryptionHandle = u32;

// Instead of start/continue + do_a_batch reencryption primitive that we
// currently have, here we provide a much higher level function.
// `client_workspace_reencrypt` should tell the reencryption monitor (a new
// component !) that the workspace is to be reencrypted.
// Reencryption monitor then trigger periodically events to indicate the remaining work.

pub enum ClientWorkspaceStartReencryptionError {
    UnknownWorkspace,
    ReencryptionAlreadyStarted,
}
#[allow(unused)]
pub async fn client_workspace_reencrypt(
    client: ClientHandle,
    workspace: EntryID,
) -> Result<WorkspaceReencryptionHandle, ClientWorkspaceStartReencryptionError> {
    unimplemented!();
}
