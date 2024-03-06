// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_types::FsPath;

/// `paths_indexed_by_inode` allocator indexed by `inode`
/// The index of path is the inode number and the free inodes are stored
/// `Pasteur` must contain his rage when he sees this code
struct PathsStore {
    paths_indexed_by_inode: Vec<FsPath>,
    stack_unused_inodes: Vec<Inode>,
}

impl Default for PathsStore {
    fn default() -> Self {
        let root: FsPath = "/".parse().expect("unreachable");

        Self {
            paths_indexed_by_inode: vec![
                // Inode 0 is error prone, so we fill it with a dummy value
                root.clone(),
                // Inode 1 is the proper root entry
                root,
            ],
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
pub(crate) type Inode = u64;

#[derive(Default)]
pub(crate) struct InodesManager {
    paths_store: PathsStore,
    opened: HashMap<FsPath, (Counter, Inode)>,
}

impl InodesManager {
    pub fn new() -> Self {
        Self {
            paths_store: PathsStore::default(),
            opened: HashMap::new(),
        }
    }

    pub(super) fn insert_path(&mut self, path: FsPath) -> Inode {
        if let Some((counter, inode)) = self.opened.get_mut(&path) {
            counter.increment();
            return *inode;
        }

        let inode = if let Some(inode) = self.paths_store.stack_unused_inodes.pop() {
            let index = inode as usize;
            self.paths_store.paths_indexed_by_inode[index] = path.clone();
            inode
        } else {
            let index = self.paths_store.paths_indexed_by_inode.len();
            self.paths_store.paths_indexed_by_inode.push(path.clone());
            index as Inode
        };

        self.opened.insert(path, (Counter::default(), inode));
        inode
    }

    /// It will panic if:
    /// - `inode` does not exist
    /// - `nlookup` is greater than the `counter` associated to the `inode`
    pub(super) fn remove_path_or_panic(&mut self, inode: Inode, nlookup: u64) {
        let index = inode as usize;
        let path = &self.paths_store.paths_indexed_by_inode[index];

        if let Some((counter, _)) = self.opened.get_mut(path) {
            counter.decrement(nlookup);

            if counter.is_zero() {
                self.opened.remove(path);
                self.paths_store.stack_unused_inodes.push(inode);
            }
        }
    }

    /// It will panic if:
    /// - `inode` does not exist
    pub(super) fn get_path_or_panic(&self, inode: Inode) -> FsPath {
        let index = inode as usize;
        self.paths_store.paths_indexed_by_inode[index].clone()
    }

    // TODO
    #[allow(unused)]
    pub(super) fn rename_path(&mut self, source: &FsPath, destination: &FsPath) {
        for path in self
            .paths_store
            .paths_indexed_by_inode
            .iter_mut()
            .filter(|path| path.is_descendant_of(source))
        {
            *path = path.replace_parent(source.parts().len(), destination.clone());
        }

        let opened = std::mem::take(&mut self.opened);
        self.opened = opened
            .into_iter()
            .map(|(path, ino)| {
                if path.is_descendant_of(source) {
                    (
                        path.replace_parent(source.parts().len(), destination.clone()),
                        ino,
                    )
                } else {
                    (path, ino)
                }
            })
            .collect();
    }
}
