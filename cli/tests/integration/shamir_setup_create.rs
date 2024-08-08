use assert_cmd::Command;

use libparsec::{tmp_path, TmpPath};

use super::bootstrap_cli_test;

#[rstest::rstest]
#[tokio::test]
#[ignore = "todo"]
async fn setup_shamir(tmp_path: TmpPath) {
    let (_, [alice, bob, ..], _) = bootstrap_cli_test(&tmp_path).await.unwrap();

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
