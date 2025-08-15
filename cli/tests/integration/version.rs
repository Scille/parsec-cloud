#[test]
fn version() {
    crate::assert_cmd_success!("--version").stdout(
        // Using `concat!` simplify updating the version using `version-updater`
        concat!("parsec-cli 3.5.0-a.6.dev.20315+a1f1b2b", "\n"),
    );
}
