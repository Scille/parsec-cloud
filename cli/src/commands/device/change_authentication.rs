// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{DeviceAccessStrategy, DeviceFileType, DeviceSaveStrategy};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
   #[clap(long, short, action)]
        password: bool,
   #[clap(long, short, action)]
        keyring: bool,
    }
);

enum NewAccessStrategyChoice {
    Password,
    Keyring,
}

pub async fn main(args: Args) -> anyhow::Result<()> {
    let device = load_device_file(&args.config_dir, args.device).await?;

    let new_save_strategy_choice = match (args.password, args.keyring) {
        (true, false) => NewAccessStrategyChoice::Password,
        (false, true) => NewAccessStrategyChoice::Keyring,
        (true, true) => {
            return Err(anyhow::anyhow!(
                "Only one of --password and --keyring can be specified"
            ));
        }
        (false, false) => {
            return Err(anyhow::anyhow!(
                "One of --password and --keyring must be specified"
            ));
        }
    };

    let current_access_strategy = match device.ty {
        DeviceFileType::Password => {
            let password = read_password(if args.password_stdin {
                ReadPasswordFrom::Stdin
            } else {
                ReadPasswordFrom::Tty {
                    prompt: "Enter current password for the device:",
                }
            })?;

            DeviceAccessStrategy::Password {
                key_file: device.key_file_path.clone(),
                password,
            }
        }

        DeviceFileType::Smartcard => DeviceAccessStrategy::Smartcard {
            key_file: device.key_file_path.clone(),
        },

        DeviceFileType::Keyring => DeviceAccessStrategy::Keyring {
            key_file: device.key_file_path.clone(),
        },

        DeviceFileType::AccountVault => {
            // In theory we should support this authentication method here,
            // however:
            // - It is cumbersome since it requires obtaining the account authentication
            //   info (login&password) from the CLI parameters.
            // - In practice it is only used on web, where CLI is never going to be used.
            return Err(LoadAndUnlockDeviceError::UnsupportedAuthentication(device.ty).into());
        }

        DeviceFileType::Recovery => {
            return Err(LoadAndUnlockDeviceError::UnsupportedAuthentication(device.ty).into());
        }
    };

    let new_save_strategy = match new_save_strategy_choice {
        NewAccessStrategyChoice::Password => {
            let password = choose_password(if args.password_stdin {
                ReadPasswordFrom::Stdin
            } else {
                ReadPasswordFrom::Tty {
                    prompt: "Enter new password for the device:",
                }
            })?;

            DeviceSaveStrategy::Password { password }
        }

        NewAccessStrategyChoice::Keyring => DeviceSaveStrategy::Keyring,
    };

    libparsec::update_device_change_authentication(
        &args.config_dir,
        current_access_strategy,
        new_save_strategy,
    )
    .await?;

    println!("Device authentication changed successfully");

    Ok(())
}
