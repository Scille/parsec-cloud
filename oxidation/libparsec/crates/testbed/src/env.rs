// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use lazy_static::lazy_static;
use reqwest::StatusCode;
use std::{
    any::Any,
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use libparsec_types::{
    BackendAddr, BackendOrganizationAddr, DeviceID, LocalDevice, OrganizationID, TimeProvider,
};

use crate::{TestbedTemplate, TESTBED_TEMPLATES};

// Testbed always works in memory, the path only acts as an identifier.
const TESTBED_BASE_DUMMY_PATH: &str = "/parsec/testbed/";

#[derive(Debug, PartialEq)]
pub enum TestbedKind {
    ClientServer,
    ClientOnly, // Server is considered offline for the whole run
}

#[derive(Debug)]
pub struct TestbedEnv {
    pub kind: TestbedKind,
    /// Fake path used used as the key identifying the testbed env.
    /// In Parsec there is two base directory: `config_dir` and `data_base_dir`
    /// when the testbed is used, both should be set to the discriminant value.
    /// Note: testbed is designed to always run mocked in memory, so this path
    /// will never exist on the file system.
    pub discriminant_dir: PathBuf,
    pub organization_addr: BackendOrganizationAddr,
    pub template: Arc<TestbedTemplate>,
    // Testbed is designed to run entirely in memory (i.e. with no FS accesses)
    //
    // This hashmap should be used by components (e.g. device storage, local database)
    // to keep track of arbitrary data that would otherwise be persisted on FS.
    // To avoid clash between components, key should be the name (e.g. "local_database").
    //
    // Note the idea is for the component to be as lazy as possible: they don't have to
    // store/initialize anything until they are actually asked to do something on a
    // given config dir, at which point they should use `test_get_testbed` and
    // initialize `persistence_store` by using the data from `template`.
    pub persistence_store: Mutex<HashMap<&'static str, Box<dyn Any + Send>>>,
    // Must keep track of the generated local devices to avoid nasty behavior
    // if the test call generate the local device multiple times for the same device.
    local_devices_cache: Mutex<Vec<Arc<LocalDevice>>>,
}

impl TestbedEnv {
    pub fn local_device(&self, device_id: DeviceID) -> Arc<LocalDevice> {
        let mut local_devices = self.local_devices_cache.lock().expect("Mutex is poisoned");
        for item in local_devices.iter() {
            if item.device_id == device_id {
                return item.clone();
            }
        }
        // Not found, must generate it
        let device = self.template.device(&device_id);
        let user = self.template.user(device_id.user_id());
        let local_device = Arc::new(LocalDevice {
            organization_addr: self.organization_addr.to_owned(),
            device_id,
            device_label: device.device_label.to_owned(),
            human_handle: user.human_handle.to_owned(),
            signing_key: device.signing_key.to_owned(),
            private_key: user.private_key.to_owned(),
            profile: user.profile.to_owned(),
            user_manifest_id: user.user_manifest_id.to_owned(),
            user_manifest_key: user.user_manifest_key.to_owned(),
            local_symkey: device.local_symkey.to_owned(),
            time_provider: TimeProvider::default(),
        });
        local_devices.push(local_device.clone());
        local_device
    }
}

// Currently in use testbeds
lazy_static! {
    static ref TESTBED_ENVS: Mutex<Vec<Arc<TestbedEnv>>> = Mutex::default();
}

lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

/// `test_new_testbed` should be called when a test starts (and be followed
/// by a `test_drop_testbed` call at it end)
pub async fn test_new_testbed(
    template: &str,
    server_addr: Option<&BackendAddr>,
) -> Arc<TestbedEnv> {
    // 1) Retrieve the template
    let template = TESTBED_TEMPLATES
        .iter()
        .find(|x| x.id == template)
        .unwrap_or_else(|| {
            panic!("No testbed template named `{}`", template);
        });

    let (kind, organization_addr) = if let Some(server_addr) = server_addr {
        // 2) Call the test server to setup the env on it side
        let url = server_addr.to_http_url(Some(&format!("/testbed/new/{}", template.id)));
        let response = HTTP_CLIENT
            .post(url)
            .send()
            .await
            .expect("Cannot communicate with testbed server");
        if response.status() != StatusCode::OK {
            panic!("Bad response status from testbed server: {:?}", response);
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
        assert_eq!(
            template.crc, server_template_crc,
            "CRC mismatch in template ! Check your server version."
        );

        let organization_addr = BackendOrganizationAddr::new(
            server_addr.to_owned(),
            organization_id,
            template.root_signing_key.verify_key(),
        );
        (TestbedKind::ClientServer, organization_addr)
    } else {
        // No server, organization ID & Addr are not relevant
        let organization_addr = BackendOrganizationAddr::new(
            "parsec://offline.example.com"
                .parse::<BackendAddr>()
                .unwrap(),
            "OfflineOrg".parse().unwrap(),
            template.root_signing_key.verify_key(),
        );
        (TestbedKind::ClientOnly, organization_addr)
    };

    // 3) Finally register the testbed env
    let mut envs = TESTBED_ENVS.lock().expect("Mutex is poisoned");
    // Config dir is used as discriminent by the testbed, hence we must make sure it
    // is unique accross the process to avoid concurrency issues
    static ENVS_COUNTER: AtomicUsize = AtomicUsize::new(0);
    let current = ENVS_COUNTER.fetch_add(1, Ordering::Relaxed);
    let discriminant_dir = {
        let mut dir = PathBuf::from(TESTBED_BASE_DUMMY_PATH);
        dir.push(current.to_string());
        dir.push(organization_addr.organization_id().as_ref());
        dir
    };
    let env = Arc::new(TestbedEnv {
        kind,
        discriminant_dir,
        organization_addr,
        template: template.clone(),
        persistence_store: Mutex::default(),
        local_devices_cache: Mutex::default(),
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
        let url = env.organization_addr.to_http_url(Some(&format!(
            "/testbed/drop/{}",
            env.organization_addr.organization_id().as_ref()
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
