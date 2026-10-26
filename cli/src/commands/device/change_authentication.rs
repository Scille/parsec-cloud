// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use std::io::Write;

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, auth]
    pub struct Args {
    }
);

pub async fn main(ui: crate::Ui, args: Args) -> anyhow::Result<()> {
    let current_access_strategy =
        crate::utils::get_access_strategy_for_device(&args.config_dir, args.device, None).await?;

    let new_save_strategy = args
        .auth
        .get_save_strategy()?
        .convert_with_side_effects()
        .await?;

    libparsec_platform_device_loader::update_device_change_authentication(
        &args.config_dir,
        &current_access_strategy,
        &new_save_strategy,
        &current_access_strategy.key_file,
    )
    .await?;

    ui.with_message(|_, out| writeln!(out, "Device authentication changed successfully"))?;

    Ok(())
}
