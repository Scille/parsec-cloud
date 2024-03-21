// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use winfsp_wrs::{
    filetime_from_utc, u16cstr, CleanupFlags, CreateFileInfo, CreateOptions, DirInfo,
    FileAccessRights, FileAttributes, FileInfo, FileSystemContext, PSecurityDescriptor,
    SecurityDescriptor, U16CStr, U16String, VolumeInfo, WriteMode, NTSTATUS, STATUS_ACCESS_DENIED,
    STATUS_DEVICE_NOT_READY, STATUS_DIRECTORY_NOT_EMPTY, STATUS_FILE_IS_A_DIRECTORY,
    STATUS_HOST_UNREACHABLE, STATUS_INVALID_HANDLE, STATUS_MEDIA_WRITE_PROTECTED,
    STATUS_NOT_A_DIRECTORY, STATUS_NOT_IMPLEMENTED, STATUS_NO_SUCH_DEVICE,
    STATUS_OBJECT_NAME_COLLISION, STATUS_OBJECT_NAME_INVALID, STATUS_OBJECT_NAME_NOT_FOUND,
    STATUS_RESOURCEMANAGER_READ_ONLY,
};

use libparsec_client::workspace::{
    EntryStat, OpenOptions, WorkspaceCreateFolderError, WorkspaceFdFlushError,
    WorkspaceFdReadError, WorkspaceFdResizeError, WorkspaceFdWriteError, WorkspaceOpenFileError,
    WorkspaceOps, WorkspaceRenameEntryError, WorkspaceStatEntryError,
};
use libparsec_types::prelude::*;

use crate::windows::winify::winify_entry_name;

use super::winify::unwinify_entry_name;

macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => { println!($target, $($arg)+) };
    ($($arg:tt)+) => { println!($($arg)+) };
}

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
    path.extension()
        .map(|ext| ext == ENTRY_INFO_EXTENSION)
        .unwrap_or_default()
}

#[derive(Debug)]
pub(crate) enum OpenedObj {
    Folder {
        parsec_file_name: FsPath,
    },
    File {
        parsec_file_name: FsPath,
        fd: FileDescriptor,
    },
    // EntryInfo {
    //     parsec_file_name: FsPath,
    //     info: FileInfo,
    // },
}

fn parsec_entry_stat_to_winfsp_file_info(stat: &EntryStat) -> FileInfo {
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
                .set_file_attributes(FileAttributes::ARCHIVE | FileAttributes::NOT_CONTENT_INDEXED)
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
pub(crate) struct ParsecFileSystemContext {
    ops: Arc<WorkspaceOps>,
    tokio_handle: tokio::runtime::Handle,
    volume_label: Mutex<U16String>,
}

impl ParsecFileSystemContext {
    pub fn new(
        ops: Arc<WorkspaceOps>,
        tokio_handle: tokio::runtime::Handle,
        volume_label: U16String,
    ) -> Self {
        Self {
            ops,
            tokio_handle,
            volume_label: Mutex::new(volume_label),
        }
    }
}

impl ParsecFileSystemContext {
    async fn get_file_info_async(&self, file_context: &OpenedObj) -> Result<FileInfo, NTSTATUS> {
        let parsec_file_name = match file_context {
            OpenedObj::Folder { parsec_file_name } => parsec_file_name,
            OpenedObj::File {
                parsec_file_name, ..
            } => parsec_file_name,
            // OpenedObj::EntryInfo { info, .. } => {
            //     return Ok(info.clone());
            // }
        };

        let outcome = self.ops.stat_entry(parsec_file_name).await;
        match outcome {
            Ok(stat) => Ok(parsec_entry_stat_to_winfsp_file_info(&stat)),
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

impl FileSystemContext for ParsecFileSystemContext {
    type FileContext = Arc<Mutex<OpenedObj>>;

    fn get_volume_info(&self) -> Result<VolumeInfo, NTSTATUS> {
        debug!("[WinFSP] get_volume_info()");

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

    fn set_volume_label(&self, volume_label: &U16CStr) -> Result<VolumeInfo, NTSTATUS> {
        debug!("[WinFSP] get_volume_info(volume_label: {:?})", volume_label);

        let mut guard = self.volume_label.lock().expect("mutex is poisoned");
        guard.clear();
        guard.push(volume_label);

        self.get_volume_info()
    }

    fn get_security_by_name(
        &self,
        file_name: &U16CStr,
        _find_reparse_point: impl Fn() -> Option<FileAttributes>,
    ) -> Result<(FileAttributes, PSecurityDescriptor, bool), NTSTATUS> {
        debug!("[WinFSP] get_security_by_name(file_name: {:?})", file_name);

        let path = os_path_to_parsec_path(file_name)?;

        self.tokio_handle.block_on(async move {
            let file_attributes = match self.ops.stat_entry(&path).await {
                Ok(stat) => parsec_entry_stat_to_winfsp_file_info(&stat).file_attributes(),
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

    fn create_ex(
        &self,
        file_name: &U16CStr,
        create_file_info: CreateFileInfo,
        security_descriptor: SecurityDescriptor,
        // Buffer contains extended attributes or reparse point, so we can ignore it
        _buffer: &[u8],
        _extra_buffer_is_reparse_point: bool,
    ) -> Result<(Self::FileContext, FileInfo), NTSTATUS> {
        debug!(
            "[WinFSP] create(file_name: {:?}, create_file_info: {:?}, security_descriptor: {:?})",
            file_name, create_file_info, security_descriptor
        );

        // `security_descriptor` is not supported yet
        // `reparse_point` is not supported yet
        self.tokio_handle.block_on(async move {
            let parsec_file_name = os_path_to_parsec_path(file_name)?;

            if create_file_info
                .create_options
                .is(CreateOptions::FILE_DIRECTORY_FILE)
            {
                self.ops
                    .create_folder(parsec_file_name.clone())
                    .await
                    .map_err(|err| match err {
                        WorkspaceCreateFolderError::Offline => STATUS_HOST_UNREACHABLE,
                        WorkspaceCreateFolderError::Stopped => STATUS_DEVICE_NOT_READY,
                        WorkspaceCreateFolderError::ParentNotFound => STATUS_OBJECT_NAME_NOT_FOUND,
                        WorkspaceCreateFolderError::ParentIsFile => STATUS_OBJECT_NAME_NOT_FOUND,
                        WorkspaceCreateFolderError::EntryExists { .. } => {
                            STATUS_OBJECT_NAME_COLLISION
                        }
                        WorkspaceCreateFolderError::ReadOnlyRealm => STATUS_MEDIA_WRITE_PROTECTED,
                        WorkspaceCreateFolderError::NoRealmAccess
                        | WorkspaceCreateFolderError::InvalidKeysBundle(_)
                        | WorkspaceCreateFolderError::InvalidCertificate(_)
                        | WorkspaceCreateFolderError::InvalidManifest(_)
                        | WorkspaceCreateFolderError::Internal(_) => STATUS_ACCESS_DENIED,
                    })?;

                let opened_obj = OpenedObj::Folder { parsec_file_name };
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
                let fd = self
                    .ops
                    .open_file(parsec_file_name.clone(), options)
                    .await
                    .map_err(|err| match err {
                        WorkspaceOpenFileError::Offline => STATUS_HOST_UNREACHABLE,
                        WorkspaceOpenFileError::Stopped => STATUS_DEVICE_NOT_READY,
                        WorkspaceOpenFileError::EntryNotFound => STATUS_OBJECT_NAME_NOT_FOUND,
                        WorkspaceOpenFileError::EntryNotAFile => STATUS_FILE_IS_A_DIRECTORY,
                        WorkspaceOpenFileError::ReadOnlyRealm => STATUS_MEDIA_WRITE_PROTECTED,
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
                    fd,
                };
                let file_info = self.get_file_info_async(&opened_obj).await?;
                let file_context = Arc::new(Mutex::new(opened_obj));

                Ok((file_context, file_info))
            }
        })
    }

    fn open(
        &self,
        file_name: &U16CStr,
        create_options: CreateOptions,
        granted_access: FileAccessRights,
    ) -> Result<(Self::FileContext, FileInfo), NTSTATUS> {
        debug!(
            "[WinFSP] open(file_name: {:?}, create_option: {:x?}, granted_access: {:x?})",
            file_name, create_options, granted_access
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
            //             info: parsec_entry_stat_to_winfsp_file_info(&stat),
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
            let outcome = self.ops.open_file(parsec_file_name.clone(), options).await;

            let opened_obj = match outcome {
                Ok(fd) => Ok(OpenedObj::File {
                    parsec_file_name,
                    fd,
                }),
                Err(err) => match err {
                    WorkspaceOpenFileError::EntryNotAFile => {
                        Ok(OpenedObj::Folder { parsec_file_name })
                    }
                    WorkspaceOpenFileError::Offline => Err(STATUS_HOST_UNREACHABLE),
                    WorkspaceOpenFileError::Stopped => Err(STATUS_NO_SUCH_DEVICE),
                    WorkspaceOpenFileError::ReadOnlyRealm => Err(STATUS_MEDIA_WRITE_PROTECTED),
                    WorkspaceOpenFileError::NoRealmAccess => Err(STATUS_ACCESS_DENIED),
                    WorkspaceOpenFileError::EntryNotFound => Err(STATUS_OBJECT_NAME_NOT_FOUND),
                    WorkspaceOpenFileError::InvalidKeysBundle(_)
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

    /// Overwrite a file.
    fn overwrite_ex(
        &self,
        file_context: Self::FileContext,
        file_attributes: FileAttributes,
        replace_file_attributes: bool,
        allocation_size: u64,
        buffer: &[u8],
    ) -> Result<FileInfo, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        debug!(
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

    /// Cleanup a file.
    fn cleanup(
        &self,
        file_context: Self::FileContext,
        file_name: Option<&U16CStr>,
        flags: CleanupFlags,
    ) {
        let fc = file_context.lock().expect("Mutex is poisoned");
        debug!(
            "[WinFSP] cleanup(file_context: {:?}, file_name: {:?}, flags: {:x?})",
            fc, file_name, flags
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
                    match &*fc {
                        OpenedObj::Folder { .. } => {
                            let _ = self.ops.remove_folder(parsec_file_name).await;
                        }
                        OpenedObj::File { .. } => {
                            let _ = self.ops.remove_file(parsec_file_name).await;
                        } // OpenedObj::EntryInfo { .. } => (),
                    }
                }
            }
        })
    }

    /// Close a file.
    fn close(&self, file_context: Self::FileContext) {
        let fc = file_context.lock().expect("Mutex is poisoned");
        debug!("[WinFSP] close(file_context: {:?})", fc);

        // The file might be deleted at this point. This is fine though as the
        // file descriptor can still be used after a deletion (posix style)
        if let OpenedObj::File { fd, .. } = &*fc {
            self.tokio_handle.block_on(async move {
                let _ = self.ops.fd_close(*fd).await;
            })
        }
    }

    /// Read a file.
    fn read(
        &self,
        file_context: Self::FileContext,
        mut buffer: &mut [u8],
        offset: u64,
    ) -> Result<usize, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        debug!(
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

    /// Write a file.
    fn write(
        &self,
        file_context: Self::FileContext,
        buffer: &[u8],
        mode: WriteMode,
    ) -> Result<(usize, FileInfo), NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        // TODO: WriteMode is not Debug yet
        debug!(
            "[WinFSP] write(file_context: {:?}, buffer: {:?}, mode: {:?})",
            fc, buffer, mode,
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

    /// Flush a file or volume.
    fn flush(&self, file_context: Self::FileContext) -> Result<FileInfo, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        debug!("[WinFSP] flush(file_context: {:?})", fc);

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

    fn get_file_info(&self, file_context: Self::FileContext) -> Result<FileInfo, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        debug!("[WinFSP] get_file_info(file_context: {:?})", fc);

        self.tokio_handle
            .block_on(async move { self.get_file_info_async(&fc).await })
    }

    /// Set file or directory basic information.
    fn set_basic_info(
        &self,
        _file_context: Self::FileContext,
        _file_attributes: FileAttributes,
        _creation_time: u64,
        _last_access_time: u64,
        _last_write_time: u64,
        _change_time: u64,
    ) -> Result<FileInfo, NTSTATUS> {
        Err(STATUS_NOT_IMPLEMENTED)
    }

    /// Set file/allocation size.
    fn set_file_size(
        &self,
        file_context: Self::FileContext,
        new_size: u64,
        set_allocation_size: bool,
    ) -> Result<FileInfo, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        debug!(
            "[WinFSP] set_file_size(file_context: {:?}, new_size: {}, set_allocation_size: {})",
            fc, new_size, set_allocation_size
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

    /// Determine whether a file or directory can be deleted.
    fn can_delete(
        &self,
        file_context: Self::FileContext,
        file_name: &U16CStr,
    ) -> Result<(), NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        debug!(
            "[WinFSP] can_delete(file_context: {:?}, file_name: {:?})",
            fc, file_name
        );

        self.tokio_handle.block_on(async move {
            let parsec_file_name = os_path_to_parsec_path(file_name)?;

            if !self.ops.get_current_name_and_self_role().1.can_write() {
                return Err(STATUS_MEDIA_WRITE_PROTECTED);
            }

            if parsec_file_name.is_root() {
                // Cannot remove root mountpoint !
                return Err(STATUS_RESOURCEMANAGER_READ_ONLY);
            }

            let outcome = self.ops.stat_entry(&parsec_file_name).await;
            match outcome {
                Ok(stat) => match stat {
                    EntryStat::File { .. } => Ok(()),
                    EntryStat::Folder { children, .. } => {
                        if children.is_empty() {
                            Ok(())
                        } else {
                            Err(STATUS_DIRECTORY_NOT_EMPTY)
                        }
                    }
                },
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
        })
    }

    /// Renames a file or directory.
    fn rename(
        &self,
        file_context: Self::FileContext,
        file_name: &U16CStr,
        new_file_name: &U16CStr,
        replace_if_exists: bool,
    ) -> Result<(), NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        debug!("[WinFSP] rename(file_context: {:?}, file_name: {:?}, new_file_name: {:?}, replace_if_exists: {:?})", fc, file_name, new_file_name, replace_if_exists);

        // `granted_access` is already handle by WinFSP

        self.tokio_handle.block_on(async move {
            let parsec_file_name = os_path_to_parsec_path(file_name)?;
            let parsec_new_file_name = os_path_to_parsec_path(new_file_name)?;

            let (parsec_new_file_parent, parsec_new_file_name) = parsec_new_file_name.into_parent();
            let parsec_new_file_name = match parsec_new_file_name {
                Some(parsec_new_file_name) => parsec_new_file_name,
                None => return Err(STATUS_OBJECT_NAME_NOT_FOUND),
            };
            // TODO: ensure rename *is* a rename (i.e. not a move)
            if !parsec_file_name.is_descendant_of(&parsec_new_file_parent) {
                return Err(STATUS_NOT_IMPLEMENTED);
            }

            self.ops
                .rename_entry(parsec_file_name, parsec_new_file_name, replace_if_exists)
                .await
                .map_err(|err| match err {
                    WorkspaceRenameEntryError::EntryNotFound => STATUS_OBJECT_NAME_NOT_FOUND,
                    WorkspaceRenameEntryError::CannotRenameRoot => STATUS_OBJECT_NAME_NOT_FOUND, // TODO
                    WorkspaceRenameEntryError::DestinationExists { .. } => {
                        STATUS_OBJECT_NAME_NOT_FOUND
                    } // TODO
                    WorkspaceRenameEntryError::Offline => STATUS_HOST_UNREACHABLE,
                    WorkspaceRenameEntryError::Stopped => STATUS_DEVICE_NOT_READY,
                    WorkspaceRenameEntryError::ReadOnlyRealm => STATUS_MEDIA_WRITE_PROTECTED,
                    WorkspaceRenameEntryError::NoRealmAccess
                    | WorkspaceRenameEntryError::InvalidKeysBundle(_)
                    | WorkspaceRenameEntryError::InvalidCertificate(_)
                    | WorkspaceRenameEntryError::InvalidManifest(_)
                    | WorkspaceRenameEntryError::Internal(_) => STATUS_ACCESS_DENIED,
                })
        })
    }

    /// Get file or directory security descriptor.
    fn get_security(
        &self,
        file_context: Self::FileContext,
    ) -> Result<PSecurityDescriptor, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        debug!("[WinFSP] get_security(file_context: {:?})", fc);

        Ok(SECURITY_DESCRIPTOR.as_ptr())
    }

    /// Set file or directory security descriptor.
    fn set_security(
        &self,
        _file_context: Self::FileContext,
        _security_information: u32,
        _modification_descriptor: PSecurityDescriptor,
    ) -> Result<(), NTSTATUS> {
        Err(STATUS_NOT_IMPLEMENTED)
    }

    fn read_directory(
        &self,
        file_context: Self::FileContext,
        marker: Option<&U16CStr>,
        mut add_dir_info: impl FnMut(DirInfo) -> bool,
    ) -> Result<(), NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        debug!(
            "[WinFSP] read_directory(file_context: {:?}, marker: {:?})",
            fc, marker
        );

        self.tokio_handle.block_on(async move {
            let path = match &*fc {
                OpenedObj::Folder { parsec_file_name } => Ok(parsec_file_name),
                OpenedObj::File { .. } => Err(STATUS_NOT_A_DIRECTORY),
                // OpenedObj::EntryInfo { .. } => Err(STATUS_NOT_A_DIRECTORY),
            }?;

            macro_rules! stat_entry {
                ($path:expr) => {
                    async move {
                        let outcome = self.ops.stat_entry($path).await;
                        match outcome {
                            Ok(stat) => Ok(stat),
                            Err(err) => Err(match err {
                                WorkspaceStatEntryError::EntryNotFound => {
                                    STATUS_OBJECT_NAME_NOT_FOUND
                                }
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
                };
            }

            let stat = stat_entry!(&path).await?;
            let file_info = parsec_entry_stat_to_winfsp_file_info(&stat);
            let children = match stat {
                EntryStat::Folder { children, .. } => Ok(children),
                // Concurrent modification may have changed the type of the entry since it has been opened
                EntryStat::File { .. } => Err(STATUS_NOT_A_DIRECTORY),
            }?;

            // NOTE: The "." and ".." directories should ONLY be included
            // if the queried directory is not root

            if !path.is_root() && marker.is_none() {
                if !add_dir_info(DirInfo::new(file_info, u16cstr!("."))) {
                    return Ok(());
                }

                let parent_path = path.parent();
                let parent_stat = stat_entry!(&parent_path).await?;
                if !add_dir_info(DirInfo::new(
                    parsec_entry_stat_to_winfsp_file_info(&parent_stat),
                    u16cstr!(".."),
                )) {
                    return Ok(());
                }
            }

            // NOTE: we *do not* rely on alphabetically sorting to compare the
            // marker given `..` is always the first element event if we could
            // have children name before it (`.-foo` for instance)
            // All remaining children are located after the marker

            if let Some(marker) = marker {
                for (child_name, _) in children
                    .into_iter()
                    .skip_while(|(name, _)| name.as_ref() != marker.to_string_lossy())
                    .skip(1)
                {
                    let winified_child_name = winify_entry_name(&child_name);
                    let child_path = path.join(child_name);
                    let child_stat = stat_entry!(&child_path).await?;

                    if !add_dir_info(DirInfo::from_str(
                        parsec_entry_stat_to_winfsp_file_info(&child_stat),
                        &winified_child_name,
                    )) {
                        return Ok(());
                    }
                }
            } else {
                for (child_name, _) in children.into_iter() {
                    let winified_child_name = winify_entry_name(&child_name);
                    let child_path = path.join(child_name);
                    let child_stat = stat_entry!(&child_path).await?;

                    if !add_dir_info(DirInfo::from_str(
                        parsec_entry_stat_to_winfsp_file_info(&child_stat),
                        &winified_child_name,
                    )) {
                        return Ok(());
                    }
                }
            }

            Ok(())
        })
    }

    /// Get reparse point.
    fn get_reparse_point(
        &self,
        _file_context: Self::FileContext,
        _file_name: &U16CStr,
        _buffer: &mut [u8],
    ) -> Result<usize, NTSTATUS> {
        Err(STATUS_NOT_IMPLEMENTED)
    }

    /// Set reparse point.
    fn set_reparse_point(
        &self,
        _file_context: Self::FileContext,
        _file_name: &U16CStr,
        _buffer: &mut [u8],
    ) -> Result<(), NTSTATUS> {
        Err(STATUS_NOT_IMPLEMENTED)
    }

    /// Delete reparse point.
    fn delete_reparse_point(
        &self,
        _file_context: Self::FileContext,
        _file_name: &U16CStr,
        _buffer: &mut [u8],
    ) -> Result<(), NTSTATUS> {
        Err(STATUS_NOT_IMPLEMENTED)
    }

    /// Get named streams information.
    fn get_stream_info(
        &self,
        _file_context: Self::FileContext,
        _buffer: &mut [u8],
    ) -> Result<usize, NTSTATUS> {
        Err(STATUS_NOT_IMPLEMENTED)
    }

    /// Get directory information for a single file or directory within a parent
    /// directory.
    fn get_dir_info_by_name(
        &self,
        file_context: Self::FileContext,
        file_name: &U16CStr,
    ) -> Result<FileInfo, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        debug!(
            "[WinFSP] get_dir_info_by_name(file_context: {:?}, file_name: {:?})",
            fc, file_name
        );

        self.tokio_handle.block_on(async move {
            let child_path = match &*fc {
                OpenedObj::Folder { parsec_file_name } => {
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

            Ok(parsec_entry_stat_to_winfsp_file_info(&stat))
        })
    }

    /// Process control code.
    fn control(
        &self,
        _file_context: Self::FileContext,
        _control_code: u32,
        _input_buffer: &[u8],
        _output_buffer: &mut [u8],
    ) -> Result<usize, NTSTATUS> {
        Err(STATUS_NOT_IMPLEMENTED)
    }

    /// Set the file delete flag.
    fn set_delete(
        &self,
        _file_context: Self::FileContext,
        _file_name: &U16CStr,
        _delete_file: bool,
    ) -> Result<(), NTSTATUS> {
        Err(STATUS_NOT_IMPLEMENTED)
    }

    /// Get extended attributes.
    fn get_ea(&self, _file_context: Self::FileContext, _buffer: &[u8]) -> Result<usize, NTSTATUS> {
        Err(STATUS_NOT_IMPLEMENTED)
    }

    /// Set extended attributes.
    fn set_ea(
        &self,
        _file_context: Self::FileContext,
        _buffer: &[u8],
    ) -> Result<FileInfo, NTSTATUS> {
        Err(STATUS_NOT_IMPLEMENTED)
    }

    /// Get reparse point given a file name.
    fn get_reparse_point_by_name(
        &self,
        _file_name: &U16CStr,
        _is_directory: bool,
        _buffer: Option<&mut [u8]>,
    ) -> Result<usize, NTSTATUS> {
        Err(STATUS_NOT_IMPLEMENTED)
    }

    fn dispatcher_stopped(&self, _normally: bool) {}
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
