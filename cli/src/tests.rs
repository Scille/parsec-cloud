// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use assert_cmd::Command;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use libparsec::{
    tmp_path, BackendAddr, BackendOrganizationBootstrapAddr, ClientConfig, LocalDevice,
    OrganizationID, TmpPath, PARSEC_CONFIG_DIR, PARSEC_DATA_DIR, PARSEC_HOME_DIR,
};

use crate::{
    create_organization::create_organization_req,
    run_testenv::{
        backend_addr_from_http_url, initialize_test_organization, new_environment, TestenvConfig,
        TESTBED_SERVER_URL,
    },
    utils::*,
};

fn get_testenv_config() -> TestenvConfig {
    if let Ok(testbed_server) = std::env::var("TESTBED_SERVER") {
        TestenvConfig::ConnectToServer(backend_addr_from_http_url(&testbed_server))
    } else {
        TestenvConfig::StartNewServer {
            stop_after_process: std::process::id(),
        }
    }
}

fn set_env(tmp_dir: &str, url: &BackendAddr) {
    std::env::set_var(TESTBED_SERVER_URL, url.to_url().to_string());
    std::env::set_var(PARSEC_HOME_DIR, format!("{tmp_dir}/cache"));
    std::env::set_var(PARSEC_DATA_DIR, format!("{tmp_dir}/share"));
    std::env::set_var(PARSEC_CONFIG_DIR, format!("{tmp_dir}/config"));
}

fn unique_org_id() -> OrganizationID {
    let uuid = uuid::Uuid::new_v4();
    format!("TestOrg-{}", &uuid.as_hyphenated().to_string()[..24])
        .parse()
        .unwrap()
}

async fn run_local_organization(
    tmp_dir: &Path,
    source_file: Option<PathBuf>,
    config: TestenvConfig,
) -> anyhow::Result<(BackendAddr, [Arc<LocalDevice>; 3])> {
    let url = new_environment(tmp_dir, source_file, config, false)
        .await?
        .unwrap();

    println!("Initializing test organization to {url}");
    initialize_test_organization(ClientConfig::default(), url.clone(), unique_org_id())
        .await
        .map(|v| (url, v))
}

#[test]
fn version() {
    Command::cargo_bin("parsec_cli")
        .unwrap()
        .arg("--version")
        .assert()
        .stdout("parsec_cli 2.16.0-a.0+dev\n");
}

#[rstest::rstest]
#[tokio::test]
async fn list_devices(tmp_path: TmpPath) {
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    set_env(&tmp_path_str, &url);

    let path = tmp_path.join("config").join("parsec-v3-alpha");
    let path_str = path.to_string_lossy();

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .arg("list-devices")
        .assert()
        .stdout(predicates::str::contains(format!(
            "Found {GREEN}4{RESET} device(s) in {YELLOW}{path_str}{RESET}:"
        )));
}

#[rstest::rstest]
#[tokio::test]
async fn create_organization(tmp_path: TmpPath) {
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    set_env(&tmp_path_str, &url);

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "create-organization",
            "--organization-id",
            &unique_org_id().to_string(),
            "--addr",
            &std::env::var(TESTBED_SERVER_URL).unwrap(),
            "--token",
            "s3cr3t",
        ])
        .assert()
        .stdout(predicates::str::contains(
            "Creating organization\nOrganization bootstrap url:",
        ));
}

#[rstest::rstest]
#[tokio::test]
async fn bootstrap_organization(tmp_path: TmpPath) {
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    set_env(&tmp_path_str, &url);

    let organization_id = unique_org_id();
    let addr = std::env::var(TESTBED_SERVER_URL).unwrap().parse().unwrap();

    println!("Creating organization {organization_id}");
    let bootstrap_token = create_organization_req(&organization_id, &addr, "s3cr3t")
        .await
        .unwrap();
    let organization_addr =
        BackendOrganizationBootstrapAddr::new(addr, organization_id, Some(bootstrap_token));

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "bootstrap-organization",
            "--addr",
            &organization_addr.to_string(),
            "--device-label",
            "pc",
            "--label",
            "Alice",
            "--email",
            "alice@example.com",
        ])
        .assert()
        .stdout(predicates::str::contains(
            "Bootstrapping organization in the server",
        ));
}

#[rstest::rstest]
#[tokio::test]
async fn invite_device(tmp_path: TmpPath) {
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..]) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    set_env(&tmp_path_str, &url);

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args(["invite-device", "--device", &alice.slughash()])
        .assert()
        .stdout(predicates::str::contains(
            "Creating device invitation\nInvitation URL:",
        ));
}

#[rstest::rstest]
#[tokio::test]
async fn invite_user(tmp_path: TmpPath) {
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..]) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    set_env(&tmp_path_str, &url);

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "invite-user",
            "--device",
            &alice.slughash(),
            "--email",
            "a@b.c",
        ])
        .assert()
        .stdout(predicates::str::contains(
            "Creating user invitation\nInvitation URL:",
        ));
}
