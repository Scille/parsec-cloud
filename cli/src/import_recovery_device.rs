// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use std::path::{Path, PathBuf};

use libparsec::{get_default_config_dir, load_recovery_device, DeviceAccessStrategy};

use crate::utils::*;

#[derive(Args)]
pub struct ImportRecoveryDevice {
    /// Parsec config directory
    #[arg(short, long, default_value_os_t = get_default_config_dir())]
    config_dir: PathBuf,
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
    let handle = start_spinner("Loading recovery device file");

    let device = load_recovery_device(
        &import_recovery_device.input,
        import_recovery_device.passphrase.into(),
    )
    .await?;

    handle.done();

    let password = choose_password()?;

    let key_file = libparsec::get_default_key_file(&import_recovery_device.config_dir, &device);

    let access = DeviceAccessStrategy::Password { key_file, password };

    let handle = start_spinner("Saving new device");

    libparsec::save_device(Path::new(""), &access, &device).await?;

    handle.done();

    println!("Saved new device");

    Ok(())
}
