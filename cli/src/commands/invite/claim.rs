// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec::{
    internal::{
        claimer_retrieve_info, DeviceClaimFinalizeCtx, DeviceClaimInProgress1Ctx,
        DeviceClaimInProgress2Ctx, DeviceClaimInProgress3Ctx, DeviceClaimInitialCtx,
        UserClaimFinalizeCtx, UserClaimInProgress1Ctx, UserClaimInProgress2Ctx,
        UserClaimInProgress3Ctx, UserClaimInitialCtx, UserOrDeviceClaimInitialCtx,
    },
    ClientConfig, DeviceAccessStrategy, ParsecInvitationAddr,
};

use crate::utils::*;

#[derive(clap::Parser)]
pub struct Args {
    /// Server invitation address (e.g.: parsec3://127.0.0.1:41997/Org?no_ssl=true&a=claim_shamir_recovery&p=xBA2FaaizwKy4qG5cGDFlXaL)
    #[arg(short, long)]
    addr: ParsecInvitationAddr,
    /// Read the password from stdin instead of a TTY.
    #[arg(long, default_value_t)]
    password_stdin: bool,
    /// Use keyring to store the password for the device.
    #[arg(long, default_value_t, conflicts_with = "password_stdin")]
    use_keyring: bool,
}

enum SaveMode {
    Password { read_from_stdin: bool },
    Keyring,
}

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        addr,
        password_stdin,
        use_keyring,
    } = args;
    log::trace!("Claiming invitation (addr={})", addr);
    let save_mode = if use_keyring {
        SaveMode::Keyring
    } else {
        SaveMode::Password {
            read_from_stdin: password_stdin,
        }
    };
    let ctx = step0(addr).await?;

    match ctx {
        UserOrDeviceClaimInitialCtx::User(ctx) => {
            let ctx = step1_user(ctx).await?;
            let ctx = step2_user(ctx).await?;
            let ctx = step3_user(ctx).await?;
            let ctx = step4_user(ctx).await?;
            save_user(ctx, save_mode).await
        }
        UserOrDeviceClaimInitialCtx::Device(ctx) => {
            let ctx = step1_device(ctx).await?;
            let ctx = step2_device(ctx).await?;
            let ctx = step3_device(ctx).await?;
            let ctx = step4_device(ctx).await?;
            save_device(ctx, save_mode).await
        }
    }
}

/// Step 0: retrieve info
async fn step0(addr: ParsecInvitationAddr) -> anyhow::Result<UserOrDeviceClaimInitialCtx> {
    let mut handle = start_spinner("Retrieving invitation info".into());

    let ctx = claimer_retrieve_info(Arc::new(ClientConfig::default().into()), addr, None).await?;

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

fn get_access_strategy(
    key_file: PathBuf,
    save_mode: SaveMode,
) -> anyhow::Result<DeviceAccessStrategy> {
    match save_mode {
        SaveMode::Password { read_from_stdin } => {
            let password = choose_password(if read_from_stdin {
                ReadPasswordFrom::Stdin
            } else {
                ReadPasswordFrom::Tty {
                    prompt: "Enter password for the new device:",
                }
            })?;
            Ok(DeviceAccessStrategy::Password { key_file, password })
        }
        SaveMode::Keyring => Ok(DeviceAccessStrategy::Keyring { key_file }),
    }
}

async fn save_user(ctx: UserClaimFinalizeCtx, save_mode: SaveMode) -> anyhow::Result<()> {
    let key_file = ctx.get_default_key_file();
    let access = get_access_strategy(key_file, save_mode)?;

    println!(
        "Saving device at: {YELLOW}{}{RESET}",
        access.key_file().display()
    );

    ctx.save_local_device(&access).await?;

    println!("Saved");

    Ok(())
}

async fn save_device(ctx: DeviceClaimFinalizeCtx, save_mode: SaveMode) -> anyhow::Result<()> {
    let key_file = ctx.get_default_key_file();
    let access = get_access_strategy(key_file, save_mode)?;

    println!(
        "Saving device at: {YELLOW}{}{RESET}",
        access.key_file().display()
    );

    ctx.save_local_device(&access).await?;

    println!("Saved");

    Ok(())
}
