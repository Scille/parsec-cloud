// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::{fmt::Write as _, io::Write as _};

use anyhow::Context;
use libparsec::{
    authenticated_cmds::latest::invite_list::InviteListItem,
    internal::{
        DeviceGreetInProgress1Ctx, DeviceGreetInProgress2Ctx, DeviceGreetInProgress3Ctx,
        DeviceGreetInProgress4Ctx, DeviceGreetInitialCtx, UserGreetInProgress1Ctx,
        UserGreetInProgress2Ctx, UserGreetInProgress3Ctx, UserGreetInProgress4Ctx,
        UserGreetInitialCtx,
    },
    AccessToken,
};
use libparsec_client::{
    Client, ShamirRecoveryGreetInProgress1Ctx, ShamirRecoveryGreetInProgress2Ctx,
    ShamirRecoveryGreetInProgress3Ctx, ShamirRecoveryGreetInitialCtx,
};

use crate::{ui::Color, utils::*};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Invitation token
        #[arg(value_parser = AccessToken::from_hex, value_hint = clap::ValueHint::Other)]
        token: AccessToken,
    }
);

crate::build_main_with_client!(main, device_greet);

pub async fn device_greet(ui: crate::Ui, args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args { token, .. } = args;
    log::trace!("Greeting invitation");

    poll_server_for_new_certificates(&ui, client).await?;

    let invitation = step0(&ui, client, token).await?;

    match invitation {
        InviteListItem::User { token, .. } => {
            let ctx = client.start_user_invitation_greet(token);

            let ctx = step1_user(&ui, ctx).await?;
            let ctx = step2_user(&ui, ctx).await?;
            let ctx = step3_user(ctx).await?;
            let ctx = step4_user(&ui, ctx).await?;
            step5_user(&ui, ctx).await
        }
        InviteListItem::Device { token, .. } => {
            let ctx = client.start_device_invitation_greet(token);

            let ctx: DeviceGreetInProgress1Ctx = step1_device(&ui, ctx).await?;
            let ctx = step2_device(&ui, ctx).await?;
            let ctx = step3_device(ctx).await?;
            let ctx = step4_device(&ui, ctx).await?;
            step5_device(&ui, ctx).await
        }
        InviteListItem::ShamirRecovery { token, .. } => {
            let ctx = client.start_shamir_recovery_invitation_greet(token).await?;

            let ctx = step1_shamir(&ui, ctx).await?;
            let ctx = step2_shamir(&ui, ctx).await?;
            let ctx = step3_shamir(ctx).await?;
            step4_shamir(ctx).await
        }
    }
}

/// Step 0: retrieve info
async fn step0(
    ui: &crate::Ui,
    client: &Client,
    invitation_token: AccessToken,
) -> anyhow::Result<InviteListItem> {
    let handle = ui.with_spinner(|_, out| write!(out, "Retrieving invitation info"))?;

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

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(invitation)
}

/// Step 1: wait peer
async fn step1_user(
    ui: &crate::Ui,
    ctx: UserGreetInitialCtx,
) -> anyhow::Result<UserGreetInProgress1Ctx> {
    let handle = ui.with_spinner(|_, out| write!(out, "Waiting for claimer"))?;

    let ctx = ctx.do_wait_peer().await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

/// Step 1: wait peer
async fn step1_device(
    ui: &crate::Ui,
    ctx: DeviceGreetInitialCtx,
) -> anyhow::Result<DeviceGreetInProgress1Ctx> {
    let handle = ui.with_spinner(|_, out| write!(out, "Waiting for claimer"))?;

    let ctx = ctx.do_wait_peer().await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

/// Step 1: wait peer
async fn step1_shamir(
    ui: &crate::Ui,
    ctx: ShamirRecoveryGreetInitialCtx,
) -> anyhow::Result<ShamirRecoveryGreetInProgress1Ctx> {
    let handle = ui.with_spinner(|_, out| write!(out, "Waiting for claimer"))?;

    let ctx = ctx.do_wait_peer().await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

/// Step 2: wait peer trust
async fn step2_user(
    ui: &crate::Ui,
    ctx: UserGreetInProgress1Ctx,
) -> anyhow::Result<UserGreetInProgress2Ctx> {
    show_claimer_code(ui, ctx.greeter_sas())?;

    let handle = ui.with_spinner(|_, out| write!(out, "Waiting for claimer"))?;

    let ctx = ctx.do_wait_peer_trust().await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

/// Step 2: wait peer trust
async fn step2_device(
    ui: &crate::Ui,
    ctx: DeviceGreetInProgress1Ctx,
) -> anyhow::Result<DeviceGreetInProgress2Ctx> {
    show_claimer_code(ui, ctx.greeter_sas())?;

    let handle = ui.with_spinner(|_, out| write!(out, "Waiting for claimer"))?;

    let ctx = ctx.do_wait_peer_trust().await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

/// Step 2: wait peer trust
async fn step2_shamir(
    ui: &crate::Ui,
    ctx: ShamirRecoveryGreetInProgress1Ctx,
) -> anyhow::Result<ShamirRecoveryGreetInProgress2Ctx> {
    show_claimer_code(ui, ctx.greeter_sas())?;

    let handle = ui.with_spinner(|_, out| write!(out, "Waiting for claimer"))?;

    let ctx = ctx.do_wait_peer_trust().await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

/// Step 3: signify trust
async fn step3_user(ctx: UserGreetInProgress2Ctx) -> anyhow::Result<UserGreetInProgress3Ctx> {
    let sas_codes = ctx.generate_claimer_sas_choices(3);
    choose_sas_code(&sas_codes, ctx.claimer_sas(), "claimer")?;

    Ok(ctx.do_signify_trust().await?)
}

/// Step 3: signify trust
async fn step3_device(ctx: DeviceGreetInProgress2Ctx) -> anyhow::Result<DeviceGreetInProgress3Ctx> {
    let sas_codes = ctx.generate_claimer_sas_choices(3);
    choose_sas_code(&sas_codes, ctx.claimer_sas(), "claimer")?;

    Ok(ctx.do_signify_trust().await?)
}

/// Step 3: signify trust
async fn step3_shamir(
    ctx: ShamirRecoveryGreetInProgress2Ctx,
) -> anyhow::Result<ShamirRecoveryGreetInProgress3Ctx> {
    let sas_codes = ctx.generate_claimer_sas_choices(3);
    choose_sas_code(&sas_codes, ctx.claimer_sas(), "claimer")?;

    Ok(ctx.do_signify_trust().await?)
}

/// Step 4: get claim requests
async fn step4_user(
    ui: &crate::Ui,
    ctx: UserGreetInProgress3Ctx,
) -> anyhow::Result<UserGreetInProgress4Ctx> {
    let handle = ui.with_spinner(|_, out| write!(out, "Waiting for claimer information"))?;

    let ctx = ctx.do_get_claim_requests().await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

/// Step 4: get claim requests
async fn step4_device(
    ui: &crate::Ui,
    ctx: DeviceGreetInProgress3Ctx,
) -> anyhow::Result<DeviceGreetInProgress4Ctx> {
    let handle = ui.with_spinner(|_, out| write!(out, "Waiting for claimer information"))?;

    let ctx = ctx.do_get_claim_requests().await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

/// Step 4: send shares
async fn step4_shamir(ctx: ShamirRecoveryGreetInProgress3Ctx) -> anyhow::Result<()> {
    Ok(ctx.do_send_share().await?)
}

/// Step 5: create new user
async fn step5_user(ui: &crate::Ui, ctx: UserGreetInProgress4Ctx) -> anyhow::Result<()> {
    let device_label = ctx.requested_device_label.clone();
    let human_handle = ctx.requested_human_handle.clone();
    ui.with_message(|_, out| writeln!(out, "New device label: [{device_label}]"))?;
    ui.with_message(|_, out| writeln!(out, "New user: [{human_handle}]"))?;

    let profile = choose_user_profile()?;

    let handle = ui.with_spinner(|_, out| write!(out, "Creating the user in the server"))?;

    ctx.do_create_new_user(device_label, human_handle, profile)
        .await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(())
}

/// Step 5: create new device
async fn step5_device(ui: &crate::Ui, ctx: DeviceGreetInProgress4Ctx) -> anyhow::Result<()> {
    let device_label = ctx.requested_device_label.clone();
    ui.with_message(|_, out| writeln!(out, "New device label: [{device_label}]"))?;

    let handle = ui.with_spinner(|_, out| write!(out, "Creating the device in the server"))?;

    ctx.do_create_new_device(device_label).await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(())
}

fn show_claimer_code(ui: &crate::Ui, code: &libparsec_types::SASCode) -> std::io::Result<()> {
    ui.with_message(|fmt, out| {
        writeln!(
            out,
            "Code to provide to claimer: {}",
            fmt.wrap_in_color(Color::Yellow, code)
        )
    })
}
