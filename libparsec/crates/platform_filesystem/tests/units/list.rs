// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(unused)] // TODO remove when a test for wasm32 is written

use crate::list_files;
#[cfg(target_arch = "wasm32")]
use crate::platform::common::{internal::Storage, wrapper::OpenOptions};
use crate::tests::CONTENT;
use libparsec_tests_fixtures::prelude::*;
use std::path::PathBuf;

#[parsec_test]
#[case::unknown_path(false)]
#[case::existing_path(true)]
async fn list_no_files(tmp_path: TmpPath, #[case] path_exists: bool) {
    if path_exists {
        #[cfg(not(target_arch = "wasm32"))]
        std::fs::create_dir(tmp_path.join("devices")).unwrap();

        #[cfg(target_arch = "wasm32")]
        {
            let storage = Storage::new().await.unwrap();
            storage
                .root_dir()
                .create_dir_all(&tmp_path.join("devices"))
                .await
                .unwrap();
        }
    }

    let files = list_files(&tmp_path, "fake").await.unwrap();
    p_assert_eq!(files, Vec::<PathBuf>::new());
}

#[parsec_test]
async fn list(tmp_path: TmpPath) {
    #[cfg(target_arch = "wasm32")]
    {
        let storage = Storage::new().await.unwrap();
        let tmp_path = storage.root_dir().path.clone();
    }
    let path1 = tmp_path.join("devices");
    let path2 = tmp_path.join("devices").join("archived");
    #[cfg(target_arch = "wasm32")]
    {
        let storage = Storage::new().await.unwrap();

        let dir1 = storage
            .root_dir()
            .get_directory_from_path(&path1, Some(OpenOptions::create()))
            .await
            .unwrap();
        let dir2 = storage
            .root_dir()
            .get_directory_from_path(&path2, Some(OpenOptions::create()))
            .await
            .unwrap();
        storage.root_dir().create_dir_all(&path2).await.unwrap();

        let file1 = dir1
            .get_file("key.fake", Some(OpenOptions::create()))
            .await
            .unwrap();
        let file2 = dir1
            .get_file("door.fake", Some(OpenOptions::create()))
            .await
            .unwrap();
        let file3 = dir1
            .get_file("key.true", Some(OpenOptions::create()))
            .await
            .unwrap();
        let file4 = dir2
            .get_file("key.fake", Some(OpenOptions::create()))
            .await
            .unwrap();
        let file5 = dir2
            .get_file("door.fake", Some(OpenOptions::create()))
            .await
            .unwrap();
        let file6 = dir2
            .get_file("key.true", Some(OpenOptions::create()))
            .await
            .unwrap();

        for file in vec![file1, file2, file3, file4, file5, file6] {
            file.write_all(CONTENT).await.unwrap()
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::fs::create_dir_all(&path2).unwrap();

        std::fs::write(path1.join("key.fake"), CONTENT).unwrap();
        std::fs::write(path1.join("door.fake"), CONTENT).unwrap();
        std::fs::write(path1.join("key.true"), CONTENT).unwrap();
        std::fs::write(path2.join("key.fake"), CONTENT).unwrap();
        std::fs::write(path2.join("door.fake"), CONTENT).unwrap();
        std::fs::write(path2.join("key.true"), CONTENT).unwrap();
    }
    let devices = list_files(&tmp_path, "fake").await.unwrap();
    p_assert_eq!(
        devices,
        [
            path2.join("door.fake"),
            path2.join("key.fake"),
            path1.join("door.fake"),
            path1.join("key.fake"),
        ]
    );
}
