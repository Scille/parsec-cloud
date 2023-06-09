// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use diesel::{table, AsChangeset, Insertable};

table! {
    prevent_sync_pattern (_id) {
        _id -> BigInt,
        pattern -> Text,
        fully_applied -> Bool,
    }
}

table! {
    realm_checkpoint (_id) {
        _id -> BigInt,
        checkpoint -> BigInt,
    }
}

table! {
    vlobs (vlob_id) {
        vlob_id -> Binary, // UUID
        base_version -> BigInt,
        remote_version -> BigInt,
        need_sync -> Bool,
        blob -> Binary,
    }
}

table! {
    chunks (chunk_id) {
        chunk_id -> Binary, // UUID
        size -> BigInt,
        offline -> Bool,
        accessed_on -> Nullable<Double>, // Timestamp
        data -> Binary,
    }
}

table! {
    remanence(_id) {
        _id -> BigInt,
        block_remanent -> Bool,
    }
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = chunks)]
pub struct NewChunk<'a> {
    pub chunk_id: &'a [u8],
    pub size: i64,
    pub offline: bool,
    pub accessed_on: Option<super::sql_types::DateTime>,
    pub data: &'a [u8],
}

#[derive(Insertable)]
#[diesel(table_name = vlobs)]
pub struct NewVlob<'a> {
    pub vlob_id: &'a [u8],
    pub base_version: i64,
    pub remote_version: i64,
    pub need_sync: bool,
    pub blob: &'a [u8],
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = realm_checkpoint)]
pub struct NewRealmCheckpoint {
    pub _id: i64,
    pub checkpoint: i64,
}

#[derive(Insertable)]
#[diesel(table_name = prevent_sync_pattern)]
pub struct NewPreventSyncPattern<'a> {
    pub _id: i64,
    pub pattern: &'a str,
    pub fully_applied: bool,
}
