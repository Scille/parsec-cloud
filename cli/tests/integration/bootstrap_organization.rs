use assert_cmd::Command;

use libparsec::{tmp_path, TmpPath};

use super::{get_testenv_config, run_local_organization, set_env, unique_org_id};
use crate::{
    create_organization::create_organization_req,
    testenv_utils::{DEFAULT_DEVICE_PASSWORD, TESTBED_SERVER_URL},
};

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
