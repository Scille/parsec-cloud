// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

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
pub(super) fn get_user_storage_db_relative_path(device: &LocalDevice) -> PathBuf {
    let slug = device.slug();
    PathBuf::from_iter([slug, format!("user_data-v{STORAGE_REVISION}.sqlite")])
}

#[allow(unused)]
/// Path relative to config dir
pub(super) fn get_workspace_storage_db_relative_path(
    device: &LocalDevice,
    realm_id: VlobID,
) -> PathBuf {
    let slug = device.slug();
    PathBuf::from_iter([
        slug,
        realm_id.hex(),
        format!("workspace_data-v{STORAGE_REVISION}.sqlite"),
    ])
}

// TODO: remove reference to workspace cache database: now cache is stored in the
//       same database as the data.
#[allow(unused)]
/// Path relative to config dir
pub(super) fn get_workspace_cache_storage_db_relative_path(
    device: &LocalDevice,
    realm_id: VlobID,
) -> PathBuf {
    let slug = device.slug();
    PathBuf::from_iter([
        slug,
        realm_id.hex(),
        format!("workspace_cache-v{STORAGE_REVISION}.sqlite"),
    ])
}

/// Do not match anything (https://stackoverflow.com/a/2302992/2846140)
const PREVENT_SYNC_PATTERN_EMPTY_PATTERN: &str = r"^\b$";

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
