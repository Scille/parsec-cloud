// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use std::{path::PathBuf, sync::Arc};

use libparsec::{
    authenticated_cmds::latest::invite_list::{self, InviteListItem},
    get_default_config_dir,
    internal::{
        DeviceGreetInProgress1Ctx, DeviceGreetInProgress2Ctx, DeviceGreetInProgress3Ctx,
        DeviceGreetInProgress4Ctx, DeviceGreetInitialCtx, EventBus, UserGreetInProgress1Ctx,
        UserGreetInProgress2Ctx, UserGreetInProgress3Ctx, UserGreetInProgress4Ctx,
        UserGreetInitialCtx,
    },
    AuthenticatedCmds, InvitationToken,
};

use crate::utils::*;

#[derive(Args)]
pub struct GreetInvitation {
    /// Parsec config directory
    #[arg(short, long, default_value_os_t = get_default_config_dir())]
    config_dir: PathBuf,
    /// Device slughash
    #[arg(short, long)]
    device: Option<String>,
    /// Invitation token
    #[arg(short, long)]
    token: String,
}

pub async fn greet_invitation(greet_invitation: GreetInvitation) -> anyhow::Result<()> {
    load_cmds_and_run(
        greet_invitation.config_dir,
        greet_invitation.device,
        |cmds, device| async move {
            let invitation_token = InvitationToken::from_hex(&greet_invitation.token)
                .map_err(|e| anyhow::anyhow!(e))?;

            let invitation = step0(&cmds, invitation_token).await?;

            match invitation {
                InviteListItem::User { .. } => {
                    let ctx = UserGreetInitialCtx::new(
                        device,
                        Arc::new(cmds),
                        EventBus::default(),
                        invitation_token,
                    );

                    let ctx = step1_user(ctx).await?;
                    let ctx = step2_user(ctx).await?;
                    let ctx = step3_user(ctx).await?;
                    let ctx = step4_user(ctx).await?;
                    step5_user(ctx).await
                }
                InviteListItem::Device { .. } => {
                    let ctx = DeviceGreetInitialCtx::new(
                        device,
                        Arc::new(cmds),
                        EventBus::default(),
                        invitation_token,
                    );

                    let ctx = step1_device(ctx).await?;
                    let ctx = step2_device(ctx).await?;
                    let ctx = step3_device(ctx).await?;
                    let ctx = step4_device(ctx).await?;
                    step5_device(ctx).await
                }
            }
        },
    )
    .await
}

/// Step 0: retrieve info
async fn step0(
    cmds: &AuthenticatedCmds,
    invitation_token: InvitationToken,
) -> anyhow::Result<InviteListItem> {
    let handle = start_spinner("Retrieving invitation info");

    let rep = cmds.send(invite_list::Req).await?;

    let invitations = match rep {
        invite_list::InviteListRep::Ok { invitations } => invitations,
        rep => return Err(anyhow::anyhow!("Server error: {rep:?}")),
    };

    let invitation = match invitations.into_iter().find(|invitation| match invitation {
        InviteListItem::User { token, .. } if *token == invitation_token => true,
        InviteListItem::Device { token, .. } if *token == invitation_token => true,
        _ => false,
    }) {
        Some(invitation) => invitation,
        None => return Err(anyhow::anyhow!("Invitation not found")),
    };

    handle.done();

    Ok(invitation)
}

/// Step 1: wait peer
async fn step1_user(ctx: UserGreetInitialCtx) -> anyhow::Result<UserGreetInProgress1Ctx> {
    let handle = start_spinner("Waiting for claimer");

    let ctx = ctx.do_wait_peer().await?;

    handle.done();

    Ok(ctx)
}

/// Step 1: wait peer
async fn step1_device(ctx: DeviceGreetInitialCtx) -> anyhow::Result<DeviceGreetInProgress1Ctx> {
    let handle = start_spinner("Waiting for claimer");

    let ctx = ctx.do_wait_peer().await?;

    handle.done();

    Ok(ctx)
}

/// Step 2: wait peer trust
async fn step2_user(ctx: UserGreetInProgress1Ctx) -> anyhow::Result<UserGreetInProgress2Ctx> {
    println!(
        "Code to provide to claimer: {YELLOW}{}{RESET}",
        ctx.greeter_sas()
    );

    let handle = start_spinner("Waiting for claimer");

    let ctx = ctx.do_wait_peer_trust().await?;

    handle.done();

    Ok(ctx)
}

/// Step 2: wait peer trust
async fn step2_device(ctx: DeviceGreetInProgress1Ctx) -> anyhow::Result<DeviceGreetInProgress2Ctx> {
    println!(
        "Code to provide to claimer: {YELLOW}{}{RESET}",
        ctx.greeter_sas()
    );

    let handle = start_spinner("Waiting for claimer");

    let ctx = ctx.do_wait_peer_trust().await?;

    handle.done();

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

/// Step 4: get claim requests
async fn step4_user(ctx: UserGreetInProgress3Ctx) -> anyhow::Result<UserGreetInProgress4Ctx> {
    Ok(ctx.do_get_claim_requests().await?)
}

/// Step 4: get claim requests
async fn step4_device(ctx: DeviceGreetInProgress3Ctx) -> anyhow::Result<DeviceGreetInProgress4Ctx> {
    Ok(ctx.do_get_claim_requests().await?)
}

/// Step 5: create new user
async fn step5_user(ctx: UserGreetInProgress4Ctx) -> anyhow::Result<()> {
    let mut input = String::new();
    let device_label = ctx.requested_device_label.clone();
    let human_handle = ctx.requested_human_handle.clone();
    println!("New device label: [{device_label}]");
    println!("New user: [{human_handle}]");

    let profile = choose_user_profile(&mut input)?;

    let handle = start_spinner("Creating the user in the server");

    ctx.do_create_new_user(device_label, human_handle, profile)
        .await?;

    handle.done();

    Ok(())
}

/// Step 5: create new device
async fn step5_device(ctx: DeviceGreetInProgress4Ctx) -> anyhow::Result<()> {
    let device_label = ctx.requested_device_label.clone();
    println!("New device label: [{device_label}]");

    let handle = start_spinner("Creating the device in the server");

    ctx.do_create_new_device(device_label).await?;

    handle.done();

    Ok(())
}
