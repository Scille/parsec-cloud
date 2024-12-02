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

crate::build_main_with_client!(main, export_recovery_device);

pub async fn export_recovery_device(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args { output, .. } = args;
    log::trace!("Exporting recovery device at {}", output.display());

    let mut handle = start_spinner("Saving recovery device file".into());

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
