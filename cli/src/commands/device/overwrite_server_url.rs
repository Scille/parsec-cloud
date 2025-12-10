// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{AvailableDevice, AvailableDeviceType, DeviceAccessStrategy, ParsecAddr};

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
        AvailableDeviceType::Password => {
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

        AvailableDeviceType::Smartcard => {
            todo!("read smartcard #11270");
            // DeviceAccessStrategy::Smartcard {
            //     certificate_reference: todo!(),
            //     key_file: device.key_file_path.clone(),
            // }
        }

        AvailableDeviceType::PKI { .. } => {
            todo!("read PKI #11270");
            // DeviceAccessStrategy::PKI {
            //     certificate_reference: todo!(),
            //     key_file: device.key_file_path.clone(),
            // }
        }

        AvailableDeviceType::Keyring => DeviceAccessStrategy::Keyring {
            key_file: device.key_file_path.clone(),
        },

        AvailableDeviceType::AccountVault => {
            // In theory we should support this authentication method here,
            // however:
            // - It is cumbersome since it requires obtaining the account authentication
            //   info (login&password) from the CLI parameters.
            // - In practice it is only used on web, where CLI is never going to be used.
            return Err(LoadAndUnlockDeviceError::UnsupportedAuthentication(device.ty).into());
        }

        AvailableDeviceType::Recovery => {
            return Err(LoadAndUnlockDeviceError::UnsupportedAuthentication(device.ty).into());
        }

        AvailableDeviceType::OpenBao { .. } => {
            // In theory we should support this authentication method here,
            // however:
            // - It is cumbersome since it requires opening a browser window for login
            //   and redirect its result to a server listening on localhost...
            // - In practice it is a niche usage that will most likely only be used in the GUI.
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
    println!("Current server URL: {YELLOW}{}{RESET}", device.server_addr);
    println!("New server URL: {YELLOW}{}{RESET}", args.server_url);
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
