// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use serde::Serialize;
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

/// Special Path which implement `AsRef<str>` and `FromStr` to be used in a generator
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct StrPath(PathBuf);

/// We do this trick to generate client code, this is not used in parsec
impl std::str::FromStr for StrPath {
    // Type Err require AsRef<str> & Display for generator
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

/// We do this trick to generate client code, this is not used in parsec
impl AsRef<str> for StrPath {
    fn as_ref(&self) -> &str {
        self.to_str().expect("Couldn't convert path to str")
    }
}

impl AsRef<Path> for StrPath {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl std::ops::Deref for StrPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for StrPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<PathBuf> for StrPath {
    fn from(path: PathBuf) -> Self {
        Self(path)
    }
}

impl From<&Path> for StrPath {
    fn from(path: &Path) -> Self {
        Self(path.to_path_buf())
    }
}

impl From<&str> for StrPath {
    fn from(path: &str) -> Self {
        Self(PathBuf::from(path))
    }
}

impl From<StrPath> for PathBuf {
    fn from(path: StrPath) -> Self {
        path.0
    }
}
