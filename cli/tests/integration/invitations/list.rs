use libparsec::{
    authenticated_cmds::latest::invite_new_device, get_default_config_dir, tmp_path, AccessToken,
    AuthenticatedCmds, ProxyConfig, TmpPath,
};
use predicates::prelude::PredicateBooleanExt;

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};
use parsec_cli::utils::{RESET, YELLOW};

async fn invite_device(cmds: &AuthenticatedCmds) -> AccessToken {
    let rep = cmds
        .send(invite_new_device::Req { send_email: false })
        .await
        .unwrap();

    match rep {
        invite_new_device::InviteNewDeviceRep::Ok { token, .. } => token,
        rep => {
            panic!("Server refused to create device invitation: {rep:?}");
        }
    }
}

#[rstest::rstest]
#[tokio::test]
async fn list_invitations(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let cmds = AuthenticatedCmds::new(
        &get_default_config_dir(),
        alice.clone(),
        ProxyConfig::new_from_env().unwrap(),
    )
    .unwrap();

    let token = invite_device(&cmds).await;

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "invite",
        "list",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains(format!(
        "{}\t{YELLOW}pending{RESET}\tdevice",
        token.hex()
    )));
}

#[rstest::rstest]
#[tokio::test]
async fn issue_9176_list_more_than_one_invitations(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let cmds = AuthenticatedCmds::new(
        &get_default_config_dir(),
        alice.clone(),
        ProxyConfig::new_from_env().unwrap(),
    )
    .unwrap();

    let token1 = invite_device(&cmds).await;
    let token2 = invite_device(&cmds).await;
    let token3 = invite_device(&cmds).await;

    let contains_invite = |token: AccessToken| {
        predicates::str::contains(format!("{token}\t{YELLOW}pending{RESET}\tdevice"))
    };

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "invite",
        "list",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(
        contains_invite(token1)
            .and(contains_invite(token2))
            .and(contains_invite(token3)),
    );
}

#[rstest::rstest]
#[tokio::test]
async fn no_invitations(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "invite",
        "list",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains("No invitation."));
}
