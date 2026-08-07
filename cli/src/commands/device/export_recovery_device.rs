// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::fmt::Write;

use std::path::PathBuf;

use libparsec::{DateTime, DeviceLabel};

use crate::{
    ui::{CLIDisplay, Color},
    utils::*,
};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Path where to save recovery device data
        #[arg(value_hint = clap::ValueHint::FilePath)]
        output: PathBuf,
    }
);

crate::build_main_with_client!(main, export_recovery_device);

pub async fn export_recovery_device(
    ui: crate::Ui,
    args: Args,
    client: &StartedClient,
) -> anyhow::Result<()> {
    let Args { output, .. } = args;
    log::trace!("Exporting recovery device at {}", output.display());

    let handle = ui.with_spinner(|_, out| write!(out, "Saving recovery device file"))?;

    let now = DateTime::now();
    let (passphrase, data) = client
        .export_recovery_device(DeviceLabel::try_from(format!("recovery-{now}").as_str())?)
        .await?;

    tokio::fs::write(&output, data).await?;

    handle.stop();

    ui.data_print(&ExportedRecoveryDev {
        path: output,
        passphrase: passphrase.as_str(),
    })?;

    Ok(())
}

#[derive(serde::Serialize)]
struct ExportedRecoveryDev<'a> {
    path: PathBuf,
    passphrase: &'a str,
}

impl<'a> CLIDisplay for ExportedRecoveryDev<'a> {
    fn plain_write<W: std::io::prelude::Write>(
        &self,
        fmt: &crate::ui::ColorFormatter,
        mut w: W,
    ) -> std::io::Result<()> {
        writeln!(w, "Recovery device saved at {}", self.path.display())?;
        writeln!(
            w,
            "{save_msg} {passwd}",
            save_msg =
                fmt.wrap_in_color(Color::Red, "Save the recovery passphrase in a safe place:"),
            passwd = fmt.wrap_in_color(Color::Green, self.passphrase)
        )
    }
}
