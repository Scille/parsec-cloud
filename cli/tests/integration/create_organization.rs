use libparsec::{tmp_path, TmpPath};

use super::{bootstrap_cli_test, unique_org_id};
use crate::testenv_utils::{DEFAULT_ADMINISTRATION_TOKEN, TESTBED_SERVER_URL};

#[rstest::rstest]
#[tokio::test]
async fn create_organization(tmp_path: TmpPath) {
    bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        "create-organization",
        "--organization-id",
        unique_org_id().as_ref(),
        "--addr",
        &std::env::var(TESTBED_SERVER_URL).unwrap(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN
    )
    .stdout(predicates::str::contains("Organization bootstrap url:"));
}