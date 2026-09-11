// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    fmt::Write as _,
    io::{IsTerminal, Write as _},
    sync::Arc,
};

use anyhow::anyhow;
use libparsec::{
    internal::{
        claimer_retrieve_info, AnyClaimRetrievedInfoCtx, DeviceClaimFinalizeCtx,
        DeviceClaimInProgress1Ctx, DeviceClaimInProgress2Ctx, DeviceClaimInProgress3Ctx,
        DeviceClaimInitialCtx, ShamirRecoveryClaimInProgress1Ctx,
        ShamirRecoveryClaimInProgress2Ctx, ShamirRecoveryClaimInProgress3Ctx,
        ShamirRecoveryClaimInitialCtx, ShamirRecoveryClaimMaybeFinalizeCtx,
        ShamirRecoveryClaimMaybeRecoverDeviceCtx, ShamirRecoveryClaimPickRecipientCtx,
        ShamirRecoveryClaimRecoverDeviceCtx, ShamirRecoveryClaimShare, UserClaimFinalizeCtx,
        UserClaimInProgress1Ctx, UserClaimInProgress2Ctx, UserClaimInProgress3Ctx,
        UserClaimInitialCtx, UserClaimListAdministratorsCtx,
    },
    ClientConfig, ParsecInvitationAddr, Url,
};
use libparsec_client::ShamirRecoveryClaimFinalizeCtx;

use crate::{
    ui::{compat::ShortAvailableDeviceDisplay, Color},
    utils::*,
};
use dialoguer::{Confirm, FuzzySelect, Input};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, data_dir, password_stdin, auth]
    pub struct Args {
        // cspell:disable-next-line
        /// Server invitation address (e.g.: parsec3://127.0.0.1:41997/Org?no_ssl=true&a=claim_shamir_recovery&p=xBA2FaaizwKy4qG5cGDFlXaL`
        /// or http://127.0.0.1:41997/Org?no_ssl=true&a=claim_shamir_recovery&p=xBA2FaaizwKy4qG5cGDFlXaL`)
        #[arg(value_hint = clap::ValueHint::Url)]
        addr: Url,
    }
);

pub async fn main(ui: crate::Ui, args: Args) -> anyhow::Result<()> {
    let Args {
        config_dir,
        data_dir,
        addr,
        password_stdin,
        auth,
    } = args;
    let addr = ParsecInvitationAddr::from_any(addr.as_str())?;
    log::trace!("Claiming invitation (addr={addr})");

    let config = ClientConfig {
        config_dir,
        data_base_dir: data_dir,
        ..Default::default()
    };
    let ctx = step0(&ui, addr, config).await?;

    match ctx {
        AnyClaimRetrievedInfoCtx::User(ctx) => {
            let ctx = user_pick_admin(ctx)?;
            let ctx = step1_user(&ui, ctx).await?;
            let ctx = step2_user(ctx).await?;
            let ctx = step3_user(&ui, ctx).await?;
            let ctx = step4_user(&ui, ctx).await?;
            let save_strategy = auth.get_client_save_strategy(password_stdin).await?;

            save_user(&ui, ctx, save_strategy).await
        }
        AnyClaimRetrievedInfoCtx::Device(ctx) => {
            let ctx = step1_device(&ui, ctx).await?;
            let ctx = step2_device(ctx).await?;
            let ctx = step3_device(&ui, ctx).await?;
            let ctx = step4_device(&ui, ctx).await?;
            let save_strategy = auth.get_client_save_strategy(password_stdin).await?;

            save_device(&ui, ctx, save_strategy).await
        }
        AnyClaimRetrievedInfoCtx::ShamirRecovery(ctx) => {
            let mut pick_ctx = ctx;

            let mut device_ctx = loop {
                let ctx = shamir_pick_recipient(&ui, &pick_ctx)?;
                let ctx = step1_shamir(&ui, ctx).await?;
                let ctx = step2_shamir(ctx).await?;
                let ctx = step3_shamir(&ui, ctx).await?;
                let share_ctx = step4_shamir(&ui, ctx).await?;
                let maybe = pick_ctx.add_share(share_ctx)?;
                match maybe {
                    ShamirRecoveryClaimMaybeRecoverDeviceCtx::RecoverDevice(
                        shamir_recovery_claim_recover_device_ctx,
                    ) => {
                        break shamir_recovery_claim_recover_device_ctx;
                    }
                    // need more shares
                    ShamirRecoveryClaimMaybeRecoverDeviceCtx::PickRecipient(
                        shamir_recovery_claim_pick_recipient_ctx,
                    ) => pick_ctx = shamir_recovery_claim_pick_recipient_ctx,
                }
            };

            let final_ctx = loop {
                let ctx = step5_shamir(&ui, device_ctx).await?;
                match ctx {
                    ShamirRecoveryClaimMaybeFinalizeCtx::Offline(ctx) => {
                        if std::io::stdin().is_terminal()
                            && Confirm::new()
                                .with_prompt("Unable to join server, do you want to retry?")
                                .interact()?
                        {
                            device_ctx = ctx;
                            continue;
                        } else {
                            return Err(anyhow!("Server offline, try again later."));
                        }
                    }
                    ShamirRecoveryClaimMaybeFinalizeCtx::Finalize(
                        shamir_recovery_claim_finalize_ctx,
                    ) => {
                        break shamir_recovery_claim_finalize_ctx;
                    }
                }
            };
            let save_strategy = auth.get_client_save_strategy(password_stdin).await?;

            save_shamir_recovery(&ui, final_ctx, save_strategy).await
        }
    }
}

/// Step 0: retrieve info
async fn step0(
    ui: &crate::Ui,
    addr: ParsecInvitationAddr,
    config: ClientConfig,
) -> anyhow::Result<AnyClaimRetrievedInfoCtx> {
    let handle = ui.with_spinner(|_, out| write!(out, "Retrieving invitation info"))?;

    let ctx = claimer_retrieve_info(Arc::new(config.into()), addr, None).await?;

    handle.stop_with(|fmt, out| write!(out, "{}", fmt.wrap_in_color(Color::Green, CHECKMARK)))?;

    Ok(ctx)
}

enum UserClaimAdministratorPick {
    Single(Box<UserClaimInitialCtx>),
    Multiple(Vec<UserClaimInitialCtx>),
}

fn user_pick_admin(
    ctx: UserClaimListAdministratorsCtx,
) -> anyhow::Result<UserClaimAdministratorPick> {
    let ctxs = ctx.list_initial_ctxs();
    assert!(!ctxs.is_empty());
    // Only one admin, no need to choose
    if ctxs.len() == 1 {
        return Ok(UserClaimAdministratorPick::Single(
            ctxs.into_iter()
                .next()
                .map(Box::new)
                .expect("ctxs is non-empty"),
        ));
    }
    let humans = ctxs
        .iter()
        .map(|ctx| format!("{}", ctx.greeter_human_handle()))
        .chain(std::iter::once("All administrators".to_string()));
    let selection = FuzzySelect::new()
        .default(0)
        .with_prompt("Choose an administrator to contact now")
        .items(humans)
        .interact()?;
    if selection == ctxs.len() {
        return Ok(UserClaimAdministratorPick::Multiple(ctxs));
    }
    Ok(UserClaimAdministratorPick::Single(
        ctxs.into_iter()
            .nth(selection)
            .map(Box::new)
            .expect("selection should correspond to a ctx"),
    ))
}

/// Step 0.5: choose recipient
fn shamir_pick_recipient(
    ui: &crate::Ui,
    ctx: &ShamirRecoveryClaimPickRecipientCtx,
) -> anyhow::Result<ShamirRecoveryClaimInitialCtx> {
    let recipients = ctx.recipients_without_a_share();
    let human_recipients = recipients.iter().map(|r| {
        format!(
            "{} - {} share{}",
            r.human_handle,
            r.shares,
            maybe_plural(r.shares.get())
        )
    });
    if ctx.retrieved_shares().is_empty() {
        ui.with_message(|_, out| {
            writeln!(
                out,
                "{} share{} needed for recovery",
                ctx.threshold(),
                maybe_plural(ctx.threshold().get())
            )
        })?;
    } else {
        ui.with_message(|_, out| {
            writeln!(
                out,
                "Out of {} shares needed for recovery, {} were retrieved.",
                ctx.threshold(),
                ctx.retrieved_shares()
                    .iter()
                    .fold(0_u8, |acc, (_, s)| acc + u8::from(*s))
            )
        })?;
    }
    let selection = FuzzySelect::new()
        .default(0)
        .with_prompt("Choose a person to contact now")
        .items(human_recipients)
        .interact()?;
    Ok(ctx.pick_recipient(recipients[selection].user_id)?)
}

/// Step 1: wait peer
async fn step1_user(
    ui: &crate::Ui,
    ctx: UserClaimAdministratorPick,
) -> anyhow::Result<UserClaimInProgress1Ctx> {
    match ctx {
        UserClaimAdministratorPick::Single(ctx) => {
            ui.with_message(|fmt, out| {
                writeln!(
                    out,
                    "Invitation greeter: {}",
                    fmt.wrap_in_color(Color::Yellow, ctx.greeter_human_handle())
                )
            })?;

            let handle = ui.with_spinner(|_, out| {
                write!(out, "Waiting the greeter to start the invitation procedure")
            })?;

            let ctx = ctx.do_wait_peer().await?;

            handle.stop_with_symbol(make_checkmark_symbol)?;

            Ok(ctx)
        }
        UserClaimAdministratorPick::Multiple(ctxs) => {
            let handle = ui.with_spinner(|_, out| {
                write!(
                    out,
                    "Waiting for an administrator to start the invitation procedure"
                )
            })?;

            let ctx = UserClaimInitialCtx::do_wait_multiple_peer(ctxs).await?;

            handle.stop_with_symbol(make_checkmark_symbol)?;

            ui.with_message(|fmt, out| {
                writeln!(
                    out,
                    "Invitation greeter: {}",
                    fmt.wrap_in_color(Color::Yellow, ctx.greeter_human_handle())
                )
            })?;

            Ok(ctx)
        }
    }
}

/// Step 1: wait peer
async fn step1_device(
    ui: &crate::Ui,
    ctx: DeviceClaimInitialCtx,
) -> anyhow::Result<DeviceClaimInProgress1Ctx> {
    ui.with_message(|fmt, out| {
        writeln!(
            out,
            "Invitation greeter: {}",
            fmt.wrap_in_color(Color::Yellow, ctx.greeter_human_handle())
        )
    })?;

    let handle = ui.with_spinner(|_, out| {
        write!(out, "Waiting the greeter to start the invitation procedure")
    })?;

    let ctx = ctx.do_wait_peer().await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

/// Step 1: wait peer
async fn step1_shamir(
    ui: &crate::Ui,
    ctx: ShamirRecoveryClaimInitialCtx,
) -> anyhow::Result<ShamirRecoveryClaimInProgress1Ctx> {
    ui.with_message(|fmt, out| {
        writeln!(
            out,
            "Invitation greeter: {}",
            fmt.wrap_in_color(Color::Yellow, ctx.greeter_human_handle())
        )
    })?;

    let handle = ui.with_spinner(|_, out| {
        write!(
            out,
            "Waiting the greeter {} to start the invitation procedure",
            ctx.greeter_human_handle()
        )
    })?;

    let ctx = ctx.do_wait_peer().await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

/// Step 2: signify trust
async fn step2_user(ctx: UserClaimInProgress1Ctx) -> anyhow::Result<UserClaimInProgress2Ctx> {
    let sas_codes = ctx.generate_greeter_sas_choices(3);

    choose_sas_code(&sas_codes, ctx.greeter_sas(), "greeter")?;

    Ok(ctx.do_signify_trust().await?)
}

/// Step 2: signify trust
async fn step2_device(ctx: DeviceClaimInProgress1Ctx) -> anyhow::Result<DeviceClaimInProgress2Ctx> {
    let sas_codes = ctx.generate_greeter_sas_choices(3);

    choose_sas_code(&sas_codes, ctx.greeter_sas(), "greeter")?;

    Ok(ctx.do_signify_trust().await?)
}

/// Step 2: signify trust
async fn step2_shamir(
    ctx: ShamirRecoveryClaimInProgress1Ctx,
) -> anyhow::Result<ShamirRecoveryClaimInProgress2Ctx> {
    let sas_codes = ctx.generate_greeter_sas_choices(3);

    choose_sas_code(&sas_codes, ctx.greeter_sas(), "greeter")?;

    Ok(ctx.do_signify_trust().await?)
}

/// Step 3: wait peer trust
async fn step3_user(
    ui: &crate::Ui,
    ctx: UserClaimInProgress2Ctx,
) -> anyhow::Result<UserClaimInProgress3Ctx> {
    ui.with_message(|fmt, out| {
        writeln!(
            out,
            "Code to provide to greeter: {}",
            fmt.wrap_in_color(Color::Yellow, ctx.claimer_sas())
        )
    })?;

    let handle = ui.with_spinner(|_, out| write!(out, "Waiting for greeter"))?;

    let ctx = ctx.do_wait_peer_trust().await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

/// Step 3: wait peer trust
async fn step3_shamir(
    ui: &crate::Ui,
    ctx: ShamirRecoveryClaimInProgress2Ctx,
) -> anyhow::Result<ShamirRecoveryClaimInProgress3Ctx> {
    ui.with_message(|fmt, out| {
        writeln!(
            out,
            "Code to provide to greeter: {}",
            fmt.wrap_in_color(Color::Yellow, ctx.claimer_sas())
        )
    })?;

    let handle = ui.with_spinner(|_, out| write!(out, "Waiting for greeter"))?;

    let ctx = ctx.do_wait_peer_trust().await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

/// Step 3: wait peer trust
async fn step3_device(
    ui: &crate::Ui,
    ctx: DeviceClaimInProgress2Ctx,
) -> anyhow::Result<DeviceClaimInProgress3Ctx> {
    ui.with_message(|fmt, out| {
        writeln!(
            out,
            "Code to provide to greeter: {}",
            fmt.wrap_in_color(Color::Yellow, ctx.claimer_sas())
        )
    })?;

    let handle = ui.with_spinner(|_, out| write!(out, "Waiting for greeter"))?;

    let ctx = ctx.do_wait_peer_trust().await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

/// Step 4: claim user
async fn step4_user(
    ui: &crate::Ui,
    ctx: UserClaimInProgress3Ctx,
) -> anyhow::Result<UserClaimFinalizeCtx> {
    let mut input = String::new();
    let device_label = choose_device_label(ui, &mut input)?;
    let human_handle = choose_human_handle(ui, &mut input)?;

    let handle = ui.with_spinner(|_, out| write!(out, "Waiting for greeter"))?;

    let ctx = ctx.do_claim_user(device_label, human_handle).await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

/// Step 4: claim device
async fn step4_device(
    ui: &crate::Ui,
    ctx: DeviceClaimInProgress3Ctx,
) -> anyhow::Result<DeviceClaimFinalizeCtx> {
    let mut input = String::new();
    let device_label = choose_device_label(ui, &mut input)?;

    let handle = ui.with_spinner(|_, out| write!(out, "Waiting for greeter"))?;

    let ctx = ctx.do_claim_device(device_label).await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

/// Step 4: retrieve share
async fn step4_shamir(
    ui: &crate::Ui,
    ctx: ShamirRecoveryClaimInProgress3Ctx,
) -> anyhow::Result<ShamirRecoveryClaimShare> {
    let handle = ui.with_spinner(|_, out| write!(out, "Waiting for greeter"))?;

    let ctx = ctx.do_recover_share().await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

/// Step 5: recover device
async fn step5_shamir(
    ui: &crate::Ui,
    ctx: ShamirRecoveryClaimRecoverDeviceCtx,
) -> anyhow::Result<ShamirRecoveryClaimMaybeFinalizeCtx> {
    let device_label = Input::new().with_prompt("Enter device label").interact()?;

    let handle = ui.with_spinner(|_, out| write!(out, "Recovering device"))?;

    let ctx = ctx.recover_device(device_label).await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    Ok(ctx)
}

async fn save_user(
    ui: &crate::Ui,
    ctx: UserClaimFinalizeCtx,
    save_strategy: libparsec_client::DeviceSaveStrategy,
) -> anyhow::Result<()> {
    let key_file = ctx.get_default_key_file();
    let new_device = ctx
        .save_local_device(&save_strategy, &key_file)
        .await
        .map(ShortAvailableDeviceDisplay::from)?;

    print_new_device(ui, &new_device)
}

async fn save_device(
    ui: &crate::Ui,
    ctx: DeviceClaimFinalizeCtx,
    save_strategy: libparsec_client::DeviceSaveStrategy,
) -> anyhow::Result<()> {
    let key_file = ctx.get_default_key_file();
    let new_device = ctx
        .save_local_device(&save_strategy, &key_file)
        .await
        .map(ShortAvailableDeviceDisplay::from)?;

    print_new_device(ui, &new_device)
}

async fn save_shamir_recovery(
    ui: &crate::Ui,
    ctx: ShamirRecoveryClaimFinalizeCtx,
    save_strategy: libparsec_client::DeviceSaveStrategy,
) -> anyhow::Result<()> {
    let key_file = ctx.get_default_key_file();
    let new_device = ctx
        .save_local_device(&save_strategy, &key_file)
        .await
        .map(ShortAvailableDeviceDisplay::from)?;

    print_new_device(ui, &new_device)
}

fn print_new_device(ui: &crate::Ui, device: &ShortAvailableDeviceDisplay) -> anyhow::Result<()> {
    ui.with_message(|_, out| writeln!(out, "New device created:"))?;
    ui.data_print(device)?;

    Ok(())
}
