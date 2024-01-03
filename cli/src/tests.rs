// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use assert_cmd::Command;
use std::{
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

use libparsec::{
    authenticated_cmds::latest::invite_new::{self, InviteNewRep, UserOrDevice},
    get_default_config_dir, tmp_path, AuthenticatedCmds, BackendAddr, BackendInvitationAddr,
    BackendOrganizationBootstrapAddr, ClientConfig, InvitationType, LocalDevice, OrganizationID,
    ProxyConfig, TmpPath, PARSEC_CONFIG_DIR, PARSEC_DATA_DIR, PARSEC_HOME_DIR,
};

use crate::{
    create_organization::create_organization_req,
    run_testenv::{
        backend_addr_from_http_url, initialize_test_organization, new_environment, TestenvConfig,
        DEFAULT_ADMINISTRATION_TOKEN, TESTBED_SERVER_URL,
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
) -> anyhow::Result<(BackendAddr, [Arc<LocalDevice>; 3], OrganizationID)> {
    let url = new_environment(tmp_dir, source_file, config, false)
        .await?
        .unwrap();

    println!("Initializing test organization to {url}");
    let org_id = unique_org_id();
    initialize_test_organization(ClientConfig::default(), url.clone(), org_id.clone())
        .await
        .map(|v| (url, v, org_id))
}

fn wait_for(mut reader: impl BufRead, buf: &mut String, text: &str) {
    buf.clear();
    while let Ok(_) = reader.read_line(buf) {
        if buf.is_empty() {
            break;
        } else if buf.contains(text) {
            break;
        }
        buf.clear();
    }
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
    let (url, _, _) = run_local_organization(&tmp_path, None, config)
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
    let (url, _, _) = run_local_organization(&tmp_path, None, config)
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
    let (url, _, _) = run_local_organization(&tmp_path, None, config)
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
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
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
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
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

#[rstest::rstest]
#[tokio::test]
async fn cancel_invitation(tmp_path: TmpPath) {
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(&tmp_path_str, &url);

    let cmds = AuthenticatedCmds::new(
        &get_default_config_dir(),
        alice.clone(),
        ProxyConfig::new_from_env().unwrap(),
    )
    .unwrap();

    let rep = cmds
        .send(invite_new::Req(UserOrDevice::Device { send_email: false }))
        .await
        .unwrap();

    let invitation_addr = match rep {
        InviteNewRep::Ok { token, .. } => BackendInvitationAddr::new(
            alice.organization_addr.clone(),
            alice.organization_id().clone(),
            InvitationType::Device,
            token,
        ),
        rep => {
            panic!("Server refused to create device invitation: {rep:?}");
        }
    };

    let token = invitation_addr.token();

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "cancel-invitation",
            "--device",
            &alice.slughash(),
            "--token",
            &format!("{}", token.as_simple()),
        ])
        .assert()
        .stdout(predicates::str::contains(
            "Deleting invitation\nInvitation deleted",
        ));
}

#[rstest::rstest]
#[tokio::test]
async fn stats_organization(tmp_path: TmpPath) {
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, _, org_id) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(&tmp_path_str, &url);

    let expect = format!(
        "{:#}\n",
        serde_json::json!({
            "active_users": 3,
            "data_size": 0,
            "metadata_size": 0,
            "realms": 0,
            "users": 3,
            "users_per_profile_detail": [
                {
                    "active": 1,
                    "profile": "ADMIN",
                    "revoked": 0
                },
                {
                    "active": 1,
                    "profile": "STANDARD",
                    "revoked": 0
                },
                {
                    "active": 1,
                    "profile": "OUTSIDER",
                    "revoked": 0
                }
            ]
        })
    );

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "stats-organization",
            "--organization-id",
            &org_id.to_string(),
            "--addr",
            &url.to_string(),
            "--token",
            DEFAULT_ADMINISTRATION_TOKEN,
        ])
        .assert()
        .stdout(expect);
}

#[rstest::rstest]
#[tokio::test]
async fn export_recovery_device(tmp_path: TmpPath) {
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    let output = tmp_path.join("recovery_device");

    set_env(&tmp_path_str, &url);

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "export-recovery-device",
            "--device",
            &alice.slughash(),
            "--output",
            &output.to_string_lossy(),
        ])
        .assert()
        .stdout(predicates::str::contains(
            "Saving recovery device file\nSaved in",
        ));

    assert!(output.exists());
}

#[rstest::rstest]
#[tokio::test]
async fn import_recovery_device(tmp_path: TmpPath) {
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    let input = tmp_path.join("recovery_device");

    set_env(&tmp_path_str, &url);

    let passphrase = libparsec::save_recovery_device(&input, &alice)
        .await
        .unwrap();

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "import-recovery-device",
            "--input",
            &input.to_string_lossy(),
            "--passphrase",
            &passphrase,
        ])
        .assert()
        .stdout(predicates::str::contains(
            "Saving new device\nSaved new device",
        ));
}

#[rstest::rstest]
#[tokio::test]
async fn stats_server(tmp_path: TmpPath) {
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, _, _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(&tmp_path_str, &url);

    let expect_empty = format!(
        "{:#}\n",
        serde_json::json!({
            "stats": []
        })
    );

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "stats-server",
            "--addr",
            &url.to_string(),
            "--token",
            DEFAULT_ADMINISTRATION_TOKEN,
        ])
        .assert()
        .stdout(predicates::prelude::predicate::ne(expect_empty.clone()));

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "stats-server",
            "--addr",
            &url.to_string(),
            "--token",
            DEFAULT_ADMINISTRATION_TOKEN,
            "--format",
            "csv",
        ])
        .assert()
        .stdout(predicates::str::contains("organization_id,data_size,metadata_size,realms,active_users,admin_users_active,admin_users_revoked,standard_users_active,standard_users_revoked,outsider_users_active,outsider_users_revoked\r\n"));

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "stats-server",
            "--addr",
            &url.to_string(),
            "--token",
            DEFAULT_ADMINISTRATION_TOKEN,
            "--end-date",
            "2024-01-01T00:00:00-00:00",
        ])
        .assert()
        .stdout(expect_empty);
}

#[rstest::rstest]
#[tokio::test]
async fn status_organization(tmp_path: TmpPath) {
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, _, org_id) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(&tmp_path_str, &url);

    let expect = format!(
        "{:#}\n",
        serde_json::json!({
            "active_users_limit": null,
            "is_bootstrapped": true,
            "is_expired": false,
            "user_profile_outsider_allowed": true
        })
    );

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "status-organization",
            "--organization-id",
            &org_id.to_string(),
            "--addr",
            &url.to_string(),
            "--token",
            DEFAULT_ADMINISTRATION_TOKEN,
        ])
        .assert()
        .stdout(expect);
}

#[rstest::rstest]
#[tokio::test]
async fn list_invitations(tmp_path: TmpPath) {
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(&tmp_path_str, &url);

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args(["list-invitations", "--device", &alice.slughash()])
        .assert()
        .stdout(predicates::str::contains(
            "Listing invitations\nNo invitation.",
        ));

    let cmds = AuthenticatedCmds::new(
        &get_default_config_dir(),
        alice.clone(),
        ProxyConfig::new_from_env().unwrap(),
    )
    .unwrap();

    let rep = cmds
        .send(invite_new::Req(UserOrDevice::Device { send_email: false }))
        .await
        .unwrap();

    let invitation_addr = match rep {
        InviteNewRep::Ok { token, .. } => BackendInvitationAddr::new(
            alice.organization_addr.clone(),
            alice.organization_id().clone(),
            InvitationType::Device,
            token,
        ),
        rep => {
            panic!("Server refused to create device invitation: {rep:?}");
        }
    };

    let token = invitation_addr.token();

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args(["list-invitations", "--device", &alice.slughash()])
        .assert()
        .stdout(predicates::str::contains(format!(
            "Listing invitations\n{}\t{YELLOW}idle{RESET}\tdevice",
            token.as_simple()
        )));
}

#[rstest::rstest]
#[tokio::test]
async fn invite_device_dance(tmp_path: TmpPath) {
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    set_env(&tmp_path_str, &url);

    let cmds = AuthenticatedCmds::new(
        &get_default_config_dir(),
        alice.clone(),
        ProxyConfig::new_from_env().unwrap(),
    )
    .unwrap();

    let rep = cmds
        .send(invite_new::Req(UserOrDevice::Device { send_email: false }))
        .await
        .unwrap();

    let invitation_addr = match rep {
        InviteNewRep::Ok { token, .. } => BackendInvitationAddr::new(
            alice.organization_addr.clone(),
            alice.organization_id().clone(),
            InvitationType::Device,
            token,
        ),
        rep => {
            panic!("Server refused to create device invitation: {rep:?}");
        }
    };

    let token = invitation_addr.token();

    let p_greeter = std::process::Command::new("cargo")
        .args([
            "run",
            "--profile=ci-rust",
            "--package=parsec_cli",
            "--features=testenv",
            "greet-invitation",
            "--token",
            &format!("{}", token.as_simple()),
            "--device",
            &alice.slughash(),
        ])
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let p_claimer = std::process::Command::new("cargo")
        .args([
            "run",
            "--profile=ci-rust",
            "--package=parsec_cli",
            "--features=testenv",
            "claim-invitation",
            "--addr",
            &invitation_addr.to_url().to_string(),
        ])
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdout_greeter = BufReader::new(p_greeter.stdout.unwrap());
    let mut stdout_claimer = BufReader::new(p_claimer.stdout.unwrap());
    let mut stdin_greeter = p_greeter.stdin.unwrap();
    let mut stdin_claimer = p_claimer.stdin.unwrap();
    let mut buf = String::new();

    wait_for(&mut stdout_greeter, &mut buf, "Code to provide to claimer");
    let sas_code = buf.split_once(YELLOW).unwrap().1[..4].to_string();
    wait_for(&mut stdout_claimer, &mut buf, &sas_code);
    let sas_code_index = &buf[1..2].to_string();
    wait_for(&mut stdout_claimer, &mut buf, "Select code");
    stdin_claimer
        .write_all(format!("{sas_code_index}\n").as_bytes())
        .unwrap();

    wait_for(&mut stdout_claimer, &mut buf, "Code to provide to greeter");
    let sas_code = buf.split_once(YELLOW).unwrap().1[..4].to_string();
    wait_for(&mut stdout_greeter, &mut buf, &sas_code);
    let sas_code_index = &buf[1..2].to_string();
    wait_for(&mut stdout_greeter, &mut buf, "Select code");
    stdin_greeter
        .write_all(format!("{sas_code_index}\n").as_bytes())
        .unwrap();

    wait_for(&mut stdout_claimer, &mut buf, "Enter device label");
    stdin_claimer.write_all(b"DeviceLabelTest\n").unwrap();

    wait_for(&mut stdout_greeter, &mut buf, "Creating the device");
    wait_for(&mut stdout_claimer, &mut buf, "Saved");
}

#[rstest::rstest]
#[tokio::test]
async fn invite_user_dance(tmp_path: TmpPath) {
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    set_env(&tmp_path_str, &url);

    let cmds = AuthenticatedCmds::new(
        &get_default_config_dir(),
        alice.clone(),
        ProxyConfig::new_from_env().unwrap(),
    )
    .unwrap();

    let rep = cmds
        .send(invite_new::Req(UserOrDevice::User {
            claimer_email: "a@b.c".into(),
            send_email: false,
        }))
        .await
        .unwrap();

    let invitation_addr = match rep {
        InviteNewRep::Ok { token, .. } => BackendInvitationAddr::new(
            alice.organization_addr.clone(),
            alice.organization_id().clone(),
            InvitationType::Device,
            token,
        ),
        rep => {
            panic!("Server refused to create user invitation: {rep:?}");
        }
    };

    let token = invitation_addr.token();

    let p_greeter = std::process::Command::new("cargo")
        .args([
            "run",
            "--profile=ci-rust",
            "--package=parsec_cli",
            "--features=testenv",
            "greet-invitation",
            "--token",
            &format!("{}", token.as_simple()),
            "--device",
            &alice.slughash(),
        ])
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let p_claimer = std::process::Command::new("cargo")
        .args([
            "run",
            "--profile=ci-rust",
            "--package=parsec_cli",
            "--features=testenv",
            "claim-invitation",
            "--addr",
            &invitation_addr.to_url().to_string(),
        ])
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdout_greeter = BufReader::new(p_greeter.stdout.unwrap());
    let mut stdout_claimer = BufReader::new(p_claimer.stdout.unwrap());
    let mut stdin_greeter = p_greeter.stdin.unwrap();
    let mut stdin_claimer = p_claimer.stdin.unwrap();
    let mut buf = String::new();

    wait_for(&mut stdout_greeter, &mut buf, "Code to provide to claimer");
    let sas_code = buf.split_once(YELLOW).unwrap().1[..4].to_string();
    wait_for(&mut stdout_claimer, &mut buf, &sas_code);
    let sas_code_index = &buf[1..2].to_string();
    wait_for(&mut stdout_claimer, &mut buf, "Select code");
    stdin_claimer
        .write_all(format!("{sas_code_index}\n").as_bytes())
        .unwrap();

    wait_for(&mut stdout_claimer, &mut buf, "Code to provide to greeter");
    let sas_code = buf.split_once(YELLOW).unwrap().1[..4].to_string();
    wait_for(&mut stdout_greeter, &mut buf, &sas_code);
    let sas_code_index = &buf[1..2].to_string();
    wait_for(&mut stdout_greeter, &mut buf, "Select code");
    stdin_greeter
        .write_all(format!("{sas_code_index}\n").as_bytes())
        .unwrap();

    wait_for(&mut stdout_claimer, &mut buf, "Enter device label");
    stdin_claimer.write_all(b"DeviceLabelTest\n").unwrap();
    wait_for(&mut stdout_claimer, &mut buf, "Enter email");
    stdin_claimer.write_all(b"alice2@example.com\n").unwrap();
    wait_for(&mut stdout_claimer, &mut buf, "Enter name");
    stdin_claimer.write_all(b"alice2\n").unwrap();

    wait_for(&mut stdout_greeter, &mut buf, "Which profile?");
    stdin_greeter.write_all(b"0\n").unwrap();

    wait_for(&mut stdout_greeter, &mut buf, "Creating the user");
    wait_for(&mut stdout_claimer, &mut buf, "Saved");
}

#[rstest::rstest]
#[tokio::test]
async fn remove_device(tmp_path: TmpPath) {
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(&tmp_path_str, &url);

    let process = std::process::Command::new("cargo")
        .args([
            "run",
            "--profile=ci-rust",
            "--package=parsec_cli",
            "--features=testenv",
            "remove-device",
            "--device",
            &alice.slughash(),
        ])
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdout = BufReader::new(process.stdout.unwrap());
    let mut stdin = process.stdin.unwrap();
    let mut buf = String::new();

    let alice_device_file = tmp_path
        .join("config")
        .join("parsec-v3-alpha")
        .join("devices")
        .join(format!("{}.keys", alice.slughash()));

    assert!(alice_device_file.exists());

    wait_for(&mut stdout, &mut buf, "Are you sure? (y/n)");
    stdin.write_all(b"y\n").unwrap();

    wait_for(&mut stdout, &mut buf, "The device has been removed");

    assert!(!alice_device_file.exists());
}
