// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use assert_cmd::{cargo::CommandCargoExt, Command};
use predicates::prelude::PredicateBooleanExt;
use std::{
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

use libparsec::{
    authenticated_cmds::latest::{
        invite_new_device::{self, InviteNewDeviceRep},
        invite_new_user::{self, InviteNewUserRep},
    },
    get_default_config_dir, tmp_path, AuthenticatedCmds, ClientConfig, HumanHandle, InvitationType,
    LocalDevice, OrganizationID, ParsecAddr, ParsecInvitationAddr, ProxyConfig, TmpPath,
    PARSEC_BASE_CONFIG_DIR, PARSEC_BASE_DATA_DIR, PARSEC_BASE_HOME_DIR,
};

use crate::{
    create_organization::create_organization_req,
    run_testenv::{
        initialize_test_organization, new_environment, parsec_addr_from_http_url, TestenvConfig,
        DEFAULT_ADMINISTRATION_TOKEN, DEFAULT_DEVICE_PASSWORD, TESTBED_SERVER_URL,
    },
    utils::*,
};

fn get_testenv_config() -> TestenvConfig {
    if let Ok(testbed_server) = std::env::var("TESTBED_SERVER") {
        TestenvConfig::ConnectToServer(parsec_addr_from_http_url(&testbed_server))
    } else {
        TestenvConfig::StartNewServer {
            stop_after_process: std::process::id(),
        }
    }
}

fn set_env(tmp_dir: &str, url: &ParsecAddr) {
    std::env::set_var(TESTBED_SERVER_URL, url.to_url().to_string());
    std::env::set_var(PARSEC_BASE_HOME_DIR, format!("{tmp_dir}/cache"));
    std::env::set_var(PARSEC_BASE_DATA_DIR, format!("{tmp_dir}/share"));
    std::env::set_var(PARSEC_BASE_CONFIG_DIR, format!("{tmp_dir}/config"));
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
) -> anyhow::Result<(ParsecAddr, [Arc<LocalDevice>; 3], OrganizationID)> {
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
    while reader.read_line(buf).is_ok() {
        if buf.is_empty() || buf.contains(text) {
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
        .stdout(
            // Using `concat!` simplify updating the version using `version-updater`
            concat!("parsec_cli 3.0.0-b.12+dev", "\n"),
        );
}

#[rstest::rstest]
#[tokio::test]
async fn list_devices(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, _, _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    set_env(tmp_path_str, &url);

    let path = tmp_path.join("config/parsec3/libparsec");
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
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, _, _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    set_env(tmp_path_str, &url);

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "create-organization",
            "--organization-id",
            unique_org_id().as_ref(),
            "--addr",
            &std::env::var(TESTBED_SERVER_URL).unwrap(),
            "--token",
            "s3cr3t",
        ])
        .assert()
        .stdout(predicates::str::contains("Organization bootstrap url:"));
}

#[rstest::rstest]
#[tokio::test]
async fn bootstrap_organization(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, _, _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    set_env(tmp_path_str, &url);

    let organization_id = unique_org_id();
    let addr = std::env::var(TESTBED_SERVER_URL).unwrap().parse().unwrap();

    log::debug!("Creating organization {organization_id}");
    let organization_addr = create_organization_req(&organization_id, &addr, "s3cr3t")
        .await
        .unwrap();

    log::debug!("Bootstrapping organization {organization_id}");
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
            "--password-stdin",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .stdout(predicates::str::contains("Organization bootstrapped"));
}

#[rstest::rstest]
#[tokio::test]
async fn invite_device(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    set_env(tmp_path_str, &url);

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "invite-device",
            "--device",
            &alice.device_id.hex(),
            "--password-stdin",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .stdout(predicates::str::contains("Invitation URL:"));
}

#[rstest::rstest]
#[tokio::test]
async fn invite_user(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    set_env(tmp_path_str, &url);

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "invite-user",
            "--device",
            &alice.device_id.hex(),
            "--email",
            "a@b.c",
            "--password-stdin",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .stdout(predicates::str::contains("Invitation URL:"));
}

#[rstest::rstest]
#[tokio::test]
async fn cancel_invitation(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(tmp_path_str, &url);

    let cmds = AuthenticatedCmds::new(
        &get_default_config_dir(),
        alice.clone(),
        ProxyConfig::new_from_env().unwrap(),
    )
    .unwrap();

    let rep = cmds
        .send(invite_new_device::Req { send_email: false })
        .await
        .unwrap();

    let invitation_addr = match rep {
        InviteNewDeviceRep::Ok { token, .. } => ParsecInvitationAddr::new(
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
            &alice.device_id.hex(),
            "--token",
            &token.hex().to_string(),
            "--password-stdin",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .stdout(predicates::str::contains("Invitation deleted"));
}

#[rstest::rstest]
#[tokio::test]
async fn stats_organization(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, _, org_id) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(tmp_path_str, &url);

    let expect = format!(
        "{:#}\n",
        serde_json::json!({
            "active_users": 3,
            "data_size": 0,
            "metadata_size": 0,
            "realms": 0,
            "users": 3,
            "users_per_profile_detail": {
                "ADMIN": {
                    "active": 1,
                    "revoked": 0
                },
                "STANDARD": {
                    "active": 1,
                    "revoked": 0
                },
                "OUTSIDER": {
                    "active": 1,
                    "revoked": 0
                }
            }
        })
    );

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "stats-organization",
            "--organization-id",
            org_id.as_ref(),
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
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    let output = tmp_path.join("recovery_device");

    set_env(tmp_path_str, &url);

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "export-recovery-device",
            "--device",
            &alice.device_id.hex(),
            "--output",
            &output.to_string_lossy(),
            "--password-stdin",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .stdout(predicates::str::contains("Saved in"));

    assert!(output.exists());
}

#[rstest::rstest]
#[tokio::test]
async fn import_recovery_device(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    let input = tmp_path.join("recovery_device");

    set_env(tmp_path_str, &url);

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
            "--password-stdin",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .stdout(predicates::str::contains("Saved new device"));
}

#[rstest::rstest]
#[tokio::test]
async fn stats_server(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, _, _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(tmp_path_str, &url);

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "stats-server",
            "--addr",
            &url.to_string(),
            "--token",
            DEFAULT_ADMINISTRATION_TOKEN,
        ])
        .unwrap();

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
            "1990-01-01T00:00:00-00:00",
        ])
        .unwrap();
}

#[rstest::rstest]
#[tokio::test]
async fn status_organization(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, _, org_id) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(tmp_path_str, &url);

    let expect = format!(
        "{:#}\n",
        serde_json::json!({
            "active_users_limit": null,
            "is_bootstrapped": true,
            "is_expired": false,
            "minimum_archiving_period": 2592000,
            "user_profile_outsider_allowed": true
        })
    );

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "status-organization",
            "--organization-id",
            org_id.as_ref(),
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
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(tmp_path_str, &url);

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "list-invitations",
            "--device",
            &alice.device_id.hex(),
            "--password-stdin",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .stdout(predicates::str::contains("No invitation."));

    let cmds = AuthenticatedCmds::new(
        &get_default_config_dir(),
        alice.clone(),
        ProxyConfig::new_from_env().unwrap(),
    )
    .unwrap();

    let rep = cmds
        .send(invite_new_device::Req { send_email: false })
        .await
        .unwrap();

    let invitation_addr = match rep {
        InviteNewDeviceRep::Ok { token, .. } => ParsecInvitationAddr::new(
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
            "list-invitations",
            "--device",
            &alice.device_id.hex(),
            "--password-stdin",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .stdout(predicates::str::contains(format!(
            "{}\t{YELLOW}idle{RESET}\tdevice",
            token.hex()
        )));
}

#[rstest::rstest]
#[tokio::test]
async fn create_workspace(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(tmp_path_str, &url);

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "create-workspace",
            "--device",
            &alice.device_id.hex(),
            "--name",
            "new-workspace",
            "--password-stdin",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .stdout(predicates::str::contains("Workspace has been created"));

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "list-workspaces",
            "--device",
            &alice.device_id.hex(),
            "--password-stdin",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .stdout(predicates::str::contains("new-workspace: owner"));
}

#[rstest::rstest]
#[tokio::test]
async fn list_users(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(tmp_path_str, &url);

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "list-users",
            "--device",
            &alice.device_id.hex(),
            "--password-stdin",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .stdout(
            predicates::str::contains(format!("Found {GREEN}3{RESET} user(s)"))
                .and(predicates::str::contains("Alice"))
                .and(predicates::str::contains("Bob"))
                .and(predicates::str::contains("Toto")),
        );
}

#[rstest::rstest]
#[tokio::test]
// This test seems to fail because alice's device ID is no longer stable (it used
// to be a string, now it's a UUID regenerated at each run), hence the test process
// and the cli invocation process have different values for `alice.device_id.hex()` !
#[ignore = "TODO: fix this test !"]
async fn share_workspace(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, _, bob], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(tmp_path_str, &url);

    // FIXME: The test should not rely on the load_client_and_run since it use the stdin to read the password to unlock the device.
    let client = load_client(
        &libparsec::get_default_config_dir(),
        Some(alice.device_id.to_string()),
        false,
    )
    .await
    .unwrap();
    let wid = client
        .create_workspace("new-workspace".parse().unwrap())
        .await
        .unwrap();
    client.ensure_workspaces_bootstrapped().await.unwrap();

    client.poll_server_for_new_certificates().await.unwrap();
    let users = client.list_users(false, None, None).await.unwrap();
    let bob_id = &users
        .iter()
        .find(|x| x.human_handle == HumanHandle::new("bob@example.com", "Bob").unwrap())
        .unwrap()
        .id;

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "share-workspace",
            "--device",
            &alice.device_id.hex(),
            "--workspace-id",
            &wid.hex(),
            "--user-id",
            &bob_id.hex(),
            "--role",
            "contributor",
        ])
        .assert()
        .stdout(predicates::str::contains("Workspace has been shared"));

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args(["list-workspaces", "--device", &bob.device_id.hex()])
        .assert()
        .stdout(predicates::str::contains("new-workspace: contributor"));
}

#[rstest::rstest]
#[tokio::test]
async fn invite_device_dance(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    set_env(tmp_path_str, &url);

    let cmds = AuthenticatedCmds::new(
        &get_default_config_dir(),
        alice.clone(),
        ProxyConfig::new_from_env().unwrap(),
    )
    .unwrap();

    let rep = cmds
        .send(invite_new_device::Req { send_email: false })
        .await
        .unwrap();

    let invitation_addr = match rep {
        InviteNewDeviceRep::Ok { token, .. } => ParsecInvitationAddr::new(
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

    let p_greeter = std::process::Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "greet-invitation",
            "--token",
            &token.hex().to_string(),
            "--device",
            &alice.device_id.hex(),
            "--password-stdin",
        ])
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let p_claimer = std::process::Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "claim-invitation",
            "--addr",
            invitation_addr.to_url().as_ref(),
            "--password-stdin",
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

    stdin_greeter
        .write_all(format!("{DEFAULT_DEVICE_PASSWORD}\n").as_bytes())
        .unwrap();
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
    stdin_claimer
        .write_all(format!("{DEFAULT_DEVICE_PASSWORD}\n").as_bytes())
        .unwrap();
    wait_for(&mut stdout_claimer, &mut buf, "Saved");
}

#[rstest::rstest]
#[tokio::test]
async fn invite_user_dance(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();
    set_env(tmp_path_str, &url);

    let cmds = AuthenticatedCmds::new(
        &get_default_config_dir(),
        alice.clone(),
        ProxyConfig::new_from_env().unwrap(),
    )
    .unwrap();

    let rep = cmds
        .send(invite_new_user::Req {
            claimer_email: "a@b.c".into(),
            send_email: false,
        })
        .await
        .unwrap();

    let invitation_addr = match rep {
        InviteNewUserRep::Ok { token, .. } => ParsecInvitationAddr::new(
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

    let p_greeter = std::process::Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "greet-invitation",
            "--token",
            &token.hex().to_string(),
            "--device",
            &alice.device_id.hex(),
            "--password-stdin",
        ])
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let p_claimer = std::process::Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "claim-invitation",
            "--addr",
            invitation_addr.to_url().as_ref(),
            "--password-stdin",
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

    stdin_greeter
        .write_all(format!("{DEFAULT_DEVICE_PASSWORD}\n").as_bytes())
        .unwrap();
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
    stdin_claimer
        .write_all(format!("{DEFAULT_DEVICE_PASSWORD}\n").as_bytes())
        .unwrap();
    wait_for(&mut stdout_claimer, &mut buf, "Saved");
}

#[rstest::rstest]
#[tokio::test]
// This test seems to fail because alice's device ID is no longer stable (it used
// to be a string, now it's a UUID regenerated at each run), hence the test process
// and the cli invocation process have different values for `alice.device_id.hex()` !
#[ignore = "TODO: fix this test !"]
async fn remove_device(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(tmp_path_str, &url);

    let process = std::process::Command::new("cargo")
        .args([
            "run",
            "--profile=ci-rust",
            "--package=parsec_cli",
            "--features=testenv",
            "remove-device",
            "--device",
            &alice.device_id.hex(),
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
        .join("config/parsec3/libparsec/devices")
        .join(format!("{}.keys", alice.device_id.hex()));

    assert!(alice_device_file.exists());

    wait_for(&mut stdout, &mut buf, "Are you sure? (y/n)");
    stdin.write_all(b"y\n").unwrap();

    wait_for(&mut stdout, &mut buf, "The device has been removed");

    assert!(!alice_device_file.exists());
}

#[rstest::rstest]
#[tokio::test]
#[ignore = "todo"]
async fn setup_shamir(tmp_path: TmpPath) {
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, bob, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(tmp_path_str, &url);

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "shamir-setup-create",
            "--device",
            &alice.device_id.hex(),
            "--recipients",
            &dbg!(bob.human_handle.email()),
        ])
        .assert()
        .stdout(predicates::str::contains("Shamir setup has been created"));
    // TODO list shamir setup
}

#[rstest::rstest]
#[tokio::test]
async fn ls_files(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(tmp_path_str, &url);

    let client = libparsec::internal::Client::start(
        Arc::new(
            ClientConfig {
                with_monitors: false,
                ..Default::default()
            }
            .into(),
        ),
        libparsec::internal::EventBus::default(),
        alice.clone(),
    )
    .await
    .unwrap();

    // Create the workspace used to copy the file to
    let wid = client
        .create_workspace("new-workspace".parse().unwrap())
        .await
        .unwrap();
    client.ensure_workspaces_bootstrapped().await.unwrap();

    let workspace = client.start_workspace(wid).await.unwrap();
    workspace
        .create_file("/test.txt".parse().unwrap())
        .await
        .unwrap();
    workspace
        .create_folder("/foo".parse().unwrap())
        .await
        .unwrap();

    // List the files
    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "ls",
            "--device",
            &alice.device_id.hex(),
            "--password-stdin",
            "--workspace-id",
            &wid.hex(),
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .success()
        .stdout(predicates::str::contains("test.txt\n").and(predicates::str::contains("foo\n")));
}
