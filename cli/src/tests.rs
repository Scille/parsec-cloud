// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use assert_cmd::Command;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use libparsec::{
    tmp_path, BackendAddr, BackendOrganizationBootstrapAddr, ClientConfig, LocalDevice, TmpPath,
    PARSEC_CONFIG_DIR, PARSEC_DATA_DIR, PARSEC_HOME_DIR,
};

use crate::{
    create_organization::create_organization_req,
    run_testenv::{
        initialize_test_organization, new_environment, process_id_to_port, TESTBED_SERVER_URL,
    },
    utils::*,
};

fn set_env(tmp_dir: &str, pid: u32) -> u16 {
    let port = process_id_to_port(pid);
    std::env::set_var(
        TESTBED_SERVER_URL,
        format!("parsec://127.0.0.1:{port}?no_ssl=true"),
    );
    std::env::set_var(PARSEC_HOME_DIR, format!("{tmp_dir}/cache"));
    std::env::set_var(PARSEC_DATA_DIR, format!("{tmp_dir}/share"));
    std::env::set_var(PARSEC_CONFIG_DIR, format!("{tmp_dir}/config"));

    port
}

async fn run_local_organization(
    tmp_dir: &Path,
    source_file: Option<PathBuf>,
    stop_after_process: u32,
) -> anyhow::Result<[Arc<LocalDevice>; 3]> {
    let port = process_id_to_port(stop_after_process);
    new_environment(tmp_dir, source_file, stop_after_process, false).await?;

    initialize_test_organization(
        ClientConfig::default(),
        BackendAddr::new("127.0.0.1".into(), Some(port), false),
    )
    .await
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
    let id = std::process::id();
    let tmp_path_str = tmp_path.to_str().unwrap();

    set_env(&tmp_path_str, id);

    run_local_organization(&tmp_path, None, id).await.unwrap();

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
    let id = std::process::id();
    let tmp_path_str = tmp_path.to_str().unwrap();
    set_env(&tmp_path_str, id);

    run_local_organization(&tmp_path, None, id).await.unwrap();

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "create-organization",
            "--organization-id",
            "Org1",
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
    let id = std::process::id();
    let tmp_path_str = tmp_path.to_str().unwrap();
    set_env(&tmp_path_str, id);

    run_local_organization(&tmp_path, None, id).await.unwrap();

    let organization_id = "Org1".parse().unwrap();
    let addr = std::env::var(TESTBED_SERVER_URL).unwrap().parse().unwrap();

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
    let id = std::process::id();
    let tmp_path_str = tmp_path.to_str().unwrap();
    set_env(&tmp_path_str, id);

    let [alice, ..] = run_local_organization(&tmp_path, None, id).await.unwrap();

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args(["invite-device", "--device", &alice.slughash()])
        .assert()
        .stdout(predicates::str::contains(
            "Creating device invitation\nInvitation URL:",
        ));
}
