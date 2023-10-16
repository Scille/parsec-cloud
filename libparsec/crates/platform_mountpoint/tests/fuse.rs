// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![cfg(target_os = "linux")]

use libparsec_platform_mountpoint::{FileSystemMounted, MemFS};
use libparsec_tests_fixtures::prelude::*;

#[parsec_test]
fn unmount_with_fusermount(tmp_path: TmpPath) {
    let path = &*tmp_path;
    let path = path.join("mountpoint");
    let bar = path.join("bar");

    let _fs = FileSystemMounted::mount(&path, MemFS::default()).unwrap();

    std::fs::create_dir(&bar).unwrap();

    assert!(std::process::Command::new("fusermount")
        .args(["-u", path.to_str().unwrap()])
        .status()
        .unwrap()
        .success());

    // unmount should remove all its content
    assert!(!bar.exists());
}
