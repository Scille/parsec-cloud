// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(unused_variables)]

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use indexed_db_futures::prelude::IdbTransaction;
use indexed_db_futures::IdbDatabase;
use libparsec_types::prelude::*;

use crate::certificates::{
    FilterKind, GetCertificateError, GetCertificateQuery, PerTopicLastTimestamps, UpTo,
};
use crate::web::model::{Certificate, CertificateFilter};
use crate::web::DB_VERSION;

const COMMON_CERTIFICATES: [&str; 4] = [
    "user_certificate",
    "device_certificate",
    "user_update_certificate",
    "revoked_user_certificate",
];

const SEQUESTER_CERTIFICATES: [&str; 3] = [
    "sequester_authority_certificate",
    "sequester_service_certificate",
    "sequester_revoked_service_certificate",
];

const REALM_CERTIFICATES: [&str; 4] = [
    "realm_role_certificate",
    "realm_name_certificate",
    "realm_key_rotation_certificate",
    "realm_archiving_certificate",
];

const SHAMIR_RECOVERY_CERTIFICATES: [&str; 2] = [
    "shamir_recovery_share_certificate",
    "shamir_recovery_brief_certificate",
];

#[derive(Debug)]
pub(crate) struct PlatformCertificatesStorageForUpdateGuard<'a> {
    transaction: IdbTransaction<'a>,
}

impl<'a> PlatformCertificatesStorageForUpdateGuard<'a> {
    pub async fn commit(self) -> anyhow::Result<()> {
        super::db::commit(self.transaction).await
    }

    pub async fn get_certificate_encrypted<'b>(
        &mut self,
        query: GetCertificateQuery<'b>,
        up_to: UpTo,
    ) -> Result<(DateTime, Vec<u8>), GetCertificateError> {
        let certifs = Certificate::get_values(&self.transaction, CertificateFilter(query)).await?;

        let maybe_certif_timestamp = certifs
            .get(0)
            .map(|certif| (certif.certificate_timestamp, certif.certificate.clone()));

        let certifs = if let UpTo::Timestamp(up_to) = up_to {
            certifs
                .into_iter()
                .filter(|certif| certif.certificate_timestamp <= up_to.get_f64_with_us_precision())
                .collect()
        } else {
            certifs
        };

        if let Some(certif) = certifs.into_iter().max_by(|x, y| {
            x.certificate_timestamp
                .partial_cmp(&y.certificate_timestamp)
                .expect("Timestamp should not be undefined")
        }) {
            return Ok((
                DateTime::from_f64_with_us_precision(certif.certificate_timestamp),
                certif.certificate.to_vec(),
            ));
        }

        let UpTo::Timestamp(up_to) = up_to else {
            return Err(GetCertificateError::NonExisting);
        };

        // Determine if the result is an actual success or a ExistButTooRecent error
        if let Some((certif_timestamp, certif)) = maybe_certif_timestamp {
            let certificate_timestamp = DateTime::from_f64_with_us_precision(certif_timestamp);

            if certificate_timestamp > up_to {
                return Err(GetCertificateError::ExistButTooRecent {
                    certificate_timestamp,
                });
            }
        }

        Err(GetCertificateError::NonExisting)
    }

    /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
    pub async fn get_multiple_certificates_encrypted<'b>(
        &mut self,
        query: GetCertificateQuery<'b>,
        up_to: UpTo,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<(DateTime, Vec<u8>)>> {
        let certifs = Certificate::get_values(&self.transaction, CertificateFilter(query)).await?;

        let mut certifs = if let UpTo::Timestamp(up_to) = up_to {
            certifs
                .into_iter()
                .filter(|certif| certif.certificate_timestamp <= up_to.get_f64_with_us_precision())
                .collect()
        } else {
            certifs
        };

        certifs.sort_by(|x, y| {
            x.certificate_timestamp
                .partial_cmp(&y.certificate_timestamp)
                .expect("Timestamp should not be undefined")
        });

        let offset = offset.unwrap_or_default() as usize;
        let limit = limit.unwrap_or(certifs.len() as u32) as usize;

        Ok(certifs
            .into_iter()
            .skip(offset)
            .take(limit)
            .map(|x| {
                (
                    DateTime::from_f64_with_us_precision(x.certificate_timestamp),
                    x.certificate.to_vec(),
                )
            })
            .collect())
    }

    pub async fn forget_all_certificates(&mut self) -> anyhow::Result<()> {
        Certificate::clear(&self.transaction).await
    }

    pub async fn add_certificate(
        &mut self,
        certificate_type: &'static str,
        filter1: FilterKind<'_>,
        filter2: FilterKind<'_>,
        timestamp: DateTime,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        let filter1 = match &filter1 {
            FilterKind::Bytes(filter) => Some(filter.to_vec().into()),
            FilterKind::U64(filter) => Some(filter.to_vec().into()),
            FilterKind::Null => None,
        };

        let filter2 = match &filter2 {
            FilterKind::Bytes(filter) => Some(filter.to_vec().into()),
            FilterKind::U64(filter) => Some(filter.to_vec().into()),
            FilterKind::Null => None,
        };

        Certificate::insert(
            &Certificate {
                certificate_timestamp: timestamp.get_f64_with_us_precision(),
                certificate: encrypted.into(),
                certificate_type: certificate_type.into(),
                filter1,
                filter2,
            },
            &self.transaction,
        )
        .await
    }

    pub async fn get_last_timestamps(&mut self) -> anyhow::Result<PerTopicLastTimestamps> {
        let mut common_last_timestamp = None;
        let mut sequester_last_timestamp = None;
        let mut realm_certifs = Vec::new();
        let mut shamir_recovery_last_timestamp = None;

        for ty in COMMON_CERTIFICATES {
            common_last_timestamp = Certificate::get_values(
                &self.transaction,
                CertificateFilter(GetCertificateQuery::NoFilter {
                    certificate_type: ty,
                }),
            )
            .await?
            .into_iter()
            .map(|certif| DateTime::from_f64_with_us_precision(certif.certificate_timestamp))
            .chain(common_last_timestamp)
            .max_by(|x, y| x.cmp(y));
        }

        for ty in SEQUESTER_CERTIFICATES {
            sequester_last_timestamp = Certificate::get_values(
                &self.transaction,
                CertificateFilter(GetCertificateQuery::NoFilter {
                    certificate_type: ty,
                }),
            )
            .await?
            .into_iter()
            .map(|certif| DateTime::from_f64_with_us_precision(certif.certificate_timestamp))
            .chain(sequester_last_timestamp)
            .max_by(|x, y| x.cmp(y));
        }

        for ty in REALM_CERTIFICATES {
            realm_certifs.extend(
                Certificate::get_values(
                    &self.transaction,
                    CertificateFilter(GetCertificateQuery::NoFilter {
                        certificate_type: ty,
                    }),
                )
                .await?
                .into_iter()
                .map(|certif| {
                    (
                        certif.filter1,
                        DateTime::from_f64_with_us_precision(certif.certificate_timestamp),
                    )
                }),
            );
        }

        realm_certifs.sort_by(|x, y| x.1.cmp(&y.1));

        let per_realm_last_timestamps = realm_certifs
            .into_iter()
            .map(|x| {
                x.0.ok_or(anyhow::anyhow!("Missing realm id"))
                    .and_then(|id| {
                        VlobID::try_from(&*id)
                            .map(|id| (id, x.1))
                            .map_err(|e| anyhow::anyhow!(e))
                    })
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        for ty in SHAMIR_RECOVERY_CERTIFICATES {
            shamir_recovery_last_timestamp = Certificate::get_values(
                &self.transaction,
                CertificateFilter(GetCertificateQuery::NoFilter {
                    certificate_type: ty,
                }),
            )
            .await?
            .into_iter()
            .map(|certif| DateTime::from_f64_with_us_precision(certif.certificate_timestamp))
            .chain(shamir_recovery_last_timestamp)
            .max_by(|x, y| x.cmp(y));
        }

        Ok(PerTopicLastTimestamps {
            common: common_last_timestamp,
            sequester: sequester_last_timestamp,
            realm: per_realm_last_timestamps,
            shamir_recovery: shamir_recovery_last_timestamp,
        })
    }

    /// Only used for debugging tests
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct PlatformCertificatesStorage {
    conn: Arc<IdbDatabase>,
}

// Safety: PlatformCertificatesStorage is read only
unsafe impl Send for PlatformCertificatesStorage {}

impl PlatformCertificatesStorage {
    pub async fn no_populate_start(
        data_base_dir: &Path,
        device: &LocalDevice,
    ) -> anyhow::Result<Self> {
        // 1) Open the database

        #[cfg(feature = "test-with-testbed")]
        let name = format!(
            "{}-{}-certificates",
            data_base_dir.to_str().unwrap(),
            device.device_id.hex()
        );

        #[cfg(not(feature = "test-with-testbed"))]
        let name = format!("{}-certificates", device.device_id.hex());

        let db_req =
            IdbDatabase::open_u32(&name, DB_VERSION).map_err(|e| anyhow::anyhow!("{e:?}"))?;

        // 2) Initialize the database (if needed)

        let conn = Arc::new(super::model::initialize_model_if_needed(db_req).await?);

        // 3) All done !

        let storage = Self { conn };
        Ok(storage)
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        self.conn.close();
        Ok(())
    }

    pub async fn for_update(
        &mut self,
    ) -> anyhow::Result<PlatformCertificatesStorageForUpdateGuard> {
        Ok(PlatformCertificatesStorageForUpdateGuard {
            transaction: Certificate::write(&self.conn)?,
        })
    }

    pub async fn get_last_timestamps(&mut self) -> anyhow::Result<PerTopicLastTimestamps> {
        // TODO: transaction shouldn't be needed here (but it's currently easier to implement this way)
        let mut update = self.for_update().await?;
        update.get_last_timestamps().await
    }

    pub async fn get_certificate_encrypted<'b>(
        &mut self,
        query: GetCertificateQuery<'b>,
        up_to: UpTo,
    ) -> Result<(DateTime, Vec<u8>), GetCertificateError> {
        // TODO: transaction shouldn't be needed here (but it's currently easier to implement this way)
        let mut update = self.for_update().await?;
        update.get_certificate_encrypted(query, up_to).await
    }

    pub async fn get_multiple_certificates_encrypted<'b>(
        &mut self,
        query: GetCertificateQuery<'b>,
        up_to: UpTo,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<(DateTime, Vec<u8>)>> {
        // TODO: transaction shouldn't be needed here (but it's currently easier to implement this way)
        let mut update = self.for_update().await?;
        update
            .get_multiple_certificates_encrypted(query, up_to, offset, limit)
            .await
    }

    /// Only used for debugging tests
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        let mut update = self.for_update().await?;
        update.debug_dump().await
    }
}
