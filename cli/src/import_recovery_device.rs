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
    let ImportRecoveryDevice {
        input,
        passphrase,
        config_dir,
    } = import_recovery_device;

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
