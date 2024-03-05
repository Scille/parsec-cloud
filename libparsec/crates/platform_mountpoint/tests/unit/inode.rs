// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_tests_lite::prelude::*;
use libparsec_types::FsPath;

use super::{Counter, Inode};
use crate::{FileSystemWrapper, MemFS};

#[parsec_test]
async fn init() {
    let manager = FileSystemWrapper::new(Arc::new(MemFS::default()));

    let paths_store = manager
        .inode_manager
        .paths_store
        .read()
        .expect("Mutex is poisoned");
    let opened = manager
        .inode_manager
        .opened
        .lock()
        .expect("Mutex is poisoned");

    // Should contains root only
    assert!(paths_store.paths_indexed_by_inode[1].is_root());
    // All paths_indexed_by_inode are used
    assert!(paths_store.stack_unused_inodes.is_empty());
    // Nothing should be opened
    assert!(opened.is_empty());
}

#[parsec_test]
async fn insert_path() {
    let manager = FileSystemWrapper::new(Arc::new(MemFS::default()));
    let path: FsPath = "/foo".parse().unwrap();

    let inode = manager.inode_manager.insert_path(path.clone());

    let paths_store = manager
        .inode_manager
        .paths_store
        .read()
        .expect("Mutex is poisoned");
    let opened = manager
        .inode_manager
        .opened
        .lock()
        .expect("Mutex is poisoned");

    // We inserted /foo
    p_assert_eq!(paths_store.paths_indexed_by_inode[2], path);
    // All paths_indexed_by_inode are used
    assert!(paths_store.stack_unused_inodes.is_empty());
    // We opened /foo
    p_assert_eq!(opened.len(), 1);
    p_assert_eq!(opened[&path], (Counter(1), inode));
}

#[parsec_test]
async fn insert_two_paths() {
    let manager = FileSystemWrapper::new(Arc::new(MemFS::default()));
    let path0: FsPath = "/foo".parse().unwrap();
    let path1: FsPath = "/bar".parse().unwrap();

    let ino0 = manager.inode_manager.insert_path(path0.clone());
    let ino1 = manager.inode_manager.insert_path(path1.clone());

    let paths_store = manager
        .inode_manager
        .paths_store
        .read()
        .expect("Mutex is poisoned");
    let opened = manager
        .inode_manager
        .opened
        .lock()
        .expect("Mutex is poisoned");

    // We inserted /foo
    p_assert_eq!(paths_store.paths_indexed_by_inode[2], path0);
    // We inserted /bar
    p_assert_eq!(paths_store.paths_indexed_by_inode[3], path1);
    // All paths_indexed_by_inode are used
    assert!(paths_store.stack_unused_inodes.is_empty());
    // We opened /foo and /bar
    p_assert_eq!(opened.len(), 2);
    p_assert_eq!(opened[&path0], (Counter(1), ino0));
    p_assert_eq!(opened[&path1], (Counter(1), ino1));
}

#[parsec_test]
async fn insert_same_path_increment_counter() {
    let manager = FileSystemWrapper::new(Arc::new(MemFS::default()));
    let path: FsPath = "/foo".parse().unwrap();

    let inode = manager.inode_manager.insert_path(path.clone());
    let same_ino = manager.inode_manager.insert_path(path.clone());
    p_assert_eq!(inode, same_ino);

    let paths_store = manager
        .inode_manager
        .paths_store
        .read()
        .expect("Mutex is poisoned");
    let opened = manager
        .inode_manager
        .opened
        .lock()
        .expect("Mutex is poisoned");

    // We inserted /foo
    p_assert_eq!(paths_store.paths_indexed_by_inode[2], path);
    p_assert_eq!(paths_store.paths_indexed_by_inode.len(), 3);
    // All paths_indexed_by_inode are used
    assert!(paths_store.stack_unused_inodes.is_empty());
    // We opened /foo
    p_assert_eq!(opened.len(), 1);
    p_assert_eq!(opened[&path], (Counter(2), inode));
}

#[parsec_test]
#[case(11, 10)]
#[case(6, 3)]
#[case(1, 1)]
#[case(3, 3)]
async fn remove_path(#[case] opened_x_times: u64, #[case] closed_x_times: u64) {
    let manager = FileSystemWrapper::new(Arc::new(MemFS::default()));
    let path: FsPath = "/foo".parse().unwrap();

    let inode = manager.inode_manager.insert_path(path.clone());

    for _ in 1..opened_x_times {
        manager.inode_manager.insert_path(path.clone());
    }

    // Safety: The inode and nlookup are valid
    unsafe {
        manager.inode_manager.remove_path(inode, closed_x_times);
    }

    let paths_store = manager
        .inode_manager
        .paths_store
        .read()
        .expect("Mutex is poisoned");
    let opened = manager
        .inode_manager
        .opened
        .lock()
        .expect("Mutex is poisoned");

    if opened_x_times > closed_x_times {
        p_assert_eq!(paths_store.paths_indexed_by_inode[2], path);
        p_assert_eq!(paths_store.paths_indexed_by_inode.len(), 3);
        // All paths_indexed_by_inode are used
        assert!(paths_store.stack_unused_inodes.is_empty());
        // still opened
        p_assert_eq!(opened.len(), 1);
        p_assert_eq!(
            opened[&path],
            (Counter(opened_x_times - closed_x_times), inode)
        );
    } else {
        // Doesn't free, but reuse with a memory manager
        p_assert_eq!(paths_store.paths_indexed_by_inode.len(), 3);
        // All paths_indexed_by_inode are used
        p_assert_eq!(paths_store.stack_unused_inodes[0], 2);
        // all closed
        p_assert_eq!(opened.len(), 0);
    }
}

#[parsec_test]
async fn get_path() {
    let manager = FileSystemWrapper::new(Arc::new(MemFS::default()));
    let path: FsPath = "/foo".parse().unwrap();

    // Safety: The inode exists
    unsafe {
        assert!(manager
            .inode_manager
            .get_path(Inode::from(1usize))
            .is_root());
    }

    let inode = manager.inode_manager.insert_path(path.clone());

    // Safety: The inode exists
    unsafe {
        p_assert_eq!(manager.inode_manager.get_path(inode), path);
    }
}

#[parsec_test]
async fn rename_path() {
    let manager = FileSystemWrapper::new(Arc::new(MemFS::default()));
    let path: FsPath = "/foo".parse().unwrap();
    let new_path = "/bar".parse().unwrap();

    let inode = manager.inode_manager.insert_path(path.clone());

    manager.inode_manager.rename_path(&path, &new_path);

    let paths_store = manager
        .inode_manager
        .paths_store
        .read()
        .expect("Mutex is poisoned");
    let opened = manager
        .inode_manager
        .opened
        .lock()
        .expect("Mutex is poisoned");

    // The path is renamed
    p_assert_eq!(paths_store.paths_indexed_by_inode[2], new_path);
    p_assert_eq!(paths_store.paths_indexed_by_inode.len(), 3);
    // All paths_indexed_by_inode are used
    assert!(paths_store.stack_unused_inodes.is_empty());
    // Opened with the path /bar and same inode
    p_assert_eq!(opened.len(), 1);
    p_assert_eq!(opened[&new_path], (Counter(1), inode));
}

#[parsec_test]
async fn rename_path_non_empty() {
    let manager = FileSystemWrapper::new(Arc::new(MemFS::default()));
    let path: FsPath = "/foo".parse().unwrap();
    let path_child = "/foo/baz".parse().unwrap();
    let new_path = "/bar".parse().unwrap();
    let new_path_child = "/bar/baz".parse().unwrap();

    let inode = manager.inode_manager.insert_path(path.clone());
    let ino_child = manager.inode_manager.insert_path(path_child);

    manager.inode_manager.rename_path(&path, &new_path);

    let paths_store = manager
        .inode_manager
        .paths_store
        .read()
        .expect("Mutex is poisoned");
    let opened = manager
        .inode_manager
        .opened
        .lock()
        .expect("Mutex is poisoned");

    // The path is renamed
    p_assert_eq!(paths_store.paths_indexed_by_inode[2], new_path);
    p_assert_eq!(paths_store.paths_indexed_by_inode[3], new_path_child);
    p_assert_eq!(paths_store.paths_indexed_by_inode.len(), 4);
    // All paths_indexed_by_inode are used
    assert!(paths_store.stack_unused_inodes.is_empty());
    // Opened with the path /bar and same inode
    p_assert_eq!(opened.len(), 2);
    p_assert_eq!(opened[&new_path], (Counter(1), inode));
    p_assert_eq!(opened[&new_path_child], (Counter(1), ino_child));
}

#[parsec_test]
async fn realloc() {
    let manager = FileSystemWrapper::new(Arc::new(MemFS::default()));
    let path0: FsPath = "/foo".parse().unwrap();
    let path1: FsPath = "/bar".parse().unwrap();
    let path2: FsPath = "/baz".parse().unwrap();

    let ino0 = manager.inode_manager.insert_path(path0.clone());
    let ino1 = manager.inode_manager.insert_path(path1.clone());
    manager.inode_manager.insert_path(path2);

    // Check that contains 4 paths_indexed_by_inode
    {
        let paths_store = manager
            .inode_manager
            .paths_store
            .read()
            .expect("Mutex is poisoned");

        p_assert_eq!(paths_store.paths_indexed_by_inode.len(), 5);
    }

    // Safety: inodes and nlookup are correct
    unsafe {
        manager.inode_manager.remove_path(ino0, 1);
        manager.inode_manager.remove_path(ino1, 1);
    }

    // Now we have unused path
    {
        let paths_store = manager
            .inode_manager
            .paths_store
            .read()
            .expect("Mutex is poisoned");

        p_assert_eq!(paths_store.stack_unused_inodes.len(), 2);
        p_assert_eq!(paths_store.stack_unused_inodes[0], 2);
        p_assert_eq!(paths_store.stack_unused_inodes[1], 3);
    }

    manager.inode_manager.insert_path(path0.clone());

    {
        let paths_store = manager
            .inode_manager
            .paths_store
            .read()
            .expect("Mutex is poisoned");

        // It works like a stack (LIFO: Last In First Out)
        p_assert_eq!(paths_store.stack_unused_inodes.len(), 1);
        p_assert_eq!(paths_store.stack_unused_inodes[0], 2);
        // This is the first index released
        p_assert_eq!(paths_store.paths_indexed_by_inode[2], path0);
        // paths_store does not grow
        p_assert_eq!(paths_store.paths_indexed_by_inode.len(), 5);
    }

    manager.inode_manager.insert_path(path1.clone());

    {
        let paths_store = manager
            .inode_manager
            .paths_store
            .read()
            .expect("Mutex is poisoned");

        // All space is used now
        p_assert_eq!(paths_store.stack_unused_inodes.len(), 0);
        // This is the second index released
        p_assert_eq!(paths_store.paths_indexed_by_inode[2], path1);
        // paths_store does not grow
        p_assert_eq!(paths_store.paths_indexed_by_inode.len(), 5);
    }
}
