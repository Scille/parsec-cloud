use std::sync::{Arc, Mutex};

use libparsec::{
    authenticated_cmds::latest::invite_new_user, get_default_config_dir, tmp_path,
    AuthenticatedCmds, InvitationType, ParsecInvitationAddr, ProxyConfig, TmpPath,
};
use rexpect::spawn;

use crate::{
    integration_tests::bootstrap_cli_test,
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

    // spawn greeter thread

    let mut cmd_greeter = assert_cmd::Command::cargo_bin("parsec-cli").unwrap();
    cmd_greeter.args([
        "invite",
        "greet",
        "--device",
        &alice.device_id.hex(),
        &token.hex().to_string(),
    ]);

    let program_greeter = cmd_greeter.get_program().to_str().unwrap().to_string();
    let program_greeter = cmd_greeter
        .get_args()
        .fold(program_greeter, |acc, s| format!("{acc} {s:?}"));

    let p_greeter = Arc::new(Mutex::new(
        spawn(&dbg!(program_greeter), Some(1000)).unwrap(),
    ));

    // spawn claimer thread

    let mut cmd_claimer = assert_cmd::Command::cargo_bin("parsec-cli").unwrap();
    cmd_claimer.args(["invite", "claim", invitation_addr.to_url().as_ref()]);

    let program_claimer = cmd_claimer.get_program().to_str().unwrap().to_string();
    let program_claimer = cmd_claimer
        .get_args()
        .fold(program_claimer, |acc, s| format!("{acc} {s:?}"));

    let p_claimer = Arc::new(Mutex::new(
        spawn(&dbg!(program_claimer), Some(10_000)).unwrap(),
    ));

    // retrieve greeter code
    let greeter_cloned = p_greeter.clone();
    let greeter = tokio::task::spawn(async move {
        let mut locked = greeter_cloned.lock().unwrap();

        locked.exp_string("Enter password for the device:").unwrap();
        locked.send_line(DEFAULT_DEVICE_PASSWORD).unwrap();
        locked.exp_string("Waiting for claimer").unwrap();
    });
    let claimer_cloned = p_claimer.clone();

    let claimer = tokio::task::spawn(async move {
        let mut locked = claimer_cloned.lock().unwrap();

        locked
            .exp_string("Select code provided by greeter")
            .unwrap();
    });
    greeter.await.unwrap();
    p_greeter
        .lock()
        .unwrap()
        .exp_string("Code to provide to claimer:")
        .unwrap();
    let (_, matched) = p_greeter.lock().unwrap().exp_regex("[A-Z0-9]{4}").unwrap();
    let sas_code = dbg!(matched[matched.len() - 4..].to_string()); // last 4 chars are the sas code

    // code selection

    claimer.await.unwrap();
    let cloned_claimer = p_claimer.clone();
    {
        let mut locked = cloned_claimer.lock().unwrap();

        locked.send_line(&sas_code).unwrap();
    }

    // retrieve claimer code
    let greeter_cloned = p_greeter.clone();
    let greeter = tokio::task::spawn(async move {
        let mut locked = greeter_cloned.lock().unwrap();
        locked.exp_string("Waiting for claimer").unwrap();
        locked
            .exp_string("Select code provided by claimer")
            .unwrap();
    });

    let sas_code = {
        let mut locked = p_claimer.lock().unwrap();

        locked.exp_string("Code to provide to greeter:").unwrap();
        let (_, matched) = locked.exp_regex("[A-Z0-9]{4}").unwrap();
        dbg!(matched[matched.len() - 4..].to_string()) // last 4 chars are the sas code
    };
    greeter.await.unwrap();

    let greeter_cloned = p_greeter.clone();

    {
        let mut locked = greeter_cloned.lock().unwrap();

        locked.send_line(&sas_code).unwrap();
        locked
            .exp_string(&format!("Select code provided by claimer: {sas_code}"))
            .unwrap();
    }

    // device creation
    let claimer_cloned = p_claimer.clone();
    {
        let mut locked = claimer_cloned.lock().unwrap();
        locked.exp_string("Enter device label:").unwrap();
        locked.send_line("label").unwrap();
        locked.exp_string("Enter email").unwrap();
        locked.send_line("alice2@example.com").unwrap();
        locked.exp_string("Enter name").unwrap();
        locked.send_line("alice2").unwrap();
    }

    {
        let mut locked = p_greeter.lock().unwrap();
        locked.exp_string("New device label: [label]").unwrap();
        locked
            .exp_string("New user: [alice2 <alice2@example.com>]")
            .unwrap();
        locked.exp_string("Which profile?").unwrap();
        locked.send_line("standard").unwrap();

        locked
            .exp_string("Creating the user in the server")
            .unwrap();
        locked.exp_eof().unwrap();
    }

    {
        let mut locked = p_claimer.lock().unwrap();
        locked
            .exp_string("Enter password for the new device:")
            .unwrap();
        locked.send_line(DEFAULT_DEVICE_PASSWORD).unwrap();
        locked.exp_string("Confirm password:").unwrap();

        locked.send_line(DEFAULT_DEVICE_PASSWORD).unwrap();
        locked.exp_eof().unwrap();
    }
}
