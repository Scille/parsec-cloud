use std::{collections::HashSet, path::PathBuf, sync::Arc, vec};

use libparsec::{FsPath, OpenOptions, VlobID, anyhow::Context};
use libparsec_client::EventBus;
use tokio::io::AsyncReadExt;

use crate::utils::StartedClient;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin, workspace]
    pub struct Args {
        /// Local file to copy
        pub(crate) src: PathBuf,
        /// Workspace destination path
        pub(crate) dest: FsPath,
    }
);

crate::build_main_with_client!(
    main,
    workspace_import,
    libparsec::ClientConfig {
        with_monitors: true,
        ..Default::default()
    }
    .into()
);

pub async fn workspace_import(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args {
        src,
        dest,
        workspace: wid,
        ..
    } = args;

    log::trace!(
        "workspace_import: {src} -> {wid}:{dst}",
        src = src.display(),
        dst = dest
    );

    let workspace = client.start_workspace(wid).await?;
    let (files_to_sync, _watch_files_to_sync) = watch_workspace_sync_events(&client.event_bus, wid);
    let fd = workspace
        .open_file(
            dest,
            OpenOptions {
                read: false,
                write: true,
                truncate: true,
                create: true,
                create_new: false,
            },
        )
        .await?;

    let (notify, _event_conn) =
        notify_sync_completion(&client.event_bus, wid, files_to_sync.clone());

    copy_file_to_fd(src, &workspace, fd).await?;
    log::debug!("Flushing and closing file");
    workspace.fd_flush(fd).await?;
    workspace.fd_close(fd).await?;

    // After importing the file, we may need to wait for the workspace to sync its data with the server.
    // The sync procedure may sync other files that the one related to the operation (think parent & children).
    // So instead of being peaky about which file to sync or to wait, we just wait for all files to be synced for the workspace.
    if !files_to_sync.lock().expect("Mutex poisoned").is_empty() {
        log::debug!("Waiting for sync");
        notify.notified().await;
        log::trace!("Sync done");
    }
    Ok(())
}

/// Watch for file that need to be synced with the server
#[must_use]
fn watch_workspace_sync_events(
    event_bus: &EventBus,
    wid: VlobID,
) -> (
    Arc<std::sync::Mutex<HashSet<VlobID>>>,
    libparsec_client::EventBusConnectionLifetime<
        libparsec_client::EventWorkspaceOpsOutboundSyncNeeded,
    >,
) {
    let files_needing_sync = Arc::new(std::sync::Mutex::new(HashSet::new()));
    let dup = files_needing_sync.clone();
    let event_conn = event_bus.connect(move |event| {
        let libparsec_client::EventWorkspaceOpsOutboundSyncNeeded { realm_id, entry_id } = event;
        if realm_id == &wid {
            files_needing_sync
                .lock()
                .expect("Mutex poisoned")
                .insert(event.entry_id);
            log::debug!("Outbound sync needed for file ({entry_id})");
        } else {
            log::trace!("Ignore outbound sync event for another realm ({realm_id})");
        }
    });
    (dup, event_conn)
}

#[must_use]
fn notify_sync_completion(
    event_bus: &EventBus,
    wid: VlobID,
    files_to_sync: Arc<std::sync::Mutex<HashSet<VlobID>>>,
) -> (
    Arc<tokio::sync::Notify>,
    libparsec_client::EventBusConnectionLifetime<
        libparsec_client::EventWorkspaceOpsOutboundSyncDone,
    >,
) {
    let notify = Arc::new(tokio::sync::Notify::new());
    let notify2 = notify.clone();

    let event_conn = event_bus.connect(move |event| {
        let libparsec_client::EventWorkspaceOpsOutboundSyncDone { realm_id, entry_id } = event;
        let mut files_to_sync = files_to_sync.lock().expect("Mutex poisoned");
        if realm_id == &wid && files_to_sync.remove(entry_id) {
            log::debug!("Outbound sync done for file ({realm_id}:{entry_id})");
            if files_to_sync.is_empty() {
                log::trace!("All outbound sync done for realm {wid}");
                notify2.notify_one();
            }
        } else {
            log::trace!("Outbound sync done for another file ({realm_id}:{entry_id})");
        }
    });
    (notify, event_conn)
}

async fn copy_file_to_fd(
    src: PathBuf,
    workspace: &Arc<libparsec_client::WorkspaceOps>,
    fd: libparsec::FileDescriptor,
) -> Result<(), anyhow::Error> {
    let file = tokio::fs::File::open(&src)
        .await
        .context("Cannot open local file")?;
    let mut buf_file = tokio::io::BufReader::new(file);
    let mut buffer = vec![0_u8; 4096];
    let mut dst_offset = 0_usize;
    log::debug!("Copying file to workspace");
    loop {
        let bytes_read = buf_file
            .read(&mut buffer)
            .await
            .context("Cannot read local file")?;

        if bytes_read == 0 {
            break;
        }

        let mut bytes_written = 0_usize;
        while bytes_written < bytes_read {
            bytes_written = workspace
                .fd_write(fd, dst_offset as u64, &buffer[bytes_written..bytes_read])
                .await
                .context("Cannot write to workspace")? as usize;
            dst_offset += bytes_written;
        }
    }
    Ok(())
}
