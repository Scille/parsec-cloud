// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use libparsec::save_recovery_device;

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Recovery device output
        #[arg(short, long)]
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

    let device = load_and_unlock_device(&config_dir, device, password_stdin).await?;
    let mut handle = start_spinner("Saving recovery device file".into());

    // TODO: save recovery device instead of local device
    // The recovery device must be generated first
    let passphrase = save_recovery_device(&output, &device).await?;

    handle.stop_with_message(format!(
        "Saved in {}\n{RED}Save the recovery passphrase in a safe place:{RESET} {GREEN}{}{RESET}",
        output.display(),
        passphrase.as_str()
    ));

    Ok(())
}
