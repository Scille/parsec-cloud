// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use std::sync::Arc;

use libparsec::{
    internal::{
        claimer_retrieve_info, DeviceClaimFinalizeCtx, DeviceClaimInProgress1Ctx,
        DeviceClaimInProgress2Ctx, DeviceClaimInProgress3Ctx, DeviceClaimInitialCtx,
        UserClaimFinalizeCtx, UserClaimInProgress1Ctx, UserClaimInProgress2Ctx,
        UserClaimInProgress3Ctx, UserClaimInitialCtx, UserOrDeviceClaimInitialCtx,
    },
    BackendInvitationAddr, ClientConfig, DeviceAccessStrategy,
};

use crate::utils::*;

#[derive(Args)]
pub struct ClaimInvitation {
    /// Server invitation address (e.g.: parsec://127.0.0.1:41905/Org?no_ssl=true&action=claim_user&token=4e45cc21e7604af196173ff6c9184a1f)
    #[arg(short, long)]
    addr: BackendInvitationAddr,
}

pub async fn claim_invitation(claim_invitation: ClaimInvitation) -> anyhow::Result<()> {
    let ctx = step0(claim_invitation.addr).await?;

    match ctx {
        UserOrDeviceClaimInitialCtx::User(ctx) => {
            let ctx = step1_user(ctx).await?;
            let ctx = step2_user(ctx).await?;
            let ctx = step3_user(ctx).await?;
            let ctx = step4_user(ctx).await?;
            save_user(ctx).await
        }
        UserOrDeviceClaimInitialCtx::Device(ctx) => {
            let ctx = step1_device(ctx).await?;
            let ctx = step2_device(ctx).await?;
            let ctx = step3_device(ctx).await?;
            let ctx = step4_device(ctx).await?;
            save_device(ctx).await
        }
    }
}

/// Step 0: retrieve info
async fn step0(addr: BackendInvitationAddr) -> anyhow::Result<UserOrDeviceClaimInitialCtx> {
    let mut handle = start_spinner("Retrieving invitation info".into());

    let ctx = claimer_retrieve_info(Arc::new(ClientConfig::default().into()), addr).await?;

    handle.stop_with_newline();

    Ok(ctx)
}

/// Step 1: wait peer
async fn step1_user(ctx: UserClaimInitialCtx) -> anyhow::Result<UserClaimInProgress1Ctx> {
    println!(
        "Invitation greeter: {YELLOW}{}{RESET}",
        ctx.greeter_human_handle()
    );

    let mut handle = start_spinner("Waiting the greeter to start the invitation procedure".into());

    let ctx = ctx.do_wait_peer().await?;

    handle.stop_with_newline();

    Ok(ctx)
}

/// Step 1: wait peer
async fn step1_device(ctx: DeviceClaimInitialCtx) -> anyhow::Result<DeviceClaimInProgress1Ctx> {
    println!(
        "Invitation greeter: {YELLOW}{}{RESET}",
        ctx.greeter_human_handle()
    );

    let mut handle = start_spinner("Waiting the greeter to start the invitation procedure".into());

    let ctx = ctx.do_wait_peer().await?;

    handle.stop_with_newline();

    Ok(ctx)
}

/// Step 2: signify trust
async fn step2_user(ctx: UserClaimInProgress1Ctx) -> anyhow::Result<UserClaimInProgress2Ctx> {
    let mut input = String::new();
    let sas_codes = ctx.generate_greeter_sas_choices(3);

    for (i, sas_code) in sas_codes.iter().enumerate() {
        println!(" {i} - {YELLOW}{sas_code}{RESET}")
    }

    println!("Select code provided by greeter (0, 1, 2)");

    choose_sas_code(&mut input, &sas_codes, ctx.greeter_sas())?;

    Ok(ctx.do_signify_trust().await?)
}

/// Step 2: signify trust
async fn step2_device(ctx: DeviceClaimInProgress1Ctx) -> anyhow::Result<DeviceClaimInProgress2Ctx> {
    let mut input = String::new();
    let sas_codes = ctx.generate_greeter_sas_choices(3);

    for (i, sas_code) in sas_codes.iter().enumerate() {
        println!(" {i} - {YELLOW}{sas_code}{RESET}")
    }

    println!("Select code provided by greeter (0, 1, 2)");

    choose_sas_code(&mut input, &sas_codes, ctx.greeter_sas())?;

    Ok(ctx.do_signify_trust().await?)
}

/// Step 3: wait peer trust
async fn step3_user(ctx: UserClaimInProgress2Ctx) -> anyhow::Result<UserClaimInProgress3Ctx> {
    println!(
        "Code to provide to greeter: {YELLOW}{}{RESET}",
        ctx.claimer_sas()
    );

    let mut handle = start_spinner("Waiting for greeter".into());

    let ctx = ctx.do_wait_peer_trust().await?;

    handle.stop_with_newline();

    Ok(ctx)
}

/// Step 3: wait peer trust
async fn step3_device(ctx: DeviceClaimInProgress2Ctx) -> anyhow::Result<DeviceClaimInProgress3Ctx> {
    println!(
        "Code to provide to greeter: {YELLOW}{}{RESET}",
        ctx.claimer_sas()
    );

    let mut handle = start_spinner("Waiting for greeter".into());

    let ctx = ctx.do_wait_peer_trust().await?;

    handle.stop_with_newline();

    Ok(ctx)
}

/// Step 4: claim user
async fn step4_user(ctx: UserClaimInProgress3Ctx) -> anyhow::Result<UserClaimFinalizeCtx> {
    let mut input = String::new();
    let device_label = choose_device_label(&mut input)?;
    let human_handle = choose_human_handle(&mut input)?;

    let mut handle = start_spinner("Waiting for greeter".into());

    let ctx = ctx.do_claim_user(device_label, human_handle).await?;

    handle.stop_with_newline();

    Ok(ctx)
}

/// Step 4: claim device
async fn step4_device(ctx: DeviceClaimInProgress3Ctx) -> anyhow::Result<DeviceClaimFinalizeCtx> {
    let mut input = String::new();
    let device_label = choose_device_label(&mut input)?;

    let mut handle = start_spinner("Waiting for greeter".into());

    let ctx = ctx.do_claim_device(device_label).await?;

    handle.stop_with_newline();

    Ok(ctx)
}

async fn save_user(ctx: UserClaimFinalizeCtx) -> anyhow::Result<()> {
    let password = choose_password()?;
    let key_file = ctx.get_default_key_file();
    let key_file_str = key_file.display();

    println!("Saving device at: {YELLOW}{key_file_str}{RESET}");

    let access = DeviceAccessStrategy::Password { key_file, password };

    ctx.save_local_device(&access).await?;

    println!("Saved");

    Ok(())
}

async fn save_device(ctx: DeviceClaimFinalizeCtx) -> anyhow::Result<()> {
    let password = choose_password()?;
    let key_file = ctx.get_default_key_file();
    let key_file_str = key_file.display();

    println!("Saving device at: {YELLOW}{key_file_str}{RESET}");

    let access = DeviceAccessStrategy::Password { key_file, password };

    ctx.save_local_device(&access).await?;

    println!("Saved");

    Ok(())
}
