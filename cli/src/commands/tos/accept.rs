use std::io::Write;

use dialoguer::Confirm;

use crate::{ui::compat::TOSDisplay, utils::StartedClient};

crate::clap_parser_with_shared_opts_builder!(
    #[with = device, config_dir, password_stdin, force]
    pub struct Args {

    }
);

crate::build_main_with_client!(main, accept_tos);

pub async fn accept_tos(ui: crate::Ui, args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args { force, .. } = args;

    let tos = match client.get_tos().await {
        Ok(tos) => TOSDisplay(tos),
        Err(libparsec_client::ClientGetTosError::NoTos) => {
            ui.with_message(|_, out| writeln!(out, "No Terms of Service available"))?;
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    };

    if !force {
        ui.message_println(&tos)?;
        if !Confirm::new()
            .with_prompt("Do you accept these terms of service?")
            .interact()?
        {
            return Err(anyhow::anyhow!("Operation cancelled"));
        }
    }
    client.accept_tos(tos.0.updated_on).await?;
    Ok(())
}
