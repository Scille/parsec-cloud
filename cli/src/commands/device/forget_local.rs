// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use dialoguer::Confirm;
use libparsec::AvailableDevice;
use libparsec_client::remove_device;

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device]
    pub struct Args {}
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args { device, config_dir } = args;
    log::trace!(
        "Forgetting local device {} (confdir={})",
        device.as_deref().unwrap_or("N/A"),
        config_dir.display(),
    );

    // FIXME: https://github.com/Scille/parsec-cloud/issues/8604
    // The client config should be loaded from a config file
    let config = libparsec_client::ClientConfig::from(libparsec::ClientConfig {
        config_dir,
        ..Default::default()
    });

    let device = load_device_file(&config.config_dir, device).await?;

    let short_id = &device.device_id.hex()[..3];
    let AvailableDevice {
        organization_id,
        human_handle,
        device_label,
        ..
    } = &device;

    println!("You are about to forget the following local device:");
    println!("{YELLOW}{short_id}{RESET} - {organization_id}: {human_handle} @ {device_label}");

    if !Confirm::new().with_prompt("Are you sure?").interact()? {
        println!("Operation cancelled");
    } else {
        remove_device(&config, &device).await?;
        println!("The local device has been forgotten");
    }

    Ok(())
}
