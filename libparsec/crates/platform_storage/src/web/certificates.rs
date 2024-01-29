#![allow(unused_variables)]
// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{marker::PhantomData, path::Path};

use libparsec_types::prelude::*;

use crate::certificates::{GetCertificateError, GetCertificateQuery, PerTopicLastTimestamps, UpTo};

#[derive(Debug)]
pub(crate) struct PlatformCertificatesStorageForUpdateGuard<'a> {
    _marker: PhantomData<&'a ()>,
}

impl<'a> PlatformCertificatesStorageForUpdateGuard<'a> {
    pub async fn commit(self) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn get_certificate_encrypted(
        &mut self,
        query: GetCertificateQuery,
        up_to: UpTo,
    ) -> Result<(DateTime, Vec<u8>), GetCertificateError> {
        todo!()
    }

    /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
    pub async fn get_multiple_certificates_encrypted(
        &mut self,
        query: GetCertificateQuery,
        up_to: UpTo,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<(DateTime, Vec<u8>)>> {
        todo!()
    }

    pub async fn forget_all_certificates(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn add_certificate(
        &mut self,
        certificate_type: &'static str,
        filter1: Option<String>,
        filter2: Option<String>,
        timestamp: DateTime,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn get_last_timestamps(&mut self) -> anyhow::Result<PerTopicLastTimestamps> {
        todo!()
    }

    /// Only used for debugging tests
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct PlatformCertificatesStorage {}

impl PlatformCertificatesStorage {
    pub async fn no_populate_start(
        data_base_dir: &Path,
        device: &LocalDevice,
    ) -> anyhow::Result<Self> {
        todo!()
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn for_update(
        &mut self,
    ) -> anyhow::Result<PlatformCertificatesStorageForUpdateGuard> {
        todo!()
    }

    pub async fn get_last_timestamps(&mut self) -> anyhow::Result<PerTopicLastTimestamps> {
        todo!()
    }

    pub async fn get_certificate_encrypted(
        &mut self,
        query: GetCertificateQuery,
        up_to: UpTo,
    ) -> Result<(DateTime, Vec<u8>), GetCertificateError> {
        todo!()
    }

    pub async fn get_multiple_certificates_encrypted(
        &mut self,
        query: GetCertificateQuery,
        up_to: UpTo,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<(DateTime, Vec<u8>)>> {
        todo!()
    }

    /// Only used for debugging tests
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        todo!()
    }
}
