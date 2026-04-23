use libparsec::{tmp_path, RealmRole, TmpPath};

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
    GREEN, RESET, YELLOW,
};
use parsec_cli::utils::start_client;

#[rstest::rstest]
#[tokio::test]
async fn list_users(tmp_path: TmpPath) {
    let (
        _,
        TestOrganization {
            alice, bob, toto, ..
        },
        _,
    ) = bootstrap_cli_test(&tmp_path).await.unwrap();

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
    .stdout(predicates::str::contains(format!("Workspace {wid} is shared with {GREEN}3{RESET} user(s)")))
    .stdout(predicates::str::contains(format!("({YELLOW}ADMIN{RESET}) - {GREEN}Alice{RESET} (alice@example.com) has role {GREEN}owner{RESET}",)))
    .stdout(predicates::str::contains(format!("({YELLOW}STANDARD{RESET}) - {GREEN}Bob{RESET} (bob@example.com) has role {GREEN}manager{RESET}",)))
    .stdout(predicates::str::contains(format!("({YELLOW}OUTSIDER{RESET}) - {GREEN}Toto{RESET} (toto@example.com) has role {GREEN}reader{RESET}",)
    ));
}
