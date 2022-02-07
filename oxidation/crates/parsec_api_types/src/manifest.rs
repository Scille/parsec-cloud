// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

use super::ext_types::new_uuid_type;
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

// /*
//  * WorkspaceEntry
//  */
// #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
// pub struct WorkspaceEntry {
//     pub id: EntryID,
//     pub name: EntryName,
//     pub key: SecretKey,
//     pub encryption_revision: u32,
//     #[serde(with = "datetime_as_msgpack_extension")]
//     pub encrypted_on: DateTime<Utc>,
//     #[serde(with = "datetime_as_msgpack_extension")]
//     pub role_cached_on: DateTime<Utc>,
//     pub role: Option<RealmRole>,
// }

// impl WorkspaceEntry {
//     pub fn new(name: &EntryName) -> Self {
//         let now = Utc::now();
//         Self {
//             id: EntryID::default(),
//             name: name.to_owned(),
//             key: SecretKey::generate(),
//             encryption_revision: 1,
//             encrypted_on: now,
//             role_cached_on: now,
//             role: Some(RealmRole::Owner),
//         }
//     }

//     pub fn is_revoked(self) -> bool {
//         self.role.is_none()
//     }
// }

// /*
//  * ManifestContent
//  */
// // Prior to Parsec version 1.14, author field in manifest was only mandatory
// // for non-zero manifest version (i.e. unsynced local data had empty author field)
// // TODO: remove this code ? (considering it is unlikely to still have unsynced
// // data created from version < 1.14)

// const LOCAL_AUTHOR_LEGACY_PLACEHOLDER: &str =
//     "LOCAL_AUTHOR_LEGACY_PLACEHOLDER@LOCAL_AUTHOR_LEGACY_PLACEHOLDER";
// fn generate_local_author_legacy_placeholder() -> DeviceID {
//     lazy_static! {
//         static ref LEGACY_PLACEHOLDER: DeviceID = LOCAL_AUTHOR_LEGACY_PLACEHOLDER.parse().unwrap();
//     }
//     LEGACY_PLACEHOLDER.clone()
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(tag = "type")]
// pub enum ManifestContent {
//     #[serde(rename = "file_manifest")]
//     File {
//         // Compatibility with versions <= 1.14
//         #[serde(default = "generate_local_author_legacy_placeholder")]
//         author: DeviceID,
//         #[serde(with = "datetime_as_msgpack_extension")]
//         timestamp: DateTime<Utc>,
//         id: EntryID,
//         parent: EntryID,
//         // Version 0 means the data is not synchronized
//         version: u32,
//         #[serde(with = "datetime_as_msgpack_extension")]
//         created: DateTime<Utc>,
//         #[serde(with = "datetime_as_msgpack_extension")]
//         updated: DateTime<Utc>,
//         /// Total size of the file
//         size: u32, // TODO: this limit file size to 8Go, is this okay ?
//         /// Size of a single block
//         blocksize: u32,
//         blocks: Vec<BlockAccess>,
//     },

//     #[serde(rename = "folder_manifest")]
//     Folder {
//         // Compatibility with versions <= 1.14
//         #[serde(default = "generate_local_author_legacy_placeholder")]
//         author: DeviceID,
//         #[serde(with = "datetime_as_msgpack_extension")]
//         timestamp: DateTime<Utc>,
//         id: EntryID,
//         parent: EntryID,
//         // Version 0 means the data is not synchronized
//         version: u32,
//         #[serde(with = "datetime_as_msgpack_extension")]
//         created: DateTime<Utc>,
//         #[serde(with = "datetime_as_msgpack_extension")]
//         updated: DateTime<Utc>,
//         children: HashMap<EntryName, ManifestEntry>,
//     },

//     #[serde(rename = "workspace_manifest")]
//     Workspace {
//         // Compatibility with versions <= 1.14
//         #[serde(default = "generate_local_author_legacy_placeholder")]
//         author: DeviceID,
//         #[serde(with = "datetime_as_msgpack_extension")]
//         timestamp: DateTime<Utc>,
//         id: EntryID,
//         // Version 0 means the data is not synchronized
//         version: u32,
//         #[serde(with = "datetime_as_msgpack_extension")]
//         created: DateTime<Utc>,
//         #[serde(with = "datetime_as_msgpack_extension")]
//         updated: DateTime<Utc>,
//         children: HashMap<EntryName, ManifestEntry>,
//     },

//     #[serde(rename = "user_manifest")]
//     User {
//         // Compatibility with versions <= 1.14
//         #[serde(default = "generate_local_author_legacy_placeholder")]
//         author: DeviceID,
//         #[serde(with = "datetime_as_msgpack_extension")]
//         timestamp: DateTime<Utc>,
//         id: EntryID,
//         // Version 0 means the data is not synchronized
//         version: u32,
//         #[serde(with = "datetime_as_msgpack_extension")]
//         created: DateTime<Utc>,
//         #[serde(with = "datetime_as_msgpack_extension")]
//         updated: DateTime<Utc>,
//         last_processed_message: u32,
//         workspaces: Vec<WorkspaceEntry>,
//     },
// }

// /*
//  * FileManifest
//  */
// #[derive(Debug, PartialEq, Eq)]
// pub struct FileManifest {
//     pub author: DeviceID,
//     pub timestamp: DateTime<Utc>,
//     pub id: EntryID,
//     pub parent: EntryID,
//     // Version 0 means the data is not synchronized
//     pub version: u32,
//     pub created: DateTime<Utc>,
//     pub updated: DateTime<Utc>,
//     /// Total size of the file
//     pub size: u32, // TODO: this limit file size to 8Go, is this okay ?
//     /// Size of a single block
//     pub blocksize: u32,
//     pub blocks: Vec<BlockAccess>,
// }

// /*
//  * FolderManifest
//  */
// #[derive(Debug, PartialEq, Eq)]
// pub struct FolderManifest {
//     pub author: DeviceID,
//     pub timestamp: DateTime<Utc>,
//     pub id: EntryID,
//     // Version 0 means the data is not synchronized
//     pub version: u32,
//     pub created: DateTime<Utc>,
//     pub updated: DateTime<Utc>,
//     pub children: HashMap<EntryName, ManifestEntry>,
// }

// /*
//  * WorkspaceManifest
//  */
// #[derive(Debug, PartialEq, Eq)]
// pub struct WorkspaceManifest {
//     pub author: DeviceID,
//     pub timestamp: DateTime<Utc>,
//     pub id: EntryID,
//     // Version 0 means the data is not synchronized
//     pub version: u32,
//     pub created: DateTime<Utc>,
//     pub updated: DateTime<Utc>,
//     pub children: HashMap<EntryName, ManifestEntry>,
// }

// /*
//  * UserManifest
//  */
// #[derive(Debug, PartialEq, Eq)]
// pub struct UserManifest {
//     pub author: DeviceID,
//     pub timestamp: DateTime<Utc>,
//     pub id: EntryID,
//     // Version 0 means the data is not synchronized
//     pub version: u32,
//     pub created: DateTime<Utc>,
//     pub updated: DateTime<Utc>,
//     pub last_processed_message: u32,
//     pub workspaces: Vec<WorkspaceEntry>,
// }

// impl UserManifest {
//     pub fn get_workspace_entry(&self, workspace_id: EntryID) -> Option<&WorkspaceEntry> {
//         self.workspaces.iter().find(|x| x.id == workspace_id)
//     }
// }
