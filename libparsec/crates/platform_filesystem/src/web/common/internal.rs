// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::{error::*, wrapper::Directory};
use std::path::Path;

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
}
