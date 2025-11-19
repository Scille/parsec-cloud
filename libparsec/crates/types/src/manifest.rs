// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, num::NonZeroU64, ops::Deref, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_with::*;

use libparsec_crypto::{HashDigest, SecretKey, SigningKey, VerifyKey};
use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types,
    data_macros::impl_transparent_data_format_conversion,
    serialization::{format_v0_dump, format_vx_load},
    BlockID, DataError, DataResult, DateTime, DeviceID, EntryName, SizeInt, VersionInt, VlobID,
};

pub const DEFAULT_BLOCK_SIZE: Blocksize = Blocksize(512 * 1024); // 512 KB

macro_rules! impl_manifest_dump {
    ($name:ident) => {
        impl $name {
            /// Dump and sign [Self], this doesn't encrypt the data compared to [Self::dump_sign_and_encrypt]
            /// This enabled you to encrypt the data with another method than the one provided by [SecretKey]
            pub fn dump_and_sign(&self, author_signkey: &SigningKey) -> Vec<u8> {
                self.check_data_integrity().expect("Invalid manifest");
                let serialized = format_v0_dump(&self);
                author_signkey.sign(&serialized)
            }

            /// Dump and sign itself, then encrypt the resulting data using the provided [SecretKey]
            pub fn dump_sign_and_encrypt(
                &self,
                author_signkey: &SigningKey,
                key: &SecretKey,
            ) -> Vec<u8> {
                let signed = self.dump_and_sign(author_signkey);
                key.encrypt(&signed)
            }

            /// Test method to produce invalid payloads
            pub fn dump_sign_and_encrypt_with_data_integrity_checks_disabled(
                &self,
                author_signkey: &SigningKey,
                key: &SecretKey,
            ) -> Vec<u8> {
                let serialized = format_v0_dump(&self);
                let signed = author_signkey.sign(&serialized);
                key.encrypt(&signed)
            }
        }
    };
}

macro_rules! impl_manifest_load {
    ($name:ident) => {
        impl $name {
            pub fn decrypt_verify_and_load(
                encrypted: &[u8],
                key: &SecretKey,
                author_verify_key: &VerifyKey,
                expected_author: DeviceID,
                expected_timestamp: DateTime,
                expected_id: Option<VlobID>,
                expected_version: Option<VersionInt>,
            ) -> DataResult<Self> {
                let signed = key.decrypt(encrypted).map_err(|_| DataError::Decryption)?;

                Self::verify_and_load(
                    &signed,
                    author_verify_key,
                    expected_author,
                    expected_timestamp,
                    expected_id,
                    expected_version,
                )
            }

            /// Verify the signed value using the given `verify_key`
            /// And then, it will check for the expected values
            pub fn verify_and_load(
                signed: &[u8],
                author_verify_key: &VerifyKey,
                expected_author: DeviceID,
                expected_timestamp: DateTime,
                expected_id: Option<VlobID>,
                expected_version: Option<VersionInt>,
            ) -> DataResult<Self> {
                let serialized = author_verify_key
                    .verify(&signed)
                    .map_err(|_| DataError::Signature)?;

                let obj: Self = format_vx_load(&serialized)?;

                obj.check_data_integrity()?;

                obj.verify(
                    expected_author,
                    expected_timestamp,
                    expected_id,
                    expected_version,
                )?;

                Ok(obj)
            }
        }
    };
}

macro_rules! impl_manifest_verify {
    ($name:ident) => {
        impl $name {
            /// Verify the manifest against a set of expected values
            pub fn verify(
                &self,
                expected_author: DeviceID,
                expected_timestamp: DateTime,
                expected_id: Option<VlobID>,
                expected_version: Option<VersionInt>,
            ) -> DataResult<()> {
                if self.author != expected_author {
                    return Err(DataError::UnexpectedAuthor {
                        expected: expected_author,
                        got: Some(self.author),
                    });
                }

                if self.timestamp != expected_timestamp {
                    return Err(DataError::UnexpectedTimestamp {
                        expected: expected_timestamp,
                        got: self.timestamp,
                    });
                }

                if let Some(expected) = expected_id {
                    if self.id != expected {
                        return Err(DataError::UnexpectedId {
                            expected,
                            got: self.id,
                        });
                    }
                }

                if let Some(expected) = expected_version {
                    if self.version != expected {
                        return Err(DataError::UnexpectedVersion {
                            expected,
                            got: self.version,
                        });
                    }
                }

                Ok(())
            }
        }
    };
}

/*
 * BlockAccess
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BlockAccess {
    pub id: BlockID,
    /// Offset within the file where the block should be placed.
    ///
    /// Note this should not be confused with the offset *within the block* itself.
    pub offset: SizeInt,
    /// Size of the block.
    ///
    /// Note the size is checked along with the hash digest when validating the
    /// block integrity.
    pub size: NonZeroU64,
    pub digest: HashDigest,
}

/*
 * RealmRole
 */

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RealmRole {
    /// Owner can give/remove all roles (including Owner) and have read/write access
    Owner,
    /// Manager can give/remove Contributor/Reader roles and have read/write access
    Manager,
    /// Contributor have read/write access
    Contributor,
    /// Reader only have read access
    Reader,
}

impl RealmRole {
    pub fn can_read(&self) -> bool {
        true
    }

    pub fn can_write(&self) -> bool {
        matches!(
            self,
            RealmRole::Owner | RealmRole::Manager | RealmRole::Contributor
        )
    }

    pub fn can_grant_non_owner_role(&self) -> bool {
        matches!(self, RealmRole::Owner | RealmRole::Manager)
    }

    pub fn can_grant_owner_role(&self) -> bool {
        matches!(self, RealmRole::Owner)
    }
}

impl std::str::FromStr for RealmRole {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "owner" => Ok(Self::Owner),
            "manager" => Ok(Self::Manager),
            "contributor" => Ok(Self::Contributor),
            "reader" => Ok(Self::Reader),
            _ => Err("Failed to parse RealmRole"),
        }
    }
}

impl std::fmt::Display for RealmRole {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::Owner => "owner",
            Self::Manager => "manager",
            Self::Contributor => "contributor",
            Self::Reader => "reader",
        })
    }
}

/*
 * Blocksize
 */

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Blocksize(SizeInt);

impl Blocksize {
    /// Return the inner value of the [Blocksize].
    pub const fn inner(&self) -> SizeInt {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidBlockSize;

impl std::error::Error for InvalidBlockSize {}

impl std::fmt::Display for InvalidBlockSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid blocksize")
    }
}

impl TryFrom<SizeInt> for Blocksize {
    type Error = InvalidBlockSize;
    fn try_from(data: SizeInt) -> Result<Self, Self::Error> {
        if data < 8 {
            return Err(InvalidBlockSize);
        }

        Ok(Self(data))
    }
}

impl From<Blocksize> for SizeInt {
    fn from(data: Blocksize) -> Self {
        data.0
    }
}

impl Deref for Blocksize {
    type Target = SizeInt;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
 * FileManifest
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "FileManifestData", try_from = "FileManifestData")]
pub struct FileManifest {
    pub author: DeviceID,
    pub timestamp: DateTime,

    pub id: VlobID,
    pub parent: VlobID,
    // Version 0 means the data is not synchronized
    pub version: VersionInt,
    pub created: DateTime,
    pub updated: DateTime,
    /// Total size of the file
    pub size: SizeInt,
    /// Size of a single block.
    ///
    /// Each block must have a size of `blocksize` except the last one (that is allowed
    /// to be smaller).
    ///
    /// This implies:
    /// - Each block access has an offset aligned on `blocksize`.
    /// - If `blocksize` is updated (typically if the file grows too big, although not
    ///   implemented at the moment), all blocks must be reshaped to match the new blocksize.
    pub blocksize: Blocksize,
    pub blocks: Vec<BlockAccess>,
}

impl_manifest_dump!(FileManifest);
impl_manifest_verify!(FileManifest);

parsec_data!("schema/manifest/file_manifest.json5");

impl FileManifest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        author: DeviceID,
        timestamp: DateTime,
        id: VlobID,
        parent: VlobID,
        version: VersionInt,
        created: DateTime,
        updated: DateTime,
        size: SizeInt,
        blocksize: Blocksize,
        blocks: Vec<BlockAccess>,
    ) -> Self {
        let manifest = Self {
            author,
            timestamp,
            id,
            parent,
            version,
            created,
            updated,
            size,
            blocksize,
            blocks,
        };
        manifest.check_data_integrity().expect("Invalid manifest");
        manifest
    }

    /// The blocks in a file manifest should:
    /// - be ordered by offset
    /// - not overlap
    /// - not go passed the file size
    /// - not share the same block span
    /// - not span over multiple block spans
    ///
    /// Note that they do not have to be contiguous.
    /// Those checks have to remain compatible with `LocalFileManifest::check_data_integrity`.
    /// Also, the id and parent id should be different so the manifest does not point to itself.
    ///
    /// Note about this method being private:
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used:
    /// - As a sanity check when the manifest is created (i.e. in `FileManifest::new`).
    /// - During deserialization (i.e. in `FileManifest::verify_and_load`).
    fn check_data_integrity(&self) -> DataResult<()> {
        // Check that id and parent are different
        if self.id == self.parent {
            return Err(DataError::DataIntegrity {
                data_type: std::any::type_name::<Self>(),
                invariant: "id and parent are different",
            });
        }

        let mut current_offset = 0;
        let mut current_block_index = 0;

        for block in &self.blocks {
            // Check that blocks are ordered and not overlapping
            if current_offset > block.offset {
                return Err(DataError::DataIntegrity {
                    data_type: std::any::type_name::<Self>(),
                    invariant: "blocks are ordered and not overlapping",
                });
            }
            current_offset = block.offset + block.size.get();

            // Check that blocks are not sharing the same block span
            let block_index = block.offset / self.blocksize.inner();
            if current_block_index > block_index {
                return Err(DataError::DataIntegrity {
                    data_type: std::any::type_name::<Self>(),
                    invariant: "blocks are not sharing the same block span",
                });
            }
            current_block_index = block_index + 1;

            // Check that blocks are not spanning over multiple block spans
            let last_block_index = (block.offset + block.size.get() - 1) / self.blocksize.inner();
            if last_block_index != block_index {
                return Err(DataError::DataIntegrity {
                    data_type: std::any::type_name::<Self>(),
                    invariant: "blocks are not spanning over multiple block spans",
                });
            }
        }
        // Check that the file size is not exceeded
        if current_offset > self.size {
            return Err(DataError::DataIntegrity {
                data_type: std::any::type_name::<Self>(),
                invariant: "file size is not exceeded",
            });
        }
        Ok(())
    }
}

impl TryFrom<FileManifestData> for FileManifest {
    type Error = InvalidBlockSize;
    fn try_from(data: FileManifestData) -> Result<Self, Self::Error> {
        Ok(Self {
            author: data.author,
            timestamp: data.timestamp,
            id: data.id,
            parent: data.parent,
            version: data.version,
            created: data.created,
            updated: data.updated,
            size: data.size,
            blocksize: data.blocksize.try_into()?,
            blocks: data.blocks,
        })
    }
}

impl From<FileManifest> for FileManifestData {
    fn from(obj: FileManifest) -> Self {
        Self {
            ty: Default::default(),
            author: obj.author,
            timestamp: obj.timestamp,
            id: obj.id,
            parent: obj.parent,
            version: obj.version,
            created: obj.created,
            updated: obj.updated,
            size: obj.size,
            blocksize: obj.blocksize.into(),
            blocks: obj.blocks,
        }
    }
}

/*
 * FolderManifest
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "FolderManifestData", from = "FolderManifestData")]
pub struct FolderManifest {
    pub author: DeviceID,
    pub timestamp: DateTime,

    /// Note that, by convention, root folder manifest is identified by the realm ID
    pub id: VlobID,
    /// If root folder manifest, `parent` and `id` fields are equal
    pub parent: VlobID,
    // Version 0 means the data is not synchronized
    pub version: VersionInt,
    pub created: DateTime,
    pub updated: DateTime,
    pub children: HashMap<EntryName, VlobID>,
}

impl FolderManifest {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used as sanity check right before serialization)
    /// and not exposed publicly.
    ///
    /// Note that this method does not perform data integrity check related to the manifest being a
    /// child or root manifest.
    fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    fn check_data_integrity_as_child(&self) -> DataResult<()> {
        self.check_data_integrity()?;

        // Check that id and parent are different
        if self.id == self.parent {
            return Err(DataError::DataIntegrity {
                data_type: std::any::type_name::<Self>(),
                invariant: "id and parent are different for child manifest",
            });
        }
        Ok(())
    }

    fn check_data_integrity_as_root(&self) -> DataResult<()> {
        self.check_data_integrity()?;

        // Check that id and parent are different
        if self.id != self.parent {
            return Err(DataError::DataIntegrity {
                data_type: std::any::type_name::<Self>(),
                invariant: "id and parent are the same for root manifest",
            });
        }
        Ok(())
    }

    pub fn is_root(&self) -> bool {
        self.id == self.parent
    }
}

impl_manifest_dump!(FolderManifest);
impl_manifest_verify!(FolderManifest);

parsec_data!("schema/manifest/folder_manifest.json5");

impl_transparent_data_format_conversion!(
    FolderManifest,
    FolderManifestData,
    author,
    timestamp,
    id,
    parent,
    version,
    created,
    updated,
    children,
);

/*
 * UserManifest
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "UserManifestData", from = "UserManifestData")]
pub struct UserManifest {
    pub author: DeviceID,
    pub timestamp: DateTime,

    // Note that, by convention, user manifest is identified by the realm ID
    pub id: VlobID,
    // Version 0 means the data is not synchronized
    pub version: VersionInt,
    pub created: DateTime,
    pub updated: DateTime,
}

parsec_data!("schema/manifest/user_manifest.json5");

impl_manifest_dump!(UserManifest);
impl_manifest_load!(UserManifest);
impl_manifest_verify!(UserManifest);

impl_transparent_data_format_conversion!(
    UserManifest,
    UserManifestData,
    author,
    timestamp,
    id,
    version,
    created,
    updated,
);

impl UserManifest {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    // Test methods

    #[cfg(test)]
    pub fn deserialize_data(raw: &[u8]) -> DataResult<Self> {
        format_vx_load(raw)
    }
}

/*
 * ArcChildManifest & ChildManifest
 */

#[derive(Debug, Clone)]
pub enum ArcChildManifest {
    File(Arc<FileManifest>),
    Folder(Arc<FolderManifest>),
}

impl ArcChildManifest {
    pub fn id(&self) -> VlobID {
        match self {
            ArcChildManifest::File(m) => m.id,
            ArcChildManifest::Folder(m) => m.id,
        }
    }

    pub fn parent(&self) -> VlobID {
        match self {
            ArcChildManifest::File(m) => m.parent,
            ArcChildManifest::Folder(m) => m.parent,
        }
    }
}

impl From<Arc<FileManifest>> for ArcChildManifest {
    fn from(value: Arc<FileManifest>) -> Self {
        Self::File(value)
    }
}

impl From<Arc<FolderManifest>> for ArcChildManifest {
    fn from(value: Arc<FolderManifest>) -> Self {
        Self::Folder(value)
    }
}

impl From<ChildManifest> for ArcChildManifest {
    fn from(value: ChildManifest) -> Self {
        match value {
            ChildManifest::File(file) => ArcChildManifest::File(Arc::new(file)),
            ChildManifest::Folder(folder) => ArcChildManifest::Folder(Arc::new(folder)),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ChildManifest {
    File(FileManifest),
    Folder(FolderManifest),
}

impl_manifest_load!(ChildManifest);

impl ChildManifest {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        match self {
            Self::File(file) => file.check_data_integrity(),
            Self::Folder(folder) => folder.check_data_integrity_as_child(),
        }
    }

    pub fn verify(
        &self,
        expected_author: DeviceID,
        expected_timestamp: DateTime,
        expected_id: Option<VlobID>,
        expected_version: Option<VersionInt>,
    ) -> DataResult<()> {
        match self {
            Self::File(file) => file.verify(
                expected_author,
                expected_timestamp,
                expected_id,
                expected_version,
            ),
            Self::Folder(folder) => folder.verify(
                expected_author,
                expected_timestamp,
                expected_id,
                expected_version,
            ),
        }
    }

    // Test methods

    #[cfg(test)]
    pub fn into_file_manifest(self) -> Option<FileManifest> {
        match self {
            Self::File(file) => Some(file),
            _ => None,
        }
    }

    #[cfg(test)]
    pub fn into_folder_manifest(self) -> Option<FolderManifest> {
        match self {
            Self::Folder(folder) => Some(folder),
            _ => None,
        }
    }

    #[cfg(test)]
    pub fn deserialize_data(raw: &[u8]) -> DataResult<Self> {
        format_vx_load(raw)
    }
}

impl From<FileManifest> for ChildManifest {
    fn from(value: FileManifest) -> Self {
        Self::File(value)
    }
}

impl From<FolderManifest> for ChildManifest {
    fn from(value: FolderManifest) -> Self {
        Self::Folder(value)
    }
}

/*
 * WorkspaceManifest
 */

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct WorkspaceManifest(FolderManifest);

impl_manifest_load!(WorkspaceManifest);

impl WorkspaceManifest {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        self.0.check_data_integrity_as_root()
    }

    pub fn verify(
        &self,
        expected_author: DeviceID,
        expected_timestamp: DateTime,
        expected_id: Option<VlobID>,
        expected_version: Option<VersionInt>,
    ) -> DataResult<()> {
        self.0.verify(
            expected_author,
            expected_timestamp,
            expected_id,
            expected_version,
        )
    }
}

impl From<FolderManifest> for WorkspaceManifest {
    fn from(value: FolderManifest) -> Self {
        Self(value)
    }
}

impl From<WorkspaceManifest> for FolderManifest {
    fn from(value: WorkspaceManifest) -> Self {
        value.0
    }
}

#[cfg(test)]
#[path = "../tests/unit/manifest.rs"]
mod tests;
