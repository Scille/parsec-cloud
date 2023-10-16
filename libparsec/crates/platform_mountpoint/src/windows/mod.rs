// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod error;

use once_cell::sync::{Lazy, OnceCell};
use std::{
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use winfsp_wrs::{
    filetime_from_utc, filetime_now, u16cstr, CleanupFlags, CreateFileInfo, CreateOptions,
    FileAccessRights, FileAttributes, FileInfo, FileSystem, FileSystemContext, PSecurityDescriptor,
    Params, SecurityDescriptor, U16CStr, U16CString, VolumeInfo, VolumeParams,
    WriteMode as WinWriteMode, NTSTATUS, STATUS_DIRECTORY_NOT_EMPTY, STATUS_NOT_IMPLEMENTED,
    STATUS_RESOURCEMANAGER_READ_ONLY,
};

use libparsec_types::prelude::*;

use crate::{
    EntryInfo, EntryInfoType, FileSystemMounted, FileSystemWrapper, MountpointInterface, WriteMode,
};

/// we currently don't support arbitrary security descriptor and instead use only this one
/// https://docs.microsoft.com/fr-fr/windows/desktop/SecAuthZ/security-descriptor-string-format
static SECURITY_DESCRIPTOR: Lazy<SecurityDescriptor> = Lazy::new(|| {
    SecurityDescriptor::from_wstr(u16cstr!("O:BAG:BAD:P(A;;FA;;;SY)(A;;FA;;;BA)(A;;FA;;;WD)"))
        .expect("unreachable, valid SecurityDescriptor")
});

static VOLUME: OnceCell<Arc<Mutex<VolumeInfo>>> = OnceCell::new();

/// Used for the virtual file to retrieve entry info
/// This is used as a way for external applications to get information
/// on a file in a Parsec mountpoint by opening and reading the
/// virtual file f"{file_path}{ENTRY_INFO_EXTENSION}".
const ENTRY_INFO_EXTENSION: &str = "__parsec_entry_info__";

const VOLUME_LABEL: &U16CStr = u16cstr!("parsec");

/// The sector size is what is set in VolumeParams via the SectorSize
/// property when the file system is first created. (And of course the
/// "sector size" is misnamed for non-disk file systems, but it determines
/// the granularity at which non-buffered I/O is done.)
/// see: https://github.com/winfsp/winfsp/issues/240#issuecomment-518629301
const SECTOR_SIZE: u16 = 512;

fn is_entry_info_path(path: &Path) -> bool {
    path.extension()
        .map(|ext| ext == ENTRY_INFO_EXTENSION)
        .unwrap_or_default()
}

impl From<&EntryInfo> for FileInfo {
    fn from(value: &EntryInfo) -> Self {
        let created = filetime_from_utc(value.created);
        let updated = filetime_from_utc(value.updated);

        match value.ty {
            EntryInfoType::File => *FileInfo::default()
                .set_file_attributes(value.ty.into())
                .set_creation_time(created)
                .set_last_access_time(updated)
                .set_last_write_time(updated)
                .set_change_time(updated)
                .set_index_number(value.id.as_u128() as u64)
                .set_file_size(value.size)
                // AllocationSize is the size actually occupied on the storage medium, this is
                // meaningless for Parsec (however WinFSP requires FileSize <= AllocationSize)
                .set_allocation_size(value.size),
            EntryInfoType::Dir => *FileInfo::default()
                .set_file_attributes(value.ty.into())
                .set_creation_time(created)
                .set_last_access_time(updated)
                .set_last_write_time(updated)
                .set_change_time(updated)
                .set_index_number(value.id.as_u128() as u64),
            EntryInfoType::ParsecEntryInfo => *FileInfo::default()
                .set_file_attributes(value.ty.into())
                // TODO: should we use 0 value instead ?
                // Arbitrary non-zero size
                .set_file_size(1024)
                .set_time(filetime_now()),
        }
    }
}

impl From<EntryInfoType> for FileAttributes {
    fn from(value: EntryInfoType) -> Self {
        FileAttributes::not_content_indexed()
            | match value {
                EntryInfoType::File => FileAttributes::archive(),
                EntryInfoType::Dir => FileAttributes::directory(),
                EntryInfoType::ParsecEntryInfo => FileAttributes::archive(),
            }
    }
}

impl From<&EntryInfo> for FileAttributes {
    fn from(value: &EntryInfo) -> Self {
        value.ty.into()
    }
}

#[derive(Debug, Clone)]
pub enum Context {
    File(File),
    Dir(Dir),
    Info(ParsecEntryInfo),
}

impl Context {
    fn path(&self) -> &Path {
        match self {
            Self::File(file) => &file.path,
            Self::Dir(dir) => &dir.path,
            Self::Info(info) => &info.path,
        }
    }
}

#[derive(Debug, Clone)]
pub struct File {
    path: PathBuf,
    fd: FileDescriptor,
}

#[derive(Debug, Clone)]
pub struct Dir {
    path: PathBuf,
}

/// Special Parsec file for icon overlay handler
#[derive(Debug, Clone)]
pub struct ParsecEntryInfo {
    path: PathBuf,
    encoded: String,
}

impl ParsecEntryInfo {
    fn read(&self, buffer: &mut [u8], offset: usize) -> usize {
        let end_offset = std::cmp::min(self.encoded.len(), offset + buffer.len());
        let data = &self.encoded.as_bytes()[offset..end_offset];
        buffer[..data.len()].copy_from_slice(data);
        data.len()
    }
}

impl From<WinWriteMode> for WriteMode {
    fn from(value: WinWriteMode) -> Self {
        match value {
            WinWriteMode::Constrained => WriteMode::Constrained,
            WinWriteMode::Normal => WriteMode::Normal,
            WinWriteMode::StartEOF => WriteMode::StartEOF,
        }
    }
}

impl<T: MountpointInterface> FileSystemContext for FileSystemWrapper<T> {
    // In WinFSP, both file and directory are called `FileContext`.
    type FileContext = Arc<Mutex<Context>>;

    fn get_volume_info(&self) -> Result<VolumeInfo, NTSTATUS> {
        Ok(*VOLUME
            .get_or_init(|| {
                Arc::new(Mutex::new(VolumeInfo::new(
                    1024u64.pow(4), // 1 TB
                    1024u64.pow(4), // 1 TB
                    VOLUME_LABEL,
                )))
            })
            .lock()
            .expect("Mutex is poisoned"))
    }

    fn get_security_by_name(
        &self,
        file_name: &U16CStr,
        _find_reparse_point: impl Fn() -> Option<FileAttributes>,
    ) -> Result<(FileAttributes, PSecurityDescriptor, bool), NTSTATUS> {
        let path = PathBuf::from(file_name.to_os_string());

        let entry_info = self.interface.entry_info(&path)?;
        Ok((
            FileAttributes::from(&entry_info),
            SECURITY_DESCRIPTOR.as_ptr(),
            false,
        ))
    }

    fn open(
        &self,
        file_name: &U16CStr,
        _create_options: CreateOptions,
        granted_access: FileAccessRights,
    ) -> Result<Self::FileContext, NTSTATUS> {
        let write_mode = granted_access.is(FileAccessRights::file_write_data());
        let path = PathBuf::from(file_name.to_os_string());

        let fd = self.interface.file_open(&path, write_mode)?;

        if !write_mode && is_entry_info_path(&path) {
            let need_sync = self.interface.entry_info(&path)?.need_sync;
            return Ok(Arc::new(Mutex::new(Context::Info(ParsecEntryInfo {
                path,
                encoded: format!("{{\"need_sync\":{need_sync}}}"),
            }))));
        }

        if let Some(fd) = fd {
            Ok(Arc::new(Mutex::new(Context::File(File { path, fd }))))
        } else {
            Ok(Arc::new(Mutex::new(Context::Dir(Dir { path }))))
        }
    }

    fn get_file_info(&self, file_context: Self::FileContext) -> Result<FileInfo, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        let entry_info = self.interface.entry_info(fc.path())?;
        Ok(FileInfo::from(&entry_info))
    }

    fn read_directory(
        &self,
        file_context: Self::FileContext,
        marker: Option<&U16CStr>,
    ) -> Result<Vec<(U16CString, FileInfo)>, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");
        let mut entries = vec![];
        let path = fc.path();
        let stat = self.interface.entry_info(path)?;

        // NOTE: The "." and ".." directories should ONLY be included
        // if the queried directory is not root

        if fc.path() != Path::new("/") && marker.is_none() {
            entries.push((u16cstr!(".").into(), FileInfo::from(&stat)));
            let parent_path = fc.path().parent().expect("FileContext can't be root");
            let parent_stat = self.interface.entry_info(parent_path)?;
            entries.push((u16cstr!("..").into(), FileInfo::from(&parent_stat)));
        }

        // NOTE: we *do not* rely on alphabetically sorting to compare the
        // marker given `..` is always the first element event if we could
        // have children name before it (`.-foo` for instance)
        // All remaining children are located after the marker

        if let Some(marker) = marker {
            for child_name in stat
                .children
                .keys()
                .skip_while(|name| name.as_ref() != marker.to_string_lossy())
                .skip(1)
            {
                let child_path = fc.path().join(child_name.as_ref());
                let child_stat = self.interface.entry_info(&child_path)?;
                entries.push((
                    U16CString::from_str(child_name).expect("Contains a nul value"),
                    FileInfo::from(&child_stat),
                ));
            }
        } else {
            for child_name in stat.children.keys() {
                let child_path = fc.path().join(child_name.as_ref());
                let child_stat = self.interface.entry_info(&child_path)?;
                entries.push((
                    U16CString::from_str(child_name).expect("Contains a nul value"),
                    FileInfo::from(&child_stat),
                ));
            }
        }

        Ok(entries)
    }

    fn set_volume_label(&self, volume_label: &U16CStr) -> Result<(), NTSTATUS> {
        VOLUME
            .get_or_init(|| {
                Arc::new(Mutex::new(VolumeInfo::new(
                    1024u64.pow(4), // 1 TB
                    1024u64.pow(4), // 1 TB
                    VOLUME_LABEL,
                )))
            })
            .lock()
            .expect("Mutex is poisoned")
            .set_volume_label(volume_label);

        Ok(())
    }

    fn create(
        &self,
        file_name: &U16CStr,
        create_file_info: CreateFileInfo,
        _security_descriptor: SecurityDescriptor,
        _buffer: &[u8],
        _extra_buffer_is_reparse_point: bool,
    ) -> Result<Self::FileContext, NTSTATUS> {
        // `security_descriptor` is not supported yet
        // `reparse_point` is not supported yet
        let path = PathBuf::from(file_name.to_os_string());

        Ok(Arc::new(Mutex::new(
            if create_file_info
                .create_options
                .is(CreateOptions::file_directory_file())
            {
                self.interface.dir_create(&path)?;
                Context::Dir(Dir { path })
            } else {
                let fd = self.interface.file_create(&path, true)?;
                Context::File(File { path, fd })
            },
        )))
    }

    fn overwrite(
        &self,
        file_context: Self::FileContext,
        _file_attributes: FileAttributes,
        _replace_file_attributes: bool,
        allocation_size: u64,
        _buffer: &[u8],
    ) -> Result<(), NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");

        if let Context::File(file) = fc.deref() {
            self.interface.fd_resize(file.fd, allocation_size, true)?;
        }

        Ok(())
    }

    fn cleanup(
        &self,
        file_context: Self::FileContext,
        _file_name: Option<&U16CStr>,
        flags: CleanupFlags,
    ) {
        let fc = file_context.lock().expect("Mutex is poisoned");
        match (fc.deref(), flags.is(CleanupFlags::delete())) {
            (Context::File(file), true) => {
                let _ = self.interface.file_delete(&file.path);
            }
            (Context::Dir(dir), true) => {
                let _ = self.interface.dir_delete(&dir.path);
            }
            _ => (),
        }
    }

    fn close(&self, file_context: Self::FileContext) {
        let fc = file_context.lock().expect("Mutex is poisoned");
        if let Context::File(file) = fc.deref() {
            self.interface.fd_close(file.fd)
        }
    }

    fn read(
        &self,
        file_context: Self::FileContext,
        buffer: &mut [u8],
        offset: u64,
    ) -> Result<usize, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");

        Ok(match fc.deref() {
            Context::File(file) => self.interface.fd_read(file.fd, buffer, offset)?,
            Context::Info(info) => info.read(buffer, offset as usize),
            Context::Dir(_) => unreachable!(),
        })
    }

    fn write(
        &self,
        file_context: Self::FileContext,
        buffer: &[u8],
        offset: u64,
        mode: WinWriteMode,
    ) -> Result<usize, NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");

        let mode = WriteMode::from(mode);

        if let Context::File(file) = fc.deref() {
            Ok(self.interface.fd_write(file.fd, buffer, offset, mode)?)
        } else {
            Err(STATUS_NOT_IMPLEMENTED)
        }
    }

    fn flush(&self, file_context: Self::FileContext) -> Result<(), NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");

        if let Context::File(file) = fc.deref() {
            self.interface.fd_flush(file.fd);
        }

        Ok(())
    }

    fn set_basic_info(
        &self,
        _file_context: Self::FileContext,
        _file_attributes: FileAttributes,
        _creation_time: u64,
        _last_access_time: u64,
        _last_write_time: u64,
        _change_time: u64,
    ) -> Result<(), NTSTATUS> {
        Err(STATUS_NOT_IMPLEMENTED)
    }

    fn set_file_size(
        &self,
        file_context: Self::FileContext,
        new_size: u64,
        set_allocation_size: bool,
    ) -> Result<(), NTSTATUS> {
        let fc = file_context.lock().expect("Mutex is poisoned");

        if let Context::File(file) = fc.deref() {
            self.interface
                .fd_resize(file.fd, new_size, set_allocation_size)?;
        }

        Ok(())
    }

    fn rename(
        &self,
        _file_context: Self::FileContext,
        file_name: &U16CStr,
        new_file_name: &U16CStr,
        replace_if_exists: bool,
    ) -> Result<(), NTSTATUS> {
        let path = PathBuf::from(file_name.to_os_string());
        let new_path = PathBuf::from(new_file_name.to_os_string());
        self.interface
            .entry_rename(&path, &new_path, replace_if_exists)?;
        Ok(())
    }

    fn get_security(
        &self,
        _file_context: Self::FileContext,
    ) -> Result<PSecurityDescriptor, NTSTATUS> {
        Ok(SECURITY_DESCRIPTOR.as_ptr())
    }

    fn set_delete(
        &self,
        _file_context: Self::FileContext,
        file_name: &U16CStr,
        _delete_file: bool,
    ) -> Result<(), NTSTATUS> {
        let path = PathBuf::from(file_name.to_os_string());
        let stat = self.interface.entry_info(&path)?;

        if path == Path::new("/") {
            return Err(STATUS_RESOURCEMANAGER_READ_ONLY);
        } else if !stat.children.is_empty() {
            return Err(STATUS_DIRECTORY_NOT_EMPTY);
        }

        Ok(())
    }
}

pub fn mount<T: MountpointInterface>(
    mountpoint: &Path,
    interface: T,
) -> anyhow::Result<FileSystemMounted<T>> {
    let fs_wrapper = FileSystemWrapper::new(interface);
    let name = U16CString::from_os_str(mountpoint.file_name().unwrap_or_default())
        .expect("Unreachable, valid OsStr");
    let mountpoint =
        U16CString::from_os_str(mountpoint.as_os_str()).expect("Unreachable, valid OsStr");
    let mut volume_params = VolumeParams::default();

    volume_params
        .set_sector_size(SECTOR_SIZE)
        .set_sectors_per_allocation_unit(1)
        .set_volume_creation_time(filetime_now())
        .set_volume_serial_number(0)
        .set_file_info_timeout(1000)
        .set_case_sensitive_search(true)
        .set_case_preserved_names(true)
        .set_unicode_on_disk(true)
        .set_persistent_acls(true)
        .set_post_cleanup_when_modified_only(true)
        .set_file_system_name(&name)
        .map_err(|_| anyhow::anyhow!("Invalid file system name {name:?}"))?
        .set_prefix(u16cstr!(""))
        .expect("Unreachable");

    let params = Params {
        volume_params,
        ..Default::default()
    };

    FileSystem::new(params, Some(&mountpoint), fs_wrapper)
        .map(|fs| FileSystemMounted { fs })
        .map_err(|status| anyhow::anyhow!("Failed to init FileSystem {status}"))
}
