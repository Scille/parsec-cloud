// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// 1) it's useful to validate platform specific code against a simple memfs
// 2) we don't put it in the tests so that we can also create a binary for interactive testing

use chrono::Utc;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use libparsec_types::{FileDescriptor, FsPath, VlobID};

use crate::{
    EntryInfo, EntryInfoType, MountpointError, MountpointInterface, MountpointResult, WriteMode,
};

type Entries = Mutex<HashMap<FsPath, (EntryInfo, Option<Arc<Mutex<Vec<u8>>>>)>>;

pub struct MemFS {
    entries: Entries,
    fd_counter: Mutex<u32>,
    open_fds: Mutex<HashMap<FileDescriptor, FileData>>,
}

struct FileData {
    path: FsPath,
    data: Arc<Mutex<Vec<u8>>>,
}

impl Default for MemFS {
    fn default() -> Self {
        let mut entries = HashMap::new();
        let now = Utc::now();

        entries.insert(
            "/".parse().expect("unreachable"),
            (
                EntryInfo::new(VlobID::default(), EntryInfoType::Dir, now),
                None,
            ),
        );

        Self {
            entries: Mutex::new(entries),
            fd_counter: Mutex::new(0),
            open_fds: Mutex::new(HashMap::new()),
        }
    }
}

impl MemFS {
    fn get_next_fd(&self) -> FileDescriptor {
        let mut fd_counter = self.fd_counter.lock().expect("Mutex is poisoned");
        *fd_counter += 1;
        FileDescriptor(*fd_counter)
    }

    fn create_file_descriptor(&self, file_data: FileData) -> FileDescriptor {
        let fd = self.get_next_fd();
        self.open_fds
            .lock()
            .expect("Mutex is poisoned")
            .insert(fd, file_data);
        fd
    }

    fn remove_file_descriptor(&self, fd: &FileDescriptor) -> Option<FileData> {
        self.open_fds.lock().expect("Mutex is poisoned").remove(fd)
    }
}

impl MountpointInterface for MemFS {
    async fn check_read_rights(&self, _path: &FsPath) -> MountpointResult<()> {
        Ok(())
    }

    async fn check_write_rights(&self, _path: &FsPath) -> MountpointResult<()> {
        Ok(())
    }

    async fn entry_info(&self, path: &FsPath) -> MountpointResult<EntryInfo> {
        self.entries
            .lock()
            .expect("Mutex is poisoned")
            .get(path)
            .map(|x| x.0.clone())
            .ok_or(MountpointError::NotFound)
    }

    async fn entry_rename(
        &self,
        source: &FsPath,
        destination: &FsPath,
        overwrite: bool,
    ) -> MountpointResult<()> {
        let mut entries = self.entries.lock().expect("Mutex is poisoned");

        // Source is root
        // Destination is root
        // Cross-directory renaming is not supported
        if source.is_root() || destination.is_root() || source.parent() != destination.parent() {
            return Err(MountpointError::AccessDenied);
        }

        if entries.contains_key(destination) {
            if let (_, None) = entries.get(source).ok_or(MountpointError::NotFound)? {
                return Err(MountpointError::AccessDenied);
            }
            if overwrite {
                entries.remove(destination);
            } else {
                return Err(MountpointError::NameCollision);
            }
        }

        *entries = entries
            .drain()
            .map(|(path, info)| {
                if path.starts_with(source) {
                    (
                        path.replace_parent(source.parts().len(), destination.clone()),
                        info,
                    )
                } else {
                    (path, info)
                }
            })
            .collect();

        let (parent, _) = entries
            .get_mut(&source.parent())
            .ok_or(MountpointError::NotFound)?;
        let id = parent
            .children
            .remove(source.name().ok_or(MountpointError::InvalidName)?)
            .ok_or(MountpointError::NotFound)?;

        let (parent, _) = entries
            .get_mut(&destination.parent())
            .ok_or(MountpointError::NotFound)?;
        parent.children.insert(
            destination
                .name()
                .ok_or(MountpointError::InvalidName)?
                .clone(),
            id,
        );

        Ok(())
    }

    async fn file_create(&self, path: &FsPath, _open: bool) -> MountpointResult<FileDescriptor> {
        let mut entries = self.entries.lock().expect("Mutex is poisoned");

        if entries.contains_key(path) {
            return Err(MountpointError::NameCollision);
        }

        let data = Arc::new(Mutex::new(vec![]));

        let id = VlobID::default();
        let now = Utc::now();
        let fd = self.create_file_descriptor(FileData {
            path: path.clone(),
            data: data.clone(),
        });

        let (parent, _) = entries
            .get_mut(&path.parent())
            .ok_or(MountpointError::NotFound)?;

        let file_name = path.name().ok_or(MountpointError::InvalidName)?;

        parent.children.insert(file_name.clone(), id);

        entries.insert(
            path.clone(),
            (
                EntryInfo::new(VlobID::default(), EntryInfoType::File, now),
                Some(data),
            ),
        );

        Ok(fd)
    }

    async fn file_open(
        &self,
        path: &FsPath,
        _write_mode: bool,
    ) -> MountpointResult<Option<FileDescriptor>> {
        let entries = self.entries.lock().expect("Mutex is poisoned");

        let (_, data) = entries.get(path).ok_or(MountpointError::NotFound)?;

        if let Some(data) = data {
            Ok(Some(self.create_file_descriptor(FileData {
                path: path.clone(),
                data: data.clone(),
            })))
        } else {
            Ok(None)
        }
    }

    async fn file_delete(&self, path: &FsPath) -> MountpointResult<()> {
        let mut entries = self.entries.lock().expect("Mutex is poisoned");

        let (parent, _) = entries
            .get_mut(&path.parent())
            .ok_or(MountpointError::NotFound)?;

        parent
            .children
            .remove(path.name().ok_or(MountpointError::InvalidName)?);

        entries.remove(path).ok_or(MountpointError::NotFound)?;

        Ok(())
    }

    async fn fd_close(&self, fd: FileDescriptor) {
        self.remove_file_descriptor(&fd);
    }

    async fn fd_read(
        &self,
        fd: FileDescriptor,
        buffer: &mut [u8],
        offset: u64,
    ) -> MountpointResult<usize> {
        let offset = offset as usize;

        let open_fds = self.open_fds.lock().expect("Mutex is poisoned");
        let file_data = open_fds.get(&fd).ok_or(MountpointError::NotFound)?;
        let fdata = file_data.data.lock().expect("Mutex is poisoned");

        if offset >= fdata.len() {
            return Err(MountpointError::EndOfFile);
        }

        let end_offset = std::cmp::min(fdata.len(), offset + buffer.len());
        let data = &fdata[offset..end_offset];

        buffer[..data.len()].copy_from_slice(data);

        Ok(data.len())
    }

    async fn fd_resize(
        &self,
        fd: FileDescriptor,
        len: u64,
        truncate_only: bool,
    ) -> MountpointResult<()> {
        let len = len as usize;

        let mut open_fds = self.open_fds.lock().expect("Mutex is poisoned");
        let file_data = open_fds.get_mut(&fd).ok_or(MountpointError::NotFound)?;
        let mut fdata = file_data.data.lock().expect("Mutex is poisoned");

        if truncate_only && fdata.len() <= len {
            return Ok(());
        }

        fdata.resize(len, 0);
        self.entries
            .lock()
            .expect("Mutex is poisoned")
            .get_mut(&file_data.path)
            .ok_or(MountpointError::NotFound)?
            .0
            .size = fdata.len() as u64;

        Ok(())
    }

    async fn fd_flush(&self, _fd: FileDescriptor) {}

    async fn fd_write(
        &self,
        fd: FileDescriptor,
        data: &[u8],
        offset: u64,
        mode: WriteMode,
    ) -> MountpointResult<usize> {
        let mut offset = offset as usize;

        let mut open_fds = self.open_fds.lock().expect("Mutex is poisoned");
        let file_data = open_fds.get_mut(&fd).ok_or(MountpointError::NotFound)?;
        let mut fdata = file_data.data.lock().expect("Mutex is poisoned");

        let (transferred_length, end_offset) = match mode {
            WriteMode::Normal => (data.len(), offset + data.len()),
            WriteMode::Constrained => {
                if offset >= fdata.len() {
                    return Ok(0);
                }

                let end_offset = std::cmp::min(fdata.len(), offset + data.len());
                let transferred_length = end_offset - offset;

                (transferred_length, end_offset)
            }
            WriteMode::StartEOF => {
                offset = fdata.len();

                (data.len(), offset + data.len())
            }
        };

        if end_offset > fdata.len() {
            fdata.resize(end_offset, 0);
            self.entries
                .lock()
                .expect("Mutex is poisoned")
                .get_mut(&file_data.path)
                .ok_or(MountpointError::NotFound)?
                .0
                .size = fdata.len() as u64;
        }

        fdata[offset..end_offset].copy_from_slice(&data[..transferred_length]);

        Ok(transferred_length)
    }

    async fn dir_create(&self, path: &FsPath) -> MountpointResult<()> {
        let mut entries = self.entries.lock().expect("Mutex is poisoned");

        if entries.contains_key(path) {
            return Err(MountpointError::NameCollision);
        }

        let id = VlobID::default();
        let now = Utc::now();

        let (parent, _) = entries
            .get_mut(&path.parent())
            .ok_or(MountpointError::NotFound)?;

        parent
            .children
            .insert(path.name().ok_or(MountpointError::InvalidName)?.clone(), id);

        entries.insert(
            path.clone(),
            (EntryInfo::new(id, EntryInfoType::Dir, now), None),
        );

        Ok(())
    }

    async fn dir_delete(&self, path: &FsPath) -> MountpointResult<()> {
        let mut entries = self.entries.lock().expect("Mutex is poisoned");

        if entries.keys().any(|entry| &entry.parent() == path) {
            return Err(MountpointError::DirNotEmpty);
        }

        let (parent, _) = entries
            .get_mut(&path.parent())
            .ok_or(MountpointError::NotFound)?;

        parent
            .children
            .remove(path.name().ok_or(MountpointError::InvalidName)?);

        entries.remove(path).ok_or(MountpointError::NotFound)?;

        Ok(())
    }
}

#[cfg(all(test, target_os = "windows"))]
#[path = "../tests/unit/winfsp.rs"]
mod tests;
