use libparsec::{tmp_path, TmpPath};
use parsec_cli::ui::Ui;

use crate::{
    bootstrap_cli_test, test_ui,
    testenv_utils::{DEFAULT_ADMINISTRATION_TOKEN, TESTBED_SERVER},
    unique_org_id,
};

#[rstest::rstest]
#[tokio::test]
async fn create_organization(tmp_path: TmpPath, test_ui: &Ui) {
    bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        "organization",
        "create",
        unique_org_id().as_ref(),
        "--addr",
        std::env::var(TESTBED_SERVER).unwrap(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN
    )
    .stdout(
        predicates::str::is_match("Organization bootstrap URL: .*https?://.*/redirect/.*").unwrap(),
    );
}
