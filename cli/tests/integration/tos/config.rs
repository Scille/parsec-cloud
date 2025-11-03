use std::collections::HashMap;

use libparsec::{tmp_path, ClientGetTosError, TmpPath};

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_ADMINISTRATION_TOKEN},
};
use parsec_cli::{
    commands::tos::config::{config_tos_for_org_req, TosReq},
    utils::start_client,
};

#[rstest::rstest]
#[tokio::test]
async fn test_set_tos_from_arg(tmp_path: TmpPath) {
    let (addr, TestOrganization { alice, .. }, organization) =
        bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        "tos",
        "config",
        "--organization",
        organization.as_ref(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN,
        "--addr",
        &addr.to_string(),
        "fr_fr=http://example.com/tos",
        "en_CA=http://example.com/en/tos"
    )
    .stdout(predicates::str::is_empty());

    let client = start_client(alice).await.unwrap();

    let tos = client.get_tos().await.unwrap();

    assert_eq!(
        tos.per_locale_urls,
        HashMap::from_iter([
            ("fr_fr".into(), "http://example.com/tos".into()),
            ("en_CA".into(), "http://example.com/en/tos".into()),
        ])
    );
}

#[rstest::rstest]
#[tokio::test]
async fn test_set_tos_from_file(tmp_path: TmpPath) {
    let (addr, TestOrganization { alice, .. }, organization) =
        bootstrap_cli_test(&tmp_path).await.unwrap();

    let expected_tos = HashMap::from_iter([
        ("fr_fr".into(), "http://example.com/tos".into()),
        ("en_CA".into(), "http://example.com/en/tos".into()),
    ]);
    let tos_file = tmp_path.join("tos.json");
    tokio::fs::write(&tos_file, serde_json::to_vec(&expected_tos).unwrap())
        .await
        .unwrap();

    crate::assert_cmd_success!(
        "tos",
        "config",
        "--organization",
        organization.as_ref(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN,
        "--addr",
        &addr.to_string(),
        "--from-json",
        &tos_file.display().to_string()
    )
    .stdout(predicates::str::is_empty());

    let client = start_client(alice).await.unwrap();

    let tos = client.get_tos().await.unwrap();

    assert_eq!(tos.per_locale_urls, expected_tos);
}

#[rstest::rstest]
#[tokio::test]
async fn test_remove_tos(tmp_path: TmpPath) {
    let (addr, TestOrganization { alice, .. }, organization) =
        bootstrap_cli_test(&tmp_path).await.unwrap();

    config_tos_for_org_req(
        &addr,
        DEFAULT_ADMINISTRATION_TOKEN,
        &organization,
        TosReq::set_tos(HashMap::from_iter([("fr_fr", "http://parsec.local/tos")])),
    )
    .await
    .unwrap();

    let client = start_client(alice).await.unwrap();
    let tos = client.get_tos().await.unwrap();

    assert!(!tos.per_locale_urls.is_empty());

    crate::assert_cmd_success!(
        "tos",
        "config",
        "--organization",
        organization.as_ref(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN,
        "--addr",
        &addr.to_string(),
        "--remove"
    )
    .stdout(predicates::str::is_empty());

    let err = client.get_tos().await.unwrap_err();

    assert!(matches!(err, ClientGetTosError::NoTos));
}

#[tokio::test]
async fn test_invalid_tos_arg() {
    crate::assert_cmd_failure!(
        "tos",
        "config",
        "--organization=foobar",
        "--token=123456",
        "--addr=parsec3://example.com",
        "fr_fr"
    )
    .stderr(predicates::str::contains(
        "Missing '=<URL>' in argument ('fr_fr')",
    ));
}

#[rstest::rstest]
#[tokio::test]
async fn test_org_not_found(tmp_path: TmpPath) {
    let (addr, _, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_failure!(
        "tos",
        "config",
        "--organization=foobar",
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN,
        "--addr",
        &addr.to_string(),
        "--remove"
    )
    .stderr(predicates::str::contains("Organization foobar not found"));
}
