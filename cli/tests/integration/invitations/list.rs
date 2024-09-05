use libparsec::{
    authenticated_cmds::v4::invite_new_device, get_default_config_dir, tmp_path, AuthenticatedCmds,
    InvitationType, ParsecInvitationAddr, ProxyConfig, TmpPath,
};

use crate::tests::bootstrap_cli_test;
use crate::{
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
    utils::{RESET, YELLOW},
};

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

    let rep = cmds
        .send(invite_new_device::Req { send_email: false })
        .await
        .unwrap();

    let invitation_addr = match rep {
        invite_new_device::InviteNewDeviceRep::Ok { token, .. } => ParsecInvitationAddr::new(
            alice.organization_addr.clone(),
            alice.organization_id().clone(),
            InvitationType::Device,
            token,
        ),
        rep => {
            panic!("Server refused to create device invitation: {rep:?}");
        }
    };

    let token = invitation_addr.token();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "invite",
        "list",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains(format!(
        "{}\t{YELLOW}idle{RESET}\tdevice",
        token.hex()
    )));
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
