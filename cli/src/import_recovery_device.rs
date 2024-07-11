// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::{Path, PathBuf};

use libparsec::{load_recovery_device, DeviceAccessStrategy};

use crate::utils::*;

#[derive(clap::Parser)]
pub struct ImportRecoveryDevice {
    #[clap(flatten)]
    config: ConfigSharedOpts,
    /// Recovery file
    #[arg(short, long)]
    input: PathBuf,
    /// Passphrase
    #[arg(short, long)]
    passphrase: String,
}

pub async fn import_recovery_device(
    import_recovery_device: ImportRecoveryDevice,
) -> anyhow::Result<()> {
    let ImportRecoveryDevice {
        config: ConfigSharedOpts { config_dir },
        input,
        passphrase,
    } = import_recovery_device;
    log::trace!(
        "Importing recovery device from {} (confdir={})",
        input.display(),
        config_dir.display(),
    );

    let mut handle = start_spinner("Loading recovery device file".into());

    let device = load_recovery_device(&input, passphrase.into()).await?;

    handle.stop_with_newline();

    let password = choose_password()?;

    let key_file = libparsec::get_default_key_file(&config_dir, &device);

    let access = DeviceAccessStrategy::Password { key_file, password };

    let mut handle = start_spinner("Saving new device".into());

    libparsec::save_device(Path::new(""), &access, &device).await?;

    handle.stop_with_message("Saved new device".into());

    Ok(())
}
