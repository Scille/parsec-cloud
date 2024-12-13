use std::io::{BufReader, Write};

use assert_cmd::cargo::CommandCargoExt;

use libparsec::{
    authenticated_cmds::v4::invite_new_user, get_default_config_dir, tmp_path, AuthenticatedCmds,
    InvitationType, ParsecInvitationAddr, ProxyConfig, TmpPath,
};

use crate::{
    integration_tests::{bootstrap_cli_test, wait_for},
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
    utils::YELLOW,
};

#[rstest::rstest]
#[tokio::test]
async fn invite_user(tmp_path: TmpPath) {
    let (addr, TestOrganization { alice, .. }, org_id) =
        bootstrap_cli_test(&tmp_path).await.unwrap();

    let mut addr = addr.to_url();
    addr.path_segments_mut().unwrap().push(org_id.as_ref());
    addr.query_pairs_mut()
        .append_pair("a", "claim_user")
        .append_key_only("p");
    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "invite",
        "user",
        "--device",
        &alice.device_id.hex(),
        "a@b.c"
    )
    .stdout(predicates::str::contains(format!(
        "Invitation URL: {YELLOW}{addr}"
    )));
}

#[rstest::rstest]
#[tokio::test]
async fn invite_user_dance(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let cmds = AuthenticatedCmds::new(
        &get_default_config_dir(),
        alice.clone(),
        ProxyConfig::new_from_env().unwrap(),
    )
    .unwrap();

    let rep = cmds
        .send(invite_new_user::Req {
            claimer_email: "a@b.c".into(),
            send_email: false,
        })
        .await
        .unwrap();

    let invitation_addr = match rep {
        invite_new_user::InviteNewUserRep::Ok { token, .. } => ParsecInvitationAddr::new(
            alice.organization_addr.clone(),
            alice.organization_id().clone(),
            InvitationType::Device,
            token,
        ),
        rep => {
            panic!("Server refused to create user invitation: {rep:?}");
        }
    };

    let token = invitation_addr.token();

    let mut p_greeter = std::process::Command::cargo_bin("parsec-cli")
        .unwrap()
        .args([
            "invite",
            "greet",
            &token.hex().to_string(),
            "--device",
            &alice.device_id.hex(),
            "--password-stdin",
        ])
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let mut p_claimer = std::process::Command::cargo_bin("parsec-cli")
        .unwrap()
        .args([
            "invite",
            "claim",
            invitation_addr.to_url().as_ref(),
            "--password-stdin",
        ])
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdout_greeter = BufReader::new(p_greeter.stdout.take().unwrap());
    let mut stdout_claimer = BufReader::new(p_claimer.stdout.take().unwrap());
    let mut stdin_greeter = p_greeter.stdin.take().unwrap();
    let mut stdin_claimer = p_claimer.stdin.take().unwrap();
    let mut buf = String::new();

    stdin_greeter
        .write_all(format!("{DEFAULT_DEVICE_PASSWORD}\n").as_bytes())
        .unwrap();
    wait_for(&mut stdout_greeter, &mut buf, "Code to provide to claimer");
    let sas_code = buf.split_once(YELLOW).unwrap().1[..4].to_string();
    wait_for(&mut stdout_claimer, &mut buf, &sas_code);
    let sas_code_index = &buf[1..2].to_string();
    wait_for(&mut stdout_claimer, &mut buf, "Select code");
    stdin_claimer
        .write_all(format!("{sas_code_index}\n").as_bytes())
        .unwrap();

    wait_for(&mut stdout_claimer, &mut buf, "Code to provide to greeter");
    let sas_code = buf.split_once(YELLOW).unwrap().1[..4].to_string();
    wait_for(&mut stdout_greeter, &mut buf, &sas_code);
    let sas_code_index = &buf[1..2].to_string();
    wait_for(&mut stdout_greeter, &mut buf, "Select code");
    stdin_greeter
        .write_all(format!("{sas_code_index}\n").as_bytes())
        .unwrap();

    wait_for(&mut stdout_claimer, &mut buf, "Enter device label");
    stdin_claimer.write_all(b"DeviceLabelTest\n").unwrap();
    wait_for(&mut stdout_claimer, &mut buf, "Enter email");
    stdin_claimer.write_all(b"alice2@example.com\n").unwrap();
    wait_for(&mut stdout_claimer, &mut buf, "Enter name");
    stdin_claimer.write_all(b"alice2\n").unwrap();

    wait_for(&mut stdout_greeter, &mut buf, "Which profile?");
    stdin_greeter.write_all(b"0\n").unwrap();

    wait_for(&mut stdout_greeter, &mut buf, "Creating the user");
    stdin_claimer
        .write_all(format!("{DEFAULT_DEVICE_PASSWORD}\n").as_bytes())
        .unwrap();
    wait_for(&mut stdout_claimer, &mut buf, "Saved");
    assert!(p_greeter.wait().unwrap().success());
    assert!(p_claimer.wait().unwrap().success());
}
