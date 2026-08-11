use libparsec::{tmp_path, TmpPath};
use predicates::prelude::PredicateBooleanExt;

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};

#[rstest::rstest]
#[tokio::test]
async fn list_users(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "user",
        "list",
        "--device",
        &alice.device_id.hex()
    )
    .stderr(predicates::str::contains("Found 3 user(s)"))
    .stdout(
        predicates::str::contains("Alice")
            .and(predicates::str::contains("Bob"))
            .and(predicates::str::contains("Toto")),
    );
}
