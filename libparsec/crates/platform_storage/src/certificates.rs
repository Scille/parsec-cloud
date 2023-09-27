// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! Certificate storage relies on the upper layer to do the actual certification
//! validation work and takes care of handling concurrency issues.
//!
//! Hence no unique violation should occur under normal circumstances here.
//! However, as last failsafe, the certificate storage ensures an index cannot
//! be inserted multiple times, as this is a simple check (using an unique index).
//!
//! On top of that, storage is only responsible for storing/fetching encrypted items:
//! - no cache is handled at this level: better let the higher level components
//!   do that since they have a better idea of what should be cached
//! - no encryption/serialization: simplify code (no need for type dispatching)
//!   and testing (no need to build valid certificates objects

use std::{path::Path, sync::Arc};

use libparsec_types::prelude::*;

use crate::platform::certificates::PlatformCertificatesStorage;

#[derive(Debug, Clone, Copy)]
pub enum UpTo {
    Current,
    Index(IndexInt),
}

#[derive(Debug, thiserror::Error)]
pub enum GetCertificateError {
    #[error("Certificate doesn't exist")]
    NonExisting,
    #[error("Certificate exists but is more recent than the provided index")]
    ExistButTooRecent {
        certificate_index: IndexInt,
        certificate_timestamp: DateTime,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetTimestampBoundsError {
    #[error("Certificate index doesn't exist")]
    NonExisting,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

// Values for `certificate_type` column in `certificates` table
// Note the fact their value is similar to the `type` field in certificates, this
// is purely for simplicity as the two are totally decorrelated.
const USER_CERTIFICATE_TYPE: &str = "user_certificate";
const DEVICE_CERTIFICATE_TYPE: &str = "device_certificate";
const REVOKED_USER_CERTIFICATE_TYPE: &str = "revoked_user_certificate";
const USER_UPDATE_CERTIFICATE_TYPE: &str = "user_update_certificate";
const REALM_ROLE_CERTIFICATE_TYPE: &str = "realm_role_certificate";
const SEQUESTER_AUTHORITY_CERTIFICATE_TYPE: &str = "sequester_authority_certificate";
const SEQUESTER_SERVICE_CERTIFICATE_TYPE: &str = "sequester_service_certificate";

fn users_certificates_filters() -> (&'static str, Option<String>, Option<String>) {
    let filter1 = None;
    let filter2 = None;
    (USER_CERTIFICATE_TYPE, filter1, filter2)
}

fn user_certificate_filters(user_id: UserID) -> (&'static str, Option<String>, Option<String>) {
    let filter1 = Some(user_id.into());
    let filter2 = None;
    (USER_CERTIFICATE_TYPE, filter1, filter2)
}

fn revoked_user_certificate_filters(
    user_id: UserID,
) -> (&'static str, Option<String>, Option<String>) {
    let filter1 = Some(user_id.into());
    let filter2 = None;
    (REVOKED_USER_CERTIFICATE_TYPE, filter1, filter2)
}

fn user_update_certificates_filters(
    user_id: UserID,
) -> (&'static str, Option<String>, Option<String>) {
    let filter1 = Some(user_id.into());
    let filter2 = None;
    (USER_UPDATE_CERTIFICATE_TYPE, filter1, filter2)
}

fn device_certificate_filters(
    device_id: DeviceID,
) -> (&'static str, Option<String>, Option<String>) {
    let (user_id, device_name) = device_id.into();
    // DeviceName is already unique enough, so we provide it as first filter
    // to speed up database lookup
    let filter1 = Some(device_name.into());
    let filter2 = Some(user_id.into());
    (DEVICE_CERTIFICATE_TYPE, filter1, filter2)
}

/// Get all device certificates for a given realm
fn user_device_certificates_filters(
    user_id: UserID,
) -> (&'static str, Option<String>, Option<String>) {
    let filter1 = None;
    let filter2 = Some(user_id.into());
    (DEVICE_CERTIFICATE_TYPE, filter1, filter2)
}

fn realm_role_certificate_filters(
    realm_id: VlobID,
    user_id: UserID,
) -> (&'static str, Option<String>, Option<String>) {
    let filter1 = Some(realm_id.hex());
    let filter2 = Some(user_id.into());
    (REALM_ROLE_CERTIFICATE_TYPE, filter1, filter2)
}

/// Get all realm role certificates for a given realm
fn realm_role_certificates_filters(
    realm_id: VlobID,
) -> (&'static str, Option<String>, Option<String>) {
    let filter1 = Some(realm_id.hex());
    let filter2 = None;
    (REALM_ROLE_CERTIFICATE_TYPE, filter1, filter2)
}

/// Get all realm role certificates for a given user
fn user_realm_role_certificates_filters(
    user_id: UserID,
) -> (&'static str, Option<String>, Option<String>) {
    let filter1 = None;
    let filter2 = Some(user_id.into());
    (REALM_ROLE_CERTIFICATE_TYPE, filter1, filter2)
}

fn sequester_authority_certificate_filters() -> (&'static str, Option<String>, Option<String>) {
    (SEQUESTER_AUTHORITY_CERTIFICATE_TYPE, None, None)
}

fn sequester_service_certificate_filters(
    service_id: SequesterServiceID,
) -> (&'static str, Option<String>, Option<String>) {
    let filter1 = Some(service_id.hex());
    let filter2 = None;
    (SEQUESTER_SERVICE_CERTIFICATE_TYPE, filter1, filter2)
}

// Get all sequester service certificates
fn sequester_service_certificates_filters() -> (&'static str, Option<String>, Option<String>) {
    let filter1 = None;
    let filter2 = None;
    (SEQUESTER_SERVICE_CERTIFICATE_TYPE, filter1, filter2)
}

pub struct GetAnyCertificateData {
    pub(crate) certificate_type: String,
    pub index: IndexInt,
    pub encrypted: Vec<u8>,
}

impl GetAnyCertificateData {
    pub fn decrypt_and_load(&self, key: &SecretKey) -> anyhow::Result<AnyArcCertificate> {
        let signed = key.decrypt(&self.encrypted)?;

        let certif = match self.certificate_type.as_str() {
            USER_CERTIFICATE_TYPE => {
                let unsecure = UserCertificate::unsecure_load(signed.into())?;
                let certif =
                    unsecure.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                AnyArcCertificate::User(Arc::new(certif))
            }
            DEVICE_CERTIFICATE_TYPE => {
                let unsecure = DeviceCertificate::unsecure_load(signed.into())?;
                let certif =
                    unsecure.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                AnyArcCertificate::Device(Arc::new(certif))
            }
            REVOKED_USER_CERTIFICATE_TYPE => {
                let unsecure = RevokedUserCertificate::unsecure_load(signed.into())?;
                let certif =
                    unsecure.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                AnyArcCertificate::RevokedUser(Arc::new(certif))
            }
            USER_UPDATE_CERTIFICATE_TYPE => {
                let unsecure = UserUpdateCertificate::unsecure_load(signed.into())?;
                let certif =
                    unsecure.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                AnyArcCertificate::UserUpdate(Arc::new(certif))
            }
            REALM_ROLE_CERTIFICATE_TYPE => {
                let unsecure = RealmRoleCertificate::unsecure_load(signed.into())?;
                let certif =
                    unsecure.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                AnyArcCertificate::RealmRole(Arc::new(certif))
            }
            SEQUESTER_AUTHORITY_CERTIFICATE_TYPE => {
                let unsecure = SequesterAuthorityCertificate::unsecure_load(signed.into())?;
                let certif =
                    unsecure.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                AnyArcCertificate::SequesterAuthority(Arc::new(certif))
            }
            SEQUESTER_SERVICE_CERTIFICATE_TYPE => {
                let unsecure = SequesterServiceCertificate::unsecure_load(signed.into())?;
                let certif =
                    unsecure.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                AnyArcCertificate::SequesterService(Arc::new(certif))
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Unknown certificate_type `{}`",
                    self.certificate_type
                ))
            }
        };

        Ok(certif)
    }
}

pub struct AddCertificateData {
    // TODO: `allow(dead_code)` is needed due to web version
    #[allow(dead_code)]
    pub(crate) certificate_type: &'static str,
    #[allow(dead_code)]
    pub(crate) timestamp: DateTime,
    #[allow(dead_code)]
    pub(crate) filter1: Option<String>,
    #[allow(dead_code)]
    pub(crate) filter2: Option<String>,
    #[allow(dead_code)]
    pub(crate) encrypted: Vec<u8>,
}

impl AddCertificateData {
    pub fn from_certif(certif: &AnyArcCertificate, encrypted: Vec<u8>) -> Self {
        let (timestamp, (certificate_type, filter1, filter2)) = match certif {
            AnyArcCertificate::User(certif) => (
                certif.timestamp,
                user_certificate_filters(certif.user_id.clone()),
            ),
            AnyArcCertificate::Device(certif) => (
                certif.timestamp,
                device_certificate_filters(certif.device_id.clone()),
            ),
            AnyArcCertificate::UserUpdate(certif) => (
                certif.timestamp,
                user_update_certificates_filters(certif.user_id.clone()),
            ),
            AnyArcCertificate::RevokedUser(certif) => (
                certif.timestamp,
                revoked_user_certificate_filters(certif.user_id.clone()),
            ),
            AnyArcCertificate::RealmRole(certif) => (
                certif.timestamp,
                realm_role_certificate_filters(certif.realm_id, certif.user_id.clone()),
            ),
            AnyArcCertificate::SequesterAuthority(certif) => {
                (certif.timestamp, sequester_authority_certificate_filters())
            }
            AnyArcCertificate::SequesterService(certif) => (
                certif.timestamp,
                sequester_service_certificate_filters(certif.service_id),
            ),
        };
        Self {
            certificate_type,
            timestamp,
            filter1,
            filter2,
            encrypted,
        }
    }

    // Per-certificate constructor is useful for tests

    #[cfg(test)]
    pub(crate) fn from_user_certificate(
        encrypted: Vec<u8>,
        timestamp: DateTime,
        user_id: UserID,
    ) -> Self {
        let (certificate_type, filter1, filter2) = user_certificate_filters(user_id);
        Self {
            certificate_type,
            timestamp,
            filter1,
            filter2,
            encrypted,
        }
    }
    #[cfg(test)]
    pub(crate) fn from_device_certificate(
        encrypted: Vec<u8>,
        timestamp: DateTime,
        device_id: DeviceID,
    ) -> Self {
        let (certificate_type, filter1, filter2) = device_certificate_filters(device_id);
        Self {
            certificate_type,
            timestamp,
            filter1,
            filter2,
            encrypted,
        }
    }
    #[cfg(test)]
    pub(crate) fn from_user_update_certificate(
        encrypted: Vec<u8>,
        timestamp: DateTime,
        user_id: UserID,
    ) -> Self {
        let (certificate_type, filter1, filter2) = user_update_certificates_filters(user_id);
        Self {
            certificate_type,
            timestamp,
            filter1,
            filter2,
            encrypted,
        }
    }
    #[cfg(test)]
    pub(crate) fn from_revoked_user_certificate(
        encrypted: Vec<u8>,
        timestamp: DateTime,
        user_id: UserID,
    ) -> Self {
        let (certificate_type, filter1, filter2) = revoked_user_certificate_filters(user_id);
        Self {
            certificate_type,
            timestamp,
            filter1,
            filter2,
            encrypted,
        }
    }
    #[cfg(test)]
    pub(crate) fn from_realm_role_certificate(
        encrypted: Vec<u8>,
        timestamp: DateTime,
        realm_id: VlobID,
        user_id: UserID,
    ) -> Self {
        let (certificate_type, filter1, filter2) =
            realm_role_certificate_filters(realm_id, user_id);
        Self {
            certificate_type,
            timestamp,
            filter1,
            filter2,
            encrypted,
        }
    }
    #[cfg(test)]
    pub(crate) fn from_sequester_authority_certificate(
        encrypted: Vec<u8>,
        timestamp: DateTime,
    ) -> Self {
        let (certificate_type, filter1, filter2) = sequester_authority_certificate_filters();
        Self {
            certificate_type,
            timestamp,
            filter1,
            filter2,
            encrypted,
        }
    }
    #[cfg(test)]
    pub(crate) fn from_sequester_service_certificate(
        encrypted: Vec<u8>,
        timestamp: DateTime,
        service_id: SequesterServiceID,
    ) -> Self {
        let (certificate_type, filter1, filter2) =
            sequester_service_certificate_filters(service_id);
        Self {
            certificate_type,
            timestamp,
            filter1,
            filter2,
            encrypted,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetCertificateQuery {
    // TODO: `allow(dead_code)` is needed due to web version
    #[allow(dead_code)]
    pub(crate) certificate_type: &'static str,
    #[allow(dead_code)]
    pub(crate) filter1: Option<String>,
    #[allow(dead_code)]
    pub(crate) filter2: Option<String>,
}

impl GetCertificateQuery {
    /// Get all users
    pub fn users_certificates() -> Self {
        let (certificate_type, filter1, filter2) = users_certificates_filters();
        Self {
            certificate_type,
            filter1,
            filter2,
        }
    }

    pub fn user_certificate(user_id: UserID) -> Self {
        let (certificate_type, filter1, filter2) = user_certificate_filters(user_id);
        Self {
            certificate_type,
            filter1,
            filter2,
        }
    }

    pub fn revoked_user_certificate(user_id: UserID) -> Self {
        let (certificate_type, filter1, filter2) = revoked_user_certificate_filters(user_id);
        Self {
            certificate_type,
            filter1,
            filter2,
        }
    }

    /// Get all user update certificates for a given user
    pub fn user_update_certificates(user_id: UserID) -> Self {
        let (certificate_type, filter1, filter2) = user_update_certificates_filters(user_id);
        Self {
            certificate_type,
            filter1,
            filter2,
        }
    }

    pub fn device_certificate(device_id: DeviceID) -> Self {
        let (certificate_type, filter1, filter2) = device_certificate_filters(device_id);
        Self {
            certificate_type,
            filter1,
            filter2,
        }
    }

    /// Get all device certificates for a given user
    pub fn user_device_certificates(user_id: UserID) -> Self {
        let (certificate_type, filter1, filter2) = user_device_certificates_filters(user_id);
        Self {
            certificate_type,
            filter1,
            filter2,
        }
    }

    pub fn realm_role_certificate(realm_id: VlobID, user_id: UserID) -> Self {
        let (certificate_type, filter1, filter2) =
            realm_role_certificate_filters(realm_id, user_id);
        Self {
            certificate_type,
            filter1,
            filter2,
        }
    }

    /// Get all realm role certificates for a given realm
    pub fn realm_role_certificates(realm_id: VlobID) -> Self {
        let (certificate_type, filter1, filter2) = realm_role_certificates_filters(realm_id);
        Self {
            certificate_type,
            filter1,
            filter2,
        }
    }

    /// Get all realm role certificates for a given user
    pub fn user_realm_role_certificates(user_id: UserID) -> Self {
        let (certificate_type, filter1, filter2) = user_realm_role_certificates_filters(user_id);
        Self {
            certificate_type,
            filter1,
            filter2,
        }
    }

    pub fn sequester_authority_certificate() -> Self {
        let (certificate_type, filter1, filter2) = sequester_authority_certificate_filters();
        Self {
            certificate_type,
            filter1,
            filter2,
        }
    }

    pub fn sequester_service_certificate(service_id: SequesterServiceID) -> Self {
        let (certificate_type, filter1, filter2) =
            sequester_service_certificate_filters(service_id);
        Self {
            certificate_type,
            filter1,
            filter2,
        }
    }

    // Get all sequester service certificates
    pub fn sequester_service_certificates() -> Self {
        let (certificate_type, filter1, filter2) = sequester_service_certificates_filters();
        Self {
            certificate_type,
            filter1,
            filter2,
        }
    }
}

#[derive(Debug)]
pub struct CertificatesStorage {
    platform: PlatformCertificatesStorage,
}

impl CertificatesStorage {
    pub async fn start(data_base_dir: &Path, device: &LocalDevice) -> anyhow::Result<Self> {
        // `maybe_populate_certificate_storage` needs to start a `CertificatesStorage`,
        // leading to a recursive call which is not support for async functions.
        // Hence `no_populate_start` which breaks the recursion.
        //
        // Also note we don't try to return the `CertificatesStorage` that has been
        // use during the populate as it would change the internal state of the
        // storage (typically caches) depending of if populate has been needed or not.

        #[cfg(feature = "test-with-testbed")]
        crate::testbed::maybe_populate_certificate_storage(data_base_dir, device).await;

        Self::no_populate_start(data_base_dir, device).await
    }

    pub(crate) async fn no_populate_start(
        data_base_dir: &Path,
        device: &LocalDevice,
    ) -> anyhow::Result<Self> {
        let platform =
            PlatformCertificatesStorage::no_populate_start(data_base_dir, device).await?;
        Ok(Self { platform })
    }

    pub async fn stop(&self) {
        self.platform.stop().await
    }

    #[cfg(test)]
    pub async fn test_get_raw_certificate(
        &self,
        index: IndexInt,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        self.platform.test_get_raw_certificate(index).await
    }

    /// Return the timestamp of creation of the considered certificate index
    /// and (if any) of the certificate index following it.
    /// If this certificate index doesn't exist yet, `(None, None)` is returned.
    pub async fn get_timestamp_bounds(
        &self,
        index: IndexInt,
    ) -> Result<(DateTime, Option<DateTime>), GetTimestampBoundsError> {
        self.platform.get_timestamp_bounds(index).await
    }

    /// Return the last certificate index we know about along with the timestamp
    /// of said certificate, or `None` if the storage is empty.
    pub async fn get_last_index(&self) -> anyhow::Result<Option<(IndexInt, DateTime)>> {
        self.platform.get_last_index().await
    }

    /// Remove all certificates from the database
    /// There is no data loss from this as certificates can be re-obtained from
    /// the server, however it is only needed when switching from/to redacted
    /// certificates
    pub async fn forget_all_certificates(&self) -> anyhow::Result<()> {
        self.platform.forget_all_certificates().await
    }

    pub async fn add_next_certificate(
        &self,
        index: IndexInt,
        data: AddCertificateData,
    ) -> anyhow::Result<()> {
        self.platform.add_certificate(index, data).await
    }

    pub async fn get_any_certificate_encrypted(
        &self,
        index: IndexInt,
    ) -> Result<GetAnyCertificateData, GetCertificateError> {
        self.platform.get_any_certificate_encrypted(index).await
    }

    /// Not if multiple results are possible, the highest index is kept (i.e. latest certificate)
    pub async fn get_certificate_encrypted(
        &self,
        query: GetCertificateQuery,
        up_to: UpTo,
    ) -> Result<(IndexInt, Vec<u8>), GetCertificateError> {
        self.platform.get_certificate_encrypted(query, up_to).await
    }

    /// Certificates are returned ordered by index in increasing order
    pub async fn get_multiple_certificates_encrypted(
        &self,
        query: GetCertificateQuery,
        up_to: UpTo,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> anyhow::Result<Vec<(IndexInt, Vec<u8>)>> {
        self.platform
            .get_multiple_certificates_encrypted(query, up_to, offset, limit)
            .await
    }
}

#[cfg(test)]
#[path = "../tests/unit/certificates.rs"]
mod test;
