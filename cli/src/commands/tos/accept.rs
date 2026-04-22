use dialoguer::Confirm;

use crate::{commands::tos::list::display_tos, utils::StartedClient};

crate::clap_parser_with_shared_opts_builder!(
    #[with = device, config_dir, password_stdin]
    pub struct Args {
        #[arg(long)]
        yes: bool,
    }
);

crate::build_main_with_client!(main, accept_tos);

pub async fn accept_tos(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args { yes, .. } = args;

    let tos = match client.get_tos().await {
        Ok(tos) => tos,
        Err(libparsec_client::ClientGetTosError::NoTos) => {
            println!("No Terms of Service available");
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    };

    if !yes {
        display_tos(&tos)?;
        if !Confirm::new()
            .with_prompt("Do you accept these terms of service?")
            .interact()?
        {
            return Err(anyhow::anyhow!("Operation cancelled"));
        }
    }
    client.accept_tos(tos.updated_on).await?;
    Ok(())
}
