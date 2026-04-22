use std::collections::HashMap;

use libparsec::{tmp_path, OrganizationID, ParsecAddr, TmpPath};

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_ADMINISTRATION_TOKEN, DEFAULT_DEVICE_PASSWORD},
};
use parsec_cli::commands::tos::config::{config_tos_for_org_req, TosReq};

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

    let cmd = crate::std_cmd!(
        "tos",
        "accept",
        "--device",
        &alice.device_id.hex(),
        "--password-stdin"
    );
    let mut p = crate::spawn_interactive_command(cmd, Some(1500)).unwrap();
    p.send_line(DEFAULT_DEVICE_PASSWORD).unwrap();

    p.exp_string("Terms of Service updated on").unwrap();
    p.exp_string(&format!("en_DK: {}", tos["en_DK"])).unwrap();
    p.exp_string(&format!("fr_FR: {}", tos["fr_FR"])).unwrap();

    p.exp_regex(".*Do you accept these terms of service?.*")
        .unwrap();
    p.send_line("y").unwrap();
    p.exp_eof().unwrap();
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
async fn did_not_accept_tos(#[values("no", "No", "NO")] reply: &str, tmp_path: TmpPath) {
    let (addr, TestOrganization { alice, .. }, organization) =
        bootstrap_cli_test(&tmp_path).await.unwrap();
    let tos = config_tos(&addr, &organization).await;

    let cmd = crate::std_cmd!(
        "tos",
        "accept",
        "--device",
        &alice.device_id.hex(),
        "--password-stdin"
    );
    let mut p = crate::spawn_interactive_command(cmd, Some(1500)).unwrap();
    p.send_line(DEFAULT_DEVICE_PASSWORD).unwrap();

    p.exp_string("Terms of Service updated on").unwrap();
    p.exp_string(&format!("en_DK: {}", tos["en_DK"])).unwrap();
    p.exp_string(&format!("fr_FR: {}", tos["fr_FR"])).unwrap();

    p.exp_regex(".*Do you accept these terms of service?.*")
        .unwrap();

    p.send_line(reply).unwrap();
    p.exp_regex("Operation cancelled").unwrap();
}
