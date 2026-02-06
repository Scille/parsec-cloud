// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::tests::get_real_path;
use crate::{load_file, save_content};
use crate::{tests::CONTENT, LoadFileError};
use libparsec_tests_fixtures::prelude::*;
use std::path::PathBuf;

// Wasm impl do not use a filesystem, so we do not have invalidPath error except if the path
// contain invalid utf-8 char.
#[parsec_test(testbed = "minimal")]
async fn ok(tmp_path: TmpPath, env: &TestbedEnv) {
    let path = tmp_path.join("devices/my_device.keys");
    save_content(&path, CONTENT).await.unwrap();
    let output = load_file(&path).await.unwrap();
    assert_eq!(output, CONTENT)
}

#[parsec_test]
async fn not_found(tmp_path: TmpPath) {
    let tmp_path = get_real_path(tmp_path).await;
    let key_file = tmp_path.join(PathBuf::from("does_not_exist"));
    let outcome = load_file(&key_file).await;
    p_assert_matches!(outcome, Err(LoadFileError::NotFound));
}

#[parsec_test]
async fn not_a_file(tmp_path: TmpPath) {
    let tmp_path = get_real_path(tmp_path).await;
    let outcome = load_file(&tmp_path).await;
    p_assert_matches!(outcome, Err(LoadFileError::NotAFile));
}
