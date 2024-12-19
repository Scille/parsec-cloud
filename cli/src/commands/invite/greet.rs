// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use anyhow::Context;
use dialoguer::Select;
use libparsec::{
    authenticated_cmds::latest::invite_list::InviteListItem,
    internal::{
        DeviceGreetInProgress1Ctx, DeviceGreetInProgress2Ctx, DeviceGreetInProgress3Ctx,
        DeviceGreetInProgress4Ctx, DeviceGreetInitialCtx, UserGreetInProgress1Ctx,
        UserGreetInProgress2Ctx, UserGreetInProgress3Ctx, UserGreetInProgress4Ctx,
        UserGreetInitialCtx,
    },
    InvitationToken,
};
use libparsec_client::{
    Client, ShamirRecoveryGreetInProgress1Ctx, ShamirRecoveryGreetInProgress2Ctx,
    ShamirRecoveryGreetInProgress3Ctx, ShamirRecoveryGreetInitialCtx,
};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Invitation token
        #[arg(value_parser = InvitationToken::from_hex)]
        token: InvitationToken,
    }
);

crate::build_main_with_client!(main, device_greet);

pub async fn device_greet(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args { token, .. } = args;
    log::trace!("Greeting invitation");

    {
        let mut spinner = start_spinner("Poll server for new certificates".into());
        client.poll_server_for_new_certificates().await?;
        spinner.stop_with_symbol(GREEN_CHECKMARK);
    }

    let invitation = step0(client, token).await?;

    match invitation {
        InviteListItem::User { token, .. } => {
            let ctx = client.start_user_invitation_greet(token);

            let ctx = step1_user(ctx).await?;
            let ctx = step2_user(ctx).await?;
            let ctx = step3_user(ctx).await?;
            let ctx = step4_user(ctx).await?;
            step5_user(ctx).await
        }
        InviteListItem::Device { token, .. } => {
            let ctx = client.start_device_invitation_greet(token);

            let ctx: DeviceGreetInProgress1Ctx = step1_device(ctx).await?;
            let ctx = step2_device(ctx).await?;
            let ctx = step3_device(ctx).await?;
            let ctx = step4_device(ctx).await?;
            step5_device(ctx).await
        }
        InviteListItem::ShamirRecovery {
            token,
            claimer_user_id,
            ..
        } => {
            let ctx = client
                .start_shamir_recovery_invitation_greet(token, claimer_user_id)
                .await?;

            let ctx = step1_shamir(ctx).await?;
            let ctx = step2_shamir(ctx).await?;
            let ctx = step3_shamir(ctx).await?;
            step4_shamir(ctx).await
        }
    }
}

/// Step 0: retrieve info
async fn step0(
    client: &Client,
    invitation_token: InvitationToken,
) -> anyhow::Result<InviteListItem> {
    let mut handle = start_spinner("Retrieving invitation info".into());

    let invitations = client.list_invitations().await.context("Server error")?;

    let invitation = match invitations.into_iter().find(|invitation| match invitation {
        InviteListItem::User { token, .. } if *token == invitation_token => true,
        InviteListItem::Device { token, .. } if *token == invitation_token => true,
        InviteListItem::ShamirRecovery { token, .. } if *token == invitation_token => true,
        _ => false,
    }) {
        Some(invitation) => invitation,
        None => return Err(anyhow::anyhow!("Invitation not found")),
    };

    handle.stop_with_symbol(GREEN_CHECKMARK);

    Ok(invitation)
}

/// Step 1: wait peer
async fn step1_user(ctx: UserGreetInitialCtx) -> anyhow::Result<UserGreetInProgress1Ctx> {
    let mut handle = start_spinner("Waiting for claimer".into());

    let ctx = ctx.do_wait_peer().await?;

    handle.stop_with_symbol(GREEN_CHECKMARK);

    Ok(ctx)
}

/// Step 1: wait peer
async fn step1_device(ctx: DeviceGreetInitialCtx) -> anyhow::Result<DeviceGreetInProgress1Ctx> {
    let mut handle = start_spinner("Waiting for claimer".into());

    let ctx = ctx.do_wait_peer().await?;

    handle.stop_with_symbol(GREEN_CHECKMARK);

    Ok(ctx)
}

/// Step 1: wait peer
async fn step1_shamir(
    ctx: ShamirRecoveryGreetInitialCtx,
) -> anyhow::Result<ShamirRecoveryGreetInProgress1Ctx> {
    let mut handle = start_spinner("Waiting for claimer".into());

    let ctx = ctx.do_wait_peer().await?;

    handle.stop_with_newline();

    Ok(ctx)
}

/// Step 2: wait peer trust
async fn step2_user(ctx: UserGreetInProgress1Ctx) -> anyhow::Result<UserGreetInProgress2Ctx> {
    println!(
        "Code to provide to claimer: {YELLOW}{}{RESET}",
        ctx.greeter_sas()
    );

    let mut handle = start_spinner("Waiting for claimer".into());

    let ctx = ctx.do_wait_peer_trust().await?;

    handle.stop_with_symbol(GREEN_CHECKMARK);

    Ok(ctx)
}

/// Step 2: wait peer trust
async fn step2_device(ctx: DeviceGreetInProgress1Ctx) -> anyhow::Result<DeviceGreetInProgress2Ctx> {
    println!(
        "Code to provide to claimer: {YELLOW}{}{RESET}",
        ctx.greeter_sas()
    );

    let mut handle = start_spinner("Waiting for claimer".into());

    let ctx = ctx.do_wait_peer_trust().await?;

    handle.stop_with_symbol(GREEN_CHECKMARK);

    Ok(ctx)
}

/// Step 2: wait peer trust
async fn step2_shamir(
    ctx: ShamirRecoveryGreetInProgress1Ctx,
) -> anyhow::Result<ShamirRecoveryGreetInProgress2Ctx> {
    println!(
        "Code to provide to claimer: {YELLOW}{}{RESET}",
        ctx.greeter_sas()
    );

    let mut handle = start_spinner("Waiting for claimer".into());

    let ctx = ctx.do_wait_peer_trust().await?;

    handle.stop_with_newline();

    Ok(ctx)
}

/// Step 3: signify trust
async fn step3_user(ctx: UserGreetInProgress2Ctx) -> anyhow::Result<UserGreetInProgress3Ctx> {
    let mut input = String::new();
    let sas_codes = ctx.generate_claimer_sas_choices(3);
    for (i, sas_code) in sas_codes.iter().enumerate() {
        println!(" {i} - {YELLOW}{sas_code}{RESET}")
    }

    println!("Select code provided by claimer (0, 1, 2)");

    choose_sas_code(&mut input, &sas_codes, ctx.claimer_sas())?;

    Ok(ctx.do_signify_trust().await?)
}

/// Step 3: signify trust
async fn step3_device(ctx: DeviceGreetInProgress2Ctx) -> anyhow::Result<DeviceGreetInProgress3Ctx> {
    let mut input = String::new();
    let sas_codes = ctx.generate_claimer_sas_choices(3);
    for (i, sas_code) in sas_codes.iter().enumerate() {
        println!(" {i} - {YELLOW}{sas_code}{RESET}")
    }

    println!("Select code provided by claimer (0, 1, 2)");

    choose_sas_code(&mut input, &sas_codes, ctx.claimer_sas())?;

    Ok(ctx.do_signify_trust().await?)
}

/// Step 3: signify trust
async fn step3_shamir(
    ctx: ShamirRecoveryGreetInProgress2Ctx,
) -> anyhow::Result<ShamirRecoveryGreetInProgress3Ctx> {
    let sas_codes = ctx.generate_claimer_sas_choices(3);
    let selected_sas = Select::new()
        .items(&sas_codes)
        .with_prompt("Select code provided by claimer")
        .interact()?;
    if &sas_codes[selected_sas] != ctx.claimer_sas() {
        Err(anyhow::anyhow!("Invalid SAS code"))
    } else {
        Ok(ctx.do_signify_trust().await?)
    }
}

/// Step 4: get claim requests
async fn step4_user(ctx: UserGreetInProgress3Ctx) -> anyhow::Result<UserGreetInProgress4Ctx> {
    Ok(ctx.do_get_claim_requests().await?)
}

/// Step 4: get claim requests
async fn step4_device(ctx: DeviceGreetInProgress3Ctx) -> anyhow::Result<DeviceGreetInProgress4Ctx> {
    Ok(ctx.do_get_claim_requests().await?)
}

/// Step 4: send shares
async fn step4_shamir(ctx: ShamirRecoveryGreetInProgress3Ctx) -> anyhow::Result<()> {
    Ok(ctx.do_send_share().await?)
}

/// Step 5: create new user
async fn step5_user(ctx: UserGreetInProgress4Ctx) -> anyhow::Result<()> {
    let mut input = String::new();
    let device_label = ctx.requested_device_label.clone();
    let human_handle = ctx.requested_human_handle.clone();
    println!("New device label: [{device_label}]");
    println!("New user: [{human_handle}]");

    let profile = choose_user_profile(&mut input)?;

    let mut handle = start_spinner("Creating the user in the server".into());

    ctx.do_create_new_user(device_label, human_handle, profile)
        .await?;

    handle.stop_with_symbol(GREEN_CHECKMARK);

    Ok(())
}

/// Step 5: create new device
async fn step5_device(ctx: DeviceGreetInProgress4Ctx) -> anyhow::Result<()> {
    let device_label = ctx.requested_device_label.clone();
    println!("New device label: [{device_label}]");

    let mut handle = start_spinner("Creating the device in the server".into());

    ctx.do_create_new_device(device_label).await?;

    handle.stop_with_symbol(GREEN_CHECKMARK);

    Ok(())
}
