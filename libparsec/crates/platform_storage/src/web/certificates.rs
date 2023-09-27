#![allow(unused_variables)]
// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_types::prelude::*;

use crate::certificates::{
    AddCertificateData, GetAnyCertificateData, GetCertificateError, GetCertificateQuery,
    GetTimestampBoundsError, UpTo,
};

#[derive(Debug)]
pub(crate) struct PlatformCertificatesStorage {}

impl PlatformCertificatesStorage {
    #[allow(unused)]
    pub async fn no_populate_start(
        _data_base_dir: &Path,
        _device: &LocalDevice,
    ) -> anyhow::Result<Self> {
        todo!()
    }

    pub async fn stop(&self) {
        todo!()
    }

    #[cfg(test)]
    pub async fn test_get_raw_certificate(
        &self,
        index: IndexInt,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        todo!()
    }

    /// Return the timestamp of creation of the considered certificate index
    /// and (if any) of the certificate index following it.
    /// If this certificate index doesn't exist yet, `(None, None)` is returned.
    pub async fn get_timestamp_bounds(
        &self,
        index: IndexInt,
    ) -> Result<(DateTime, Option<DateTime>), GetTimestampBoundsError> {
        todo!()
    }

    pub async fn get_last_index(&self) -> anyhow::Result<Option<(IndexInt, DateTime)>> {
        todo!()
    }

    /// Remove all certificates from the database
    /// There is no data loss from this as certificates can be re-obtained from
    /// the server, however it is only needed when switching from/to redacted
    /// certificates
    pub async fn forget_all_certificates(&self) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn add_certificate(
        &self,
        index: IndexInt,
        data: AddCertificateData,
    ) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn get_any_certificate_encrypted(
        &self,
        index: IndexInt,
    ) -> Result<GetAnyCertificateData, GetCertificateError> {
        todo!()
    }

    /// Not if multiple results are possible, the highest index is kept (i.e. latest certificate)
    pub async fn get_certificate_encrypted(
        &self,
        query: GetCertificateQuery,
        up_to: UpTo,
    ) -> Result<(IndexInt, Vec<u8>), GetCertificateError> {
        todo!()
    }

    /// Certificates are returned ordered by index in increasing order
    pub async fn get_multiple_certificates_encrypted(
        &self,
        query: GetCertificateQuery,
        up_to: UpTo,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> anyhow::Result<Vec<(IndexInt, Vec<u8>)>> {
        todo!()
    }
}
