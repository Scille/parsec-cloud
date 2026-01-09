// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::{error::NewStorageError, wrapper::Directory};

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
}
