// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::*;
use thiserror::Error;
use unicode_normalization::UnicodeNormalization;

#[derive(Error, Debug)]
pub enum EntryNameError {
    #[error("Name too long")]
    NameTooLong,
    #[error("Invalid name")]
    InvalidName,
}

pub type EntryNameResult<T> = Result<T, EntryNameError>;

#[derive(Error, Debug)]
pub enum FsPathError {
    #[error("Path must be absolute")]
    NotAbsolute,
    #[error(transparent)]
    InvalidEntry(#[from] EntryNameError),
}

/*
 * EntryName
 */

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(try_from = "&str", into = "String")]
pub struct EntryName(String);

impl EntryName {
    pub const MAX_LENGTH_BYTES: usize = 255;

    /// Entry name is made of valid UTF8, plus follows the UNIX filesystem philosophy:
    /// - no `.` or `..` name
    /// - no `/` or null byte in the name
    /// - max 255 bytes long name
    pub fn is_valid(raw: &str) -> Result<(), EntryNameError> {
        if raw.len() > Self::MAX_LENGTH_BYTES {
            Err(EntryNameError::NameTooLong)
        } else if raw.is_empty()
            || raw == "."
            || raw == ".."
            || raw.find('/').is_some()
            || raw.find('\x00').is_some()
        {
            Err(EntryNameError::InvalidName)
        } else {
            Ok(())
        }
    }

    pub fn prefix(&self) -> &str {
        self.prefix_and_suffix().0
    }

    pub fn suffix(&self) -> Option<&str> {
        self.prefix_and_suffix().1
    }

    /// The prefix is:
    ///
    /// - The entire name if there is no embedded `.`;
    /// - The portion of the name before the first non-beginning `.`;
    /// - The entire name if the name begins with `.` and has no other `.`s within;
    /// - The portion of the name before the second `.` if the name begins with `.`
    ///
    /// Example:
    /// - `foo.tar.gz` -> prefix: `foo` suffix: `tar.gz`
    /// - `.foo.txt` -> prefix: `.foo` suffix: `txt`
    pub fn prefix_and_suffix(&self) -> (&str, Option<&str>) {
        match self.0.split_once('.') {
            Some((base, ext)) => {
                // Check for special case of hidden UNIX file (e.g. `.foo`)
                if !base.is_empty() {
                    (base, Some(ext))
                } else {
                    match ext.find('.') {
                        // The name had multiple dots (e.g. `.foo.txt`)
                        Some(i) => {
                            let (base, ext) = self.0.split_at(i + 1);
                            (base, Some(&ext[1..]))
                        }
                        // The name contained a single leading dot
                        None => (self.0.as_str(), None),
                    }
                }
            }
            // Name without any dot
            None => (self.0.as_str(), None),
        }
    }
}

impl std::convert::AsRef<str> for EntryName {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for EntryName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for EntryName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display = self.to_string();
        f.debug_tuple("EntryName").field(&display).finish()
    }
}

impl TryFrom<&str> for EntryName {
    type Error = EntryNameError;

    fn try_from(id: &str) -> Result<Self, Self::Error> {
        // NFC normalization is an important step to ensure a given name is always
        // encoded the same way (e.g. "é" can also be encoded in UTF8 as "<accent>+e").
        let id: String = id.nfc().collect();

        Self::is_valid(&id).map(|_| Self(id))
    }
}

impl TryFrom<String> for EntryName {
    type Error = EntryNameError;

    fn try_from(id: String) -> Result<Self, Self::Error> {
        let id = match unicode_normalization::is_nfc_quick(id.chars()) {
            unicode_normalization::IsNormalized::Yes => id,
            unicode_normalization::IsNormalized::No
            | unicode_normalization::IsNormalized::Maybe => id.nfc().collect(),
        };

        Self::is_valid(&id).map(|_| Self(id))
    }
}

impl From<EntryName> for String {
    fn from(item: EntryName) -> String {
        item.0
    }
}

impl std::str::FromStr for EntryName {
    type Err = EntryNameError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EntryName::try_from(s)
    }
}

/*
 * EntryNameRef
 */

pub struct EntryNameRef<'a>(&'a str);

impl std::convert::AsRef<str> for EntryNameRef<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl std::fmt::Display for EntryNameRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for EntryNameRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = self.to_string();
        f.debug_tuple("EntryNameRef").field(&display).finish()
    }
}

impl<'a> From<&'a EntryName> for EntryNameRef<'a> {
    fn from(value: &'a EntryName) -> Self {
        EntryNameRef(&value.0)
    }
}

impl<'a> TryFrom<&'a str> for EntryNameRef<'a> {
    type Error = EntryNameError;

    fn try_from(id: &'a str) -> Result<Self, Self::Error> {
        if !unicode_normalization::is_nfc(id) {
            // TODO: better error here !
            return Err(Self::Error::InvalidName);
        }

        // Stick to UNIX filesystem philosophy:
        // - no `.` or `..` name
        // - no `/` or null byte in the name
        // - max 255 bytes long name
        if id.len() >= 256 {
            Err(Self::Error::NameTooLong)
        } else if id.is_empty()
            || id == "."
            || id == ".."
            || id.find('/').is_some()
            || id.find('\x00').is_some()
        {
            Err(Self::Error::InvalidName)
        } else {
            Ok(Self(id))
        }
    }
}

/*
 * FsPath
 */

#[derive(Clone, PartialEq, Eq, PartialOrd, Hash, Default)]
pub struct FsPath {
    parts: Vec<EntryName>,
}

impl FsPath {
    pub fn name(&self) -> Option<&EntryName> {
        self.parts.last()
    }

    pub fn is_root(&self) -> bool {
        self.parts.is_empty()
    }

    pub fn parts(&self) -> &[EntryName] {
        &self.parts
    }

    pub fn parent(&self) -> Self {
        if self.parts.is_empty() {
            Self::default()
        } else {
            let parts = self.parts[..self.parts.len() - 1].to_owned();
            Self { parts }
        }
    }

    pub fn join(&self, child: EntryName) -> Self {
        let mut parts = Vec::with_capacity(self.parts.len() + 1);
        parts.extend_from_slice(&self.parts);
        parts.push(child);
        Self { parts }
    }

    pub fn into_child(mut self, child: EntryName) -> Self {
        self.parts.push(child);
        self
    }

    pub fn into_parent(mut self) -> (Self, Option<EntryName>) {
        let child = self.parts.pop();
        (self, child)
    }

    pub fn with_mountpoint(self, mountpoint: &std::path::Path) -> std::path::PathBuf {
        let mut path = mountpoint.to_path_buf();
        for item in &self.parts {
            path.push(item.as_ref());
        }
        path
    }

    pub fn is_descendant_of(&self, path: &Self) -> bool {
        if path.parts.len() > self.parts.len() {
            return false;
        }

        for (self_part, part) in self.parts().iter().zip(path.parts()) {
            if self_part != part {
                return false;
            }
        }

        true
    }

    /// Use `dest` for allocation and return it.
    pub fn replace_parent(&self, offset: usize, mut dest: Self) -> Self {
        if self.parts.len() > offset {
            dest.parts.extend_from_slice(&self.parts[offset..]);
        }
        dest
    }

    pub fn from_parts(parts: Vec<EntryName>) -> Self {
        Self { parts }
    }

    /// Normalize the path
    pub fn normalize(self) -> Self {
        let mut new_parts = Vec::with_capacity(self.parts.len());

        for part in self.parts {
            match part.0.as_str() {
                "." | "" => continue,
                ".." => {
                    new_parts.pop();
                }
                _ => new_parts.push(part),
            }
        }
        Self { parts: new_parts }
    }
}

impl std::fmt::Debug for FsPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display = self.to_string();
        f.debug_tuple("FsPath").field(&display).finish()
    }
}

impl std::fmt::Display for FsPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_root() {
            f.write_str("/")?;
        } else {
            for part in self.parts() {
                f.write_str("/")?;
                f.write_str(part.as_ref())?;
            }
        }
        Ok(())
    }
}

impl std::str::FromStr for FsPath {
    type Err = FsPathError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('/') {
            return Err(FsPathError::NotAbsolute);
        }

        let splitted = s.split('/');
        let size_hint = splitted.size_hint();
        let mut parts = Vec::with_capacity(size_hint.1.unwrap_or(size_hint.0));
        for item in splitted {
            match item {
                "." | "" => (),
                ".." => {
                    let _ = parts.pop();
                }
                item => {
                    parts.push(item.try_into()?);
                }
            }
        }
        Ok(Self { parts })
    }
}

#[cfg(test)]
#[path = "../tests/unit/fs_path.rs"]
mod tests;
