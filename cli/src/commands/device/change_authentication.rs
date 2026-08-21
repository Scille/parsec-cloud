// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::io::Write;

use libparsec::{AvailableDeviceType, DeviceAccessStrategy, DevicePrimaryProtectionStrategy};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin, auth]
    pub struct Args {
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

    let current_access_strategy = match device.ty {
        AvailableDeviceType::Password => {
            let password = read_password(if args.password_stdin {
                ReadPasswordFrom::Stdin
            } else {
                ReadPasswordFrom::Tty {
                    prompt: "Enter current password for the device:",
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

    let new_save_strategy = args.auth.get_save_strategy(args.password_stdin)?;

    libparsec::update_device_change_authentication(
        &args.config_dir,
        current_access_strategy,
        new_save_strategy,
    )
    .await?;

    ui.with_message(|_, out| writeln!(out, "Device authentication changed successfully"))?;

    Ok(())
}
