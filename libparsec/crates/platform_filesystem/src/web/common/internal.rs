// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::wrapper::{DirEntry, DirOrFileHandle};
use super::{error::*, wrapper::Directory};
use libparsec_platform_async::{lock::Mutex, stream::StreamExt};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    rc::Rc,
};

pub struct Storage {
    root_dir: Directory,
}

impl Storage {
    pub(crate) async fn new() -> Result<Self, NewStorageError> {
        let root_dir = Directory::get_root().await?;
        Ok(Self { root_dir })
    }

    pub(crate) fn root_dir(&self) -> &Directory {
        &self.root_dir
    }

    pub(crate) async fn read_file(&self, file: &Path) -> Result<Vec<u8>, ReadFileError> {
        let file = self.root_dir.get_file_from_path(file, None).await?;
        file.read_to_end().await.map_err(|e| e.into())
    }

    pub(crate) async fn list_file_entries(
        &self,
        dir: &Path,
        extension: impl AsRef<OsStr> + std::fmt::Display,
    ) -> Result<Vec<PathBuf>, ListFileEntriesError> {
        log::debug!(
            "Listing file entries in {} with extension {extension}",
            dir.display()
        );
        let dir = match self.root_dir.get_directory_from_path(&dir, None).await {
            Ok(dir) => dir,
            Err(GetDirectoryHandleError::NotFound { .. }) => {
                log::debug!("Could not find devices dir");
                return Ok(Vec::new());
            }
            Err(e) => return Err(e.into()),
        };
        let mut files = Vec::new();
        let dirs_to_explore = Rc::new(Mutex::new(Vec::from([dir])));
        while let Some(dir) = {
            let mut handle = dirs_to_explore.lock().await;
            let res = handle.pop();
            drop(handle);
            res
        } {
            log::trace!("Exploring directory {}", dir.path.display());
            let mut entries_stream = dir.entries();
            while let Some(entry) = entries_stream.next().await {
                let DirEntry { path, handle } = entry;
                match handle {
                    DirOrFileHandle::File(_) if path.extension() == Some(extension.as_ref()) => {
                        log::trace!("File {} with correct extension", path.display());
                        files.push(path)
                    }
                    DirOrFileHandle::File(_) => {
                        log::trace!("Ignoring file {} because of bad suffix", path.display());
                    }
                    DirOrFileHandle::Dir(handle) => {
                        log::trace!("New directory to explore: {}", path.display());
                        dirs_to_explore
                            .lock()
                            .await
                            .push(Directory { path, handle });
                    }
                }
            }
        }
        files.sort();
        Ok(files)
    }
}
