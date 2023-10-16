// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(target_os = "windows")]
#[test]
fn winfsp_tests() {
    use libparsec_platform_mountpoint::{FileSystemMounted, MemFS};
    use std::{path::Path, process::Command};

    let (tx, rx) = std::sync::mpsc::channel();

    let handle = std::thread::spawn(move || {
        winfsp_wrs::init().expect("Can't init WinFSP");
        let fs = FileSystemMounted::mount(Path::new("Z:"), MemFS::default()).unwrap();
        rx.recv().unwrap();
        fs.stop();
    });

    while !Path::new("Z:").exists() {}

    let exe =
        std::env::var("WINFSP_TEST_EXE").expect("specify the path of winfsp_tests in TEST_EXE");

    let mut tests = Command::new(exe)
        .args([
            "--external",
            "--resilient",
            // Require a case-insensitive file system
            "-getfileinfo_name_test",
            // Reparse point are not supported at the moment
            "-reparse_guid_test",
            "-reparse_nfs_test",
            "-reparse_symlink_test",
            "-reparse_symlink_relative_test",
            // Stream are not supported at the moment
            "-stream_*",
            // Setting file attributes is not supported at the moment
            "-create_fileattr_test",
            "-create_readonlydir_test",
            "-getfileattr_test",
            "-setfileinfo_test",
            "-delete_access_test",
            // Setting allocation size is not supported at the moment
            "-create_allocation_test",
            // Renaming has a special behavior in parsec
            "-rename_test",
            // Setting security is not supported at the moment
            "-setsecurity_test",
            // Entry name can't exceed 255 bytes
            "-create_namelen_test",
            "-querydir_namelen_test",
            // TODO: investigate misc tests
            "-create_notraverse_test",
            // TODO: investigate why this test only fails in appveyor
            "-create_backup_test",
            "-create_restore_test",
            "-create_related_test",
        ])
        .current_dir("Z:")
        .spawn()
        .expect("Can't run winfsp-tests");

    let code = tests.wait().expect("Tests should exit");
    assert!(code.success());

    tx.send(()).unwrap();
    handle.join().unwrap();
}
