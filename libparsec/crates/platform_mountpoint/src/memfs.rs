// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// 1) it's useful to validate platform specific code against a simple memfs
// 2) we don't put it in the tests so that we can also create a binary for interactive testing

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use chrono::Utc;
use libparsec_types::{EntryID, EntryName, FileDescriptor};

use crate::{
    EntryInfo, EntryInfoType, MountpointError, MountpointInterface, MountpointResult, WriteMode,
};

type Entries = Mutex<HashMap<PathBuf, (EntryInfo, Option<Arc<Mutex<Vec<u8>>>>)>>;

pub struct MountpointManager {
    entries: Entries,
    fd_counter: Mutex<u32>,
    open_fds: Mutex<HashMap<FileDescriptor, FileData>>,
}

struct FileData {
    path: PathBuf,
    data: Arc<Mutex<Vec<u8>>>,
}

impl Default for MountpointManager {
    fn default() -> Self {
        let mut entries = HashMap::new();
        let now = Utc::now();

        entries.insert(
            PathBuf::from("/"),
            (
                EntryInfo {
                    id: EntryID::default(),
                    ty: EntryInfoType::Dir,
                    created: now,
                    updated: now,
                    size: 0,
                    need_sync: false,
                    children: HashMap::new(),
                },
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

impl MountpointManager {
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

impl MountpointInterface for MountpointManager {
    fn check_read_rights(&self, _path: &Path) -> MountpointResult<()> {
        Ok(())
    }

    fn check_write_rights(&self, _path: &Path) -> MountpointResult<()> {
        Ok(())
    }

    fn entry_info(&self, path: &Path) -> MountpointResult<EntryInfo> {
        self.entries
            .lock()
            .expect("Mutex is poisoned")
            .get(path)
            .map(|x| x.0.clone())
            .ok_or(MountpointError::NotFound)
    }

    fn entry_rename(
        &self,
        source: &Path,
        destination: &Path,
        overwrite: bool,
    ) -> MountpointResult<()> {
        let mut entries = self.entries.lock().expect("Mutex is poisoned");
        let source_str = source.to_str().expect("Non utf8 source");
        let destination_str = destination.to_str().expect("Non utf8 destination");
        let root = Path::new("/");

        // Source is root
        // Destination is root
        // Cross-directory renaming is not supported
        if source == root || destination == root || source.parent() != destination.parent() {
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

        let iter_entries = entries
            .keys()
            .map(|path| path.to_str().expect("Non utf8 path").to_string())
            .filter(|path| path.starts_with(source_str))
            .collect::<Vec<String>>();

        for entry_path in iter_entries {
            let new_entry_path = PathBuf::from(entry_path.replacen(source_str, destination_str, 1));

            let entry = entries
                .remove(Path::new(&entry_path))
                .ok_or(MountpointError::NotFound)?;
            entries.insert(new_entry_path, entry);
        }

        let (parent, _) = entries
            .get_mut(source.parent().expect("Can't be root"))
            .ok_or(MountpointError::NotFound)?;
        let id = parent
            .children
            .remove(
                &EntryName::try_from(
                    source
                        .file_name()
                        .expect("Contains a name")
                        .to_str()
                        .expect("Non utf8 name"),
                )
                .expect("Should be a valid EntryName"),
            )
            .ok_or(MountpointError::NotFound)?;

        let (parent, _) = entries
            .get_mut(destination.parent().expect("Can't be root"))
            .ok_or(MountpointError::NotFound)?;
        parent.children.insert(
            EntryName::try_from(
                destination
                    .file_name()
                    .expect("Contains a name")
                    .to_str()
                    .expect("Non utf8 name"),
            )
            .expect("Should be a valid EntryName"),
            id,
        );

        Ok(())
    }

    fn file_create(&self, path: &Path, _open: bool) -> MountpointResult<FileDescriptor> {
        let mut entries = self.entries.lock().expect("Mutex is poisoned");

        if entries.contains_key(path) {
            return Err(MountpointError::NameCollision);
        }

        let data = Arc::new(Mutex::new(vec![]));

        let id = EntryID::default();
        let now = Utc::now();
        let fd = self.create_file_descriptor(FileData {
            path: path.into(),
            data: data.clone(),
        });

        let (parent, _) = entries
            .get_mut(path.parent().expect("Can't be root"))
            .ok_or(MountpointError::NotFound)?;

        let file_name = path
            .file_name()
            .expect("Contains a name")
            .to_str()
            .expect("Non utf8 name");
        parent.children.insert(
            EntryName::try_from(file_name).expect("Should be a valid EntryName"),
            id,
        );

        entries.insert(
            path.to_path_buf(),
            (
                EntryInfo {
                    id,
                    ty: EntryInfoType::File,
                    created: now,
                    updated: now,
                    size: 0,
                    need_sync: false,
                    children: HashMap::new(),
                },
                Some(data),
            ),
        );

        Ok(fd)
    }

    fn file_open(
        &self,
        path: &Path,
        _write_mode: bool,
    ) -> MountpointResult<Option<FileDescriptor>> {
        let entries = self.entries.lock().expect("Mutex is poisoned");

        let (_, data) = entries.get(path).ok_or(MountpointError::NotFound)?;

        if let Some(data) = data {
            Ok(Some(self.create_file_descriptor(FileData {
                path: path.into(),
                data: data.clone(),
            })))
        } else {
            Ok(None)
        }
    }

    fn file_delete(&self, path: &Path) -> MountpointResult<()> {
        let mut entries = self.entries.lock().expect("Mutex is poisoned");

        let (parent, _) = entries
            .get_mut(path.parent().expect("Can't be root"))
            .ok_or(MountpointError::NotFound)?;
        parent.children.remove(
            &EntryName::try_from(
                path.file_name()
                    .expect("Contains a name")
                    .to_str()
                    .expect("Non utf8 name"),
            )
            .expect("Should be a valid EntryName"),
        );

        entries.remove(path).ok_or(MountpointError::NotFound)?;

        Ok(())
    }

    fn file_resize(&self, _path: &Path, _len: u64) -> EntryID {
        todo!("UNIX only")
    }

    fn fd_close(&self, fd: FileDescriptor) {
        self.remove_file_descriptor(&fd);
    }

    fn fd_read(
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

    fn fd_resize(&self, fd: FileDescriptor, len: u64, truncate_only: bool) -> MountpointResult<()> {
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

    fn fd_flush(&self, _fd: FileDescriptor) {}

    fn fd_write(
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

    fn dir_create(&self, path: &Path) -> MountpointResult<()> {
        let mut entries = self.entries.lock().expect("Mutex is poisoned");

        if entries.contains_key(path) {
            return Err(MountpointError::NameCollision);
        }

        let id = EntryID::default();
        let now = Utc::now();

        let (parent, _) = entries
            .get_mut(path.parent().expect("Can't be root"))
            .ok_or(MountpointError::NotFound)?;
        parent.children.insert(
            EntryName::try_from(
                path.file_name()
                    .expect("Contains a name")
                    .to_str()
                    .expect("Non utf8 name"),
            )
            .expect("Should be a valid EntryName"),
            id,
        );

        entries.insert(
            path.to_path_buf(),
            (
                EntryInfo {
                    id,
                    ty: EntryInfoType::Dir,
                    created: now,
                    updated: now,
                    size: 0,
                    need_sync: false,
                    children: HashMap::new(),
                },
                None,
            ),
        );

        Ok(())
    }

    fn dir_delete(&self, path: &Path) -> MountpointResult<()> {
        let mut entries = self.entries.lock().expect("Mutex is poisoned");

        if entries.keys().any(|entry| entry.parent() == Some(path)) {
            return Err(MountpointError::DirNotEmpty);
        }

        let (parent, _) = entries
            .get_mut(path.parent().expect("Can't be root"))
            .ok_or(MountpointError::NotFound)?;
        parent.children.remove(
            &EntryName::try_from(
                path.file_name()
                    .expect("Contains a name")
                    .to_str()
                    .expect("Non utf8 name"),
            )
            .expect("Should be a valid EntryName"),
        );

        entries.remove(path).ok_or(MountpointError::NotFound)?;

        Ok(())
    }
}
