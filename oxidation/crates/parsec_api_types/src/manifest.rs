// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::*;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

use crate::data_macros::{impl_transparent_data_format_convertion, new_data_struct_type};
use crate::ext_types::{new_uuid_type, DateTimeExtFormat};
use crate::DeviceID;
use parsec_api_crypto::{HashDigest, SecretKey};

/*
 * EntryID, BlockID, RealmID, VlobID
 */

new_uuid_type!(pub EntryID);
new_uuid_type!(pub BlockID);
new_uuid_type!(pub RealmID);
new_uuid_type!(pub VlobID);

/*
 * BlockAccess
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockAccess {
    pub id: BlockID,
    pub key: SecretKey,
    pub offset: u32, // TODO: this limit file size to 8Go, is this okay ?
    pub size: u32,
    pub digest: HashDigest,
}

/*
 * RealmRole
 */

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RealmRole {
    Owner,
    Manager,
    Contributor,
    Reader,
}

impl Serialize for RealmRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let value = match self {
            RealmRole::Owner => "OWNER",
            RealmRole::Manager => "MANAGER",
            RealmRole::Contributor => "CONTRIBUTOR",
            RealmRole::Reader => "READER",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for RealmRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RealmRole;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(concat!("an user profile as string"))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "OWNER" => Ok(RealmRole::Owner),
                    "MANAGER" => Ok(RealmRole::Manager),
                    "CONTRIBUTOR" => Ok(RealmRole::Contributor),
                    "READER" => Ok(RealmRole::Reader),
                    _ => Err(serde::de::Error::invalid_type(
                        serde::de::Unexpected::Str(v),
                        &self,
                    )),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
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

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
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
pub fn generate_local_author_legacy_placeholder() -> DeviceID {
    lazy_static! {
        static ref LEGACY_PLACEHOLDER: DeviceID = LOCAL_AUTHOR_LEGACY_PLACEHOLDER
            .parse()
            .unwrap_or_else(|_| unreachable!());
    }
    LEGACY_PLACEHOLDER.clone()
}

macro_rules! impl_dump_sign_and_encrypt {
    ($name:ident) => {
        impl $name {
            pub fn dump_sign_and_encrypt(
                &self,
                author_signkey: &::parsec_api_crypto::SigningKey,
                key: &::parsec_api_crypto::SecretKey,
            ) -> Vec<u8> {
                let serialized = rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!());
                let mut e =
                    ::flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                use std::io::Write;
                e.write_all(&serialized).unwrap_or_else(|_| unreachable!());
                let compressed = e.finish().unwrap_or_else(|_| unreachable!());
                let signed = author_signkey.sign(&compressed);
                key.encrypt(&signed)
            }
        }
    };
}

macro_rules! impl_decrypt_verify_and_load {
    ($name:ident) => {
        impl $name {
            pub fn decrypt_verify_and_load(
                encrypted: &[u8],
                key: &::parsec_api_crypto::SecretKey,
                author_verify_key: &::parsec_api_crypto::VerifyKey,
                expected_author: &DeviceID,
                expected_timestamp: &DateTime<Utc>,
            ) -> Result<$name, &'static str> {
                let signed = key.decrypt(encrypted).map_err(|_| "Invalid encryption")?;
                let compressed = author_verify_key
                    .verify(&signed)
                    .map_err(|_| "Invalid signature")?;
                let mut serialized = vec![];
                use std::io::Read;
                ::flate2::read::ZlibDecoder::new(&compressed[..])
                    .read_to_end(&mut serialized)
                    .map_err(|_| "Invalid compression")?;
                let obj: $name =
                    rmp_serde::from_read_ref(&serialized).map_err(|_| "Invalid serialization")?;
                if &obj.author != expected_author {
                    return Err("Unexpected author");
                } else if &obj.timestamp != expected_timestamp {
                    Err("Unexpected timestamp")
                } else {
                    Ok(obj)
                }
            }
        }
    };
}

/*
 * FileManifest
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "FileManifestData", from = "FileManifestData")]
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
    pub size: u32, // TODO: this limit file size to 8Go, is this okay ?
    /// Size of a single block
    pub blocksize: u32,
    pub blocks: Vec<BlockAccess>,
}

impl_dump_sign_and_encrypt!(FileManifest);
impl_decrypt_verify_and_load!(FileManifest);

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
    size: u32,
    blocksize: u32,
    blocks: Vec<BlockAccess>,

);

impl_transparent_data_format_convertion!(
    FileManifest,
    FileManifestData,
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
);

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

impl_dump_sign_and_encrypt!(FolderManifest);
impl_decrypt_verify_and_load!(FolderManifest);

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

impl_transparent_data_format_convertion!(
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

impl_dump_sign_and_encrypt!(WorkspaceManifest);
impl_decrypt_verify_and_load!(WorkspaceManifest);

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

impl_transparent_data_format_convertion!(
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

impl_dump_sign_and_encrypt!(UserManifest);
impl_decrypt_verify_and_load!(UserManifest);

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

impl_transparent_data_format_convertion!(
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
