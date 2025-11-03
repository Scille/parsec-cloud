#![cfg_attr(target_family = "windows", allow(unused_imports))]

use std::sync::{Arc, Mutex};

use libparsec::{
    authenticated_cmds::latest::invite_new_device, get_default_config_dir, tmp_path,
    AuthenticatedCmds, InvitationType, ParsecInvitationAddr, ProxyConfig, TmpPath,
};

use crate::{
    bootstrap_cli_test, std_cmd,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};

#[rstest::rstest]
#[tokio::test]
async fn invite_device(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "invite",
        "device",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains("Invitation token:"))
    .stdout(predicates::str::contains("Invitation URL:"));
}

#[cfg(target_family = "unix")] // rexpect doesn't support Windows
#[rstest::rstest]
#[tokio::test]
async fn invite_device_dance(tmp_path: TmpPath) {
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

    // spawn greeter thread

    let program_greeter = std_cmd!(
        "invite",
        "greet",
        "--device",
        &alice.device_id.hex(),
        &token.hex()
    );

    let p_greeter = Arc::new(Mutex::new(
        crate::spawn_interactive_command(program_greeter, Some(1000)).unwrap(),
    ));

    // spawn claimer thread

    let program_claimer = std_cmd!("invite", "claim", invitation_addr.to_url().as_ref());

    let p_claimer = Arc::new(Mutex::new(
        crate::spawn_interactive_command(dbg!(program_claimer), Some(10_000)).unwrap(),
    ));

    // retrieve greeter code
    let greeter_cloned = p_greeter.clone();
    let greeter = tokio::task::spawn(async move {
        let mut locked = greeter_cloned.lock().unwrap();

        locked.exp_string("Enter password for the device:").unwrap();
        locked.send_line(DEFAULT_DEVICE_PASSWORD).unwrap();
        let _ = locked.exp_string("Poll server for new certificates");
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

    {
        let mut locked = p_claimer.lock().unwrap();
        locked.exp_string("Enter device label:").unwrap();
        locked.send_line("label").unwrap();

        locked
            .exp_string("Enter password for the new device:")
            .unwrap();
        locked.send_line(DEFAULT_DEVICE_PASSWORD).unwrap();
        locked.exp_string("Confirm password:").unwrap();

        locked.send_line(DEFAULT_DEVICE_PASSWORD).unwrap();
        locked.exp_eof().unwrap();
    }

    {
        let mut locked = p_greeter.lock().unwrap();
        locked.exp_string("New device label: [label]").unwrap();
        locked
            .exp_string("Creating the device in the server")
            .unwrap();
        locked.exp_eof().unwrap();
    }
}
