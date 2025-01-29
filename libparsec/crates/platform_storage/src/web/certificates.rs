// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(unused_variables)]

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use indexed_db_futures::prelude::IdbTransaction;
use indexed_db_futures::IdbDatabase;
use libparsec_types::prelude::*;

use crate::certificates::{
    FilterKind, GetCertificateError, GetCertificateQuery, PerTopicLastTimestamps,
    StorableCertificateTopic, UpTo,
};
use crate::web::model::{Certificate, CertificateFilter};
use crate::web::DB_VERSION;

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
                .filter(|certif| certif.certificate_timestamp <= up_to.as_timestamp_micros())
                .collect()
        } else {
            certifs
        };

        if let Some(certif) = certifs.into_iter().max_by(|x, y| {
            x.certificate_timestamp
                .partial_cmp(&y.certificate_timestamp)
                .expect("Timestamp should not be undefined")
        }) {
            let certificate_timestamp =
                DateTime::from_timestamp_micros(certif.certificate_timestamp)
                    .map_err(|err| GetCertificateError::Internal(err.into()))?;

            return Ok((certificate_timestamp, certif.certificate.to_vec()));
        }

        let UpTo::Timestamp(up_to) = up_to else {
            return Err(GetCertificateError::NonExisting);
        };

        // Determine if the result is an actual success or a ExistButTooRecent error
        if let Some((certif_timestamp, certif)) = maybe_certif_timestamp {
            let certificate_timestamp = DateTime::from_timestamp_micros(certif_timestamp)
                .map_err(|err| GetCertificateError::Internal(err.into()))?;

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
                .filter(|certif| certif.certificate_timestamp <= up_to.as_timestamp_micros())
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

        let mut res = vec![];
        for certif in certifs.into_iter().skip(offset).take(limit) {
            let dt = DateTime::from_timestamp_micros(certif.certificate_timestamp)?;
            res.push((dt, certif.certificate.to_vec()));
        }

        Ok(res)
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
                certificate_timestamp: timestamp.as_timestamp_micros(),
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
        let mut per_realm_last_timestamps = HashMap::new();
        let mut shamir_recovery_last_timestamp = None;

        for ty in <CommonTopicArcCertificate as StorableCertificateTopic>::TYPES {
            let certifs = Certificate::get_values(
                &self.transaction,
                CertificateFilter(GetCertificateQuery::NoFilter {
                    certificate_type: ty,
                }),
            )
            .await?;
            for certif in certifs {
                let timestamp = DateTime::from_timestamp_micros(certif.certificate_timestamp)?;
                match common_last_timestamp {
                    None => common_last_timestamp = Some(timestamp),
                    Some(last_timestamp) => {
                        common_last_timestamp = Some(std::cmp::max(last_timestamp, timestamp))
                    }
                }
            }
        }

        for ty in <SequesterTopicArcCertificate as StorableCertificateTopic>::TYPES {
            let certifs = Certificate::get_values(
                &self.transaction,
                CertificateFilter(GetCertificateQuery::NoFilter {
                    certificate_type: ty,
                }),
            )
            .await?;
            for certif in certifs {
                let timestamp = DateTime::from_timestamp_micros(certif.certificate_timestamp)?;
                match sequester_last_timestamp {
                    None => sequester_last_timestamp = Some(timestamp),
                    Some(last_timestamp) => {
                        sequester_last_timestamp = Some(std::cmp::max(last_timestamp, timestamp))
                    }
                }
            }
        }

        for ty in <RealmTopicArcCertificate as StorableCertificateTopic>::TYPES {
            let certifs = Certificate::get_values(
                &self.transaction,
                CertificateFilter(GetCertificateQuery::NoFilter {
                    certificate_type: ty,
                }),
            )
            .await?;
            for certif in certifs {
                let vlob_id = match &certif.filter1 {
                    Some(filter1) => {
                        VlobID::try_from(filter1.as_ref()).map_err(|e| anyhow::anyhow!(e))?
                    }
                    None => return Err(anyhow::anyhow!("Missing realm ID as filter1")),
                };
                let timestamp = DateTime::from_timestamp_micros(certif.certificate_timestamp)?;
                match per_realm_last_timestamps.entry(vlob_id) {
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(timestamp);
                    }
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        *entry.get_mut() = std::cmp::max(*entry.get(), timestamp);
                    }
                }
            }
        }

        for ty in <ShamirRecoveryTopicArcCertificate as StorableCertificateTopic>::TYPES {
            let certifs = Certificate::get_values(
                &self.transaction,
                CertificateFilter(GetCertificateQuery::NoFilter {
                    certificate_type: ty,
                }),
            )
            .await?;
            for certif in certifs {
                let timestamp = DateTime::from_timestamp_micros(certif.certificate_timestamp)?;
                match shamir_recovery_last_timestamp {
                    None => shamir_recovery_last_timestamp = Some(timestamp),
                    Some(last_timestamp) => {
                        shamir_recovery_last_timestamp =
                            Some(std::cmp::max(last_timestamp, timestamp))
                    }
                }
            }
        }

        Ok(PerTopicLastTimestamps {
            common: common_last_timestamp,
            sequester: sequester_last_timestamp,
            realm: per_realm_last_timestamps,
            shamir_recovery: shamir_recovery_last_timestamp,
        })
    }

    /// Only used for debugging tests
    #[cfg(any(test, feature = "expose-test-methods"))]
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
        #[cfg_attr(not(feature = "test-with-testbed"), allow(unused_variables))]
        data_base_dir: &Path,
        device: &LocalDevice,
    ) -> anyhow::Result<Self> {
        // 1) Open the database

        #[cfg(feature = "test-with-testbed")]
        let name = format!(
            "{}-{}-certificates",
            data_base_dir.display(),
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
    #[cfg(any(test, feature = "expose-test-methods"))]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        let mut update = self.for_update().await?;
        update.debug_dump().await
    }
}
