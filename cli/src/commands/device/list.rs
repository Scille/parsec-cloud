// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::list_available_devices;
use std::{fmt::Write as _, io::Write as _};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir]
    pub struct Args {}
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let config_dir = args.config_dir;
    log::trace!("Listing devices under {}", config_dir.display());
    let devices = list_available_devices(&config_dir).await?;
    let config_dir_str = config_dir.to_string_lossy();

    if devices.is_empty() {
        println!("No devices found in {YELLOW}{config_dir_str}{RESET}");
    } else {
        let n = devices.len();
        let mut message = String::new();
        writeln!(
            message,
            "Found {GREEN}{n}{RESET} device(s) in {YELLOW}{config_dir_str}{RESET}:"
        )?;
        format_devices(&devices, &mut message)?;
        std::io::stdout().lock().write_all(message.as_bytes())?;
    }

    Ok(())
}
