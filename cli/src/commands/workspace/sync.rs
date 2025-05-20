use crate::utils::{StartedClient, start_spinner};

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

    let (name, role) = workspace.get_current_name_and_self_role();
    let mut handle = start_spinner(format!("Syncing workspace {}", name));

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
        log::debug!("Entries to inbound sync: {:?}", entries_to_sync);
        for entry in entries_to_sync {
            workspace.inbound_sync(entry).await?;
        }
    }
    if role.can_write() {
        loop {
            let entries_to_sync = workspace
                .get_need_outbound_sync(OUTBOUND_SYNC_BATCH_SIZE)
                .await?;
            if entries_to_sync.is_empty() {
                log::debug!("No more entries to outbound sync");
                break;
            }
            log::debug!("Entries to outbound sync: {:?}", entries_to_sync);
            for entry in entries_to_sync {
                workspace.outbound_sync(entry).await?;
            }
        }
    }

    handle.stop_with_message(format!("Workspace {} has been synced", name));

    drop(workspace);
    client.stop_workspace(wid).await;

    Ok(())
}
