use libparsec::{tmp_path, RealmRole, TmpPath};
use predicates::boolean::PredicateBooleanExt;

use crate::{
    bootstrap_cli_test, test_ui,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};
use parsec_cli::{ui::Ui, utils::start_client};

#[rstest::rstest]
#[tokio::test]
async fn list_users(tmp_path: TmpPath, test_ui: &Ui) {
    let (
        _,
        TestOrganization {
            alice, bob, toto, ..
        },
        _,
    ) = bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

    let client = start_client(alice).await.unwrap();

    let wid = client
        .create_workspace("test".try_into().unwrap())
        .await
        .unwrap();
    client
        .share_workspace(wid, toto.user_id, Some(RealmRole::Reader))
        .await
        .unwrap();

    client
        .share_workspace(wid, bob.user_id, Some(RealmRole::Manager))
        .await
        .unwrap();
    client.stop().await;

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "workspace",
        "list-users",
        "--device",
        &bob.device_id.hex(),
        "--workspace",
        &wid.hex()
    )
    .stderr(predicates::str::contains(format!(
        "Workspace {wid} is shared with 3 user(s)"
    )))
    .stdout(
        predicates::str::contains("(ADMIN) - Alice (alice@example.com) has role owner")
            .and(predicates::str::contains(
                "(STANDARD) - Bob (bob@example.com) has role manager",
            ))
            .and(predicates::str::contains(
                "(OUTSIDER) - Toto (toto@example.com) has role reader",
            )),
    );
}
