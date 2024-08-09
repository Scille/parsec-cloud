use assert_cmd::Command;

use libparsec::{tmp_path, TmpPath};

use super::{get_testenv_config, run_local_organization, set_env};
use crate::testenv_utils::DEFAULT_DEVICE_PASSWORD;

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
