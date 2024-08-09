use assert_cmd::Command;

use libparsec::{tmp_path, TmpPath};

use super::{get_testenv_config, run_local_organization, set_env};

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
