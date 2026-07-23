#[test]
fn version() {
    crate::assert_cmd_success!("--version").stdout(
        // Using `concat!` simplify updating the version using `version-updater`
        concat!("parsec-cli 3.9.3-a.0.dev.20657+a5aa7a6", "\n"),
    );
}
