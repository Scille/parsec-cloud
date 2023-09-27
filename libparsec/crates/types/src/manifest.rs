// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashMap,
    io::{Read, Write},
    num::NonZeroU64,
    ops::Deref,
};

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use serde::{Deserialize, Serialize};
use serde_with::*;

use libparsec_crypto::{HashDigest, SecretKey, SigningKey, VerifyKey};
use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types, data_macros::impl_transparent_data_format_conversion, BlockID,
    DataError, DataResult, DateTime, DeviceID, EntryName, IndexInt, SizeInt, VersionInt, VlobID,
};

pub const DEFAULT_BLOCK_SIZE: Blocksize = Blocksize(512 * 1024); // 512 KB

macro_rules! impl_manifest_dump_load {
    ($name:ident) => {
        impl $name {
            /// Dump and sign [Self], this doesn't encrypt the data compared to [Self::dump_sign_and_encrypt]
            /// This enabled you to encrypt the data with another method than the one provided by [SecretKey]
            pub fn dump_and_sign(&self, author_signkey: &SigningKey) -> Vec<u8> {
                let serialized =
                    ::rmp_serde::to_vec_named(&self).expect("object should be serializable");
                let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
                let compressed = e
                    .write_all(&serialized)
                    .and_then(|_| e.finish())
                    .expect("in-memory buffer should not fail");
                author_signkey.sign(&compressed)
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
                expected_author: &DeviceID,
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
                expected_author: &DeviceID,
                expected_timestamp: DateTime,
                expected_id: Option<VlobID>,
                expected_version: Option<VersionInt>,
            ) -> DataResult<Self> {
                let compressed = author_verify_key
                    .verify(&signed)
                    .map_err(|_| DataError::Signature)?;
                let mut serialized = vec![];

                ZlibDecoder::new(&compressed[..])
                    .read_to_end(&mut serialized)
                    .map_err(|_| DataError::Compression)?;

                let obj = rmp_serde::from_slice::<Self>(&serialized)
                    .map_err(|_| DataError::Serialization)?;

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
                expected_author: &DeviceID,
                expected_timestamp: DateTime,
                expected_id: Option<VlobID>,
                expected_version: Option<VersionInt>,
            ) -> DataResult<()> {
                if self.author != *expected_author {
                    return Err(DataError::UnexpectedAuthor {
                        expected: Box::new(expected_author.clone()),
                        got: Some(Box::new(self.author.clone())),
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
    pub key: SecretKey,
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

/*
 * WorkspaceEntry
 */

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceEntry {
    pub id: VlobID,
    pub name: EntryName,
    pub key: SecretKey,
    pub encryption_revision: IndexInt,
    pub encrypted_on: DateTime,
    // As they name suggest, `role`/`role_cached_on` are only cache information.
    // However they are no longer needed given certificates are now eagerly
    // fetched by the client (see `CertificatesOps` internals).
    // Hence why they are not public: they are only kept for compatibility reason.
    #[serde(rename = "role_cached_on")]
    pub legacy_role_cache_timestamp: DateTime,
    #[serde(rename = "role")]
    pub legacy_role_cache_value: Option<RealmRole>,
}

impl WorkspaceEntry {
    pub fn new(
        id: VlobID,
        name: EntryName,
        key: SecretKey,
        encryption_revision: IndexInt,
        encrypted_on: DateTime,
    ) -> Self {
        Self {
            id,
            name,
            key,
            encryption_revision,
            encrypted_on,
            legacy_role_cache_timestamp: DateTime::from_f64_with_us_precision(0.0),
            legacy_role_cache_value: None,
        }
    }

    pub fn generate(name: EntryName, timestamp: DateTime) -> Self {
        Self {
            id: VlobID::default(),
            name,
            key: SecretKey::generate(),
            encryption_revision: 1,
            encrypted_on: timestamp,
            legacy_role_cache_timestamp: timestamp,
            legacy_role_cache_value: Some(RealmRole::Owner),
        }
    }
}

/*
 * Helpers
 */

// Prior to Parsec version 1.14, author field in manifest was only mandatory
// for non-zero manifest version (i.e. unsynced local data had empty author field)
// TODO: remove this code ? (considering it is unlikely to still have unsynced
// data created from version < 1.14)

const LOCAL_AUTHOR_LEGACY_PLACEHOLDER: &str =
    "LOCAL_AUTHOR_LEGACY_PLACEHOLDER@LOCAL_AUTHOR_LEGACY_PLACEHOLDER";
fn generate_local_author_legacy_placeholder() -> DeviceID {
    lazy_static! {
        static ref LEGACY_PLACEHOLDER: DeviceID = LOCAL_AUTHOR_LEGACY_PLACEHOLDER
            .parse()
            .expect("`LOCAL_AUTHOR_LEGACY_PLACEHOLDER` string should be parsable");
    }
    LEGACY_PLACEHOLDER.clone()
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

    pub id: VlobID,
    pub parent: VlobID,
    // Version 0 means the data is not synchronized
    pub version: VersionInt,
    pub created: DateTime,
    pub updated: DateTime,
    pub children: HashMap<EntryName, VlobID>,
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
 * WorkspaceManifest
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "WorkspaceManifestData", from = "WorkspaceManifestData")]
pub struct WorkspaceManifest {
    pub author: DeviceID,
    pub timestamp: DateTime,

    // Note that, by convention, workspace manifest is identified by the realm ID
    pub id: VlobID,
    // Version 0 means the data is not synchronized
    pub version: VersionInt,
    pub created: DateTime,
    pub updated: DateTime,
    pub children: HashMap<EntryName, VlobID>,
}

parsec_data!("schema/manifest/workspace_manifest.json5");

impl_manifest_dump_load!(WorkspaceManifest);

impl_transparent_data_format_conversion!(
    WorkspaceManifest,
    WorkspaceManifestData,
    author,
    timestamp,
    id,
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
    pub last_processed_message: IndexInt,
    pub workspaces: Vec<WorkspaceEntry>,
}

impl UserManifest {
    pub fn get_workspace_entry(&self, realm_id: VlobID) -> Option<&WorkspaceEntry> {
        self.workspaces.iter().find(|x| x.id == realm_id)
    }
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
    last_processed_message,
    workspaces,
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
    pub fn decrypt_verify_and_load(
        encrypted: &[u8],
        key: &SecretKey,
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
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
        expected_author: &DeviceID,
        expected_timestamp: DateTime,
        expected_id: Option<VlobID>,
        expected_version: Option<VersionInt>,
    ) -> DataResult<Self> {
        let compressed = author_verify_key
            .verify(signed)
            .map_err(|_| DataError::Signature)?;

        let obj = Self::deserialize_data(compressed)?;

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

    fn deserialize_data(data: &[u8]) -> DataResult<Self> {
        let mut deserialized = Vec::new();

        ZlibDecoder::new(data)
            .read_to_end(&mut deserialized)
            .map_err(|_| DataError::Compression)?;

        rmp_serde::from_slice(&deserialized).map_err(|_| DataError::Serialization)
    }
}

#[cfg(test)]
#[path = "../tests/unit/manifest.rs"]
mod tests;
