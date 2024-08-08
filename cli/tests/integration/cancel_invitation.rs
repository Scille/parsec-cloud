use assert_cmd::Command;

use libparsec::{
    authenticated_cmds::v4::invite_new_device, get_default_config_dir, tmp_path, AuthenticatedCmds,
    InvitationType, ParsecInvitationAddr, ProxyConfig, TmpPath,
};

use super::{get_testenv_config, run_local_organization, set_env};
use crate::testenv_utils::DEFAULT_DEVICE_PASSWORD;

#[rstest::rstest]
#[tokio::test]
async fn cancel_invitation(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(tmp_path_str, &url);

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
            "cancel-invitation",
            "--device",
            &alice.device_id.hex(),
            "--token",
            &token.hex().to_string(),
            "--password-stdin",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .stdout(predicates::str::contains("Invitation deleted"));
}
