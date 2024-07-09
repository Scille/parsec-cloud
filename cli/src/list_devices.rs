// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::list_available_devices;

use crate::utils::*;

#[derive(clap::Parser)]
pub struct ListDevices {
    #[clap(flatten)]
    config: ConfigSharedOpts,
}

pub async fn list_devices(list_devices: ListDevices) -> anyhow::Result<()> {
    let config_dir = list_devices.config.config_dir;
    let devices = list_available_devices(&config_dir).await;
    let config_dir_str = config_dir.to_string_lossy();

    if devices.is_empty() {
        println!("No devices found in {YELLOW}{config_dir_str}{RESET}");
    } else {
        let n = devices.len();
        println!("Found {GREEN}{n}{RESET} device(s) in {YELLOW}{config_dir_str}{RESET}:");
        format_devices(&devices);
    }

    Ok(())
}
