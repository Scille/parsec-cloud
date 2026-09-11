// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::io::Write;

use std::path::PathBuf;

use libparsec::DeviceLabel;

use crate::{ui::compat::ShortAvailableDeviceDisplay, utils::*};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, password_stdin, auth]
    pub struct Args {
        /// Path where encrypted recovery device data is
        #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
        input: PathBuf,
        /// New device label
        #[arg(short, long, value_hint = clap::ValueHint::Other)]
        label: String,
    }
);

pub async fn main(ui: crate::Ui, args: Args) -> anyhow::Result<()> {
    let Args {
        input,
        config_dir,
        password_stdin,
        label,
        auth,
    } = args;
    // TODO: fail if password std and keyring ?
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
    let strategy = auth.get_client_save_strategy(password_stdin).await?;
    let device_label = DeviceLabel::try_from(label.as_str())?;
    let new_device = libparsec_client::import_recovery_device(
        &config_dir,
        &recovery_device,
        passphrase.to_string(),
        device_label.clone(),
        strategy,
    )
    .await
    .map(ShortAvailableDeviceDisplay::from)?;

    ui.with_message(|_, out| writeln!(out, "New device created:"))?;
    ui.data_print(&new_device)?;

    Ok(())
}
