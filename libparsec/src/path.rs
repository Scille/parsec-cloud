// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::{EntryName, FsPath};

pub fn path_join(parent: &FsPath, child: &EntryName) -> FsPath {
    parent.join(child.clone())
}

pub fn path_parent(path: &FsPath) -> FsPath {
    path.parent()
}

pub fn path_filename(path: &FsPath) -> Option<&EntryName> {
    path.name()
}

pub fn path_split(path: &FsPath) -> Vec<EntryName> {
    path.parts().to_vec()
}

pub fn path_normalize(path: FsPath) -> FsPath {
    path.normalize()
}
