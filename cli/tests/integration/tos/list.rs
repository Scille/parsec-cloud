use std::collections::HashMap;

use libparsec::{tmp_path, TmpPath};
use predicates::prelude::PredicateBooleanExt;

use crate::{
    commands::tos::config::{config_tos_for_org_req, TosReq},
    integration_tests::bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_ADMINISTRATION_TOKEN, DEFAULT_DEVICE_PASSWORD},
};

#[rstest::rstest]
#[tokio::test]
async fn list_no_tos_available(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "tos",
        "list",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains("No Terms of Service available"));
}

#[rstest::rstest]
#[tokio::test]
async fn list_tos_ok(tmp_path: TmpPath) {
    let (addr, TestOrganization { alice, .. }, organization) =
        bootstrap_cli_test(&tmp_path).await.unwrap();

    let tos = HashMap::from_iter([
        ("fr_FR", "http://example.com/tos"),
        ("en_DK", "http://example.com/en/tos"),
    ]);
    config_tos_for_org_req(
        &addr,
        DEFAULT_ADMINISTRATION_TOKEN,
        &organization,
        TosReq::set_tos(tos),
    )
    .await
    .unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "tos",
        "list",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(
        predicates::str::contains("Terms of Service updated on")
            .and(predicates::str::contains("- fr_FR: http://example.com/tos"))
            .and(predicates::str::contains(
                "- en_DK: http://example.com/en/tos",
            )),
    );
}
