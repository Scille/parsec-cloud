// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{client_change_authentication, ClientConfig, DeviceSaveStrategy};

use crate::utils::*;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum AuthenticationMethod {
    Keyring,
    Password,
}

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Use keyring to store the password for the device.
        #[arg(long)]
        method: AuthenticationMethod,
    }
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        config_dir,
        password_stdin,
        method,
        device,
        ..
    } = args;

    // Get current device access strategy
    let current_auth = get_device_access_strategy(&config_dir, device, password_stdin).await?;

    // Get new device access strategy
    let new_auth = match method {
        AuthenticationMethod::Keyring => DeviceSaveStrategy::Keyring,
        AuthenticationMethod::Password => {
            let password = choose_password(ReadPasswordFrom::Tty {
                prompt: "Enter the new password for the device:",
            })?;
            DeviceSaveStrategy::Password { password }
        }
    };

    // Update device access strategy
    let mut handle = start_spinner("Updating authentication".into());

    let config = ClientConfig {
        config_dir,
        ..Default::default()
    };
    client_change_authentication(config, current_auth, new_auth).await?;

    handle.stop_with_message(format!("Authentication updated to {:?}", method,));

    Ok(())
}
