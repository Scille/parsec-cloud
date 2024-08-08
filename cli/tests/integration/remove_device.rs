use std::io::{BufReader, Write};

use libparsec::{tmp_path, TmpPath};

use super::{get_testenv_config, run_local_organization, set_env, wait_for};

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

    // TODO: Replace me with `assert_cmd::Command::cargo_bin` !
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
