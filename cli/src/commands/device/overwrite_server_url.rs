// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{AvailableDevice, DeviceAccessStrategy, DeviceFileType, ParsecAddr};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// The new server URL
        #[arg(short, long, value_parser = ParsecAddr::from_http_url)]
        server_url: ParsecAddr,
    }
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let device = load_device_file(&args.config_dir, args.device).await?;

    let access_strategy = match device.ty {
        DeviceFileType::Password => {
            let password = read_password(if args.password_stdin {
                ReadPasswordFrom::Stdin
            } else {
                ReadPasswordFrom::Tty {
                    prompt: "Enter password for the device:",
                }
            })?;

            DeviceAccessStrategy::Password {
                key_file: device.key_file_path.clone(),
                password: password.clone(),
            }
        }

        DeviceFileType::Smartcard => DeviceAccessStrategy::Smartcard {
            key_file: device.key_file_path.clone(),
        },

        DeviceFileType::Keyring => DeviceAccessStrategy::Keyring {
            key_file: device.key_file_path.clone(),
        },

        DeviceFileType::Recovery => {
            return Err(LoadAndUnlockDeviceError::UnsupportedAuthentication(device.ty).into());
        }
    };

    let short_id = &device.device_id.hex()[..3];
    let AvailableDevice {
        organization_id,
        human_handle,
        device_label,
        ..
    } = &device;

    println!("You are about to update the following device:");
    println!("{YELLOW}{short_id}{RESET} - {organization_id}: {human_handle} @ {device_label}");
    println!("Current server URL: {YELLOW}{}{RESET}", device.server_url);
    println!(
        "New server URL: {YELLOW}{}{RESET}",
        args.server_url.to_http_url(None)
    );
    println!("Are you sure? (y/n)");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    match input.trim() {
        "y" => (),
        _ => {
            eprintln!("Operation cancelled");
            return Ok(());
        }
    }

    libparsec::update_device_overwrite_server_addr(
        &args.config_dir,
        access_strategy,
        args.server_url.clone(),
    )
    .await?;

    println!("Device updated successfully");

    Ok(())
}
