// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::{error::Error, path::PathBuf};

use crate::schema::{block, device, info, vlob_atom};
use parsec_api_crypto::{SecretKey, VerifyKey};
use parsec_api_types::{
    DateTime, DeviceCertificate, DeviceID, FileManifest, FolderManifest, Manifest,
    WorkspaceManifest,
};

#[derive(Debug, Queryable, PartialEq)]
pub struct Block {
    pub block_id: Vec<u8>,
    pub data: Vec<u8>,
    pub author: i32,
    pub size: i32,
    pub created_on: f64,
}

#[derive(Debug, Queryable, PartialEq)]
pub struct VlobAtom {
    pub vlob_id: Vec<u8>,
    pub version: i32,
    pub blob: Vec<u8>,
    pub size: i32,
    pub author: i32,
    pub timestamp: f64,
}

#[derive(Debug, Queryable, PartialEq)]
pub struct Info {
    pub magic: i32,
    pub version: i32,
    pub realm_id: Vec<u8>,
}

#[derive(Debug)]
pub struct Workspace;

impl Workspace {
    pub fn dump(
        conn: &mut SqliteConnection,
        key: &SecretKey,
        out: &PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let workspace_id = info::table
            .select(info::realm_id)
            .filter(info::magic.eq(87947).and(info::version.eq(1)))
            .first::<Vec<u8>>(conn)?;

        let mut devices = HashMap::new();

        Self::_dump(conn, key, out, &workspace_id, &mut devices)?;

        Ok(())
    }

    fn _dump(
        conn: &mut SqliteConnection,
        key: &SecretKey,
        path: &PathBuf,
        manifest_id: &[u8],
        devices: &mut HashMap<i32, (VerifyKey, DeviceID)>,
    ) -> Result<(), Box<dyn Error>> {
        let (manifest, author, timestamp) = vlob_atom::table
            .select((vlob_atom::blob, vlob_atom::author, vlob_atom::timestamp))
            .filter(vlob_atom::vlob_id.eq(manifest_id))
            .order_by(vlob_atom::timestamp.desc())
            .first::<(Vec<u8>, i32, f64)>(conn)?;

        let (verify_key, device_id) = if let Some(device) = devices.get(&author) {
            device
        } else {
            let device = device::table
                .select(device::device_certificate)
                .filter(device::_id.eq(author))
                .first::<Vec<u8>>(conn)?;

            let certif = DeviceCertificate::unsecure_load(&device)?;

            devices.insert(author, (certif.verify_key, certif.device_id));
            devices.get(&author).expect("Inserted before")
        };

        match Manifest::decrypt_verify_and_load(
            &manifest,
            key,
            verify_key,
            device_id,
            DateTime::from_f64_with_us_precision(timestamp),
        )? {
            Manifest::File(FileManifest { mut blocks, .. }) => {
                blocks.sort_by(|x, y| x.offset.cmp(&y.offset));

                let content = blocks
                    .iter()
                    .map(|x| {
                        block::table
                            .select(block::data)
                            .filter(block::block_id.eq((*x.id).as_ref()))
                            .first::<Vec<u8>>(conn)
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .concat();

                let mut file = File::create(path)?;
                file.write_all(&content)?;
            }
            Manifest::Folder(FolderManifest { children, .. })
            | Manifest::Workspace(WorkspaceManifest { children, .. }) => {
                std::fs::create_dir(&path)?;
                for (name, id) in children.into_iter() {
                    let path = path.join(name.as_ref());
                    Self::_dump(conn, key, &path, (*id).as_ref(), devices)?;
                }
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}
