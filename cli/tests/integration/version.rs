#[test]
fn version() {
    crate::assert_cmd_success!("--version").stdout(
        // Using `concat!` simplify updating the version using `version-updater`
        concat!("parsec-cli 3.4.0-a.7.dev.20231+edb2da1", "\n"),
    );
}
