// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    any::Any,
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use reqwest::StatusCode;

use libparsec_types::prelude::*;

use crate::{
    test_get_template, TestbedEvent, TestbedEventBootstrapOrganization, TestbedEventNewDevice,
    TestbedEventNewUser, TestbedTemplate, TestbedTemplateBuilder,
};

// Testbed always works in memory, the path only acts as an identifier.
const TESTBED_BASE_DUMMY_PATH: &str = "/parsec/testbed/";

// Currently in use testbeds
lazy_static! {
    static ref TESTBED_ENVS: Mutex<Vec<Arc<TestbedEnv>>> = Mutex::default();
}

// HTTP client used to communicate with the testbed server
lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

/// Wow this is some seriously cursed stuff O_o'
///
/// This is a wrapper that allows to modify a value until it is sealed (after
/// that, any modification attempt will panic), while also exposing the value
/// as if it were a regular non-mutable structure.
///
/// This is "safe" as long as no reference is kept on the value before any
/// replace is done.
/// Of course this is totally unsafe for a generic usage, but it should be
/// okay for customizing the testbed template:
/// - The testbed template can only be modified in the customization step.
/// - The customization step is only allowed once, and must be done before the
///   testbed is sealed (which occurs as soon as the template is used to
///   generated extra data such as certificates).
/// - In practice this means the customization step is always the very first
///   thing done in the test.
///
/// In theory the right approach for our problem would be to put the testbed's
/// template field behind a mutex. However:
/// - This would make accessing the template significantly more cumbersome
///   (ex: it wouldn't be possible to just do `env.template.events.iter()`
///   which is extremely common).
/// - It would require modifying all existing tests :(
pub struct YoloInit<T: Sized>(std::cell::UnsafeCell<(bool, T)>);

// SAFETY: Taken from the Arc implementation
unsafe impl<T: Sized + Send> Send for YoloInit<T> {}
// SAFETY: Taken from the Arc implementation
unsafe impl<T: Sized + Send> Sync for YoloInit<T> {}

impl<T: Sized> YoloInit<T> {
    pub fn new(data: T) -> Self {
        Self(std::cell::UnsafeCell::new((false, data)))
    }

    pub fn seal(&self) {
        // SAFETY: This is safe as long as no other reference is kept on the value...
        let (seal_flag, _) = unsafe { &mut *self.0.get() };
        *seal_flag = true;
    }

    pub fn yolo_replace(&self, data: T) {
        // SAFETY: This is safe as long as no other reference is kept on the value...
        let (seal_flag, t) = unsafe { &mut *self.0.get() };
        assert!(!*seal_flag, "Already sealed !");
        *t = data;
    }
}

impl<T: Sized> std::ops::Deref for YoloInit<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: Once sealed, this is perfectly safe. Before that this is only
        // safe if `YoloInit::yolo_replace` is currently called.
        // On top of that, extra care should be paid to discard the reference as
        // soon as possible if the seal hasn't been done yet.
        let (_, t) = unsafe { &*self.0.get() };
        t
    }
}

#[derive(Default)]
struct TestbedEnvCache {
    organization_addr: Option<Arc<ParsecOrganizationAddr>>,
    // Testbed is designed to run entirely in memory (i.e. with no FS accesses)
    //
    // This hashmap should be used by components (e.g. device storage, local database)
    // to keep track of arbitrary data that would otherwise be persisted on FS.
    // To avoid clash between components, key should be their name (e.g. "local_database").
    //
    // Note the idea is for the component to be as lazy as possible: they don't have to
    // store/initialize anything until they are actually asked to do something on a
    // given config dir, at which point they should use `test_get_testbed` and
    // initialize `per_component_store` by using the data from `template`.
    per_component_store: Vec<(&'static str, Arc<dyn Any + Send + Sync>)>,
    // Must keep track of the generated local devices to avoid nasty behavior
    // if the test call generate the local device multiple times for the same device.
    local_devices: Vec<Arc<LocalDevice>>,
}

impl TestbedEnvCache {
    pub fn local_device(&mut self, env: &TestbedEnv, device_id: DeviceID) -> Arc<LocalDevice> {
        for item in self.local_devices.iter() {
            if item.device_id == device_id {
                return item.clone();
            }
        }

        // Not found, must generate it

        let (user_id, device_label, signing_key, local_symkey) =
            match env.template.device_creation_event(device_id) {
                TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                    first_user_id: user_id,
                    first_user_first_device_label: device_label,
                    first_user_first_device_signing_key: signing_key,
                    first_user_local_symkey: local_symkey,
                    ..
                })
                | TestbedEvent::NewUser(TestbedEventNewUser {
                    user_id,
                    first_device_label: device_label,
                    first_device_signing_key: signing_key,
                    local_symkey,
                    ..
                })
                | TestbedEvent::NewDevice(TestbedEventNewDevice {
                    user_id,
                    device_label,
                    signing_key,
                    local_symkey,
                    ..
                }) => (*user_id, device_label, signing_key, local_symkey),
                _ => unreachable!(),
            };

        let (human_handle, private_key, profile, user_realm_id, user_realm_key) =
            match env.template.user_creation_event(user_id) {
                TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                    first_user_human_handle: human_handle,
                    first_user_private_key: private_key,
                    first_user_user_realm_id: user_realm_id,
                    first_user_user_realm_key: user_realm_key,
                    ..
                }) => (
                    human_handle,
                    private_key,
                    UserProfile::Admin,
                    user_realm_id,
                    user_realm_key,
                ),
                TestbedEvent::NewUser(TestbedEventNewUser {
                    human_handle,
                    private_key,
                    initial_profile,
                    user_realm_id,
                    user_realm_key,
                    ..
                }) => (
                    human_handle,
                    private_key,
                    *initial_profile,
                    user_realm_id,
                    user_realm_key,
                ),
                _ => unreachable!(),
            };

        let local_device = Arc::new(LocalDevice {
            organization_addr: (*self.organization_addr(env)).clone(),
            user_id,
            device_id,
            device_label: device_label.to_owned(),
            human_handle: human_handle.to_owned(),
            signing_key: signing_key.to_owned(),
            private_key: private_key.to_owned(),
            initial_profile: profile,
            user_realm_id: user_realm_id.to_owned(),
            user_realm_key: user_realm_key.to_owned(),
            local_symkey: local_symkey.to_owned(),
            time_provider: TimeProvider::default(),
        });

        self.local_devices.push(local_device.clone());
        local_device
    }

    pub fn organization_addr(&mut self, env: &TestbedEnv) -> Arc<ParsecOrganizationAddr> {
        self.organization_addr
            .get_or_insert_with(|| {
                let root_signing_key = env.template.root_signing_key();
                let organization_addr = ParsecOrganizationAddr::new(
                    env.server_addr.clone(),
                    env.organization_id.clone(),
                    root_signing_key.verify_key(),
                );
                Arc::new(organization_addr)
            })
            .clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TestbedKind {
    ClientServer,
    ClientOnly, // Server is considered offline for the whole run
}

/// Most of the time you want to customize your environment for your test,
/// hence you can use the methods `customize` or `customize_with_map` if you
/// need to return anything.
///
/// See [`TestbedEvent`] for more information about the possible events.
pub struct TestbedEnv {
    pub kind: TestbedKind,
    /// Fake path used used as the key identifying the testbed env.
    /// In Parsec there is two base directory: `config_dir` and `data_base_dir`
    /// when the testbed is used, both should be set to the discriminant value.
    /// Note: testbed is designed to always run mocked in memory, so this path
    /// will never exist on the file system.
    pub discriminant_dir: PathBuf,
    pub server_addr: ParsecAddr,
    pub organization_id: OrganizationID,
    pub template: YoloInit<Arc<TestbedTemplate>>,

    cache: Mutex<Option<TestbedEnvCache>>,
}

impl std::fmt::Debug for TestbedEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut ds = f.debug_struct("TestbedEnv");

        ds.field("template", &self.template.id)
            .field("kind", &self.kind)
            .field("discriminant_dir", &self.discriminant_dir);

        if let TestbedKind::ClientServer = self.kind {
            ds.field("server_addr", &self.server_addr)
                .field("organization_id", &self.organization_id);
        }

        ds.finish()
    }
}

impl TestbedEnv {
    /// Env sealing is done lazily when data are first accessed
    fn seal_and<R>(&self, cb: impl FnOnce(&mut TestbedEnvCache) -> R) -> R {
        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        let cache = guard.get_or_insert_with(TestbedEnvCache::default);
        self.template.seal();
        cb(cache)
    }

    pub fn is_sealed(&self) -> bool {
        let guard = self.cache.lock().expect("Mutex is poisoned");
        guard.is_some()
    }

    /// Customize the env testbed
    ///
    /// ```
    /// let realm_id = env.customize(|builder| {
    ///   // Add any events you want
    ///   builder.new_user("bob");
    ///   let realm_id = builder.new_user_realm("bob").map(|x| x.id);
    ///   // Clear client storages (certificate, user, workspace data&cache)
    ///   builder.filter_client_storage_events(|_event| true);
    ///   realm_id
    /// }).await;
    /// ```
    ///
    /// Only one call to `customize` is allowed per testbed env, it should be done
    /// before the env is sealed (i.e. before any non-template data is accessed).
    ///
    /// ⚠️ This method contains some cursed dark magic to patch the template while it is
    /// not behind a mutex, so don't do anything too concurrent before calling it !
    pub async fn customize<T>(&self, cb: impl FnOnce(&mut TestbedTemplateBuilder) -> T) -> T {
        let mut builder = TestbedTemplateBuilder::new_from_template("custom", &self.template);
        let ret = cb(&mut builder);

        // Retrieve current testbed env in the global store...
        let env = {
            let mut envs = TESTBED_ENVS.lock().expect("Mutex is poisoned");
            envs.iter_mut()
                .find(|env| env.discriminant_dir == self.discriminant_dir)
                .expect("Must exist")
                .clone()
        };

        // ...and patch it with the new template
        assert!(
            !env.is_sealed(),
            "Testbed template cannot be customized once the env is sealed"
        );
        let new_template = builder.finalize();
        // This is the cursed part where we patch `env.template` while it is not behind a mutex !
        // This is "fine" as long as customization occurs early in the test while there is no
        // concurrent operation occurring...
        env.template.yolo_replace(new_template);

        // If the testbed has been customized, we now have to transmit the extra events
        // to the server.
        if matches!(self.kind, TestbedKind::ClientServer) && !self.template.events.is_empty() {
            let url = self.server_addr.to_http_url(Some(&format!(
                "/testbed/customize/{}",
                self.organization_id.as_ref()
            )));
            let customization =
                rmp_serde::to_vec(self.template.custom_events()).expect("Cannot serialize events");
            let response = HTTP_CLIENT
                .post(url)
                .body(customization)
                .send()
                .await
                .unwrap_or_else(|e| panic!("Cannot communicate with testbed server: {}", e));
            if response.status() != StatusCode::OK {
                panic!("Bad response status from testbed server: {:?}", response);
            }
        }

        // Finally force seal the env, this is to avoid multiple customization (as it is
        // exotic stuff and a single customization should be enough anyway).
        env.seal_and(|_| ());

        ret
    }

    pub fn organization_addr(&self) -> Arc<ParsecOrganizationAddr> {
        self.seal_and(|cache| cache.organization_addr(self))
    }

    pub fn local_device(&self, device_id: impl TryInto<DeviceID>) -> Arc<LocalDevice> {
        let device_id = device_id
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid DeviceID !"));
        self.seal_and(|cache| cache.local_device(self, device_id))
    }

    pub fn get_last_realm_keys_bundle(&self, realm_id: VlobID) -> Bytes {
        let event = self
            .template
            .events
            .iter()
            .rev()
            .find_map(|event| match event {
                TestbedEvent::RotateKeyRealm(x) if x.realm == realm_id => Some(x),
                _ => None,
            })
            .expect("Realm has had no key rotation");
        event.keys_bundle(&self.template)
    }

    pub fn get_last_realm_keys_bundle_index(&self, realm_id: VlobID) -> IndexInt {
        self.template
            .events
            .iter()
            .rev()
            .find_map(|event| match event {
                TestbedEvent::RotateKeyRealm(x) if x.realm == realm_id => Some(x.key_index),
                _ => None,
            })
            .expect("Realm has had no key rotation")
    }

    pub fn get_last_realm_keys_bundle_access_for(&self, realm_id: VlobID, user: UserID) -> Bytes {
        self.template
            .events
            .iter()
            .rev()
            .find_map(|event| match event {
                TestbedEvent::RotateKeyRealm(x) if x.realm == realm_id => Some(
                    x.per_participant_keys_bundle_access()
                        .remove(&user)
                        .expect("No keys bundle access for user"),
                ),
                TestbedEvent::ShareRealm(x) if x.realm == realm_id && x.user == user => Some(
                    x.recipient_keys_bundle_access(&self.template)
                        .expect("No keys bundle access for user"),
                ),
                _ => None,
            })
            .expect("Realm has had no key rotation involving this user")
    }

    pub fn get_realm_keys_bundle(&self, realm_id: VlobID, key_index: IndexInt) -> Bytes {
        let event = self
            .template
            .events
            .iter()
            .rev()
            .find_map(|event| match event {
                TestbedEvent::RotateKeyRealm(x)
                    if x.realm == realm_id && x.key_index == key_index =>
                {
                    Some(x)
                }
                _ => None,
            })
            .expect("Realm has had no key rotation with this key index");
        event.keys_bundle(&self.template)
    }

    pub fn get_keys_bundle_access_for(
        &self,
        realm_id: VlobID,
        user: UserID,
        key_index: IndexInt,
    ) -> Bytes {
        self.template
            .events
            .iter()
            .rev()
            .find_map(|event| match event {
                TestbedEvent::RotateKeyRealm(x)
                    if x.realm == realm_id && x.key_index == key_index =>
                {
                    Some(
                        x.per_participant_keys_bundle_access()
                            .remove(&user)
                            .expect("No keys bundle access for user"),
                    )
                }
                TestbedEvent::ShareRealm(x)
                    if x.realm == realm_id && x.user == user && x.key_index == Some(key_index) =>
                {
                    Some(
                        x.recipient_keys_bundle_access(&self.template)
                            .expect("No keys bundle access for user"),
                    )
                }
                _ => None,
            })
            .expect("Realm has had no key rotation with this index involving this user")
    }

    pub fn get_realm_keys(&self, realm_id: VlobID) -> &Vec<KeyDerivation> {
        self.template
            .events
            .iter()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::RotateKeyRealm(x) if x.realm == realm_id => Some(&x.keys),
                _ => None,
            })
            .expect("Realm has had no key rotation")
    }

    pub fn get_last_realm_key(&self, realm_id: VlobID) -> (&KeyDerivation, IndexInt) {
        self.template
            .events
            .iter()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::RotateKeyRealm(x) if x.realm == realm_id => Some((
                    x.keys.last().expect("Never empty"),
                    x.keys.len() as IndexInt,
                )),
                _ => None,
            })
            .expect("Realm has had no key rotation")
    }

    pub fn get_last_realm_keys_bundle_access_key(&self, realm_id: VlobID) -> &SecretKey {
        let event = self
            .template
            .events
            .iter()
            .rev()
            .find_map(|event| match event {
                TestbedEvent::RotateKeyRealm(x) if x.realm == realm_id => Some(x),
                _ => None,
            })
            .expect("Realm has had no key rotation");
        &event.keys_bundle_access_key
    }

    pub fn get_common_certificates_signed(&self) -> Vec<Bytes> {
        self.template
            .certificates()
            .filter_map(|event| match event.certificate {
                AnyArcCertificate::User(_)
                | AnyArcCertificate::Device(_)
                | AnyArcCertificate::UserUpdate(_)
                | AnyArcCertificate::RevokedUser(_) => Some(event.signed.clone()),
                AnyArcCertificate::RealmRole(_)
                | AnyArcCertificate::RealmName(_)
                | AnyArcCertificate::RealmKeyRotation(_)
                | AnyArcCertificate::RealmArchiving(_)
                | AnyArcCertificate::ShamirRecoveryBrief(_)
                | AnyArcCertificate::ShamirRecoveryShare(_)
                | AnyArcCertificate::ShamirRecoveryDeletion(_)
                | AnyArcCertificate::SequesterAuthority(_)
                | AnyArcCertificate::SequesterService(_)
                | AnyArcCertificate::SequesterRevokedService(_) => None,
            })
            .collect()
    }

    pub fn get_realms_certificates_signed(&self) -> HashMap<VlobID, Vec<Bytes>> {
        let mut output: HashMap<VlobID, Vec<Bytes>> = HashMap::new();
        for event in self.template.certificates() {
            match event.certificate {
                AnyArcCertificate::RealmRole(certif) => match output.entry(certif.realm_id) {
                    std::collections::hash_map::Entry::Occupied(entry) => {
                        entry.into_mut().push(event.signed.clone());
                    }
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(vec![event.signed.clone()]);
                    }
                },
                AnyArcCertificate::RealmName(certif) => match output.entry(certif.realm_id) {
                    std::collections::hash_map::Entry::Occupied(entry) => {
                        entry.into_mut().push(event.signed.clone());
                    }
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(vec![event.signed.clone()]);
                    }
                },
                AnyArcCertificate::RealmKeyRotation(certif) => {
                    match output.entry(certif.realm_id) {
                        std::collections::hash_map::Entry::Occupied(entry) => {
                            entry.into_mut().push(event.signed.clone());
                        }
                        std::collections::hash_map::Entry::Vacant(entry) => {
                            entry.insert(vec![event.signed.clone()]);
                        }
                    }
                }
                AnyArcCertificate::RealmArchiving(certif) => match output.entry(certif.realm_id) {
                    std::collections::hash_map::Entry::Occupied(entry) => {
                        entry.into_mut().push(event.signed.clone());
                    }
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(vec![event.signed.clone()]);
                    }
                },
                AnyArcCertificate::User(_)
                | AnyArcCertificate::Device(_)
                | AnyArcCertificate::UserUpdate(_)
                | AnyArcCertificate::RevokedUser(_)
                | AnyArcCertificate::ShamirRecoveryBrief(_)
                | AnyArcCertificate::ShamirRecoveryShare(_)
                | AnyArcCertificate::ShamirRecoveryDeletion(_)
                | AnyArcCertificate::SequesterAuthority(_)
                | AnyArcCertificate::SequesterService(_)
                | AnyArcCertificate::SequesterRevokedService(_) => (),
            }
        }
        output
    }

    pub fn get_sequester_certificates_signed(&self) -> Vec<Bytes> {
        self.template
            .certificates()
            .filter_map(|event| match event.certificate {
                AnyArcCertificate::SequesterAuthority(_)
                | AnyArcCertificate::SequesterService(_)
                | AnyArcCertificate::SequesterRevokedService(_) => Some(event.signed.clone()),
                AnyArcCertificate::User(_)
                | AnyArcCertificate::Device(_)
                | AnyArcCertificate::UserUpdate(_)
                | AnyArcCertificate::RevokedUser(_)
                | AnyArcCertificate::RealmRole(_)
                | AnyArcCertificate::RealmName(_)
                | AnyArcCertificate::RealmArchiving(_)
                | AnyArcCertificate::ShamirRecoveryBrief(_)
                | AnyArcCertificate::ShamirRecoveryShare(_)
                | AnyArcCertificate::ShamirRecoveryDeletion(_)
                | AnyArcCertificate::RealmKeyRotation(_) => None,
            })
            .collect()
    }

    pub fn get_shamir_recovery_certificates_signed(&self) -> Vec<Bytes> {
        self.template
            .certificates()
            .filter_map(|event| match event.certificate {
                AnyArcCertificate::ShamirRecoveryBrief(_)
                | AnyArcCertificate::ShamirRecoveryShare(_)
                | AnyArcCertificate::ShamirRecoveryDeletion(_) => Some(event.signed.clone()),
                AnyArcCertificate::User(_)
                | AnyArcCertificate::Device(_)
                | AnyArcCertificate::UserUpdate(_)
                | AnyArcCertificate::RevokedUser(_)
                | AnyArcCertificate::RealmRole(_)
                | AnyArcCertificate::RealmName(_)
                | AnyArcCertificate::RealmKeyRotation(_)
                | AnyArcCertificate::RealmArchiving(_)
                | AnyArcCertificate::SequesterAuthority(_)
                | AnyArcCertificate::SequesterService(_)
                | AnyArcCertificate::SequesterRevokedService(_) => None,
            })
            .collect()
    }

    pub fn get_shamir_recovery_certificates_signed_for_user_topic(
        &self,
        user: UserID,
    ) -> Vec<Bytes> {
        self.template
            .certificates()
            .filter_map(|event| match event.certificate {
                AnyArcCertificate::ShamirRecoveryBrief(e) => {
                    if e.user_id == user || e.per_recipient_shares.contains_key(&user) {
                        Some(event.signed.clone())
                    } else {
                        None
                    }
                }
                AnyArcCertificate::ShamirRecoveryShare(e) => {
                    if e.recipient == user {
                        Some(event.signed.clone())
                    } else {
                        None
                    }
                }
                AnyArcCertificate::ShamirRecoveryDeletion(e) => {
                    if e.setup_to_delete_user_id == user || e.share_recipients.contains(&user) {
                        Some(event.signed.clone())
                    } else {
                        None
                    }
                }
                AnyArcCertificate::User(_)
                | AnyArcCertificate::Device(_)
                | AnyArcCertificate::UserUpdate(_)
                | AnyArcCertificate::RevokedUser(_)
                | AnyArcCertificate::RealmRole(_)
                | AnyArcCertificate::RealmName(_)
                | AnyArcCertificate::RealmKeyRotation(_)
                | AnyArcCertificate::RealmArchiving(_)
                | AnyArcCertificate::SequesterAuthority(_)
                | AnyArcCertificate::SequesterService(_)
                | AnyArcCertificate::SequesterRevokedService(_) => None,
            })
            .collect()
    }

    pub fn get_user_certificate(
        &self,
        user_id: impl TryInto<UserID>,
    ) -> (Arc<UserCertificate>, Bytes) {
        let user_id = user_id
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid UserID !"));

        self.template
            .certificates()
            .find_map(|event| match event.certificate {
                AnyArcCertificate::User(certif) if certif.user_id == user_id => {
                    Some((certif, event.signed))
                }
                _ => None,
            })
            .unwrap()
    }

    pub fn get_device_certificate(
        &self,
        device_id: impl TryInto<DeviceID>,
    ) -> (Arc<DeviceCertificate>, Bytes) {
        let device_id = device_id
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid DeviceID !"));

        self.template
            .certificates()
            .find_map(|event| match event.certificate {
                AnyArcCertificate::Device(certif) if certif.device_id == device_id => {
                    Some((certif, event.signed))
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("unable to find {device_id}"))
    }

    pub fn get_revoked_certificate(
        &self,
        user_id: impl TryInto<UserID>,
    ) -> (Arc<RevokedUserCertificate>, Bytes) {
        let user_id = user_id
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid UserID !"));

        self.template
            .certificates()
            .find_map(|event| match event.certificate {
                AnyArcCertificate::RevokedUser(certif) if certif.user_id == user_id => {
                    Some((certif, event.signed))
                }
                _ => None,
            })
            .unwrap()
    }

    pub fn get_last_realm_certificate_timestamp_for_all_realms(&self) -> HashMap<VlobID, DateTime> {
        let mut timestamps = HashMap::new();

        for event in self.template.certificates() {
            let (realm_id, timestamp) = match event.certificate {
                AnyArcCertificate::RealmRole(certif) => (certif.realm_id, certif.timestamp),
                AnyArcCertificate::RealmName(certif) => (certif.realm_id, certif.timestamp),
                AnyArcCertificate::RealmKeyRotation(certif) => (certif.realm_id, certif.timestamp),
                AnyArcCertificate::RealmArchiving(certif) => (certif.realm_id, certif.timestamp),

                // Exhaustive match so that we detect when new certificates are added
                AnyArcCertificate::User(_)
                | AnyArcCertificate::Device(_)
                | AnyArcCertificate::UserUpdate(_)
                | AnyArcCertificate::RevokedUser(_)
                | AnyArcCertificate::ShamirRecoveryBrief(_)
                | AnyArcCertificate::ShamirRecoveryShare(_)
                | AnyArcCertificate::ShamirRecoveryDeletion(_)
                | AnyArcCertificate::SequesterAuthority(_)
                | AnyArcCertificate::SequesterService(_)
                | AnyArcCertificate::SequesterRevokedService(_) => continue,
            };
            // No need to compare with the previous timestamp, as the events are sorted
            timestamps.insert(realm_id, timestamp);
        }

        timestamps
    }

    pub fn get_last_realm_certificate_timestamp(&self, realm_id: VlobID) -> DateTime {
        self.template
            .certificates_rev()
            .find_map(|event| {
                let (candidate_realm_id, timestamp) = match event.certificate {
                    AnyArcCertificate::RealmRole(certif) => (certif.realm_id, certif.timestamp),
                    AnyArcCertificate::RealmName(certif) => (certif.realm_id, certif.timestamp),
                    AnyArcCertificate::RealmKeyRotation(certif) => {
                        (certif.realm_id, certif.timestamp)
                    }
                    AnyArcCertificate::RealmArchiving(certif) => {
                        (certif.realm_id, certif.timestamp)
                    }

                    // Exhaustive match so that we detect when new certificates are added
                    AnyArcCertificate::User(_)
                    | AnyArcCertificate::Device(_)
                    | AnyArcCertificate::UserUpdate(_)
                    | AnyArcCertificate::RevokedUser(_)
                    | AnyArcCertificate::ShamirRecoveryBrief(_)
                    | AnyArcCertificate::ShamirRecoveryShare(_)
                    | AnyArcCertificate::ShamirRecoveryDeletion(_)
                    | AnyArcCertificate::SequesterAuthority(_)
                    | AnyArcCertificate::SequesterService(_)
                    | AnyArcCertificate::SequesterRevokedService(_) => return None,
                };
                if candidate_realm_id == realm_id {
                    Some(timestamp)
                } else {
                    None
                }
            })
            .unwrap()
    }

    pub fn get_last_common_certificate_timestamp(&self) -> DateTime {
        self.template
            .certificates_rev()
            .find_map(|event| match event.certificate {
                AnyArcCertificate::User(certif) => Some(certif.timestamp),
                AnyArcCertificate::Device(certif) => Some(certif.timestamp),
                AnyArcCertificate::UserUpdate(certif) => Some(certif.timestamp),
                AnyArcCertificate::RevokedUser(certif) => Some(certif.timestamp),
                // Exhaustive match so that we detect when new certificates are added
                AnyArcCertificate::RealmRole(_)
                | AnyArcCertificate::RealmName(_)
                | AnyArcCertificate::RealmKeyRotation(_)
                | AnyArcCertificate::RealmArchiving(_)
                | AnyArcCertificate::ShamirRecoveryBrief(_)
                | AnyArcCertificate::ShamirRecoveryShare(_)
                | AnyArcCertificate::ShamirRecoveryDeletion(_)
                | AnyArcCertificate::SequesterAuthority(_)
                | AnyArcCertificate::SequesterService(_)
                | AnyArcCertificate::SequesterRevokedService(_) => None,
            })
            .unwrap()
    }

    pub fn get_last_sequester_certificate_timestamp(&self) -> Option<DateTime> {
        self.template
            .certificates_rev()
            .find_map(|event| match event.certificate {
                AnyArcCertificate::SequesterAuthority(certif) => Some(certif.timestamp),
                AnyArcCertificate::SequesterService(certif) => Some(certif.timestamp),
                AnyArcCertificate::SequesterRevokedService(certif) => Some(certif.timestamp),
                // Exhaustive match so that we detect when new certificates are added
                AnyArcCertificate::User(_)
                | AnyArcCertificate::Device(_)
                | AnyArcCertificate::UserUpdate(_)
                | AnyArcCertificate::RevokedUser(_)
                | AnyArcCertificate::RealmRole(_)
                | AnyArcCertificate::RealmName(_)
                | AnyArcCertificate::RealmKeyRotation(_)
                | AnyArcCertificate::RealmArchiving(_)
                | AnyArcCertificate::ShamirRecoveryBrief(_)
                | AnyArcCertificate::ShamirRecoveryShare(_)
                | AnyArcCertificate::ShamirRecoveryDeletion(_) => None,
            })
    }

    pub fn get_last_shamir_recovery_certificate_timestamp(
        &self,
        user_id: UserID,
    ) -> Option<DateTime> {
        self.template.certificates_rev().find_map(|event| {
            match &event.certificate {
                AnyArcCertificate::ShamirRecoveryBrief(certif) => {
                    if certif.user_id == user_id
                        || certif.per_recipient_shares.contains_key(&user_id)
                    {
                        Some(certif.timestamp)
                    } else {
                        None
                    }
                }
                AnyArcCertificate::ShamirRecoveryShare(certif) => {
                    if certif.user_id == user_id {
                        Some(certif.timestamp)
                    } else {
                        None
                    }
                }
                AnyArcCertificate::ShamirRecoveryDeletion(certif) => {
                    if certif.setup_to_delete_user_id == user_id
                        || certif.share_recipients.contains(&user_id)
                    {
                        Some(certif.timestamp)
                    } else {
                        None
                    }
                }

                // Exhaustive match so that we detect when new certificates are added
                AnyArcCertificate::User(_)
                | AnyArcCertificate::Device(_)
                | AnyArcCertificate::UserUpdate(_)
                | AnyArcCertificate::RevokedUser(_)
                | AnyArcCertificate::RealmRole(_)
                | AnyArcCertificate::RealmName(_)
                | AnyArcCertificate::RealmKeyRotation(_)
                | AnyArcCertificate::RealmArchiving(_)
                | AnyArcCertificate::SequesterAuthority(_)
                | AnyArcCertificate::SequesterService(_)
                | AnyArcCertificate::SequesterRevokedService(_) => None,
            }
        })
    }

    pub fn get_last_realm_role_certificate(
        &self,
        user_id: impl TryInto<UserID>,
        realm_id: VlobID,
    ) -> (Arc<RealmRoleCertificate>, Bytes) {
        let user_id = user_id
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid UserID !"));

        self.template
            .certificates_rev()
            .find_map(|event| match event.certificate {
                AnyArcCertificate::RealmRole(certif)
                    if certif.user_id == user_id && certif.realm_id == realm_id =>
                {
                    Some((certif, event.signed))
                }
                _ => None,
            })
            .unwrap()
    }

    pub fn get_last_user_update_certificate(
        &self,
        user_id: impl TryInto<UserID>,
    ) -> (Arc<UserUpdateCertificate>, Bytes) {
        let user_id = user_id
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid UserID !"));

        self.template
            .certificates_rev()
            .find_map(|event| match event.certificate {
                AnyArcCertificate::UserUpdate(certif) if certif.user_id == user_id => {
                    Some((certif, event.signed))
                }
                _ => None,
            })
            .unwrap()
    }

    pub fn get_sequester_authority_certificate(
        &self,
    ) -> (Arc<SequesterAuthorityCertificate>, Bytes) {
        self.template
            .certificates()
            .find_map(|event| match event.certificate {
                AnyArcCertificate::SequesterAuthority(certif) => Some((certif, event.signed)),
                _ => None,
            })
            .unwrap()
    }

    pub fn get_sequester_service_certificate(
        &self,
        service_id: SequesterServiceID,
    ) -> (Arc<SequesterServiceCertificate>, Bytes) {
        self.template
            .certificates()
            .find_map(|event| match event.certificate {
                AnyArcCertificate::SequesterService(certif) if certif.service_id == service_id => {
                    Some((certif, event.signed))
                }
                _ => None,
            })
            .unwrap()
    }

    pub fn get_certificates_signed(&self) -> impl Iterator<Item = Bytes> + '_ {
        self.template.certificates().map(|e| e.signed)
    }
}

pub enum TestbedTimeToLive {
    Unlimited,
    Limited { seconds: u32 },
}

/// `test_new_testbed` should be called when a test starts (and be followed
/// by a `test_drop_testbed` call at it end)
pub async fn test_new_testbed(
    template: &str,
    server_addr: Option<&ParsecAddr>,
    ttl: TestbedTimeToLive,
) -> anyhow::Result<Arc<TestbedEnv>> {
    // 1) Retrieve the template
    let template =
        test_get_template(template).ok_or(anyhow::anyhow!("Testbed template not found"))?;

    let (kind, server_addr, organization_id) = if let Some(server_addr) = server_addr {
        // 2) Call the test server to setup the env on it side
        let url = server_addr.to_http_url(Some(&format!("/testbed/new/{}", template.id)));
        let request_builder = HTTP_CLIENT.post(url);
        let request_builder = match ttl {
            TestbedTimeToLive::Unlimited => request_builder,
            TestbedTimeToLive::Limited { seconds } => {
                request_builder.query(&[("ttl", &seconds.to_string())])
            }
        };
        let response = request_builder
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Cannot communicate with testbed server: {}", e))?;
        if response.status() != StatusCode::OK {
            let url = response.url().to_owned();
            let status = response.status();
            let body = response.text().await.unwrap_or("".to_owned());
            return Err(anyhow::anyhow!(
                "POST {}: bad response from testbed server: {}\n{}",
                url,
                status,
                body
            ));
        }
        let (organization_id, server_template_crc) = {
            let response_body = response
                .text()
                .await
                .map_err(|e| anyhow::anyhow!("Bad response body from testbed server: {}", e))?;

            // Body should be something like `CoolOrg\n12345`
            let mut s = response_body.split('\n');
            match (
                s.next().map(OrganizationID::from_str),
                s.next().map(|x| x.parse::<u32>()),
                s.next(),
            ) {
                (Some(Ok(org_id)), Some(Ok(crc)), None) => (org_id, crc),
                _ => {
                    return Err(anyhow::anyhow!(
                        "Bad response body from testbed server: {:?}",
                        response_body
                    ))
                }
            }
        };

        // Ensure there is not inconsistency in the template's data with the server
        let template_crc = template.compute_crc();
        if template_crc != server_template_crc {
            return Err(anyhow::anyhow!(
                "CRC mismatch in template ({} vs {}) ! Check your server version. Maybe run `./make.py python-dev-rebuild` ?",
                template_crc,
                server_template_crc,
            ));
        }

        (
            TestbedKind::ClientServer,
            server_addr.to_owned(),
            organization_id,
        )
    } else {
        // No server, organization ID & Addr are not relevant
        let organization_id = "OfflineOrg".parse().unwrap();
        let dummy_server_addr = "parsec3://noserver.example.com".parse().unwrap();
        (TestbedKind::ClientOnly, dummy_server_addr, organization_id)
    };

    // 3) Finally register the testbed env
    let mut envs = TESTBED_ENVS.lock().expect("Mutex is poisoned");

    #[cfg(not(target_arch = "wasm32"))]
    let discriminant_dir = {
        // Config dir is used as discriminant by the testbed, hence we must make sure
        // it is unique for each test across the process to avoid concurrency issues.
        // Note this is likely not needed (but not harmful either ^^) when using
        // `cargo nextest` since each test runs in its own process then.

        use std::sync::atomic::{AtomicUsize, Ordering};

        static ENVS_COUNTER: AtomicUsize = AtomicUsize::new(0);
        let current = ENVS_COUNTER.fetch_add(1, Ordering::Relaxed);

        let mut dir = PathBuf::from(TESTBED_BASE_DUMMY_PATH);
        dir.push(current.to_string());
        dir.push(organization_id.as_ref());
        dir
    };

    #[cfg(target_arch = "wasm32")]
    let discriminant_dir = {
        // In web mode the tests are never run in parallel, hence there is no
        // need to make the discriminant unique.
        // Worst: since the database stays persistent in web, with a unique
        // name we would end up with a lot of unused databases.
        PathBuf::from(TESTBED_BASE_DUMMY_PATH)
    };

    let env = Arc::new(TestbedEnv {
        kind,
        discriminant_dir,
        server_addr,
        organization_id,
        template: YoloInit::new(template),
        cache: Mutex::default(),
    });
    envs.push(env.clone());

    Ok(env)
}

/// `test_get_testbed` should be used by components (e.g. local storage) as a hook
/// before actually doing FS access to see if they should instead switch to the in-memory
/// mock behavior.
/// If `None` is returned, that means the current config dir doesn't correspond to an
/// in-memory mock and hence the normal FS access should be done (for instance if the
/// current test is actually testing the behavior of the on-disk storage !)
pub fn test_get_testbed(discriminant_dir: &Path) -> Option<Arc<TestbedEnv>> {
    let envs = TESTBED_ENVS.lock().expect("Mutex is poisoned");
    envs.iter()
        .find(|x| x.discriminant_dir == discriminant_dir)
        .cloned()
}

pub fn test_get_testbed_component_store<T>(
    discriminant_dir: &Path,
    component_key: &'static str,
    store_factory: impl FnOnce(&TestbedEnv) -> Arc<dyn Any + Send + Sync>,
) -> Option<Arc<T>>
where
    T: Send + Sync + 'static,
{
    test_get_testbed(discriminant_dir).map(|env| {
        let component_store = {
            env.seal_and(|cache| {
                cache
                    .per_component_store
                    .iter()
                    .find_map(|(candidate_key, component_store)| {
                        if component_key == *candidate_key {
                            // Component store already exists
                            Some(component_store.to_owned())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| {
                        // Component store doesn't exist, create it
                        let store = store_factory(&env);
                        cache
                            .per_component_store
                            .push((component_key, store.clone()));
                        store
                    })
            })
        };

        component_store.downcast::<T>().unwrap_or_else(|_| {
            panic!("Unexpected component storage type for `{}`", component_key);
        })
    })
}

/// Nothing wrong will occur if `test_drop_testbed` is not called at the end of a test.
/// Only resources won't be freed, which builds up ram consumption (especially on the
/// test server if it is shared between test runs !)
pub async fn test_drop_testbed(discriminant_dir: &Path) -> anyhow::Result<()> {
    // 1) Unregister the testbed env
    let env = {
        let mut envs = TESTBED_ENVS.lock().expect("Mutex is poisoned");
        let index = envs
            .iter()
            .position(|x| x.discriminant_dir == discriminant_dir);
        match index {
            Some(index) => envs.swap_remove(index),
            None => {
                return Err(anyhow::anyhow!(
                    "No testbed with path `{:?}`",
                    discriminant_dir
                ))
            }
        }
    };

    // 2) Notify the testbed server about the end of test
    if env.kind == TestbedKind::ClientServer {
        let url = env.server_addr.to_http_url(Some(&format!(
            "/testbed/drop/{}",
            env.organization_id.as_ref()
        )));
        let response = HTTP_CLIENT
            .post(url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Cannot communicate with testbed server: {}", e))?;
        if response.status() != StatusCode::OK {
            return Err(anyhow::anyhow!(
                "Bad response status from testbed server: {:?}",
                response
            ));
        }
    }

    Ok(())
}

/// Returns the list of received emails (sender, timestamp, body), ordered
/// from oldest to newest.
pub async fn test_check_mailbox(
    server_addr: &ParsecAddr,
    email: &EmailAddress,
) -> anyhow::Result<Vec<(EmailAddress, DateTime, String)>> {
    let url = server_addr.to_http_url(Some(&format!("/testbed/mailbox/{}", email)));
    let response = HTTP_CLIENT
        .get(url)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Cannot communicate with testbed server: {}", e))?;
    if response.status() != StatusCode::OK {
        return Err(anyhow::anyhow!(
            "Bad response status from testbed server: {:?}",
            response
        ));
    }

    let response_body = response
        .text()
        .await
        .map_err(|e| anyhow::anyhow!("Bad response body from testbed server: {}", e))?;

    // Body should be something like `<sender_email>\t<timestamp>\t<base64(body)\n`,
    // also multiple mails can be send one after the other
    let mut mails = vec![];
    for raw_mail in response_body.split('\n') {
        if raw_mail.is_empty() {
            continue;
        }
        let mut s = raw_mail.split('\t');

        let base64_to_str = |b64: &str| -> Result<String, ()> {
            let as_bytes = data_encoding::BASE64
                .decode(b64.as_bytes())
                .map_err(|_| ())?;
            String::from_utf8(as_bytes).map_err(|_| ())
        };

        let (sender, timestamp, body) = match (
            s.next().map(EmailAddress::from_str),
            s.next().map(DateTime::from_str),
            s.next().map(base64_to_str),
            s.next(),
        ) {
            (Some(Ok(sender)), Some(Ok(timestamp)), Some(Ok(body)), None) => {
                (sender, timestamp, body)
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Bad response body from testbed server: {:?}",
                    response_body
                ))
            }
        };

        mails.push((sender, timestamp, body));
    }

    Ok(mails)
}

pub async fn test_new_account(
    server_addr: &ParsecAddr,
) -> anyhow::Result<(HumanHandle, KeyDerivation)> {
    let auth_method_master_secret = KeyDerivation::generate();

    let auth_method_id = AccountAuthMethodID::from(
        auth_method_master_secret.derive_uuid_from_uuid(AUTH_METHOD_ID_DERIVATION_UUID),
    );
    let auth_method_mac_key =
        auth_method_master_secret.derive_secret_key_from_uuid(AUTH_METHOD_MAC_KEY_DERIVATION_UUID);
    let auth_method_secret_key = auth_method_master_secret
        .derive_secret_key_from_uuid(AUTH_METHOD_SECRET_KEY_DERIVATION_UUID);

    let url = server_addr.to_http_url(Some("/testbed/account/new"));
    let vault_key_access = AccountVaultKeyAccess {
        vault_key: SecretKey::generate(),
    }
    .dump_and_encrypt(&auth_method_secret_key);
    let config = format!(
        "{}\n{}\n{}",
        &auth_method_id.hex(),
        &data_encoding::BASE64.encode(auth_method_mac_key.as_ref()),
        &data_encoding::BASE64.encode(vault_key_access.as_ref()),
    );
    let response = HTTP_CLIENT
        .post(url)
        .body(config.into_bytes())
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Cannot communicate with testbed server: {}", e))?;
    if response.status() != StatusCode::OK {
        return Err(anyhow::anyhow!(
            "Bad response status from testbed server: {:?}",
            response
        ));
    }

    let response_body = response
        .text()
        .await
        .map_err(|e| anyhow::anyhow!("Bad response body from testbed server: {}", e))?;

    let human_handle = HumanHandle::from_str(&response_body)
        .map_err(|e| anyhow::anyhow!("Bad response body from testbed server: {}", e))?;

    Ok((human_handle, auth_method_master_secret))
}
