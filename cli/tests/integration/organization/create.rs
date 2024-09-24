use libparsec::{tmp_path, TmpPath};

use crate::{
    integration_tests::{bootstrap_cli_test, unique_org_id},
    testenv_utils::{DEFAULT_ADMINISTRATION_TOKEN, TESTBED_SERVER},
};

#[rstest::rstest]
#[tokio::test]
async fn create_organization(tmp_path: TmpPath) {
    bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        "organization",
        "create",
        "--organization-id",
        unique_org_id().as_ref(),
        "--addr",
        &std::env::var(TESTBED_SERVER).unwrap(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN
    )
    .stdout(predicates::str::contains("Organization bootstrap url:"));
}
