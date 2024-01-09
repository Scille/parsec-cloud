// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use std::path::PathBuf;

use libparsec::get_default_config_dir;

use crate::utils::*;

#[derive(Args)]
pub struct RemoveDevice {
    /// Parsec config directory
    #[arg(short, long, default_value_os_t = get_default_config_dir())]
    config_dir: PathBuf,
    /// Device slughash
    #[arg(short, long)]
    device: Option<String>,
}

pub async fn remove_device(remove_device: RemoveDevice) -> anyhow::Result<()> {
    let RemoveDevice { config_dir, device } = remove_device;

    load_device_file_and_run(config_dir, device, |device| async move {
        let slug = &device.slughash()[..3];
        let organization_id = &device.organization_id;
        let human_handle = &device.human_handle;
        let device_label = &device.device_label;

        println!("You are about to remove the following device:");
        println!("{YELLOW}{slug}{RESET} - {organization_id}: {human_handle} @ {device_label}");
        println!("Are you sure? (y/n)");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        match input.trim() {
            "y" => {
                std::fs::remove_file(&device.key_file_path)?;
                println!("The device has been removed");
            }
            _ => eprintln!("Operation cancelled"),
        }

        Ok(())
    })
    .await
}
