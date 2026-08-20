// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::io::Write;

use dialoguer::Confirm;
use libparsec::{
    AvailableDeviceType, DeviceAccessStrategy, DevicePrimaryProtectionStrategy, ParsecAddr,
};

use crate::{
    ui::{compat::AvailableDeviceDisplay, Color},
    utils::*,
};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin, force]
    pub struct Args {
        /// The new server URL
        #[arg(short, long, value_parser = ParsecAddr::from_http_url, value_hint = clap::ValueHint::Url)]
        server_url: ParsecAddr,
    }
);

pub async fn main(ui: crate::Ui, args: Args) -> anyhow::Result<()> {
    let device = load_device_file(&args.config_dir, args.device).await?;

    if device.totp_opaque_key_id.is_some() {
        // In theory we should support this authentication method here,
        // however:
        // - It is cumbersome since it requires a TOTP challenge involving the server.
        // - In practice it is a niche usage that will most likely only be used in the GUI.
        return Err(LoadAndUnlockDeviceError::UnsupportedTOTPAuthentication.into());
    }

    let access_strategy = match device.ty {
        AvailableDeviceType::Password => {
            let password = read_password(if args.password_stdin {
                ReadPasswordFrom::Stdin
            } else {
                ReadPasswordFrom::Tty {
                    prompt: "Enter password for the device:",
                }
            })?;

            DeviceAccessStrategy {
                key_file: device.key_file_path.clone(),
                totp_protection: None,
                primary_protection: DevicePrimaryProtectionStrategy::Password { password },
            }
        }

        AvailableDeviceType::PKI { .. } => {
            todo!("read PKI #11270");
            // DeviceAccessStrategy::PKI {
            //     certificate_reference: todo!(),
            //     key_file: device.key_file_path.clone(),
            // }
        }

        AvailableDeviceType::Keyring => DeviceAccessStrategy {
            key_file: device.key_file_path.clone(),
            totp_protection: None,
            primary_protection: DevicePrimaryProtectionStrategy::Keyring,
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
        libparsec::update_device_overwrite_server_addr(
            &args.config_dir,
            access_strategy,
            args.server_url.clone(),
        )
        .await?;

        ui.with_message(|_, out| write!(out, "Device updated successfully"))?;
    } else {
        ui.with_message(|_, out| writeln!(out, "Operation cancelled"))?;
    }

    Ok(())
}
