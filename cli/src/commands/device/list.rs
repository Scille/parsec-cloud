// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::list_available_devices;
use std::io::Write as _;

use crate::ui::{compat::ShortAvailableDeviceDisplay, Color};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir]
    pub struct Args {}
);

pub async fn main(ui: crate::Ui, args: Args) -> anyhow::Result<()> {
    let config_dir = args.config_dir;
    log::trace!("Listing devices under {}", config_dir.display());
    let devices = list_available_devices(&config_dir)
        .await?
        .into_iter()
        .map(ShortAvailableDeviceDisplay::from)
        .collect::<Vec<_>>();

    if devices.is_empty() {
        ui.with_message(|fmt, out| {
            writeln!(
                out,
                "No devices found in {dir}",
                dir = fmt.wrap_in_color(Color::Yellow, config_dir.display())
            )
        })?;
    } else {
        let n = devices.len();
        ui.with_message(|fmt, out| {
            writeln!(
                out,
                "Found {num_found} device(s) in {dir}",
                num_found = fmt.wrap_in_color(Color::Green, n),
                dir = fmt.wrap_in_color(Color::Yellow, config_dir.display())
            )
        })?;

        ui.data_print(&devices.as_slice())?;
    }

    Ok(())
}
