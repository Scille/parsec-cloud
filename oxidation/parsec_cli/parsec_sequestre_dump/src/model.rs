// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, Queryable, RunQueryDsl, SqliteConnection,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::error::ExportError;
use crate::schema::{block, device, info, vlob_atom};
use libparsec_crypto::{SecretKey, VerifyKey};
use libparsec_types::{
    DateTime, DeviceCertificate, DeviceID, EntryID, FileManifest, FolderManifest, Manifest,
    WorkspaceManifest,
};

#[derive(Debug, Queryable, PartialEq)]
pub struct Block {
    pub block_id: Vec<u8>,
    pub data: Vec<u8>,
    pub author: i64,
    pub size: i64,
    pub created_on: f64,
}

#[derive(Debug, Queryable, PartialEq)]
pub struct VlobAtom {
    pub vlob_id: Vec<u8>,
    pub version: i64,
    pub blob: Vec<u8>,
    pub size: i64,
    pub author: i64,
    pub timestamp: f64,
}

#[derive(Debug, Queryable, PartialEq, Eq)]
pub struct Info {
    pub magic: i64,
    pub version: i64,
    pub realm_id: Vec<u8>,
}

#[derive(Debug)]
pub struct Workspace;

impl Workspace {
    pub fn dump(
        conn: &mut SqliteConnection,
        key: &SecretKey,
        out: &PathBuf,
    ) -> Result<(), ExportError> {
        let workspace_id = info::table
            .select(info::realm_id)
            .filter(info::magic.eq(87947).and(info::version.eq(1)))
            .first::<Vec<u8>>(conn)
            .map_err(|_| ExportError::MissingWorkspaceID)?;
        let workspace_id = EntryID::from(
            <[u8; 16]>::try_from(&workspace_id[..])
                .map_err(|_| ExportError::InvalidEntryID(workspace_id))?,
        );

        let mut devices = HashMap::new();

        for (author, device) in device::table
            .select((device::_id, device::device_certificate))
            .load::<(i64, Vec<u8>)>(conn)
            .map_err(|_| ExportError::MissingDevice)?
        {
            let certif = DeviceCertificate::unsecure_load(&device)
                .map_err(|_| ExportError::InvalidCertificate { author })?;
            devices.insert(author, (certif.verify_key, certif.device_id));
        }

        Self::_dump(conn, key, out, workspace_id, &mut devices)?;

        Ok(())
    }

    fn _dump(
        conn: &mut SqliteConnection,
        key: &SecretKey,
        path: &PathBuf,
        manifest_id: EntryID,
        devices: &mut HashMap<i64, (VerifyKey, DeviceID)>,
    ) -> Result<(), ExportError> {
        let (manifest, author, timestamp) = vlob_atom::table
            .select((vlob_atom::blob, vlob_atom::author, vlob_atom::timestamp))
            .filter(vlob_atom::vlob_id.eq((*manifest_id).as_ref()))
            .order_by(vlob_atom::timestamp.desc())
            .first::<(Vec<u8>, i64, f64)>(conn)
            .map_err(|_| ExportError::MissingManifest {
                entry_id: manifest_id,
            })?;

        let (verify_key, device_id) = match devices.get(&author) {
            Some(device) => device,
            None => return Err(ExportError::MissingSpecificDevice { author }),
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
                            .map_err(|_| ExportError::MissingSpecificBlock { block_id: x.id })
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .concat();

                let mut file = File::create(path)
                    .map_err(|_| ExportError::CreateFileFailed { path: path.clone() })?;
                file.write_all(&content)
                    .map_err(|_| ExportError::WriteFailed)?;
            }
            Manifest::Folder(FolderManifest { children, .. })
            | Manifest::Workspace(WorkspaceManifest { children, .. }) => {
                std::fs::create_dir(&path)
                    .map_err(|_| ExportError::CreateDirFailed { path: path.clone() })?;
                for (name, id) in children.into_iter() {
                    let path = path.join(name.as_ref());
                    Self::_dump(conn, key, &path, id, devices)?;
                }
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}
