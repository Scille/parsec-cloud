use std::io::Write;

use crate::{ui::compat::TOSDisplay, utils::StartedClient};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
    }
);

crate::build_main_with_client!(main, list_tos);

pub async fn list_tos(ui: crate::Ui, _args: Args, client: &StartedClient) -> anyhow::Result<()> {
    log::trace!("Listing Term of Service");

    match client.get_tos().await {
        Ok(tos) => ui.data_print(&TOSDisplay(tos)).map_err(Into::into),
        Err(libparsec_client::ClientGetTosError::NoTos) => {
            ui.with_message(|_, out| writeln!(out, "No Terms of Service available"))?;
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
