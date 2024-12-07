#[test]
fn version() {
    crate::assert_cmd_success!("--version").stdout(
        // Using `concat!` simplify updating the version using `version-updater`
        concat!("parsec-cli 3.2.1-a.0.dev.20064+8ff7b4d", "\n"),
    );
}
