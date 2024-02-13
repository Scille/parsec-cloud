// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! Certificate storage relies on the upper layer to do the actual certification
//! validation work and takes care of handling concurrency issues.
//!
//! Hence no unique violation should occur under normal circumstances here.
//!
//! On top of that, storage is only responsible for storing/fetching encrypted items:
//! - No cache is handled at this level: better let the higher level components
//!   do that since they have a better idea of what should be cached.
//! - No encryption/serialization: simplify code (no need for type dispatching)
//!   and testing (no need to build valid certificates objects.

use paste::paste;
use std::{collections::HashMap, path::Path};

use libparsec_types::prelude::*;

use crate::platform::certificates::{
    PlatformCertificatesStorage, PlatformCertificatesStorageForUpdateGuard,
};

#[derive(Debug, Clone, Copy)]
pub enum UpTo {
    Current,
    Timestamp(DateTime),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerTopicLastTimestamps {
    pub common: Option<DateTime>,
    pub sequester: Option<DateTime>,
    pub realm: HashMap<VlobID, DateTime>,
    pub shamir_recovery: Option<DateTime>,
}

impl PerTopicLastTimestamps {
    pub fn is_empty(&self) -> bool {
        self.common.is_none()
            && self.sequester.is_none()
            && self.realm.is_empty()
            && self.shamir_recovery.is_none()
    }
    pub fn new_for_common(common_timestamp: DateTime) -> Self {
        Self {
            common: Some(common_timestamp),
            sequester: None,
            realm: HashMap::default(),
            shamir_recovery: None,
        }
    }
    pub fn new_for_realm(realm_id: VlobID, realm_timestamp: DateTime) -> Self {
        Self {
            common: None,
            sequester: None,
            realm: HashMap::from([(realm_id, realm_timestamp)]),
            shamir_recovery: None,
        }
    }
    pub fn new_for_common_and_realm(
        common_timestamp: DateTime,
        realm_id: VlobID,
        realm_timestamp: DateTime,
    ) -> Self {
        Self {
            common: Some(common_timestamp),
            sequester: None,
            realm: HashMap::from([(realm_id, realm_timestamp)]),
            shamir_recovery: None,
        }
    }
}

impl PerTopicLastTimestamps {
    /// Ensure we know about all topics specified in other and we have a younger or
    /// equal timestamp on each one of them.
    pub fn is_up_to_date(&self, other: &PerTopicLastTimestamps) -> bool {
        if self.common < other.common
            || self.sequester < other.sequester
            || self.shamir_recovery < other.shamir_recovery
        {
            return false;
        }
        for (realm_id, other_realm_timestamp) in other.realm.iter() {
            match self.realm.get(realm_id) {
                // We don't know about this realm
                None => return false,
                // We are not up to date with this realm
                Some(self_realm_timestamp) if self_realm_timestamp < other_realm_timestamp => {
                    return false
                }
                _ => (),
            }
        }
        true
    }
}

pub trait StorableCertificate {
    /// Values for `certificate_type` column in `certificates` table
    /// Note the fact their value is similar to the `type` field in certificates, this
    /// is purely for simplicity as the two are totally decorrelated.
    const TYPE: &'static str;

    fn filters(&self) -> (Option<String>, Option<String>);
    fn timestamp(&self) -> DateTime;
}

/// Certificates are grouped into topic, each of which being provided by the server
/// with their timestamp strictly growing.
/// We need this group of information in the storage so that we can retrieve
/// the last timestamp of each topic when polling the server for new ones.
pub(super) trait StorableCertificateTopic {
    const TYPES: &'static [&'static str];
    /// This method is not meant to be executed, instead it acts as a canary by making
    /// the compilation fails if a new certificate is added to a topic without updating
    /// the implementation of `StorableCertificateTopic` trait for said topic.
    #[cfg(test)]
    fn topic_certifs_correspondance_canary(certif: Self);
}

macro_rules! impl_storable_certificate_topic {
    ($topic_struct:ident, [ $( $certif:ident ),* $(,)? ]) => {
        impl StorableCertificateTopic for $topic_struct {
            const TYPES: &'static [&'static str] = &[
                $(
                    <paste!{ [< $certif Certificate >] } as StorableCertificate>::TYPE,
                )*
            ];
            #[cfg(test)]
            fn topic_certifs_correspondance_canary(certif: Self) {
                match certif {
                    $(
                        $topic_struct::$certif(_) => (),
                    )*
                }
            }
        }
    };
}

impl_storable_certificate_topic!(
    CommonTopicArcCertificate,
    [User, Device, RevokedUser, UserUpdate,]
);

impl_storable_certificate_topic!(
    SequesterTopicArcCertificate,
    [
        SequesterAuthority,
        SequesterService,
        SequesterRevokedService
    ]
);

impl_storable_certificate_topic!(
    RealmTopicArcCertificate,
    [RealmRole, RealmName, RealmKeyRotation, RealmArchiving]
);

impl_storable_certificate_topic!(
    ShamirRecoveryTopicArcCertificate,
    [ShamirRecoveryBrief, ShamirRecoveryShare]
);

impl StorableCertificate for UserCertificate {
    const TYPE: &'static str = "user_certificate";
    fn filters(&self) -> (Option<String>, Option<String>) {
        user_certificate_filters(self.user_id.clone())
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for DeviceCertificate {
    const TYPE: &'static str = "device_certificate";
    fn filters(&self) -> (Option<String>, Option<String>) {
        device_certificate_filters(self.device_id.clone())
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for RevokedUserCertificate {
    const TYPE: &'static str = "revoked_user_certificate";
    fn filters(&self) -> (Option<String>, Option<String>) {
        revoked_user_certificate_filters(self.user_id.clone())
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for UserUpdateCertificate {
    const TYPE: &'static str = "user_update_certificate";
    fn filters(&self) -> (Option<String>, Option<String>) {
        user_update_certificate_filters(self.user_id.clone())
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for RealmRoleCertificate {
    const TYPE: &'static str = "realm_role_certificate";
    fn filters(&self) -> (Option<String>, Option<String>) {
        realm_role_certificate_filters(self.realm_id, self.user_id.clone())
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for RealmNameCertificate {
    const TYPE: &'static str = "realm_name_certificate";
    fn filters(&self) -> (Option<String>, Option<String>) {
        realm_name_certificate_filters(self.realm_id)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for RealmKeyRotationCertificate {
    const TYPE: &'static str = "realm_key_rotation_certificate";
    fn filters(&self) -> (Option<String>, Option<String>) {
        realm_key_rotation_certificate_filters(self.realm_id, self.key_index)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for RealmArchivingCertificate {
    const TYPE: &'static str = "realm_archiving_certificate";
    fn filters(&self) -> (Option<String>, Option<String>) {
        realm_archiving_certificate_filters(self.realm_id)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for SequesterAuthorityCertificate {
    const TYPE: &'static str = "sequester_authority_certificate";
    fn filters(&self) -> (Option<String>, Option<String>) {
        sequester_authority_certificate_filters()
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for SequesterServiceCertificate {
    const TYPE: &'static str = "sequester_service_certificate";
    fn filters(&self) -> (Option<String>, Option<String>) {
        sequester_service_certificate_filters(self.service_id)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for SequesterRevokedServiceCertificate {
    const TYPE: &'static str = "sequester_revoked_service_certificate";
    fn filters(&self) -> (Option<String>, Option<String>) {
        sequester_revoked_service_certificate_filters(self.service_id)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for ShamirRecoveryBriefCertificate {
    const TYPE: &'static str = "shamir_recovery_brief_certificate";
    fn filters(&self) -> (Option<String>, Option<String>) {
        shamir_recovery_brief_certificate_filters(self.author.user_id().to_owned())
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for ShamirRecoveryShareCertificate {
    const TYPE: &'static str = "shamir_recovery_share_certificate";
    fn filters(&self) -> (Option<String>, Option<String>) {
        shamir_recovery_share_certificate_filters(
            self.author.user_id().to_owned(),
            self.recipient.clone(),
        )
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

fn user_certificate_filters(user_id: UserID) -> (Option<String>, Option<String>) {
    let filter1 = Some(user_id.into());
    let filter2 = None;
    (filter1, filter2)
}

/// Get all user certificates
fn users_certificates_filters() -> (Option<String>, Option<String>) {
    let filter1 = None;
    let filter2 = None;
    (filter1, filter2)
}

fn device_certificate_filters(device_id: DeviceID) -> (Option<String>, Option<String>) {
    let (user_id, device_name) = device_id.into();
    // DeviceName is already unique enough, so we provide it as first filter
    // to speed up database lookup
    let filter1 = Some(device_name.into());
    let filter2 = Some(user_id.into());
    (filter1, filter2)
}

/// Get all device certificates for a given user
fn user_device_certificates_filters(user_id: UserID) -> (Option<String>, Option<String>) {
    let filter1 = None;
    let filter2 = Some(user_id.into());
    (filter1, filter2)
}

fn revoked_user_certificate_filters(user_id: UserID) -> (Option<String>, Option<String>) {
    let filter1 = Some(user_id.into());
    let filter2 = None;
    (filter1, filter2)
}

fn user_update_certificate_filters(user_id: UserID) -> (Option<String>, Option<String>) {
    let filter1 = Some(user_id.into());
    let filter2 = None;
    (filter1, filter2)
}

/// Get all user update certificates for a given user
fn user_update_certificates_filters(user_id: UserID) -> (Option<String>, Option<String>) {
    // User update certificates only have filter on the user
    user_update_certificate_filters(user_id)
}

fn realm_role_certificate_filters(
    realm_id: VlobID,
    user_id: UserID,
) -> (Option<String>, Option<String>) {
    let filter1 = Some(realm_id.hex());
    let filter2 = Some(user_id.into());
    (filter1, filter2)
}

fn realm_name_certificate_filters(realm_id: VlobID) -> (Option<String>, Option<String>) {
    let filter1 = Some(realm_id.hex());
    (filter1, None)
}

fn realm_key_rotation_certificate_filters(
    realm_id: VlobID,
    key_index: IndexInt,
) -> (Option<String>, Option<String>) {
    let filter1 = Some(realm_id.hex());
    let filter2 = Some(key_index.to_string());
    (filter1, filter2)
}

/// Get all realm name certificates for a given realm
fn realm_key_rotation_certificates_filters(realm_id: VlobID) -> (Option<String>, Option<String>) {
    let filter1 = Some(realm_id.hex());
    let filter2 = None;
    (filter1, filter2)
}

fn realm_archiving_certificate_filters(realm_id: VlobID) -> (Option<String>, Option<String>) {
    let filter1 = Some(realm_id.hex());
    (filter1, None)
}

/// Get all realm role certificates for a given realm
fn realm_role_certificates_filters(realm_id: VlobID) -> (Option<String>, Option<String>) {
    let filter1 = Some(realm_id.hex());
    let filter2 = None;
    (filter1, filter2)
}

/// Get all realm role certificates for a given user
fn user_realm_role_certificates_filters(user_id: UserID) -> (Option<String>, Option<String>) {
    let filter1 = None;
    let filter2 = Some(user_id.into());
    (filter1, filter2)
}

fn sequester_authority_certificate_filters() -> (Option<String>, Option<String>) {
    // No filter is needed as there is a most one authority certificate
    let filter1 = None;
    let filter2 = None;
    (filter1, filter2)
}

fn shamir_recovery_brief_certificate_filters(author: UserID) -> (Option<String>, Option<String>) {
    let filter1 = Some(author.into());
    (filter1, None)
}

fn shamir_recovery_share_certificate_filters(
    author: UserID,
    recipient: UserID,
) -> (Option<String>, Option<String>) {
    let filter1 = Some(author.into());
    let filter2 = Some(recipient.into());
    (filter1, filter2)
}

fn sequester_service_certificate_filters(
    service_id: SequesterServiceID,
) -> (Option<String>, Option<String>) {
    let filter1 = Some(service_id.hex());
    let filter2 = None;
    (filter1, filter2)
}

// Get all sequester service certificates
fn sequester_service_certificates_filters() -> (Option<String>, Option<String>) {
    let filter1 = None;
    let filter2 = None;
    (filter1, filter2)
}

fn sequester_revoked_service_certificate_filters(
    service_id: SequesterServiceID,
) -> (Option<String>, Option<String>) {
    let filter1 = Some(service_id.hex());
    let filter2 = None;
    (filter1, filter2)
}

// Get all sequester revoked service certificates
fn sequester_revoked_service_certificates_filters() -> (Option<String>, Option<String>) {
    let filter1 = None;
    let filter2 = None;
    (filter1, filter2)
}

#[derive(Debug, thiserror::Error)]
pub enum GetCertificateError {
    #[error("Certificate doesn't exist")]
    NonExisting,
    #[error("Certificate exists but is more recent than the provided timestamp")]
    ExistButTooRecent { certificate_timestamp: DateTime },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
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
        let (filter1, filter2) = users_certificates_filters();
        Self {
            certificate_type: <UserCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    pub fn user_certificate(user_id: UserID) -> Self {
        let (filter1, filter2) = user_certificate_filters(user_id);
        Self {
            certificate_type: <UserCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    pub fn revoked_user_certificate(user_id: UserID) -> Self {
        let (filter1, filter2) = revoked_user_certificate_filters(user_id);
        Self {
            certificate_type: <RevokedUserCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    /// Get all user update certificates for a given user
    pub fn user_update_certificates(user_id: UserID) -> Self {
        let (filter1, filter2) = user_update_certificates_filters(user_id);
        Self {
            certificate_type: <UserUpdateCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    pub fn device_certificate(device_id: DeviceID) -> Self {
        let (filter1, filter2) = device_certificate_filters(device_id);
        Self {
            certificate_type: <DeviceCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    /// Get all device certificates for a given user
    pub fn user_device_certificates(user_id: UserID) -> Self {
        let (filter1, filter2) = user_device_certificates_filters(user_id);
        Self {
            certificate_type: <DeviceCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    pub fn realm_role_certificate(realm_id: VlobID, user_id: UserID) -> Self {
        let (filter1, filter2) = realm_role_certificate_filters(realm_id, user_id);
        Self {
            certificate_type: <RealmRoleCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    /// Get all realm name certificates for a given realm
    pub fn realm_name_certificates(realm_id: VlobID) -> Self {
        let (filter1, filter2) = realm_name_certificate_filters(realm_id);
        Self {
            certificate_type: <RealmNameCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    pub fn realm_key_rotation_certificate(realm_id: VlobID, key_index: IndexInt) -> Self {
        let (filter1, filter2) = realm_key_rotation_certificate_filters(realm_id, key_index);
        Self {
            certificate_type: <RealmKeyRotationCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    /// Get all realm name certificates for a given realm
    pub fn realm_key_rotation_certificates(realm_id: VlobID) -> Self {
        let (filter1, filter2) = realm_key_rotation_certificates_filters(realm_id);
        Self {
            certificate_type: <RealmKeyRotationCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    /// Get all realm archiving certificates for a given realm
    pub fn realm_archiving_certificates(realm_id: VlobID) -> Self {
        let (filter1, filter2) = realm_archiving_certificate_filters(realm_id);
        Self {
            certificate_type: <RealmArchivingCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    /// Get all realm role certificates for a given realm
    pub fn realm_role_certificates(realm_id: VlobID) -> Self {
        let (filter1, filter2) = realm_role_certificates_filters(realm_id);
        Self {
            certificate_type: <RealmRoleCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    /// Get all realm role certificates for a given user
    pub fn user_realm_role_certificates(user_id: UserID) -> Self {
        let (filter1, filter2) = user_realm_role_certificates_filters(user_id);
        Self {
            certificate_type: <RealmRoleCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    /// Get all shamir recovery brief certificates we know about
    pub fn shamir_recovery_brief_certificates() -> Self {
        Self {
            certificate_type: <ShamirRecoveryBriefCertificate as StorableCertificate>::TYPE,
            filter1: None,
            filter2: None,
        }
    }

    /// Get all shamir recovery share certificates we know about
    pub fn shamir_recovery_share_certificates() -> Self {
        Self {
            certificate_type: <ShamirRecoveryShareCertificate as StorableCertificate>::TYPE,
            filter1: None,
            filter2: None,
        }
    }

    /// Get all shamir recovery brief certificates for a given user
    pub fn user_shamir_recovery_brief_certificates(user_id: UserID) -> Self {
        let (filter1, filter2) = shamir_recovery_brief_certificate_filters(user_id);
        Self {
            certificate_type: <ShamirRecoveryBriefCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    /// Get the recipient's shamir recovery share certificate to recover author
    pub fn user_recipient_shamir_recovery_share_certificates(
        author: UserID,
        recipient: UserID,
    ) -> Self {
        let (filter1, filter2) = shamir_recovery_share_certificate_filters(author, recipient);
        Self {
            certificate_type: <ShamirRecoveryShareCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    pub fn sequester_authority_certificate() -> Self {
        let (filter1, filter2) = sequester_authority_certificate_filters();
        Self {
            certificate_type: <SequesterAuthorityCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    pub fn sequester_service_certificate(service_id: SequesterServiceID) -> Self {
        let (filter1, filter2) = sequester_service_certificate_filters(service_id);
        Self {
            certificate_type: <SequesterServiceCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    // Get all sequester service certificates
    pub fn sequester_service_certificates() -> Self {
        let (filter1, filter2) = sequester_service_certificates_filters();
        Self {
            certificate_type: <SequesterServiceCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    pub fn sequester_revoked_service_certificate(service_id: SequesterServiceID) -> Self {
        let (filter1, filter2) = sequester_revoked_service_certificate_filters(service_id);
        Self {
            certificate_type: <SequesterRevokedServiceCertificate as StorableCertificate>::TYPE,
            filter1,
            filter2,
        }
    }

    // Get all sequester service certificates
    pub fn sequester_revoked_service_certificates() -> Self {
        let (filter1, filter2) = sequester_revoked_service_certificates_filters();
        Self {
            certificate_type: <SequesterRevokedServiceCertificate as StorableCertificate>::TYPE,
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
        // leading to a recursive call which is not supported for async functions.
        // Hence `no_populate_start` which breaks the recursion.
        //
        // Also note we don't try to return the `CertificatesStorage` that has been
        // used during the populate as it would change the internal state of the
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

    pub async fn stop(self) -> anyhow::Result<()> {
        self.platform.stop().await
    }

    pub async fn for_update(&mut self) -> anyhow::Result<CertificatesStorageUpdater<'_>> {
        self.platform
            .for_update()
            .await
            .map(|platform| CertificatesStorageUpdater { platform })
    }

    /// Return the last certificate timestamp we know about for the given topic, or `None`
    /// if the storage is empty regarding this topic (and the topics it depends on).
    pub async fn get_last_timestamps(&mut self) -> anyhow::Result<PerTopicLastTimestamps> {
        self.platform.get_last_timestamps().await
    }

    /// Note if multiple results are possible, the latest certificate is kept
    pub async fn get_certificate_encrypted(
        &mut self,
        query: GetCertificateQuery,
        up_to: UpTo,
    ) -> Result<(DateTime, Vec<u8>), GetCertificateError> {
        self.platform.get_certificate_encrypted(query, up_to).await
    }

    /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
    pub async fn get_multiple_certificates_encrypted(
        &mut self,
        query: GetCertificateQuery,
        up_to: UpTo,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<(DateTime, Vec<u8>)>> {
        self.platform
            .get_multiple_certificates_encrypted(query, up_to, offset, limit)
            .await
    }

    /// Only used for debugging tests
    #[allow(unused)]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        self.platform.debug_dump().await
    }
}

#[derive(Debug)]
pub struct CertificatesStorageUpdater<'a> {
    platform: PlatformCertificatesStorageForUpdateGuard<'a>,
}

impl<'a> CertificatesStorageUpdater<'a> {
    /// Finish the update and commit the underlying transaction to database.
    /// If the guard is dropped without calling this method, the transaction
    /// is rollback.
    pub async fn commit(self) -> anyhow::Result<()> {
        self.platform.commit().await
    }

    /// Remove all certificates from the database
    /// There is no data loss from this as certificates can be re-obtained from
    /// the server, however it is only needed when switching from/to redacted
    /// certificates
    pub async fn forget_all_certificates(&mut self) -> anyhow::Result<()> {
        self.platform.forget_all_certificates().await
    }

    /// Add a (already validated) certificate.
    pub async fn add_certificate<C: StorableCertificate>(
        &mut self,
        certif: &C,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        let (filter1, filter2) = certif.filters();
        self.platform
            .add_certificate(C::TYPE, filter1, filter2, certif.timestamp(), encrypted)
            .await
    }

    /// Return the last certificate timestamp we know about for the given topic, or `None`
    /// if the storage is empty regarding this topic (and the topics it depends on).
    pub async fn get_last_timestamps(&mut self) -> anyhow::Result<PerTopicLastTimestamps> {
        self.platform.get_last_timestamps().await
    }

    /// Note if multiple results are possible, the latest certificate is kept
    pub async fn get_certificate_encrypted(
        &mut self,
        query: GetCertificateQuery,
        up_to: UpTo,
    ) -> Result<(DateTime, Vec<u8>), GetCertificateError> {
        self.platform.get_certificate_encrypted(query, up_to).await
    }

    /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
    pub async fn get_multiple_certificates_encrypted(
        &mut self,
        query: GetCertificateQuery,
        up_to: UpTo,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<(DateTime, Vec<u8>)>> {
        self.platform
            .get_multiple_certificates_encrypted(query, up_to, offset, limit)
            .await
    }

    /// Only used for debugging tests
    #[allow(unused)]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        self.platform.debug_dump().await
    }
}

#[cfg(test)]
#[path = "../tests/unit/certificates.rs"]
mod test;
