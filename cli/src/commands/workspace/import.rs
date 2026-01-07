use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
    vec,
};

use libparsec::{EntryName, EntryNameError, FsPath, OpenOptions, VlobID, WorkspaceOpenFileError};
use libparsec_client::{EventBus, WorkspaceOps};
use tokio::{io::AsyncReadExt, task::JoinSet};

use crate::utils::StartedClient;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin, workspace]
    pub struct Args {
        /// Local file or folder to import (e.g. "myfile.txt")
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
        /// Control how existing files are updated.
        ///
        /// (similar to `cp --update=...`)
        #[clap(long, value_enum, default_value_t)]
        update: UpdateMode,
    }
);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum UpdateMode {
    /// Existing files in destination are replaced
    All,
    /// Existing files in destination are not replaced but raise and error instead.
    #[default]
    NoneFail,
}

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
        update,
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

    if let Err(err) = import_item(workspace, src, dest, update).await {
        match err {
            ImportItemError::Folder(ImportFolderError::ImportErrors(mut errors)) => {
                eprintln!("Errors during folder import:");
                let mut error_count = errors.len();
                while let Some(error) = errors.pop() {
                    match error {
                        ImportItemError::Folder(ImportFolderError::ImportErrors(errs)) => {
                            error_count += errs.len();
                            errors.extend(errs);
                            continue;
                        }
                        err => eprintln!("{err}"),
                    }
                }
                anyhow::bail!("{error_count} errors during folder import")
            }
            _ => return Err(err.into()),
        }
    };

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

#[derive(Debug, thiserror::Error)]
enum ImportItemError {
    #[error("{}: {source}", path.display())]
    File {
        path: PathBuf,
        source: ImportFileError,
    },
    #[error(transparent)]
    Folder(#[from] ImportFolderError),
    #[error("{}: unsupported file type ({file_type:?})", path.display())]
    UnsupportedFileType {
        path: PathBuf,
        file_type: std::fs::FileType,
    },
    #[error("{}: cannot stat file: {source}", path.display())]
    CannotStat {
        path: PathBuf,
        source: std::io::Error,
    },
}

async fn import_item(
    workspace: Arc<WorkspaceOps>,
    src: PathBuf,
    dest: FsPath,
    update: UpdateMode,
) -> Result<(), ImportItemError> {
    let src_metadata =
        tokio::fs::metadata(&src)
            .await
            .map_err(|source| ImportItemError::CannotStat {
                path: src.clone(),
                source,
            })?;
    let src_file_type = src_metadata.file_type();
    if src_file_type.is_file() {
        import_file(workspace, &src, dest, update)
            .await
            .map_err(|source| ImportItemError::File {
                path: src.clone(),
                source,
            })
    } else if src_file_type.is_dir() {
        import_folder(workspace, src, dest, update)
            .await
            .map_err(Into::into)
    } else {
        Err(ImportItemError::UnsupportedFileType {
            path: src.clone(),
            file_type: src_file_type,
        })
    }
}

#[derive(Debug, thiserror::Error)]
enum ImportFileError {
    #[error("already exists")]
    AlreadyExists,
    #[error("open error: {0}")]
    OpenError(#[from] WorkspaceOpenFileError),
    #[error("copy error: {0}")]
    CopyError(#[from] CopyFileError),
    #[error("close error: {0}")]
    CloseError(anyhow::Error),
}

async fn import_file(
    workspace: Arc<WorkspaceOps>,
    src: &Path,
    dest: FsPath,
    update: UpdateMode,
) -> Result<(), ImportFileError> {
    // Open the remote file (create it if it does not exists)
    let fd = workspace
        .open_file(
            dest.clone(),
            OpenOptions {
                read: false,
                write: true,
                truncate: true,
                create: true,
                // When update mode is set to none, we can use the `create_new` option to ensure
                // that the opened file did not exist beforehand
                // (so not updating something that exist)
                create_new: update == UpdateMode::NoneFail,
            },
        )
        .await
        .map_err(|e| match e {
            WorkspaceOpenFileError::EntryExistsInCreateNewMode { .. }
                if update == UpdateMode::NoneFail =>
            {
                ImportFileError::AlreadyExists
            }
            _ => ImportFileError::OpenError(e),
        })?;

    // Copy content from local file to remote file description
    copy_file_to_fd(src, &workspace, fd).await?;
    log::debug!("Flushing and closing file");
    workspace
        .fd_flush(fd)
        .await
        .map_err(anyhow::Error::from)
        .map_err(ImportFileError::CloseError)?;
    workspace
        .fd_close(fd)
        .await
        .map_err(anyhow::Error::from)
        .map_err(ImportFileError::CloseError)?;

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

#[derive(Debug, thiserror::Error)]
enum CopyFileError {
    #[error("cannot open source file: {0}")]
    CannotOpenSrcFile(std::io::Error),
    #[error("cannot read source file: {0}")]
    ReadError(std::io::Error),
    #[error("cannot write: {0}")]
    WriteError(libparsec::WorkspaceFdWriteError),
}

async fn copy_file_to_fd(
    src: &Path,
    workspace: &Arc<libparsec_client::WorkspaceOps>,
    fd: libparsec::FileDescriptor,
) -> Result<(), CopyFileError> {
    let file = tokio::fs::File::open(src)
        .await
        .map_err(CopyFileError::CannotOpenSrcFile)?;
    let mut buf_file = tokio::io::BufReader::new(file);
    let mut buffer = vec![0_u8; 4096];
    let mut dst_offset = 0_usize;
    log::debug!("Copying file to workspace");
    loop {
        let bytes_read = buf_file
            .read(&mut buffer)
            .await
            .map_err(CopyFileError::ReadError)?;
        let mut buf = &buffer[..bytes_read];

        if bytes_read == 0 {
            break;
        }

        while !buf.is_empty() {
            let bytes_written = workspace
                .fd_write(fd, dst_offset as u64, buf)
                .await
                .map_err(CopyFileError::WriteError)? as usize;
            buf = &buf[bytes_written..];
            dst_offset += bytes_written;
        }
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum ImportFolderError {
    #[error("{}: cannot create folder: {source}", path.display())]
    CreateFolder {
        path: PathBuf,
        source: libparsec::WorkspaceCreateFolderError,
    },
    #[error("{}: cannot read directory: {source}", path.display())]
    ReadDirectory {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("{}: invalid filename: {source}", path.display())]
    InvalidEntryName {
        path: PathBuf,
        source: libparsec::EntryNameError,
    },
    #[error("{} import errors", .0.len())]
    ImportErrors(Vec<ImportItemError>),
}

fn import_folder(
    workspace: Arc<WorkspaceOps>,
    src: PathBuf,
    dest: FsPath,
    update: UpdateMode,
) -> impl std::future::Future<Output = Result<(), ImportFolderError>> + Send + 'static {
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
            })
            .map_err(|source| ImportFolderError::CreateFolder {
                path: src.clone(),
                source,
            })?;
        let mut import_tasks = JoinSet::new();
        let mut readdir =
            tokio::fs::read_dir(&src)
                .await
                .map_err(|source| ImportFolderError::ReadDirectory {
                    path: src.clone(),
                    source,
                })?;
        loop {
            let Some(entry) =
                readdir
                    .next_entry()
                    .await
                    .map_err(|source| ImportFolderError::ReadDirectory {
                        path: src.clone(),
                        source,
                    })?
            else {
                log::trace!("Finished exploring child of {}", src.display());
                break;
            };
            let entry_path = entry.path();
            let entry_name = match filename_to_entryname(&entry_path) {
                Some(Ok(v)) => v,
                Some(Err(e)) => {
                    import_tasks.spawn(async {
                        Err(ImportItemError::Folder(
                            ImportFolderError::InvalidEntryName {
                                path: entry_path,
                                source: e,
                            },
                        ))
                    });
                    continue;
                }
                None => continue,
            };
            let entry_dest = dest.join(entry_name);
            let dup_workspace = workspace.clone();
            import_tasks.spawn(async move {
                import_item(dup_workspace, entry_path, entry_dest, update).await
            });
        }
        let errs = import_tasks
            .join_all()
            .await
            .into_iter()
            .filter_map(Result::err)
            .collect::<Vec<_>>();
        if errs.is_empty() {
            Ok(())
        } else {
            Err(ImportFolderError::ImportErrors(errs))
        }
    }
}

fn filename_to_entryname(path: &Path) -> Option<Result<EntryName, EntryNameError>> {
    path.file_name().map(TryFrom::try_from)
}
