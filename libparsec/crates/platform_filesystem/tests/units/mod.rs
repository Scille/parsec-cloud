// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use std::path::{Path, PathBuf};

#[cfg(target_arch = "wasm32")]
libparsec_tests_lite::platform::wasm_bindgen_test_configure!(run_in_browser run_in_shared_worker);
#[cfg(target_arch = "wasm32")]
use crate::platform::common::{internal::Storage, wrapper::OpenOptions};

const CONTENT: &[u8] =  b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed porta augue ante. Morbi molestie sapien eget nisi aliquet, ut commodo turpis venenatis. Maecenas porttitor mauris sapien, at gravida dui euismod et.";

mod list;
mod load;
mod remove;
mod rename;
mod save;

async fn create_dir_all(path: &Path) {
    #[cfg(not(target_arch = "wasm32"))]
    std::fs::create_dir_all(path).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        let storage = Storage::new().await.unwrap();
        storage.root_dir().create_dir_all(path).await.unwrap();
    }
}

async fn create_write_file(file_path: &Path) {
    #[cfg(not(target_arch = "wasm32"))]
    std::fs::write(file_path, CONTENT).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        let storage = Storage::new().await.unwrap();
        let parent_dir_path = file_path.parent().unwrap();

        let dir = storage
            .root_dir()
            .get_directory_from_path(&parent_dir_path, Some(OpenOptions::create()))
            .await
            .unwrap();

        let file = dir
            .get_file(
                file_path.file_name().unwrap().to_str().unwrap(),
                Some(OpenOptions::create()),
            )
            .await
            .unwrap();
        file.write_all(CONTENT).await.unwrap();
    }
}

/// The tmp_path fixture does not return a path
/// that exists in web.
async fn get_real_path(tmp_path: TmpPath) -> PathBuf {
    #[cfg(not(target_arch = "wasm32"))]
    {
        tmp_path.parent().unwrap().to_path_buf()
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = tmp_path; // ignored because it's not useful
        let storage = Storage::new().await.unwrap();
        storage.root_dir().path.clone()
    }
}
