// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use libparsec::{DateTime, DeviceLabel};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Path where to save recovery device data
        output: PathBuf,
    }
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        output,
        device,
        config_dir,
        password_stdin,
    } = args;
    log::trace!(
        "Exporting recovery device at {} (confdir={}, device={})",
        output.display(),
        config_dir.display(),
        device.as_deref().unwrap_or("N/A")
    );

    let mut handle = start_spinner("Saving recovery device file".into());

    let client = load_client(&config_dir, device.clone(), password_stdin).await?;

    let now = DateTime::now();
    let (passphrase, data) = client
        .export_recovery_device(DeviceLabel::try_from(format!("recovery-{now}").as_str())?)
        .await?;

    tokio::fs::write(&output, data).await?;

    handle.stop_with_message(format!(
        "Recovery device saved at {path}\n{RED}Save the recovery passphrase in a safe place:{RESET} {GREEN}{passwd}{RESET}",
        path = output.display(),
        passwd = passphrase.as_str()
    ));

    Ok(())
}
