// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashMap,
    ffi::OsString,
    os::unix::prelude::{OsStrExt, OsStringExt},
    path::{Path, PathBuf},
    sync::{Mutex, RwLock},
};

use crate::{FileSystemWrapper, MountpointInterface};

/// `paths_indexed_by_inode` allocator indexed by `inode`
/// The index of path is the inode number and the free inodes are stored
/// `Pasteur` must contain his rage when he sees this code
struct PathsStore {
    paths_indexed_by_inode: Vec<PathBuf>,
    stack_unused_inodes: Vec<usize>,
}

impl Default for PathsStore {
    fn default() -> Self {
        Self {
            paths_indexed_by_inode: vec![PathBuf::from(""), PathBuf::from("/")],
            stack_unused_inodes: vec![],
        }
    }
}

/// `Counter` wrapper used to interface with `FUSE` low level `API`
///
/// - Every time `reply_entry` or `reply_created` are called, `inode's counter`
/// should be incremented.
/// - Every time `forget` is called, `inode's counter` should be decremented by
/// `nlookup`.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) struct Counter(u64);

impl Default for Counter {
    fn default() -> Self {
        Self(1)
    }
}

impl Counter {
    fn increment(&mut self) {
        self.0 += 1;
    }

    fn decrement(&mut self, nlookup: u64) {
        self.0 -= nlookup;
    }

    fn is_zero(&mut self) -> bool {
        self.0 == 0
    }
}

/// `Inode` wrapper used to interface with `FUSE` low level `API`.
///
/// We use that structure to provide a high level `API`
#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(super) struct Inode(usize);

impl From<usize> for Inode {
    /// -> `Inode` starts at `1` which is the root directory
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Inode> for usize {
    /// -> `Inode` starts at `1` which is the root directory
    fn from(value: Inode) -> Self {
        value.0
    }
}

impl From<u64> for Inode {
    fn from(value: u64) -> Self {
        Self(value as usize)
    }
}

impl From<Inode> for u64 {
    fn from(value: Inode) -> Self {
        value.0 as u64
    }
}

#[derive(Default)]
pub(crate) struct InodeManager {
    paths_store: RwLock<PathsStore>,
    opened: Mutex<HashMap<PathBuf, (Counter, Inode)>>,
}

impl<T: MountpointInterface> FileSystemWrapper<T> {
    pub(super) fn insert_path(&self, path: PathBuf) -> Inode {
        let mut paths_store = self
            .inode_manager
            .paths_store
            .write()
            .expect("Mutex is poisoned");
        let mut opened = self.inode_manager.opened.lock().expect("Mutex is poisoned");

        if let Some((counter, inode)) = opened.get_mut(&path) {
            counter.increment();
            return *inode;
        }

        let inode = if let Some(i) = paths_store.stack_unused_inodes.pop() {
            paths_store.paths_indexed_by_inode[i] = path.clone();
            i
        } else {
            let i = paths_store.paths_indexed_by_inode.len();
            paths_store.paths_indexed_by_inode.push(path.clone());
            i
        }
        .into();

        opened.insert(path, (Counter::default(), inode));
        inode
    }

    /// # Safety:
    /// It will panic if:
    /// - `inode` does not exist
    /// - `nlookup` is greater than the `counter` associated to the `inode`
    pub(super) unsafe fn remove_path(&self, inode: Inode, nlookup: u64) {
        let mut paths_store = self
            .inode_manager
            .paths_store
            .write()
            .expect("Mutex is poisoned");
        let mut opened = self.inode_manager.opened.lock().expect("Mutex is poisoned");
        let i: usize = inode.into();

        let path = &paths_store.paths_indexed_by_inode[i];

        if let Some((counter, _)) = opened.get_mut(path) {
            counter.decrement(nlookup);

            if counter.is_zero() {
                opened.remove(path);
                paths_store.stack_unused_inodes.push(i);
            }
        }
    }

    /// Safety:
    /// It will panic if:
    /// - `inode` does not exist
    pub(super) unsafe fn get_path(&self, inode: Inode) -> PathBuf {
        let paths_store = self
            .inode_manager
            .paths_store
            .read()
            .expect("Mutex is poisoned");

        paths_store.paths_indexed_by_inode[usize::from(inode)].clone()
    }

    pub(super) fn rename_path(&self, source: &Path, destination: &Path) {
        let mut paths_store = self
            .inode_manager
            .paths_store
            .write()
            .expect("Mutex is poisoned");
        let mut opened = self.inode_manager.opened.lock().expect("Mutex is poisoned");
        let source_bytes = source.as_os_str().as_bytes();
        let destination_bytes = destination.as_os_str().as_bytes();

        for path in paths_store
            .paths_indexed_by_inode
            .iter_mut()
            .filter(|path| path.as_os_str().as_bytes().starts_with(source_bytes))
        {
            let path_bytes = path.as_os_str().as_bytes();

            let mut new_path =
                Vec::with_capacity(path_bytes.len() - source_bytes.len() + destination_bytes.len());

            new_path.extend(destination_bytes);
            new_path.extend(&path_bytes[source_bytes.len()..]);

            *path.as_mut_os_string() = OsString::from_vec(new_path);
        }

        *opened = opened
            .drain()
            .filter(|(path, _)| path.as_os_str().as_bytes().starts_with(source_bytes))
            .map(|(mut path, v)| {
                let path_bytes = path.as_os_str().as_bytes();

                let mut new_path = Vec::with_capacity(
                    path_bytes.len() - source_bytes.len() + destination_bytes.len(),
                );

                new_path.extend(destination_bytes);
                new_path.extend(&path_bytes[source_bytes.len()..]);
                *path.as_mut_os_string() = OsString::from_vec(new_path);

                (path, v)
            })
            .collect();
    }
}

#[cfg(test)]
#[path = "../../tests/unit/inode.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
