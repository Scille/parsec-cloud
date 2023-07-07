// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::{Arc, Mutex};

use libparsec_types::prelude::*;

use super::crc_hash::CrcHash;
use super::{utils, TestbedTemplate, TestbedTemplateBuilder};

enum TestbedEventCacheEntry<T> {
    Populated(T),
    Stalled,
}

impl<T> Default for TestbedEventCacheEntry<T> {
    fn default() -> Self {
        Self::Stalled
    }
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
    pub certificate_index: IndexInt,
    pub certificate: AnyArcCertificate,
    pub raw: Bytes,
    // `raw_redacted` is the same than `raw` if the certificate has no redacted flavour
    pub raw_redacted: Bytes,
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
                    let populate = || $populate(self, template);
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
    NewUser(TestbedEventNewUser),
    NewDevice(TestbedEventNewDevice),
    UpdateUserProfile(TestbedEventUpdateUserProfile),
    RevokeUser(TestbedEventRevokeUser),
    NewRealm(TestbedEventNewRealm),
    ShareRealm(TestbedEventShareRealm),

    // 2) Client/server interaction events not producing certificates
    StartRealmReencryption(TestbedEventStartRealmReencryption),
    FinishRealmReencryption(TestbedEventFinishRealmReencryption),
    CreateOrUpdateUserManifestVlob(TestbedEventCreateOrUpdateUserManifestVlob),
    CreateOrUpdateOpaqueVlob(TestbedEventCreateOrUpdateOpaqueVlob),
    CreateOpaqueBlock(TestbedEventCreateOpaqueBlock),

    // 3) Client-side only events
    CertificatesStorageFetchCertificates(TestbedEventCertificatesStorageFetchCertificates),
    UserStorageFetchUserVlob(TestbedEventUserStorageFetchUserVlob),
    UserStorageLocalUpdate(TestbedEventUserStorageLocalUpdate),
    WokspaceStorageFetchVlob(TestbedEventWokspaceStorageFetchVlob),
    WokspaceStorageFetchBlock(TestbedEventWokspaceStorageFetchBlock),
    WorkspaceStorageLocalUpdate(TestbedEventWorkspaceStorageLocalUpdate),
}

impl CrcHash for TestbedEvent {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        match self {
            TestbedEvent::BootstrapOrganization(x) => x.crc_hash(hasher),
            TestbedEvent::NewSequesterService(x) => x.crc_hash(hasher),
            TestbedEvent::NewUser(x) => x.crc_hash(hasher),
            TestbedEvent::NewDevice(x) => x.crc_hash(hasher),
            TestbedEvent::UpdateUserProfile(x) => x.crc_hash(hasher),
            TestbedEvent::RevokeUser(x) => x.crc_hash(hasher),
            TestbedEvent::NewRealm(x) => x.crc_hash(hasher),
            TestbedEvent::ShareRealm(x) => x.crc_hash(hasher),
            TestbedEvent::StartRealmReencryption(x) => x.crc_hash(hasher),
            TestbedEvent::FinishRealmReencryption(x) => x.crc_hash(hasher),
            TestbedEvent::CreateOrUpdateUserManifestVlob(x) => x.crc_hash(hasher),
            TestbedEvent::CreateOrUpdateOpaqueVlob(x) => x.crc_hash(hasher),
            TestbedEvent::CreateOpaqueBlock(x) => x.crc_hash(hasher),
            TestbedEvent::CertificatesStorageFetchCertificates(x) => x.crc_hash(hasher),
            TestbedEvent::UserStorageFetchUserVlob(x) => x.crc_hash(hasher),
            TestbedEvent::UserStorageLocalUpdate(x) => x.crc_hash(hasher),
            TestbedEvent::WokspaceStorageFetchVlob(x) => x.crc_hash(hasher),
            TestbedEvent::WokspaceStorageFetchBlock(x) => x.crc_hash(hasher),
            TestbedEvent::WorkspaceStorageLocalUpdate(x) => x.crc_hash(hasher),
        }
    }
}

pub enum TestbedEventCertificatesIterator<A, B, C, D, E, F, G, H>
where
    A: Iterator<Item = TestbedTemplateEventCertificate>,
    B: Iterator<Item = TestbedTemplateEventCertificate>,
    C: Iterator<Item = TestbedTemplateEventCertificate>,
    D: Iterator<Item = TestbedTemplateEventCertificate>,
    E: Iterator<Item = TestbedTemplateEventCertificate>,
    F: Iterator<Item = TestbedTemplateEventCertificate>,
    G: Iterator<Item = TestbedTemplateEventCertificate>,
    H: Iterator<Item = TestbedTemplateEventCertificate>,
{
    BootstrapOrganization(A),
    NewSequesterService(B),
    NewUser(C),
    NewDevice(D),
    UpdateUserProfile(E),
    RevokeUser(F),
    NewRealm(G),
    ShareRealm(H),
    Other,
}

impl<A, B, C, D, E, F, G, H> Iterator for TestbedEventCertificatesIterator<A, B, C, D, E, F, G, H>
where
    A: Iterator<Item = TestbedTemplateEventCertificate>,
    B: Iterator<Item = TestbedTemplateEventCertificate>,
    C: Iterator<Item = TestbedTemplateEventCertificate>,
    D: Iterator<Item = TestbedTemplateEventCertificate>,
    E: Iterator<Item = TestbedTemplateEventCertificate>,
    F: Iterator<Item = TestbedTemplateEventCertificate>,
    G: Iterator<Item = TestbedTemplateEventCertificate>,
    H: Iterator<Item = TestbedTemplateEventCertificate>,
{
    type Item = TestbedTemplateEventCertificate;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::BootstrapOrganization(iter) => iter.next(),
            Self::NewSequesterService(iter) => iter.next(),
            Self::NewUser(iter) => iter.next(),
            Self::NewDevice(iter) => iter.next(),
            Self::UpdateUserProfile(iter) => iter.next(),
            Self::RevokeUser(iter) => iter.next(),
            Self::NewRealm(iter) => iter.next(),
            Self::ShareRealm(iter) => iter.next(),
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
            _ => TestbedEventCertificatesIterator::Other,
        }
    }
}

/*
 * TestbedEventBootstrapOrganization
 */

#[derive(Clone)]
pub struct TestbedEventBootstrapOrganizationSequesterAuthority {
    pub certificate_index: IndexInt,
    pub signing_key: SequesterSigningKeyDer,
    pub verify_key: SequesterVerifyKeyDer,
}

#[derive(Clone)]
pub struct TestbedEventBootstrapOrganization {
    pub timestamp: DateTime,
    pub root_signing_key: SigningKey,
    pub sequester_authority: Option<TestbedEventBootstrapOrganizationSequesterAuthority>,
    pub first_user_certificate_index: IndexInt,
    pub first_user_device_id: DeviceID,
    pub first_user_human_handle: Option<HumanHandle>,
    pub first_user_private_key: PrivateKey,
    pub first_user_first_device_certificate_index: IndexInt,
    pub first_user_first_device_label: Option<DeviceLabel>,
    pub first_user_first_device_signing_key: SigningKey,
    pub first_user_user_manifest_id: EntryID,
    pub first_user_user_manifest_key: SecretKey,
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
            sequester_authority.certificate_index.crc_hash(state);
            // In theory signing and verify keys correspond to one another, but
            // we don't want to do such assumption when computing the CRC
            sequester_authority.signing_key.crc_hash(state);
            sequester_authority.verify_key.crc_hash(state);
        }
        self.first_user_certificate_index.crc_hash(state);
        self.first_user_device_id.crc_hash(state);
        if let Some(first_user_human_handle) = &self.first_user_human_handle {
            first_user_human_handle.crc_hash(state);
        }
        self.first_user_private_key.crc_hash(state);
        if let Some(first_user_first_device_label) = &self.first_user_first_device_label {
            first_user_first_device_label.crc_hash(state);
        }
        self.first_user_first_device_certificate_index
            .crc_hash(state);
        self.first_user_first_device_signing_key.crc_hash(state);
        self.first_user_user_manifest_id.crc_hash(state);
        self.first_user_user_manifest_key.crc_hash(state);
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

        assert!(
            builder.events.is_empty(),
            "Organization already bootstrapped !"
        );

        // 2) Actual creation

        let first_user_certificate_index = builder.counters.next_certificate_index();
        let first_user_first_device_certificate_index = builder.counters.next_certificate_index();

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
            first_user_certificate_index,
            first_user_device_id: DeviceID::new(first_user_id, device_name),
            first_user_human_handle: Some(human_handle),
            first_user_private_key: builder.counters.next_private_key(),
            first_user_first_device_certificate_index,
            first_user_first_device_label: Some(device_label),
            first_user_first_device_signing_key: builder.counters.next_signing_key(),
            first_user_user_manifest_id: builder.counters.next_entry_id(),
            first_user_user_manifest_key: builder.counters.next_secret_key(),
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
                        let raw: Bytes = certif.dump_and_sign(&self.root_signing_key).into();
                        TestbedTemplateEventCertificate {
                            certificate_index: sequester_authority.certificate_index,
                            certificate: AnyArcCertificate::SequesterAuthority(Arc::new(certif)),
                            raw_redacted: raw.clone(),
                            raw,
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
                            human_handle: None,
                            public_key: self.first_user_private_key.public_key(),
                            profile: UserProfile::Admin,
                        };
                        let raw_redacted: Bytes =
                            certif.dump_and_sign(&self.root_signing_key).into();
                        let raw = if self.first_user_human_handle.is_some() {
                            certif.human_handle = self.first_user_human_handle.to_owned();
                            certif.dump_and_sign(&self.root_signing_key).into()
                        } else {
                            raw_redacted.clone()
                        };
                        TestbedTemplateEventCertificate {
                            certificate: AnyArcCertificate::User(Arc::new(certif)),
                            certificate_index: self.first_user_certificate_index,
                            raw,
                            raw_redacted,
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
                            device_label: None,
                            verify_key: self.first_user_first_device_signing_key.verify_key(),
                        };
                        let raw_redacted: Bytes =
                            certif.dump_and_sign(&self.root_signing_key).into();
                        let raw = if self.first_user_first_device_label.is_some() {
                            certif.device_label = self.first_user_first_device_label.to_owned();
                            certif.dump_and_sign(&self.root_signing_key).into()
                        } else {
                            raw_redacted.clone()
                        };
                        TestbedTemplateEventCertificate {
                            certificate: AnyArcCertificate::Device(Arc::new(certif)),
                            certificate_index: self.first_user_first_device_certificate_index,
                            raw,
                            raw_redacted,
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
        certificate_index: IndexInt,
    ],
    |e: &TestbedEventNewSequesterService, t: &TestbedTemplate| {
        let certif = SequesterServiceCertificate {
            timestamp: e.timestamp,
            service_id: e.id,
            service_label: e.label.clone(),
            encryption_key_der: e.encryption_public_key.clone(),
        };
        let raw: Bytes = t
            .sequester_authority_signing_key()
            .sign(&certif.dump())
            .into();
        TestbedTemplateEventCertificate {
            certificate: AnyArcCertificate::SequesterService(Arc::new(certif)),
            certificate_index: e.certificate_index,
            raw_redacted: raw.clone(),
            raw,
        }
    },
    no_hash
);

impl TestbedEventNewSequesterService {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder) -> Self {
        // 1) Consistency checks

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

        // 2) Actual creation

        let (id, label, encryption_private_key, encryption_public_key) =
            builder.counters.next_sequester_service_identity();

        Self {
            timestamp: builder.counters.next_timestamp(),
            id,
            label,
            encryption_private_key,
            encryption_public_key,
            certificate_index: builder.counters.next_certificate_index(),
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
        self.certificate_index.crc_hash(state);
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
    pub human_handle: Option<HumanHandle>,
    pub private_key: PrivateKey,
    pub user_certificate_index: IndexInt,
    pub first_device_certificate_index: IndexInt,
    pub first_device_label: Option<DeviceLabel>,
    pub first_device_signing_key: SigningKey,
    pub initial_profile: UserProfile,
    pub user_manifest_id: EntryID,
    pub user_manifest_key: SecretKey,
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
        if let Some(human_handle) = &self.human_handle {
            human_handle.crc_hash(state);
        }
        self.private_key.crc_hash(state);
        self.user_certificate_index.crc_hash(state);
        self.first_device_certificate_index.crc_hash(state);
        if let Some(first_device_label) = &self.first_device_label {
            first_device_label.crc_hash(state);
        }
        self.first_device_signing_key.crc_hash(state);
        self.initial_profile.crc_hash(state);
        self.user_manifest_id.crc_hash(state);
        self.user_manifest_key.crc_hash(state);
        self.local_symkey.crc_hash(state);
        self.local_password.crc_hash(state);
    }
}

impl TestbedEventNewUser {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, user_id: UserID) -> Self {
        // 1) Consistency checks

        let author = utils::assert_organization_bootstrapped(&builder.events)
            .first_user_device_id
            .clone();
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

        // 2) Actual creation

        let user_certificate_index = builder.counters.next_certificate_index();
        let first_device_certificate_index = builder.counters.next_certificate_index();

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
            user_certificate_index,
            device_id: DeviceID::new(user_id, device_name),
            human_handle: Some(human_handle),
            private_key: builder.counters.next_private_key(),
            first_device_certificate_index,
            first_device_label: Some(device_label),
            first_device_signing_key: builder.counters.next_signing_key(),
            user_manifest_id: builder.counters.next_entry_id(),
            user_manifest_key: builder.counters.next_secret_key(),
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
                            human_handle: None,
                            public_key: self.private_key.public_key(),
                            profile: self.initial_profile,
                        };
                        let raw_redacted: Bytes = certif.dump_and_sign(author_signkey).into();
                        let raw = if self.human_handle.is_some() {
                            certif.human_handle = self.human_handle.clone();
                            certif.dump_and_sign(author_signkey).into()
                        } else {
                            raw_redacted.clone()
                        };

                        TestbedTemplateEventCertificate {
                            certificate: AnyArcCertificate::User(Arc::new(certif)),
                            certificate_index: self.user_certificate_index,
                            raw,
                            raw_redacted,
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
                            device_label: None,
                            verify_key: self.first_device_signing_key.verify_key(),
                        };
                        let raw_redacted: Bytes = certif.dump_and_sign(author_signkey).into();
                        let raw = if self.first_device_label.is_some() {
                            certif.device_label = self.first_device_label.clone();
                            certif.dump_and_sign(author_signkey).into()
                        } else {
                            raw_redacted.clone()
                        };

                        TestbedTemplateEventCertificate {
                            certificate: AnyArcCertificate::Device(Arc::new(certif)),
                            certificate_index: self.first_device_certificate_index,
                            raw,
                            raw_redacted,
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
        device_label: Option<DeviceLabel>,
        signing_key: SigningKey,
        local_symkey: SecretKey,
        local_password: &'static str,
        certificate_index: IndexInt,
    ],
    |e: &TestbedEventNewDevice, t: &TestbedTemplate| {
        let author_signkey = t.device_signing_key(&e.author);
        let mut certif = DeviceCertificate {
            author: CertificateSignerOwned::User(e.author.clone()),
            timestamp: e.timestamp,
            device_id: e.device_id.clone(),
            device_label: None,
            verify_key: e.signing_key.verify_key(),
        };
        let raw_redacted: Bytes = certif.dump_and_sign(author_signkey).into();
        let raw = if e.device_label.is_some() {
            certif.device_label = e.device_label.clone();
            certif.dump_and_sign(author_signkey).into()
        } else {
            raw_redacted.clone()
        };
        TestbedTemplateEventCertificate {
            certificate: AnyArcCertificate::Device(Arc::new(certif)),
            certificate_index: e.certificate_index,
            raw,
            raw_redacted,
        }
    },
    no_hash
);

impl TestbedEventNewDevice {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, user: UserID) -> Self {
        // 1) Consistency checks

        utils::assert_organization_bootstrapped(&builder.events);

        let author = match utils::assert_user_exists_and_not_revoked(&builder.events, &user) {
            TestbedEvent::BootstrapOrganization(x) => &x.first_user_device_id,
            TestbedEvent::NewUser(x) => &x.device_id,
            _ => unreachable!(),
        }
        .clone();
        utils::assert_user_exists_and_not_revoked(&builder.events, &user);

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
            device_label: Some(device_label),
            signing_key: builder.counters.next_signing_key(),
            local_symkey: builder.counters.next_secret_key(),
            local_password: "P@ssw0rd.",
            certificate_index: builder.counters.next_certificate_index(),
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
        self.signing_key.as_ref().crc_hash(state);
        self.local_symkey.crc_hash(state);
        self.local_password.crc_hash(state);
        self.certificate_index.crc_hash(state);
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
        certificate_index: IndexInt,
    ],
    |e: &TestbedEventUpdateUserProfile, t: &TestbedTemplate| {
        let author_signkey = t.device_signing_key(&e.author);
        let certif = UserUpdateCertificate {
            author: e.author.clone(),
            timestamp: e.timestamp,
            user_id: e.user.clone(),
            new_profile: e.profile,
        };
        let raw: Bytes = certif.dump_and_sign(author_signkey).into();
        TestbedTemplateEventCertificate {
            certificate: AnyArcCertificate::UserUpdate(Arc::new(certif)),
            certificate_index: e.certificate_index,
            raw_redacted: raw.clone(),
            raw,
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

        utils::assert_organization_bootstrapped(&builder.events);
        utils::assert_user_exists_and_not_revoked(&builder.events, &user);

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
            certificate_index: builder.counters.next_certificate_index(),
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
        certificate_index: IndexInt,
    ],
    |e: &TestbedEventRevokeUser, t: &TestbedTemplate| {
        let author_signkey = t.device_signing_key(&e.author);
        let certif = RevokedUserCertificate {
            author: e.author.clone(),
            timestamp: e.timestamp,
            user_id: e.user.clone(),
        };
        let raw: Bytes = certif.dump_and_sign(author_signkey).into();
        TestbedTemplateEventCertificate {
            certificate: AnyArcCertificate::RevokedUser(Arc::new(certif)),
            certificate_index: e.certificate_index,
            raw_redacted: raw.clone(),
            raw,
        }
    }
);

impl TestbedEventRevokeUser {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, user: UserID) -> Self {
        // 1) Consistency checks

        utils::assert_organization_bootstrapped(&builder.events);
        utils::assert_user_exists_and_not_revoked(&builder.events, &user);

        let author = utils::non_revoked_admins(&builder.events)
            .find(|author| author.user_id() != &user)
            .expect("Not available user to act as author (organization with a single user ?)")
            .to_owned();

        // 2) Actual creation

        Self {
            timestamp: builder.counters.next_timestamp(),
            author,
            user,
            certificate_index: builder.counters.next_certificate_index(),
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
        realm_id: RealmID,
        realm_key: SecretKey,
        certificate_index: IndexInt,
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
        let raw: Bytes = certif.dump_and_sign(author_signkey).into();
        TestbedTemplateEventCertificate {
            certificate: AnyArcCertificate::RealmRole(Arc::new(certif)),
            certificate_index: e.certificate_index,
            raw_redacted: raw.clone(),
            raw,
        }
    }
);

impl TestbedEventNewRealm {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, first_owner: UserID) -> Self {
        // 1) Consistency checks

        utils::assert_organization_bootstrapped(&builder.events);
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
            realm_id: builder.counters.next_entry_id().into(),
            realm_key: builder.counters.next_secret_key(),
            certificate_index: builder.counters.next_certificate_index(),
            cache: Arc::default(),
        }
    }
}

/*
 * TestbedEventShareRealm
 */

#[derive(Clone)]
pub struct TestbedEventShareRealmRecipientMessage {
    pub message: Arc<MessageContent>,
    pub raw: Bytes,
}

#[derive(Clone)]
pub struct TestbedEventShareRealm {
    pub timestamp: DateTime,
    pub author: DeviceID,
    pub realm: RealmID,
    pub user: UserID,
    pub role: Option<RealmRole>,
    pub certificate_index: IndexInt,
    pub realm_entry_name: EntryName,
    pub realm_encryption_revision: IndexInt,
    pub realm_encrypted_on: DateTime,
    pub realm_key: SecretKey,
    cache: Arc<
        Mutex<(
            TestbedEventCertificatesCache,
            TestbedEventCacheEntry<TestbedEventShareRealmRecipientMessage>,
        )>,
    >,
}

impl_event_debug!(
    TestbedEventShareRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: RealmID,
        user: UserID,
        role: Option<RealmRole>,
        certificate_index: IndexInt
    ]
);
impl_event_crc_hash!(
    TestbedEventShareRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: RealmID,
        user: UserID,
        role: Option<RealmRole>,
        certificate_index: IndexInt
    ]
);

impl TestbedEventShareRealm {
    pub(super) fn from_builder(
        builder: &mut TestbedTemplateBuilder,
        realm: RealmID,
        user: UserID,
        role: Option<RealmRole>,
    ) -> Self {
        // 1) Consistency checks

        utils::assert_organization_bootstrapped(&builder.events);
        let author = utils::non_revoked_realm_owners(&builder.events, &realm)
            .find(|author| author.user_id() != &user)
            .expect("No author available (realm with a single owner ?)")
            .to_owned();
        let (realm_encryption_revision, realm_encrypted_on, realm_key) = builder
            .events
            .iter()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::NewRealm(x) if x.realm_id == realm => {
                    Some((1, x.timestamp, x.realm_key.clone()))
                }
                TestbedEvent::StartRealmReencryption(x) if x.realm == realm => {
                    Some((x.encryption_revision, x.timestamp, x.key.clone()))
                }
                _ => None,
            })
            .expect("Realm doesn't exist");

        // 2) Actual creation

        Self {
            timestamp: builder.counters.next_timestamp(),
            author,
            realm,
            user,
            role,
            certificate_index: builder.counters.next_certificate_index(),
            realm_entry_name: "Wksp".parse().unwrap(),
            realm_encrypted_on,
            realm_encryption_revision,
            realm_key,
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
            let raw: Bytes = certif.dump_and_sign(author_signkey).into();
            TestbedTemplateEventCertificate {
                certificate: AnyArcCertificate::RealmRole(Arc::new(certif)),
                certificate_index: self.certificate_index,
                raw_redacted: raw.clone(),
                raw,
            }
        };

        std::iter::once(()).map(move |_| {
            let mut guard = self.cache.lock().expect("Mutex is poisoned");
            guard.0.populated(populate).to_owned()
        })
    }

    pub fn recipient_message(
        &self,
        template: &TestbedTemplate,
    ) -> TestbedEventShareRealmRecipientMessage {
        let populate = || {
            let message = Arc::new(MessageContent::SharingGranted {
                author: self.author.clone(),
                timestamp: self.timestamp,
                name: self.realm_entry_name.clone(),
                id: self.realm,
                encryption_revision: self.realm_encryption_revision,
                encrypted_on: self.realm_encrypted_on,
                key: self.realm_key.clone(),
            });
            let author_signkey = template.device_signing_key(&self.author);
            let recipient_pubkey = template.user_private_key(&self.user).public_key();
            let raw = message
                .dump_sign_and_encrypt_for(author_signkey, &recipient_pubkey)
                .into();
            TestbedEventShareRealmRecipientMessage { message, raw }
        };
        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        guard.1.populated(populate).to_owned()
    }
}

/*
 * TestbedEventStartRealmReencryption
 */

#[derive(Clone)]
pub struct TestbedEventStartRealmReencryptionPerParticipantMessage {
    pub items: Vec<(UserID, Arc<MessageContent>, Bytes)>,
}

no_certificate_event!(
    TestbedEventStartRealmReencryption,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: RealmID,
        entry_name: EntryName,
        encryption_revision: IndexInt,
        key: SecretKey,
    ],
    cache:
        Arc<Mutex<TestbedEventCacheEntry<TestbedEventStartRealmReencryptionPerParticipantMessage>>>
);

impl TestbedEventStartRealmReencryption {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, realm: RealmID) -> Self {
        // 1) Consistency checks

        utils::assert_organization_bootstrapped(&builder.events);
        let current_encryption_revision =
            utils::assert_realm_exists_and_not_under_reencryption(&builder.events, &realm);
        let author = utils::non_revoked_realm_owners(&builder.events, &realm)
            .next()
            .expect("At least one owner must exists")
            .to_owned();

        // 2) Actual creation

        Self {
            timestamp: builder.counters.next_timestamp(),
            author,
            realm,
            entry_name: "Wksp".parse().unwrap(),
            encryption_revision: current_encryption_revision + 1,
            key: builder.counters.next_secret_key(),
            cache: Arc::default(),
        }
    }

    pub fn per_participant_message(
        &self,
        template: &TestbedTemplate,
    ) -> TestbedEventStartRealmReencryptionPerParticipantMessage {
        let populate = || {
            let author_signkey = template.device_signing_key(&self.author);
            let items = utils::non_revoked_realm_members(&template.events, &self.realm)
                .map(|recipient| {
                    let message = Arc::new(MessageContent::SharingReencrypted {
                        author: self.author.clone(),
                        timestamp: self.timestamp,
                        name: self.entry_name.clone(),
                        id: self.realm,
                        encryption_revision: self.encryption_revision,
                        encrypted_on: self.timestamp,
                        key: self.key.clone(),
                    });

                    let recipient_pubkey =
                        template.user_private_key(recipient.user_id()).public_key();
                    let raw = message
                        .dump_sign_and_encrypt_for(author_signkey, &recipient_pubkey)
                        .into();

                    (recipient.user_id().to_owned(), message, raw)
                })
                .collect();
            TestbedEventStartRealmReencryptionPerParticipantMessage { items }
        };
        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        guard.populated(populate).to_owned()
    }
}

/*
 * TestbedEventFinishRealmReencryption
 */

no_certificate_event!(
    TestbedEventFinishRealmReencryption,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: RealmID,
        encryption_revision: IndexInt,
    ]
);

impl TestbedEventFinishRealmReencryption {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, realm: RealmID) -> Self {
        // 1) Consistency checks

        utils::assert_organization_bootstrapped(&builder.events);
        let encryption_revision =
            utils::assert_realm_exists_and_under_reencryption(&builder.events, &realm);
        let author = utils::non_revoked_realm_owners(&builder.events, &realm)
            .next()
            .expect("At least one owner must exists")
            .to_owned();

        // 2) Actual creation

        Self {
            timestamp: builder.counters.next_timestamp(),
            author,
            realm,
            encryption_revision,
        }
    }
}

/*
 * TestbedEventCreateOrUpdateUserManifestVlob
 */

#[derive(Clone)]
pub struct TestbedEventCreateOrUpdateUserManifestCacheVlob {
    pub signed: Bytes,
    pub encrypted: Bytes,
}

#[derive(Clone)]
pub struct TestbedEventCreateOrUpdateUserManifestVlob {
    pub manifest: Arc<UserManifest>,
    cache: Arc<Mutex<TestbedEventCacheEntry<TestbedEventCreateOrUpdateUserManifestCacheVlob>>>,
}

impl_event_debug!(
    TestbedEventCreateOrUpdateUserManifestVlob,
    [manifest: Arc<UserManifest>]
);

impl CrcHash for TestbedEventCreateOrUpdateUserManifestVlob {
    fn crc_hash(&self, state: &mut crc32fast::Hasher) {
        "TestbedEventCreateOrUpdateUserManifestVlob".crc_hash(state);

        // Use exhaustive destructuring here to ensure this will fail whenever
        // a new field is added to the manifest type
        let UserManifest {
            author,
            timestamp,
            id,
            version,
            created,
            updated,
            last_processed_message,
            workspaces,
        } = &*self.manifest;

        author.crc_hash(state);
        timestamp.crc_hash(state);
        id.crc_hash(state);
        version.crc_hash(state);
        created.crc_hash(state);
        updated.crc_hash(state);
        last_processed_message.crc_hash(state);

        for entry in workspaces {
            let WorkspaceEntry {
                id,
                name,
                key,
                encryption_revision,
                encrypted_on,
                legacy_role_cache_timestamp,
                legacy_role_cache_value,
            } = entry;

            id.crc_hash(state);
            name.crc_hash(state);
            key.crc_hash(state);
            encryption_revision.crc_hash(state);
            encrypted_on.crc_hash(state);
            legacy_role_cache_timestamp.crc_hash(state);
            legacy_role_cache_value.crc_hash(state);
        }
    }
}

impl TestbedEventCreateOrUpdateUserManifestVlob {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, user: UserID) -> Self {
        // 1) Consistency checks

        utils::assert_organization_bootstrapped(&builder.events);
        let (author, id) = builder
            .events
            .iter()
            .find_map(|e| match e {
                TestbedEvent::BootstrapOrganization(x)
                    if x.first_user_device_id.user_id() == &user =>
                {
                    Some((
                        x.first_user_device_id.clone(),
                        x.first_user_user_manifest_id,
                    ))
                }
                TestbedEvent::NewUser(x) if x.device_id.user_id() == &user => {
                    Some((x.device_id.clone(), x.user_manifest_id))
                }
                _ => None,
            })
            .expect("User doesn't exist");
        utils::assert_realm_exists_and_not_under_reencryption(&builder.events, &id.into());

        // 2) Actual creation

        let (version, last_processed_message, workspaces) = builder
            .events
            .iter()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::CreateOrUpdateUserManifestVlob(x)
                    if x.manifest.author.user_id() == &user =>
                {
                    Some((
                        x.manifest.version + 1,
                        x.manifest.last_processed_message,
                        x.manifest.workspaces.to_owned(),
                    ))
                }
                TestbedEvent::CreateOrUpdateOpaqueVlob(x)
                    if x.realm == id.into() && x.vlob_id == id.into() =>
                {
                    Some((x.version + 1, 0, vec![]))
                }
                _ => None,
            })
            .unwrap_or_else(|| (1, 0, vec![]));

        let timestamp = builder.counters.next_timestamp();
        Self {
            manifest: Arc::new(UserManifest {
                timestamp,
                author,
                id,
                version,
                created: timestamp,
                updated: timestamp,
                last_processed_message,
                workspaces,
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

    fn cache(&self, template: &TestbedTemplate) -> TestbedEventCreateOrUpdateUserManifestCacheVlob {
        let populate = || {
            let author_signkey = template.device_signing_key(&self.manifest.author);
            let local_symkey = template.device_local_symkey(&self.manifest.author);

            let signed: Bytes = self.manifest.dump_and_sign(author_signkey).into();
            let encrypted = local_symkey.encrypt(&signed).into();

            TestbedEventCreateOrUpdateUserManifestCacheVlob { signed, encrypted }
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
        realm: RealmID,
        encryption_revision: IndexInt,
        vlob_id: VlobID,
        version: VersionInt,
        signed: Bytes,
        encrypted: Bytes,
        sequester_blob: Option<Vec<(SequesterServiceID, Bytes)>>,
    ]
);

/*
 * TestbedEventCreateOpaqueBlock
 */

no_certificate_event!(
    TestbedEventCreateOpaqueBlock,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: RealmID,
        block_id: BlockID,
        block: Vec<u8>,
    ]
);

/*
 * TestbedEventCertificatesStorageFetchCertificates
 */

no_certificate_event!(
    TestbedEventCertificatesStorageFetchCertificates,
    [device: DeviceID, up_to_index: IndexInt,]
);

impl TestbedEventCertificatesStorageFetchCertificates {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, device: DeviceID) -> Self {
        // 1) Consistency checks

        utils::assert_organization_bootstrapped(&builder.events);
        // We don't care that the user is revoked here given we don't modify
        // anything in the server
        utils::assert_device_exists(&builder.events, &device);

        // 2) Actual creation

        Self {
            device,
            up_to_index: builder.counters.current_certificate_index(),
        }
    }
}

/*
 * TestbedEventUserStorageFetchUserVlob
 */

no_certificate_event!(
    TestbedEventUserStorageFetchUserVlob,
    [
        device: DeviceID,
        realm_checkpoint: IndexInt,
        local_user_manifest: Arc<LocalUserManifest>
    ]
);

impl TestbedEventUserStorageFetchUserVlob {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, device: DeviceID) -> Self {
        // 1) Consistency checks

        utils::assert_organization_bootstrapped(&builder.events);
        // We don't care that the user is revoked here given we don't modify
        // anything in the server
        utils::assert_device_exists(&builder.events, &device);

        let user_manifest_id = builder
            .events
            .iter()
            .find_map(|e| match e {
                TestbedEvent::BootstrapOrganization(x)
                    if x.first_user_device_id.user_id() == device.user_id() =>
                {
                    Some(x.first_user_user_manifest_id)
                }
                TestbedEvent::NewUser(x) if x.device_id.user_id() == device.user_id() => {
                    Some(x.user_manifest_id)
                }
                _ => None,
            })
            .expect("User existence already checked");

        let local_user_manifest = builder.events.iter().rev().find_map(|e| match e {
            TestbedEvent::CreateOrUpdateUserManifestVlob(x) if x.manifest.id == user_manifest_id && x.manifest.author.user_id() == device.user_id() => {
                Some(Arc::new(LocalUserManifest::from_remote((*x.manifest).clone())))
            }
            TestbedEvent::CreateOrUpdateOpaqueVlob(x) if x.realm == user_manifest_id.into() && x.vlob_id == user_manifest_id.into() => {
                panic!("Last user vlob create/update for user {} is opaque, cannot deduce what to put in the local user storage !", device.user_id());
            }
            _ => None,
        }).unwrap_or_else( || panic!("User manifest has never been synced for user {}", device.user_id()) );

        let realm_checkpoint = builder.events.iter().fold(0, |acc, e| match e {
            TestbedEvent::CreateOrUpdateUserManifestVlob(x)
                if x.manifest.author.user_id() == device.user_id() =>
            {
                acc + 1
            }
            TestbedEvent::CreateOrUpdateOpaqueVlob(x)
                if x.realm == user_manifest_id.into() && x.vlob_id == user_manifest_id.into() =>
            {
                acc + 1
            }
            _ => acc,
        });

        // 2) Actual creation

        Self {
            device,
            realm_checkpoint,
            local_user_manifest,
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
        local_user_manifest: Arc<LocalUserManifest>
    ]
);

impl TestbedEventUserStorageLocalUpdate {
    pub(super) fn from_builder(builder: &mut TestbedTemplateBuilder, device: DeviceID) -> Self {
        // 1) Consistency checks

        utils::assert_organization_bootstrapped(&builder.events);
        // We don't care that the user is revoked here given we don't modify
        // anything in the server
        utils::assert_device_exists(&builder.events, &device);

        let timestamp = builder.counters.next_timestamp();

        let local_user_manifest = builder
            .events
            .iter()
            .rev()
            .find_map(|e| {
                match e {
                    TestbedEvent::UserStorageFetchUserVlob(x) if x.device == device => {
                        Some(x.local_user_manifest.clone())
                    }
                    TestbedEvent::UserStorageLocalUpdate(x) if x.device == device => {
                        Some(x.local_user_manifest.clone())
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
                // No previous local user manifest, create one

                let user_manifest_id = builder
                    .events
                    .iter()
                    .find_map(|e| match e {
                        TestbedEvent::BootstrapOrganization(x)
                            if x.first_user_device_id.user_id() == device.user_id() =>
                        {
                            Some(x.first_user_user_manifest_id)
                        }
                        TestbedEvent::NewUser(x) if x.device_id.user_id() == device.user_id() => {
                            Some(x.user_manifest_id)
                        }
                        _ => None,
                    })
                    .expect("User existence already checked");

                Arc::new(LocalUserManifest::new(
                    device.clone(),
                    timestamp,
                    user_manifest_id.into(),
                    false,
                ))
            });

        // 2) Actual creation

        Self {
            timestamp,
            device,
            local_user_manifest,
        }
    }
}

/*
 * TestbedEventWokspaceStorageFetchVlob
 */

no_certificate_event!(
    TestbedEventWokspaceStorageFetchVlob,
    [
        device: DeviceID,
        workspace: RealmID,
        vlob: VlobID,
        version: VersionInt,
    ]
);

/*
 * TestbedEventWokspaceStorageFetchBlock
 */

no_certificate_event!(
    TestbedEventWokspaceStorageFetchBlock,
    [device: DeviceID, workspace: RealmID, block: BlockID,]
);

/*
 * TestbedEventWorkspaceStorageLocalUpdate
 */

no_certificate_event!(
    TestbedEventWorkspaceStorageLocalUpdate,
    [
        timestamp: DateTime,
        device: DeviceID,
        workspace: RealmID,
        entry: EntryID,
        encrypted: Vec<u8>,
    ]
);
