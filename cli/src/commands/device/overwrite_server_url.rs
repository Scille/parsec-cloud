// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use std::io::Write;

use dialoguer::Confirm;
use libparsec::ParsecAddr;

use crate::{
    ui::{Color, compat::AvailableDeviceDisplay},
    utils::*,
};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, force]
    pub struct Args {
        /// The new server URL
        #[arg(short, long, value_parser = ParsecAddr::from_http_url, value_hint = clap::ValueHint::Url)]
        server_url: ParsecAddr,
    }
);

pub async fn main(ui: crate::Ui, args: Args) -> anyhow::Result<()> {
    let device = load_device_file(&args.config_dir, args.device.clone()).await?;

    let device = AvailableDeviceDisplay(device);

    ui.with_message(|_, out| write!(out, "You are about to update the following device:"))?;
    ui.message_println(&device)?;
    ui.with_message(|fmt, out| {
        writeln!(
            out,
            "Current server URL: {url}",
            url = fmt.wrap_in_color(Color::Yellow, &device.server_addr)
        )
    })?;
    ui.with_message(|fmt, out| {
        writeln!(
            out,
            "New server URL: {url}",
            url = fmt.wrap_in_color(Color::Yellow, &args.server_url)
        )
    })?;

    if args.force || Confirm::new().with_prompt("Are you sure? ").interact()? {
        let access_strategy = crate::utils::get_access_strategy_for_device(
            &args.config_dir,
            args.device,
            Some(args.server_url.clone()),
        )
        .await?;

        libparsec_platform_device_loader::update_device_overwrite_server_addr(
            &args.config_dir,
            &access_strategy,
            args.server_url.clone(),
        )
        .await?;

        ui.with_message(|_, out| write!(out, "Device updated successfully"))?;
    } else {
        ui.with_message(|_, out| writeln!(out, "Operation cancelled"))?;
    }

    Ok(())
}
