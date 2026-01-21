// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#[cfg(target_arch = "wasm32")]
use crate::web::common::internal::Storage;
use crate::{load_file, save_content};
use crate::{tests::CONTENT, LoadFileError};
use libparsec_tests_fixtures::prelude::*;
use std::path::PathBuf;

// Wasm impl do not use a filesystem, so we do not have invalidPath error except if the path
// contain invalid utf-8 char.
#[parsec_test(testbed = "minimal")]
async fn ok(tmp_path: TmpPath, #[case] env: &TestbedEnv) {
    let path = tmp_path.join("devices/my_device.keys");

    save_content(&path, CONTENT).await.unwrap();
    let output = load_file(&path).await.unwrap();
    assert_eq!(output, CONTENT)
}

#[parsec_test]
async fn not_found(#[cfg_attr(target_arch = "wasm32", expect(unused))] tmp_path: TmpPath) {
    #[cfg(target_arch = "wasm32")]
    let tmp_path = {
        let storage = Storage::new().await.unwrap();
        storage.root_dir().path.clone()
    };
    let key_file = tmp_path.join(PathBuf::from("does_not_exist"));
    let outcome = load_file(&key_file).await;
    p_assert_matches!(outcome, Err(LoadFileError::NotFound));
}

#[parsec_test]
async fn not_a_file(#[cfg_attr(target_arch = "wasm32", expect(unused))] tmp_path: TmpPath) {
    #[cfg(target_arch = "wasm32")]
    let tmp_path = {
        let storage = Storage::new().await.unwrap();
        storage.root_dir().path.clone()
    };
    let outcome = load_file(&tmp_path).await;
    p_assert_matches!(outcome, Err(LoadFileError::NotAFile));
}
