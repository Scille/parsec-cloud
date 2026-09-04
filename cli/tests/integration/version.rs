use predicates::boolean::PredicateBooleanExt;

#[rstest::rstest]
#[case::short("-V")]
#[case::long("--version")]
fn version(#[case] version_arg: &str) {
    crate::assert_cmd_success!(version_arg).stdout(
        predicates::str::contains(
            // Using `concat!` simplify updating the version using `version-updater`
            concat!("parsec-cli 3.9.4-a.0.dev.20700+72e13e8", "\n"),
        )
        .and(predicates::str::is_match("branch: .*").unwrap())
        .and(predicates::str::is_match("commit_hash: .*").unwrap())
        .and(predicates::str::is_match("build_target: .*").unwrap())
        .and(predicates::str::is_match("build_os: .*").unwrap())
        .and(predicates::str::is_match("build_profile: .*").unwrap())
        .and(predicates::str::is_match("cli_features: .*").unwrap())
        .and(predicates::str::is_match("crypto_backend: .*").unwrap())
        .and(predicates::str::is_match("rust_version: .*").unwrap())
        .and(predicates::str::is_match("rust_channel: .*").unwrap())
        .and(predicates::str::is_match("cargo_version: .*").unwrap())
        .and(predicates::str::is_match("git_clean: .*").unwrap()),
    );
}
