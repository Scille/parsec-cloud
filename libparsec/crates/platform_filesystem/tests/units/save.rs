// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(unused)] // TODO remove when a test for wasm32 is written
use crate::{save_content, SaveContentError};
use libparsec_tests_fixtures::prelude::*;
use std::path::PathBuf;

const CONTENT: &[u8] =  b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed porta augue ante. Morbi molestie sapien eget nisi aliquet, ut commodo turpis venenatis. Maecenas porttitor mauris sapien, at gravida dui euismod et.";

#[cfg(not(target_arch = "wasm32"))]
enum BadPathKind {
    PathIsDir,
    TmpPathIsDir,
    PathIsRoot,
    PathHasNoName,
}

// Wasm impl do not use a filesystem, so we do not have invalidPath error except if the path
// contain invalid utf-8 char.
#[cfg(not(target_arch = "wasm32"))]
#[parsec_test(testbed = "minimal")]
#[case::path_is_dir(BadPathKind::PathIsDir)]
#[case::path_is_dir(BadPathKind::TmpPathIsDir)]
#[case::path_is_root(BadPathKind::PathIsRoot)]
#[case::path_has_no_name(BadPathKind::PathHasNoName)]
async fn bad_path(tmp_path: TmpPath, #[case] kind: BadPathKind, env: &TestbedEnv) {
    let (path, error) = match kind {
        BadPathKind::PathIsDir => {
            let path = tmp_path.join("devices/my_device.keys");
            std::fs::create_dir_all(&path).unwrap();
            #[cfg(target_os = "windows")]
            let error = SaveContentError::CannotEdit;
            #[cfg(not(target_os = "windows"))]
            let error = SaveContentError::NotAFile;
            (path, error)
        }
        BadPathKind::TmpPathIsDir => {
            let path = tmp_path.join("devices/my_device.keys");
            let tmp_path = tmp_path.join("devices/my_device.keys.tmp");
            std::fs::create_dir_all(tmp_path).unwrap();
            #[cfg(target_os = "windows")]
            let error = SaveContentError::CannotEdit;
            #[cfg(not(target_os = "windows"))]
            let error = SaveContentError::NotAFile;
            (path, error)
        }
        BadPathKind::PathIsRoot => (PathBuf::from("/"), SaveContentError::NotAFile),
        BadPathKind::PathHasNoName => (
            tmp_path.join("devices/my_device.keys/.."),
            SaveContentError::NotAFile,
        ),
    };

    let outcome = save_content(&path, CONTENT).await.unwrap_err();
    p_assert_eq!(outcome, error);
}

#[cfg(not(target_arch = "wasm32"))]
enum OkKind {
    ExistingParentDir,
    MissingParentDir,
    OverwritingFile,
}

// Wasm impl do not use a filesystem, so we do not have invalidPath error except if the path
// contain invalid utf-8 char.
#[cfg(not(target_arch = "wasm32"))]
#[parsec_test(testbed = "minimal")]
#[case::existing_parent_dir(OkKind::ExistingParentDir)]
#[case::missing_parent_dir(OkKind::MissingParentDir)]
#[case::overwriting_file(OkKind::OverwritingFile)]
async fn ok(tmp_path: TmpPath, #[case] kind: OkKind, env: &TestbedEnv) {
    let path = match kind {
        OkKind::ExistingParentDir => {
            let path = tmp_path.join("devices/my_device.keys");
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            path
        }
        OkKind::MissingParentDir => {
            // Both parent and grand parent dirs are missing
            tmp_path.join("devices/sub/my_device.keys")
        }
        OkKind::OverwritingFile => {
            let path = tmp_path.join("devices/my_device.keys");
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, b"<dummy>").unwrap();
            path
        }
    };

    save_content(&path, CONTENT).await.unwrap();
    // TODO check roundtrip with load #12005
}
