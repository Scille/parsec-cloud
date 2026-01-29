// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::ListFilesError;
use libparsec_platform_async::lock::Mutex;

pub async fn list_files(
    root_dir: &Path,
    extension: impl AsRef<OsStr> + std::fmt::Display,
) -> Result<Vec<PathBuf>, ListFilesError> {
    log::debug!(
        "Listing file entries in {} with extension {extension}",
        root_dir.display()
    );

    let mut files = Vec::new();
    let dir = root_dir;
    let dirs_to_explore = Arc::new(Mutex::new(Vec::from([dir.to_path_buf()])));

    'outer: while let Some(dir) = {
        let mut handle = dirs_to_explore.lock().await;
        let res = handle.pop();
        drop(handle);
        res
    } {
        log::trace!("Exploring directory {}", dir.display());
        let mut entries_stream = match tokio::fs::read_dir(&dir).await {
            Ok(v) => v,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                log::info!(
                    "Path {} not found while attempting to list entries",
                    dir.display()
                );
                continue 'outer;
            }
            Err(err) => {
                log::error!(
                    "Cannot list pending request files in {}: {err}",
                    root_dir.display()
                );
                return Err(ListFilesError::StorageNotAvailable);
            }
        };
        'inner: while let Some(entry) = entries_stream.next_entry().await.unwrap_or_default() {
            // could not get metadata, ignore entry
            let Ok(metadata) = entry.metadata().await else {
                continue 'inner;
            };
            let path = entry.path();
            if metadata.is_dir() {
                log::trace!("New directory to explore: {}", path.display());
                dirs_to_explore.lock().await.push(path);
            } else if metadata.is_file() {
                if path.extension() == Some(extension.as_ref()) {
                    files.push(path);
                } else {
                    log::trace!("ignoring file at {path:?}")
                }
            }
        }
    }
    files.sort();
    Ok(files)
}
