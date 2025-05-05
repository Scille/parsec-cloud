// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub use platform::*;

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    use std::path::Path;

    pub async fn create_device_file(path: &Path, content: &[u8]) {
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();
        tokio::fs::write(path, content).await.unwrap();
    }

    pub async fn key_present_in_system(path: &Path) -> bool {
        path.exists()
    }

    pub async fn key_is_archived(path: &Path) -> bool {
        let expected_archive_path = path.with_extension("device.archived");
        !key_present_in_system(path).await && key_present_in_system(&expected_archive_path).await
    }
}

#[cfg(target_arch = "wasm32")]
mod platform {
    use crate::{
        get_device_archive_path,
        platform::{
            error::GetFileHandleError,
            wrapper::{Directory, OpenOptions},
        },
    };
    use std::{ffi::OsStr, path::Path};

    async fn get_storage() -> Directory {
        Directory::get_root().await.expect("Cannot get storage")
    }

    pub async fn create_device_file(path: &Path, content: &[u8]) {
        let storage = get_storage().await;
        let dir = if let Some(parent) = path.parent() {
            let parent = storage
                .create_dir_all(parent)
                .await
                .expect("Cannot create dir all");
            Some(parent)
        } else {
            None
        };
        let filename = path
            .file_name()
            .and_then(OsStr::to_str)
            .expect("Missing filename");
        let file = dir
            .as_ref()
            .unwrap_or(&storage)
            .get_file(filename, Some(OpenOptions::create()))
            .await
            .expect("Cannot get file");
        file.write_all(content)
            .await
            .expect("Cannot add device to storage");
    }

    pub async fn key_present_in_system(path: &Path) -> bool {
        let storage = get_storage().await;
        match storage.get_file_from_path(path, None).await {
            Ok(_) => true,
            Err(GetFileHandleError::NotFound { .. }) => false,
            Err(_) => panic!("Cannot get item in storage"),
        }
    }

    pub async fn key_is_archived(path: &Path) -> bool {
        let archived_path = get_device_archive_path(path);

        !key_present_in_system(path).await && key_present_in_system(&archived_path).await
    }
}
