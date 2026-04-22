// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use dialoguer::Confirm;

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
    }
);

crate::build_main_with_client!(main, forget_all_certificates);

pub async fn forget_all_certificates(_args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let short_id = &client.device_id().hex()[..3];
    let organization_id = client.organization_id();
    let human_handle = client.human_handle();
    let device_label = client.device_label();

    println!("You are about to clear the local certificates database for device:");
    println!("{YELLOW}{short_id}{RESET} - {organization_id}: {human_handle} @ {device_label}");

    if !Confirm::new().with_prompt("Are you sure?").interact()? {
        println!("Operation cancelled");
    } else {
        client.forget_all_certificates().await?;
        println!("The local certificates database has been cleared");
    }

    Ok(())
}
