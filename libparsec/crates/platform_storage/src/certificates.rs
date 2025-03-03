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
    pub fn new_for_sequester(sequester_timestamp: DateTime) -> Self {
        Self {
            common: None,
            sequester: Some(sequester_timestamp),
            realm: HashMap::default(),
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
    pub fn new_for_shamir(shamir_timestamp: DateTime) -> Self {
        Self {
            common: None,
            sequester: None,
            realm: HashMap::default(),
            shamir_recovery: Some(shamir_timestamp),
        }
    }
    pub fn new_for_common_and_shamir(
        common_timestamp: DateTime,
        shamir_timestamp: DateTime,
    ) -> Self {
        Self {
            common: Some(common_timestamp),
            sequester: None,
            realm: HashMap::new(),
            shamir_recovery: Some(shamir_timestamp),
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

    fn filters(&self) -> (FilterKind, FilterKind);
    fn timestamp(&self) -> DateTime;
}

#[derive(Debug, Clone)]
pub enum FilterKind<'a> {
    Null,
    Bytes(&'a [u8]),
    U64([u8; 8]),
}

impl FilterKind<'_> {
    pub fn from_u64(value: u64) -> Self {
        Self::U64(value.to_le_bytes())
    }
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
    ///
    /// See the implementation of this method in the [`impl_storable_certificate_topic!`] macro.
    #[cfg(test)]
    #[allow(unused)]
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
                // `certif` is an enum defined in `libparsec_types` (it is considered the
                // source of truth that gets updated whenever a new certificate is added
                // to a topic).
                //
                // We match it against the possible variants listed in the macro invocation,
                // so if both are not in sync this will trigger a compilation error.
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
    [
        ShamirRecoveryBrief,
        ShamirRecoveryShare,
        ShamirRecoveryDeletion
    ]
);

impl StorableCertificate for UserCertificate {
    const TYPE: &'static str = "user_certificate";
    fn filters(&self) -> (FilterKind, FilterKind) {
        let filter1 = FilterKind::Bytes(self.user_id.as_bytes());
        let filter2 = FilterKind::Null;
        (filter1, filter2)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for DeviceCertificate {
    const TYPE: &'static str = "device_certificate";
    fn filters(&self) -> (FilterKind, FilterKind) {
        let filter1 = FilterKind::Bytes(self.device_id.as_bytes());
        let filter2 = FilterKind::Bytes(self.user_id.as_bytes());
        (filter1, filter2)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for RevokedUserCertificate {
    const TYPE: &'static str = "revoked_user_certificate";
    fn filters(&self) -> (FilterKind, FilterKind) {
        let filter1 = FilterKind::Bytes(self.user_id.as_bytes());
        let filter2 = FilterKind::Null;
        (filter1, filter2)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for UserUpdateCertificate {
    const TYPE: &'static str = "user_update_certificate";
    fn filters(&self) -> (FilterKind, FilterKind) {
        let filter1 = FilterKind::Bytes(self.user_id.as_bytes());
        let filter2 = FilterKind::Null;
        (filter1, filter2)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for RealmRoleCertificate {
    const TYPE: &'static str = "realm_role_certificate";
    fn filters(&self) -> (FilterKind, FilterKind) {
        let filter1 = FilterKind::Bytes(self.realm_id.as_bytes());
        let filter2 = FilterKind::Bytes(self.user_id.as_bytes());
        (filter1, filter2)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for RealmNameCertificate {
    const TYPE: &'static str = "realm_name_certificate";
    fn filters(&self) -> (FilterKind, FilterKind) {
        let filter1 = FilterKind::Bytes(self.realm_id.as_bytes());
        let filter2 = FilterKind::Null;
        (filter1, filter2)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for RealmKeyRotationCertificate {
    const TYPE: &'static str = "realm_key_rotation_certificate";
    fn filters(&self) -> (FilterKind, FilterKind) {
        let filter1 = FilterKind::Bytes(self.realm_id.as_bytes());
        let filter2 = FilterKind::from_u64(self.key_index);
        (filter1, filter2)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for RealmArchivingCertificate {
    const TYPE: &'static str = "realm_archiving_certificate";
    fn filters(&self) -> (FilterKind, FilterKind) {
        let filter1 = FilterKind::Bytes(self.realm_id.as_bytes());
        let filter2 = FilterKind::Null;
        (filter1, filter2)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for SequesterAuthorityCertificate {
    const TYPE: &'static str = "sequester_authority_certificate";
    fn filters(&self) -> (FilterKind, FilterKind) {
        // No filter is needed as there is a most one authority certificate
        let filter1 = FilterKind::Null;
        let filter2 = FilterKind::Null;
        (filter1, filter2)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for SequesterServiceCertificate {
    const TYPE: &'static str = "sequester_service_certificate";
    fn filters(&self) -> (FilterKind, FilterKind) {
        let filter1 = FilterKind::Bytes(self.service_id.as_bytes());
        let filter2 = FilterKind::Null;
        (filter1, filter2)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for SequesterRevokedServiceCertificate {
    const TYPE: &'static str = "sequester_revoked_service_certificate";
    fn filters(&self) -> (FilterKind, FilterKind) {
        let filter1 = FilterKind::Bytes(self.service_id.as_bytes());
        let filter2 = FilterKind::Null;
        (filter1, filter2)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for ShamirRecoveryBriefCertificate {
    const TYPE: &'static str = "shamir_recovery_brief_certificate";
    fn filters(&self) -> (FilterKind, FilterKind) {
        let filter1 = FilterKind::Bytes(self.user_id.as_bytes());
        let filter2 = FilterKind::Null;
        (filter1, filter2)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for ShamirRecoveryShareCertificate {
    const TYPE: &'static str = "shamir_recovery_share_certificate";
    fn filters(&self) -> (FilterKind, FilterKind) {
        let filter1 = FilterKind::Bytes(self.user_id.as_bytes());
        let filter2 = FilterKind::Bytes(self.recipient.as_bytes());
        (filter1, filter2)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

impl StorableCertificate for ShamirRecoveryDeletionCertificate {
    const TYPE: &'static str = "shamir_recovery_deletion_certificate";
    fn filters(&self) -> (FilterKind, FilterKind) {
        let filter1 = FilterKind::Bytes(self.setup_to_delete_user_id.as_bytes());
        let filter2 = FilterKind::Null;
        (filter1, filter2)
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
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
pub enum GetCertificateQuery<'a> {
    /// SELECT * FROM certificates
    NoFilter { certificate_type: &'static str },
    /// SELECT * FROM certificates WHERE type = ? AND filter1 = ?
    Filter1 {
        certificate_type: &'static str,
        filter1: FilterKind<'a>,
    },
    /// SELECT * FROM certificates WHERE type = ? AND filter2 = ?
    Filter2 {
        certificate_type: &'static str,
        filter2: FilterKind<'a>,
    },
    /// SELECT * FROM certificates WHERE type = ? AND filter1 = ? AND filter2 = ?
    BothFilters {
        certificate_type: &'static str,
        filter1: FilterKind<'a>,
        filter2: FilterKind<'a>,
    },
    /// SELECT * FROM certificates WHERE type = ? AND filter1 = (SELECT filter2 FROM certificates WHERE type = ? AND filter1 = ?)
    Filter1EqFilter2WhereFilter1 {
        certificate_type: &'static str,
        subquery_certificate_type: &'static str,
        filter1: FilterKind<'a>,
    },
    /// SELECT * FROM certificates WHERE type = ? AND filter1 = (SELECT filter1 FROM certificates WHERE type = ? AND filter2 = ?)
    Filter1EqFilter1WhereFilter2 {
        certificate_type: &'static str,
        subquery_certificate_type: &'static str,
        filter2: FilterKind<'a>,
    },
    /// SELECT * FROM certificates WHERE type = ? AND filter2 = (SELECT filter1 FROM certificates WHERE type = ? AND filter2 = ?)
    Filter2EqFilter1WhereFilter2 {
        certificate_type: &'static str,
        subquery_certificate_type: &'static str,
        filter2: FilterKind<'a>,
    },
    /// SELECT * FROM certificates WHERE type = ? AND filter2 = (SELECT filter2 FROM certificates WHERE type = ? AND filter1 = ?)
    Filter2EqFilter2WhereFilter1 {
        certificate_type: &'static str,
        subquery_certificate_type: &'static str,
        filter1: FilterKind<'a>,
    },
}

impl<'a> GetCertificateQuery<'a> {
    /// Get all users
    pub fn users_certificates() -> Self {
        Self::NoFilter {
            certificate_type: <UserCertificate as StorableCertificate>::TYPE,
        }
    }

    pub fn user_certificate(user_id: &'a UserID) -> Self {
        Self::Filter1 {
            certificate_type: <UserCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(user_id.as_bytes()),
        }
    }

    pub fn user_certificate_from_device_id(device_id: &'a DeviceID) -> Self {
        Self::Filter1EqFilter2WhereFilter1 {
            certificate_type: <UserCertificate as StorableCertificate>::TYPE,
            subquery_certificate_type: <DeviceCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(device_id.as_bytes()),
        }
    }

    pub fn revoked_user_certificate(user_id: &'a UserID) -> Self {
        Self::Filter1 {
            certificate_type: <RevokedUserCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(user_id.as_bytes()),
        }
    }

    pub fn revoked_user_certificate_from_device_id(device_id: &'a DeviceID) -> Self {
        Self::Filter1EqFilter2WhereFilter1 {
            certificate_type: <RevokedUserCertificate as StorableCertificate>::TYPE,
            subquery_certificate_type: <DeviceCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(device_id.as_bytes()),
        }
    }

    /// Get all user update certificates for a given user
    pub fn user_update_certificates(user_id: &'a UserID) -> Self {
        Self::Filter1 {
            certificate_type: <UserUpdateCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(user_id.as_bytes()),
        }
    }

    /// Get all user update certificates for a given user
    pub fn user_update_certificates_from_device_id(device_id: &'a DeviceID) -> Self {
        Self::Filter1EqFilter2WhereFilter1 {
            certificate_type: <UserUpdateCertificate as StorableCertificate>::TYPE,
            subquery_certificate_type: <DeviceCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(device_id.as_bytes()),
        }
    }

    pub fn device_certificate(device_id: &'a DeviceID) -> Self {
        Self::Filter1 {
            certificate_type: <DeviceCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(device_id.as_bytes()),
        }
    }

    /// Get all device certificates for a given user
    pub fn user_devices_certificates(user_id: &'a UserID) -> Self {
        Self::Filter2 {
            certificate_type: <DeviceCertificate as StorableCertificate>::TYPE,
            filter2: FilterKind::Bytes(user_id.as_bytes()),
        }
    }

    pub fn realm_role_certificate(realm_id: &'a VlobID, user_id: &'a UserID) -> Self {
        Self::BothFilters {
            certificate_type: <RealmRoleCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(realm_id.as_bytes()),
            filter2: FilterKind::Bytes(user_id.as_bytes()),
        }
    }

    /// Get all realm name certificates for a given realm
    pub fn realm_name_certificates(realm_id: &'a VlobID) -> Self {
        Self::Filter1 {
            certificate_type: <RealmNameCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(realm_id.as_bytes()),
        }
    }

    pub fn realm_key_rotation_certificate(realm_id: &'a VlobID, key_index: IndexInt) -> Self {
        Self::BothFilters {
            certificate_type: <RealmKeyRotationCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(realm_id.as_bytes()),
            filter2: FilterKind::from_u64(key_index),
        }
    }

    /// Get all realm name certificates for a given realm
    pub fn realm_key_rotation_certificates(realm_id: &'a VlobID) -> Self {
        Self::Filter1 {
            certificate_type: <RealmKeyRotationCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(realm_id.as_bytes()),
        }
    }

    /// Get all realm archiving certificates for a given realm
    pub fn realm_archiving_certificates(realm_id: &'a VlobID) -> Self {
        Self::Filter1 {
            certificate_type: <RealmArchivingCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(realm_id.as_bytes()),
        }
    }

    /// Get all realm role certificates for a given realm
    pub fn realm_role_certificates(realm_id: &'a VlobID) -> Self {
        Self::Filter1 {
            certificate_type: <RealmRoleCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(realm_id.as_bytes()),
        }
    }

    /// Get all realm role certificates for a given user
    pub fn user_realm_role_certificates(user_id: &'a UserID) -> Self {
        Self::Filter2 {
            certificate_type: <RealmRoleCertificate as StorableCertificate>::TYPE,
            filter2: FilterKind::Bytes(user_id.as_bytes()),
        }
    }

    /// Get all shamir recovery brief certificates we know about
    pub fn shamir_recovery_brief_certificates() -> Self {
        Self::NoFilter {
            certificate_type: <ShamirRecoveryBriefCertificate as StorableCertificate>::TYPE,
        }
    }

    /// Get all shamir recovery share certificates we know about
    pub fn shamir_recovery_share_certificates() -> Self {
        Self::NoFilter {
            certificate_type: <ShamirRecoveryShareCertificate as StorableCertificate>::TYPE,
        }
    }

    /// Get all removed shamir recovery certificates we know about
    pub fn shamir_recovery_deletion_certificates() -> Self {
        Self::NoFilter {
            certificate_type: <ShamirRecoveryDeletionCertificate as StorableCertificate>::TYPE,
        }
    }

    /// Get all shamir recovery brief certificates for a given user
    pub fn user_shamir_recovery_brief_certificates(user_id: &'a UserID) -> Self {
        Self::Filter1 {
            certificate_type: <ShamirRecoveryBriefCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(user_id.as_bytes()),
        }
    }

    /// Get the recipient's shamir recovery share certificate to recover author
    pub fn user_recipient_shamir_recovery_share_certificates(
        user_id: &'a UserID,
        recipient: &'a UserID,
    ) -> Self {
        Self::BothFilters {
            certificate_type: <ShamirRecoveryShareCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(user_id.as_bytes()),
            filter2: FilterKind::Bytes(recipient.as_bytes()),
        }
    }

    /// Get all removed shamir recovery certificates for a given user
    pub fn user_shamir_recovery_deletion_certificates(user_id: &'a UserID) -> Self {
        Self::Filter1 {
            certificate_type: <ShamirRecoveryDeletionCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(user_id.as_bytes()),
        }
    }

    pub fn sequester_authority_certificate() -> Self {
        // No filter is needed as there is a most one authority certificate
        Self::NoFilter {
            certificate_type: <SequesterAuthorityCertificate as StorableCertificate>::TYPE,
        }
    }

    pub fn sequester_service_certificate(service_id: &'a SequesterServiceID) -> Self {
        Self::Filter1 {
            certificate_type: <SequesterServiceCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(service_id.as_bytes()),
        }
    }

    // Get all sequester service certificates
    pub fn sequester_service_certificates() -> Self {
        Self::NoFilter {
            certificate_type: <SequesterServiceCertificate as StorableCertificate>::TYPE,
        }
    }

    pub fn sequester_revoked_service_certificate(service_id: &'a SequesterServiceID) -> Self {
        Self::Filter1 {
            certificate_type: <SequesterRevokedServiceCertificate as StorableCertificate>::TYPE,
            filter1: FilterKind::Bytes(service_id.as_bytes()),
        }
    }

    // Get all sequester service certificates
    pub fn sequester_revoked_service_certificates() -> Self {
        Self::NoFilter {
            certificate_type: <SequesterRevokedServiceCertificate as StorableCertificate>::TYPE,
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

    /// Get an exclusive access on the storage to perform modifications.
    ///
    /// Note all changes are done in a transaction, and `cb`'s result is used to
    /// determine if the transaction should be committed or rolled back.
    pub async fn for_update<R, E>(
        &mut self,
        cb: impl AsyncFnOnce(CertificatesStorageUpdater) -> Result<R, E>,
    ) -> anyhow::Result<Result<R, E>> {
        self.platform
            .for_update(async |platform_updater| {
                let updater = CertificatesStorageUpdater {
                    platform: platform_updater,
                };
                cb(updater).await
            })
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
        query: GetCertificateQuery<'_>,
        up_to: UpTo,
    ) -> Result<(DateTime, Vec<u8>), GetCertificateError> {
        self.platform.get_certificate_encrypted(query, up_to).await
    }

    /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
    pub async fn get_multiple_certificates_encrypted(
        &mut self,
        query: GetCertificateQuery<'_>,
        up_to: UpTo,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<(DateTime, Vec<u8>)>> {
        self.platform
            .get_multiple_certificates_encrypted(query, up_to, offset, limit)
            .await
    }

    /// Only used for debugging tests
    #[cfg(any(test, feature = "expose-test-methods"))]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        self.platform.debug_dump().await
    }
}

#[derive(Debug)]
pub struct CertificatesStorageUpdater<'a> {
    platform: PlatformCertificatesStorageForUpdateGuard<'a>,
}

impl CertificatesStorageUpdater<'_> {
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
        query: GetCertificateQuery<'_>,
        up_to: UpTo,
    ) -> Result<(DateTime, Vec<u8>), GetCertificateError> {
        self.platform.get_certificate_encrypted(query, up_to).await
    }

    /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
    pub async fn get_multiple_certificates_encrypted(
        &mut self,
        query: GetCertificateQuery<'_>,
        up_to: UpTo,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<(DateTime, Vec<u8>)>> {
        self.platform
            .get_multiple_certificates_encrypted(query, up_to, offset, limit)
            .await
    }

    /// Only used for debugging tests
    #[cfg(any(test, feature = "expose-test-methods"))]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        self.platform.debug_dump().await
    }
}

#[cfg(test)]
#[path = "../tests/unit/certificates.rs"]
mod test;
