// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;
use std::num::NonZeroU64;
use std::sync::{Arc, Mutex};

use libparsec_types::prelude::*;

use super::crc_hash::CrcHash;
use super::{utils, TestbedTemplate, TestbedTemplateBuilder};

#[derive(Default)]
enum TestbedEventCacheEntry<T> {
    Populated(T),
    #[default]
    Stalled,
}

impl<T> TestbedEventCacheEntry<T> {
    fn populated(&mut self, populate: impl FnOnce() -> T) -> &T {
        match self {
            Self::Populated(entry) => entry,
            Self::Stalled => {
                *self = Self::Populated(populate());
                match self {
                    Self::Populated(entry) => entry,
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct TestbedTemplateEventCertificate {
    pub certificate: AnyArcCertificate,
    pub signed: Bytes,
    // `signed_redacted` is the same than `signed` if the certificate has no redacted flavour
    pub signed_redacted: Bytes,
}

type TestbedEventCertificatesCache = TestbedEventCacheEntry<TestbedTemplateEventCertificate>;

macro_rules! impl_event_debug {
    ($struct_name:ident, [ $($field_name: ident: $field_type: ty),+ $(,)? ]) => {
        impl std::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                $( .field(stringify!($field_name), &self.$field_name) )*
                .finish()
            }
        }
    };
}

macro_rules! impl_event_crc_hash {
    ($struct_name:ident, [ $($field_name: ident: $field_type: ty),+ $(,)? ]) => {
        impl CrcHash for $struct_name {
            fn crc_hash(&self, state: &mut crc32fast::Hasher) {
                stringify!($struct_name).crc_hash(state);
                $( self.$field_name.crc_hash(state); )*
            }
        }
    };
}

macro_rules! no_certificate_event {
    ($struct_name:ident, [ $($field_name: ident: $field_type: ty),+ $(,)? ] $(, cache: $cache_type: ty )? $(,)? ) => {
        #[derive(Clone)]
        pub struct $struct_name {
            $( pub $field_name: $field_type,)*
            $( cache: $cache_type, )?
        }
        impl_event_debug!($struct_name, [ $( $field_name: $field_type ),* ]);
        impl_event_crc_hash!($struct_name, [ $( $field_name: $field_type ),* ]);
    };
}

macro_rules! impl_certificates_meth_for_single_certificate {
    ($struct_name:ident, $populate: expr) => {
        impl $struct_name {
            pub fn certificates<'a: 'c, 'b: 'c, 'c>(
                &'a self,
                template: &'b TestbedTemplate,
            ) -> impl Iterator<Item = TestbedTemplateEventCertificate> + 'c {
                std::iter::once(()).map(move |_| {
                    let mut guard = self.cache.lock().expect("Mutex is poisoned");
                    let populate_fn = $populate;
                    let populate = || populate_fn(self, template);
                    guard.populated(populate).to_owned()
                })
            }
        }
    };
}

macro_rules! single_certificate_event {
    ($struct_name:ident, [ $($field_name: ident: $field_type: ty),+ $(,)? ], $populate: expr, no_hash) => {
        #[derive(Clone)]
        pub struct $struct_name {
            $( pub $field_name: $field_type,)*
            cache: Arc<Mutex<TestbedEventCertificatesCache>>,
        }
        impl $struct_name {
            #[allow(clippy::too_many_arguments)]
            pub fn new($( $field_name: $field_type),* ) -> Self {
                Self {
                    $( $field_name, )*
                    cache: Arc::default(),
                }
            }
        }
        impl_event_debug!($struct_name, [ $( $field_name: $field_type ),* ]);
        impl_certificates_meth_for_single_certificate!($struct_name, $populate);
    };
    ($struct_name:ident, [ $($field_name: ident: $field_type: ty),+ $(,)? ], $populate: expr) => {
        #[derive(Clone)]
        pub struct $struct_name {
            $( pub $field_name: $field_type,)*
            cache: Arc<Mutex<TestbedEventCertificatesCache>>,
        }
        impl_event_debug!($struct_name, [ $( $field_name: $field_type ),* ]);
        impl_event_crc_hash!($struct_name, [ $( $field_name: $field_type ),* ]);
        impl_certificates_meth_for_single_certificate!($struct_name, $populate);
    };
}

/*
 * TestbedEvent
 */

#[derive(Debug, Clone)]
pub enum TestbedEvent {
    // 1) Client/server interaction events producing certificates
    BootstrapOrganization(TestbedEventBootstrapOrganization),
    NewSequesterService(TestbedEventNewSequesterService),
    RevokeSequesterService(TestbedEventRevokeSequesterService),
    NewUser(TestbedEventNewUser),
    NewDevice(TestbedEventNewDevice),
    UpdateUserProfile(TestbedEventUpdateUserProfile),
    RevokeUser(TestbedEventRevokeUser),
    NewRealm(TestbedEventNewRealm),
    ShareRealm(TestbedEventShareRealm),
    RenameRealm(TestbedEventRenameRealm),
    RotateKeyRealm(TestbedEventRotateKeyRealm),
    ArchiveRealm(TestbedEventArchiveRealm),
    NewShamirRecovery(TestbedEventNewShamirRecovery),

    // 2) Client/server interaction events not producing certificates
    NewDeviceInvitation(TestbedEventNewDeviceInvitation),
    NewUserInvitation(TestbedEventNewUserInvitation),
    CreateOrUpdateUserManifestVlob(TestbedEventCreateOrUpdateUserManifestVlob),
    CreateOrUpdateWorkspaceManifestVlob(TestbedEventCreateOrUpdateWorkspaceManifestVlob),
    CreateOrUpdateFileManifestVlob(TestbedEventCreateOrUpdateFileManifestVlob),
    CreateOrUpdateFolderManifestVlob(TestbedEventCreateOrUpdateFolderManifestVlob),
    CreateOrUpdateOpaqueVlob(TestbedEventCreateOrUpdateOpaqueVlob),
    CreateBlock(TestbedEventCreateBlock),
    CreateOpaqueBlock(TestbedEventCreateOpaqueBlock),

    // 3) Client-side only events
    CertificatesStorageFetchCertificates(TestbedEventCertificatesStorageFetchCertificates),
    UserStorageFetchUserVlob(TestbedEventUserStorageFetchUserVlob),
    UserStorageFetchRealmCheckpoint(TestbedEventUserStorageFetchRealmCheckpoint),
    UserStorageLocalUpdate(TestbedEventUserStorageLocalUpdate),
    WorkspaceDataStorageFetchWorkspaceVlob(TestbedEventWorkspaceDataStorageFetchWorkspaceVlob),
    WorkspaceDataStorageFetchFileVlob(TestbedEventWorkspaceDataStorageFetchFileVlob),
    WorkspaceDataStorageFetchFolderVlob(TestbedEventWorkspaceDataStorageFetchFolderVlob),
    WorkspaceCacheStorageFetchBlock(TestbedEventWorkspaceCacheStorageFetchBlock),
    WorkspaceDataStorageLocalWorkspaceManifestUpdate(
        TestbedEventWorkspaceDataStorageLocalWorkspaceManifestUpdate,
    ),
    WorkspaceDataStorageLocalFolderManifestCreateOrUpdate(
        TestbedEventWorkspaceDataStorageLocalFolderManifestCreateOrUpdate,
    ),
    WorkspaceDataStorageLocalFileManifestCreateOrUpdate(
        TestbedEventWorkspaceDataStorageLocalFileManifestCreateOrUpdate,
    ),
    WorkspaceDataStorageFetchRealmCheckpoint(TestbedEventWorkspaceDataStorageFetchRealmCheckpoint),
}

impl CrcHash for TestbedEvent {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        match self {
            TestbedEvent::BootstrapOrganization(x) => x.crc_hash(hasher),
            TestbedEvent::NewSequesterService(x) => x.crc_hash(hasher),
            TestbedEvent::RevokeSequesterService(x) => x.crc_hash(hasher),
            TestbedEvent::NewUser(x) => x.crc_hash(hasher),
            TestbedEvent::NewDevice(x) => x.crc_hash(hasher),
            TestbedEvent::UpdateUserProfile(x) => x.crc_hash(hasher),
            TestbedEvent::RevokeUser(x) => x.crc_hash(hasher),
            TestbedEvent::NewDeviceInvitation(x) => x.crc_hash(hasher),
            TestbedEvent::NewUserInvitation(x) => x.crc_hash(hasher),
            TestbedEvent::NewRealm(x) => x.crc_hash(hasher),
            TestbedEvent::ShareRealm(x) => x.crc_hash(hasher),
            TestbedEvent::RenameRealm(x) => x.crc_hash(hasher),
            TestbedEvent::RotateKeyRealm(x) => x.crc_hash(hasher),
            TestbedEvent::ArchiveRealm(x) => x.crc_hash(hasher),
            TestbedEvent::NewShamirRecovery(x) => x.crc_hash(hasher),
            TestbedEvent::CreateOrUpdateUserManifestVlob(x) => x.crc_hash(hasher),
            TestbedEvent::CreateOrUpdateWorkspaceManifestVlob(x) => x.crc_hash(hasher),
            TestbedEvent::CreateOrUpdateFileManifestVlob(x) => x.crc_hash(hasher),
            TestbedEvent::CreateOrUpdateFolderManifestVlob(x) => x.crc_hash(hasher),
            TestbedEvent::CreateOrUpdateOpaqueVlob(x) => x.crc_hash(hasher),
            TestbedEvent::CreateBlock(x) => x.crc_hash(hasher),
            TestbedEvent::CreateOpaqueBlock(x) => x.crc_hash(hasher),
            TestbedEvent::CertificatesStorageFetchCertificates(x) => x.crc_hash(hasher),
            TestbedEvent::UserStorageFetchUserVlob(x) => x.crc_hash(hasher),
            TestbedEvent::UserStorageFetchRealmCheckpoint(x) => x.crc_hash(hasher),
            TestbedEvent::UserStorageLocalUpdate(x) => x.crc_hash(hasher),
            TestbedEvent::WorkspaceDataStorageFetchWorkspaceVlob(x) => x.crc_hash(hasher),
            TestbedEvent::WorkspaceDataStorageFetchFileVlob(x) => x.crc_hash(hasher),
            TestbedEvent::WorkspaceDataStorageFetchFolderVlob(x) => x.crc_hash(hasher),
            TestbedEvent::WorkspaceDataStorageFetchRealmCheckpoint(x) => x.crc_hash(hasher),
            TestbedEvent::WorkspaceCacheStorageFetchBlock(x) => x.crc_hash(hasher),
            TestbedEvent::WorkspaceDataStorageLocalWorkspaceManifestUpdate(x) => x.crc_hash(hasher),
            TestbedEvent::WorkspaceDataStorageLocalFolderManifestCreateOrUpdate(x) => {
                x.crc_hash(hasher)
            }
            TestbedEvent::WorkspaceDataStorageLocalFileManifestCreateOrUpdate(x) => {
                x.crc_hash(hasher)
            }
        }
    }
}

pub enum TestbedEventCertificatesIterator<A, B, C, D, E, F, G, H, I, J, K, L, M>
where
    A: Iterator<Item = TestbedTemplateEventCertificate>,
    B: Iterator<Item = TestbedTemplateEventCertificate>,
    C: Iterator<Item = TestbedTemplateEventCertificate>,
    D: Iterator<Item = TestbedTemplateEventCertificate>,
    E: Iterator<Item = TestbedTemplateEventCertificate>,
    F: Iterator<Item = TestbedTemplateEventCertificate>,
    G: Iterator<Item = TestbedTemplateEventCertificate>,
    H: Iterator<Item = TestbedTemplateEventCertificate>,
    I: Iterator<Item = TestbedTemplateEventCertificate>,
    J: Iterator<Item = TestbedTemplateEventCertificate>,
    K: Iterator<Item = TestbedTemplateEventCertificate>,
    L: Iterator<Item = TestbedTemplateEventCertificate>,
    M: Iterator<Item = TestbedTemplateEventCertificate>,
{
    BootstrapOrganization(A),
    NewSequesterService(B),
    RevokeSequesterService(C),
    NewUser(D),
    NewDevice(E),
    UpdateUserProfile(F),
    RevokeUser(G),
    NewRealm(H),
    ShareRealm(I),
    RenameRealm(J),
    RotateKeyRealm(K),
    ArchiveRealm(L),
    NewShamirRecovery(M),
    Other,
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M> Iterator
    for TestbedEventCertificatesIterator<A, B, C, D, E, F, G, H, I, J, K, L, M>
where
    A: Iterator<Item = TestbedTemplateEventCertificate>,
    B: Iterator<Item = TestbedTemplateEventCertificate>,
    C: Iterator<Item = TestbedTemplateEventCertificate>,
    D: Iterator<Item = TestbedTemplateEventCertificate>,
    E: Iterator<Item = TestbedTemplateEventCertificate>,
    F: Iterator<Item = TestbedTemplateEventCertificate>,
    G: Iterator<Item = TestbedTemplateEventCertificate>,
    H: Iterator<Item = TestbedTemplateEventCertificate>,
    I: Iterator<Item = TestbedTemplateEventCertificate>,
    J: Iterator<Item = TestbedTemplateEventCertificate>,
    K: Iterator<Item = TestbedTemplateEventCertificate>,
    L: Iterator<Item = TestbedTemplateEventCertificate>,
    M: Iterator<Item = TestbedTemplateEventCertificate>,
{
    type Item = TestbedTemplateEventCertificate;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::BootstrapOrganization(iter) => iter.next(),
            Self::NewSequesterService(iter) => iter.next(),
            Self::RevokeSequesterService(iter) => iter.next(),
            Self::NewUser(iter) => iter.next(),
            Self::NewDevice(iter) => iter.next(),
            Self::UpdateUserProfile(iter) => iter.next(),
            Self::RevokeUser(iter) => iter.next(),
            Self::NewRealm(iter) => iter.next(),
            Self::ShareRealm(iter) => iter.next(),
            Self::RotateKeyRealm(iter) => iter.next(),
            Self::RenameRealm(iter) => iter.next(),
            Self::ArchiveRealm(iter) => iter.next(),
            Self::NewShamirRecovery(iter) => iter.next(),
            Self::Other => None,
        }
    }
}

impl TestbedEvent {
    pub fn certificates<'a: 'c, 'b: 'c, 'c>(
        &'a self,
        template: &'b TestbedTemplate,
    ) -> impl Iterator<Item = TestbedTemplateEventCertificate> + 'c {
        match self {
            TestbedEvent::BootstrapOrganization(x) => {
                let iter = x.certificates(template);
                TestbedEventCertificatesIterator::BootstrapOrganization(iter)
            }
            TestbedEvent::NewSequesterService(x) => {
                let iter = x.certificates(template);
                TestbedEventCertificatesIterator::NewSequesterService(iter)
            }
            TestbedEvent::RevokeSequesterService(x) => {
                let iter = x.certificates(template);
                TestbedEventCertificatesIterator::RevokeSequesterService(iter)
            }
            TestbedEvent::NewUser(x) => {
                let iter = x.certificates(template);
                TestbedEventCertificatesIterator::NewUser(iter)
            }
            TestbedEvent::NewDevice(x) => {
                let iter = x.certificates(template);
                TestbedEventCertificatesIterator::NewDevice(iter)
            }
            TestbedEvent::UpdateUserProfile(x) => {
                let iter = x.certificates(template);
                TestbedEventCertificatesIterator::UpdateUserProfile(iter)
            }
            TestbedEvent::RevokeUser(x) => {
                let iter = x.certificates(template);
                TestbedEventCertificatesIterator::RevokeUser(iter)
            }
            TestbedEvent::NewRealm(x) => {
                let iter = x.certificates(template);
                TestbedEventCertificatesIterator::NewRealm(iter)
            }
            TestbedEvent::ShareRealm(x) => {
                let iter = x.certificates(template);
                TestbedEventCertificatesIterator::ShareRealm(iter)
            }
            TestbedEvent::RenameRealm(x) => {
                let iter = x.certificates(template);
                TestbedEventCertificatesIterator::RenameRealm(iter)
            }
            TestbedEvent::RotateKeyRealm(x) => {
                let iter = x.certificates(template);
                TestbedEventCertificatesIterator::RotateKeyRealm(iter)
            }
            TestbedEvent::ArchiveRealm(x) => {
                let iter = x.certificates(template);
                TestbedEventCertificatesIterator::ArchiveRealm(iter)
            }
            TestbedEvent::NewShamirRecovery(x) => {
                let iter = x.certificates(template);
                TestbedEventCertificatesIterator::NewShamirRecovery(iter)
            }

            TestbedEvent::NewDeviceInvitation(_)
            | TestbedEvent::NewUserInvitation(_)
            | TestbedEvent::CreateOrUpdateUserManifestVlob(_)
            | TestbedEvent::CreateOrUpdateWorkspaceManifestVlob(_)
            | TestbedEvent::CreateOrUpdateFileManifestVlob(_)
            | TestbedEvent::CreateOrUpdateFolderManifestVlob(_)
            | TestbedEvent::CreateOrUpdateOpaqueVlob(_)
            | TestbedEvent::CreateBlock(_)
            | TestbedEvent::CreateOpaqueBlock(_)
            | TestbedEvent::CertificatesStorageFetchCertificates(_)
            | TestbedEvent::UserStorageFetchUserVlob(_)
            | TestbedEvent::UserStorageFetchRealmCheckpoint(_)
            | TestbedEvent::UserStorageLocalUpdate(_)
            | TestbedEvent::WorkspaceDataStorageFetchWorkspaceVlob(_)
            | TestbedEvent::WorkspaceDataStorageFetchFileVlob(_)
            | TestbedEvent::WorkspaceDataStorageFetchFolderVlob(_)
            | TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
            | TestbedEvent::WorkspaceDataStorageLocalWorkspaceManifestUpdate(_)
            | TestbedEvent::WorkspaceDataStorageLocalFolderManifestCreateOrUpdate(_)
            | TestbedEvent::WorkspaceDataStorageLocalFileManifestCreateOrUpdate(_)
            | TestbedEvent::WorkspaceDataStorageFetchRealmCheckpoint(_) => {
                TestbedEventCertificatesIterator::Other
            }
        }
    }

    pub fn is_client_side(&self) -> bool {
        match self {
            TestbedEvent::BootstrapOrganization(_)
            | TestbedEvent::NewSequesterService(_)
            | TestbedEvent::RevokeSequesterService(_)
            | TestbedEvent::NewUser(_)
            | TestbedEvent::NewDevice(_)
            | TestbedEvent::UpdateUserProfile(_)
            | TestbedEvent::RevokeUser(_)
            | TestbedEvent::NewDeviceInvitation(_)
            | TestbedEvent::NewUserInvitation(_)
            | TestbedEvent::NewRealm(_)
            | TestbedEvent::ShareRealm(_)
            | TestbedEvent::RenameRealm(_)
            | TestbedEvent::RotateKeyRealm(_)
            | TestbedEvent::ArchiveRealm(_)
            | TestbedEvent::NewShamirRecovery(_)
            | TestbedEvent::CreateOrUpdateUserManifestVlob(_)
            | TestbedEvent::CreateOrUpdateWorkspaceManifestVlob(_)
            | TestbedEvent::CreateOrUpdateFileManifestVlob(_)
            | TestbedEvent::CreateOrUpdateFolderManifestVlob(_)
            | TestbedEvent::CreateOrUpdateOpaqueVlob(_)
            | TestbedEvent::CreateBlock(_)
            | TestbedEvent::CreateOpaqueBlock(_) => false,

            TestbedEvent::CertificatesStorageFetchCertificates(_)
            | TestbedEvent::UserStorageFetchUserVlob(_)
            | TestbedEvent::UserStorageFetchRealmCheckpoint(_)
            | TestbedEvent::UserStorageLocalUpdate(_)
            | TestbedEvent::WorkspaceDataStorageFetchWorkspaceVlob(_)
            | TestbedEvent::WorkspaceDataStorageFetchFileVlob(_)
            | TestbedEvent::WorkspaceDataStorageFetchFolderVlob(_)
            | TestbedEvent::WorkspaceDataStorageFetchRealmCheckpoint(_)
            | TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
            | TestbedEvent::WorkspaceDataStorageLocalWorkspaceManifestUpdate(_)
            | TestbedEvent::WorkspaceDataStorageLocalFolderManifestCreateOrUpdate(_)
            | TestbedEvent::WorkspaceDataStorageLocalFileManifestCreateOrUpdate(_) => true,
        }
    }
}

/*
 * TestbedEventBootstrapOrganization
 */

#[derive(Clone)]
pub struct TestbedEventBootstrapOrganizationSequesterAuthority {
    pub signing_key: SequesterSigningKeyDer,
    pub verify_key: SequesterVerifyKeyDer,
}

#[derive(Clone)]
pub struct TestbedEventBootstrapOrganization {
    pub timestamp: DateTime,
    pub root_signing_key: SigningKey,
    pub sequester_authority: Option<TestbedEventBootstrapOrganizationSequesterAuthority>,
    pub first_user_device_id: DeviceID,
    pub first_user_human_handle: HumanHandle,
    pub first_user_private_key: PrivateKey,
    pub first_user_first_device_label: DeviceLabel,
    pub first_user_first_device_signing_key: SigningKey,
    pub first_user_user_realm_id: VlobID,
    pub first_user_user_realm_key: SecretKey,
    pub first_user_local_symkey: SecretKey,
    pub first_user_local_password: &'static str,
    cache: Arc<
        Mutex<(
            TestbedEventCertificatesCache,
            TestbedEventCertificatesCache,
            TestbedEventCertificatesCache,
        )>,
    >,
}

impl std::fmt::Debug for TestbedEventBootstrapOrganization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TestbedEventBootstrapOrganization")
            .field("timestamp", &self.timestamp)
            .field("first_user", &self.first_user_device_id)
            .field("sequestered", &self.sequester_authority.is_some())
            .finish()
    }
}

impl CrcHash for TestbedEventBootstrapOrganization {
    fn crc_hash(&self, state: &mut crc32fast::Hasher) {
        b"BootstrapOrganization".crc_hash(state);
        self.timestamp.crc_hash(state);
        self.root_signing_key.crc_hash(state);
        if let Some(sequester_authority) = self.sequester_authority.as_ref() {
            // In theory signing and verify keys correspond to one another, but
            // we don't want to do such assumption when computing the CRC
            sequester_authority.signing_key.crc_hash(state);
            sequester_authority.verify_key.crc_hash(state);
        }
        self.first_user_device_id.crc_hash(state);
        self.first_user_human_handle.crc_hash(state);
        self.first_user_private_key.crc_hash(state);
        self.first_user_first_device_label.crc_hash(state);
        self.first_user_first_device_signing_key.crc_hash(state);
        self.first_user_user_realm_id.crc_hash(state);
        self.first_user_user_realm_key.crc_hash(state);
        self.first_user_local_symkey.crc_hash(state);
        self.first_user_local_password.crc_hash(state);
    }
}

impl TestbedEventBootstrapOrganization {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        first_user_id: UserID,
    ) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            assert!(
                builder.events.is_empty(),
                "Organization already bootstrapped !"
            );
        }

        // 2) Actual creation

        let human_handle = HumanHandle::new(&format!("{}@example.com", first_user_id.as_ref()), &{
            let mut buff = format!(
                "{}y Mc{}Face",
                first_user_id.as_ref(),
                first_user_id.as_ref()
            );
            let name_len = first_user_id.as_ref().len();
            // "alicey McaliceFace" -> "Alicey McaliceFace"
            buff[..1].make_ascii_uppercase();
            // "Alicey McaliceFace" -> "Alicey McAliceFace"
            buff[name_len + 4..name_len + 5].make_ascii_uppercase();
            buff
        })
        .unwrap();
        let device_name = "dev1".parse().unwrap();
        let device_label = "My dev1 machine".parse().unwrap();

        Self {
            timestamp: builder.counters.next_timestamp(),
            root_signing_key: builder.counters.next_signing_key(),
            sequester_authority: None,
            first_user_device_id: DeviceID::new(first_user_id, device_name),
            first_user_human_handle: human_handle,
            first_user_private_key: builder.counters.next_private_key(),
            first_user_first_device_label: device_label,
            first_user_first_device_signing_key: builder.counters.next_signing_key(),
            first_user_user_realm_id: builder.counters.next_entry_id(),
            first_user_user_realm_key: builder.counters.next_secret_key(),
            first_user_local_symkey: builder.counters.next_secret_key(),
            first_user_local_password: "P@ssw0rd.",
            cache: Arc::default(),
        }
    }

    pub fn certificates<'a: 'c, 'b: 'c, 'c>(
        &'a self,
        _template: &'b TestbedTemplate,
    ) -> impl Iterator<Item = TestbedTemplateEventCertificate> + 'c {
        (0..3).map_while(move |mut i| {
            if self.sequester_authority.is_none() {
                i += 1;
                if i > 2 {
                    return None;
                }
            }
            let mut guard = self.cache.lock().expect("Mutex is poisoned");
            match i {
                // Sequester service
                0 => {
                    let sequester_authority =
                        self.sequester_authority.as_ref().expect("Already checked");
                    let populate = || {
                        let certif = SequesterAuthorityCertificate {
                            timestamp: self.timestamp,
                            verify_key_der: sequester_authority.verify_key.clone(),
                        };
                        let signed: Bytes = certif.dump_and_sign(&self.root_signing_key).into();
                        TestbedTemplateEventCertificate {
                            certificate: AnyArcCertificate::SequesterAuthority(Arc::new(certif)),
                            signed_redacted: signed.clone(),
                            signed,
                        }
                    };
                    Some(guard.0.populated(populate).to_owned())
                }

                // First user
                1 => {
                    let populate = || {
                        let mut certif = UserCertificate {
                            author: CertificateSignerOwned::Root,
                            timestamp: self.timestamp,
                            user_id: self.first_user_device_id.user_id().to_owned(),
                            human_handle: MaybeRedacted::Redacted(HumanHandle::new_redacted(
                                self.first_user_device_id.user_id(),
                            )),
                            public_key: self.first_user_private_key.public_key(),
                            profile: UserProfile::Admin,
                        };
                        let signed_redacted: Bytes =
                            certif.dump_and_sign(&self.root_signing_key).into();
                        let signed = {
                            certif.human_handle =
                                MaybeRedacted::Real(self.first_user_human_handle.to_owned());
                            certif.dump_and_sign(&self.root_signing_key).into()
                        };
                        TestbedTemplateEventCertificate {
                            certificate: AnyArcCertificate::User(Arc::new(certif)),
                            signed,
                            signed_redacted,
                        }
                    };
                    Some(guard.1.populated(populate).to_owned())
                }

                // First device
                2 => {
                    let populate = || {
                        let mut certif = DeviceCertificate {
                            author: CertificateSignerOwned::Root,
                            timestamp: self.timestamp,
                            device_id: self.first_user_device_id.to_owned(),
                            device_label: MaybeRedacted::Redacted(DeviceLabel::new_redacted(
                                self.first_user_device_id.device_name(),
                            )),
                            verify_key: self.first_user_first_device_signing_key.verify_key(),
                        };
                        let signed_redacted: Bytes =
                            certif.dump_and_sign(&self.root_signing_key).into();
                        let signed = {
                            certif.device_label =
                                MaybeRedacted::Real(self.first_user_first_device_label.to_owned());
                            certif.dump_and_sign(&self.root_signing_key).into()
                        };
                        TestbedTemplateEventCertificate {
                            certificate: AnyArcCertificate::Device(Arc::new(certif)),
                            signed,
                            signed_redacted,
                        }
                    };
                    Some(guard.2.populated(populate).to_owned())
                }
                _ => unreachable!(),
            }
        })
    }
}

/*
 * TestbedEventNewSequesterService
 */

single_certificate_event!(
    TestbedEventNewSequesterService,
    [
        timestamp: DateTime,
        id: SequesterServiceID,
        label: String,
        encryption_private_key: SequesterPrivateKeyDer,
        encryption_public_key: SequesterPublicKeyDer,
    ],
    |e: &TestbedEventNewSequesterService, t: &TestbedTemplate| {
        let certif = SequesterServiceCertificate {
            timestamp: e.timestamp,
            service_id: e.id,
            service_label: e.label.clone(),
            encryption_key_der: e.encryption_public_key.clone(),
        };
        let signed: Bytes = t
            .sequester_authority_signing_key()
            .sign(&certif.dump())
            .into();
        TestbedTemplateEventCertificate {
            certificate: AnyArcCertificate::SequesterService(Arc::new(certif)),
            signed_redacted: signed.clone(),
            signed,
        }
    },
    no_hash
);

impl TestbedEventNewSequesterService {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);

            let is_sequestered = builder
                .events
                .iter()
                .find_map(|e| match e {
                    TestbedEvent::BootstrapOrganization(x) => Some(x.sequester_authority.is_some()),
                    _ => None,
                })
                .unwrap_or(false);
            assert!(is_sequestered, "Not a sequestered organization");
        }

        // 2) Actual creation

        let (id, label, encryption_private_key, encryption_public_key) =
            builder.counters.next_sequester_service_identity();

        Self {
            timestamp: builder.counters.next_timestamp(),
            id,
            label,
            encryption_private_key,
            encryption_public_key,
            cache: Arc::default(),
        }
    }
}

impl CrcHash for TestbedEventNewSequesterService {
    fn crc_hash(&self, state: &mut crc32fast::Hasher) {
        b"TestbedEventNewSequesterService".crc_hash(state);
        self.timestamp.crc_hash(state);
        self.id.crc_hash(state);
        self.label.crc_hash(state);
        self.encryption_private_key.crc_hash(state);
        self.encryption_public_key.crc_hash(state);
    }
}

/*
 * TestbedEventRevokeSequesterService
 */

single_certificate_event!(
    TestbedEventRevokeSequesterService,
    [
        timestamp: DateTime,
        id: SequesterServiceID,
    ],
    |e: &TestbedEventRevokeSequesterService, t: &TestbedTemplate| {
        let certif = SequesterRevokedServiceCertificate {
            timestamp: e.timestamp,
            service_id: e.id,
        };
        let signed: Bytes = t
            .sequester_authority_signing_key()
            .sign(&certif.dump())
            .into();
        TestbedTemplateEventCertificate {
            certificate: AnyArcCertificate::SequesterRevokedService(Arc::new(certif)),
            signed_redacted: signed.clone(),
            signed,
        }
    },
    no_hash
);

impl TestbedEventRevokeSequesterService {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        service_id: SequesterServiceID,
    ) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);

            let is_sequestered = builder
                .events
                .iter()
                .find_map(|e| match e {
                    TestbedEvent::BootstrapOrganization(x) => Some(x.sequester_authority.is_some()),
                    _ => None,
                })
                .unwrap_or(false);
            assert!(is_sequestered, "Not a sequestered organization");

            assert!(
                builder
                    .events
                    .iter()
                    .find(
                        |e| matches!(e, TestbedEvent::NewSequesterService(x) if x.id == service_id)
                    )
                    .is_some(),
                "Sequester service does not exist"
            );
        }

        // 2) Actual creation

        Self {
            timestamp: builder.counters.next_timestamp(),
            id: service_id,
            cache: Arc::default(),
        }
    }
}

impl CrcHash for TestbedEventRevokeSequesterService {
    fn crc_hash(&self, state: &mut crc32fast::Hasher) {
        b"TestbedEventRevokeSequesterService".crc_hash(state);
        self.timestamp.crc_hash(state);
        self.id.crc_hash(state);
    }
}

/*
 * TestbedEventNewUser
 */

#[derive(Clone)]
pub struct TestbedEventNewUser {
    pub timestamp: DateTime,
    pub author: DeviceID,
    pub device_id: DeviceID,
    pub human_handle: HumanHandle,
    pub private_key: PrivateKey,
    pub first_device_label: DeviceLabel,
    pub first_device_signing_key: SigningKey,
    pub initial_profile: UserProfile,
    pub user_realm_id: VlobID,
    pub user_realm_key: SecretKey,
    pub local_symkey: SecretKey,
    pub local_password: &'static str,
    cache: Arc<Mutex<(TestbedEventCertificatesCache, TestbedEventCertificatesCache)>>,
}

impl std::fmt::Debug for TestbedEventNewUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TestbedEventNewUser")
            .field("timestamp", &self.timestamp)
            .field("author", &self.author)
            .field("device_id", &self.device_id)
            .field("initial_profile", &self.initial_profile)
            .finish()
    }
}

impl CrcHash for TestbedEventNewUser {
    fn crc_hash(&self, state: &mut crc32fast::Hasher) {
        b"NewUser".crc_hash(state);
        self.timestamp.crc_hash(state);
        self.author.crc_hash(state);
        self.device_id.crc_hash(state);
        self.human_handle.crc_hash(state);
        self.private_key.crc_hash(state);
        self.first_device_label.crc_hash(state);
        self.first_device_signing_key.crc_hash(state);
        self.initial_profile.crc_hash(state);
        self.user_realm_id.crc_hash(state);
        self.user_realm_key.crc_hash(state);
        self.local_symkey.crc_hash(state);
        self.local_password.crc_hash(state);
    }
}

impl TestbedEventNewUser {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, user_id: UserID) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            let already_exist = builder.events.iter().any(|e| match e {
                TestbedEvent::BootstrapOrganization(x)
                    if x.first_user_device_id.user_id() == &user_id =>
                {
                    true
                }
                TestbedEvent::NewUser(x) if x.device_id.user_id() == &user_id => true,
                _ => false,
            });
            assert!(!already_exist, "User already exist");
        }

        let author = utils::assert_organization_bootstrapped(&builder.events)
            .first_user_device_id
            .clone();

        // 2) Actual creation

        let human_handle = HumanHandle::new(&format!("{}@example.com", user_id.as_ref()), &{
            let mut buff = format!("{}y Mc{}Face", user_id.as_ref(), user_id.as_ref());
            let name_len = user_id.as_ref().len();
            // "alicey McaliceFace" -> "Alicey McaliceFace"
            buff[..1].make_ascii_uppercase();
            // "Alicey McaliceFace" -> "Alicey McAliceFace"
            buff[name_len + 4..name_len + 5].make_ascii_uppercase();
            buff
        })
        .unwrap();
        let device_name = "dev1".parse().unwrap();
        let device_label = "My dev1 machine".parse().unwrap();

        Self {
            timestamp: builder.counters.next_timestamp(),
            author,
            initial_profile: UserProfile::Standard,
            device_id: DeviceID::new(user_id, device_name),
            human_handle,
            private_key: builder.counters.next_private_key(),
            first_device_label: device_label,
            first_device_signing_key: builder.counters.next_signing_key(),
            user_realm_id: builder.counters.next_entry_id(),
            user_realm_key: builder.counters.next_secret_key(),
            local_symkey: builder.counters.next_secret_key(),
            local_password: "P@ssw0rd.",
            cache: Arc::default(),
        }
    }

    pub fn certificates<'a: 'c, 'b: 'c, 'c>(
        &'a self,
        template: &'b TestbedTemplate,
    ) -> impl Iterator<Item = TestbedTemplateEventCertificate> + 'c {
        (0..2).map(|i| {
            let mut guard = self.cache.lock().expect("Mutex is poisoned");
            match i {
                // User certificate
                0 => {
                    let populate = || {
                        let author_signkey = template.device_signing_key(&self.author);

                        let mut certif = UserCertificate {
                            author: CertificateSignerOwned::User(self.author.clone()),
                            timestamp: self.timestamp,
                            user_id: self.device_id.user_id().to_owned(),
                            human_handle: MaybeRedacted::Redacted(HumanHandle::new_redacted(
                                self.device_id.user_id(),
                            )),
                            public_key: self.private_key.public_key(),
                            profile: self.initial_profile,
                        };
                        let signed_redacted: Bytes = certif.dump_and_sign(author_signkey).into();
                        let signed = {
                            certif.human_handle = MaybeRedacted::Real(self.human_handle.clone());
                            certif.dump_and_sign(author_signkey).into()
                        };

                        TestbedTemplateEventCertificate {
                            certificate: AnyArcCertificate::User(Arc::new(certif)),
                            signed,
                            signed_redacted,
                        }
                    };
                    guard.0.populated(populate).to_owned()
                }

                // First device certificate
                1 => {
                    let populate = || {
                        let author_signkey = template.device_signing_key(&self.author);

                        let mut certif = DeviceCertificate {
                            author: CertificateSignerOwned::User(self.author.clone()),
                            timestamp: self.timestamp,
                            device_id: self.device_id.clone(),
                            device_label: MaybeRedacted::Redacted(DeviceLabel::new_redacted(
                                self.device_id.device_name(),
                            )),
                            verify_key: self.first_device_signing_key.verify_key(),
                        };
                        let signed_redacted: Bytes = certif.dump_and_sign(author_signkey).into();
                        let signed = {
                            certif.device_label =
                                MaybeRedacted::Real(self.first_device_label.clone());
                            certif.dump_and_sign(author_signkey).into()
                        };

                        TestbedTemplateEventCertificate {
                            certificate: AnyArcCertificate::Device(Arc::new(certif)),
                            signed,
                            signed_redacted,
                        }
                    };
                    guard.1.populated(populate).to_owned()
                }
                _ => unreachable!(),
            }
        })
    }
}

/*
 * TestbedEventNewDevice
 */

single_certificate_event!(
    TestbedEventNewDevice,
    [
        timestamp: DateTime,
        author: DeviceID,
        device_id: DeviceID,
        device_label: DeviceLabel,
        signing_key: SigningKey,
        local_symkey: SecretKey,
        local_password: &'static str,
    ],
    |e: &TestbedEventNewDevice, t: &TestbedTemplate| {
        let author_signkey = t.device_signing_key(&e.author);
        let mut certif = DeviceCertificate {
            author: CertificateSignerOwned::User(e.author.clone()),
            timestamp: e.timestamp,
            device_id: e.device_id.clone(),
            device_label: MaybeRedacted::Redacted(DeviceLabel::new_redacted(e.device_id.device_name())),
            verify_key: e.signing_key.verify_key(),
        };
        let signed_redacted: Bytes = certif.dump_and_sign(author_signkey).into();
        let signed = {
            certif.device_label = MaybeRedacted::Real(e.device_label.clone());
            certif.dump_and_sign(author_signkey).into()
        };
        TestbedTemplateEventCertificate {
            certificate: AnyArcCertificate::Device(Arc::new(certif)),
            signed,
            signed_redacted,
        }
    },
    no_hash
);

impl TestbedEventNewDevice {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, user: UserID) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
        }

        let author = match utils::assert_user_exists_and_not_revoked(&builder.events, &user) {
            TestbedEvent::BootstrapOrganization(x) => &x.first_user_device_id,
            TestbedEvent::NewUser(x) => &x.device_id,
            _ => unreachable!(),
        }
        .clone();

        let dev_index = builder.events.iter().fold(2, |c, e| match e {
            TestbedEvent::NewDevice(x) if x.device_id.user_id() == &user => c + 1,
            _ => c,
        });
        let device_name = format!("dev{}", dev_index).parse().unwrap();
        let device_label = format!("My {} machine", device_name).parse().unwrap();

        // 2) Actual creation
        let device_id = DeviceID::new(user, device_name);

        Self {
            timestamp: builder.counters.next_timestamp(),
            author,
            device_id,
            device_label,
            signing_key: builder.counters.next_signing_key(),
            local_symkey: builder.counters.next_secret_key(),
            local_password: "P@ssw0rd.",
            cache: Arc::default(),
        }
    }
}

impl CrcHash for TestbedEventNewDevice {
    fn crc_hash(&self, state: &mut crc32fast::Hasher) {
        b"TestbedEventNewDevice".crc_hash(state);
        self.timestamp.crc_hash(state);
        self.author.crc_hash(state);
        self.device_id.crc_hash(state);
        self.device_label.crc_hash(state);
        self.signing_key.crc_hash(state);
        self.local_symkey.crc_hash(state);
        self.local_password.crc_hash(state);
    }
}

/*
 * TestbedEventUpdateUserProfile
 */

single_certificate_event!(
    TestbedEventUpdateUserProfile,
    [
        timestamp: DateTime,
        author: DeviceID,
        user: UserID,
        profile: UserProfile,
    ],
    |e: &TestbedEventUpdateUserProfile, t: &TestbedTemplate| {
        let author_signkey = t.device_signing_key(&e.author);
        let certif = UserUpdateCertificate {
            author: e.author.clone(),
            timestamp: e.timestamp,
            user_id: e.user.clone(),
            new_profile: e.profile,
        };
        let signed: Bytes = certif.dump_and_sign(author_signkey).into();
        TestbedTemplateEventCertificate {
            certificate: AnyArcCertificate::UserUpdate(Arc::new(certif)),
            signed_redacted: signed.clone(),
            signed,
        }
    }
);

impl TestbedEventUpdateUserProfile {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        user: UserID,
        profile: UserProfile,
    ) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            utils::assert_user_exists_and_not_revoked(&builder.events, &user);
        }

        let author = utils::non_revoked_admins(&builder.events)
            .find(|author| author.user_id() != &user)
            .expect("Not available user to act as author (organization with a single user ?)")
            .to_owned();

        // 2) Actual creation

        Self {
            timestamp: builder.counters.next_timestamp(),
            author,
            user,
            profile,
            cache: Arc::default(),
        }
    }
}

/*
 * TestbedEventRevokeUser
 */

single_certificate_event!(
    TestbedEventRevokeUser,
    [
        timestamp: DateTime,
        author: DeviceID,
        user: UserID,
    ],
    |e: &TestbedEventRevokeUser, t: &TestbedTemplate| {
        let author_signkey = t.device_signing_key(&e.author);
        let certif = RevokedUserCertificate {
            author: e.author.clone(),
            timestamp: e.timestamp,
            user_id: e.user.clone(),
        };
        let signed: Bytes = certif.dump_and_sign(author_signkey).into();
        TestbedTemplateEventCertificate {
            certificate: AnyArcCertificate::RevokedUser(Arc::new(certif)),
            signed_redacted: signed.clone(),
            signed,
        }
    }
);

impl TestbedEventRevokeUser {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, user: UserID) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            utils::assert_user_exists_and_not_revoked(&builder.events, &user);
        }

        let author = utils::non_revoked_admins(&builder.events)
            .find(|author| author.user_id() != &user)
            .expect("Not available user to act as author (organization with a single user ?)")
            .to_owned();

        // 2) Actual creation

        Self {
            timestamp: builder.counters.next_timestamp(),
            author,
            user,
            cache: Arc::default(),
        }
    }
}

/*
 * TestbedEventNewRealm
 */

single_certificate_event!(
    TestbedEventNewRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm_id: VlobID,
    ],
    |e: &TestbedEventNewRealm, t: &TestbedTemplate| {
        let author_signkey = t.device_signing_key(&e.author);
        let certif = RealmRoleCertificate {
            author: CertificateSignerOwned::User(e.author.clone()),
            timestamp: e.timestamp,
            user_id: e.author.user_id().to_owned(),
            realm_id: e.realm_id,
            role: Some(RealmRole::Owner),
        };
        let signed: Bytes = certif.dump_and_sign(author_signkey).into();
        TestbedTemplateEventCertificate {
            certificate: AnyArcCertificate::RealmRole(Arc::new(certif)),
            signed_redacted: signed.clone(),
            signed,
        }
    }
);

impl TestbedEventNewRealm {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, first_owner: UserID) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
        }

        let author = match utils::assert_user_exists_and_not_revoked(&builder.events, &first_owner)
        {
            TestbedEvent::BootstrapOrganization(x) => &x.first_user_device_id,
            TestbedEvent::NewUser(x) => &x.device_id,
            _ => unreachable!(),
        }
        .to_owned();

        // 2) Actual creation

        Self {
            timestamp: builder.counters.next_timestamp(),
            author,
            realm_id: builder.counters.next_entry_id(),
            cache: Arc::default(),
        }
    }
}

/*
 * TestbedEventShareRealm
 */

#[derive(Clone)]
pub struct TestbedEventShareRealm {
    pub timestamp: DateTime,
    pub author: DeviceID,
    pub realm: VlobID,
    pub user: UserID,
    pub role: Option<RealmRole>,
    /// None if role is None, or if we are simulating a legacy workspace which
    /// has been shared before the initial key rotation.
    pub key_index: Option<IndexInt>,
    /// Customize only needed for testing bad key bundle access.
    /// Always None if role is None.
    pub custom_keys_bundle_access: Option<Bytes>,
    cache: Arc<
        Mutex<(
            TestbedEventCertificatesCache,
            // Encrypted keys bundle access for recipient (or None is role is None)
            TestbedEventCacheEntry<Option<Bytes>>,
        )>,
    >,
}

impl_event_debug!(
    TestbedEventShareRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        user: UserID,
        role: Option<RealmRole>,
        key_index: Option<IndexInt>,
        custom_keys_bundle_access: Option<Bytes>,
    ]
);

impl_event_crc_hash!(
    TestbedEventShareRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        user: UserID,
        role: Option<RealmRole>,
        key_index: Option<IndexInt>,
        custom_keys_bundle_access: Option<Bytes>,
    ]
);

impl TestbedEventShareRealm {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        realm: VlobID,
        user: impl TryInto<UserID>,
        role: Option<RealmRole>,
    ) -> Self {
        // 1) Consistency checks
        let user = user
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid UserID !"));

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            utils::assert_realm_exists(&builder.events, realm);
        }

        let author = utils::non_revoked_realm_owners(&builder.events, realm)
            .find(|author| author.user_id() != &user)
            .expect("No author available (realm with a single owner ?)")
            .to_owned();

        let key_index = if role.is_none() {
            None
        } else {
            let last_key_index = builder.events.iter().rev().find_map(|e| match e {
                TestbedEvent::RotateKeyRealm(x) if x.realm == realm => Some(x.key_index),
                _ => None,
            });

            match (last_key_index, builder.check_consistency) {
                (Some(last_key_index), _) => Some(last_key_index),
                (None, true) => panic!("Realm need to have a key rotation before any sharing !"),
                // Sharing before key rotation is useful to simulate the behavior of Parsec < v3.
                (None, false) => None,
            }
        };

        // 2) Actual creation

        Self {
            timestamp: builder.counters.next_timestamp(),
            author,
            realm,
            user,
            role,
            key_index,
            custom_keys_bundle_access: None,
            cache: Arc::default(),
        }
    }

    pub fn certificates<'a: 'c, 'b: 'c, 'c>(
        &'a self,
        template: &'b TestbedTemplate,
    ) -> impl Iterator<Item = TestbedTemplateEventCertificate> + 'c {
        let populate = || {
            let author_signkey = template.device_signing_key(&self.author);
            let certif = RealmRoleCertificate {
                author: CertificateSignerOwned::User(self.author.clone()),
                timestamp: self.timestamp,
                user_id: self.user.clone(),
                realm_id: self.realm,
                role: self.role,
            };
            let signed: Bytes = certif.dump_and_sign(author_signkey).into();
            TestbedTemplateEventCertificate {
                certificate: AnyArcCertificate::RealmRole(Arc::new(certif)),
                signed_redacted: signed.clone(),
                signed,
            }
        };

        std::iter::once(()).map(move |_| {
            let mut guard = self.cache.lock().expect("Mutex is poisoned");
            guard.0.populated(populate).to_owned()
        })
    }

    pub fn recipient_keys_bundle_access(&self, template: &TestbedTemplate) -> Option<Bytes> {
        let populate = || {
            if self.role.is_none() {
                return None;
            }

            if self.custom_keys_bundle_access.is_some() {
                return self.custom_keys_bundle_access.clone();
            }

            let keys_bundle = {
                template
                    .events
                    .iter()
                    .rev()
                    .find_map(|e| match e {
                        TestbedEvent::RotateKeyRealm(x) if x.realm == self.realm => {
                            Some(x.keys_bundle(&template))
                        }
                        _ => None,
                    })
                    .expect("Realm needs to have a key rotation before any sharing !")
            };

            let recipient_public_key = template.user_private_key(&self.user).public_key();

            Some(recipient_public_key.encrypt_for_self(&keys_bundle).into())
        };
        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        guard.1.populated(populate).to_owned()
    }
}

/*
 * TestbedEventRenameRealm
 */

#[derive(Clone)]
pub struct TestbedEventRenameRealm {
    pub timestamp: DateTime,
    pub author: DeviceID,
    pub realm: VlobID,
    pub name: EntryName,
    pub key_index: IndexInt,
    pub key: SecretKey,
    cache: Arc<Mutex<TestbedEventCertificatesCache>>,
}

impl_event_debug!(
    TestbedEventRenameRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        name: EntryName,
        key_index: IndexInt,
        key: SecretKey,
    ]
);
impl_event_crc_hash!(
    TestbedEventRenameRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        name: EntryName,
        key_index: IndexInt,
        key: SecretKey,
    ]
);

impl TestbedEventRenameRealm {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        realm: VlobID,
        name: impl TryInto<EntryName>,
    ) -> Self {
        // 1) Consistency checks
        let name = name
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid EntryID !"));

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            utils::assert_realm_exists(&builder.events, realm);
        }

        let author = utils::non_revoked_realm_owners(&builder.events, realm)
            .next()
            .expect("At least one owner must be present at anytime")
            .to_owned();

        let (key_index, key) = utils::realm_keys(&builder.events, realm)
            .next()
            .expect("Realm must have had at least one key rotation before rename is possible !");

        // 2) Actual creation

        Self {
            timestamp: builder.counters.next_timestamp(),
            author,
            realm,
            name,
            key: key.to_owned(),
            key_index,
            cache: Arc::default(),
        }
    }

    pub fn certificates<'a: 'c, 'b: 'c, 'c>(
        &'a self,
        template: &'b TestbedTemplate,
    ) -> impl Iterator<Item = TestbedTemplateEventCertificate> + 'c {
        let populate = || {
            let author_signkey = template.device_signing_key(&self.author);
            let encrypted_name = self.key.encrypt(self.name.as_ref().as_bytes());
            let certif = RealmNameCertificate {
                author: self.author.clone(),
                timestamp: self.timestamp,
                realm_id: self.realm,
                key_index: self.key_index,
                encrypted_name,
            };
            let signed: Bytes = certif.dump_and_sign(author_signkey).into();
            TestbedTemplateEventCertificate {
                certificate: AnyArcCertificate::RealmName(Arc::new(certif)),
                signed_redacted: signed.clone(),
                signed,
            }
        };

        std::iter::once(()).map(move |_| {
            let mut guard = self.cache.lock().expect("Mutex is poisoned");
            guard.populated(populate).to_owned()
        })
    }
}

/*
 * TestbedEventRotateKeyRealm
 */

#[derive(Clone)]
pub struct TestbedEventRotateKeyRealm {
    pub timestamp: DateTime,
    pub author: DeviceID,
    pub realm: VlobID,
    pub key_index: IndexInt,
    pub keys: Vec<SecretKey>,
    pub keys_bundle_access_key: SecretKey,
    pub encryption_algorithm: SecretKeyAlgorithm,
    pub hash_algorithm: HashAlgorithm,
    // Customize the key canary is only useful to test bad key canary
    pub custom_key_canary: Option<Vec<u8>>,
    pub participants: Vec<(UserID, PublicKey)>,
    cache: Arc<
        Mutex<(
            TestbedEventCertificatesCache,
            TestbedEventCacheEntry<Bytes>,
            TestbedEventCacheEntry<HashMap<UserID, Bytes>>,
        )>,
    >,
}

impl_event_debug!(
    TestbedEventRotateKeyRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        key_index: IndexInt,
        keys: Vec<SecretKey>,
        keys_bundle_access_key: SecretKey,
        encryption_algorithm: SecretKeyAlgorithm,
        hash_algorithm: HashAlgorithm,
        custom_key_canary: Option<Vec<u8>>,
        participants: Vec<(UserID, PublicKey)>,
    ]
);
impl_event_crc_hash!(
    TestbedEventRotateKeyRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        key_index: IndexInt,
        keys: Vec<SecretKey>,
        keys_bundle_access_key: SecretKey,
        encryption_algorithm: SecretKeyAlgorithm,
        hash_algorithm: HashAlgorithm,
        custom_key_canary: Option<Vec<u8>>,
        participants: Vec<(UserID, PublicKey)>,
    ]
);

impl TestbedEventRotateKeyRealm {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, realm: VlobID) -> Self {
        // 1) Consistency checks
        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
        }

        let author = utils::non_revoked_realm_owners(&builder.events, realm)
            .next()
            .expect("At least one owner must be present at anytime")
            .to_owned();

        // 2) Actual creation

        let participants = utils::non_revoked_realm_members(&builder.events, realm)
            .map(|(device, _)| {
                let participant_user_id = device.user_id();
                let participant_public_key = builder
                    .events
                    .iter()
                    .find_map(|e| match e {
                        TestbedEvent::BootstrapOrganization(x)
                            if x.first_user_device_id.user_id() == participant_user_id =>
                        {
                            Some(x.first_user_private_key.public_key())
                        }
                        TestbedEvent::NewUser(x)
                            if x.device_id.user_id() == participant_user_id =>
                        {
                            Some(x.private_key.public_key())
                        }
                        _ => None,
                    })
                    .expect("User must exist");
                (participant_user_id.to_owned(), participant_public_key)
            })
            .collect();

        let (key_index, keys) = builder
            .events
            .iter()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::RotateKeyRealm(x) if x.realm == realm => {
                    let mut keys = x.keys.clone();
                    keys.push(builder.counters.next_secret_key());
                    Some((x.key_index + 1, keys))
                }
                _ => None,
            })
            .unwrap_or((1, vec![builder.counters.next_secret_key()]));

        let keys_bundle_access_key = builder.counters.next_secret_key();

        Self {
            timestamp: builder.counters.next_timestamp(),
            author,
            realm,
            key_index,
            keys,
            keys_bundle_access_key,
            custom_key_canary: None,
            encryption_algorithm: SecretKeyAlgorithm::Xsalsa20Poly1305,
            hash_algorithm: HashAlgorithm::Sha256,
            participants,
            cache: Arc::default(),
        }
    }

    // We need three lifetimes here to describe the fact the output iterator
    // (lifetime 'c) wraps data from both the self object (lifetime 'a)
    // and the template (lifetime 'b). Hence 'c outliving both 'a and 'b.
    pub fn certificates<'a: 'c, 'b: 'c, 'c>(
        &'a self,
        template: &'b TestbedTemplate,
    ) -> impl Iterator<Item = TestbedTemplateEventCertificate> + 'c {
        let populate = || {
            // Note `key_canary` being the result of an encryption it is not stable
            // accross runs (as encryption involves the use of random nonce). This
            // is not much of an issue, but it means the certificate generate on
            // the test is different from the ones on the testbed server (given
            // they are two separate processes).
            let key_canary = self.custom_key_canary.clone().unwrap_or_else(|| {
                assert!(self.key_index > 0);
                self.keys[self.key_index as usize - 1].encrypt(b"")
            });
            let author_signkey = template.device_signing_key(&self.author);
            let certif = RealmKeyRotationCertificate {
                author: self.author.clone(),
                timestamp: self.timestamp,
                realm_id: self.realm,
                encryption_algorithm: self.encryption_algorithm,
                hash_algorithm: self.hash_algorithm,
                key_index: self.key_index,
                key_canary,
            };
            let signed: Bytes = certif.dump_and_sign(author_signkey).into();
            TestbedTemplateEventCertificate {
                certificate: AnyArcCertificate::RealmKeyRotation(Arc::new(certif)),
                signed_redacted: signed.clone(),
                signed,
            }
        };

        std::iter::once(()).map(move |_| {
            let mut guard = self.cache.lock().expect("Mutex is poisoned");
            guard.0.populated(populate).to_owned()
        })
    }

    pub fn keys_bundle(&self, template: &TestbedTemplate) -> Bytes {
        let populate = || {
            let bundle = RealmKeysBundle::new(
                self.author.clone(),
                self.timestamp,
                self.realm,
                self.keys.clone(),
            );
            let author_signkey = template.device_signing_key(&self.author);
            let encrypted = self
                .keys_bundle_access_key
                .encrypt(&bundle.dump_and_sign(&author_signkey));
            encrypted.into()
        };
        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        guard.1.populated(populate).to_owned()
    }

    pub fn per_participant_keys_bundle_access(&self) -> HashMap<UserID, Bytes> {
        let populate = || {
            let access = RealmKeysBundleAccess {
                keys_bundle_key: self.keys_bundle_access_key.clone(),
            }
            .dump();

            self.participants
                .iter()
                .map(|(user_id, public_key)| {
                    let encrypted = public_key.encrypt_for_self(&access);
                    (user_id.to_owned(), Bytes::from(encrypted))
                })
                .collect()
        };
        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        guard.2.populated(populate).to_owned()
    }
}

/*
 * TestbedEventArchiveRealm
 */

#[derive(Clone)]
pub struct TestbedEventArchiveRealm {
    pub timestamp: DateTime,
    pub author: DeviceID,
    pub realm: VlobID,
    pub configuration: RealmArchivingConfiguration,
    cache: Arc<Mutex<TestbedEventCertificatesCache>>,
}

impl_event_debug!(
    TestbedEventArchiveRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        configuration: RealmArchivingConfiguration,
    ]
);
impl_event_crc_hash!(
    TestbedEventArchiveRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        configuration: RealmArchivingConfiguration,
    ]
);

impl TestbedEventArchiveRealm {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        realm: VlobID,
        configuration: RealmArchivingConfiguration,
    ) -> Self {
        // 1) Consistency checks
        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
        }

        let author = utils::non_revoked_realm_owners(&builder.events, realm)
            .next()
            .expect("At least one owner must be present at anytime")
            .to_owned();

        // 2) Actual creation

        Self {
            timestamp: builder.counters.next_timestamp(),
            author,
            realm,
            configuration,
            cache: Arc::default(),
        }
    }

    // We need three lifetimes here to describe the fact the output iterator
    // (lifetime 'c) wraps data from both the self object (lifetime 'a)
    // and the template (lifetime 'b). Hence 'c outliving both 'a and 'b.
    pub fn certificates<'a: 'c, 'b: 'c, 'c>(
        &'a self,
        template: &'b TestbedTemplate,
    ) -> impl Iterator<Item = TestbedTemplateEventCertificate> + 'c {
        let populate = || {
            let author_signkey = template.device_signing_key(&self.author);
            let certif = RealmArchivingCertificate {
                author: self.author.clone(),
                timestamp: self.timestamp,
                realm_id: self.realm,
                configuration: self.configuration.clone(),
            };
            let signed: Bytes = certif.dump_and_sign(author_signkey).into();
            TestbedTemplateEventCertificate {
                certificate: AnyArcCertificate::RealmArchiving(Arc::new(certif)),
                signed_redacted: signed.clone(),
                signed,
            }
        };

        std::iter::once(()).map(move |_| {
            let mut guard = self.cache.lock().expect("Mutex is poisoned");
            guard.populated(populate).to_owned()
        })
    }
}

/*
 * TestbedEventNewShamirRecovery
 */

#[derive(Clone)]
pub struct TestbedEventNewShamirRecovery {
    pub timestamp: DateTime,
    pub author: DeviceID,
    pub threshold: NonZeroU64,
    pub per_recipient_shares: HashMap<UserID, NonZeroU64>,
    cache: Arc<Mutex<TestbedEventCacheEntry<Vec<TestbedTemplateEventCertificate>>>>,
}

impl_event_debug!(
    TestbedEventNewShamirRecovery,
    [
        timestamp: DateTime,
        author: DeviceID,
        threshold: NonZeroU64,
        per_recipient_shares: HashMap<UserID, NonZeroU64>,
    ]
);
impl_event_crc_hash!(
    TestbedEventNewShamirRecovery,
    [
        timestamp: DateTime,
        author: DeviceID,
        threshold: NonZeroU64,
        per_recipient_shares: HashMap<UserID, NonZeroU64>,
    ]
);

impl TestbedEventNewShamirRecovery {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        user: UserID,
        threshold: NonZeroU64,
        per_recipient_shares: HashMap<UserID, NonZeroU64>,
    ) -> Self {
        // 1) Consistency checks
        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
        }
        utils::assert_user_exists_and_not_revoked(&builder.events, &user);

        let author = match utils::assert_user_exists_and_not_revoked(&builder.events, &user) {
            TestbedEvent::BootstrapOrganization(x) => &x.first_user_device_id,
            TestbedEvent::NewUser(x) => &x.device_id,
            _ => unreachable!(),
        }
        .clone();

        // 2) Actual creation

        Self {
            timestamp: builder.counters.next_timestamp(),
            author,
            threshold,
            per_recipient_shares,
            cache: Arc::default(),
        }
    }

    // We need three lifetimes here to describe the fact the output iterator
    // (lifetime 'c) wraps data from both the self object (lifetime 'a)
    // and the template (lifetime 'b). Hence 'c outliving both 'a and 'b.
    pub fn certificates<'a: 'c, 'b: 'c, 'c>(
        &'a self,
        template: &'b TestbedTemplate,
    ) -> impl Iterator<Item = TestbedTemplateEventCertificate> + 'c {
        // One brief certificate + one share certificate per recipient
        let certifs = self.per_recipient_shares.len() + 1;

        (0..certifs).map(move |i| {
            let mut guard = self.cache.lock().expect("Mutex is poisoned");

            let populate = || {
                let author_signkey = template.device_signing_key(&self.author);
                let mut certifs = Vec::with_capacity(certifs);

                // Brief certificate

                let certif = ShamirRecoveryBriefCertificate {
                    author: self.author.clone(),
                    timestamp: self.timestamp,
                    threshold: self.threshold,
                    per_recipient_shares: self.per_recipient_shares.clone(),
                };
                let signed: Bytes = certif.dump_and_sign(author_signkey).into();

                let brief_certif = TestbedTemplateEventCertificate {
                    certificate: AnyArcCertificate::ShamirRecoveryBrief(Arc::new(certif)),
                    signed_redacted: signed.clone(),
                    signed,
                };
                certifs.push(brief_certif);

                // Share certificates

                for (recipient, _) in &self.per_recipient_shares {
                    // TODO: Put a real share here once shamir recovery is implemented
                    let ciphered_share = b"".to_vec();

                    let certif = ShamirRecoveryShareCertificate {
                        author: self.author.clone(),
                        timestamp: self.timestamp,
                        recipient: recipient.to_owned(),
                        ciphered_share,
                    };
                    let signed: Bytes = certif.dump_and_sign(author_signkey).into();

                    let share_certif = TestbedTemplateEventCertificate {
                        certificate: AnyArcCertificate::ShamirRecoveryShare(Arc::new(certif)),
                        signed_redacted: signed.clone(),
                        signed,
                    };
                    certifs.push(share_certif);
                }

                certifs
            };

            let certifs = guard.populated(populate);
            certifs[i].clone()
        })
    }
}

/*
 * TestbedEventNewDeviceInvitation
 */

no_certificate_event!(
    TestbedEventNewDeviceInvitation,
    [
        greeter_user_id: UserID,
        created_on: DateTime,
        token: InvitationToken,
    ]
);

impl TestbedEventNewDeviceInvitation {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        greeter_user_id: UserID,
    ) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            utils::assert_user_exists_and_not_revoked(&builder.events, &greeter_user_id);
        }

        // 2) Actual creation

        let token = builder.counters.next_invitation_token();
        Self {
            created_on: builder.counters.next_timestamp(),
            greeter_user_id,
            token,
        }
    }
}

/*
 * TestbedEventNewUserInvitation
 */

no_certificate_event!(
    TestbedEventNewUserInvitation,
    [
        greeter_user_id: UserID,
        claimer_email: String,
        created_on: DateTime,
        token: InvitationToken,
    ]
);

impl TestbedEventNewUserInvitation {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        claimer_email: String,
    ) -> Self {
        // 1) Consistency checks

        let greeter_user_id = utils::assert_organization_bootstrapped(&builder.events)
            .first_user_device_id
            .user_id()
            .to_owned();

        // 2) Actual creation

        let token = builder.counters.next_invitation_token();
        Self {
            created_on: builder.counters.next_timestamp(),
            greeter_user_id,
            claimer_email,
            token,
        }
    }
}

/*
 * TestbedEventCreateOrUpdateUserManifestVlob
 */

#[derive(Clone)]
pub struct TestbedEventCreateOrUpdateVlobCache {
    pub signed: Bytes,
    pub encrypted: Bytes,
    pub sequestered: Option<Vec<(SequesterServiceID, Bytes)>>,
}

#[derive(Clone)]
pub struct TestbedEventCreateOrUpdateUserManifestVlob {
    pub manifest: Arc<UserManifest>,
    cache: Arc<Mutex<TestbedEventCacheEntry<TestbedEventCreateOrUpdateVlobCache>>>,
}

impl_event_debug!(
    TestbedEventCreateOrUpdateUserManifestVlob,
    [manifest: Arc<UserManifest>]
);

impl CrcHash for TestbedEventCreateOrUpdateUserManifestVlob {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        "TestbedEventCreateOrUpdateUserManifestVlob".crc_hash(hasher);
        self.manifest.crc_hash(hasher);
    }
}

impl TestbedEventCreateOrUpdateUserManifestVlob {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, user: UserID) -> Self {
        let (author, id) = builder
            .events
            .iter()
            .find_map(|e| match e {
                TestbedEvent::BootstrapOrganization(x)
                    if x.first_user_device_id.user_id() == &user =>
                {
                    Some((x.first_user_device_id.clone(), x.first_user_user_realm_id))
                }
                TestbedEvent::NewUser(x) if x.device_id.user_id() == &user => {
                    Some((x.device_id.clone(), x.user_realm_id))
                }
                _ => None,
            })
            .expect("User doesn't exist");

        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            utils::assert_realm_exists(&builder.events, id);
            utils::assert_realm_member_has_write_access(&builder.events, id, &user);
        }

        // 2) Actual creation

        let version = builder
            .events
            .iter()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::CreateOrUpdateUserManifestVlob(x)
                    if x.manifest.author.user_id() == &user =>
                {
                    Some(x.manifest.version + 1)
                }
                TestbedEvent::CreateOrUpdateOpaqueVlob(x) if x.realm == id && x.vlob_id == id => {
                    Some(x.version + 1)
                }
                _ => None,
            })
            .unwrap_or_else(|| 1);

        let timestamp = builder.counters.next_timestamp();
        Self {
            manifest: Arc::new(UserManifest {
                timestamp,
                author,
                id,
                version,
                created: timestamp,
                updated: timestamp,
                workspaces_legacy_initial_info: vec![],
            }),
            cache: Arc::default(),
        }
    }

    pub fn signed(&self, template: &TestbedTemplate) -> Bytes {
        self.cache(template).signed
    }

    pub fn encrypted(&self, template: &TestbedTemplate) -> Bytes {
        self.cache(template).encrypted
    }

    pub fn sequestered(
        &self,
        template: &TestbedTemplate,
    ) -> Option<Vec<(SequesterServiceID, Bytes)>> {
        self.cache(template).sequestered
    }

    fn cache(&self, template: &TestbedTemplate) -> TestbedEventCreateOrUpdateVlobCache {
        let populate = || {
            let author_signkey = template.device_signing_key(&self.manifest.author);
            let local_symkey = template.device_local_symkey(&self.manifest.author);

            let signed: Bytes = self.manifest.dump_and_sign(author_signkey).into();
            let encrypted = local_symkey.encrypt(&signed).into();
            let sequestered = template.sequester_services_public_key().map(|iter| {
                iter.map(|(id, pubkey)| (id.to_owned(), Bytes::from(pubkey.encrypt(&signed))))
                    .collect()
            });

            TestbedEventCreateOrUpdateVlobCache {
                signed,
                encrypted,
                sequestered,
            }
        };
        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        guard.populated(populate).to_owned()
    }
}

/*
 * TestbedEventCreateOrUpdateWorkspaceManifestVlob
 */

#[derive(Clone)]
pub struct TestbedEventCreateOrUpdateWorkspaceManifestVlob {
    pub key_index: IndexInt,
    pub key: SecretKey,
    pub manifest: Arc<WorkspaceManifest>,
    cache: Arc<Mutex<TestbedEventCacheEntry<TestbedEventCreateOrUpdateVlobCache>>>,
}

impl_event_debug!(
    TestbedEventCreateOrUpdateWorkspaceManifestVlob,
    [manifest: Arc<WorkspaceManifest>]
);

impl CrcHash for TestbedEventCreateOrUpdateWorkspaceManifestVlob {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        "TestbedEventCreateOrUpdateWorkspaceManifestVlob".crc_hash(hasher);
        self.manifest.crc_hash(hasher);
    }
}

impl TestbedEventCreateOrUpdateWorkspaceManifestVlob {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        author: DeviceID,
        realm: VlobID,
    ) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            utils::assert_device_exists_and_not_revoked(&builder.events, &author);
            utils::assert_realm_exists(&builder.events, realm);
            utils::assert_realm_member_has_write_access(&builder.events, realm, author.user_id());
        }

        let (key_index, key) = utils::realm_keys(&builder.events, realm).last().expect(
            "Realm must have had at least one key rotation before vlob create/update is possible !",
        );

        // 2) Actual creation

        let id = realm;

        let (version, children) = builder
            .events
            .iter()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::CreateOrUpdateWorkspaceManifestVlob(x) if x.manifest.id == id => {
                    Some((x.manifest.version + 1, x.manifest.children.to_owned()))
                }
                TestbedEvent::CreateOrUpdateOpaqueVlob(x)
                    if x.vlob_id == id =>
                {
                    // A given VlobID can only be part of a single realm
                    assert_eq!(realm, x.realm, "VlobID {} is part of realm {}, not {}", id, x.realm, realm);
                    Some((x.version + 1, HashMap::new()))
                }
                // Try to detect common mistake in testbed env definition
                TestbedEvent::CreateOrUpdateFileManifestVlob(x)
                    if x.manifest.id == id
                => panic!("Expected vlob {} to be a workspace, but it previous version contains file manifest !", id),
                TestbedEvent::CreateOrUpdateFolderManifestVlob(x)
                    if x.manifest.id == id
                => panic!("Expected vlob {} to be a workspace, but it previous version contains folder manifest !", id),
                _ => None,
            })
            // Manifest doesn't exist yet, we create it then !
            .unwrap_or_else(|| (1, HashMap::new()));

        let timestamp = builder.counters.next_timestamp();
        Self {
            key_index,
            key: key.to_owned(),
            manifest: Arc::new(WorkspaceManifest {
                timestamp,
                author,
                id,
                version,
                created: timestamp,
                updated: timestamp,
                children,
            }),
            cache: Arc::default(),
        }
    }

    pub fn signed(&self, template: &TestbedTemplate) -> Bytes {
        self.cache(template).signed
    }

    pub fn encrypted(&self, template: &TestbedTemplate) -> Bytes {
        self.cache(template).encrypted
    }

    pub fn sequestered(
        &self,
        template: &TestbedTemplate,
    ) -> Option<Vec<(SequesterServiceID, Bytes)>> {
        self.cache(template).sequestered
    }

    fn cache(&self, template: &TestbedTemplate) -> TestbedEventCreateOrUpdateVlobCache {
        let populate = || {
            let author_signkey = template.device_signing_key(&self.manifest.author);

            let signed: Bytes = self.manifest.dump_and_sign(author_signkey).into();
            let encrypted = self.key.encrypt(&signed).into();
            let sequestered = template.sequester_services_public_key().map(|iter| {
                iter.map(|(id, pubkey)| (id.to_owned(), Bytes::from(pubkey.encrypt(&signed))))
                    .collect()
            });

            TestbedEventCreateOrUpdateVlobCache {
                signed,
                encrypted,
                sequestered,
            }
        };
        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        guard.populated(populate).to_owned()
    }
}

/*
 * TestbedEventCreateOrUpdateFolderManifestVlob
 */

#[derive(Clone)]
pub struct TestbedEventCreateOrUpdateFolderManifestVlob {
    pub realm: VlobID,
    pub key_index: IndexInt,
    pub key: SecretKey,
    pub manifest: Arc<FolderManifest>,
    cache: Arc<Mutex<TestbedEventCacheEntry<TestbedEventCreateOrUpdateVlobCache>>>,
}

impl_event_debug!(
    TestbedEventCreateOrUpdateFolderManifestVlob,
    [realm: VlobID, manifest: Arc<FolderManifest>]
);

impl CrcHash for TestbedEventCreateOrUpdateFolderManifestVlob {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        "TestbedEventCreateOrUpdateFolderManifestVlob".crc_hash(hasher);
        self.manifest.crc_hash(hasher);
    }
}

impl TestbedEventCreateOrUpdateFolderManifestVlob {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        author: DeviceID,
        realm: VlobID,
        vlob: Option<VlobID>,
    ) -> Self {
        let vlob = vlob.unwrap_or_else(|| builder.counters.next_entry_id());

        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            utils::assert_device_exists_and_not_revoked(&builder.events, &author);
            utils::assert_realm_exists(&builder.events, realm);
            utils::assert_realm_member_has_write_access(&builder.events, realm, author.user_id());
        }

        let (key_index, key) = utils::realm_keys(&builder.events, realm).last().expect(
            "Realm must have had at least one key rotation before vlob create/update is possible !",
        );

        // 2) Actual creation

        let (version, parent, children) = builder
            .events
            .iter()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::CreateOrUpdateFolderManifestVlob(x)
                    if x.manifest.id == vlob =>
                {
                    // A given VlobID can only be part of a single realm
                    assert_eq!(realm, x.realm, "VlobID {} is part of realm {}, not {}", vlob, x.realm, realm);
                    Some((
                        x.manifest.version + 1,
                        x.manifest.parent,
                        x.manifest.children.to_owned(),
                    ))
                }
                TestbedEvent::CreateOrUpdateOpaqueVlob(x)
                    if x.vlob_id == vlob =>
                {
                    // A given VlobID can only be part of a single realm
                    assert_eq!(realm, x.realm, "VlobID {} is part of realm {}, not {}", vlob, x.realm, realm);
                    // Cannot read opaque vlob, so use default values instead
                    Some((x.version + 1, realm, HashMap::new()))
                }
                // Try to detect common mistake in testbed env definition
                TestbedEvent::CreateOrUpdateFileManifestVlob(x)
                    if x.manifest.id == vlob
                => panic!("Expected vlob {} to be a folder, but it previous version contains file manifest !", vlob),
                TestbedEvent::CreateOrUpdateWorkspaceManifestVlob(x)
                    if x.manifest.id == vlob
                => panic!("Expected vlob {} to be a folder, but it previous version contains workspace manifest !", vlob),
                _ => None,
            })
            // Manifest doesn't exist yet, we create it then !
            // (note we use the workspace manifest as parent of our manifest)
            .unwrap_or_else(|| (1, realm, HashMap::new()));

        let timestamp = builder.counters.next_timestamp();
        Self {
            realm,
            key_index,
            key: key.to_owned(),
            manifest: Arc::new(FolderManifest {
                timestamp,
                author,
                id: vlob,
                parent,
                version,
                created: timestamp,
                updated: timestamp,
                children,
            }),
            cache: Arc::default(),
        }
    }

    pub fn signed(&self, template: &TestbedTemplate) -> Bytes {
        self.cache(template).signed
    }

    pub fn encrypted(&self, template: &TestbedTemplate) -> Bytes {
        self.cache(template).encrypted
    }

    pub fn sequestered(
        &self,
        template: &TestbedTemplate,
    ) -> Option<Vec<(SequesterServiceID, Bytes)>> {
        self.cache(template).sequestered
    }

    fn cache(&self, template: &TestbedTemplate) -> TestbedEventCreateOrUpdateVlobCache {
        let populate = || {
            let author_signkey = template.device_signing_key(&self.manifest.author);

            let signed: Bytes = self.manifest.dump_and_sign(author_signkey).into();
            let encrypted = self.key.encrypt(&signed).into();
            let sequestered = template.sequester_services_public_key().map(|iter| {
                iter.map(|(id, pubkey)| (id.to_owned(), Bytes::from(pubkey.encrypt(&signed))))
                    .collect()
            });

            TestbedEventCreateOrUpdateVlobCache {
                signed,
                encrypted,
                sequestered,
            }
        };
        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        guard.populated(populate).to_owned()
    }
}

/*
 * TestbedEventCreateOrUpdateFileManifestVlob
 */

#[derive(Clone)]
pub struct TestbedEventCreateOrUpdateFileManifestVlob {
    pub realm: VlobID,
    pub key_index: IndexInt,
    pub key: SecretKey,
    pub manifest: Arc<FileManifest>,
    cache: Arc<Mutex<TestbedEventCacheEntry<TestbedEventCreateOrUpdateVlobCache>>>,
}

impl_event_debug!(
    TestbedEventCreateOrUpdateFileManifestVlob,
    [realm: VlobID, manifest: Arc<FileManifest>]
);

impl CrcHash for TestbedEventCreateOrUpdateFileManifestVlob {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        "TestbedEventCreateOrUpdateFileManifestVlob".crc_hash(hasher);
        self.manifest.crc_hash(hasher);
    }
}

impl TestbedEventCreateOrUpdateFileManifestVlob {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        author: DeviceID,
        realm: VlobID,
        vlob: Option<VlobID>,
    ) -> Self {
        let vlob = vlob.unwrap_or_else(|| builder.counters.next_entry_id());

        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            utils::assert_device_exists_and_not_revoked(&builder.events, &author);
            utils::assert_realm_exists(&builder.events, realm);
            utils::assert_realm_member_has_write_access(&builder.events, realm, author.user_id());
        }

        let (key_index, key) = utils::realm_keys(&builder.events, realm).last().expect(
            "Realm must have had at least one key rotation before vlob create/update is possible !",
        );

        // 2) Actual creation

        let (version, parent, blocksize, size, blocks) = builder
            .events
            .iter()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::CreateOrUpdateFileManifestVlob(x)
                    if x.manifest.id == vlob =>
                {
                    // A given VlobID can only be part of a single realm
                    assert_eq!(realm, x.realm, "VlobID {} is part of realm {}, not {}", vlob, x.realm, realm);
                    Some((
                        x.manifest.version + 1,
                        x.manifest.parent,
                        x.manifest.blocksize,
                        x.manifest.size,
                        x.manifest.blocks.clone(),
                    ))
                }
                TestbedEvent::CreateOrUpdateOpaqueVlob(x)
                    if x.vlob_id == vlob =>
                {
                    // A given VlobID can only be part of a single realm
                    assert_eq!(realm, x.realm, "VlobID {} is part of realm {}, not {}", vlob, x.realm, realm);
                    // Cannot read opaque vlob, so use default values instead
                    Some((
                        x.version + 1,
                        realm,
                        Blocksize::try_from(512).expect("valid blocksize"),
                        0,
                        vec![],
                    ))
                }
                // Try to detect common mistake in testbed env definition
                TestbedEvent::CreateOrUpdateFolderManifestVlob(x)
                    if x.manifest.id == vlob
                => panic!("Expected vlob {} to be a file, but it previous version contains folder manifest !", vlob),
                TestbedEvent::CreateOrUpdateWorkspaceManifestVlob(x)
                    if x.manifest.id == vlob
                => panic!("Expected vlob {} to be a file, but it previous version contains workspace manifest !", vlob),
                _ => None,
            })
            // Manifest doesn't exist yet, we create it then !
            // (note we use the workspace manifest as parent of our manifest)
            .unwrap_or_else(|| {
                (
                    1,
                    realm,
                    Blocksize::try_from(512).expect("valid blocksize"),
                    0,
                    vec![],
                )
            });

        let timestamp = builder.counters.next_timestamp();
        Self {
            realm,
            key_index,
            key: key.to_owned(),
            manifest: Arc::new(FileManifest {
                timestamp,
                author,
                id: vlob,
                parent,
                version,
                created: timestamp,
                updated: timestamp,
                size,
                blocksize,
                blocks,
            }),
            cache: Arc::default(),
        }
    }

    pub fn signed(&self, template: &TestbedTemplate) -> Bytes {
        self.cache(template).signed
    }

    pub fn encrypted(&self, template: &TestbedTemplate) -> Bytes {
        self.cache(template).encrypted
    }

    pub fn sequestered(
        &self,
        template: &TestbedTemplate,
    ) -> Option<Vec<(SequesterServiceID, Bytes)>> {
        self.cache(template).sequestered
    }

    fn cache(&self, template: &TestbedTemplate) -> TestbedEventCreateOrUpdateVlobCache {
        let populate = || {
            let author_signkey = template.device_signing_key(&self.manifest.author);

            let signed: Bytes = self.manifest.dump_and_sign(author_signkey).into();
            let encrypted = self.key.encrypt(&signed).into();
            let sequestered = template.sequester_services_public_key().map(|iter| {
                iter.map(|(id, pubkey)| (id.to_owned(), Bytes::from(pubkey.encrypt(&signed))))
                    .collect()
            });

            TestbedEventCreateOrUpdateVlobCache {
                signed,
                encrypted,
                sequestered,
            }
        };
        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        guard.populated(populate).to_owned()
    }
}

/*
 * TestbedEventCreateOrUpdateOpaqueVlob
 */

no_certificate_event!(
    TestbedEventCreateOrUpdateOpaqueVlob,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        vlob_id: VlobID,
        key_index: IndexInt,
        version: VersionInt,
        signed: Bytes,
        encrypted: Bytes,
        sequestered: Option<Vec<(SequesterServiceID, Bytes)>>,
    ]
);

/*
 * TestbedEventCreateBlock
 */

#[derive(Clone)]
pub struct TestbedEventCreateBlockCache {
    pub encrypted: Bytes,
}

no_certificate_event!(
    TestbedEventCreateBlock,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        block_id: BlockID,
        block_key: SecretKey,
        cleartext: Bytes,
    ],
    cache: Arc<Mutex<TestbedEventCacheEntry<TestbedEventCreateBlockCache>>>,
);

impl TestbedEventCreateBlock {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        author: DeviceID,
        realm: VlobID,
        cleartext: Bytes,
    ) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            utils::assert_device_exists_and_not_revoked(&builder.events, &author);
            utils::assert_realm_exists(&builder.events, realm);
            utils::assert_realm_member_has_write_access(&builder.events, realm, author.user_id());
        }

        // 2) Actual creation

        let block_id = BlockID::from(*builder.counters.next_entry_id());
        let block_key = builder.counters.next_secret_key();

        let timestamp = builder.counters.next_timestamp();
        Self {
            timestamp,
            author,
            realm,
            block_id,
            block_key,
            cleartext,
            cache: Arc::default(),
        }
    }

    pub fn encrypted(&self, _template: &TestbedTemplate) -> Bytes {
        let populate = || {
            let encrypted = self.block_key.encrypt(&self.cleartext).into();
            TestbedEventCreateBlockCache { encrypted }
        };
        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        guard.populated(populate).to_owned().encrypted
    }
}

/*
 * TestbedEventCreateOpaqueBlock
 */

no_certificate_event!(
    TestbedEventCreateOpaqueBlock,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        block_id: BlockID,
        encrypted: Bytes,
    ]
);

impl TestbedEventCreateOpaqueBlock {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        author: DeviceID,
        realm: VlobID,
        block_id: BlockID,
        encrypted: Bytes,
    ) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            utils::assert_device_exists_and_not_revoked(&builder.events, &author);
            utils::assert_realm_exists(&builder.events, realm);
            utils::assert_realm_member_has_write_access(&builder.events, realm, author.user_id());
        }

        // 2) Actual creation

        let timestamp = builder.counters.next_timestamp();
        Self {
            timestamp,
            author,
            realm,
            block_id,
            encrypted,
        }
    }
}

/*
 * TestbedEventCertificatesStorageFetchCertificates
 */

no_certificate_event!(
    TestbedEventCertificatesStorageFetchCertificates,
    [device: DeviceID, up_to: DateTime]
);

impl TestbedEventCertificatesStorageFetchCertificates {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, device: DeviceID) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            // We don't care that the user is revoked here given we don't modify
            // anything in the server
            utils::assert_device_exists(&builder.events, &device);
        }

        // 2) Actual creation

        Self {
            device,
            up_to: builder.counters.current_timestamp(),
        }
    }
}

/*
 * TestbedEventUserStorageFetchUserVlob
 */

no_certificate_event!(
    TestbedEventUserStorageFetchUserVlob,
    [device: DeviceID, local_manifest: Arc<LocalUserManifest>]
);

impl TestbedEventUserStorageFetchUserVlob {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, device: DeviceID) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            // We don't care that the user is revoked here given we don't modify
            // anything in the server
            utils::assert_device_exists(&builder.events, &device);
        }

        let user_realm_id = builder
            .events
            .iter()
            .find_map(|e| match e {
                TestbedEvent::BootstrapOrganization(x)
                    if x.first_user_device_id.user_id() == device.user_id() =>
                {
                    Some(x.first_user_user_realm_id)
                }
                TestbedEvent::NewUser(x) if x.device_id.user_id() == device.user_id() => {
                    Some(x.user_realm_id)
                }
                _ => None,
            })
            .expect("User existence already checked");

        let local_manifest = builder.events.iter().rev().find_map(|e| match e {
            TestbedEvent::CreateOrUpdateUserManifestVlob(x) if x.manifest.id == user_realm_id && x.manifest.author.user_id() == device.user_id() => {
                Some(Arc::new(LocalUserManifest::from_remote((*x.manifest).clone())))
            }
            TestbedEvent::CreateOrUpdateOpaqueVlob(x) if x.realm == user_realm_id && x.vlob_id == user_realm_id => {
                panic!("Last user vlob create/update for user {} is opaque, cannot deduce what to put in the local user storage !", device.user_id());
            }
            _ => None,
        }).unwrap_or_else( || panic!("User manifest has never been synced for user {}", device.user_id()) );

        // 2) Actual creation

        Self {
            device,
            local_manifest,
        }
    }
}

/*
 * TestbedEventUserStorageFetchRealmCheckpoint
 */

no_certificate_event!(
    TestbedEventUserStorageFetchRealmCheckpoint,
    [
        device: DeviceID,
        checkpoint: IndexInt,
        remote_user_manifest_version: Option<VersionInt>,
    ]
);

impl TestbedEventUserStorageFetchRealmCheckpoint {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, device: DeviceID) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            // We don't care that the user is revoked here given we don't modify
            // anything in the server
            utils::assert_device_exists(&builder.events, &device);
        }

        let user_realm_id = builder
            .events
            .iter()
            .find_map(|e| match e {
                TestbedEvent::BootstrapOrganization(x)
                    if x.first_user_device_id.user_id() == device.user_id() =>
                {
                    Some(x.first_user_user_realm_id)
                }
                TestbedEvent::NewUser(x) if x.device_id.user_id() == device.user_id() => {
                    Some(x.user_realm_id)
                }
                _ => None,
            })
            .expect("User existence already checked");

        let mut remote_user_manifest_version = None;

        let checkpoint = builder.events.iter().fold(0, |acc, e| match e {
            TestbedEvent::CreateOrUpdateUserManifestVlob(x)
                if x.manifest.author.user_id() == device.user_id() =>
            {
                remote_user_manifest_version = Some(x.manifest.version);
                acc + 1
            }
            TestbedEvent::CreateOrUpdateOpaqueVlob(x) if x.realm == user_realm_id => {
                if x.vlob_id == user_realm_id {
                    remote_user_manifest_version = Some(x.version);
                }
                acc + 1
            }
            _ => acc,
        });

        // 2) Actual creation

        Self {
            device,
            checkpoint,
            remote_user_manifest_version,
        }
    }
}

/*
 * TestbedEventUserStorageLocalUpdate
 */

no_certificate_event!(
    TestbedEventUserStorageLocalUpdate,
    [
        timestamp: DateTime,
        device: DeviceID,
        local_manifest: Arc<LocalUserManifest>
    ]
);

impl TestbedEventUserStorageLocalUpdate {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, device: DeviceID) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            // We don't care that the user is revoked here given we don't modify
            // anything in the server
            utils::assert_device_exists(&builder.events, &device);
        }

        let timestamp = builder.counters.next_timestamp();

        let local_manifest = builder
            .events
            .iter()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::UserStorageFetchUserVlob(x) if x.device == device => {
                    Some(x.local_manifest.clone())
                }
                TestbedEvent::UserStorageLocalUpdate(x) if x.device == device => {
                    Some(x.local_manifest.clone())
                }
                _ => None,
            })
            .unwrap_or_else(|| {
                // No previous local user manifest, create one

                let user_realm_id = builder
                    .events
                    .iter()
                    .find_map(|e| match e {
                        TestbedEvent::BootstrapOrganization(x)
                            if x.first_user_device_id.user_id() == device.user_id() =>
                        {
                            Some(x.first_user_user_realm_id)
                        }
                        TestbedEvent::NewUser(x) if x.device_id.user_id() == device.user_id() => {
                            Some(x.user_realm_id)
                        }
                        _ => None,
                    })
                    .expect("User existence already checked");

                Arc::new(LocalUserManifest::new(
                    device.clone(),
                    timestamp,
                    user_realm_id.into(),
                    false,
                ))
            });

        // 2) Actual creation

        Self {
            timestamp,
            device,
            local_manifest,
        }
    }
}

/*
 * TestbedEventWorkspaceDataStorageFetchWorkspaceVlob
 */

no_certificate_event!(
    TestbedEventWorkspaceDataStorageFetchWorkspaceVlob,
    [
        device: DeviceID,
        realm: VlobID,
        local_manifest: Arc<LocalWorkspaceManifest>
    ]
);

impl TestbedEventWorkspaceDataStorageFetchWorkspaceVlob {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        device: DeviceID,
        realm: VlobID,
        prevent_sync_pattern: Option<Regex>,
    ) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            // We don't care that the user is revoked here given we don't modify
            // anything in the server
            utils::assert_device_exists(&builder.events, &device);
            utils::assert_realm_exists(&builder.events, realm);
            utils::assert_realm_member_has_read_access(&builder.events, realm, device.user_id());
        }

        let local_manifest = builder.events.iter().rev().find_map(|e| match e {
            TestbedEvent::CreateOrUpdateWorkspaceManifestVlob(x)
                if x.manifest.id == realm => {
                Some(Arc::new(LocalWorkspaceManifest::from_remote(
                    (*x.manifest).clone(),
                    prevent_sync_pattern.as_ref(),
                )))
            }
            TestbedEvent::CreateOrUpdateOpaqueVlob(x)
                if x.realm == realm && x.vlob_id == VlobID::from(*realm) => {
                panic!("Last workspace vlob create/update for realm {} is opaque, cannot deduce what to put in the local user storage !", realm);
            }
            _ => None,
        }).unwrap_or_else( || panic!("Workspace manifest has never been synced for realm {}", realm) );

        // 2) Actual creation

        Self {
            device,
            realm,
            local_manifest,
        }
    }
}

/*
 * TestbedEventWorkspaceDataStorageFetchFolderVlob
 */

no_certificate_event!(
    TestbedEventWorkspaceDataStorageFetchFolderVlob,
    [
        device: DeviceID,
        realm: VlobID,
        local_manifest: Arc<LocalFolderManifest>
    ]
);

impl TestbedEventWorkspaceDataStorageFetchFolderVlob {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        device: DeviceID,
        realm: VlobID,
        vlob: VlobID,
        prevent_sync_pattern: Option<Regex>,
    ) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            // We don't care that the user is revoked here given we don't modify
            // anything in the server
            utils::assert_device_exists(&builder.events, &device);
            utils::assert_realm_exists(&builder.events, realm);
            utils::assert_realm_member_has_read_access(&builder.events, realm, device.user_id());
        }

        let local_manifest = builder.events.iter().rev().find_map(|e| match e {
            TestbedEvent::CreateOrUpdateFolderManifestVlob(x)
                if x.realm == realm && x.manifest.id == vlob => {
                Some(Arc::new(LocalFolderManifest::from_remote(
                    (*x.manifest).clone(),
                    prevent_sync_pattern.as_ref(),
                )))
            }
            TestbedEvent::CreateOrUpdateOpaqueVlob(x)
                if x.realm == realm && x.vlob_id == vlob => {
                panic!("Last Folder vlob create/update for realm {} is opaque, cannot deduce what to put in the local user storage !", realm);
            }
            TestbedEvent::CreateOrUpdateFileManifestVlob(x)
                if x.realm == realm && x.manifest.id == vlob => {
                panic!("Try to fetch realm {} vlob {} as folder, but it is a file !", realm, vlob);
            }
            _ => None,
        }).unwrap_or_else( || panic!("Folder manifest has never been synced for realm {} vlob {}", realm, vlob) );

        // 2) Actual creation

        Self {
            device,
            realm,
            local_manifest,
        }
    }
}

/*
 * TestbedEventWorkspaceDataStorageFetchFileVlob
 */

no_certificate_event!(
    TestbedEventWorkspaceDataStorageFetchFileVlob,
    [
        device: DeviceID,
        realm: VlobID,
        local_manifest: Arc<LocalFileManifest>
    ]
);

impl TestbedEventWorkspaceDataStorageFetchFileVlob {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        device: DeviceID,
        realm: VlobID,
        vlob: VlobID,
    ) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            // We don't care that the user is revoked here given we don't modify
            // anything in the server
            utils::assert_device_exists(&builder.events, &device);
            utils::assert_realm_exists(&builder.events, realm);
            utils::assert_realm_member_has_read_access(&builder.events, realm, device.user_id());
        }

        let local_manifest = builder.events.iter().rev().find_map(|e| match e {
            TestbedEvent::CreateOrUpdateFileManifestVlob(x)
                if x.realm == realm && x.manifest.id == vlob => {
                Some(Arc::new(LocalFileManifest::from_remote(
                    (*x.manifest).clone(),
                )))
            }
            TestbedEvent::CreateOrUpdateOpaqueVlob(x)
                if x.realm == realm && x.vlob_id == vlob => {
                panic!("Last File vlob create/update for realm {} is opaque, cannot deduce what to put in the local user storage !", realm);
            }
            TestbedEvent::CreateOrUpdateFolderManifestVlob(x)
                if x.realm == realm && x.manifest.id == vlob => {
                panic!("Try to fetch realm {} vlob {} as file, but it is a folder !", realm, vlob);
            }
            _ => None,
        }).unwrap_or_else( || panic!("File manifest has never been synced for realm {} vlob {}", realm, vlob) );

        // 2) Actual creation

        Self {
            device,
            realm,
            local_manifest,
        }
    }
}

/*
 * TestbedEventWorkspaceDataStorageFetchRealmCheckpoint
 */

no_certificate_event!(
    TestbedEventWorkspaceDataStorageFetchRealmCheckpoint,
    [
        device: DeviceID,
        realm: VlobID,
        checkpoint: IndexInt,
        changed_vlobs: Vec<(VlobID, VersionInt)>,
    ]
);

impl TestbedEventWorkspaceDataStorageFetchRealmCheckpoint {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        device: DeviceID,
        realm: VlobID,
    ) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            // We don't care that the user is revoked here given we don't modify
            // anything in the server
            utils::assert_device_exists(&builder.events, &device);
            utils::assert_realm_exists(&builder.events, realm);
            utils::assert_realm_member_has_read_access(&builder.events, realm, device.user_id());
        }

        let mut changed_vlobs = vec![];
        let mut insert_change = |id, version| {
            for (cid, cv) in &mut changed_vlobs {
                if *cid == id {
                    *cv = version;
                    return;
                }
            }
            changed_vlobs.push((id, version));
        };

        let checkpoint = builder.events.iter().fold(0, |acc, e| match e {
            TestbedEvent::CreateOrUpdateWorkspaceManifestVlob(x) if x.manifest.id == realm => {
                insert_change(x.manifest.id, x.manifest.version);
                acc + 1
            }
            TestbedEvent::CreateOrUpdateFileManifestVlob(x) if x.realm == realm => {
                insert_change(x.manifest.id, x.manifest.version);
                acc + 1
            }
            TestbedEvent::CreateOrUpdateFolderManifestVlob(x) if x.realm == realm => {
                insert_change(x.manifest.id, x.manifest.version);
                acc + 1
            }
            TestbedEvent::CreateOrUpdateOpaqueVlob(x) if x.realm == realm => {
                insert_change(x.vlob_id, x.version);
                acc + 1
            }
            _ => acc,
        });

        // 2) Actual creation

        Self {
            device,
            realm,
            checkpoint,
            changed_vlobs,
        }
    }
}

/*
 * TestbedEventWorkspaceCacheStorageFetchBlock
 */

no_certificate_event!(
    TestbedEventWorkspaceCacheStorageFetchBlock,
    [
        device: DeviceID,
        realm: VlobID,
        block_id: BlockID,
        cleartext: Bytes,
    ]
);

impl TestbedEventWorkspaceCacheStorageFetchBlock {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        device: DeviceID,
        realm: VlobID,
        block: BlockID,
    ) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            // We don't care that the user is revoked here given we don't modify
            // anything in the server
            utils::assert_device_exists(&builder.events, &device);
            utils::assert_realm_exists(&builder.events, realm);
            utils::assert_realm_member_has_read_access(&builder.events, realm, device.user_id());
        }

        let cleartext = builder.events.iter().rev().find_map(|e| match e {
            TestbedEvent::CreateOpaqueBlock(x) if x.realm == realm && x.block_id == block => {
                panic!("Block {} is opaque, cannot deduce what to put in the local workspace data storage !", block);
            }
            TestbedEvent::CreateBlock(x) if x.realm == realm && x.block_id == block => {
                Some(x.cleartext.clone())
            }
            _ => None,
        }).unwrap_or_else( || panic!("Block {} doesn't exist", block));

        // 2) Actual creation

        Self {
            device,
            realm,
            block_id: block,
            cleartext,
        }
    }
}

/*
 * TestbedEventWorkspaceDataStorageLocalWorkspaceManifestUpdate
 */

no_certificate_event!(
    TestbedEventWorkspaceDataStorageLocalWorkspaceManifestUpdate,
    [
        timestamp: DateTime,
        device: DeviceID,
        realm: VlobID,
        local_manifest: Arc<LocalWorkspaceManifest>
    ]
);

impl TestbedEventWorkspaceDataStorageLocalWorkspaceManifestUpdate {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        device: DeviceID,
        realm: VlobID,
    ) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            // We don't care that the user is revoked here given we don't modify
            // anything in the server
            utils::assert_device_exists(&builder.events, &device);
            utils::assert_realm_exists(&builder.events, realm);
            // Changes are in local, so no need to check for realm read/write access
        }

        let timestamp = builder.counters.next_timestamp();

        let local_manifest = builder
            .events
            .iter()
            .rev()
            .find_map(|e| {
                match e {
                    TestbedEvent::WorkspaceDataStorageFetchWorkspaceVlob(x)
                        if x.device == device && x.realm == realm =>
                    {
                        Some(x.local_manifest.clone())
                    }
                    TestbedEvent::WorkspaceDataStorageLocalWorkspaceManifestUpdate(x)
                        if x.device == device && x.realm == realm =>
                    {
                        Some(x.local_manifest.clone())
                    }
                    _ => None,
                }
                .map(|mut manifest| {
                    let m = Arc::make_mut(&mut manifest);
                    m.updated = timestamp;
                    m.need_sync = true;
                    manifest
                })
            })
            .unwrap_or_else(|| {
                // No previous local workspace manifest, create one
                Arc::new(LocalWorkspaceManifest::new(
                    device.clone(),
                    timestamp,
                    Some(realm),
                    false,
                ))
            });

        // 2) Actual creation

        Self {
            timestamp,
            device,
            realm,
            local_manifest,
        }
    }
}

/*
 * TestbedEventWorkspaceDataStorageLocalFolderManifestCreateOrUpdate
 */

no_certificate_event!(
    TestbedEventWorkspaceDataStorageLocalFolderManifestCreateOrUpdate,
    [
        timestamp: DateTime,
        device: DeviceID,
        realm: VlobID,
        local_manifest: Arc<LocalFolderManifest>
    ]
);

impl TestbedEventWorkspaceDataStorageLocalFolderManifestCreateOrUpdate {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        device: DeviceID,
        realm: VlobID,
        vlob: Option<VlobID>,
    ) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            // We don't care that the user is revoked here given we don't modify
            // anything in the server
            utils::assert_device_exists(&builder.events, &device);
            utils::assert_realm_exists(&builder.events, realm);
            // Changes are in local, so no need to check for realm read/write access
        }

        let timestamp = builder.counters.next_timestamp();

        let local_manifest = match vlob {
            // Create new local manifest
            None => {
                let vlob = builder.counters.next_entry_id();
                let mut local_manifest = LocalFolderManifest::new(device.clone(), realm, timestamp);
                local_manifest.base.id = vlob;
                Arc::new(local_manifest)
            }
            // Update existing local manifest
            Some(vlob) => {
                let mut local_manifest = builder
                    .events
                    .iter()
                    .rev()
                    .find_map(|e| match e {
                        TestbedEvent::WorkspaceDataStorageFetchFolderVlob(x)
                            if x.device == device
                                && x.realm == realm
                                && x.local_manifest.base.id == vlob =>
                        {
                            Some(x.local_manifest.clone())
                        }
                        TestbedEvent::WorkspaceDataStorageLocalFolderManifestCreateOrUpdate(x)
                            if x.device == device
                                && x.realm == realm
                                && x.local_manifest.base.id == vlob =>
                        {
                            Some(x.local_manifest.clone())
                        }
                        _ => None,
                    })
                    .unwrap_or_else(|| {
                        panic!(
                            "Vlob {} in realm {} doesn't exist on device {} !",
                            realm, vlob, device
                        );
                    });

                let m = Arc::make_mut(&mut local_manifest);
                m.updated = timestamp;
                m.need_sync = true;

                local_manifest
            }
        };

        // 2) Actual creation

        Self {
            timestamp,
            device,
            realm,
            local_manifest,
        }
    }
}

/*
 * TestbedEventWorkspaceDataStorageLocalFileManifestCreateOrUpdate
 */

no_certificate_event!(
    TestbedEventWorkspaceDataStorageLocalFileManifestCreateOrUpdate,
    [
        timestamp: DateTime,
        device: DeviceID,
        realm: VlobID,
        local_manifest: Arc<LocalFileManifest>
    ]
);

impl TestbedEventWorkspaceDataStorageLocalFileManifestCreateOrUpdate {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        device: DeviceID,
        realm: VlobID,
        vlob: Option<VlobID>,
    ) -> Self {
        // 1) Consistency checks

        if builder.check_consistency {
            utils::assert_organization_bootstrapped(&builder.events);
            // We don't care that the user is revoked here given we don't modify
            // anything in the server
            utils::assert_device_exists(&builder.events, &device);
            utils::assert_realm_exists(&builder.events, realm);
            // Changes are in local, so no need to check for realm read/write access
        }

        let timestamp = builder.counters.next_timestamp();

        let local_manifest = match vlob {
            // Create new local manifest
            None => {
                let vlob = builder.counters.next_entry_id();
                let mut local_manifest = LocalFileManifest::new(device.clone(), realm, timestamp);
                local_manifest.base.id = vlob;
                Arc::new(local_manifest)
            }
            // Update existing local manifest
            Some(vlob) => builder
                .events
                .iter()
                .rev()
                .find_map(|e| {
                    match e {
                        TestbedEvent::WorkspaceDataStorageFetchFileVlob(x)
                            if x.device == device
                                && x.realm == realm
                                && x.local_manifest.base.id == vlob =>
                        {
                            Some(x.local_manifest.clone())
                        }
                        TestbedEvent::WorkspaceDataStorageLocalFileManifestCreateOrUpdate(x)
                            if x.device == device
                                && x.realm == realm
                                && x.local_manifest.base.id == vlob =>
                        {
                            Some(x.local_manifest.clone())
                        }
                        _ => None,
                    }
                    .map(|mut manifest| {
                        let m = Arc::make_mut(&mut manifest);
                        m.updated = timestamp;
                        m.need_sync = true;
                        manifest
                    })
                })
                .unwrap_or_else(|| {
                    panic!(
                        "Vlob {} in realm {} doesn't exist on device {} !",
                        realm, vlob, device
                    );
                }),
        };

        // 2) Actual creation

        Self {
            timestamp,
            device,
            realm,
            local_manifest,
        }
    }
}
