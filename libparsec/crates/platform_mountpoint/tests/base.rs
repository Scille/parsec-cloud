// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![cfg(not(target_os = "macos"))]

use std::fs::{File, OpenOptions};

use libparsec_platform_mountpoint::{FileSystemMounted, MemFS};
use libparsec_tests_fixtures::prelude::*;

macro_rules! mount {
    ($tmp_path: ident) => {{
        #[cfg(target_os = "windows")]
        winfsp_wrs::init().expect("Can't init WinFSP");

        // Windows can't mount on existing directory
        let path = (*$tmp_path).join("mountpoint");
        let fs = FileSystemMounted::mount(&path, MemFS::default()).unwrap();

        (path, fs)
    }};
}

#[parsec_test]
fn create_dir(tmp_path: TmpPath) {
    let (path, fs) = mount!(tmp_path);
    let bar = path.join("bar");

    std::fs::create_dir(&bar).unwrap();

    assert!(bar.exists());

    fs.stop();
}

#[parsec_test]
fn create_file(tmp_path: TmpPath) {
    let (path, fs) = mount!(tmp_path);
    let bar = path.join("bar");

    File::create(&bar).unwrap();

    assert!(bar.exists());

    fs.stop();
}

#[parsec_test]
fn stop(tmp_path: TmpPath) {
    let (path, fs) = mount!(tmp_path);
    let bar = path.join("bar");

    std::fs::create_dir(&bar).unwrap();

    fs.stop();

    // unmount should remove all its content
    assert!(!bar.exists());
}

#[parsec_test]
#[case("x".repeat(256))]
#[case("飞".repeat(85) + "x")]
fn dir_exceeding_255_bytes(tmp_path: TmpPath, #[case] name: String) {
    let (path, fs) = mount!(tmp_path);

    // TODO: assert_eq it
    // ErrorKind::InvalidFilename is under `io_error_more` feature which is
    // available on nightly only
    std::fs::create_dir(path.join(name)).unwrap_err();

    fs.stop();
}

#[parsec_test]
#[case("x".repeat(256))]
#[case("飞".repeat(85) + "x")]
fn file_exceeding_255_bytes(tmp_path: TmpPath, #[case] name: String) {
    let (path, fs) = mount!(tmp_path);

    // TODO: assert_eq it
    // ErrorKind::InvalidFilename is under `io_error_more` feature which is
    // available on nightly only
    File::create(path.join(name)).unwrap_err();

    fs.stop();
}

#[parsec_test]
fn read_write(tmp_path: TmpPath) {
    let (path, fs) = mount!(tmp_path);
    let bar = path.join("bar");
    let data = b"Lorem ipsum";

    File::create(&bar).unwrap();

    std::fs::write(&bar, data).unwrap();

    p_assert_eq!(std::fs::read(&bar).unwrap(), data);

    fs.stop();
}

#[parsec_test]
fn remove_dir(tmp_path: TmpPath) {
    let (path, fs) = mount!(tmp_path);
    let bar = path.join("bar");

    std::fs::create_dir(&bar).unwrap();
    std::fs::remove_dir(&bar).unwrap();

    assert!(!bar.exists());

    fs.stop();
}

#[parsec_test]
fn remove_dir_non_empty(tmp_path: TmpPath) {
    let (path, fs) = mount!(tmp_path);
    let bar = path.join("bar");
    let foo = bar.join("foo");

    std::fs::create_dir(&bar).unwrap();
    std::fs::create_dir(foo).unwrap();

    // TODO: assert_eq it
    // ErrorKind::DirectoryNotEmpty is under `io_error_more` feature which is
    // available on nightly only
    std::fs::remove_dir(&bar).unwrap_err();

    std::fs::remove_dir_all(&bar).unwrap();

    fs.stop();
}

#[parsec_test]
fn remove_file(tmp_path: TmpPath) {
    let (path, fs) = mount!(tmp_path);
    let bar = path.join("bar");

    File::create(&bar).unwrap();
    std::fs::remove_file(&bar).unwrap();

    assert!(!bar.exists());

    fs.stop();
}

#[parsec_test]
fn rename_dir(tmp_path: TmpPath) {
    let (path, fs) = mount!(tmp_path);
    let bar = path.join("bar");
    let foo = path.join("foo");

    std::fs::create_dir(&bar).unwrap();
    std::fs::rename(&bar, &foo).unwrap();

    assert!(!bar.exists());
    assert!(foo.exists());

    fs.stop();
}

#[parsec_test]
fn rename_file(tmp_path: TmpPath) {
    let (path, fs) = mount!(tmp_path);
    let bar = path.join("bar");
    let foo = path.join("foo");

    File::create(&bar).unwrap();
    std::fs::rename(&bar, &foo).unwrap();

    assert!(!bar.exists());
    assert!(foo.exists());

    fs.stop();
}

#[parsec_test]
fn rename_dir_non_empty(tmp_path: TmpPath) {
    let (path, fs) = mount!(tmp_path);
    let bar = path.join("bar");
    let foo = path.join("foo");
    let baz = bar.join("baz");
    let new_baz = foo.join("baz");

    std::fs::create_dir(&bar).unwrap();
    File::create(&baz).unwrap();
    std::fs::rename(&bar, &foo).unwrap();

    assert!(!bar.exists());
    assert!(!baz.exists());
    assert!(foo.exists());
    assert!(new_baz.exists());

    fs.stop();
}

#[parsec_test]
fn copy(tmp_path: TmpPath) {
    let (path, fs) = mount!(tmp_path);
    let bar = path.join("bar");
    let foo = path.join("foo");
    let data = b"Lorem ipsum";

    File::create(&bar).unwrap();
    File::create(&foo).unwrap();
    std::fs::write(&bar, data).unwrap();

    std::fs::copy(&bar, &foo).unwrap();

    p_assert_eq!(std::fs::read(&bar).unwrap(), data);
    p_assert_eq!(std::fs::read(&foo).unwrap(), data);

    fs.stop();
}

#[parsec_test]
fn remove_then_close_file(tmp_path: TmpPath) {
    let (path, fs) = mount!(tmp_path);

    let with = path.join("with_fsync.txt");
    let without = path.join("without_fsync.txt");

    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&with)
        .unwrap();

    std::fs::remove_file(&with).unwrap();
    fd.sync_all().unwrap();
    drop(fd);

    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&without)
        .unwrap();

    std::fs::remove_file(&without).unwrap();
    drop(fd);

    fs.stop();
}

#[parsec_test]
fn read_dir(tmp_path: TmpPath) {
    let (path, fs) = mount!(tmp_path);

    for i in 0..100 {
        std::fs::create_dir(path.join(format!("foo{i}"))).unwrap();
        File::create(path.join(format!("bar{i}"))).unwrap();
    }

    let entries = std::fs::read_dir(path).unwrap();

    p_assert_eq!(entries.count(), 200);

    fs.stop();
}
