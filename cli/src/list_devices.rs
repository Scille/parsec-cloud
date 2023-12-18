// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use std::path::PathBuf;

use libparsec::{get_default_config_dir, list_available_devices};

use crate::utils::*;

#[derive(Args)]
pub struct ListDevices {
    /// Parsec config directory
    #[arg(short, long, default_value_os_t = get_default_config_dir())]
    config_dir: PathBuf,
}

pub async fn list_devices(list_devices: ListDevices) -> anyhow::Result<()> {
    let config_dir = list_devices.config_dir;
    let config_dir_str = config_dir.to_string_lossy();
    let devices = list_available_devices(&config_dir).await;

    if devices.is_empty() {
        println!("No devices found in {YELLOW}{config_dir_str}{RESET}");
    } else {
        let n = devices.len();
        println!("Found {GREEN}{n}{RESET} device(s) in {YELLOW}{config_dir_str}{RESET}:");
        format_devices(&devices);
    }

    Ok(())
}
