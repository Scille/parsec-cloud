use std::collections::HashMap;

use libparsec::{tmp_path, OrganizationID, ParsecAddr, TmpPath};
use predicates::prelude::PredicateBooleanExt;

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_ADMINISTRATION_TOKEN, DEFAULT_DEVICE_PASSWORD},
};
use parsec_cli::{
    commands::tos::config::{config_tos_for_org_req, TosReq},
    utils::BULLET_CHAR,
};

async fn config_tos(
    addr: &ParsecAddr,
    org_id: &OrganizationID,
) -> HashMap<&'static str, &'static str> {
    let tos = HashMap::from_iter([
        ("fr_FR", "http://example.com/tos"),
        ("en_DK", "http://example.com/en/tos"),
    ]);
    config_tos_for_org_req(
        addr,
        DEFAULT_ADMINISTRATION_TOKEN,
        org_id,
        TosReq::set_tos(tos.clone()),
    )
    .await
    .unwrap();
    tos
}

#[rstest::rstest]
#[tokio::test]
async fn test_accept_tos(tmp_path: TmpPath) {
    let (addr, TestOrganization { alice, .. }, organization) =
        bootstrap_cli_test(&tmp_path).await.unwrap();
    let tos = config_tos(&addr, &organization).await;

    crate::assert_cmd!(
        "tos",
        "accept",
        "--device",
        &alice.device_id.hex(),
        "--password-stdin"
    )
    .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\ny\n"))
    .assert()
    .success()
    .stdout(
        predicates::str::contains("Terms of Service updated on")
            .and(predicates::str::contains(format!(
                "{BULLET_CHAR} fr_FR: {}",
                tos["fr_FR"]
            )))
            .and(predicates::str::contains(format!(
                "{BULLET_CHAR} en_DK: {}",
                tos["en_DK"]
            )))
            .and(predicates::str::contains(
                "Do you accept these terms of service? (y/N)",
            )),
    );
}

#[rstest::rstest]
#[tokio::test]
async fn tldr_skip_with_yes(tmp_path: TmpPath) {
    let (addr, TestOrganization { alice, .. }, organization) =
        bootstrap_cli_test(&tmp_path).await.unwrap();
    config_tos(&addr, &organization).await;

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "tos",
        "accept",
        "--yes",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::is_empty());
}

#[rstest::rstest]
#[tokio::test]
async fn no_tos(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "tos",
        "accept",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains("No Terms of Service available"));
}

#[rstest::rstest]
#[tokio::test]
async fn did_not_accept_tos(#[values("no", "No", "NO", "S")] reply: &str, tmp_path: TmpPath) {
    let (addr, TestOrganization { alice, .. }, organization) =
        bootstrap_cli_test(&tmp_path).await.unwrap();
    let tos = config_tos(&addr, &organization).await;

    crate::assert_cmd!(
        "tos",
        "accept",
        "--device",
        &alice.device_id.hex(),
        "--password-stdin"
    )
    .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n{reply}\n"))
    .assert()
    .failure()
    .stdout(
        predicates::str::contains("Terms of Service updated on")
            .and(predicates::str::contains(format!(
                "{BULLET_CHAR} fr_FR: {}",
                tos["fr_FR"]
            )))
            .and(predicates::str::contains(format!(
                "{BULLET_CHAR} en_DK: {}",
                tos["en_DK"]
            ))),
    )
    .stderr(predicates::str::contains("Operation cancelled"));
}
