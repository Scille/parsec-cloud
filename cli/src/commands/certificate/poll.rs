// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{ui::Color, utils::*};
use std::fmt::Write;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
    }
);

crate::build_main_with_client!(main, poll);

pub async fn poll(ui: crate::Ui, _args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let spinner = ui.with_spinner(|_, out| write!(out, "Poll server for new certificates"))?;
    let new_certificates = client.poll_server_for_new_certificates().await?;
    spinner.stop_with(|fmt, out| {
        write!(
            out,
            "Added {new_certificates} new certificates {checkmark}",
            checkmark = fmt.wrap_in_color(Color::Green, "✔")
        )
    })?;
    Ok(())
}
