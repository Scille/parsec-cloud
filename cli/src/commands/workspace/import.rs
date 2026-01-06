use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
    vec,
};

use libparsec::{anyhow::Context, EntryName, EntryNameError, FsPath, OpenOptions, VlobID};
use libparsec_client::{EventBus, WorkspaceOps};
use tokio::{io::AsyncReadExt, task::JoinSet};

use crate::utils::StartedClient;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin, workspace]
    pub struct Args {
        /// Local file to import (e.g. "myfile.txt")
        pub(crate) src: PathBuf,
        /// Destination path in the workspace (e.g. "/path/to/myfile.txt")
        ///
        /// The path is absolute from the workspace root directory. The command will fail if the
        /// parent directories do not exist, unless the `parents` option is enabled.
        ///
        /// If the destination file already exists, its content will be replaced.
        pub(crate) dest: FsPath,
        /// If specified, create parent directories as needed
        ///
        /// No error if parent directories already exist (similar to `mkdir -p`)
        #[clap(long, short, action)]
        pub(crate) parents: bool,
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
        parents,
        workspace: wid,
        ..
    } = args;

    log::trace!(
        "workspace_import: {src} -> {wid}:{dst}",
        src = src.display(),
        dst = dest
    );

    // Start workspace and watch for files need to be synced
    let workspace = client.start_workspace(wid).await?;
    let (files_to_sync, _watch_files_to_sync) = watch_workspace_sync_events(&client.event_bus, wid);

    let (notify, _event_conn) =
        notify_sync_completion(&client.event_bus, wid, files_to_sync.clone());

    // Create parent directories if the `--parents` flag was specified
    if parents {
        log::debug!("Creating parent directories");
        workspace
            .create_folder_all(dest.clone().parent())
            .await
            .or_else(|e| match e {
                // `create_folder_all` returns an error if the full path already exists
                // but in our case this is not a problem so we explicitly ignore it
                libparsec::WorkspaceCreateFolderError::EntryExists { entry_id } => Ok(entry_id),
                // All other errors are propagated
                _ => Err(e),
            })?;
    }
    let src_metadata = src.metadata()?;
    let src_file_type = src_metadata.file_type();
    if src_file_type.is_file() {
        import_file(workspace, &src, dest).await?;
    } else if src_file_type.is_dir() {
        import_folder(workspace, src, dest).await?;
    } else {
        anyhow::bail!("Unsupported file type {src_file_type:?}")
    }
    // After importing the file/folder, we may need to wait for the workspace to sync its data with the server.
    // Note that, in addition to the files being imported, this operation may involve syncing other files.
    // So instead of being peaky about which file to sync or wait, we just wait for all files to be synced for this workspace.
    if !files_to_sync.lock().expect("Mutex poisoned").is_empty() {
        log::debug!("Waiting for sync");
        notify.notified().await;
        log::trace!("Sync done");
    }

    Ok(())
}

async fn import_file(workspace: Arc<WorkspaceOps>, src: &Path, dest: FsPath) -> anyhow::Result<()> {
    // Open the remote file (create it if it does not exists)
    let fd = workspace
        .open_file(
            dest.clone(),
            OpenOptions {
                read: false,
                write: true,
                truncate: true,
                create: true,
                create_new: false,
            },
        )
        .await
        .with_context(|| {
            format!(
                "Cannot open file {dest} in workspace {}",
                workspace.realm_id()
            )
        })?;

    // Copy content from local file to remote file description
    copy_file_to_fd(src, &workspace, fd).await?;
    log::debug!("Flushing and closing file");
    workspace.fd_flush(fd).await?;
    workspace.fd_close(fd).await?;

    Ok(())
}

/// Watch for files that need to be synced with the server
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
        let libparsec_client::EventWorkspaceOpsOutboundSyncDone {
            ref realm_id,
            entry_id,
        } = event;
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
    src: &Path,
    workspace: &Arc<libparsec_client::WorkspaceOps>,
    fd: libparsec::FileDescriptor,
) -> Result<(), anyhow::Error> {
    let file = tokio::fs::File::open(src)
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

fn import_folder(
    workspace: Arc<WorkspaceOps>,
    src: PathBuf,
    dest: FsPath,
) -> impl std::future::Future<Output = anyhow::Result<()>> + Send + 'static {
    log::debug!(
        "Importing {} to {}:{dest}",
        src.display(),
        workspace.realm_id()
    );
    async move {
        workspace
            .create_folder(dest.clone())
            .await
            .or_else(|e| match e {
                // `create_folder` returns an error if the full path already exists
                // but in our case this is not a problem so we explicitly ignore it
                libparsec::WorkspaceCreateFolderError::EntryExists { entry_id } => Ok(entry_id),
                // All other errors are propagated
                _ => Err(e),
            })?;
        let mut import_tasks = JoinSet::new();
        let mut readdir = tokio::fs::read_dir(&src).await?;
        loop {
            let Some(entry) = readdir.next_entry().await? else {
                log::trace!("Finished exploring child of {}", src.display());
                break;
            };
            let entry_ft = entry.file_type().await?;
            let entry_path = entry.path();
            let entry_name = match filename_to_entryname(&entry_path) {
                Some(Ok(v)) => v,
                Some(Err(e)) => {
                    log::warn!(
                        "Skipping {} because it has an invalid filename ({e})",
                        entry_path.display()
                    );
                    continue;
                }
                None => continue,
            };
            let entry_dest = dest.join(entry_name);
            let dup_workspace = workspace.clone();
            if entry_ft.is_file() {
                import_tasks.spawn(async move {
                    import_file(dup_workspace, &entry_path, entry_dest).await
                });
            } else if entry_ft.is_dir() {
                import_tasks.spawn(async move {
                    import_folder(dup_workspace, entry_path, entry_dest).await
                });
            } else {
                log::warn!(
                    "Skipping {} because it has an unsupported file type ({entry_ft:?})",
                    entry_path.display()
                )
            }
        }
        import_tasks
            .join_all()
            .await
            .into_iter()
            .find(Result::is_err)
            .transpose()
            .and(Ok(()))
    }
}

fn filename_to_entryname(path: &Path) -> Option<Result<EntryName, EntryNameError>> {
    path.file_name().map(TryFrom::try_from)
}
