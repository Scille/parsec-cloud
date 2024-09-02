use libparsec::{tmp_path, TmpPath};

use super::{bootstrap_cli_test, unique_org_id};
use crate::{
    create_organization::create_organization_req,
    testenv_utils::{DEFAULT_ADMINISTRATION_TOKEN, DEFAULT_DEVICE_PASSWORD, TESTBED_SERVER},
};

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
        "bootstrap-organization",
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
