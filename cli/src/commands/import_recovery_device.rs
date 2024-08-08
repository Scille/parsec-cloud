// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::{Path, PathBuf};

use libparsec::{load_recovery_device, DeviceAccessStrategy};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, password_stdin]
    pub struct ImportRecoveryDevice {
        /// Recovery file
        #[arg(short, long)]
        input: PathBuf,
        /// Passphrase
        #[arg(short, long)]
        passphrase: String,
    }
);

pub async fn import_recovery_device(
    import_recovery_device: ImportRecoveryDevice,
) -> anyhow::Result<()> {
    let ImportRecoveryDevice {
        input,
        passphrase,
        config_dir,
        password_stdin,
    } = import_recovery_device;
    log::trace!(
        "Importing recovery device from {} (confdir={})",
        input.display(),
        config_dir.display(),
    );

    let mut handle = start_spinner("Loading recovery device file".into());

    // TODO this is a recovery device: a new local device must be created
    let device = load_recovery_device(&input, passphrase.into()).await?;

    handle.stop_with_newline();

    let password = choose_password(if password_stdin {
        ReadPasswordFrom::Stdin
    } else {
        ReadPasswordFrom::Tty {
            prompt: "Enter password for the new device:",
        }
    })?;

    let key_file = libparsec::get_default_key_file(&config_dir, &device);

    let access = DeviceAccessStrategy::Password { key_file, password };

    let mut handle = start_spinner("Saving new device".into());

    libparsec::save_device(Path::new(""), &access, &device).await?;

    handle.stop_with_message("Saved new device".into());

    Ok(())
}
