// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use indexed_db_futures::prelude::*;
use libparsec_types::prelude::*;

use crate::web::{
    model::{get_user_storage_db_name, RealmCheckpoint, Vlob},
    DB_VERSION,
};

#[allow(unused)]
pub struct NeedSyncEntries {
    pub remote: Vec<VlobID>,
    pub local: Vec<VlobID>,
}

#[derive(Debug)]
pub struct PlatformUserStorage {
    conn: Arc<IdbDatabase>,
    realm_id: VlobID,
}

// Safety: PlatformUserStorage is read only
unsafe impl Send for PlatformUserStorage {}

impl PlatformUserStorage {
    pub(crate) async fn no_populate_start(
        data_base_dir: &Path,
        device: &LocalDevice,
    ) -> anyhow::Result<Self> {
        // 1) Open the database

        let name = get_user_storage_db_name(data_base_dir, device.device_id);
        let db_req =
            IdbDatabase::open_u32(&name, DB_VERSION).map_err(|e| anyhow::anyhow!("{e:?}"))?;

        // 2) Initialize the database (if needed)

        let conn = Arc::new(super::model::initialize_model_if_needed(db_req).await?);

        // 3) All done !

        Ok(Self {
            conn,
            realm_id: device.user_realm_id,
        })
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
        self.conn.close();
        Ok(())
    }

    pub async fn get_realm_checkpoint(&self) -> anyhow::Result<IndexInt> {
        let transaction = RealmCheckpoint::read(&self.conn)?;
        Ok(RealmCheckpoint::get(&transaction)
            .await?
            .map(|x| x.checkpoint)
            .unwrap_or_default())
    }

    pub async fn update_realm_checkpoint(
        &self,
        new_checkpoint: IndexInt,
        remote_user_manifest_version: Option<VersionInt>,
    ) -> anyhow::Result<()> {
        if let Some(remote_user_manifest_version) = remote_user_manifest_version {
            let transaction = Vlob::write(&self.conn)?;

            if let Some(mut vlob) =
                Vlob::get(&transaction, &self.realm_id.as_bytes().to_vec().into()).await?
            {
                Vlob::remove(&transaction, &vlob.vlob_id).await?;
                vlob.remote_version = remote_user_manifest_version;
                vlob.insert(&transaction).await?;
            }

            super::db::commit(transaction).await?;
        }

        let transaction = RealmCheckpoint::write(&self.conn)?;

        RealmCheckpoint {
            checkpoint: new_checkpoint,
        }
        .insert(&transaction)
        .await?;

        super::db::commit(transaction).await?;

        Ok(())
    }

    pub async fn get_user_manifest(&self) -> anyhow::Result<Option<Vec<u8>>> {
        let transaction = Vlob::read(&self.conn)?;

        Ok(
            Vlob::get(&transaction, &self.realm_id.as_bytes().to_vec().into())
                .await?
                .map(|x| x.blob.into()),
        )
    }

    pub async fn update_user_manifest(
        &mut self,
        encrypted: &[u8],
        need_sync: bool,
        base_version: VersionInt,
    ) -> anyhow::Result<()> {
        let transaction = Vlob::write(&self.conn)?;

        match Vlob::get(&transaction, &self.realm_id.as_bytes().to_vec().into()).await? {
            Some(old_vlob) => {
                Vlob::remove(&transaction, &old_vlob.vlob_id).await?;
                Vlob {
                    vlob_id: old_vlob.vlob_id.clone(),
                    blob: encrypted.to_vec().into(),
                    need_sync,
                    base_version,
                    remote_version: std::cmp::max(base_version, old_vlob.remote_version),
                }
                .insert(&transaction)
                .await
            }
            None => {
                Vlob {
                    vlob_id: self.realm_id.as_bytes().to_vec().into(),
                    blob: encrypted.to_vec().into(),
                    need_sync,
                    base_version,
                    remote_version: base_version,
                }
                .insert(&transaction)
                .await
            }
        }
    }

    #[cfg(feature = "expose-test-methods")]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        todo!();
    }
}
