// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use libparsec::save_recovery_device;

use crate::utils::*;

#[derive(clap::Parser)]
pub struct ExportRecoveryDevice {
    #[clap(flatten)]
    config: ConfigWithDeviceSharedOpts,
    /// Recovery device output
    #[arg(short, long)]
    output: PathBuf,
}

pub async fn export_recovery_device(
    export_recovery_device: ExportRecoveryDevice,
) -> anyhow::Result<()> {
    let ExportRecoveryDevice {
        config: ConfigWithDeviceSharedOpts { config_dir, device },
        output,
    } = export_recovery_device;
    log::trace!(
        "Exporting recovery device at {} (confdir={}, device={})",
        output.display(),
        config_dir.display(),
        device.as_deref().unwrap_or("N/A")
    );

    load_device_and_run(config_dir, device, |device| async move {
        let mut handle = start_spinner("Saving recovery device file".into());

        // TODO save recovery device instead of local device
        // The recovery device must be generated first
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
