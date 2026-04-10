use crate::utils::{start_spinner, StartedClient};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin, workspace]
    pub struct Args {}
);

const INBOUND_SYNC_BATCH_SIZE: u32 = 32;
const OUTBOUND_SYNC_BATCH_SIZE: u32 = 32;

crate::build_main_with_client!(main, workspace_sync);

pub async fn workspace_sync(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args { workspace: wid, .. } = args;

    log::trace!("workspace_sync: {wid}");

    let workspace = client.start_workspace(wid).await?;

    let (name, is_read_only) = workspace
        .get_workspace_external_info(|info| (info.entry.name.clone(), info.entry.is_read_only()));
    let mut handle = start_spinner(format!("Syncing workspace {name}"));

    log::debug!("Refreshing realm checkpoint");
    workspace.refresh_realm_checkpoint().await?;

    loop {
        let entries_to_sync = workspace
            .get_need_inbound_sync(INBOUND_SYNC_BATCH_SIZE)
            .await?;
        if entries_to_sync.is_empty() {
            log::debug!("No more entries to inbound sync");
            break;
        }
        log::debug!("Entries to inbound sync: {entries_to_sync:?}");
        for entry in entries_to_sync {
            workspace.inbound_sync(entry).await?;
        }
    }
    if !is_read_only {
        loop {
            let entries_to_sync = workspace
                .get_need_outbound_sync(OUTBOUND_SYNC_BATCH_SIZE)
                .await?;
            if entries_to_sync.is_empty() {
                log::debug!("No more entries to outbound sync");
                break;
            }
            log::debug!("Entries to outbound sync: {entries_to_sync:?}");
            for entry in entries_to_sync {
                workspace.outbound_sync(entry).await?;
            }
        }
    }

    handle.stop_with_message(format!("Workspace {name} has been synced"));

    drop(workspace);
    client.stop_workspace(wid).await;

    Ok(())
}
