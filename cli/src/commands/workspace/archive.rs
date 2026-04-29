// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::RequestedRealmArchivingConfiguration;

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, workspace, password_stdin]
    #[command(group(clap::ArgGroup::new("mode").required(true)))]
    pub struct Args {
        /// Archive the workspace in read-only mode
        #[arg(long, group = "mode")]
        archived: bool,
        /// Plan workspace deletion, i.e. the workspace will be available for
        /// deletion once the archiving period (provided in seconds) is elapsed
        #[arg(long, group = "mode")]
        deletion_planned_in_seconds: Option<u32>,
        /// Make the workspace available again
        #[arg(long, group = "mode")]
        available: bool,
    }
);

crate::build_main_with_client!(main, archive_workspace);

pub async fn archive_workspace(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args {
        workspace: wid,
        archived,
        deletion_planned_in_seconds,
        available,
        ..
    } = args;

    let configuration = if archived {
        RequestedRealmArchivingConfiguration::Archived
    } else if let Some(archiving_period_in_seconds) = deletion_planned_in_seconds {
        RequestedRealmArchivingConfiguration::DeletionPlanned {
            archiving_period_in_seconds,
        }
    } else if available {
        RequestedRealmArchivingConfiguration::Available
    } else {
        unreachable!("clap argument group should enforce exactly one mode");
    };

    log::trace!("Archiving workspace {wid} with configuration {configuration:?}");

    let mut handle = start_spinner("Updating workspace archiving status".into());

    client.archive_workspace(wid, configuration).await?;

    handle.stop_with_message("Workspace archiving status has been updated".into());

    client.stop().await;

    Ok(())
}
