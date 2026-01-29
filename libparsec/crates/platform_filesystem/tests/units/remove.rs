// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use crate::{tests::CONTENT, RemoveFileError};

use crate::{list_files, remove_file, save_content};
use libparsec_tests_fixtures::prelude::*;
use libparsec_tests_lite::p_assert_matches;

#[parsec_test(testbed = "minimal")]
async fn ok(tmp_path: TmpPath, #[case] env: &TestbedEnv) {
    let file_path = tmp_path.join("devices/my_device.keys");
    let dir_path = tmp_path.join("devices");

    save_content(&file_path, CONTENT).await.unwrap();
    assert_eq!(
        list_files(&dir_path, "keys").await.unwrap(),
        vec![file_path.clone()]
    );
    remove_file(&file_path).await.unwrap();
    assert!(list_files(&dir_path, "keys").await.unwrap().is_empty());
}

#[parsec_test(testbed = "minimal")]
async fn no_file_to_remove(tmp_path: TmpPath, #[case] env: &TestbedEnv) {
    let file_path = tmp_path.join("devices/my_device.keys");

    p_assert_matches!(
        remove_file(&file_path).await.unwrap_err(),
        RemoveFileError::NotFound
    );
}
