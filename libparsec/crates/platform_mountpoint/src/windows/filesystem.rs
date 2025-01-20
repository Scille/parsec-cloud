// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use winfsp_wrs::{
    filetime_from_utc, u16cstr, CleanupFlags, CreateFileInfo, CreateOptions, DirInfo,
    FileAccessRights, FileAttributes, FileInfo, FileSystemInterface, PSecurityDescriptor,
    SecurityDescriptor, U16CStr, U16String, VolumeInfo, WriteMode, NTSTATUS, STATUS_ACCESS_DENIED,
    STATUS_DEVICE_NOT_READY, STATUS_DIRECTORY_NOT_EMPTY, STATUS_FILE_IS_A_DIRECTORY,
    STATUS_HOST_UNREACHABLE, STATUS_INVALID_HANDLE, STATUS_MEDIA_WRITE_PROTECTED,
    STATUS_NOT_A_DIRECTORY, STATUS_NO_SUCH_DEVICE, STATUS_OBJECT_NAME_COLLISION,
    STATUS_OBJECT_NAME_INVALID, STATUS_OBJECT_NAME_NOT_FOUND,
};

use libparsec_client::workspace::{
    EntryStat, FileStat, FolderReader, FolderReaderStatEntryError, FolderReaderStatNextOutcome,
    MoveEntryMode, OpenOptions, WorkspaceCreateFolderError, WorkspaceFdFlushError,
    WorkspaceFdReadError, WorkspaceFdResizeError, WorkspaceFdStatError, WorkspaceFdWriteError,
    WorkspaceMoveEntryError, WorkspaceOpenFileError, WorkspaceOpenFolderReaderError, WorkspaceOps,
    WorkspaceStatEntryError,
};
use libparsec_types::prelude::*;

use crate::windows::winify::winify_entry_name;

use super::winify::unwinify_entry_name;

/// we currently don't support arbitrary security descriptor and instead use only this one
/// https://docs.microsoft.com/fr-fr/windows/desktop/SecAuthZ/security-descriptor-string-format
static SECURITY_DESCRIPTOR: Lazy<SecurityDescriptor> = Lazy::new(|| {
    SecurityDescriptor::from_wstr(u16cstr!("O:BAG:BAD:P(A;;FA;;;SY)(A;;FA;;;BA)(A;;FA;;;WD)"))
        .expect("unreachable, valid SecurityDescriptor")
});

// TODO: support entry info

/// Used for the virtual file to retrieve entry info
/// This is used as a way for external applications to get information
/// on a file in a Parsec mountpoint by opening and reading the
/// virtual file f"{file_path}.{ENTRY_INFO_EXTENSION}".
const ENTRY_INFO_EXTENSION: &str = "__parsec_entry_info__";

#[allow(dead_code)]
fn is_entry_info_path(path: &FsPath) -> bool {
    path.parts()
        .last()
        .and_then(|name| name.suffix())
        .map(|suffix| suffix == ENTRY_INFO_EXTENSION)
        .unwrap_or_default()
}

/// In Windows logic opening a file/folder provides an exclusive handle
/// on it so that no concurrent modification can occur.
///
/// `OpenedObj` represents such opened resource.
///
/// Note there is a trick here, as in Parsec not all modifications go through
/// the filesystem (i.e. GUI and remote changes), so a concurrent modification
/// may move the resource (or event modify its type !).
#[derive(Debug)]
pub(crate) enum OpenedObj {
    Folder {
        parsec_file_name: FsPath,
        id: VlobID,
        folder_reader: Option<FolderReader>,
    },
    File {
        // Keeping file name here is useful for debug logs
        #[allow(dead_code)]
        parsec_file_name: FsPath,
        #[allow(dead_code)]
        id: VlobID,
        fd: FileDescriptor,
    },
}

fn parsec_file_stat_to_winfsp_file_info(stat: &FileStat) -> FileInfo {
    let created = filetime_from_utc((stat.created).into());
    let updated = filetime_from_utc((stat.updated).into());
    *FileInfo::default()
        // FILE_ATTRIBUTE_ARCHIVE is a good default attribute
        // This way, we don't need to deal with the weird semantics of
        // FILE_ATTRIBUTE_NORMAL which means "no other attributes is set"
        // Also, this is what the winfsp memfs does.
        .set_file_attributes(FileAttributes::ARCHIVE | FileAttributes::NOT_CONTENT_INDEXED)
        .set_creation_time(created)
        .set_last_access_time(updated)
        .set_last_write_time(updated)
        .set_change_time(updated)
        // TODO: We truncate the EntryID to 64bits as index number, given EntryID is a UUIDv4 this
        // increases the risk of collision... We should investigate to see if this is really needed.
        //
        // Also regarding endianess, `UUID::as_u128` always reads the bytes in big endian,
        // hence the conversion to `u64` left us with the UUID's bytes 8 to 15.
        // There is not much reason (apart for simplicity) as to why those bytes are used
        // given UUIDv4 is just random stuff (well not totally since some bits are used
        // to indicate version, but this is negligeable).
        .set_index_number(stat.id.as_u128() as u64)
        .set_file_size(stat.size)
        // AllocationSize is the size actually occupied on the storage medium, this is
        // meaningless for Parsec (however WinFSP requires FileSize <= AllocationSize)
        .set_allocation_size(stat.size)
}

fn parsec_entry_stat_to_winfsp_file_info(stat: &EntryStat, is_read_only: bool) -> FileInfo {
    // TODO: consider using FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS/FILE_ATTRIBUTE_RECALL_ON_OPEN ?
    // (see https://docs.microsoft.com/en-us/windows/desktop/fileio/file-attribute-constants)
    match stat {
        EntryStat::File {
            id,
            created,
            updated,
            size,
            ..
        } => {
            let created = filetime_from_utc((*created).into());
            let updated = filetime_from_utc((*updated).into());
            *FileInfo::default()
                // FILE_ATTRIBUTE_ARCHIVE is a good default attribute
                // This way, we don't need to deal with the weird semantics of
                // FILE_ATTRIBUTE_NORMAL which means "no other attributes is set"
                // Also, this is what the winfsp memfs does.
                .set_file_attributes(if is_read_only {
                    FileAttributes::ARCHIVE
                        | FileAttributes::NOT_CONTENT_INDEXED
                        | FileAttributes::READONLY
                } else {
                    FileAttributes::ARCHIVE | FileAttributes::NOT_CONTENT_INDEXED
                })
                .set_creation_time(created)
                .set_last_access_time(updated)
                .set_last_write_time(updated)
                .set_change_time(updated)
                // TODO: We truncate the EntryID to 64bits as index number, given EntryID is a UUIDv4 this
                // increases the risk of collision... We should investigate to see if this is really needed.
                .set_index_number(id.as_u128() as u64)
                .set_file_size(*size)
                // AllocationSize is the size actually occupied on the storage medium, this is
                // meaningless for Parsec (however WinFSP requires FileSize <= AllocationSize)
                .set_allocation_size(*size)
        }
        EntryStat::Folder {
            id,
            created,
            updated,
            ..
        } => {
            let created = filetime_from_utc((*created).into());
            let updated = filetime_from_utc((*updated).into());
            *FileInfo::default()
                .set_file_attributes(
                    FileAttributes::DIRECTORY | FileAttributes::NOT_CONTENT_INDEXED,
                )
                .set_creation_time(created)
                .set_last_access_time(updated)
                .set_last_write_time(updated)
                .set_change_time(updated)
                // TODO: We truncate the EntryID to 64bits as index number, given EntryID is a UUIDv4 this
                // increases the risk of collision... We should investigate to see if this is really needed.
                .set_index_number(id.as_u128() as u64)
        }
    }
}

#[cfg(test)]
#[allow(clippy::type_complexity)]
// TODO
#[allow(dead_code)]
pub(crate) static LOOKUP_HOOK: Mutex<
    Option<Box<dyn FnMut(&FsPath) -> Option<Result<EntryStat, WorkspaceStatEntryError>> + Send>>,
> = Mutex::new(None);

#[derive(Debug)]
pub(crate) struct ParsecFileSystemInterface {
    is_read_only: bool,
    ops: Arc<WorkspaceOps>,
    tokio_handle: tokio::runtime::Handle,
    volume_label: Mutex<U16String>,
}

impl ParsecFileSystemInterface {
    pub fn new(
        is_read_only: bool,
        ops: Arc<WorkspaceOps>,
        tokio_handle: tokio::runtime::Handle,
        volume_label: U16String,
    ) -> Self {
        Self {
            is_read_only,
            ops,
            tokio_handle,
            volume_label: Mutex::new(volume_label),
        }
    }
}

impl ParsecFileSystemInterface {
    async fn get_file_info_async(&self, file_context: &OpenedObj) -> Result<FileInfo, NTSTATUS> {
        match file_context {
            OpenedObj::File { fd, .. } => {
                let outcome = self.ops.fd_stat(*fd).await;
                match outcome {
                    Ok(stat) => Ok(parsec_file_stat_to_winfsp_file_info(&stat)),
                    Err(err) => Err(match err {
                        WorkspaceFdStatError::BadFileDescriptor => STATUS_INVALID_HANDLE,
                        WorkspaceFdStatError::Internal(_) => STATUS_ACCESS_DENIED,
                    }),
                }
            }

            OpenedObj::Folder { id, .. } => {
                // Confinement point information is unused here so ignore it
                let outcome = self
                    .ops
                    .stat_entry_by_id_ignore_confinement_point(*id)
                    .await;
                match outcome {
                    Ok(stat) => Ok(parsec_entry_stat_to_winfsp_file_info(
                        &stat,
                        self.is_read_only,
                    )),
                    Err(err) => Err(match err {
                        WorkspaceStatEntryError::EntryNotFound => STATUS_OBJECT_NAME_NOT_FOUND,
                        WorkspaceStatEntryError::Offline => STATUS_HOST_UNREACHABLE,
                        WorkspaceStatEntryError::Stopped => STATUS_DEVICE_NOT_READY,
                        WorkspaceStatEntryError::NoRealmAccess
                        | WorkspaceStatEntryError::InvalidKeysBundle(_)
                        | WorkspaceStatEntryError::InvalidCertificate(_)
                        | WorkspaceStatEntryError::InvalidManifest(_)
                        | WorkspaceStatEntryError::Internal(_) => STATUS_ACCESS_DENIED,
                    }),
                }
            }
        }
    }
}

impl FileSystemInterface for ParsecFileSystemInterface {
    type FileContext = Arc<Mutex<OpenedObj>>;

    const GET_VOLUME_INFO_DEFINED: bool = true;
    fn get_volume_info(&self) -> Result<VolumeInfo, NTSTATUS> {
        log::debug!("[WinFSP] get_volume_info()");

        // We have currently no way of easily getting the size of workspace
        // Also, the total size of a workspace is not limited
        // For the moment let's settle on 0 MB used for 1 TB available
        Ok(VolumeInfo::new(
            1024u64.pow(4), // 1 TB
            1024u64.pow(4), // 1 TB
            &self.volume_label.lock().expect("mutex is poisoned"),
        )
        .expect("volume label length already checked"))
    }

    const SET_VOLUME_LABEL_DEFINED: bool = true;
    fn set_volume_label(&self, volume_label: &U16CStr) -> Result<VolumeInfo, NTSTATUS> {
        log::debug!("[WinFSP] get_volume_info(volume_label: {:?})", volume_label);

        let mut guard = self.volume_label.lock().expect("mutex is poisoned");
        guard.clear();
        guard.push(volume_label);

        self.get_volume_info()
    }

    const GET_SECURITY_BY_NAME_DEFINED: bool = true;
    fn get_security_by_name(
        &self,
        file_name: &U16CStr,
        _find_reparse_point: impl Fn() -> Option<FileAttributes>,
    ) -> Result<(FileAttributes, PSecurityDescriptor, bool), NTSTATUS> {
        log::debug!("[WinFSP] get_security_by_name(file_name: {:?})", file_name);

        let path = os_path_to_parsec_path(file_name)?;

        self.tokio_handle.block_on(async move {
            let file_attributes = match self.ops.stat_entry(&path).await {
                Ok(stat) => parsec_entry_stat_to_winfsp_file_info(&stat, self.is_read_only)
                    .file_attributes(),
                Err(err) => {
                    return Err(match err {
                        WorkspaceStatEntryError::EntryNotFound => STATUS_OBJECT_NAME_NOT_FOUND,
                        WorkspaceStatEntryError::Offline => STATUS_HOST_UNREACHABLE,
                        WorkspaceStatEntryError::Stopped => STATUS_DEVICE_NOT_READY,
                        WorkspaceStatEntryError::NoRealmAccess
                        | WorkspaceStatEntryError::InvalidKeysBundle(_)
                        | WorkspaceStatEntryError::InvalidCertificate(_)
                        | WorkspaceStatEntryError::InvalidManifest(_)
                        | WorkspaceStatEntryError::Internal(_) => STATUS_ACCESS_DENIED,
                    })
                }
            };

            Ok((file_attributes, SECURITY_DESCRIPTOR.as_ptr(), false))
        })
    }

    const CREATE_EX_DEFINED: bool = true;
    fn create_ex(
        &self,
        file_name: &U16CStr,
        create_file_info: CreateFileInfo,
        security_descriptor: SecurityDescriptor,
        // Buffer contains extended attributes or reparse point, so we can ignore it
        _buffer: &[u8],
        _extra_buffer_is_reparse_point: bool,
    ) -> Result<(Self::FileContext, FileInfo), NTSTATUS> {
        log::debug!(
            "[WinFSP] create(file_name: {:?}, create_file_info: {:?}, security_descriptor: {:?})",
            file_name,
            create_file_info,
            security_descriptor
        );

        // `security_descriptor` is not supported yet
        // `reparse_point` is not supported yet
        self.tokio_handle.block_on(async move {
            let parsec_file_name = os_path_to_parsec_path(file_name)?;

            if create_file_info
                .create_options
                .is(CreateOptions::FILE_DIRECTORY_FILE)
            {
                let id = self
                    .ops
                    .create_folder(parsec_file_name.clone())
                    .await
                    .map_err(|err| match err {
                        WorkspaceCreateFolderError::Offline => STATUS_HOST_UNREACHABLE,
                        WorkspaceCreateFolderError::Stopped => STATUS_DEVICE_NOT_READY,
                        WorkspaceCreateFolderError::ParentNotFound => STATUS_OBJECT_NAME_NOT_FOUND,
                        WorkspaceCreateFolderError::ParentNotAFolder => {
                            STATUS_OBJECT_NAME_NOT_FOUND
                        }
                        WorkspaceCreateFolderError::EntryExists { .. } => {
                            STATUS_OBJECT_NAME_COLLISION
                        }
                        // WinFSP lacks a proper read-only support, so we will receive write
                        // operations no matter what (see https://github.com/winfsp/winfsp/issues/84)
                        //
                        // Using `STATUS_MEDIA_WRITE_PROTECTED` here causes a "Catastrophic Failure"
                        // error in Windows Explorer, so instead we must use `STATUS_ACCESS_DENIED`.
                        //
                        // TODO: `STATUS_ACCESS_DENIED` causes a "You need to confirm this operation" dialogue
                        // in Windows Explorer, which is obviously not the best user experience :/
                        WorkspaceCreateFolderError::ReadOnlyRealm => STATUS_ACCESS_DENIED,
                        WorkspaceCreateFolderError::NoRealmAccess
                        | WorkspaceCreateFolderError::InvalidKeysBundle(_)
                        | WorkspaceCreateFolderError::InvalidCertificate(_)
                        | WorkspaceCreateFolderError::InvalidManifest(_)
                        | WorkspaceCreateFolderError::Internal(_) => STATUS_ACCESS_DENIED,
                    })?;

                let opened_obj = OpenedObj::Folder {
                    parsec_file_name,
                    id,
                    folder_reader: None,
                };
                let file_info = self.get_file_info_async(&opened_obj).await?;
                let file_context = Arc::new(Mutex::new(opened_obj));

                Ok((file_context, file_info))
            } else {
                // TODO: would be useful to use `CreateFileInfo::allocation_size` to settle the final blocksize
                let options = OpenOptions {
                    read: true,
                    write: true,
                    truncate: false,
                    create: true,
                    create_new: true,
                };
                let (fd, id) = self
                    .ops
                    .open_file_and_get_id(parsec_file_name.clone(), options)
                    .await
                    .map_err(|err| match err {
                        WorkspaceOpenFileError::Offline => STATUS_HOST_UNREACHABLE,
                        WorkspaceOpenFileError::Stopped => STATUS_DEVICE_NOT_READY,
                        WorkspaceOpenFileError::EntryNotFound => STATUS_OBJECT_NAME_NOT_FOUND,
                        WorkspaceOpenFileError::EntryNotAFile { .. } => STATUS_FILE_IS_A_DIRECTORY,
                        // WinFSP lacks a proper read-only support, so we will receive write
                        // operations no matter what (see https://github.com/winfsp/winfsp/issues/84)
                        //
                        // Using `STATUS_MEDIA_WRITE_PROTECTED` here causes a "Catastrophic Failure"
                        // error in Windows Explorer, so instead we must use `STATUS_ACCESS_DENIED`.
                        //
                        // TODO: `STATUS_ACCESS_DENIED` causes a "You need to confirm this operation" dialogue
                        // in Windows Explorer, which is obviously not the best user experience :/
                        WorkspaceOpenFileError::ReadOnlyRealm => STATUS_ACCESS_DENIED,
                        WorkspaceOpenFileError::EntryExistsInCreateNewMode { .. } => {
                            STATUS_OBJECT_NAME_COLLISION
                        }
                        WorkspaceOpenFileError::NoRealmAccess
                        | WorkspaceOpenFileError::InvalidKeysBundle(_)
                        | WorkspaceOpenFileError::InvalidCertificate(_)
                        | WorkspaceOpenFileError::InvalidManifest(_)
                        | WorkspaceOpenFileError::Internal(_) => STATUS_ACCESS_DENIED,
                    })?;

                let opened_obj = OpenedObj::File {
                    parsec_file_name,
                    id,
                    fd,
                };
                let file_info = self.get_file_info_async(&opened_obj).await?;
                let file_context = Arc::new(Mutex::new(opened_obj));

                Ok((file_context, file_info))
            }
        })
    }

    const OPEN_DEFINED: bool = true;
    fn open(
        &self,
        file_name: &U16CStr,
        create_options: CreateOptions,
        granted_access: FileAccessRights,
    ) -> Result<(Self::FileContext, FileInfo), NTSTATUS> {
        log::debug!(
            "[WinFSP] open(file_name: {:?}, create_option: {:x?}, granted_access: {:x?})",
            file_name,
            create_options,
            granted_access
        );

        // `granted_access` is already handle by WinFSP

        self.tokio_handle.block_on(async move {
            let write_mode = granted_access.is(FileAccessRights::FILE_WRITE_DATA)
                || granted_access.is(FileAccessRights::FILE_APPEND_DATA);
            let read_mode = granted_access.is(FileAccessRights::FILE_READ_DATA);
            let parsec_file_name = os_path_to_parsec_path(file_name)?;

            // TODO
            // if !write_mode && is_entry_info_path(&parsec_file_name) {
            //     let outcome = self.ops.stat_entry(&parsec_file_name).await;
            //     return match outcome {
            //         Ok(stat) => Ok(Arc::new(Mutex::new(OpenedObj::EntryInfo {
            //             parsec_file_name,
            //             info: parsec_entry_stat_to_winfsp_file_info(&stat, , self.is_read_only),
            //         }))),
            //         Err(err) => Err(match err {
            //             WorkspaceStatEntryError::EntryNotFound => STATUS_OBJECT_NAME_NOT_FOUND,
            //             WorkspaceStatEntryError::Offline => STATUS_HOST_UNREACHABLE,
            //             WorkspaceStatEntryError::Stopped => STATUS_DEVICE_NOT_READY,
            //             WorkspaceStatEntryError::NoRealmAccess
            //             | WorkspaceStatEntryError::InvalidKeysBundle(_)
            //             | WorkspaceStatEntryError::InvalidCertificate(_)
            //             | WorkspaceStatEntryError::InvalidManifest(_)
            //             | WorkspaceStatEntryError::Internal(_) => STATUS_ACCESS_DENIED,
            //         }),
            //     };
            // }

            let options = OpenOptions {
                read: read_mode,
                write: write_mode,
                truncate: false,
                create: false,
                create_new: false,
            };
            let outcome = self
                .ops
                .open_file_and_get_id(parsec_file_name.clone(), options)
                .await;

            let opened_obj = match outcome {
                Ok((fd, id)) => Ok(OpenedObj::File {
                    parsec_file_name,
                    id,
                    fd,
                }),
                Err(err) => match err {
                    WorkspaceOpenFileError::EntryNotAFile { entry_id } => Ok(OpenedObj::Folder {
                        parsec_file_name,
                        id: entry_id,
                        folder_reader: None,
                    }),
                    WorkspaceOpenFileError::Offline => Err(STATUS_HOST_UNREACHABLE),
                    WorkspaceOpenFileError::Stopped => Err(STATUS_NO_SUCH_DEVICE),
                    WorkspaceOpenFileError::EntryNotFound => Err(STATUS_OBJECT_NAME_NOT_FOUND),
                    // WinFSP lacks a proper read-only support, so we will receive write
                    // operations no matter what (see https://github.com/winfsp/winfsp/issues/84)
                    WorkspaceOpenFileError::ReadOnlyRealm => Err(STATUS_MEDIA_WRITE_PROTECTED),
                    WorkspaceOpenFileError::NoRealmAccess
                    | WorkspaceOpenFileError::InvalidKeysBundle(_)
                    | WorkspaceOpenFileError::InvalidCertificate(_)
                    | WorkspaceOpenFileError::InvalidManifest(_)
                    | WorkspaceOpenFileError::Internal(_) => Err(STATUS_ACCESS_DENIED),
                    WorkspaceOpenFileError::EntryExistsInCreateNewMode { .. } => unreachable!(),
                },
            }?;

            let file_info = self.get_file_info_async(&opened_obj).await?;
            let file_context = Arc::new(Mutex::new(opened_obj));

            Ok((file_context, file_info))
        })
    }

    const OVERWRITE_EX_DEFINED: bool = true;
    fn overwrite_ex(
        &self,
        file_context: Self::FileContext,
        file_attributes: FileAttributes,
        replace_file_attributes: bool,
        allocation_size: u64,
        buffer: &[u8],
    ) -> Result<FileInfo, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        log::debug!(
            "[WinFSP] overwrite(file_context: {:?}, file_attributes: {:?}, replace_file_attributes: {:?}, allocation_size: {:?}, buffer_size: {})",
            fc, file_attributes, replace_file_attributes, allocation_size, buffer.len()
        );

        self.tokio_handle.block_on(async move {
            // The file might be deleted at this point. This is fine though as the
            // file descriptor can still be used after a deletion (posix style)
            if let OpenedObj::File { fd, .. } = &*fc {
                self.ops
                    .fd_resize(*fd, allocation_size, true)
                    .await
                    .map_err(|err| match err {
                        WorkspaceFdResizeError::BadFileDescriptor => STATUS_INVALID_HANDLE,
                        WorkspaceFdResizeError::NotInWriteMode => STATUS_ACCESS_DENIED,
                        WorkspaceFdResizeError::Internal(_) => STATUS_ACCESS_DENIED,
                    })?;

                self.get_file_info_async(&fc).await
            } else {
                Err(STATUS_FILE_IS_A_DIRECTORY)
            }
        })
    }

    const CLEANUP_DEFINED: bool = true;
    fn cleanup(
        &self,
        file_context: Self::FileContext,
        file_name: Option<&U16CStr>,
        flags: CleanupFlags,
    ) {
        let fc = file_context.lock().expect("Mutex is poisoned");
        log::debug!(
            "[WinFSP] cleanup(file_context: {:?}, file_name: {:?}, flags: {:x?})",
            fc,
            file_name,
            flags
        );

        self.tokio_handle.block_on(async move {
            // The file name is only provided for a delete operation, it is `None` otherwise
            if let Some(file_name) = file_name {
                let parsec_file_name = match os_path_to_parsec_path(file_name) {
                    Ok(name) => name,
                    Err(_) => return,
                };

                // Cleanup operation is causal but close is not, so it's important
                // to delete file and folder here in order to make sure the file/folder
                // is actually deleted by the time the API call returns.
                if flags.is(CleanupFlags::DELETE) {
                    // In Windows logic when we are here we have an exclusive handle on
                    // the resource (i.e. the file/folder that have been opened), so
                    // no concurrent modification can occur and hence we know precisely
                    // what is the resource (file or folder with/without children, see
                    // `can_delete` method).
                    //
                    // BUT ! In Parsec not all modifications go through the filesystem
                    // (i.e. GUI and remote changes), so a concurrent modification may
                    // break Windows's assumptions...
                    //
                    // For this reason the remove can fail here, in which case we can
                    // just ignore it: the final situation is similar to what would
                    // have occured if the removal had occured first, only for *then*
                    // the concurrent change to arrive.
                    match &*fc {
                        OpenedObj::Folder { .. } => {
                            let _ = self.ops.remove_folder(parsec_file_name).await;
                        }
                        OpenedObj::File { .. } => {
                            let _ = self.ops.remove_file(parsec_file_name).await;
                        }
                    }
                }
            }
        })
    }

    const CLOSE_DEFINED: bool = true;
    fn close(&self, file_context: Self::FileContext) {
        let fc = file_context.lock().expect("Mutex is poisoned");
        log::debug!("[WinFSP] close(file_context: {:?})", fc);

        // The file might be deleted at this point. This is fine though as the
        // file descriptor can still be used after a deletion (posix style)
        if let OpenedObj::File { fd, .. } = &*fc {
            self.tokio_handle.block_on(async move {
                let _ = self.ops.fd_close(*fd).await;
            })
        }
    }

    const READ_DEFINED: bool = true;
    fn read(
        &self,
        file_context: Self::FileContext,
        mut buffer: &mut [u8],
        offset: u64,
    ) -> Result<usize, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        log::debug!(
            "[WinFSP] read(file_context: {:?}, buffer_size: {}, offset: {:?})",
            fc,
            buffer.len(),
            offset
        );

        self.tokio_handle.block_on(async move {
            // The file might be deleted at this point. This is fine though as the
            // file descriptor can still be used after a deletion (posix style)
            if let OpenedObj::File { fd, .. } = &*fc {
                self.ops
                    .fd_read(*fd, offset, buffer.len() as u64, &mut buffer)
                    .await
                    .map_err(|err| match err {
                        WorkspaceFdReadError::Offline => STATUS_HOST_UNREACHABLE,
                        WorkspaceFdReadError::NoRealmAccess => STATUS_ACCESS_DENIED,
                        WorkspaceFdReadError::Stopped => STATUS_NO_SUCH_DEVICE,
                        WorkspaceFdReadError::BadFileDescriptor => STATUS_INVALID_HANDLE,
                        WorkspaceFdReadError::NotInReadMode => STATUS_ACCESS_DENIED,
                        WorkspaceFdReadError::InvalidBlockAccess(_)
                        | WorkspaceFdReadError::InvalidKeysBundle(_)
                        | WorkspaceFdReadError::InvalidCertificate(_)
                        | WorkspaceFdReadError::Internal(_) => STATUS_ACCESS_DENIED,
                    })
                    .map(|read| read as usize)
            } else {
                Err(STATUS_FILE_IS_A_DIRECTORY)
            }
        })
    }

    const WRITE_DEFINED: bool = true;
    fn write(
        &self,
        file_context: Self::FileContext,
        buffer: &[u8],
        mode: WriteMode,
    ) -> Result<(usize, FileInfo), NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        // TODO: WriteMode is not Debug yet
        log::debug!(
            "[WinFSP] write(file_context: {:?}, buffer.len(): {}, mode: {:?})",
            fc,
            buffer.len(),
            mode,
        );

        self.tokio_handle.block_on(async move {
            // The file might be deleted at this point. This is fine though as the
            // file descriptor can still be used after a deletion (posix style)
            if let OpenedObj::File { fd, .. } = &*fc {
                let outcome = match mode {
                    WriteMode::Normal { offset } => self.ops.fd_write(*fd, offset, buffer).await,
                    WriteMode::ConstrainedIO { offset } => {
                        self.ops.fd_write_constrained_io(*fd, offset, buffer).await
                    }
                    WriteMode::WriteToEOF => self.ops.fd_write_start_eof(*fd, buffer).await,
                };

                let written = outcome
                    .map_err(|err| match err {
                        WorkspaceFdWriteError::BadFileDescriptor => STATUS_INVALID_HANDLE,
                        WorkspaceFdWriteError::NotInWriteMode => STATUS_ACCESS_DENIED,
                        WorkspaceFdWriteError::Internal(_) => STATUS_ACCESS_DENIED,
                    })
                    .map(|written| written as usize)?;

                let file_info = self.get_file_info_async(&fc).await?;

                Ok((written, file_info))
            } else {
                Err(STATUS_FILE_IS_A_DIRECTORY)
            }
        })
    }

    const FLUSH_DEFINED: bool = true;
    fn flush(&self, file_context: Self::FileContext) -> Result<FileInfo, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        log::debug!("[WinFSP] flush(file_context: {:?})", fc);

        self.tokio_handle.block_on(async move {
            // The file might be deleted at this point. This is fine though as the
            // file descriptor can still be used after a deletion (posix style)
            if let OpenedObj::File { fd, .. } = &*fc {
                self.ops.fd_flush(*fd).await.map_err(|err| match err {
                    WorkspaceFdFlushError::Stopped => STATUS_NO_SUCH_DEVICE,
                    WorkspaceFdFlushError::BadFileDescriptor => STATUS_INVALID_HANDLE,
                    WorkspaceFdFlushError::NotInWriteMode => STATUS_ACCESS_DENIED,
                    WorkspaceFdFlushError::Internal(_) => STATUS_ACCESS_DENIED,
                })?;

                self.get_file_info_async(&fc).await
            } else {
                Err(STATUS_FILE_IS_A_DIRECTORY)
            }
        })
    }

    const GET_FILE_INFO_DEFINED: bool = true;
    fn get_file_info(&self, file_context: Self::FileContext) -> Result<FileInfo, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        log::debug!("[WinFSP] get_file_info(file_context: {:?})", fc);

        self.tokio_handle
            .block_on(async move { self.get_file_info_async(&fc).await })
    }

    const SET_BASIC_INFO_DEFINED: bool = true;
    fn set_basic_info(
        &self,
        file_context: Self::FileContext,
        file_attributes: FileAttributes,
        creation_time: u64,
        last_access_time: u64,
        last_write_time: u64,
        change_time: u64,
    ) -> Result<FileInfo, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        log::debug!(
            "[WinFSP] set_basic_info(file_context: {:?}, file_attributes: {:?}, creation_time: {:?}, last_access_time: {:?}, last_write_time: {:?}, change_time: {:?})",
            fc,
            file_attributes,
            creation_time,
            last_access_time,
            last_write_time,
            change_time,
        );

        // Note this method actually does nothing !
        // But if we don't define it, `explorer.exe` displays "Invalid MS-DOS function" errors
        // when copying some directory (see https://github.com/Scille/parsec-cloud/issues/7640).

        self.tokio_handle
            .block_on(async move { self.get_file_info_async(&fc).await })
    }

    const SET_FILE_SIZE_DEFINED: bool = true;
    fn set_file_size(
        &self,
        file_context: Self::FileContext,
        new_size: u64,
        set_allocation_size: bool,
    ) -> Result<FileInfo, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        log::debug!(
            "[WinFSP] set_file_size(file_context: {:?}, new_size: {}, set_allocation_size: {})",
            fc,
            new_size,
            set_allocation_size
        );

        self.tokio_handle.block_on(async move {
            // The file might be deleted at this point. This is fine though as the
            // file descriptor can still be used after a deletion (posix style)
            if let OpenedObj::File { fd, .. } = &*fc {
                self.ops
                    .fd_resize(*fd, new_size, set_allocation_size)
                    .await
                    .map_err(|err| match err {
                        WorkspaceFdResizeError::BadFileDescriptor => STATUS_INVALID_HANDLE,
                        WorkspaceFdResizeError::NotInWriteMode => STATUS_ACCESS_DENIED,
                        WorkspaceFdResizeError::Internal(_) => STATUS_ACCESS_DENIED,
                    })?;
                self.ops.fd_flush(*fd).await.map_err(|err| match err {
                    WorkspaceFdFlushError::Stopped => STATUS_DEVICE_NOT_READY,
                    WorkspaceFdFlushError::BadFileDescriptor => STATUS_INVALID_HANDLE,
                    WorkspaceFdFlushError::NotInWriteMode => STATUS_ACCESS_DENIED,
                    WorkspaceFdFlushError::Internal(_) => STATUS_ACCESS_DENIED,
                })?;

                self.get_file_info_async(&fc).await
            } else {
                Err(STATUS_FILE_IS_A_DIRECTORY)
            }
        })
    }

    const CAN_DELETE_DEFINED: bool = true;
    fn can_delete(
        &self,
        file_context: Self::FileContext,
        file_name: &U16CStr,
    ) -> Result<(), NTSTATUS> {
        let mut fc = file_context.lock().expect("Mutex is poisoned");
        log::debug!(
            "[WinFSP] can_delete(file_context: {:?}, file_name: {:?})",
            fc,
            file_name
        );

        // Note Parsec doesn't provide any guarantee against concurrent modification.
        // Hence this is implemented in "best-effort" mode and may lead to the
        // removal of new changes (see the `cleanup` implementation for more info).

        self.tokio_handle.block_on(async move {
            if !self.ops.get_current_name_and_self_role().1.can_write() {
                // WinFSP lacks a proper read-only support, so we will receive write
                // operations no matter what (see https://github.com/winfsp/winfsp/issues/84)
                return Err(STATUS_MEDIA_WRITE_PROTECTED);
            }

            let reader = match &mut *fc {
                OpenedObj::File { .. } => return Ok(()),

                OpenedObj::Folder {
                    parsec_file_name,
                    id,
                    ..
                } => {
                    if parsec_file_name.is_root() {
                        // Cannot remove root mountpoint !
                        return Err(STATUS_ACCESS_DENIED);
                    }

                    self.ops
                        .open_folder_reader_by_id(*id)
                        .await
                        .map_err(|err| match err {
                            // Concurrent modification may have changed the type of the entry since it has been opened.
                            // In this case we shouldn't return ok since we know the opened object and no longer
                            // corresponds to what Parsec contains, and hence we would remove the wrong thing !
                            WorkspaceOpenFolderReaderError::EntryIsFile => {
                                STATUS_OBJECT_NAME_NOT_FOUND
                            }

                            WorkspaceOpenFolderReaderError::Offline => STATUS_HOST_UNREACHABLE,
                            WorkspaceOpenFolderReaderError::Stopped => STATUS_DEVICE_NOT_READY,
                            WorkspaceOpenFolderReaderError::EntryNotFound => {
                                STATUS_OBJECT_NAME_NOT_FOUND
                            }
                            WorkspaceOpenFolderReaderError::NoRealmAccess
                            | WorkspaceOpenFolderReaderError::InvalidKeysBundle(_)
                            | WorkspaceOpenFolderReaderError::InvalidCertificate(_)
                            | WorkspaceOpenFolderReaderError::InvalidManifest(_)
                            | WorkspaceOpenFolderReaderError::Internal(_) => STATUS_ACCESS_DENIED,
                        })?
                }
            };

            let mut index = 0;
            loop {
                let outcome =
                    reader
                        .stat_child(&self.ops, index)
                        .await
                        .map_err(|err| match err {
                            FolderReaderStatEntryError::Offline => STATUS_HOST_UNREACHABLE,
                            FolderReaderStatEntryError::Stopped => STATUS_DEVICE_NOT_READY,
                            FolderReaderStatEntryError::NoRealmAccess
                            | FolderReaderStatEntryError::InvalidKeysBundle(_)
                            | FolderReaderStatEntryError::InvalidCertificate(_)
                            | FolderReaderStatEntryError::InvalidManifest(_)
                            | FolderReaderStatEntryError::Internal(_) => STATUS_ACCESS_DENIED,
                        })?;
                match outcome {
                    FolderReaderStatNextOutcome::NoMoreEntries => break Ok(()),
                    FolderReaderStatNextOutcome::Entry { .. } => {
                        break Err(STATUS_DIRECTORY_NOT_EMPTY)
                    }
                    // Current entry is invalid, just ignore it
                    FolderReaderStatNextOutcome::InvalidChild => {
                        index += 1;
                        continue;
                    }
                }
            }
        })
    }

    const RENAME_DEFINED: bool = true;
    fn rename(
        &self,
        file_context: Self::FileContext,
        file_name: &U16CStr,
        new_file_name: &U16CStr,
        replace_if_exists: bool,
    ) -> Result<(), NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        log::debug!("[WinFSP] rename(file_context: {:?}, file_name: {:?}, new_file_name: {:?}, replace_if_exists: {:?})", fc, file_name, new_file_name, replace_if_exists);

        // `granted_access` is already handle by WinFSP

        self.tokio_handle.block_on(async move {
            let parsec_file_name = os_path_to_parsec_path(file_name)?;
            let parsec_new_file_name = os_path_to_parsec_path(new_file_name)?;

            let mode = if replace_if_exists {
                MoveEntryMode::CanReplaceFileOnly
            } else {
                MoveEntryMode::NoReplace
            };
            self.ops
                .move_entry(parsec_file_name, parsec_new_file_name, mode)
                .await
                .map_err(|err| match err {
                    WorkspaceMoveEntryError::SourceNotFound => STATUS_OBJECT_NAME_NOT_FOUND,
                    WorkspaceMoveEntryError::DestinationNotFound => STATUS_OBJECT_NAME_NOT_FOUND,
                    WorkspaceMoveEntryError::CannotMoveRoot => STATUS_ACCESS_DENIED,
                    WorkspaceMoveEntryError::DestinationExists { .. } => {
                        if replace_if_exists {
                            // Occurs if the destination is a folder
                            STATUS_ACCESS_DENIED
                        } else {
                            STATUS_OBJECT_NAME_COLLISION
                        }
                    }
                    WorkspaceMoveEntryError::Offline => STATUS_HOST_UNREACHABLE,
                    WorkspaceMoveEntryError::Stopped => STATUS_DEVICE_NOT_READY,
                    // WinFSP lacks a proper read-only support, so we will receive write
                    // operations no matter what (see https://github.com/winfsp/winfsp/issues/84)
                    WorkspaceMoveEntryError::ReadOnlyRealm => STATUS_MEDIA_WRITE_PROTECTED,
                    WorkspaceMoveEntryError::NoRealmAccess
                    | WorkspaceMoveEntryError::InvalidKeysBundle(_)
                    | WorkspaceMoveEntryError::InvalidCertificate(_)
                    | WorkspaceMoveEntryError::InvalidManifest(_)
                    | WorkspaceMoveEntryError::Internal(_) => STATUS_ACCESS_DENIED,
                })
        })
    }

    const READ_DIRECTORY_DEFINED: bool = true;
    fn read_directory(
        &self,
        file_context: Self::FileContext,
        marker: Option<&U16CStr>,
        mut add_dir_info: impl FnMut(DirInfo) -> bool,
    ) -> Result<(), NTSTATUS> {
        let mut fc = file_context.lock().expect("Mutex is poisoned");
        log::debug!(
            "[WinFSP] read_directory(file_context: {:?}, marker: {:?})",
            fc,
            marker
        );

        self.tokio_handle.block_on(async move {
            let (is_root, reader) = match &mut *fc {
                OpenedObj::Folder {
                    parsec_file_name,
                    id,
                    folder_reader: folder_reader @ None,
                } => {
                    let reader = self
                        .ops
                        .open_folder_reader_by_id(*id)
                        .await
                        .map_err(|err| match err {
                            // Concurrent modification may have changed the type of the entry since it has been opened
                            WorkspaceOpenFolderReaderError::EntryIsFile => STATUS_NOT_A_DIRECTORY,

                            WorkspaceOpenFolderReaderError::Offline => STATUS_HOST_UNREACHABLE,
                            WorkspaceOpenFolderReaderError::Stopped => STATUS_DEVICE_NOT_READY,
                            WorkspaceOpenFolderReaderError::EntryNotFound => {
                                STATUS_OBJECT_NAME_NOT_FOUND
                            }
                            WorkspaceOpenFolderReaderError::NoRealmAccess
                            | WorkspaceOpenFolderReaderError::InvalidKeysBundle(_)
                            | WorkspaceOpenFolderReaderError::InvalidCertificate(_)
                            | WorkspaceOpenFolderReaderError::InvalidManifest(_)
                            | WorkspaceOpenFolderReaderError::Internal(_) => STATUS_ACCESS_DENIED,
                        })?;
                    *folder_reader = Some(reader);
                    (
                        parsec_file_name.is_root(),
                        folder_reader.as_ref().expect("set above"),
                    )
                }

                OpenedObj::Folder {
                    folder_reader: Some(folder_reader),
                    parsec_file_name,
                    ..
                } => (parsec_file_name.is_root(), &*folder_reader),

                OpenedObj::File { .. } => return Err(STATUS_NOT_A_DIRECTORY),
            };

            // NOTE: The "." and ".." directories should ONLY be included
            // if the queried directory is not root
            // (see https://github.com/winfsp/winfsp/blob/507c7944709e42b909c3f0c363d78b62c530ce7f/tst/memfs/memfs.cpp#L1901-L1920)

            if !is_root {
                if marker.is_none() {
                    let directory_stat = reader.stat_folder();

                    if !add_dir_info(DirInfo::new(
                        parsec_entry_stat_to_winfsp_file_info(&directory_stat, self.is_read_only),
                        u16cstr!("."),
                    )) {
                        return Ok(());
                    }
                }

                if marker.is_none() || marker == Some(u16cstr!(".")) {
                    let directory_parent_id = match reader.stat_folder() {
                        EntryStat::File { parent, .. } => parent,
                        EntryStat::Folder { parent, .. } => parent,
                    };

                    let parent_stat = self
                        .ops
                        // Confinement point information is unused here so ignore it
                        .stat_entry_by_id_ignore_confinement_point(directory_parent_id)
                        .await
                        .map_err(|err| match err {
                            WorkspaceStatEntryError::Offline => STATUS_HOST_UNREACHABLE,
                            WorkspaceStatEntryError::Stopped => STATUS_DEVICE_NOT_READY,
                            WorkspaceStatEntryError::EntryNotFound => STATUS_OBJECT_NAME_NOT_FOUND,
                            WorkspaceStatEntryError::NoRealmAccess
                            | WorkspaceStatEntryError::InvalidKeysBundle(_)
                            | WorkspaceStatEntryError::InvalidCertificate(_)
                            | WorkspaceStatEntryError::InvalidManifest(_)
                            | WorkspaceStatEntryError::Internal(_) => STATUS_ACCESS_DENIED,
                        })?;

                    if !add_dir_info(DirInfo::new(
                        parsec_entry_stat_to_winfsp_file_info(&parent_stat, self.is_read_only),
                        u16cstr!(".."),
                    )) {
                        return Ok(());
                    }
                }
            }

            let index = match marker {
                Some(marker) => match marker.to_string_lossy().parse::<EntryName>() {
                    Ok(marker) => {
                        let outcome = reader
                            .get_index_for_name(&self.ops, &marker)
                            .await
                            .map_err(|err| match err {
                                FolderReaderStatEntryError::Offline => STATUS_HOST_UNREACHABLE,
                                FolderReaderStatEntryError::Stopped => STATUS_DEVICE_NOT_READY,
                                FolderReaderStatEntryError::NoRealmAccess
                                | FolderReaderStatEntryError::InvalidKeysBundle(_)
                                | FolderReaderStatEntryError::InvalidCertificate(_)
                                | FolderReaderStatEntryError::InvalidManifest(_)
                                | FolderReaderStatEntryError::Internal(_) => STATUS_ACCESS_DENIED,
                            })?;
                        match outcome {
                            Some(previous_index) => previous_index + 1,
                            None => 0,
                        }
                    }
                    Err(_) => 0,
                },
                None => 0,
            };

            for index in index.. {
                let maybe_child_stat =
                    reader
                        .stat_child(&self.ops, index)
                        .await
                        .map_err(|err| match err {
                            FolderReaderStatEntryError::Offline => STATUS_HOST_UNREACHABLE,
                            FolderReaderStatEntryError::Stopped => STATUS_DEVICE_NOT_READY,
                            FolderReaderStatEntryError::NoRealmAccess
                            | FolderReaderStatEntryError::InvalidKeysBundle(_)
                            | FolderReaderStatEntryError::InvalidCertificate(_)
                            | FolderReaderStatEntryError::InvalidManifest(_)
                            | FolderReaderStatEntryError::Internal(_) => STATUS_ACCESS_DENIED,
                        })?;

                let (child_name, child_stat) = match maybe_child_stat {
                    FolderReaderStatNextOutcome::Entry { name, stat } => (name, stat),
                    FolderReaderStatNextOutcome::NoMoreEntries => break,
                    FolderReaderStatNextOutcome::InvalidChild => continue,
                };
                let winified_child_name = winify_entry_name(child_name);

                if !add_dir_info(DirInfo::from_str(
                    parsec_entry_stat_to_winfsp_file_info(&child_stat, self.is_read_only),
                    &winified_child_name,
                )) {
                    break;
                }
            }

            Ok(())
        })
    }

    const GET_DIR_INFO_BY_NAME_DEFINED: bool = true;
    fn get_dir_info_by_name(
        &self,
        file_context: Self::FileContext,
        file_name: &U16CStr,
    ) -> Result<FileInfo, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        log::debug!(
            "[WinFSP] get_dir_info_by_name(file_context: {:?}, file_name: {:?})",
            fc,
            file_name
        );

        self.tokio_handle.block_on(async move {
            let child_path = match &*fc {
                OpenedObj::Folder {
                    parsec_file_name, ..
                } => {
                    let child_name = file_name
                        .to_string()
                        .map_err(|_| STATUS_OBJECT_NAME_INVALID)
                        .and_then(|name| {
                            unwinify_entry_name(&name).map_err(|_| STATUS_OBJECT_NAME_INVALID)
                        })?;
                    parsec_file_name.join(child_name)
                }
                _ => return Err(STATUS_NOT_A_DIRECTORY),
            };

            let stat = self
                .ops
                .stat_entry(&child_path)
                .await
                .map_err(|err| match err {
                    WorkspaceStatEntryError::EntryNotFound => STATUS_OBJECT_NAME_NOT_FOUND,
                    WorkspaceStatEntryError::Offline => STATUS_HOST_UNREACHABLE,
                    WorkspaceStatEntryError::Stopped => STATUS_DEVICE_NOT_READY,
                    WorkspaceStatEntryError::NoRealmAccess
                    | WorkspaceStatEntryError::InvalidKeysBundle(_)
                    | WorkspaceStatEntryError::InvalidCertificate(_)
                    | WorkspaceStatEntryError::InvalidManifest(_)
                    | WorkspaceStatEntryError::Internal(_) => STATUS_ACCESS_DENIED,
                })?;

            Ok(parsec_entry_stat_to_winfsp_file_info(
                &stat,
                self.is_read_only,
            ))
        })
    }
}

fn os_path_to_parsec_path(path: &U16CStr) -> Result<FsPath, NTSTATUS> {
    // Windows name doesn't allow `/` character, so no need to check if path
    // already contains it.
    path.to_os_string()
        .to_str()
        .ok_or(STATUS_OBJECT_NAME_INVALID)?
        .replace('\\', "/")
        .split('/')
        .filter(|&part| !(part.is_empty() || part == "."))
        .map(unwinify_entry_name)
        .collect::<Result<Vec<_>, _>>()
        .map(FsPath::from_parts)
        .map_err(|_| STATUS_OBJECT_NAME_INVALID)
}
