// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use std::path::PathBuf;

use libparsec::{get_default_config_dir, save_recovery_device};

use crate::utils::*;

#[derive(Args)]
pub struct ExportRecoveryDevice {
    /// Parsec config directory
    #[arg(short, long, default_value_os_t = get_default_config_dir())]
    config_dir: PathBuf,
    /// Device slughash
    #[arg(short, long)]
    device: Option<String>,
    /// Recovery device output
    #[arg(short, long)]
    output: PathBuf,
}

pub async fn export_recovery_device(
    export_recovery_device: ExportRecoveryDevice,
) -> anyhow::Result<()> {
    let ExportRecoveryDevice {
        config_dir,
        device,
        output,
    } = export_recovery_device;

    load_device_and_run(config_dir, device, |device| async move {
        let mut handle = start_spinner("Saving recovery device file".into());

        let passphrase = save_recovery_device(&output, &device).await?;

        handle.stop_with_message(
            format!(
                "Saved in {}\n{RED}Save the recovery passphrase in a safe place:{RESET} {GREEN}{}{RESET}",
                output.display(),
                passphrase.as_str()
            )
        );

        Ok(())
    })
    .await
}
