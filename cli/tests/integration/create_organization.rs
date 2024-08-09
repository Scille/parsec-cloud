use assert_cmd::Command;

use libparsec::{tmp_path, TmpPath};

use super::{get_testenv_config, run_local_organization, set_env, unique_org_id};
use crate::testenv_utils::{DEFAULT_ADMINISTRATION_TOKEN, TESTBED_SERVER_URL};

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
            DEFAULT_ADMINISTRATION_TOKEN,
        ])
        .assert()
        .stdout(predicates::str::contains("Organization bootstrap url:"));
}
