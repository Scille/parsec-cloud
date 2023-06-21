// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::{Arc, Mutex};

use libparsec_types::prelude::*;

use super::crc_hash::CrcHash;
use super::TestbedTemplate;

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
    ($struct_name:ident, [ $($field_name: ident: $field_type: ty),+ $(,)? ]) => {
        pub struct $struct_name {
            $( pub $field_name: $field_type,)*
        }
        impl $struct_name {
            #[allow(clippy::too_many_arguments)]
            pub fn new($( $field_name: $field_type),* ) -> Self {
                Self { $( $field_name, )* }
            }
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
        pub struct $struct_name {
            $( pub $field_name: $field_type,)*
            cache: Mutex<TestbedEventCertificatesCache>,
        }
        impl $struct_name {
            #[allow(clippy::too_many_arguments)]
            pub fn new($( $field_name: $field_type),* ) -> Self {
                Self {
                    $( $field_name, )*
                    cache: Mutex::default(),
                }
            }
        }
        impl_event_debug!($struct_name, [ $( $field_name: $field_type ),* ]);
        impl_certificates_meth_for_single_certificate!($struct_name, $populate);
    };
    ($struct_name:ident, [ $($field_name: ident: $field_type: ty),+ $(,)? ], $populate: expr) => {
        pub struct $struct_name {
            $( pub $field_name: $field_type,)*
            cache: Mutex<TestbedEventCertificatesCache>,
        }
        impl $struct_name {
            pub fn new($( $field_name: $field_type),* ) -> Self {
                Self {
                    $( $field_name, )*
                    cache: Mutex::default(),
                }
            }
        }
        impl_event_debug!($struct_name, [ $( $field_name: $field_type ),* ]);
        impl_event_crc_hash!($struct_name, [ $( $field_name: $field_type ),* ]);
        impl_certificates_meth_for_single_certificate!($struct_name, $populate);
    };
}

#[derive(Debug)]
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
    NewVlob(TestbedEventNewVlob),
    UpdateVlob(TestbedEventUpdateVlob),
    NewBlock(TestbedEventNewBlock),

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
            TestbedEvent::NewVlob(x) => x.crc_hash(hasher),
            TestbedEvent::UpdateVlob(x) => x.crc_hash(hasher),
            TestbedEvent::NewBlock(x) => x.crc_hash(hasher),
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

pub struct TestbedEventBootstrapOrganizationSequesterAuthority {
    pub certificate_index: IndexInt,
    pub signing_key: SequesterSigningKeyDer,
    pub verify_key: SequesterVerifyKeyDer,
}

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
    cache: Mutex<(
        TestbedEventCertificatesCache,
        TestbedEventCertificatesCache,
        TestbedEventCertificatesCache,
    )>,
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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        timestamp: DateTime,
        root_signing_key: SigningKey,
        sequester_authority: Option<TestbedEventBootstrapOrganizationSequesterAuthority>,
        first_user_certificate_index: IndexInt,
        first_user_device_id: DeviceID,
        first_user_human_handle: Option<HumanHandle>,
        first_user_private_key: PrivateKey,
        first_user_first_device_certificate_index: IndexInt,
        first_user_first_device_label: Option<DeviceLabel>,
        first_user_first_device_signing_key: SigningKey,
        first_user_user_manifest_id: EntryID,
        first_user_user_manifest_key: SecretKey,
        first_user_local_symkey: SecretKey,
        first_user_local_password: &'static str,
    ) -> Self {
        Self {
            timestamp,
            root_signing_key,
            sequester_authority,
            first_user_certificate_index,
            first_user_device_id,
            first_user_human_handle,
            first_user_private_key,
            first_user_first_device_certificate_index,
            first_user_first_device_label,
            first_user_first_device_signing_key,
            first_user_user_manifest_id,
            first_user_user_manifest_key,
            first_user_local_symkey,
            first_user_local_password,
            cache: Mutex::default(),
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
    cache: Mutex<(TestbedEventCertificatesCache, TestbedEventCertificatesCache)>,
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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        timestamp: DateTime,
        author: DeviceID,
        device_id: DeviceID,
        human_handle: Option<HumanHandle>,
        private_key: PrivateKey,
        user_certificate_index: IndexInt,
        first_device_certificate_index: IndexInt,
        first_device_label: Option<DeviceLabel>,
        first_device_signing_key: SigningKey,
        initial_profile: UserProfile,
        user_manifest_id: EntryID,
        user_manifest_key: SecretKey,
        local_symkey: SecretKey,
        local_password: &'static str,
    ) -> Self {
        Self {
            timestamp,
            author,
            device_id,
            human_handle,
            private_key,
            user_certificate_index,
            first_device_certificate_index,
            first_device_label,
            first_device_signing_key,
            initial_profile,
            user_manifest_id,
            user_manifest_key,
            local_symkey,
            local_password,
            cache: Mutex::default(),
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

single_certificate_event!(
    TestbedEventShareRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: RealmID,
        user: UserID,
        role: Option<RealmRole>,
        certificate_index: IndexInt,
        recipient_message: Option<Vec<u8>>,
    ],
    |e: &TestbedEventShareRealm, t: &TestbedTemplate| {
        let author_signkey = t.device_signing_key(&e.author);
        let certif = RealmRoleCertificate {
            author: CertificateSignerOwned::User(e.author.clone()),
            timestamp: e.timestamp,
            user_id: e.user.clone(),
            realm_id: e.realm,
            role: e.role,
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

no_certificate_event!(
    TestbedEventStartRealmReencryption,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: RealmID,
        encryption_revision: IndexInt,
        per_participant_message: Vec<(UserID, Vec<u8>)>,
    ]
);

no_certificate_event!(
    TestbedEventFinishRealmReencryption,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: RealmID,
        encryption_revision: IndexInt,
    ]
);

no_certificate_event!(
    TestbedEventNewVlob,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: RealmID,
        encryption_revision: IndexInt,
        vlob_id: VlobID,
        blob: Vec<u8>,
        sequester_blob: Option<Vec<(SequesterServiceID, Vec<u8>)>>,
    ]
);

no_certificate_event!(
    TestbedEventUpdateVlob,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: RealmID,
        encryption_revision: IndexInt,
        vlob: VlobID,
        version: VersionInt,
        blob: Vec<u8>,
        sequester_blob: Option<Vec<(SequesterServiceID, Vec<u8>)>>,
    ]
);

no_certificate_event!(
    TestbedEventNewBlock,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: RealmID,
        block_id: BlockID,
        block: Vec<u8>,
    ]
);

no_certificate_event!(
    TestbedEventCertificatesStorageFetchCertificates,
    [device: DeviceID, up_to_index: IndexInt,]
);

no_certificate_event!(
    TestbedEventUserStorageFetchUserVlob,
    [device: DeviceID, version: VersionInt,]
);

no_certificate_event!(
    TestbedEventUserStorageLocalUpdate,
    [timestamp: DateTime, device: DeviceID, encrypted: Vec<u8>,]
);

no_certificate_event!(
    TestbedEventWokspaceStorageFetchVlob,
    [
        device: DeviceID,
        workspace: RealmID,
        vlob: VlobID,
        version: VersionInt,
    ]
);

no_certificate_event!(
    TestbedEventWokspaceStorageFetchBlock,
    [device: DeviceID, workspace: RealmID, block: BlockID,]
);

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

#[derive(Clone)]
pub struct TestbedTemplateEventCertificate {
    pub certificate_index: IndexInt,
    pub certificate: AnyArcCertificate,
    pub raw: Bytes,
    // `raw_redacted` is the same than `raw` if the certificate has no redacted flavour
    pub raw_redacted: Bytes,
}

pub enum TestbedEventCertificatesCache {
    Populated(TestbedTemplateEventCertificate),
    Stalled,
}

impl Default for TestbedEventCertificatesCache {
    fn default() -> Self {
        Self::Stalled
    }
}

impl TestbedEventCertificatesCache {
    fn populated(
        &mut self,
        populate: impl FnOnce() -> TestbedTemplateEventCertificate,
    ) -> &TestbedTemplateEventCertificate {
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
