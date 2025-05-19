// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use sqlx::{
    ConnectOptions, Row, SqliteConnection,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous},
};

use libparsec_types::prelude::*;

const OUTPUT_DB_MAGIC_NUMBER: u32 = 87948;
const OUTPUT_DB_VERSION: u32 = 1;

#[derive(Debug, thiserror::Error)]
pub enum RealmExportDBStartError {
    #[error("Cannot open the database: {0}")]
    CannotOpenDatabase(anyhow::Error),
    #[error("The database is not a valid realm export: {0}")]
    InvalidDatabase(anyhow::Error),
    #[error("Unsupported realm export format version `{found}` (supported: `{supported}`)")]
    UnsupportedDatabaseVersion { supported: u32, found: u32 },
    #[error("The database contains an incomplete realm export")]
    IncompleteRealmExport,
}

#[derive(Debug, thiserror::Error)]
pub enum RealmExportDBFetchCertificatesError {
    #[error("The database is not a valid realm export: {0}")]
    InvalidDatabase(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RealmExportDBFetchKeysBundleError {
    #[error("No keys bundle access found for this sequester/user")]
    BundleNotFound,
    #[error("The database is not a valid realm export: {0}")]
    InvalidDatabase(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RealmExportDBFetchKeysBundleAccessError {
    #[error("No keys bundle access found for this sequester/user")]
    AccessNotFound,
    #[error("The database is not a valid realm export: {0}")]
    InvalidDatabase(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RealmExportDBFetchManifestError {
    #[error("Entry doesn't exist on the realm export")]
    EntryNotFound,
    #[error("The database is not a valid realm export: {0}")]
    InvalidDatabase(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RealmExportDBFetchBlockError {
    #[error("The block doesn't exist on the realm export")]
    BlockNotFound,
    #[error("The database is not a valid realm export: {0}")]
    InvalidDatabase(anyhow::Error),
}

#[derive(Debug)]
pub struct RealmExportDB {
    conn: SqliteConnection,
}

impl RealmExportDB {
    pub async fn start(
        export_db_path: &Path,
    ) -> Result<(Self, OrganizationID, VlobID, VerifyKey, DateTime), RealmExportDBStartError> {
        // 1) Open the database

        let mut conn = SqliteConnectOptions::new()
            .filename(export_db_path)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .connect()
            .await
            .map_err(|e| RealmExportDBStartError::CannotOpenDatabase(e.into()))?;

        // 2) Check the database format

        let row = sqlx::query(
            "SELECT version
            FROM info \
            WHERE magic = ?1 \
            ",
        )
        .bind(OUTPUT_DB_MAGIC_NUMBER)
        .fetch_optional(&mut conn)
        .await
        .map_err(|e| RealmExportDBStartError::InvalidDatabase(e.into()))?;
        match row {
            None => {
                return Err(RealmExportDBStartError::InvalidDatabase(anyhow::anyhow!(
                    "Magic number `{}` not found",
                    OUTPUT_DB_MAGIC_NUMBER
                )));
            }
            Some(row) => {
                let version = row
                    .try_get::<u32, _>(0)
                    .map_err(|err| RealmExportDBStartError::InvalidDatabase(err.into()))?;
                if version != OUTPUT_DB_VERSION {
                    return Err(RealmExportDBStartError::UnsupportedDatabaseVersion {
                        supported: OUTPUT_DB_VERSION,
                        found: version,
                    });
                }
            }
        }

        // 3) Retrieve base info

        let row = sqlx::query(
            "SELECT \
                organization_id, \
                realm_id, \
                root_verify_key, \
                snapshot_timestamp, \
                certificates_export_done, \
                vlobs_export_done, \
                blocks_metadata_export_done, \
                blocks_data_export_done
            FROM info \
            WHERE magic = ?1 \
            ",
        )
        .bind(OUTPUT_DB_MAGIC_NUMBER)
        .fetch_one(&mut conn)
        .await
        .map_err(|e| RealmExportDBStartError::InvalidDatabase(e.into()))?;

        macro_rules! parse_and_cook {
            ($row:expr, $idx:expr, $sqlite_ty:ty, $cooking:expr) => {
                $row.try_get::<$sqlite_ty, _>($idx)
                    .map_err(|err| RealmExportDBStartError::InvalidDatabase(err.into()))
                    .and_then(|raw| {
                        $cooking(raw)
                            .map_err(|err| RealmExportDBStartError::InvalidDatabase(err.into()))
                    })
            };
        }

        let organization_id =
            parse_and_cook!(row, 0, String, |raw: String| raw.parse::<OrganizationID>())?;
        let realm_id = parse_and_cook!(row, 1, &[u8], |raw: &[u8]| VlobID::try_from(raw))?;
        let root_verify_key =
            parse_and_cook!(row, 2, &[u8], |raw: &[u8]| VerifyKey::try_from(raw))?;
        let snapshot_timestamp =
            parse_and_cook!(row, 3, i64, |raw: i64| DateTime::from_timestamp_micros(raw))?;
        let certificates_export_done = row
            .try_get::<u8, _>(4)
            .map(|raw| raw == 1)
            .map_err(|err| RealmExportDBStartError::InvalidDatabase(err.into()))?;
        let vlobs_export_done = row
            .try_get::<u8, _>(5)
            .map(|raw| raw == 1)
            .map_err(|err| RealmExportDBStartError::InvalidDatabase(err.into()))?;
        let blocks_metadata_export_done = row
            .try_get::<u8, _>(6)
            .map(|raw| raw == 1)
            .map_err(|err| RealmExportDBStartError::InvalidDatabase(err.into()))?;
        let blocks_data_export_done = row
            .try_get::<u8, _>(7)
            .map(|raw| raw == 1)
            .map_err(|err| RealmExportDBStartError::InvalidDatabase(err.into()))?;

        if !certificates_export_done
            || !vlobs_export_done
            || !blocks_metadata_export_done
            || !blocks_data_export_done
        {
            return Err(RealmExportDBStartError::IncompleteRealmExport);
        }

        Ok((
            Self { conn },
            organization_id,
            realm_id,
            root_verify_key,
            snapshot_timestamp,
        ))
    }

    /// Certificates are provided in order (oldest first)
    pub async fn fetch_common_certificates(
        &mut self,
    ) -> Result<Vec<Vec<u8>>, RealmExportDBFetchCertificatesError> {
        sqlx::query("SELECT certificate FROM common_certificate")
            .fetch_all(&mut self.conn)
            .await
            .map_err(|e| RealmExportDBFetchCertificatesError::InvalidDatabase(e.into()))
            .and_then(|rows| {
                rows.into_iter()
                    .map(|row| {
                        row.try_get(0).map_err(|e| {
                            RealmExportDBFetchCertificatesError::InvalidDatabase(e.into())
                        })
                    })
                    .collect()
            })
    }

    /// Certificates are provided in order (oldest first)
    pub async fn fetch_sequester_certificates(
        &mut self,
    ) -> Result<Vec<Vec<u8>>, RealmExportDBFetchCertificatesError> {
        sqlx::query("SELECT certificate FROM sequester_certificate")
            .fetch_all(&mut self.conn)
            .await
            .map_err(|e| RealmExportDBFetchCertificatesError::InvalidDatabase(e.into()))
            .and_then(|rows| {
                rows.into_iter()
                    .map(|row| {
                        row.try_get(0).map_err(|e| {
                            RealmExportDBFetchCertificatesError::InvalidDatabase(e.into())
                        })
                    })
                    .collect()
            })
    }

    /// Certificates are provided in order (oldest first)
    pub async fn fetch_realm_certificates(
        &mut self,
    ) -> Result<Vec<Vec<u8>>, RealmExportDBFetchCertificatesError> {
        sqlx::query("SELECT certificate FROM realm_certificate")
            .fetch_all(&mut self.conn)
            .await
            .map_err(|e| RealmExportDBFetchCertificatesError::InvalidDatabase(e.into()))
            .and_then(|rows| {
                rows.into_iter()
                    .map(|row| {
                        row.try_get(0).map_err(|e| {
                            RealmExportDBFetchCertificatesError::InvalidDatabase(e.into())
                        })
                    })
                    .collect()
            })
    }

    pub async fn fetch_encrypted_keys_bundle(
        &mut self,
        key_index: IndexInt,
    ) -> Result<Vec<u8>, RealmExportDBFetchKeysBundleError> {
        let maybe_row = sqlx::query(
            "\
                SELECT \
                    keys_bundle \
                FROM realm_keys_bundle \
                WHERE
                    key_index = ?1 \
            ",
        )
        .bind(key_index as u32)
        .fetch_optional(&mut self.conn)
        .await
        .map_err(|e| RealmExportDBFetchKeysBundleError::InvalidDatabase(e.into()))?;

        let row = match maybe_row {
            Some(row) => row,
            None => return Err(RealmExportDBFetchKeysBundleError::BundleNotFound),
        };

        row.try_get(0)
            .map_err(|e| RealmExportDBFetchKeysBundleError::InvalidDatabase(e.into()))
    }

    /// Note the `skip` parameter, this is because a user can have multiple keys bundle
    /// access for a given realm ID & key index pair (since a new access is provided
    /// each time a sharing is done).
    pub async fn fetch_keys_bundle_accesses_for_user(
        &mut self,
        key_index: IndexInt,
        user_id: UserID,
        skip: u32,
    ) -> Result<Vec<u8>, RealmExportDBFetchKeysBundleAccessError> {
        let maybe_row = sqlx::query(
            "\
                SELECT \
                    access \
                FROM realm_keys_bundle_access \
                WHERE
                    key_index = ?1 \
                    AND user_id = ?2 \
                ORDER BY _id \
                LIMIT 1 \
                OFFSET ?3 \
            ",
        )
        .bind(key_index as u32)
        .bind(user_id.as_bytes())
        .bind(skip)
        .fetch_optional(&mut self.conn)
        .await
        .map_err(|e| RealmExportDBFetchKeysBundleAccessError::InvalidDatabase(e.into()))?;

        let row = match maybe_row {
            Some(row) => row,
            None => return Err(RealmExportDBFetchKeysBundleAccessError::AccessNotFound),
        };

        row.try_get(0)
            .map_err(|e| RealmExportDBFetchKeysBundleAccessError::InvalidDatabase(e.into()))
    }

    pub async fn fetch_keys_bundle_accesses_for_sequester(
        &mut self,
        key_index: IndexInt,
        sequester_service_id: SequesterServiceID,
    ) -> Result<Vec<u8>, RealmExportDBFetchKeysBundleAccessError> {
        let maybe_row = sqlx::query(
            "\
                SELECT \
                    access \
                FROM realm_sequester_keys_bundle_access \
                WHERE
                    key_index = ?1 \
                    AND sequester_service_id = ?2 \
            ",
        )
        .bind(key_index as u32)
        .bind(sequester_service_id.as_bytes())
        .fetch_optional(&mut self.conn)
        .await
        .map_err(|e| RealmExportDBFetchKeysBundleAccessError::InvalidDatabase(e.into()))?;

        let row = match maybe_row {
            Some(row) => row,
            None => return Err(RealmExportDBFetchKeysBundleAccessError::AccessNotFound),
        };

        row.try_get(0)
            .map_err(|e| RealmExportDBFetchKeysBundleAccessError::InvalidDatabase(e.into()))
    }

    pub async fn fetch_encrypted_manifest(
        &mut self,
        at: DateTime,
        entry_id: VlobID,
    ) -> Result<(DeviceID, DateTime, VersionInt, IndexInt, Vec<u8>), RealmExportDBFetchManifestError>
    {
        let maybe_row = sqlx::query(
            "SELECT \
                author, \
                timestamp, \
                version, \
                key_index, \
                blob \
            FROM vlob_atom \
            WHERE vlob_id = ?1 AND timestamp <= ?2 \
            ORDER BY timestamp DESC \
            LIMIT 1 \
            ",
        )
        .bind(entry_id.as_bytes())
        .bind(at.as_timestamp_micros())
        .fetch_optional(&mut self.conn)
        .await
        .map_err(|e| RealmExportDBFetchManifestError::InvalidDatabase(e.into()))?;

        let row = match maybe_row {
            Some(row) => row,
            None => return Err(RealmExportDBFetchManifestError::EntryNotFound),
        };

        let author = row
            .try_get(0)
            .map_err(|e| RealmExportDBFetchManifestError::InvalidDatabase(e.into()))
            .and_then(|raw: &[u8]| {
                DeviceID::try_from(raw).map_err(|err: InvalidDeviceID| {
                    RealmExportDBFetchManifestError::InvalidDatabase(err.into())
                })
            })?;

        let timestamp = row
            .try_get(1)
            .map_err(|e| RealmExportDBFetchManifestError::InvalidDatabase(e.into()))
            .and_then(|raw: i64| {
                DateTime::from_timestamp_micros(raw)
                    .map_err(|err| RealmExportDBFetchManifestError::InvalidDatabase(err.into()))
            })?;

        let version = row
            .try_get::<VersionInt, _>(2)
            .map_err(|e| RealmExportDBFetchManifestError::InvalidDatabase(e.into()))?;

        let key_index = row
            .try_get::<u32, _>(3)
            .map(|x| x as IndexInt)
            .map_err(|e| RealmExportDBFetchManifestError::InvalidDatabase(e.into()))?;

        let encrypted = row
            .try_get(4)
            .map_err(|e| RealmExportDBFetchManifestError::InvalidDatabase(e.into()))?;

        Ok((author, timestamp, version, key_index, encrypted))
    }

    pub async fn get_encrypted_workspace_manifest_v1(
        &mut self,
    ) -> Result<(DeviceID, DateTime, IndexInt, Vec<u8>), RealmExportDBFetchManifestError> {
        let maybe_row = sqlx::query(
            "SELECT \
                author, \
                timestamp, \
                key_index, \
                blob \
            FROM vlob_atom \
            WHERE
                vlob_id = (SELECT realm_id FROM info) \
                AND version = 1 \
            LIMIT 1 \
            ",
        )
        .fetch_optional(&mut self.conn)
        .await
        .map_err(|e| RealmExportDBFetchManifestError::InvalidDatabase(e.into()))?;

        let row = match maybe_row {
            Some(row) => row,
            None => return Err(RealmExportDBFetchManifestError::EntryNotFound),
        };

        let author = row
            .try_get(0)
            .map_err(|e| RealmExportDBFetchManifestError::InvalidDatabase(e.into()))
            .and_then(|raw: &[u8]| {
                DeviceID::try_from(raw).map_err(|err: InvalidDeviceID| {
                    RealmExportDBFetchManifestError::InvalidDatabase(err.into())
                })
            })?;

        let timestamp = row
            .try_get(1)
            .map_err(|e| RealmExportDBFetchManifestError::InvalidDatabase(e.into()))
            .and_then(|raw: i64| {
                DateTime::from_timestamp_micros(raw)
                    .map_err(|err| RealmExportDBFetchManifestError::InvalidDatabase(err.into()))
            })?;

        let key_index = row
            .try_get::<u32, _>(2)
            .map(|x| x as IndexInt)
            .map_err(|e| RealmExportDBFetchManifestError::InvalidDatabase(e.into()))?;

        let encrypted = row
            .try_get(3)
            .map_err(|e| RealmExportDBFetchManifestError::InvalidDatabase(e.into()))?;

        Ok((author, timestamp, key_index, encrypted))
    }

    pub async fn fetch_encrypted_block(
        &mut self,
        block_id: BlockID,
    ) -> Result<(IndexInt, Vec<u8>), RealmExportDBFetchBlockError> {
        let maybe_row = sqlx::query(
            "SELECT \
                block.key_index, \
                block_data.data \
            FROM block LEFT JOIN block_data ON block.sequential_id = block_data.block \
            WHERE block.block_id = ?1 \
            ",
        )
        .bind(block_id.as_bytes())
        .fetch_optional(&mut self.conn)
        .await
        .map_err(|e| RealmExportDBFetchBlockError::InvalidDatabase(e.into()))?;

        let row = match maybe_row {
            Some(row) => row,
            None => return Err(RealmExportDBFetchBlockError::BlockNotFound),
        };

        let key_index = row
            .try_get::<u32, _>(0)
            .map(|x| x as IndexInt)
            .map_err(|e| RealmExportDBFetchBlockError::InvalidDatabase(e.into()))?;

        let encrypted = row
            .try_get(1)
            .map_err(|e| RealmExportDBFetchBlockError::InvalidDatabase(e.into()))?;

        Ok((key_index, encrypted))
    }
}
