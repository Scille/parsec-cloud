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
        let _spinner = start_spinner("Poll server for new certificates".into());
        client.poll_server_for_new_certificates().await?;
    }

    client.delete_shamir_recovery().await?;

    println!("Deleted shared recovery for {}", client.user_id());

    client.stop().await;

    Ok(())
}
