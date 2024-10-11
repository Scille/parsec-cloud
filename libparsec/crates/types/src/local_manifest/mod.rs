// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! Types used to store the content of the workspace on local storage.
//! These types are never sent to the server.
//!
//! The local manifests are based on remote manifests but contain additional information.
//! Unlike remote manifests that represent an immutable version of a data
//! that have been uploaded to the server, the local manifests represent the
//! data as it is currently known by the client:
//! - It is based on a remote manifest (see the `base` field).
//! - If the local manifest has just been created locally, then its base remote
//!   manifest is a placeholder with version = 0.
//! - It gets modified by the client (e.g. when a file is created, the parent folder
//!   manifest is updated to add the new file entry).
//! - It gets converted into a remote manifest when data is synchronized with the server.
//! - When synchronizing, the remote manifest (in which the local manifest is based on),
//!   is used to merge the local changes with the remote changes.
mod file;
mod folder;
mod user;

use std::sync::Arc;

use crate::{self as libparsec_types, DataResult, DateTime, DeviceID, VlobID};

pub use file::{
    ChunkView, ChunkViewGetBlockAccessError, ChunkViewPromoteAsBlockError, LocalFileManifest,
};
pub use folder::{LocalFolderManifest, UnconfinedLocalFolderManifest};
use serde::Deserialize;
pub use user::{CertificateBasedInfoOrigin, LocalUserManifest, LocalUserManifestWorkspaceEntry};

macro_rules! impl_local_manifest_dump {
    ($name:ident) => {
        impl $name {
            pub fn dump_and_encrypt(&self, key: &libparsec_crypto::SecretKey) -> Vec<u8> {
                self.check_data_integrity().expect("Manifest integrity");
                let serialized = $crate::serialization::format_v0_dump(&self);
                key.encrypt(&serialized)
            }
        }
    };
}

pub(super) use impl_local_manifest_dump;

macro_rules! impl_local_manifest_load {
    ($name:ident) => {
        impl $name {
            pub fn decrypt_and_load(
                encrypted: &[u8],
                key: &libparsec_crypto::SecretKey,
            ) -> libparsec_types::DataResult<Self> {
                let serialized = key
                    .decrypt(encrypted)
                    .map_err(|_| $crate::error::DataError::Decryption)?;
                let result = $crate::serialization::format_vx_load(&serialized);
                result.and_then(|manifest: Self| manifest.check_data_integrity().map(|_| manifest))
            }
        }
    };
}

pub(super) use impl_local_manifest_load;

/*
 * ArcLocalChildManifest & LocalChildManifest
 */

#[derive(Debug, Clone)]
pub enum ArcLocalChildManifest {
    File(Arc<LocalFileManifest>),
    Folder(Arc<LocalFolderManifest>),
}

impl ArcLocalChildManifest {
    pub fn id(&self) -> VlobID {
        match self {
            ArcLocalChildManifest::File(m) => m.base.id,
            ArcLocalChildManifest::Folder(m) => m.base.id,
        }
    }

    pub fn parent(&self) -> VlobID {
        match self {
            ArcLocalChildManifest::File(m) => m.parent,
            ArcLocalChildManifest::Folder(m) => m.parent,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum LocalChildManifest {
    File(LocalFileManifest),
    Folder(LocalFolderManifest),
}

impl_local_manifest_load!(LocalChildManifest);

impl LocalChildManifest {
    pub fn id(&self) -> VlobID {
        match self {
            Self::File(manifest) => manifest.base.id,
            Self::Folder(manifest) => manifest.base.id,
        }
    }

    pub fn need_sync(&self) -> bool {
        match self {
            Self::File(manifest) => manifest.need_sync,
            Self::Folder(manifest) => manifest.need_sync,
        }
    }

    pub fn base_version(&self) -> u32 {
        match self {
            Self::File(manifest) => manifest.base.version,
            Self::Folder(manifest) => manifest.base.version,
        }
    }

    /// This structure represents mutable data (it gets loaded from disk, updated, then stored back modified)
    /// Hence this `check_data_integrity`'s main goal is during deserialization.
    /// However it is also useful as sanity check:
    /// - Right before serialization
    /// - After any modification (hence the need for this method to be public)
    pub fn check_data_integrity(&self) -> DataResult<()> {
        match self {
            Self::File(manifest) => manifest.check_data_integrity()?,
            Self::Folder(manifest) => manifest.check_data_integrity_as_child()?,
        }
        Ok(())
    }
}

impl From<LocalFileManifest> for LocalChildManifest {
    fn from(value: LocalFileManifest) -> Self {
        Self::File(value)
    }
}

impl From<LocalFolderManifest> for LocalChildManifest {
    fn from(value: LocalFolderManifest) -> Self {
        Self::Folder(value)
    }
}

impl TryFrom<LocalChildManifest> for LocalFileManifest {
    type Error = ();

    fn try_from(value: LocalChildManifest) -> Result<Self, Self::Error> {
        match value {
            LocalChildManifest::File(manifest) => Ok(manifest),
            _ => Err(()),
        }
    }
}

impl TryFrom<LocalChildManifest> for LocalFolderManifest {
    type Error = ();

    fn try_from(value: LocalChildManifest) -> Result<Self, Self::Error> {
        match value {
            LocalChildManifest::Folder(manifest) => Ok(manifest),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LocalWorkspaceManifest(pub LocalFolderManifest);

impl_local_manifest_load!(LocalWorkspaceManifest);

impl LocalWorkspaceManifest {
    pub fn new(author: DeviceID, realm: VlobID, timestamp: DateTime, speculative: bool) -> Self {
        Self(LocalFolderManifest::new_root(
            author,
            realm,
            timestamp,
            speculative,
        ))
    }

    /// This structure represents mutable data (it gets loaded from disk, updated, then stored back modified)
    /// Hence this `check_data_integrity`'s main goal is during deserialization.
    /// However it is also useful as sanity check:
    /// - Right before serialization
    /// - After any modification (hence the need for this method to be public)
    fn check_data_integrity(&self) -> DataResult<()> {
        self.0.check_data_integrity_as_root()
    }
}

impl From<LocalFolderManifest> for LocalWorkspaceManifest {
    fn from(value: LocalFolderManifest) -> Self {
        Self(value)
    }
}

impl From<LocalWorkspaceManifest> for LocalFolderManifest {
    fn from(value: LocalWorkspaceManifest) -> Self {
        value.0
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[path = "../../tests/unit/local_manifest.rs"]
mod tests;
