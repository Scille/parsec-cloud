// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use super::*;

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
    // Config dir is used as the key identifying the testbed env
    // note testbed is designed to always run mocked in memory
    pub client_config_dir: PathBuf,
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
pub async fn test_new_testbed<'a>(
    template: &'a str,
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
        if response.status().as_u16() != 200 {
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
            "parsec://offline.example.com".parse().unwrap(),
            "OfflineOrg".parse().unwrap(),
            template.root_signing_key.verify_key(),
        );
        (TestbedKind::ClientOnly, organization_addr)
    };

    // 3) Finally register the testbed env
    let mut envs = TESTBED_ENVS.lock().expect("Mutex is poisoned");
    let client_config_dir = {
        let mut dir = PathBuf::from(TESTBED_BASE_DUMMY_PATH);
        dir.push(organization_addr.organization_id().as_ref());
        dir
    };
    let env = Arc::new(TestbedEnv {
        kind,
        client_config_dir,
        organization_addr,
        template: template.clone(),
        persistence_store: Mutex::default(),
    });
    envs.push(env.clone());
    env
}

/// `test_get_testbed` should be used by components (e.g. local storage) as a hook
/// before actually doing FS access to see if they should instead switch to the in-memory
/// mock behavior.
/// If `None` is returned, that means the current config dir doesn't correspond to an
/// in-memory mock and hence the normal FS acess should be done (for instance if the
/// current test is actually testing the behavior of the on-disk storage !)
pub fn test_get_testbed(client_config_dir: &Path) -> Option<Arc<TestbedEnv>> {
    let envs = TESTBED_ENVS.lock().expect("Mutex is poisoned");
    envs.iter()
        .find(|x| x.client_config_dir == client_config_dir)
        .cloned()
}

/// Nothing wrong will occur if `test_drop_testbed` is not called at the end of a test.
/// Only resources won't be freed, which builds up ram consumtion (especially on the
/// test server if it is shared between test runs !)
pub async fn test_drop_testbed(client_config_dir: &Path) {
    // 1) Unregister the testbed env
    let env = {
        let mut envs = TESTBED_ENVS.lock().expect("Mutex is poisoned");
        let index = envs
            .iter()
            .position(|x| x.client_config_dir == client_config_dir);
        match index {
            Some(index) => envs.swap_remove(index),
            None => panic!("No testbed with path `{:?}`", client_config_dir),
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
        if response.status().as_u16() != 200 {
            panic!("Bad response status from testbed server: {:?}", response);
        }
    }
}
