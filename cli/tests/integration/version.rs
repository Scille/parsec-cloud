#[test]
fn version() {
    crate::assert_cmd_success!("--version").stdout(
        // Using `concat!` simplify updating the version using `version-updater`
        concat!("parsec-cli 3.2.4-rc.3+dev", "\n"),
    );
}
