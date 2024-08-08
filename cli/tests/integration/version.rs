use assert_cmd::Command;

#[test]
fn version() {
    Command::cargo_bin("parsec_cli")
        .unwrap()
        .arg("--version")
        .assert()
        .stdout(
            // Using `concat!` simplify updating the version using `version-updater`
            concat!("parsec_cli 3.0.0-rc.1+dev", "\n"),
        );
}
