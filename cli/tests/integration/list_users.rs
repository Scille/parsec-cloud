use assert_cmd::Command;

use libparsec::{tmp_path, TmpPath};
use predicates::prelude::PredicateBooleanExt;

use super::{get_testenv_config, run_local_organization, set_env};
use crate::{
    testenv_utils::DEFAULT_DEVICE_PASSWORD,
    utils::{GREEN, RESET},
};

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
