#[test]
fn version() {
    crate::assert_cmd_success!("--version").stdout(
        // Using `concat!` simplify updating the version using `version-updater`
        concat!("parsec_cli 3.0.0-b.12+dev", "\n"),
    );
}
