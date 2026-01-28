// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use crate::tests::CONTENT;

use crate::{list_files, rename_file, save_content, RenameFileError};
use libparsec_tests_fixtures::prelude::*;
use libparsec_tests_lite::p_assert_matches;

#[parsec_test(testbed = "minimal")]
async fn ok(tmp_path: TmpPath, #[case] env: &TestbedEnv) {
    let file_path = tmp_path.join("devices/my_device.keys");
    let dir_path = tmp_path.join("devices");
    let new_file_path = tmp_path.join("devices/archived/my_device.keys");

    save_content(&file_path, CONTENT).await.unwrap();
    assert_eq!(
        list_files(&dir_path, "keys").await.unwrap(),
        vec![file_path.clone()]
    );

    #[cfg(not(target_arch = "wasm32"))]
    std::fs::create_dir(dir_path.join("archived")).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use crate::platform::common::internal::Storage;
        let storage = Storage::new().await.unwrap();
        storage
            .root_dir()
            .create_dir_all(&dir_path.join("archived"))
            .await
            .unwrap();
    }

    rename_file(&file_path, &new_file_path).await.unwrap();
    assert_eq!(
        list_files(&dir_path, "keys").await.unwrap(),
        vec![new_file_path]
    );
}

#[parsec_test(testbed = "minimal")]
async fn no_file_to_rename(tmp_path: TmpPath, #[case] env: &TestbedEnv) {
    let file_path = tmp_path.join("devices/my_device.keys");
    let new_file_path = tmp_path.join("devices/archived/my_device.keys");

    p_assert_matches!(
        rename_file(&file_path, &new_file_path).await.unwrap_err(),
        RenameFileError::NotFound
    );
}
