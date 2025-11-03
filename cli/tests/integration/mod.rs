#![allow(clippy::unwrap_used)]

mod certificate;
mod device;
mod device_option;
mod invitations;
mod ls;
mod mount_realm_export;
mod organization;
mod rm;
mod server;
mod shared_recovery;
mod tos;
mod user;
mod version;
mod workspace;

use std::{
    io::BufRead,
    path::{Path, PathBuf},
};

use libparsec::LocalDevice;
use libparsec::{
    ClientConfig, OrganizationID, ParsecAddr, TmpPath, PARSEC_BASE_CONFIG_DIR,
    PARSEC_BASE_DATA_DIR, PARSEC_BASE_HOME_DIR,
};
use parsec_cli::utils::{GREEN, RED, RESET};
use std::sync::Arc;

pub use parsec_cli::testenv_utils;
use parsec_cli::testenv_utils::{
    initialize_test_organization, new_environment, parsec_addr_from_http_url, TestOrganization,
    TestenvConfig, DEFAULT_DEVICE_PASSWORD, TESTBED_SERVER,
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
    std::env::set_var(TESTBED_SERVER, url.to_url().to_string());
    std::env::set_var(PARSEC_BASE_HOME_DIR, format!("{tmp_dir}/cache"));
    std::env::set_var(PARSEC_BASE_DATA_DIR, format!("{tmp_dir}/share"));
    std::env::set_var(PARSEC_BASE_CONFIG_DIR, format!("{tmp_dir}/config"));
    // Hidden environ variable only used for CLI testing to customize the throttling time
    // in invitation polling, without that the tests would be much slower for no reason.
    std::env::set_var("_PARSEC_INVITE_POLLING_THROTTLE_MS", "10");

    // Remove the PARSEC_* variables that will conflict with the CLI tests
    std::env::remove_var("PARSEC_DEVICE_ID");
    std::env::remove_var("PARSEC_WORKSPACE_ID");
    std::env::remove_var("PARSEC_ORGANISATION_ID");
    std::env::remove_var("PARSEC_SERVER_ADDR");
    std::env::remove_var("PARSEC_ADMINISTRATION_TOKEN");
    std::env::remove_var("PARSEC_CONFIG_DIR");
    std::env::remove_var("PARSEC_DATA_DIR");
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
) -> anyhow::Result<(ParsecAddr, TestOrganization, OrganizationID)> {
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
    let mut pos = buf.len();
    while reader.read_line(buf).is_ok() {
        let new = &buf[pos..];
        if new.is_empty() || new.contains(text) {
            break;
        }
        pos = buf.len();
    }
}

async fn bootstrap_cli_test(
    tmp_path: &TmpPath,
) -> anyhow::Result<(ParsecAddr, TestOrganization, OrganizationID)> {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, devices, org_id) = run_local_organization(tmp_path, None, config).await?;

    set_env(tmp_path_str, &url);
    Ok((url, devices, org_id))
}

#[macro_export]
macro_rules! assert_cmd_success {
    (with_password=$pass:expr, $($cmd:expr),+) => {
        $crate::assert_cmd!(with_password=$pass, $($cmd),+)
            .assert()
            .success()
    };
    ($($cmd:expr),+) => {
        $crate::assert_cmd!($($cmd),+)
            .assert()
            .success()
    }
}

#[macro_export]
macro_rules! assert_cmd {
    (with_password=$pass:expr, $($cmd:expr),+) => {
        $crate::assert_cmd!($($cmd),+ , "--password-stdin")
            .write_stdin(format!("{password}\n", password = $pass))
    };
    ($($cmd:expr),+) => {
        assert_cmd::cargo::cargo_bin_cmd!("parsec-cli")
            .args([$($cmd),+])
    };
}

#[macro_export]
macro_rules! assert_cmd_failure {
    (with_password=$pass:expr, $($cmd:expr),+) => {
        $crate::assert_cmd!(with_password=$pass, $($cmd),+)
            .assert()
            .failure()
    };
    ($($cmd:expr),+) => {
        $crate::assert_cmd!($($cmd),+)
            .assert()
            .failure()
    }
}

/// Creates a std `Command` to be run with the `parsec-cli` binary.
#[macro_export]
macro_rules! std_cmd {
    ($($args:expr),*) => {
        {
            let mut command = std::process::Command::new(assert_cmd::cargo::cargo_bin!("parsec-cli"));
            command.args([$($args),*]);
            command
        }
    }
}

/// For Alice, with Bob and Toto as recipients
fn shared_recovery_create(
    alice: &Arc<LocalDevice>,
    bob: &Arc<LocalDevice>,
    toto: Option<&Arc<LocalDevice>>,
) {
    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "info",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains(format!(
        "Shared recovery {RED}never setup{RESET}"
    )));

    if let Some(toto) = toto {
        crate::assert_cmd_success!(
            with_password = DEFAULT_DEVICE_PASSWORD,
            "shared-recovery",
            "create",
            "--device",
            &alice.device_id.hex(),
            "--recipients",
            &bob.human_handle.email().to_string(),
            &toto.human_handle.email().to_string(),
            "--weights",
            "1",
            "1",
            "--threshold",
            "1",
            "--no-confirmation"
        )
        .stdout(predicates::str::contains(
            "Shared recovery setup has been created",
        ));
    } else {
        crate::assert_cmd_success!(
            with_password = DEFAULT_DEVICE_PASSWORD,
            "shared-recovery",
            "create",
            "--device",
            &alice.device_id.hex(),
            "--recipients",
            &bob.human_handle.email().to_string(),
            "--weights",
            "1",
            "--threshold",
            "1",
            "--no-confirmation"
        )
        .stdout(predicates::str::contains(
            "Shared recovery setup has been created",
        ));
    }

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "info",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains(format!(
        "Shared recovery {GREEN}set up{RESET}"
    )));
}

#[cfg(target_family = "unix")] // rexpect doesn't support Windows
/// Spawns `command` in the background with the given `timeout`
/// and returns a session to interact with the started process.
fn spawn_interactive_command(
    command: std::process::Command,
    timeout: Option<u64>,
) -> Result<rexpect::session::PtySession, rexpect::error::Error> {
    rexpect::spawn_with_options(
        command,
        rexpect::reader::Options {
            // On CI we disable the timeout to avoid flakiness,
            // If a test is too slow, it will be killed by the timeout set on the CI
            timeout_ms: if std::option_env!("CI") == Some("true") {
                None
            } else {
                timeout
            },
            strip_ansi_escape_codes: true,
        },
    )
}
