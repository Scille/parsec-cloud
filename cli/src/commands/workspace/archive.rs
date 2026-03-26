// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{DateTime, RealmArchivingConfiguration};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, workspace, password_stdin]
    #[command(group(clap::ArgGroup::new("mode").required(true)))]
    pub struct Args {
        /// Archive the workspace in read-only mode
        #[arg(long, group = "mode")]
        archived: bool,
        /// Plan workspace deletion at the given date (RFC 3339, e.g. 2026-05-18T00:00:00Z)
        #[arg(long, group = "mode")]
        deletion_planned: Option<DateTime>,
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
        deletion_planned,
        available,
        ..
    } = args;

    let configuration = if archived {
        RealmArchivingConfiguration::Archived
    } else if let Some(deletion_date) = deletion_planned {
        RealmArchivingConfiguration::DeletionPlanned { deletion_date }
    } else if available {
        RealmArchivingConfiguration::Available
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
