#[test]
fn version() {
    crate::assert_cmd_success!("--version").stdout(
        // Using `concat!` simplify updating the version using `version-updater`
        concat!("parsec-cli 3.5.1-a.0.dev.20371+6378ac2", "\n"),
    );
}
