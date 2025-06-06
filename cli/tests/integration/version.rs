#[test]
fn version() {
    crate::assert_cmd_success!("--version").stdout(
        // Using `concat!` simplify updating the version using `version-updater`
        concat!("parsec-cli 3.4.0-a.7.dev.20245+e6718a7", "\n"),
    );
}
