// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
    }
);

crate::build_main_with_client!(main, poll);

pub async fn poll(_args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let mut spinner = start_spinner("Poll server for new certificates".into());
    let new_certificates = client.poll_server_for_new_certificates().await?;
    spinner.stop_with_message(format!(
        "Added {new_certificates} new certificates {GREEN_CHECKMARK}"
    ));
    Ok(())
}
