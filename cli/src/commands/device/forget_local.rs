// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::io::{IsTerminal, Write};

use dialoguer::Confirm;
use libparsec_client::remove_device;

use crate::{
    ui::{compat::AvailableDeviceDisplay, CLIDisplay},
    utils::*,
};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, force]
    pub struct Args {
    }
);

pub async fn main(ui: crate::Ui, args: Args) -> anyhow::Result<()> {
    let Args {
        device,
        config_dir,
        force,
    } = args;
    log::trace!(
        "Forgetting local device {} (confdir={})",
        device.as_deref().unwrap_or("N/A"),
        config_dir.display(),
    );

    anyhow::ensure!(
        std::io::stdout().is_terminal() || force,
        "Need to pass `--force` when not a terminal"
    );

    // FIXME: https://github.com/Scille/parsec-cloud/issues/8604
    // The client config should be loaded from a config file
    let config = libparsec_client::ClientConfig::from(libparsec::ClientConfig {
        config_dir,
        ..Default::default()
    });

    let device = load_device_file(&config.config_dir, device)
        .await
        .map(AvailableDeviceDisplay)?;

    ui.with_message(|fmt, out| {
        writeln!(out, "You are about to forget the following local device:")?;
        device.plain_write(&fmt, &mut *out)?;
        out.write_all(b"\n")
    })?;

    if force || Confirm::new().with_prompt("Are you sure?").interact()? {
        remove_device(&config, &device).await?;
        ui.with_message(|_fmt, out| writeln!(out, "The local device has been forgotten"))?;
    } else {
        ui.with_message(|_fmt, out| writeln!(out, "Operation cancelled"))?;
    }

    Ok(())
}
