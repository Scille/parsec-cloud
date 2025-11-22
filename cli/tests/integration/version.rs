#[test]
fn version() {
    crate::assert_cmd_success!("--version").stdout(
        // Using `concat!` simplify updating the version using `version-updater`
        concat!("parsec-cli 3.6.2-a.0.dev.20414+2abed6f", "\n"),
    );
}
