// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use diesel::{ExpressionMethods, RunQueryDsl};
use std::{path::Path, sync::Arc};

use libparsec_types::prelude::*;

use super::db::{DatabaseResult, LocalDatabase, VacuumMode};
use super::model::get_workspace_data_storage_db_relative_path;

pub async fn workspace_storage_non_speculative_init(
    data_base_dir: &Path,
    device: &LocalDevice,
    workspace_id: EntryID,
) -> anyhow::Result<()> {
    // 1) Open the database

    let db_relative_path = get_workspace_data_storage_db_relative_path(device, &workspace_id);
    let db =
        LocalDatabase::from_path(data_base_dir, &db_relative_path, VacuumMode::default()).await?;

    // 2) Initialize the database

    super::model::initialize_model_if_needed(&db).await?;

    // 3) Populate the database with the workspace manifest

    let timestamp = device.now();
    let manifest = Arc::new(LocalWorkspaceManifest::new(
        device.device_id.clone(),
        timestamp,
        Some(workspace_id),
        false,
    ));
    db_set_workspace_manifest(&db, device, manifest).await?;

    // 4) All done ! Don't forget the close the database before exit ;-)

    db.close().await;
    Ok(())
}

async fn db_set_workspace_manifest(
    db: &LocalDatabase,
    device: &LocalDevice,
    manifest: Arc<LocalWorkspaceManifest>,
) -> DatabaseResult<()> {
    let blob = manifest.dump_and_encrypt(&device.local_symkey);

    db.exec(move |conn| {
        conn.immediate_transaction(|conn| {
            let new_vlob = super::model::NewVlob {
                vlob_id: manifest.base.id.as_bytes(),
                base_version: manifest.base.version as i64,
                remote_version: manifest.base.version as i64,
                need_sync: manifest.need_sync,
                blob: &blob,
            };

            {
                use diesel::dsl::sql;
                use diesel::upsert::excluded;
                use super::model::vlobs::dsl::*;

                diesel::insert_into(vlobs)
                    .values(new_vlob)
                    .on_conflict(vlob_id)
                    .do_update()
                    .set(
                        (
                            base_version.eq(excluded(base_version)),
                            remote_version.eq(
                                sql("(CASE WHEN `remote_version` > `excluded`.`remote_version` THEN `remote_version` ELSE `excluded`.`remote_version` END)")
                            ),
                            need_sync.eq(excluded(need_sync)),
                            blob.eq(excluded(blob)),
                        )
                    )
                    .execute(conn)?;
            }

            Ok(())
        })
    }).await
}
