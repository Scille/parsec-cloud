// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, num::NonZeroU64, ops::Deref};

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

macro_rules! impl_manifest_dump_load {
    ($name:ident) => {
        impl $name {
            /// Dump and sign [Self], this doesn't encrypt the data compared to [Self::dump_sign_and_encrypt]
            /// This enabled you to encrypt the data with another method than the one provided by [SecretKey]
            pub fn dump_and_sign(&self, author_signkey: &SigningKey) -> Vec<u8> {
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

                obj.verify(
                    expected_author,
                    expected_timestamp,
                    expected_id,
                    expected_version,
                )?;

                Ok(obj)
            }

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
    pub offset: SizeInt,
    pub size: NonZeroU64,
    pub digest: HashDigest,
}

/*
 * RealmRole
 */

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
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

impl TryFrom<SizeInt> for Blocksize {
    type Error = &'static str;
    fn try_from(data: SizeInt) -> Result<Self, Self::Error> {
        if data < 8 {
            return Err("Invalid blocksize");
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
    /// Size of a single block
    pub blocksize: Blocksize,
    pub blocks: Vec<BlockAccess>,
}

impl_manifest_dump_load!(FileManifest);

parsec_data!("schema/manifest/file_manifest.json5");

impl FileManifest {
    /// The blocks in a file manifest should:
    /// - be ordered by offset
    /// - not overlap
    /// - not go passed the file size
    /// - not share the same block span
    /// - not span over multiple block spans
    /// Note that they do not have to be contiguous.
    /// Those checks have to remain compatible with `LocalFileManifest::check_integrity`.
    /// Also, the id and parent id should be different so the manifest does not point to itself.
    pub fn check_integrity(&self) -> DataResult<()> {
        // Check that id and parent are different
        if self.id == self.parent {
            return Err(DataError::FileManifestIntegrity {
                invariant: "id and parent are different",
            });
        }

        let mut current_offset = 0;
        let mut current_block_index = 0;

        for block in &self.blocks {
            // Check that blocks are ordered and not overlapping
            if current_offset > block.offset {
                return Err(DataError::FileManifestIntegrity {
                    invariant: "blocks are ordered and not overlapping",
                });
            }
            current_offset = block.offset + block.size.get();

            // Check that blocks are not sharing the same block span
            let block_index = block.offset / self.blocksize.inner();
            if current_block_index > block_index {
                return Err(DataError::FileManifestIntegrity {
                    invariant: "blocks are not sharing the same block span",
                });
            }
            current_block_index = block_index + 1;

            // Check that blocks are not spanning over multiple block spans
            let last_block_index = (block.offset + block.size.get() - 1) / self.blocksize.inner();
            if last_block_index != block_index {
                return Err(DataError::FileManifestIntegrity {
                    invariant: "blocks are not spanning over multiple block spans",
                });
            }
        }
        // Check that the file size is not exceeded
        if current_offset > self.size {
            return Err(DataError::FileManifestIntegrity {
                invariant: "file size is not exceeded",
            });
        }
        Ok(())
    }
}

impl TryFrom<FileManifestData> for FileManifest {
    type Error = &'static str;
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
    pub fn check_integrity(&self) -> DataResult<()> {
        // Check that id and parent are different
        if self.id == self.parent {
            return Err(DataError::FolderManifestIntegrity {
                invariant: "id and parent are different",
            });
        }
        Ok(())
    }

    pub fn is_root(&self) -> bool {
        self.id == self.parent
    }
}

impl_manifest_dump_load!(FolderManifest);

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

impl_manifest_dump_load!(UserManifest);

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

/*
 * ChildManifest
 */

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ChildManifest {
    File(FileManifest),
    Folder(FolderManifest),
}

impl ChildManifest {
    pub fn check_integrity(&self) -> DataResult<()> {
        match self {
            Self::File(file) => file.check_integrity(),
            Self::Folder(folder) => folder.check_integrity(),
        }
    }

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

    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: DateTime,
        expected_id: Option<VlobID>,
        expected_version: Option<VersionInt>,
    ) -> DataResult<Self> {
        let serialized = author_verify_key
            .verify(signed)
            .map_err(|_| DataError::Signature)?;

        let obj = Self::deserialize_data(serialized)?;

        macro_rules! internal_verify {
            ($obj:ident) => {{
                $obj.verify(
                    expected_author,
                    expected_timestamp,
                    expected_id,
                    expected_version,
                )?;
            }};
        }

        match &obj {
            Self::File(file) => internal_verify!(file),
            Self::Folder(folder) => {
                internal_verify!(folder)
            }
        }
        Ok(obj)
    }

    fn deserialize_data(raw: &[u8]) -> DataResult<Self> {
        format_vx_load(raw)
    }
}

#[cfg(test)]
#[path = "../tests/unit/manifest.rs"]
mod tests;
