use libparsec::{tmp_path, TmpPath};

use crate::testenv_utils::TestOrganization;

use super::bootstrap_cli_test;

#[rstest::rstest]
#[tokio::test]
#[ignore = "todo"]
async fn setup_shamir(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, bob, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        "shamir-setup-create",
        "--device",
        &alice.device_id.hex(),
        "--recipients",
        &dbg!(bob.human_handle.email()),
        "--threshold",
        "1"
    )
    .stdout(predicates::str::contains("Shamir setup has been created"));
    // TODO list shamir setup
}
