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

    let mut handle = start_spinner("Saving new device".into());

    libparsec_client::import_recovery_device(
        &config_dir,
        &recovery_device,
        passphrase.to_string(),
        DeviceLabel::try_from(label.as_str())?,
        libparsec::DeviceSaveStrategy::Password { password },
    )
    .await?;

    handle.stop_with_message("Saved new device".into());

    Ok(())
}
