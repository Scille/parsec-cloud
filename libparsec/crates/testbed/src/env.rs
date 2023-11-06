// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    any::Any,
    path::{Path, PathBuf},
    str::FromStr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
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

lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

#[derive(Default)]
struct TestbedEnvCache {
    organization_addr: Option<Arc<BackendOrganizationAddr>>,
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

        let (human_handle, private_key, profile, user_realm_id, user_realm_key) = env
            .template
            .events
            .iter()
            .find_map(|e| match e {
                TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                    first_user_device_id: candidate,
                    first_user_human_handle: human_handle,
                    first_user_private_key: private_key,
                    first_user_user_realm_id: user_realm_id,
                    first_user_user_realm_key: user_realm_key,
                    ..
                }) if candidate.user_id() == device_id.user_id() => Some((
                    human_handle,
                    private_key,
                    UserProfile::Admin,
                    user_realm_id,
                    user_realm_key,
                )),
                TestbedEvent::NewUser(TestbedEventNewUser {
                    device_id: candidate,
                    human_handle,
                    private_key,
                    initial_profile,
                    user_realm_id,
                    user_realm_key,
                    ..
                }) if candidate.user_id() == device_id.user_id() => Some((
                    human_handle,
                    private_key,
                    *initial_profile,
                    user_realm_id,
                    user_realm_key,
                )),
                _ => None,
            })
            .expect("User not found");

        let (device_label, signing_key, local_symkey) = env
            .template
            .events
            .iter()
            .find_map(|e| match e {
                TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                    first_user_device_id: candidate,
                    first_user_first_device_label: device_label,
                    first_user_first_device_signing_key: signing_key,
                    first_user_local_symkey: local_symkey,
                    ..
                })
                | TestbedEvent::NewUser(TestbedEventNewUser {
                    device_id: candidate,
                    first_device_label: device_label,
                    first_device_signing_key: signing_key,
                    local_symkey,
                    ..
                })
                | TestbedEvent::NewDevice(TestbedEventNewDevice {
                    device_id: candidate,
                    device_label,
                    signing_key,
                    local_symkey,
                    ..
                }) if candidate == &device_id => Some((device_label, signing_key, local_symkey)),
                _ => None,
            })
            .expect("Device not found");

        let local_device = Arc::new(LocalDevice {
            organization_addr: (*self.organization_addr(env)).clone(),
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

    pub fn organization_addr(&mut self, env: &TestbedEnv) -> Arc<BackendOrganizationAddr> {
        self.organization_addr
            .get_or_insert_with(|| {
                let root_signing_key = env.template.root_signing_key();
                let organization_addr = BackendOrganizationAddr::new(
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
    pub server_addr: BackendAddr,
    pub organization_id: OrganizationID,
    pub template: Arc<TestbedTemplate>,

    cache: Mutex<Option<TestbedEnvCache>>,
}

impl Clone for TestbedEnv {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind,
            discriminant_dir: self.discriminant_dir.clone(),
            server_addr: self.server_addr.clone(),
            organization_id: self.organization_id.clone(),
            template: self.template.clone(),
            cache: Mutex::default(),
        }
    }
}

impl std::fmt::Debug for TestbedEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        cb(cache)
    }

    pub fn is_sealed(&self) -> bool {
        let guard = self.cache.lock().expect("Mutex is poisoned");
        guard.is_some()
    }

    /// Customize the env testbed
    ///
    /// ```
    /// let new_env = env.customize(|builder| {
    ///   // Add any events you want
    ///   builder.new_user("bob");
    ///   builder.new_user_realm("bob");
    ///   // Clear client storages (certificate, user, workspace data&cache)
    ///   builder.filter_client_storage_events(|_event| true);
    /// });
    /// // `env` can still be used to access stuff, but it will be missing the new
    /// // items, so the best is to just shadow it with `new_env`.
    /// ```
    /// Be careful env is going to be duplicated under the hood to apply the customization.
    /// Hence only the last call of `customize` with be taken into account.
    pub fn customize(&self, cb: impl FnOnce(&mut TestbedTemplateBuilder)) -> Arc<TestbedEnv> {
        self.customize_with_map(cb).0
    }

    pub fn customize_with_map<T>(
        &self,
        cb: impl FnOnce(&mut TestbedTemplateBuilder) -> T,
    ) -> (Arc<TestbedEnv>, T) {
        let allow_server_side_events = matches!(self.kind, TestbedKind::ClientOnly);
        let mut builder = TestbedTemplateBuilder::new_from_template(
            "custom",
            &self.template,
            allow_server_side_events,
        );
        let ret = cb(&mut builder);

        // Retrieve current testbed env in the global store...
        let mut envs = TESTBED_ENVS.lock().expect("Mutex is poisoned");
        let env = envs
            .iter_mut()
            .find(|env| env.discriminant_dir == self.discriminant_dir)
            .expect("Must exist");

        // ...and patch it with the new template
        assert!(
            !env.is_sealed(),
            "Testbed template cannot be customized once the env is sealed"
        );
        Arc::make_mut(env).template = builder.finalize();

        ((*env).clone(), ret)
    }

    pub fn organization_addr(&self) -> Arc<BackendOrganizationAddr> {
        self.seal_and(|cache| cache.organization_addr(self))
    }

    pub fn local_device(&self, device_id: impl TryInto<DeviceID>) -> Arc<LocalDevice> {
        let device_id = device_id
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid DeviceID !"));
        self.seal_and(|cache| cache.local_device(self, device_id))
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
            .unwrap()
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
    pub fn get_last_certificate_index(&self) -> IndexInt {
        self.template
            .certificates_rev()
            .map(|x| x.certificate_index)
            .next()
            .unwrap_or(0)
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

/// `test_new_testbed` should be called when a test starts (and be followed
/// by a `test_drop_testbed` call at it end)
pub async fn test_new_testbed(
    template: &str,
    server_addr: Option<&BackendAddr>,
) -> Arc<TestbedEnv> {
    // 1) Retrieve the template
    let template = test_get_template(template).expect("Testbed template not found");

    let (kind, server_addr, organization_id) = if let Some(server_addr) = server_addr {
        // 2) Call the test server to setup the env on it side
        let url = server_addr.to_http_url(Some(&format!("/testbed/new/{}", template.id)));
        let response = HTTP_CLIENT
            .post(url)
            .send()
            .await
            .expect("Cannot communicate with testbed server");
        if response.status() != StatusCode::OK {
            let url = response.url().to_owned();
            let status = response.status();
            let body = response.text().await.unwrap_or("".to_owned());
            panic!(
                "POST {}: bad response from testbed server: {}\n{}",
                url, status, body
            );
        }
        let (organization_id, server_template_crc) = {
            let response_body = response
                .text()
                .await
                .expect("Bad response body from testbed server");

            // Body should be something like `CoolOrg\n12345`
            let mut s = response_body.split('\n');
            match (
                s.next().map(OrganizationID::from_str),
                s.next().map(|x| x.parse::<u32>()),
                s.next(),
            ) {
                (Some(Ok(org_id)), Some(Ok(crc)), None) => (org_id, crc),
                _ => panic!("Bad response body from testbed server"),
            }
        };

        // Ensure there is not inconsistency in the template's data with the server
        let template_crc = template.compute_crc();
        assert_eq!(
            template_crc, server_template_crc,
            "CRC mismatch in template ! Check your server version. Maybe run `./make.py python-dev-rebuild` ?"
        );

        (
            TestbedKind::ClientServer,
            server_addr.to_owned(),
            organization_id,
        )
    } else {
        // No server, organization ID & Addr are not relevant
        let organization_id = "OfflineOrg".parse().unwrap();
        let dummy_server_addr = "parsec://noserver.example.com".parse().unwrap();
        (TestbedKind::ClientOnly, dummy_server_addr, organization_id)
    };

    // 3) Finally register the testbed env
    let mut envs = TESTBED_ENVS.lock().expect("Mutex is poisoned");
    // Config dir is used as discriminent by the testbed, hence we must make sure it
    // is unique across the process to avoid concurrency issues
    static ENVS_COUNTER: AtomicUsize = AtomicUsize::new(0);
    let current = ENVS_COUNTER.fetch_add(1, Ordering::Relaxed);
    let discriminant_dir = {
        let mut dir = PathBuf::from(TESTBED_BASE_DUMMY_PATH);
        dir.push(current.to_string());
        dir.push(organization_id.as_ref());
        dir
    };
    let env = Arc::new(TestbedEnv {
        kind,
        discriminant_dir,
        server_addr,
        organization_id,
        template: template.clone(),
        cache: Mutex::default(),
    });
    envs.push(env.clone());
    env
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

        let downcasted = component_store.downcast::<T>().unwrap_or_else(|_| {
            panic!("Unexpected component storage type for `{}`", component_key);
        });

        downcasted
    })
}

/// Nothing wrong will occur if `test_drop_testbed` is not called at the end of a test.
/// Only resources won't be freed, which builds up ram consumption (especially on the
/// test server if it is shared between test runs !)
pub async fn test_drop_testbed(discriminant_dir: &Path) {
    // 1) Unregister the testbed env
    let env = {
        let mut envs = TESTBED_ENVS.lock().expect("Mutex is poisoned");
        let index = envs
            .iter()
            .position(|x| x.discriminant_dir == discriminant_dir);
        match index {
            Some(index) => envs.swap_remove(index),
            None => panic!("No testbed with path `{:?}`", discriminant_dir),
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
            .expect("Cannot communicate with testbed server");
        if response.status() != StatusCode::OK {
            panic!("Bad response status from testbed server: {:?}", response);
        }
    }
}
