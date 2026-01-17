#[test]
fn version() {
    crate::assert_cmd_success!("--version").stdout(
        // Using `concat!` simplify updating the version using `version-updater`
        concat!("parsec-cli 3.7.2-a.0.dev.20470+8c87f1d", "\n"),
    );
}
