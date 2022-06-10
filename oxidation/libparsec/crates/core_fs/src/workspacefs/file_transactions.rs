// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use std::cmp::{max, min};
use std::collections::HashMap;
use std::sync::Mutex;

use parsec_api_types::{
    BlockAccess, DeviceID, EntryID, FileDescriptor, FileManifest, FolderManifest,
};
use parsec_client_types::{
    Chunk, LocalDevice, LocalFileManifest, LocalFolderManifest, LocalManifest,
    LocalWorkspaceManifest,
};

use crate::storage::WorkspaceStorage;
use crate::{FSError, FSResult, Language};

/// A stateless class to centralize all file transactions.
///
/// The actual state is stored in the local storage and file transactions
/// have access to the remote loader to download missing resources.
///
/// The exposed transactions all take a file descriptor as first argument.
/// The file descriptors correspond to an entry id which points to a file
/// on the file system (i.e. a file manifest).
///
/// The corresponding file is locked while performing the change (i.e. between
/// the reading and writing of the corresponding manifest) in order to avoid
/// race conditions and data corruption.
///
/// The table below lists the effects of the 6 file transactions:
/// - close    -> remove file descriptor from local storage
/// - write    -> affects file content and possibly file size
/// - truncate -> affects file size and possibly file content
/// - read     -> no side effect
/// - flush    -> no-op
pub struct FileTransactions {
    workspace_id: EntryID,
    device: LocalDevice,
    local_storage: WorkspaceStorage,
    write_count: Mutex<HashMap<FileDescriptor, u64>>,
    prefered_language: Language,
}

impl FileTransactions {
    pub fn new(
        workspace_id: EntryID,
        device: LocalDevice,
        local_storage: WorkspaceStorage,
        prefered_language: Language,
    ) -> Self {
        Self {
            workspace_id,
            device,
            local_storage,
            write_count: Mutex::new(HashMap::new()),
            prefered_language,
        }
    }

    pub fn local_author(&self) -> &DeviceID {
        &self.device.device_id
    }

    fn normalize_argument(arg: i32, manifest: LocalFileManifest) -> u64 {
        if arg < 0 {
            manifest.size
        } else {
            arg as u64
        }
    }

    /// Return the data between the start and stop index.
    /// The data is treated as padded with an infinite amount of null bytes before index 0.
    fn padded_data(data: &[u8], start: i64, stop: i64) -> Vec<u8> {
        if start <= stop && stop <= 0 {
            return vec![0; (stop - start) as usize];
        }
        if 0 <= start && start <= stop {
            return data[start as usize..stop as usize].to_vec();
        }

        let mut res = vec![0; (stop - start) as usize];
        (&mut res[start.abs() as usize..]).copy_from_slice(&data[..stop as usize]);

        res
    }

    fn read_chunk<'a>(chunk: &Chunk, data: &'a [u8]) -> FSResult<&'a [u8]> {
        let begin = (chunk.start - chunk.raw_offset) as usize;
        let end = (u64::from(chunk.stop) - chunk.raw_offset) as usize;
        let len = data.len();
        if begin < end && end <= len {
            Ok(&data[begin..end])
        } else {
            Err(FSError::InvalidIndexes { begin, end, len })
        }
    }

    async fn write_chunk(&self, chunk: &Chunk, content: &[u8], offset: i64) -> FSResult<usize> {
        let data = Self::padded_data(
            content,
            offset,
            offset + u64::from(chunk.stop) as i64 - chunk.start as i64,
        );
        self.local_storage.set_chunk(chunk.id, &data)?;
        Ok(data.len())
    }

    async fn build_data(&self, chunks: &[Chunk]) -> FSResult<(Vec<u8>, Vec<BlockAccess>)> {
        if chunks.is_empty() {
            return Ok((vec![], vec![]));
        }

        let mut missing = vec![];
        let (start, stop) = (chunks[0].start, chunks[chunks.len() - 1].stop);
        let mut result = vec![0; (u64::from(stop) - start) as usize];

        for chunk in chunks {
            let data = self.local_storage.get_chunk(chunk.id)?;
            match Self::read_chunk(chunk, &data) {
                Ok(data) => (&mut result
                    [(chunk.start - start) as usize..(u64::from(chunk.stop) - start) as usize])
                    .copy_from_slice(data),
                _ => {
                    if let Some(access) = &chunk.access {
                        missing.push(access.clone())
                    }
                }
            }
        }

        Ok((result, missing))
    }

    async fn get_confinement_point(&self, entry_id: EntryID) -> Option<EntryID> {
        // Load the corresponding manifest
        let mut current_manifest = match self.local_storage.get_manifest(entry_id) {
            Ok(manifest) => manifest,
            // A missing entry is never confined
            _ => return None,
        };

        // Walk the parent chain until the workspace manifest is reached
        while let LocalManifest::File(LocalFileManifest {
            base: FileManifest { parent, .. },
            ..
        })
        | LocalManifest::Folder(LocalFolderManifest {
            base: FolderManifest { parent, .. },
            ..
        }) = current_manifest
        {
            current_manifest = match self.local_storage.get_manifest(parent) {
                Ok(manifest) => {
                    // A parent manifest is necessarely a local folder/workspace manifest
                    match &manifest {
                        LocalManifest::Folder(LocalFolderManifest {
                            local_confinement_points,
                            ..
                        })
                        | LocalManifest::Workspace(LocalWorkspaceManifest {
                            local_confinement_points,
                            ..
                        }) => {
                            // The entry is not confined
                            if local_confinement_points.contains(&current_manifest.id()) {
                                return Some(manifest.id());
                            }
                        }
                        _ => unreachable!(),
                    }

                    // Walk down
                    manifest
                }
                // In the very unlikely case where the parent manifest is not present,
                // simply consider the entry to be not confined as having false negative
                // on confinement detection is not a big deal
                _ => return None,
            };
        }

        None
    }

    // Atomic transactions

    pub async fn fd_size(&self, fd: FileDescriptor) -> FSResult<u64> {
        Ok(self.local_storage.load_file_descriptor(fd)?.size)
    }

    pub async fn fd_close(&self, fd: FileDescriptor) -> FSResult<()> {
        let manifest = self.local_storage.load_file_descriptor(fd)?;

        let entry_id = manifest.base.id;
        let mutex = self.local_storage.lock_entry_id(entry_id);
        let guard = mutex.lock().expect("Mutex is poisoned");

        // Force writing to disk
        self.local_storage
            .ensure_manifest_persistent(entry_id, true)?;
        // Atomic change
        self.local_storage.remove_file_descriptor(fd);
        // Clear write count
        self.write_count
            .lock()
            .expect("Mutex is poisoned")
            .remove(&fd);

        self.local_storage.release_entry_id(entry_id, guard);
        Ok(())
    }

    pub async fn fd_write(
        &self,
        fd: FileDescriptor,
        mut content: &[u8],
        offset: i32,
        constrained: bool,
    ) -> FSResult<usize> {
        let manifest = self.local_storage.load_file_descriptor(fd)?;

        let entry_id = manifest.base.id;
        let mutex = self.local_storage.lock_entry_id(entry_id);
        let guard = mutex.lock().expect("Mutex is poisoned");

        // Constrained - truncate content to the right length
        if constrained {
            let end_offset = min(manifest.size as i32, offset + content.len() as i32);
            let len = max(end_offset - offset, 0);
            content = &content[..len as usize];
        }

        // No-op
        if content.is_empty() {
            return Ok(0);
        }

        // Prepare
        let _updated = self.device.timestamp();
        // TODO

        // Writing

        // Atomic change

        // Reshaping

        self.local_storage.release_entry_id(entry_id, guard);

        Ok(0)
    }

    pub async fn fd_resize(
        &self,
        fd: FileDescriptor,
        length: u64,
        truncate_only: bool,
    ) -> FSResult<()> {
        let manifest = self.local_storage.load_file_descriptor(fd)?;

        if truncate_only && manifest.size <= length {
            return Ok(());
        }

        let entry_id = manifest.base.id;
        let mutex = self.local_storage.lock_entry_id(entry_id);
        let guard = mutex.lock().expect("Mutex is poisoned");

        self.manifest_resize(manifest, length, true)?;

        self.local_storage.release_entry_id(entry_id, guard);
        Ok(())
    }

    pub async fn fd_read(
        &self,
        _fd: FileDescriptor,
        _size: i32,
        _offset: i32,
        _raise_eof: bool,
    ) -> FSResult<Vec<u8>> {
        todo!();
    }

    pub async fn fd_flush(&self, fd: FileDescriptor) -> FSResult<()> {
        let manifest = self.local_storage.load_file_descriptor(fd)?;

        let entry_id = manifest.base.id;
        let mutex = self.local_storage.lock_entry_id(entry_id);
        let guard = mutex.lock().expect("Mutex is poisoned");

        self.manifest_reshape(manifest, false);
        self.local_storage
            .ensure_manifest_persistent(entry_id, true)?;

        self.local_storage.release_entry_id(entry_id, guard);

        Ok(())
    }

    fn manifest_resize(
        &self,
        _manifest: LocalFileManifest,
        _length: u64,
        _cache_only: bool,
    ) -> FSResult<()> {
        todo!();
    }

    fn manifest_reshape(
        &self,
        _manifest: LocalFileManifest,
        _cache_only: bool,
    ) -> Vec<BlockAccess> {
        todo!();
    }
}
