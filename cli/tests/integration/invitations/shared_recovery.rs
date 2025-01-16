use std::sync::{Arc, Mutex};

use libparsec::{
    authenticated_cmds::latest::invite_new_shamir_recovery, get_default_config_dir, tmp_path,
    AuthenticatedCmds, InvitationType, ParsecInvitationAddr, ProxyConfig, TmpPath,
};
use rexpect::{session::PtySession, spawn};

use crate::{
    integration_tests::{bootstrap_cli_test, shared_recovery_create},
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};

macro_rules! match_sas_code {
    ($locked:ident, $sas_code:ident) => {
        $locked.read_line().unwrap(); //empty line
        let first = dbg!($locked.read_line().unwrap());
        let second = dbg!($locked.read_line().unwrap());
        let third = dbg!($locked.read_line().unwrap());

        if $sas_code == first[first.len() - 4..] {
            $locked.send_line("").unwrap();
        } else if $sas_code == second[second.len() - 4..] {
            $locked.send_line("j").unwrap();
        } else if $sas_code == third[third.len() - 4..] {
            $locked.send_line("jj").unwrap();
        } else {
            panic!("no corresponding sas code available")
        }
    };
}

#[rstest::rstest]
#[tokio::test]
async fn invite_shared_recovery_dance(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, bob, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    shared_recovery_create(&alice, &bob, None);
    let cmds = AuthenticatedCmds::new(
        &get_default_config_dir(),
        bob.clone(),
        ProxyConfig::new_from_env().unwrap(),
    )
    .unwrap();

    let rep = cmds
        .send(invite_new_shamir_recovery::Req {
            send_email: false,
            claimer_user_id: alice.user_id,
        })
        .await
        .unwrap();

    let invitation_addr = match rep {
        invite_new_shamir_recovery::InviteNewShamirRecoveryRep::Ok { token, .. } => {
            ParsecInvitationAddr::new(
                alice.organization_addr.clone(),
                alice.organization_id().clone(),
                InvitationType::ShamirRecovery,
                token,
            )
        }
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
        &bob.device_id.hex(),
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
            .exp_string("Choose a person to contact now:")
            .unwrap();
        // down to choose bob
        locked.send_line("j").unwrap();

        locked
            .exp_string("Select code provided by greeter:")
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

        match_sas_code!(locked, sas_code);
    }

    // retrieve claimer code
    let greeter_cloned = p_greeter.clone();
    let greeter = tokio::task::spawn(async move {
        let mut locked = greeter_cloned.lock().unwrap();
        locked.exp_string("Waiting for claimer").unwrap();
        locked
            .exp_string("Select code provided by claimer:")
            .unwrap();
    });

    let sas_code = {
        let mut locked = p_claimer.lock().unwrap();

        locked.exp_string("Code to provide to greeter:").unwrap();
        let (_, matched) = locked.exp_regex("[A-Z0-9]{4}").unwrap();
        dbg!(matched[matched.len() - 4..].to_string()) // last 4 chars are the sas code
    };
    greeter.await.unwrap();

    {
        let mut locked = p_greeter.lock().unwrap();

        match_sas_code!(locked, sas_code);
    }
    let mut greeter = Arc::<Mutex<PtySession>>::try_unwrap(p_greeter)
        .ok()
        .unwrap()
        .into_inner()
        .unwrap();
    greeter.exp_eof().unwrap();
    greeter.process.exit().unwrap();
    drop(greeter);
    // device creation
    let mut locked = p_claimer.lock().unwrap();
    locked.exp_string("Enter device label:").unwrap();
    locked.send_line("label").unwrap();
    locked.exp_string("Recovering device").unwrap();

    locked
        .exp_string("Enter password for the new device:")
        .unwrap();
    locked.send_line(DEFAULT_DEVICE_PASSWORD).unwrap();
    locked.exp_string("Confirm password:").unwrap();

    locked.send_line(DEFAULT_DEVICE_PASSWORD).unwrap();
    locked.exp_eof().unwrap();
}
