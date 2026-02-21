// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use libparsec::DeviceLabel;

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, password_stdin]
    pub struct Args {
        /// Path where encrypted recovery device data is
        #[arg(short, long)]
        input: PathBuf,
        /// new device label
        #[arg(short, long)]
        label: String,
    }
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        input,
        config_dir,
        password_stdin,
        label,
    } = args;
    log::trace!(
        "Importing recovery device from {} (confdir={})",
        input.display(),
        config_dir.display(),
    );
    let recovery_device = tokio::fs::read(input).await?;

    let passphrase = read_password(if password_stdin {
        ReadPasswordFrom::Stdin
    } else {
        ReadPasswordFrom::Tty {
            prompt: "Enter passphrase for the recovery file:",
        }
    })?;
    let password = choose_password(if password_stdin {
        ReadPasswordFrom::Stdin
    } else {
        ReadPasswordFrom::Tty {
            prompt: "Enter password for the new device:",
        }
    })?;

    let device_label = DeviceLabel::try_from(label.as_str())?;
    let new_device = libparsec_client::import_recovery_device(
        &config_dir,
        &recovery_device,
        passphrase.to_string(),
        device_label.clone(),
        libparsec_client::DeviceSaveStrategy::new_password(password),
    )
    .await?;

    println!("New device created:");
    println!("{}", &format_single_device(&new_device));

    Ok(())
}
