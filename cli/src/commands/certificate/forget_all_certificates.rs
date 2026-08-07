// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use dialoguer::Confirm;
use std::io::Write;

use crate::{ui::compat::LocalDeviceDisplayRef, utils::*};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
    }
);

crate::build_main_with_client!(main, forget_all_certificates);

pub async fn forget_all_certificates(
    ui: crate::Ui,
    _args: Args,
    client: &StartedClient,
) -> anyhow::Result<()> {
    let dev = LocalDeviceDisplayRef(client.device());

    ui.with_message(|_, out| {
        writeln!(
            out,
            "You are about to clear the local certificates database for device:"
        )
    })?;
    ui.message_println(dev)?;

    if !Confirm::new().with_prompt("Are you sure?").interact()? {
        ui.with_message(|_, out| writeln!(out, "Operation cancelled"))?;
    } else {
        client.forget_all_certificates().await?;
        ui.with_message(|_, out| {
            writeln!(out, "The local certificates database has been cleared")
        })?;
    }

    Ok(())
}
