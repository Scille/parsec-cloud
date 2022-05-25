// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use parsec_api_crypto::SecretKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub block_id: Vec<u8>,
    pub data: Vec<u8>,
    pub author: i32,
    pub size: i32,
    pub created_on: f32,
}

#[derive(Debug, Queryable, Serialize, Deserialize, PartialEq)]
pub struct VlobAtom {
    pub vlob_id: Vec<u8>,
    pub version: i32,
    pub blob: Vec<u8>,
    pub size: i32,
    pub author: i32,
    pub timestamp: f32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Data {
    pub blocks: Vec<Block>,
    pub vlob_atoms: Vec<VlobAtom>,
    pub role_certificates: Vec<Vec<u8>>,
    pub user_certificates: Vec<Vec<u8>>,
    pub device_certificates: Vec<Vec<u8>>,
}

impl Data {
    pub fn save(&self, key: &SecretKey, path: &str) {
        let data = rmp_serde::to_vec_named(self).unwrap();
        let encrypted = key.encrypt(&data);
        std::fs::write(path, &encrypted).unwrap();
    }

    #[cfg(test)]
    pub fn load(key: &SecretKey, path: &str) -> Self {
        let encrypted = std::fs::read(path).unwrap();
        let data = key.decrypt(&encrypted).unwrap();
        rmp_serde::from_slice(&data).unwrap()
    }
}
