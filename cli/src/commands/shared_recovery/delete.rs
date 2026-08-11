use crate::{
    build_main_with_client,
    ui::Color,
    utils::{poll_server_for_new_certificates, StartedClient, CHECKMARK},
};
use std::fmt::Write;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
    }
);

build_main_with_client!(main, delete_shared_recovery);

pub async fn delete_shared_recovery(
    ui: crate::Ui,
    _args: Args,
    client: &StartedClient,
) -> anyhow::Result<()> {
    poll_server_for_new_certificates(&ui, client).await?;

    let handle = ui.with_spinner(|_, out| write!(out, "Deleting shared recovery setup"))?;

    client.delete_shamir_recovery().await?;

    handle.stop_with(|fmt, out| {
        write!(
            out,
            "{} Shared recovery setup has been deleted",
            fmt.wrap_in_color(Color::Green, CHECKMARK)
        )
    })?;

    Ok(())
}
