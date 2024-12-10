use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
    }
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        password_stdin,
        device,
        config_dir,
    } = args;

    let client = load_client(&config_dir, device, password_stdin).await?;

    {
        let mut spinner = start_spinner("Poll server for new certificates".into());
        client.poll_server_for_new_certificates().await?;
        spinner.stop_with_symbol(GREEN_CHECKMARK);
    }
    let mut handle = start_spinner("Deleting shared recovery setup".into());

    client.delete_shamir_recovery().await?;

    handle.stop_with_message(format!(
        "{GREEN_CHECKMARK} Shared recovery setup has been deleted"
    ));

    client.stop().await;

    Ok(())
}
