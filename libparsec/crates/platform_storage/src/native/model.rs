// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use diesel::{table, AsChangeset, Insertable};
use std::path::PathBuf;

use super::db::{DatabaseResult, LocalDatabase};
use libparsec_types::prelude::*;

// In theory we should have multiple kinds of database after what they are used for:
// - workspace data storage: store local manifests & chunks of file that are not
//   synchronized yet
// - workspace cache storage: store blocks of file that are synchronized (i.e. this
//   database can be destroyed without occurring memory loss)
// - user storage: store the user manifest
// - certifs storage: store all the certificates for the organization
//
// In practice, it is just simpler to have a single datamodel that support all those
// needs: the tables that are not needed are just left alone.
pub(super) const STORAGE_REVISION: u32 = 1;

/// Path relative to config dir
pub(super) fn get_certificates_storage_db_relative_path(device: &LocalDevice) -> PathBuf {
    let slug = device.slug();
    PathBuf::from_iter([slug, format!("certificates-v{STORAGE_REVISION}.sqlite")])
}

/// Path relative to config dir
pub(super) fn get_user_data_storage_db_relative_path(device: &LocalDevice) -> PathBuf {
    let slug = device.slug();
    PathBuf::from_iter([slug, format!("user_data-v{STORAGE_REVISION}.sqlite")])
}

#[allow(unused)]
/// Path relative to config dir
pub(super) fn get_workspace_data_storage_db_relative_path(
    device: &LocalDevice,
    realm_id: &VlobID,
) -> PathBuf {
    let slug = device.slug();
    PathBuf::from_iter([
        slug,
        realm_id.hex(),
        format!("workspace_data-v{STORAGE_REVISION}.sqlite"),
    ])
}

#[allow(unused)]
/// Path relative to config dir
pub(super) fn get_workspace_cache_storage_db_relative_path(
    device: &LocalDevice,
    realm_id: &VlobID,
) -> PathBuf {
    let slug = device.slug();
    PathBuf::from_iter([
        slug,
        realm_id.hex(),
        format!("workspace_cache-v{STORAGE_REVISION}.sqlite"),
    ])
}

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
    remanence (_id) {
        _id -> BigInt,
        block_remanent -> Bool,
    }
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = chunks)]
pub(super) struct NewChunk<'a> {
    pub chunk_id: &'a [u8],
    pub size: i64,
    pub offline: bool,
    pub accessed_on: Option<super::db::DateTime>,
    pub data: &'a [u8],
}

#[derive(Insertable)]
#[diesel(table_name = vlobs)]
pub(super) struct NewVlob<'a> {
    pub vlob_id: &'a [u8],
    pub base_version: i64,
    pub remote_version: i64,
    pub need_sync: bool,
    pub blob: &'a [u8],
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = realm_checkpoint)]
pub(super) struct NewRealmCheckpoint {
    pub _id: i64,
    pub checkpoint: i64,
}

#[derive(Insertable)]
#[diesel(table_name = prevent_sync_pattern)]
pub(super) struct NewPreventSyncPattern<'a> {
    pub _id: i64,
    pub pattern: &'a str,
    pub fully_applied: bool,
}

/// Do not match anything (https://stackoverflow.com/a/2302992/2846140)
const PREVENT_SYNC_PATTERN_EMPTY_PATTERN: &str = r"^\b$";

pub(super) async fn initialize_model_if_needed(db: &LocalDatabase) -> DatabaseResult<()> {
    db.exec(move |conn| {
        conn.exclusive_transaction(|conn| {
            use diesel::{insert_or_ignore_into, sql_query, RunQueryDsl};

            // 1) create the tables

            sql_query(std::include_str!("sql/create-vlobs-table.sql")).execute(conn)?;
            sql_query(std::include_str!("sql/create-realm-checkpoint-table.sql")).execute(conn)?;
            sql_query(std::include_str!(
                "sql/create-prevent-sync-pattern-table.sql"
            ))
            .execute(conn)?;
            sql_query(std::include_str!("sql/create-chunks-table.sql")).execute(conn)?;
            sql_query(std::include_str!("sql/create-remanence-table.sql")).execute(conn)?;
            sql_query(std::include_str!("sql/create-certificates-table.sql")).execute(conn)?;

            // 2) Populate the tables

            // Set the default "prevent sync" pattern if it doesn't exist
            let pattern = NewPreventSyncPattern {
                _id: 0,
                pattern: PREVENT_SYNC_PATTERN_EMPTY_PATTERN,
                fully_applied: false,
            };
            insert_or_ignore_into(prevent_sync_pattern::table)
                .values(pattern)
                .execute(conn)?;

            Ok(())
        })
    })
    .await
}

pub(super) async fn sqlx_initialize_model_if_needed(
    conn: &mut sqlx::SqliteConnection,
) -> anyhow::Result<()> {
    use sqlx::Connection;
    let mut transaction = conn.begin().await?;

    // 1) create the tables

    sqlx::query(std::include_str!("sql/create-vlobs-table.sql"))
        .execute(&mut *transaction)
        .await?;
    sqlx::query(std::include_str!("sql/create-realm-checkpoint-table.sql"))
        .execute(&mut *transaction)
        .await?;
    sqlx::query(std::include_str!(
        "sql/create-prevent-sync-pattern-table.sql"
    ))
    .execute(&mut *transaction)
    .await?;
    sqlx::query(std::include_str!("sql/create-chunks-table.sql"))
        .execute(&mut *transaction)
        .await?;
    sqlx::query(std::include_str!("sql/create-remanence-table.sql"))
        .execute(&mut *transaction)
        .await?;
    sqlx::query(std::include_str!("sql/create-certificates-table.sql"))
        .execute(&mut *transaction)
        .await?;

    // 2) Populate the tables

    // Set the default "prevent sync" pattern if it doesn't exist
    sqlx::query(
        "INSERT INTO prevent_sync_pattern(_id, pattern, fully_applied) \
        VALUES( \
            0, \
            ?1, \
            FALSE \
        ) \
        ON CONFLICT DO NOTHING \
        ",
    )
    .bind(PREVENT_SYNC_PATTERN_EMPTY_PATTERN)
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}
