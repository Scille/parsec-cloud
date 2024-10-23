mod device;
mod device_option;
mod invitations;
mod ls;
mod organization;
mod rm;
mod server;
mod shamir_setup_create;
mod tos;
mod user;
mod version;
mod workspace;

use std::{
    io::BufRead,
    path::{Path, PathBuf},
};

use libparsec::{
    ClientConfig, OrganizationID, ParsecAddr, TmpPath, PARSEC_BASE_CONFIG_DIR,
    PARSEC_BASE_DATA_DIR, PARSEC_BASE_HOME_DIR,
};

use crate::testenv_utils::{
    initialize_test_organization, new_environment, parsec_addr_from_http_url, TestOrganization,
    TestenvConfig, TESTBED_SERVER,
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
    buf.clear();
    while reader.read_line(buf).is_ok() {
        if buf.is_empty() || buf.contains(text) {
            break;
        }
        buf.clear();
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
        assert_cmd::Command::cargo_bin("parsec-cli")
            .unwrap()
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
