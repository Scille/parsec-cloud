// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use crate::schema::{block, device, info, realm_role, user_ as user, vlob_atom};
use chrono::NaiveDateTime;
use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};
use parsec_api_crypto::SecretKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub block_id: Vec<u8>,
    pub data: Vec<u8>,
    pub author: i32,
    pub size: i32,
    pub created_on: NaiveDateTime,
}

#[derive(Debug, Queryable, Serialize, Deserialize, PartialEq)]
pub struct VlobAtom {
    pub vlob_id: Vec<u8>,
    pub version: i32,
    pub blob: Vec<u8>,
    pub size: i32,
    pub author: i32,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Queryable, Serialize, Deserialize, PartialEq)]
pub struct Info {
    pub magic: i32,
    pub version: i32,
    pub realm_id: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Data {
    pub blocks: Vec<Block>,
    pub vlob_atoms: Vec<VlobAtom>,
    pub role_certificates: Vec<Vec<u8>>,
    pub user_certificates: Vec<Vec<u8>>,
    pub device_certificates: Vec<Vec<u8>>,
    pub info: Vec<Info>,
}

impl Data {
    pub fn dump(&self, key: &SecretKey, path: &str) {
        let data = rmp_serde::to_vec_named(self).unwrap();
        let encrypted = key.encrypt(&data);
        std::fs::write(path, &encrypted).unwrap();
    }

    pub fn query_all(conn: &mut SqliteConnection) -> Data {
        let blocks = block::table
            .select((
                block::block_id,
                block::data,
                block::author,
                block::size,
                block::created_on,
            ))
            .load::<Block>(conn)
            .unwrap();

        let vlob_atoms = vlob_atom::table
            .select((
                vlob_atom::vlob_id,
                vlob_atom::version,
                vlob_atom::blob,
                vlob_atom::size,
                vlob_atom::author,
                vlob_atom::timestamp,
            ))
            .load::<VlobAtom>(conn)
            .unwrap();

        let role_certificates = realm_role::table
            .select(realm_role::role_certificate)
            .load::<Vec<u8>>(conn)
            .unwrap();

        let user_certificates = user::table
            .select(user::user_certificate)
            .load::<Vec<u8>>(conn)
            .unwrap();

        let device_certificates = device::table
            .select(device::device_certificate)
            .load::<Vec<u8>>(conn)
            .unwrap();

        let info = info::table.load::<Info>(conn).unwrap();

        Self {
            blocks,
            vlob_atoms,
            role_certificates,
            user_certificates,
            device_certificates,
            info,
        }
    }

    #[cfg(test)]
    pub fn load(key: &SecretKey, path: &str) -> Self {
        let encrypted = std::fs::read(path).unwrap();
        let data = key.decrypt(&encrypted).unwrap();
        rmp_serde::from_slice(&data).unwrap()
    }

    #[cfg(test)]
    pub fn init_db(conn: &mut SqliteConnection) {
        use diesel::ExpressionMethods;

        diesel::insert_or_ignore_into(info::table)
            .values((
                info::magic.eq(87947),
                info::version.eq(1),
                info::realm_id.eq(&b"realm_id"[..]),
            ))
            .execute(conn)
            .unwrap();

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
                block::created_on.eq("2000-01-01T00:00:00"),
            ))
            .execute(conn)
            .unwrap();

        diesel::insert_or_ignore_into(vlob_atom::table)
            .values((
                vlob_atom::_id.eq(0),
                vlob_atom::vlob_id.eq(&b"vlob_id"[..]),
                vlob_atom::version.eq(1),
                vlob_atom::blob.eq(&b"blob"[..]),
                vlob_atom::size.eq(4),
                vlob_atom::author.eq(0),
                vlob_atom::timestamp.eq("2000-01-01T00:00:00"),
            ))
            .execute(conn)
            .unwrap();
    }
}
