use assert_cmd::Command;

use libparsec::{
    authenticated_cmds::v4::invite_new_device, get_default_config_dir, tmp_path, AuthenticatedCmds,
    InvitationType, ParsecInvitationAddr, ProxyConfig, TmpPath,
};

use super::bootstrap_cli_test;
use crate::{
    testenv_utils::DEFAULT_DEVICE_PASSWORD,
    utils::{RESET, YELLOW},
};

#[rstest::rstest]
#[tokio::test]
// TODO: Split me into two tests
async fn list_invitations(tmp_path: TmpPath) {
    let (_, [alice, ..], _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "list-invitations",
            "--device",
            &alice.device_id.hex(),
            "--password-stdin",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .stdout(predicates::str::contains("No invitation."));

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

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "list-invitations",
            "--device",
            &alice.device_id.hex(),
            "--password-stdin",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .stdout(predicates::str::contains(format!(
            "{}\t{YELLOW}idle{RESET}\tdevice",
            token.hex()
        )));
}
