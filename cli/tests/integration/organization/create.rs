// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{TmpPath, tmp_path};
use parsec_cli::ui::Ui;

use crate::{
    bootstrap_cli_test, test_ui,
    testenv_utils::{DEFAULT_ADMINISTRATION_TOKEN, TESTBED_SERVER},
    unique_org_id,
};
use predicates::PredicateBoxExt;

#[rstest::rstest]
#[case::plain("plain",
        predicates::str::is_match("^Organization bootstrap URL: .*https?://.*/redirect/.*\\?a=bootstrap_organization&.*\n$")
            .unwrap().boxed(),
)]
#[case::json("json",
    predicates::str::is_match("^\"https?://.*/redirect/.*\\?a=bootstrap_organization&.*\"\n$").unwrap().boxed()
)]
#[tokio::test]
async fn create_organization(
    tmp_path: TmpPath,
    test_ui: &Ui,
    #[case] format: &str,
    #[case] pred: predicates::BoxPredicate<str>,
) {
    bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        "--format",
        format,
        "organization",
        "create",
        unique_org_id().as_ref(),
        "--addr",
        std::env::var(TESTBED_SERVER).unwrap(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN
    )
    .stdout(pred);
}
