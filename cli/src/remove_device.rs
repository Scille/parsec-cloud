// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use std::path::PathBuf;

use libparsec::{get_default_config_dir, list_available_devices};

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
    let devices = list_available_devices(&remove_device.config_dir).await;

    if let Some(device_slughash) = remove_device.device {
        let mut possible_devices = vec![];

        for device in &devices {
            if device.slughash().starts_with(&device_slughash) {
                possible_devices.push(device);
            }
        }

        match possible_devices.len() {
            0 => {
                println!("Device `{device_slughash}` not found, available devices:");
                format_devices(&devices);
            }
            1 => {
                let device = possible_devices[0];
                let slug = &device.slughash()[..3];
                let organization_id = &device.organization_id;
                let human_handle = &device.human_handle;
                let device_label = &device.device_label;

                println!("You are about to remove the following device:");
                println!(
                    "{YELLOW}{slug}{RESET} - {organization_id}: {human_handle} @ {device_label}"
                );
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
            }
            _ => {
                println!("Multiple devices found for `{device_slughash}`:");
                format_devices(&devices);
            }
        }
    } else {
        println!("Error: Missing option '--device'\n");
        println!("Available devices:");
        format_devices(&devices);
    }

    Ok(())
}
