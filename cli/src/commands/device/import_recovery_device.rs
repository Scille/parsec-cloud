// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::{Path, PathBuf};

use libparsec::{load_recovery_device, DeviceAccessStrategy};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, password_stdin]
    pub struct Args {
        /// Recovery file
        #[arg(short, long)]
        input: PathBuf,
    }
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        input,
        config_dir,
        password_stdin,
    } = args;
    log::trace!(
        "Importing recovery device from {} (confdir={})",
        input.display(),
        config_dir.display(),
    );

    let device = {
        let passphrase = read_password(if password_stdin {
            ReadPasswordFrom::Stdin
        } else {
            ReadPasswordFrom::Tty {
                prompt: "Enter passphrase for the recovery file:",
            }
        })?;
        let mut handle = start_spinner("Loading recovery device file".into());
        let res = load_recovery_device(&input, passphrase).await?;

        handle.stop_with_newline();
        res
    };

    let password = choose_password(if password_stdin {
        ReadPasswordFrom::Stdin
    } else {
        ReadPasswordFrom::Tty {
            prompt: "Enter password for the new device:",
        }
    })?;

    let key_file = libparsec::get_default_key_file(&config_dir, &device.device_id);

    let access = DeviceAccessStrategy::Password { key_file, password };

    let mut handle = start_spinner("Saving new device".into());

    libparsec::save_device(Path::new(""), &access, &device).await?;

    handle.stop_with_message("Saved new device".into());

    Ok(())
}
