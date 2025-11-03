use libparsec::{tmp_path, SequesterKeySize, SequesterSigningKeyDer, TmpPath};

use crate::{
    bootstrap_cli_test,
    testenv_utils::{DEFAULT_ADMINISTRATION_TOKEN, DEFAULT_DEVICE_PASSWORD, TESTBED_SERVER},
    unique_org_id,
};
use parsec_cli::commands::organization::create::create_organization_req;

#[rstest::rstest]
#[tokio::test]
async fn bootstrap_organization(tmp_path: TmpPath) {
    bootstrap_cli_test(&tmp_path).await.unwrap();

    let organization_id = unique_org_id();
    let addr = std::env::var(TESTBED_SERVER).unwrap().parse().unwrap();

    log::debug!("Creating organization {organization_id}");
    let organization_addr =
        create_organization_req(&organization_id, &addr, DEFAULT_ADMINISTRATION_TOKEN)
            .await
            .unwrap();

    log::debug!("Bootstrapping organization {organization_id}");
    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "organization",
        "bootstrap",
        "--addr",
        &organization_addr.to_string(),
        "--device-label",
        "pc",
        "--label",
        "Alice",
        "--email",
        "alice@example.com"
    )
    .stdout(predicates::str::contains("Organization bootstrapped"));
}

#[rstest::rstest]
#[tokio::test]
async fn bootstrap_organization_with_sequester(tmp_path: TmpPath) {
    bootstrap_cli_test(&tmp_path).await.unwrap();

    // write verify key to file
    let (_private_key, verify_key) =
        SequesterSigningKeyDer::generate_pair(SequesterKeySize::_1024Bits);
    let path = tmp_path.join("verify_key.pem");
    tokio::fs::write(&path, &verify_key.dump_pem())
        .await
        .unwrap();

    let organization_id = unique_org_id();
    let addr = std::env::var(TESTBED_SERVER).unwrap().parse().unwrap();

    log::debug!("Creating organization {organization_id}");
    let organization_addr =
        create_organization_req(&organization_id, &addr, DEFAULT_ADMINISTRATION_TOKEN)
            .await
            .unwrap();

    log::debug!("Bootstrapping organization {organization_id}");
    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "organization",
        "bootstrap",
        "--addr",
        &organization_addr.to_string(),
        "--device-label",
        "pc",
        "--label",
        "Alice",
        "--email",
        "alice@example.com",
        "--sequester-key",
        &path.to_string_lossy()
    )
    .stdout(predicates::str::contains("Organization bootstrapped"));
}
