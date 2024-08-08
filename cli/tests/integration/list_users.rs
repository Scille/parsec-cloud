use assert_cmd::Command;

use libparsec::{tmp_path, TmpPath};
use predicates::prelude::PredicateBooleanExt;

use super::bootstrap_cli_test;
use crate::{
    testenv_utils::DEFAULT_DEVICE_PASSWORD,
    utils::{GREEN, RESET},
};

#[rstest::rstest]
#[tokio::test]
async fn list_users(tmp_path: TmpPath) {
    let (_, [alice, ..], _) = bootstrap_cli_test(&tmp_path).await.unwrap();

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
