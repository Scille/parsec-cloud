#![allow(unused_variables)]
// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_types::prelude::*;

#[derive(Debug)]
pub struct CertificatesStorage {}

impl CertificatesStorage {
    pub async fn start(data_base_dir: &Path, device: &LocalDevice) -> anyhow::Result<Self> {
        todo!()
    }

    pub(crate) async fn no_populate_start(
        data_base_dir: &Path,
        device: &LocalDevice,
    ) -> anyhow::Result<Self> {
        todo!()
    }

    pub async fn stop(&self) {
        todo!()
    }

    pub async fn add_user_certificate(
        &self,
        index: IndexInt,
        timestamp: DateTime,
        user_id: UserID,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn add_revoked_user_certificate(
        &self,
        index: IndexInt,
        timestamp: DateTime,
        user_id: UserID,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn add_user_update_certificate(
        &self,
        index: IndexInt,
        timestamp: DateTime,
        user_id: UserID,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn add_device_certificate(
        &self,
        index: IndexInt,
        timestamp: DateTime,
        device_id: DeviceID,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn add_realm_role_certificate(
        &self,
        index: IndexInt,
        timestamp: DateTime,
        realm_id: RealmID,
        user_id: UserID,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn add_sequester_authority_certificate(
        &self,
        index: IndexInt,
        timestamp: DateTime,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn add_sequester_service_certificate(
        &self,
        index: IndexInt,
        timestamp: DateTime,
        service_id: SequesterServiceID,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn get_user_certificate(
        &self,
        user_id: UserID,
    ) -> anyhow::Result<Option<(IndexInt, Vec<u8>)>> {
        todo!()
    }

    pub async fn get_revoked_user_certificate(
        &self,
        user_id: UserID,
    ) -> anyhow::Result<Option<(IndexInt, Vec<u8>)>> {
        todo!()
    }

    pub async fn get_user_update_certificates(
        &self,
        user_id: UserID,
    ) -> anyhow::Result<Vec<(IndexInt, Vec<u8>)>> {
        todo!()
    }

    pub async fn get_device_certificate(
        &self,
        device_id: DeviceID,
    ) -> anyhow::Result<Option<(IndexInt, Vec<u8>)>> {
        todo!()
    }

    // Get all realm role certificates for a given realm
    pub async fn get_realm_certificates(
        &self,
        realm_id: RealmID,
    ) -> anyhow::Result<Vec<(IndexInt, Vec<u8>)>> {
        todo!()
    }

    // Get all realm role certificates for a given user
    pub async fn get_user_realms_certificates(
        &self,
        user_id: UserID,
    ) -> anyhow::Result<Vec<(IndexInt, Vec<u8>)>> {
        todo!()
    }

    pub async fn get_sequester_authority_certificate(
        &self,
    ) -> anyhow::Result<Option<(IndexInt, Vec<u8>)>> {
        todo!()
    }

    pub async fn get_sequester_service_certificates(
        &self,
    ) -> anyhow::Result<Vec<(IndexInt, Vec<u8>)>> {
        todo!()
    }

    pub async fn get_certificate(&self, index: IndexInt) -> anyhow::Result<Option<Vec<u8>>> {
        todo!()
    }

    /// Returns the timestamp of creation of the considered certificate index
    /// and (if any) of the certificate index following it.
    /// If this certificate index doesn't exist yet, `(None, None)` is returned.
    pub async fn get_timestamp_bounds(
        &self,
        index: IndexInt,
    ) -> anyhow::Result<(Option<DateTime>, Option<DateTime>)> {
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
}
