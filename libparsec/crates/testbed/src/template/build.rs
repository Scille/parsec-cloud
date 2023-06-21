// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use super::{
    TestbedEvent, TestbedEventBootstrapOrganization,
    TestbedEventBootstrapOrganizationSequesterAuthority,
    TestbedEventCertificatesStorageFetchCertificates, TestbedEventFinishRealmReencryption,
    TestbedEventNewBlock, TestbedEventNewDevice, TestbedEventNewRealm,
    TestbedEventNewSequesterService, TestbedEventNewUser, TestbedEventNewVlob,
    TestbedEventRevokeUser, TestbedEventShareRealm, TestbedEventStartRealmReencryption,
    TestbedEventUpdateUserProfile, TestbedEventUpdateVlob, TestbedEventUserStorageFetchUserVlob,
    TestbedEventUserStorageLocalUpdate, TestbedEventWokspaceStorageFetchBlock,
    TestbedEventWokspaceStorageFetchVlob, TestbedEventWorkspaceStorageLocalUpdate, TestbedTemplate,
};

macro_rules! try_into_or_die {
    ($what: tt) => {
        match $what.try_into() {
            Ok(res) => res,
            Err(_) => panic!(concat!("Invalid value for `", stringify!($what), "`")),
        }
    };
}

macro_rules! parse_or_die {
    ($what: tt) => {
        $what
            .parse()
            .expect(concat!("Invalid value for `", stringify!($what), "`"))
    };
}

pub struct TestbedTemplateBuilder<R, C: FnOnce(Arc<TestbedTemplate>) -> R> {
    t: Box<TestbedTemplate>,
    finalize: Option<C>,
}

impl<R, C: FnOnce(Arc<TestbedTemplate>) -> R> Drop for TestbedTemplateBuilder<R, C> {
    fn drop(&mut self) {
        // assert!(matches!(self.finalize, None), "`finalized()` must be called to apply the builder !")
    }
}

impl<R, C: FnOnce(Arc<TestbedTemplate>) -> R> TestbedTemplateBuilder<R, C> {
    pub fn new(id: &'static str, finalize: C) -> Self {
        Self {
            t: Box::new(TestbedTemplate::new(id)),
            finalize: Some(finalize),
        }
    }

    pub fn new_from_base(id: &'static str, base: Arc<TestbedTemplate>, finalize: C) -> Self {
        let mut t = Box::new(TestbedTemplate::new(id));
        t.base = Some(base);
        Self {
            t,
            finalize: Some(finalize),
        }
    }

    pub fn finalize(mut self) -> R {
        let cb = self.finalize.take().expect("Only used once here");
        let template = std::mem::replace(&mut self.t, Box::new(TestbedTemplate::new("dummy")));
        cb(Arc::from(template))
    }

    fn next_timestamp(&mut self) -> DateTime {
        self.t.last_timestamp += Duration::days(1);
        self.t.last_timestamp
    }

    fn assert_organization_boostrapped(&self) {
        assert!(self.t.events().len() > 0, "Bootstrap organization first !");
    }

    fn assert_device_exists<'a>(&'a self, author: &DeviceID) -> &'a TestbedEvent {
        self.assert_events_any("Device doesn't exist", |e| {
            matches!(e,
                TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization { first_user_device_id: candidate, .. }) |
                TestbedEvent::NewUser(TestbedEventNewUser { device_id: candidate, .. }) |
                TestbedEvent::NewDevice(TestbedEventNewDevice { device_id: candidate, .. })
                if candidate == author)
        })
    }

    fn assert_user_exists<'a>(&'a self, id: &UserID) -> &'a TestbedEvent {
        self.assert_events_any("User doesn't exist", |e| {
            matches!(e,
                TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization{ first_user_device_id: candidate, .. }) |
                TestbedEvent::NewUser(TestbedEventNewUser{ device_id: candidate, .. })
                if candidate.user_id() == id)
        })
    }

    fn assert_realm_exists<'a>(&'a self, realm: &RealmID) -> &'a TestbedEvent {
        self.assert_events_any("Realm doesn't exist", |e| {
            matches!(
                e,
                TestbedEvent::NewRealm(x)
                if x.realm_id == *realm)
        })
    }

    fn assert_under_reencryption(&self, realm: &RealmID) {
        let currently_reencrypting = self
            .t
            .events()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::StartRealmReencryption(x) if x.realm == *realm => Some(true),
                TestbedEvent::FinishRealmReencryption(x) if x.realm == *realm => Some(false),
                _ => None,
            })
            .unwrap_or(false);
        assert!(
            currently_reencrypting,
            "Realm not currently under reencryption"
        );
    }

    fn assert_realm_not_under_reencryption(&self, realm: &RealmID) {
        let currently_reencrypting = self
            .t
            .events()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::StartRealmReencryption(x) if x.realm == *realm => Some(true),
                TestbedEvent::FinishRealmReencryption(x) if x.realm == *realm => Some(false),
                _ => None,
            })
            .unwrap_or(false);
        assert!(
            !currently_reencrypting,
            "Realm currently under reencryption"
        );
    }

    fn assert_events_any<'a>(
        &'a self,
        msg: &'static str,
        cb: impl Fn(&TestbedEvent) -> bool,
    ) -> &'a TestbedEvent {
        let found = self.t.events().find(|e| cb(e));
        match found {
            Some(e) => e,
            None => panic!("{}", msg),
        }
    }

    fn assert_events_none(&self, msg: &'static str, cb: impl Fn(&TestbedEvent) -> bool) {
        let err = self.t.events().find(|e| cb(e));
        assert!(err.is_none(), "{}", msg);
    }

    fn _encrypt_for_sequester_services(
        &self,
        data: &[u8],
    ) -> Option<Vec<(SequesterServiceID, Vec<u8>)>> {
        match self.t.events().next() {
            None => return None,
            Some(TestbedEvent::BootstrapOrganization(x)) => {
                x.sequester_authority.as_ref()?;
            }
            Some(_) => unreachable!(),
        }
        Some(
            self.t
                .events()
                .filter_map(|e| match e {
                    TestbedEvent::NewSequesterService(x) => {
                        let encrypted = x.encryption_public_key.encrypt(data);
                        Some((x.id, encrypted))
                    }
                    _ => None,
                })
                .collect(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn bootstrap_organization(
        mut self,
        root_signing_key: impl Into<SigningKey>,
        sequester_authority: Option<(SequesterSigningKeyDer, SequesterVerifyKeyDer)>,
        first_user_device_id: impl TryInto<DeviceID>,
        first_user_human_handle: Option<impl TryInto<HumanHandle>>,
        first_user_private_key: impl Into<PrivateKey>,
        first_user_first_device_label: Option<impl TryInto<DeviceLabel>>,
        first_user_first_device_signing_key: impl Into<SigningKey>,
        first_user_user_manifest_id: impl Into<EntryID>,
        first_user_user_manifest_key: impl Into<SecretKey>,
        first_user_local_symkey: impl Into<SecretKey>,
        first_user_local_password: &'static str,
    ) -> Self {
        let root_signing_key: SigningKey = root_signing_key.into();
        let first_user_device_id = try_into_or_die!(first_user_device_id);
        let first_user_human_handle = first_user_human_handle
            .map(|first_user_human_handle| try_into_or_die!(first_user_human_handle));
        let first_user_private_key = first_user_private_key.into();
        let first_user_first_device_label = first_user_first_device_label
            .map(|first_user_first_device_label| try_into_or_die!(first_user_first_device_label));
        let first_user_first_device_signing_key = first_user_first_device_signing_key.into();
        let first_user_user_manifest_id = first_user_user_manifest_id.into();
        let first_user_user_manifest_key = first_user_user_manifest_key.into();
        let first_user_local_symkey = first_user_local_symkey.into();

        // Sanity check
        assert!(
            self.t.events.is_empty(),
            "Organization already bootstrapped !"
        );

        let sequester_authority = sequester_authority.map(|(signing_key, verify_key)| {
            self.t.last_certificate_index += 1;
            let certificate_index = self.t.last_certificate_index;
            TestbedEventBootstrapOrganizationSequesterAuthority {
                certificate_index,
                signing_key,
                verify_key,
            }
        });
        self.t.last_certificate_index += 1;
        let first_user_certificate_index = self.t.last_certificate_index;
        self.t.last_certificate_index += 1;
        let first_user_first_device_certificate_index = self.t.last_certificate_index;
        let timestamp = self.next_timestamp();
        let event = TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization::new(
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
        ));
        self.t.events.push(event);

        self
    }

    pub fn new_sequester_service(
        mut self,
        id: impl TryInto<SequesterServiceID>,
        label: &str,
        encryption_private_key: impl TryInto<SequesterPrivateKeyDer>,
        encryption_public_key: impl TryInto<SequesterPublicKeyDer>,
    ) -> Self {
        let id = try_into_or_die!(id);
        let label = parse_or_die!(label);
        let encryption_private_key = try_into_or_die!(encryption_private_key);
        let encryption_public_key = try_into_or_die!(encryption_public_key);

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_events_any(
            "Not a sequestered organization",
            |e| matches!(e, TestbedEvent::NewSequesterService(x) if x.id == id),
        );
        self.assert_events_none(
            "Sequestered service already exists",
            |e| matches!(e, TestbedEvent::NewSequesterService(x) if x.id == id),
        );

        self.t.last_certificate_index += 1;
        let certificate_index = self.t.last_certificate_index;
        let timestamp = self.next_timestamp();
        let event = TestbedEvent::NewSequesterService(TestbedEventNewSequesterService::new(
            timestamp,
            id,
            label,
            encryption_private_key,
            encryption_public_key,
            certificate_index,
        ));
        self.t.events.push(event);

        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_user(
        mut self,
        author: impl TryInto<DeviceID>,
        device_id: impl TryInto<DeviceID>,
        human_handle: Option<impl TryInto<HumanHandle>>,
        private_key: impl Into<PrivateKey>,
        first_device_label: Option<impl TryInto<DeviceLabel>>,
        first_device_signing_key: impl Into<SigningKey>,
        initial_profile: UserProfile,
        user_manifest_id: impl Into<EntryID>,
        user_manifest_key: impl Into<SecretKey>,
        local_symkey: impl Into<SecretKey>,
        local_password: &'static str,
    ) -> Self {
        let author = try_into_or_die!(author);
        let device_id: DeviceID = try_into_or_die!(device_id);
        let human_handle = human_handle.map(|human_handle| try_into_or_die!(human_handle));
        let private_key = private_key.into();
        let first_device_label =
            first_device_label.map(|first_device_label| try_into_or_die!(first_device_label));
        let first_device_signing_key = first_device_signing_key.into();
        let user_manifest_id = user_manifest_id.into();
        let user_manifest_key = user_manifest_key.into();
        let local_symkey = local_symkey.into();

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&author);
        self.assert_events_none("User already exists", |e| {
            matches!(e, TestbedEvent::NewUser(x) if x.device_id.user_id() == device_id.user_id())
        });

        self.t.last_certificate_index += 1;
        let user_certificate_index = self.t.last_certificate_index;
        self.t.last_certificate_index += 1;
        let first_device_certificate_index = self.t.last_certificate_index;
        let timestamp = self.next_timestamp();
        let event = TestbedEvent::NewUser(TestbedEventNewUser::new(
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
        ));
        self.t.events.push(event);

        self
    }

    pub fn new_device(
        mut self,
        author: impl TryInto<DeviceID>,
        device_id: impl TryInto<DeviceID>,
        device_label: Option<&str>,
        signing_key: impl Into<SigningKey>,
        local_symkey: impl Into<SecretKey>,
        local_password: &'static str,
    ) -> Self {
        let author = try_into_or_die!(author);
        let device_id = try_into_or_die!(device_id);
        let device_label = device_label.map(|device_label| parse_or_die!(device_label));
        let signing_key = signing_key.into();
        let local_symkey = local_symkey.into();

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&author);
        self.assert_events_none("Device already exists", |e| {
            matches!(
                e,
                TestbedEvent::NewUser(TestbedEventNewUser{ device_id: candidate, .. }) |
                TestbedEvent::NewDevice(TestbedEventNewDevice{ device_id: candidate, .. })
                if *candidate == device_id)
        });

        self.t.last_certificate_index += 1;
        let certificate_index = self.t.last_certificate_index;
        let timestamp = self.next_timestamp();
        let event = TestbedEvent::NewDevice(TestbedEventNewDevice::new(
            timestamp,
            author,
            device_id,
            device_label,
            signing_key,
            local_symkey,
            local_password,
            certificate_index,
        ));
        self.t.events.push(event);

        self
    }

    pub fn update_user_profile(
        mut self,
        author: impl TryInto<DeviceID>,
        user: impl TryInto<UserID>,
        profile: UserProfile,
    ) -> Self {
        let author = try_into_or_die!(author);
        let user = try_into_or_die!(user);

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&author);
        self.assert_events_none("User already exists", |e| {
            matches!(
                e,
                TestbedEvent::NewUser(x)
                if *x.device_id.user_id() == user)
        });

        self.t.last_certificate_index += 1;
        let certificate_index = self.t.last_certificate_index;
        let timestamp = self.next_timestamp();
        let event = TestbedEvent::UpdateUserProfile(TestbedEventUpdateUserProfile::new(
            timestamp,
            author,
            user,
            profile,
            certificate_index,
        ));
        self.t.events.push(event);

        self
    }

    pub fn revoke_user(
        mut self,
        author: impl TryInto<DeviceID>,
        user: impl TryInto<UserID>,
    ) -> Self {
        let author = try_into_or_die!(author);
        let user = try_into_or_die!(user);

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&author);
        self.assert_events_none("User already exists", |e| {
            matches!(
                e,
                TestbedEvent::NewUser(x)
                if *x.device_id.user_id() == user)
        });
        self.assert_events_none("User already revoked", |e| {
            matches!(
                e,
                TestbedEvent::RevokeUser(x)
                if x.user == user)
        });

        self.t.last_certificate_index += 1;
        let certificate_index = self.t.last_certificate_index;
        let timestamp = self.next_timestamp();
        let event = TestbedEvent::RevokeUser(TestbedEventRevokeUser::new(
            timestamp,
            author,
            user,
            certificate_index,
        ));
        self.t.events.push(event);

        self
    }

    pub fn new_realm(
        mut self,
        author: impl TryInto<DeviceID>,
        realm_id: impl Into<RealmID>,
        realm_key: impl Into<SecretKey>,
    ) -> Self {
        let author = try_into_or_die!(author);
        let realm_id = realm_id.into();
        let realm_key = realm_key.into();

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&author);
        self.assert_events_none("Realm already exists", |e| {
            matches!(
                e,
                TestbedEvent::NewRealm(x)
                if x.realm_id == realm_id)
        });

        self.t.last_certificate_index += 1;
        let certificate_index = self.t.last_certificate_index;
        let timestamp = self.next_timestamp();
        let event = TestbedEvent::NewRealm(TestbedEventNewRealm::new(
            timestamp,
            author,
            realm_id,
            realm_key,
            certificate_index,
        ));
        self.t.events.push(event);

        self
    }

    pub fn share_realm(
        mut self,
        author: impl TryInto<DeviceID>,
        realm: impl Into<RealmID>,
        user: impl TryInto<UserID>,
        role: Option<RealmRole>,
        recipient_message: Option<Vec<u8>>,
    ) -> Self {
        let author = try_into_or_die!(author);
        let realm = realm.into();
        let user = try_into_or_die!(user);

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&author);
        self.assert_user_exists(&user);
        self.assert_realm_exists(&realm);

        self.t.last_certificate_index += 1;
        let certificate_index = self.t.last_certificate_index;
        let timestamp = self.next_timestamp();
        let event = TestbedEvent::ShareRealm(TestbedEventShareRealm::new(
            timestamp,
            author,
            realm,
            user,
            role,
            certificate_index,
            recipient_message,
        ));
        self.t.events.push(event);

        self
    }

    /// Only update `encryption_revision`/`encrypted_on` fields, as is pointless in
    /// tests to actually re-encrypt data.
    pub fn start_realm_reencryption(
        mut self,
        author: impl TryInto<DeviceID>,
        realm: impl Into<RealmID>,
        encryption_revision: IndexInt,
        per_participant_message: impl Iterator<Item = (impl TryInto<UserID>, Vec<u8>)>,
    ) -> Self {
        let author = try_into_or_die!(author);
        let realm = realm.into();
        let per_participant_message = per_participant_message
            .map(|(user_id, msg)| {
                let user_id = try_into_or_die!(user_id);
                (user_id, msg)
            })
            .collect();

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&author);
        self.assert_events_any("Realm doesn't exist", |e| {
            matches!(
                e,
                TestbedEvent::NewRealm(x)
                if x.realm_id == realm)
        });
        self.assert_realm_not_under_reencryption(&realm);

        let timestamp = self.next_timestamp();
        let event = TestbedEvent::StartRealmReencryption(TestbedEventStartRealmReencryption::new(
            timestamp,
            author,
            realm,
            encryption_revision,
            per_participant_message,
        ));
        self.t.events.push(event);

        self
    }

    pub fn finish_realm_reencryption(
        mut self,
        author: impl TryInto<DeviceID>,
        encryption_revision: IndexInt,
        realm: impl Into<RealmID>,
    ) -> Self {
        let author = try_into_or_die!(author);
        let realm = realm.into();

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&author);
        self.assert_realm_exists(&realm);
        self.assert_under_reencryption(&realm);

        let timestamp = self.next_timestamp();
        let event = TestbedEvent::FinishRealmReencryption(
            TestbedEventFinishRealmReencryption::new(timestamp, author, realm, encryption_revision),
        );
        self.t.events.push(event);

        self
    }

    pub fn new_vlob(
        mut self,
        author: impl TryInto<DeviceID>,
        realm: impl Into<RealmID>,
        vlob_id: impl Into<VlobID>,
        blob: Vec<u8>,
        sequester_blob: Option<Vec<(SequesterServiceID, Vec<u8>)>>,
    ) -> Self {
        let author = try_into_or_die!(author);
        let realm = realm.into();
        let vlob_id = vlob_id.into();

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&author);
        self.assert_realm_exists(&realm);
        self.assert_realm_not_under_reencryption(&realm);
        self.assert_events_none("Vlob already exists", |e| {
            matches!(
                e,
                TestbedEvent::NewVlob(x)
                if x.vlob_id == vlob_id)
        });

        let encryption_revision = self
            .t
            .events()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::FinishRealmReencryption(x) if x.realm == realm => {
                    Some(x.encryption_revision)
                }
                _ => None,
            })
            .unwrap_or(1);

        let timestamp = self.next_timestamp();
        let event = TestbedEvent::NewVlob(TestbedEventNewVlob::new(
            timestamp,
            author,
            realm,
            encryption_revision,
            vlob_id,
            blob,
            sequester_blob,
        ));
        self.t.events.push(event);

        self
    }

    pub fn update_vlob(
        mut self,
        author: impl TryInto<DeviceID>,
        realm: impl Into<RealmID>,
        vlob: impl Into<VlobID>,
        blob: Vec<u8>,
        sequester_blob: Option<Vec<(SequesterServiceID, Vec<u8>)>>,
    ) -> Self {
        let author = try_into_or_die!(author);
        let realm = realm.into();
        let vlob = vlob.into();

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&author);
        self.assert_realm_exists(&realm);
        self.assert_realm_not_under_reencryption(&realm);

        let encryption_revision = self
            .t
            .events()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::FinishRealmReencryption(x) if x.realm == realm => {
                    Some(x.encryption_revision)
                }
                _ => None,
            })
            .unwrap_or(1);
        let version = self
            .t
            .events()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::NewVlob(x) if x.vlob_id == vlob => Some(1),
                TestbedEvent::UpdateVlob(x) if x.vlob == vlob => Some(x.version),
                _ => None,
            })
            .expect("Vlob doesn't exist");

        let timestamp = self.next_timestamp();
        let event = TestbedEvent::UpdateVlob(TestbedEventUpdateVlob::new(
            timestamp,
            author,
            realm,
            encryption_revision,
            vlob,
            version,
            blob,
            sequester_blob,
        ));
        self.t.events.push(event);

        self
    }

    pub fn new_block(
        mut self,
        author: impl TryInto<DeviceID>,
        realm: impl Into<RealmID>,
        block_id: impl Into<BlockID>,
        block: Vec<u8>,
    ) -> Self {
        let author = try_into_or_die!(author);
        let realm = realm.into();
        let block_id = block_id.into();

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&author);
        self.assert_realm_exists(&realm);
        self.assert_events_none("Block already exist", |e| {
            matches!(
                e,
                TestbedEvent::NewBlock(x)
                if x.block_id == block_id)
        });

        let timestamp = self.next_timestamp();
        let event = TestbedEvent::NewBlock(TestbedEventNewBlock::new(
            timestamp, author, realm, block_id, block,
        ));
        self.t.events.push(event);

        self
    }

    pub fn certificates_storage_fetch_certificates(
        mut self,
        device: impl TryInto<DeviceID>,
        up_to_index: Option<IndexInt>,
    ) -> Self {
        let device = try_into_or_die!(device);

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&device);

        let event = TestbedEvent::CertificatesStorageFetchCertificates(
            TestbedEventCertificatesStorageFetchCertificates::new(
                device,
                up_to_index.unwrap_or(self.t.last_certificate_index),
            ),
        );
        self.t.events.push(event);

        self
    }

    pub fn user_storage_fetch_user_vlob(
        mut self,
        device: impl TryInto<DeviceID>,
        version: Option<VersionInt>,
    ) -> Self {
        let device = try_into_or_die!(device);

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&device);
        let user_manifest_id = match self.assert_user_exists(device.user_id()) {
            TestbedEvent::BootstrapOrganization(x) => x.first_user_user_manifest_id,
            TestbedEvent::NewUser(x) => x.user_manifest_id,
            _ => unreachable!(),
        };
        let last_version = self
            .t
            .events()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::NewVlob(x) if x.vlob_id == user_manifest_id.into() => Some(1),
                TestbedEvent::UpdateVlob(x) if x.vlob == user_manifest_id.into() => Some(x.version),
                _ => None,
            })
            .expect("Vlob doesn't exist");

        let event = TestbedEvent::UserStorageFetchUserVlob(
            TestbedEventUserStorageFetchUserVlob::new(device, version.unwrap_or(last_version)),
        );
        self.t.events.push(event);

        self
    }

    pub fn user_storage_local_update(
        mut self,
        device: impl TryInto<DeviceID>,
        encrypted: Vec<u8>,
    ) -> Self {
        let device = try_into_or_die!(device);

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&device);

        let timestamp = self.next_timestamp();
        let event = TestbedEvent::UserStorageLocalUpdate(TestbedEventUserStorageLocalUpdate::new(
            timestamp, device, encrypted,
        ));
        self.t.events.push(event);

        self
    }

    pub fn workspace_storage_fetch_vlob(
        mut self,
        device: impl TryInto<DeviceID>,
        workspace: impl Into<RealmID>,
        vlob: impl Into<VlobID>,
        version: Option<VersionInt>,
    ) -> Self {
        let device = try_into_or_die!(device);
        let workspace = workspace.into();
        let vlob = vlob.into();

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&device);
        let last_version = self
            .t
            .events()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::NewVlob(x) if x.vlob_id == vlob => Some(1),
                TestbedEvent::UpdateVlob(x) if x.vlob == vlob => Some(x.version),
                _ => None,
            })
            .expect("Vlob doesn't exist");

        let event =
            TestbedEvent::WokspaceStorageFetchVlob(TestbedEventWokspaceStorageFetchVlob::new(
                device,
                workspace,
                vlob,
                version.unwrap_or(last_version),
            ));
        self.t.events.push(event);

        self
    }

    pub fn workspace_storage_fetch_block(
        mut self,
        device: impl TryInto<DeviceID>,
        workspace: impl Into<RealmID>,
        block: impl Into<BlockID>,
    ) -> Self {
        let device = try_into_or_die!(device);
        let workspace = workspace.into();
        let block = block.into();

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&device);
        self.assert_events_any("Block doesn't exist", |e| {
            matches!(
                e,
                TestbedEvent::NewBlock(x)
                if x.realm == workspace && x.block_id == block)
        });

        let event = TestbedEvent::WokspaceStorageFetchBlock(
            TestbedEventWokspaceStorageFetchBlock::new(device, workspace, block),
        );
        self.t.events.push(event);

        self
    }

    pub fn workspace_storage_local_update(
        mut self,
        device: impl TryInto<DeviceID>,
        workspace: impl Into<RealmID>,
        entry: impl Into<EntryID>,
        encrypted: Vec<u8>,
    ) -> Self {
        let device = try_into_or_die!(device);
        let workspace = workspace.into();
        let entry = entry.into();

        // Sanity checks
        self.assert_organization_boostrapped();
        self.assert_device_exists(&device);

        let timestamp = self.next_timestamp();
        let event = TestbedEvent::WorkspaceStorageLocalUpdate(
            TestbedEventWorkspaceStorageLocalUpdate::new(
                timestamp, device, workspace, entry, encrypted,
            ),
        );
        self.t.events.push(event);

        self
    }
}
