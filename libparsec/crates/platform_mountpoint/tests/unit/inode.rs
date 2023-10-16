// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_tests_lite::prelude::*;

use super::{Counter, Inode};
use crate::{FileSystemWrapper, MemFS};

#[test]
fn init() {
    let manager = FileSystemWrapper::new(MemFS::default());

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
    p_assert_eq!(paths_store.paths_indexed_by_inode[1], Path::new("/"));
    // All paths_indexed_by_inode are used
    assert!(paths_store.stack_unused_inodes.is_empty());
    // Nothing should be opened
    assert!(opened.is_empty());
}

#[test]
fn insert_path() {
    let manager = FileSystemWrapper::new(MemFS::default());
    let path = Path::new("/foo");

    let inode = manager.insert_path(path.into());

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
    p_assert_eq!(opened[path], (Counter(1), inode));
}

#[test]
fn insert_two_paths() {
    let manager = FileSystemWrapper::new(MemFS::default());
    let path0 = Path::new("/foo");
    let path1 = Path::new("/bar");

    let ino0 = manager.insert_path(path0.into());
    let ino1 = manager.insert_path(path1.into());

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
    p_assert_eq!(opened[path0], (Counter(1), ino0));
    p_assert_eq!(opened[path1], (Counter(1), ino1));
}

#[test]
fn insert_same_path_increment_counter() {
    let manager = FileSystemWrapper::new(MemFS::default());
    let path = Path::new("/foo");

    let inode = manager.insert_path(path.into());
    let same_ino = manager.insert_path(path.into());
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
    p_assert_eq!(opened[path], (Counter(2), inode));
}

#[parsec_test]
#[case(11, 10)]
#[case(6, 3)]
#[case(1, 1)]
#[case(3, 3)]
fn remove_path(#[case] opened_x_times: u64, #[case] closed_x_times: u64) {
    let manager = FileSystemWrapper::new(MemFS::default());
    let path = Path::new("/foo");

    let inode = manager.insert_path(path.into());

    for _ in 1..opened_x_times {
        manager.insert_path(path.into());
    }

    // Safety: The inode and nlookup are valid
    unsafe {
        manager.remove_path(inode, closed_x_times);
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
            opened[path],
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

#[test]
fn get_path() {
    let manager = FileSystemWrapper::new(MemFS::default());
    let path = Path::new("/foo");

    // Safety: The inode exists
    unsafe {
        p_assert_eq!(manager.get_path(Inode::from(1usize)), Path::new("/"));
    }

    let inode = manager.insert_path(path.into());

    // Safety: The inode exists
    unsafe {
        p_assert_eq!(manager.get_path(inode), path);
    }
}

#[test]
fn rename_path() {
    let manager = FileSystemWrapper::new(MemFS::default());
    let path = Path::new("/foo");
    let new_path = Path::new("/bar");

    let inode = manager.insert_path(path.into());

    manager.rename_path(path, new_path);

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
    p_assert_eq!(opened[new_path], (Counter(1), inode));
}

#[test]
fn rename_path_non_empty() {
    let manager = FileSystemWrapper::new(MemFS::default());
    let path = Path::new("/foo");
    let path_child = Path::new("/foo/baz");
    let new_path = Path::new("/bar");
    let new_path_child = Path::new("/bar/baz");

    let inode = manager.insert_path(path.into());
    let ino_child = manager.insert_path(path_child.into());

    manager.rename_path(path, new_path);

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
    p_assert_eq!(opened[new_path], (Counter(1), inode));
    p_assert_eq!(opened[new_path_child], (Counter(1), ino_child));
}

#[test]
fn realloc() {
    let manager = FileSystemWrapper::new(MemFS::default());
    let path0 = Path::new("/foo");
    let path1 = Path::new("/bar");
    let path2 = Path::new("/baz");

    let ino0 = manager.insert_path(path0.into());
    let ino1 = manager.insert_path(path1.into());
    manager.insert_path(path2.into());

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
        manager.remove_path(ino0, 1);
        manager.remove_path(ino1, 1);
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

    manager.insert_path(path0.into());

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

    manager.insert_path(path1.into());

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
