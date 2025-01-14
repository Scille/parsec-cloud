use crate::{build_main_with_client, utils::*};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
    }
);

build_main_with_client!(main, delete_shared_recovery);

pub async fn delete_shared_recovery(_args: Args, client: &StartedClient) -> anyhow::Result<()> {
    poll_server_for_new_certificates(client).await?;

    let mut handle = start_spinner("Deleting shared recovery setup".into());

    client.delete_shamir_recovery().await?;

    handle.stop_with_message(format!(
        "{GREEN_CHECKMARK} Shared recovery setup has been deleted"
    ));

    Ok(())
}
