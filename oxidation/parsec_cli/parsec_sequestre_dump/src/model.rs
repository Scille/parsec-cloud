// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use std::error::Error;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{block, info, vlob_atom};
use parsec_api_crypto::SecretKey;
use parsec_api_types::{DeviceID, FileManifest, FolderManifest, Manifest, WorkspaceManifest};

type Time = chrono::DateTime<chrono::Utc>;

#[cfg(test)]
use crate::schema::{device, realm_role, user_ as user};
#[cfg(test)]
use tests_fixtures::Device;

#[derive(Debug, Queryable, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub block_id: Vec<u8>,
    pub data: Vec<u8>,
    pub author: i32,
    pub size: i32,
    pub created_on: f64,
}

#[derive(Debug, Queryable, Serialize, Deserialize, PartialEq)]
pub struct VlobAtom {
    pub vlob_id: Vec<u8>,
    pub version: i32,
    pub blob: Vec<u8>,
    pub size: i32,
    pub author: i32,
    pub timestamp: f64,
}

#[derive(Debug, Queryable, Serialize, Deserialize, PartialEq)]
pub struct Info {
    pub magic: i32,
    pub version: i32,
    pub realm_id: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    pub author: DeviceID,
    pub timestamp: Time,
    pub id: Uuid,
    pub version: u32,
    pub created: Time,
    pub updated: Time,
    pub size: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Data>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub content: String,
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Data")
            .field("name", &self.name)
            .field("children", &self.children)
            .finish()
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            name: String::default(),
            author: DeviceID::default(),
            timestamp: chrono::Utc::now(),
            id: Uuid::default(),
            version: u32::default(),
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
            size: u64::default(),
            children: Vec::default(),
            content: String::default(),
        }
    }
}

impl PartialEq for Data {
    // This implementation is slow, it is used only in test
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.author == other.author
            && self.version == other.version
            && self.created == other.created
            && self.updated == other.updated
            && self.size == other.size
            && self
                .children
                .iter()
                .map(|x| other.children.contains(x))
                .reduce(|acc, x| acc & x)
                .unwrap_or(true)
            && self.content == other.content
    }
}

impl Data {
    fn load(
        conn: &mut SqliteConnection,
        key: &SecretKey,
        name: String,
        manifest: &[u8],
    ) -> Result<Self, Box<dyn Error>> {
        Ok(match Manifest::decrypt_and_load(manifest, key)? {
            Manifest::File(FileManifest {
                author,
                timestamp,
                id,
                version,
                created,
                updated,
                size,
                mut blocks,
                ..
            }) => {
                blocks.sort_by(|x, y| x.offset.cmp(&y.offset));

                let content = blocks
                    .iter()
                    .map(|x| {
                        block::table
                            .select(block::data)
                            .filter(block::block_id.eq((*x.id).as_ref()))
                            .first::<Vec<u8>>(conn)
                            .map(|x| String::from_utf8_lossy(&x).to_string())
                    })
                    .collect::<Result<_, _>>()?;

                Self {
                    name,
                    author,
                    timestamp: *timestamp.as_ref(),
                    id: *id,
                    version,
                    created: *created.as_ref(),
                    updated: *updated.as_ref(),
                    size,
                    children: vec![],
                    content,
                }
            }
            Manifest::Folder(FolderManifest {
                author,
                timestamp,
                id,
                version,
                created,
                updated,
                children,
                ..
            })
            | Manifest::Workspace(WorkspaceManifest {
                author,
                timestamp,
                id,
                version,
                created,
                updated,
                children,
                ..
            }) => {
                let children = children
                    .into_iter()
                    .map(|(name, id)| {
                        vlob_atom::table
                            .select(vlob_atom::blob)
                            .filter(vlob_atom::vlob_id.eq((*id).as_ref()))
                            .order_by(vlob_atom::timestamp.desc())
                            .first::<Vec<u8>>(conn)
                            .map(|x| Self::load(conn, key, name.as_ref().to_string(), &x))
                    })
                    .collect::<Result<Result<Vec<_>, _>, _>>()??;

                let size = children.iter().map(|x| x.size).sum();

                Self {
                    name,
                    author,
                    timestamp: *timestamp.as_ref(),
                    id: *id,
                    version,
                    created: *created.as_ref(),
                    updated: *updated.as_ref(),
                    size,
                    children,
                    content: String::new(),
                }
            }
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Workspace(pub Data);

impl Workspace {
    pub fn dump(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, &data)?;

        Ok(())
    }

    pub fn query_all(conn: &mut SqliteConnection, key: &SecretKey) -> Result<Self, Box<dyn Error>> {
        let workspace_id = info::table.select(info::realm_id).first::<Vec<u8>>(conn)?;

        let workspace = vlob_atom::table
            .select(vlob_atom::blob)
            .filter(vlob_atom::vlob_id.eq(workspace_id))
            .first::<Vec<u8>>(conn)?;

        Ok(Self(Data::load(conn, key, String::new(), &workspace)?))
    }

    #[cfg(test)]
    pub fn load(path: &str) -> Self {
        let data = std::fs::read(path).unwrap();
        serde_json::from_slice(&data).unwrap()
    }

    #[cfg(test)]
    pub fn init_db(conn: &mut SqliteConnection, device: &Device) {
        use parsec_api_crypto::HashDigest;
        use parsec_api_types::{BlockAccess, BlockID, Blocksize, DateTime, EntryID};
        use std::{collections::HashMap, num::NonZeroU64};

        let now = DateTime::now();
        let _now = now.get_f64_with_us_precision();
        let t0 = "2000-1-1T00:00:00Z".parse::<DateTime>().unwrap();
        let _t0 = t0.get_f64_with_us_precision();

        diesel::insert_or_ignore_into(realm_role::table)
            .values((
                realm_role::_id.eq(0),
                realm_role::role_certificate.eq(&b"role"[..]),
            ))
            .execute(conn)
            .unwrap();

        diesel::insert_or_ignore_into(user::table)
            .values((user::_id.eq(0), user::user_certificate.eq(&b"user"[..])))
            .execute(conn)
            .unwrap();

        diesel::insert_or_ignore_into(device::table)
            .values((
                device::_id.eq(0),
                device::device_certificate.eq(&b"device"[..]),
            ))
            .execute(conn)
            .unwrap();

        diesel::insert_or_ignore_into(block::table)
            .values((
                block::_id.eq(0),
                block::block_id.eq(&b"block_id"[..]),
                block::data.eq(&b"data"[..]),
                block::author.eq(0),
                block::size.eq(4),
                block::created_on.eq(0.),
            ))
            .execute(conn)
            .unwrap();

        // WorkspaceManifest
        // |_ FileManifest (file0)
        // |_ FolderManifest (folder0)
        //    |_ FileManifest (file00)
        //    |_ FolderManifest (folder00)
        //    |_ FolderManifest (folder01)
        //       |_ FileManifest (file010)
        //       |_ FileManifest (file011)

        let workspace_id = EntryID::default();
        let file0_id = EntryID::default();
        let folder0_id = EntryID::default();
        let file00_id = EntryID::default();
        let folder00_id = EntryID::default();
        let folder01_id = EntryID::default();
        let file010_id = EntryID::default();
        let file011_id = EntryID::default();

        let file0_block0_id = BlockID::default();
        let file0_block1_id = BlockID::default();
        let file0_block2_id = BlockID::default();
        let file00_block_id = BlockID::default();
        let file010_block_id = BlockID::default();
        let file011_block_id = BlockID::default();

        let file0_data0 = b"Hello ";
        let file0_data1 = b"World ";
        let file0_data2 = [0; 1];
        let file00_data = b"file00's content";
        let file010_data = b"file010's content";
        let file011_data = b"file011's content";

        let insert_manifest = |conn: &mut SqliteConnection, id: &[u8], manifest: &[u8]| {
            diesel::insert_or_ignore_into(vlob_atom::table)
                .values((
                    vlob_atom::vlob_id.eq(id),
                    vlob_atom::version.eq(1),
                    vlob_atom::blob.eq(manifest),
                    vlob_atom::size.eq(manifest.len() as i32),
                    vlob_atom::author.eq(0),
                    vlob_atom::timestamp.eq(_now),
                ))
                .execute(conn)
                .unwrap()
        };

        let insert_data = |conn: &mut SqliteConnection, id: &[u8], data: &[u8]| {
            diesel::insert_or_ignore_into(block::table)
                .values((
                    block::block_id.eq(id),
                    block::data.eq(data),
                    block::author.eq(0),
                    block::size.eq(data.len() as i32),
                    block::created_on.eq(_t0),
                ))
                .execute(conn)
                .unwrap()
        };

        diesel::insert_or_ignore_into(info::table)
            .values((
                info::magic.eq(87947),
                info::version.eq(1),
                info::realm_id.eq((*workspace_id).as_ref()),
            ))
            .execute(conn)
            .unwrap();

        let workspace_manifest = Manifest::Workspace(WorkspaceManifest {
            author: device.device_id.clone(),
            timestamp: now,
            id: workspace_id,
            version: 1,
            created: t0,
            updated: t0,
            children: HashMap::from([
                ("file0".parse().unwrap(), file0_id),
                ("folder0".parse().unwrap(), folder0_id),
            ]),
        })
        .dump_and_encrypt(&device.local_symkey)
        .unwrap();

        let file0 = Manifest::File(FileManifest {
            author: device.device_id.clone(),
            timestamp: now,
            id: file0_id,
            parent: workspace_id,
            version: 1,
            created: t0,
            updated: t0,
            size: (file0_data0.len() + file0_data1.len() + file0_data2.len()) as u64,
            blocksize: Blocksize::try_from(512).expect("Invalid blocksize"),
            blocks: vec![
                BlockAccess {
                    id: file0_block0_id,
                    key: device.local_symkey.clone(),
                    offset: 0,
                    size: NonZeroU64::try_from(file0_data0.len() as u64).unwrap(),
                    digest: HashDigest::from_data(&file0_data0[..]),
                },
                BlockAccess {
                    id: file0_block1_id,
                    key: device.local_symkey.clone(),
                    offset: file0_data0.len() as u64,
                    size: NonZeroU64::try_from(file0_data1.len() as u64).unwrap(),
                    digest: HashDigest::from_data(&file0_data1[..]),
                },
                BlockAccess {
                    id: file0_block2_id,
                    key: device.local_symkey.clone(),
                    offset: (file0_data0.len() + file0_data1.len()) as u64,
                    size: NonZeroU64::try_from(file0_data2.len() as u64).unwrap(),
                    digest: HashDigest::from_data(&file0_data2[..]),
                },
            ],
        })
        .dump_and_encrypt(&device.local_symkey)
        .unwrap();

        let folder0 = Manifest::Folder(FolderManifest {
            author: device.device_id.clone(),
            timestamp: now,
            id: folder0_id,
            parent: workspace_id,
            version: 1,
            created: t0,
            updated: t0,
            children: HashMap::from([
                ("file00".parse().unwrap(), file00_id),
                ("folder00".parse().unwrap(), folder00_id),
                ("folder01".parse().unwrap(), folder01_id),
            ]),
        })
        .dump_and_encrypt(&device.local_symkey)
        .unwrap();

        let file00 = Manifest::File(FileManifest {
            author: device.device_id.clone(),
            timestamp: now,
            id: file00_id,
            parent: EntryID::from(folder0_id),
            version: 1,
            created: t0,
            updated: t0,
            size: file00_data.len() as u64,
            blocksize: Blocksize::try_from(512).expect("Invalid blocksize"),
            blocks: vec![BlockAccess {
                id: file00_block_id,
                key: device.local_symkey.clone(),
                offset: 0,
                size: NonZeroU64::try_from(file00_data.len() as u64).unwrap(),
                digest: HashDigest::from_data(&file00_data[..]),
            }],
        })
        .dump_and_encrypt(&device.local_symkey)
        .unwrap();

        let folder00 = Manifest::Folder(FolderManifest {
            author: device.device_id.clone(),
            timestamp: now,
            id: folder00_id,
            parent: EntryID::from(folder0_id),
            version: 1,
            created: t0,
            updated: t0,
            children: HashMap::new(),
        })
        .dump_and_encrypt(&device.local_symkey)
        .unwrap();

        let folder01 = Manifest::Folder(FolderManifest {
            author: device.device_id.clone(),
            timestamp: now,
            id: folder01_id,
            parent: EntryID::from(folder0_id),
            version: 1,
            created: t0,
            updated: t0,
            children: HashMap::from([
                ("file010".parse().unwrap(), file010_id),
                ("file011".parse().unwrap(), file011_id),
            ]),
        })
        .dump_and_encrypt(&device.local_symkey)
        .unwrap();

        let file010 = Manifest::File(FileManifest {
            author: device.device_id.clone(),
            timestamp: now,
            id: file010_id,
            parent: EntryID::from(folder01_id),
            version: 1,
            created: t0,
            updated: t0,
            size: file010_data.len() as u64,
            blocksize: Blocksize::try_from(512).expect("Invalid blocksize"),
            blocks: vec![BlockAccess {
                id: file010_block_id,
                key: device.local_symkey.clone(),
                offset: 0,
                size: NonZeroU64::try_from(file010_data.len() as u64).unwrap(),
                digest: HashDigest::from_data(&file010_data[..]),
            }],
        })
        .dump_and_encrypt(&device.local_symkey)
        .unwrap();

        let file011 = Manifest::File(FileManifest {
            author: device.device_id.clone(),
            timestamp: now,
            id: file011_id,
            parent: EntryID::from(folder01_id),
            version: 1,
            created: t0,
            updated: t0,
            size: file011_data.len() as u64,
            blocksize: Blocksize::try_from(512).expect("Invalid blocksize"),
            blocks: vec![BlockAccess {
                id: file011_block_id,
                key: device.local_symkey.clone(),
                offset: 0,
                size: NonZeroU64::try_from(file011_data.len() as u64).unwrap(),
                digest: HashDigest::from_data(&file011_data[..]),
            }],
        })
        .dump_and_encrypt(&device.local_symkey)
        .unwrap();

        insert_manifest(conn, (*workspace_id).as_ref(), &workspace_manifest);
        insert_manifest(conn, (*file0_id).as_ref(), &file0);
        insert_data(conn, (*file0_block0_id).as_ref(), &file0_data0[..]);
        insert_data(conn, (*file0_block1_id).as_ref(), &file0_data1[..]);
        insert_data(conn, (*file0_block2_id).as_ref(), &file0_data2[..]);
        insert_manifest(conn, (*folder0_id).as_ref(), &folder0);
        insert_manifest(conn, (*file00_id).as_ref(), &file00);
        insert_data(conn, (*file00_block_id).as_ref(), &file00_data[..]);
        insert_manifest(conn, (*folder00_id).as_ref(), &folder00);
        insert_manifest(conn, (*folder01_id).as_ref(), &folder01);
        insert_manifest(conn, (*file010_id).as_ref(), &file010);
        insert_data(conn, (*file010_block_id).as_ref(), &file010_data[..]);
        insert_manifest(conn, (*file011_id).as_ref(), &file011);
        insert_data(conn, (*file011_block_id).as_ref(), &file011_data[..]);
    }
}
