use crate::{commands::tos::list::display_tos, utils::load_client};

crate::clap_parser_with_shared_opts_builder!(
    #[with = device, config_dir, password_stdin]
    pub struct Args {
        #[arg(long)]
        yes: bool,
    }
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        config_dir,
        password_stdin,
        device,
        yes,
    } = args;

    let client = load_client(&config_dir, device, password_stdin).await?;
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

        println!("Do you accept these terms of service? (y/N) ");
        let mut input = String::with_capacity(2);
        std::io::stdin().read_line(&mut input)?;
        if input.trim() != "y" {
            return Err(anyhow::anyhow!("Operation cancelled"));
        }
    }
    client.accept_tos(tos.updated_on).await?;
    Ok(())
}
