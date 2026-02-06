// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{
    list_files,
    tests::{create_dir_all, create_write_file},
};
use libparsec_tests_fixtures::prelude::*;
use std::path::PathBuf;

#[parsec_test]
#[case::unknown_path(false)]
#[case::existing_path(true)]
async fn list_no_files(tmp_path: TmpPath, #[case] path_exists: bool) {
    if path_exists {
        create_dir_all(&tmp_path.join("devices")).await;
    }

    let files = list_files(&tmp_path, "fake").await.unwrap();
    p_assert_eq!(files, Vec::<PathBuf>::new());
}

#[parsec_test]
async fn list(tmp_path: TmpPath) {
    let path1 = tmp_path.join("devices");
    let path2 = tmp_path.join("devices").join("archived");
    create_dir_all(&path2).await;

    for file_path in [
        path1.join("key.fake"),
        path1.join("door.fake"),
        path1.join("key.true"),
        path2.join("key.fake"),
        path2.join("door.fake"),
        path2.join("key.true"),
    ] {
        println!("{file_path:?}");
        create_write_file(&file_path).await;
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
