// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::*;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

use crate::data_macros::{impl_transparent_data_format_conversion, new_data_struct_type};
use crate::ext_types::{new_uuid_type, DateTimeExtFormat};
use crate::{CompSignEncrypt, DeviceID, Verify};
use parsec_api_crypto::{HashDigest, SecretKey};

/*
 * EntryID, BlockID, RealmID, VlobID
 */

new_uuid_type!(pub EntryID);
new_uuid_type!(pub BlockID);
new_uuid_type!(pub RealmID);
new_uuid_type!(pub VlobID);
new_uuid_type!(pub ChunkID);

/*
 * BlockAccess
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockAccess {
    pub id: BlockID,
    pub key: SecretKey,
    pub offset: u64,
    pub size: u64,
    pub digest: HashDigest,
}

/*
 * RealmRole
 */

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RealmRole {
    Owner,
    Manager,
    Contributor,
    Reader,
}

/*
 * EntryName
 */

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "&str", into = "String")]
pub struct EntryName(String);

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = self.to_string();
        f.debug_tuple(stringify!(EntryName))
            .field(&display)
            .finish()
    }
}

impl TryFrom<&str> for EntryName {
    type Error = &'static str;

    fn try_from(id: &str) -> Result<Self, Self::Error> {
        let id: String = id.nfc().collect();

        // Stick to UNIX filesystem philosophy:
        // - no `.` or `..` name
        // - no `/` or null byte in the name
        // - max 255 bytes long name
        if id.len() >= 256 {
            Err("Name too long")
        } else if id.is_empty()
            || id == "."
            || id == ".."
            || id.find('/').is_some()
            || id.find('\x00').is_some()
        {
            Err("Invalid name")
        } else {
            Ok(Self(id))
        }
    }
}

impl From<EntryName> for String {
    fn from(item: EntryName) -> String {
        item.0
    }
}

impl std::str::FromStr for EntryName {
    type Err = &'static str;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EntryName::try_from(s)
    }
}

/*
 * ManifestEntry
 */

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ManifestEntry(pub EntryID);

impl AsRef<EntryID> for ManifestEntry {
    #[inline]
    fn as_ref(&self) -> &EntryID {
        &self.0
    }
}

impl From<EntryID> for ManifestEntry {
    fn from(entry_id: EntryID) -> Self {
        Self(entry_id)
    }
}

impl std::str::FromStr for ManifestEntry {
    type Err = &'static str;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.parse::<EntryID>()?))
    }
}

/*
 * WorkspaceEntry
 */

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceEntry {
    pub id: EntryID,
    pub name: EntryName,
    pub key: SecretKey,
    pub encryption_revision: u32,
    #[serde_as(as = "DateTimeExtFormat")]
    pub encrypted_on: DateTime<Utc>,
    #[serde_as(as = "DateTimeExtFormat")]
    pub role_cached_on: DateTime<Utc>,
    pub role: Option<RealmRole>,
}

impl WorkspaceEntry {
    pub fn generate(name: EntryName, timestamp: DateTime<Utc>) -> Self {
        Self {
            id: EntryID::default(),
            name,
            key: SecretKey::generate(),
            encryption_revision: 1,
            encrypted_on: timestamp,
            role_cached_on: timestamp,
            role: Some(RealmRole::Owner),
        }
    }

    pub fn is_revoked(&self) -> bool {
        self.role.is_none()
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
            .unwrap_or_else(|_| unreachable!());
    }
    LEGACY_PLACEHOLDER.clone()
}

/*
 * FileManifest
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "FileManifestData", try_from = "FileManifestData")]
pub struct FileManifest {
    pub author: DeviceID,
    pub timestamp: DateTime<Utc>,

    pub id: EntryID,
    pub parent: EntryID,
    // Version 0 means the data is not synchronized
    pub version: u32,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    /// Total size of the file
    pub size: u64,
    /// Size of a single block
    pub blocksize: u64,
    pub blocks: Vec<BlockAccess>,
}

impl FileManifest {
    pub fn new<T>(
        author: DeviceID,
        id: T,
        parent: T,
        version: u32,
        blocksize: u64,
        blocks: Vec<BlockAccess>,
    ) -> Result<Self, &'static str>
    where
        EntryID: From<T>,
    {
        if blocksize < 8 {
            return Err("Invalid blocksize");
        }

        let now = Utc::now();

        Ok(Self {
            author,
            timestamp: now,
            id: EntryID::from(id),
            parent: EntryID::from(parent),
            version,
            updated: now,
            created: now,
            size: blocks.len() as u64,
            blocksize,
            blocks,
        })
    }
}

impl Verify for FileManifest {
    fn author(&self) -> &DeviceID {
        &self.author
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

impl CompSignEncrypt for FileManifest {}

new_data_struct_type!(
    FileManifestData,
    type: "file_manifest",

    // Compatibility with versions <= 1.14
    #[serde(default = "generate_local_author_legacy_placeholder")]
    author: DeviceID,
    #[serde_as(as = "DateTimeExtFormat")]
    timestamp: DateTime<Utc>,
    id: EntryID,
    parent: EntryID,
    version: u32,
    #[serde_as(as = "DateTimeExtFormat")]
    created: DateTime<Utc>,
    #[serde_as(as = "DateTimeExtFormat")]
    updated: DateTime<Utc>,
    size: u64,
    blocksize: u64,
    blocks: Vec<BlockAccess>,

);

impl TryFrom<FileManifestData> for FileManifest {
    type Error = &'static str;
    fn try_from(data: FileManifestData) -> Result<Self, Self::Error> {
        if data.blocksize < 8 {
            return Err("Invalid blocksize");
        }

        Ok(Self {
            author: data.author,
            timestamp: data.timestamp,
            id: data.id,
            parent: data.parent,
            version: data.version,
            created: data.created,
            updated: data.updated,
            size: data.size,
            blocksize: data.blocksize,
            blocks: data.blocks,
        })
    }
}

impl From<FileManifest> for FileManifestData {
    fn from(obj: FileManifest) -> Self {
        Self {
            type_: FileManifestDataDataType,
            author: obj.author,
            timestamp: obj.timestamp,
            id: obj.id,
            parent: obj.parent,
            version: obj.version,
            created: obj.created,
            updated: obj.updated,
            size: obj.size,
            blocksize: obj.blocksize,
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
    pub timestamp: DateTime<Utc>,

    pub id: EntryID,
    pub parent: EntryID,
    // Version 0 means the data is not synchronized
    pub version: u32,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub children: HashMap<EntryName, ManifestEntry>,
}

impl Verify for FolderManifest {
    fn author(&self) -> &DeviceID {
        &self.author
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

impl CompSignEncrypt for FolderManifest {}

new_data_struct_type!(
    FolderManifestData,
    type: "folder_manifest",

    // Compatibility with versions <= 1.14
    #[serde(default = "generate_local_author_legacy_placeholder")]
    author: DeviceID,
    #[serde_as(as = "DateTimeExtFormat")]
    timestamp: DateTime<Utc>,
    id: EntryID,
    parent: EntryID,
    version: u32,
    #[serde_as(as = "DateTimeExtFormat")]
    created: DateTime<Utc>,
    #[serde_as(as = "DateTimeExtFormat")]
    updated: DateTime<Utc>,
    children: HashMap<EntryName, ManifestEntry>,
);

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
    pub timestamp: DateTime<Utc>,

    pub id: EntryID,
    // Version 0 means the data is not synchronized
    pub version: u32,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub children: HashMap<EntryName, ManifestEntry>,
}

impl Verify for WorkspaceManifest {
    fn author(&self) -> &DeviceID {
        &self.author
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

impl CompSignEncrypt for WorkspaceManifest {}

new_data_struct_type!(
    WorkspaceManifestData,
    type: "workspace_manifest",

    // Compatibility with versions <= 1.14
    #[serde(default = "generate_local_author_legacy_placeholder")]
    author: DeviceID,
    #[serde_as(as = "DateTimeExtFormat")]
    timestamp: DateTime<Utc>,
    id: EntryID,
    version: u32,
    #[serde_as(as = "DateTimeExtFormat")]
    created: DateTime<Utc>,
    #[serde_as(as = "DateTimeExtFormat")]
    updated: DateTime<Utc>,
    children: HashMap<EntryName, ManifestEntry>,
);

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
    pub timestamp: DateTime<Utc>,

    pub id: EntryID,
    // Version 0 means the data is not synchronized
    pub version: u32,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub last_processed_message: u32,
    pub workspaces: Vec<WorkspaceEntry>,
}

impl UserManifest {
    pub fn get_workspace_entry(&self, workspace_id: EntryID) -> Option<&WorkspaceEntry> {
        self.workspaces.iter().find(|x| x.id == workspace_id)
    }
}

impl Verify for UserManifest {
    fn author(&self) -> &DeviceID {
        &self.author
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

impl CompSignEncrypt for UserManifest {}

new_data_struct_type!(
    UserManifestData,
    type: "user_manifest",

    // Compatibility with versions <= 1.14
    #[serde(default = "generate_local_author_legacy_placeholder")]
    author: DeviceID,
    #[serde_as(as = "DateTimeExtFormat")]
    timestamp: DateTime<Utc>,
    id: EntryID,
    version: u32,
    #[serde_as(as = "DateTimeExtFormat")]
    created: DateTime<Utc>,
    #[serde_as(as = "DateTimeExtFormat")]
    updated: DateTime<Utc>,
    last_processed_message: u32,
    workspaces: Vec<WorkspaceEntry>,
);

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
 * FileOrFolderManifest
 */

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum FileOrFolderManifest {
    #[serde(rename = "file_manifest")]
    File(FileManifest),
    #[serde(rename = "folder_manifest")]
    Folder(FolderManifest),
}
