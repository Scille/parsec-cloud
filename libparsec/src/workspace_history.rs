// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

pub use libparsec_client::{
    workspace_history::{
        WorkspaceHistoryEntryStat, WorkspaceHistoryFdCloseError, WorkspaceHistoryFdReadError,
        WorkspaceHistoryFdStatError, WorkspaceHistoryFileStat, WorkspaceHistoryOpenFileError,
        WorkspaceHistorySetTimestampOfInterestError, WorkspaceHistoryStatEntryError,
        WorkspaceHistoryStatFolderChildrenError,
    },
    WorkspaceHistoryOpsStartError as WorkspaceHistoryStartError,
};
use libparsec_types::prelude::*;

use crate::{
    device::DeviceAccessStrategy,
    handle::{
        borrow_from_handle, register_handle_with_init, take_and_close_handle, Handle, HandleItem,
    },
};

fn borrow_workspace_history(
    workspace_history: Handle,
) -> anyhow::Result<Arc<libparsec_client::WorkspaceHistoryOps>> {
    borrow_from_handle(workspace_history, |x| match x {
        HandleItem::WorkspaceHistory {
            workspace_history_ops,
            ..
        } => Some(workspace_history_ops.clone()),
        _ => None,
    })
}

/*
 * Start workspace history
 */

pub async fn client_start_workspace_history(
    client: Handle,
    realm_id: VlobID,
) -> Result<Handle, WorkspaceHistoryStartError> {
    // 1. Register the start of the workspace history

    // Unlike for `WorkspaceOps`, it is totally possible to have multiple
    // `WorkspaceHistoryOps` concurrently for the same realm.
    // For this reason `register_handle_with_init` is used in a special way:
    // - We still need it to correctly handle client shutdown (i.e. a client stop
    //   must wait for the completion of a concurrent workspace history start in order
    //   to close its handle right away).
    // - We consider each workspace history start totally isolated from each other,
    //   hance having an always succeeding precondition.
    let initializing = register_handle_with_init(
        HandleItem::StartingClientWorkspaceHistory {
            client,
            to_wake_on_done: vec![],
        },
        |_handle, _item| Result::<_, ()>::Ok(()),
    )
    .expect("Always succeeds");

    // 2. Actually start the workspace

    let client_handle = client;
    let client = borrow_from_handle(client_handle, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    let workspace_history_ops = client.start_workspace_history(realm_id).await?;

    // 3. Finally register the workspace history ops to get a handle

    let workspace_history_handle = initializing.initialized(move |item: &mut HandleItem| {
        let starting = std::mem::replace(
            item,
            HandleItem::WorkspaceHistory {
                client: Some(client_handle),
                workspace_history_ops,
            },
        );
        match starting {
            HandleItem::StartingClientWorkspaceHistory {
                to_wake_on_done, ..
            } => {
                for event in to_wake_on_done {
                    event.notify(usize::MAX);
                }
            }
            _ => unreachable!(),
        }
    });

    Ok(workspace_history_handle)
}

pub enum WorkspaceHistoryRealmExportDecryptor {
    User {
        access: DeviceAccessStrategy,
    },
    SequesterService {
        sequester_service_id: SequesterServiceID,
        private_key_pem_path: PathBuf,
    },
}

pub async fn workspace_history_start_with_realm_export(
    #[allow(unused_variables)] config: super::ClientConfig,
    #[allow(unused_variables)] export_db_path: PathBuf,
    #[allow(unused_variables)] decryptors: Vec<WorkspaceHistoryRealmExportDecryptor>,
) -> Result<Handle, WorkspaceHistoryStartError> {
    // Realm export database support is not available on web.
    #[cfg(target_arch = "wasm32")]
    {
        Err(WorkspaceHistoryStartError::Internal(anyhow::anyhow!(
            "Realm export database support is not available on web"
        )))
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::handle::register_handle;

        // No need to start by registering the start of the init here !
        // This is because each realm export based workspace history works in isolation,
        // even if they share the same underlying realm export database (since the said
        // database is only accessed in read-only).

        let config: Arc<libparsec_client::ClientConfig> = config.into();

        let mut cooked_decryptors = Vec::with_capacity(decryptors.len());
        for raw_decryptor in decryptors {
            let cooked_decryptor = match raw_decryptor {
                WorkspaceHistoryRealmExportDecryptor::User { access } => {
                    let access = access.convert_with_side_effects()?;

                    let device =
                        libparsec_platform_device_loader::load_device(&config.config_dir, &access)
                            .await
                            .map_err(|e| {
                                WorkspaceHistoryStartError::Internal(
                                    anyhow::Error::new(e).context("Cannot load device"),
                                )
                            })?;

                    libparsec_client::WorkspaceHistoryRealmExportDecryptor::User {
                        user_id: device.user_id,
                        private_key: Box::new(device.private_key.clone()),
                    }
                }

                WorkspaceHistoryRealmExportDecryptor::SequesterService {
                    sequester_service_id,
                    private_key_pem_path,
                } => {
                    let private_key_pem =
                        std::fs::read_to_string(private_key_pem_path).map_err(|e| {
                            WorkspaceHistoryStartError::Internal(
                                anyhow::Error::new(e)
                                    .context("Cannot load sequester private key PEM file"),
                            )
                        })?;

                    let private_key =
                        libparsec_crypto::SequesterPrivateKeyDer::load_pem(&private_key_pem)
                            .map_err(|e| {
                                WorkspaceHistoryStartError::Internal(
                                    anyhow::Error::new(e)
                                        .context("Cannot load sequester private key PEM file"),
                                )
                            })?;

                    libparsec_client::WorkspaceHistoryRealmExportDecryptor::SequesterService {
                        sequester_service_id,
                        private_key: Box::new(private_key),
                    }
                }
            };

            cooked_decryptors.push(cooked_decryptor);
        }

        let workspace_history_ops =
            libparsec_client::workspace_history::WorkspaceHistoryOps::start_with_realm_export(
                config,
                &export_db_path,
                cooked_decryptors,
            )
            .await
            .map(Arc::new)?;

        let handle = register_handle(HandleItem::WorkspaceHistory {
            client: None,
            workspace_history_ops,
        });

        Ok(handle)
    }
}

/*
 * Stop workspace history
 */

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceHistoryInternalOnlyError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub fn workspace_history_stop(
    workspace_history: Handle,
) -> Result<(), WorkspaceHistoryInternalOnlyError> {
    take_and_close_handle(workspace_history, |x| match *x {
        HandleItem::WorkspaceHistory { .. } => {
            // `WorkspaceHistoryOps` has no stop method, dropping it is enough
            Ok(())
        }
        _ => Err(x),
    })
    .map_err(|err| err.into())
}

/*
 * Workspace history FS operations
 */

pub fn workspace_history_get_timestamp_lower_bound(
    workspace_history: Handle,
) -> Result<DateTime, WorkspaceHistoryInternalOnlyError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    Ok(workspace_history.timestamp_lower_bound())
}

pub fn workspace_history_get_timestamp_higher_bound(
    workspace_history: Handle,
) -> Result<DateTime, WorkspaceHistoryInternalOnlyError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    Ok(workspace_history.timestamp_higher_bound())
}

pub fn workspace_history_get_timestamp_of_interest(
    workspace_history: Handle,
) -> Result<DateTime, WorkspaceHistoryInternalOnlyError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    Ok(workspace_history.timestamp_of_interest())
}

pub async fn workspace_history_set_timestamp_of_interest(
    workspace_history: Handle,
    toi: DateTime,
) -> Result<(), WorkspaceHistorySetTimestampOfInterestError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.set_timestamp_of_interest(toi).await
}

pub async fn workspace_history_stat_entry(
    workspace_history: Handle,
    path: &FsPath,
) -> Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.stat_entry(path).await
}

pub async fn workspace_history_stat_entry_by_id(
    workspace_history: Handle,
    entry_id: VlobID,
) -> Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.stat_entry_by_id(entry_id).await
}

pub async fn workspace_history_stat_folder_children(
    workspace_history: Handle,
    path: &FsPath,
) -> Result<Vec<(EntryName, WorkspaceHistoryEntryStat)>, WorkspaceHistoryStatFolderChildrenError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.stat_folder_children(path).await
}

pub async fn workspace_history_stat_folder_children_by_id(
    workspace_history: Handle,
    entry_id: VlobID,
) -> Result<Vec<(EntryName, WorkspaceHistoryEntryStat)>, WorkspaceHistoryStatFolderChildrenError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.stat_folder_children_by_id(entry_id).await
}

pub async fn workspace_history_open_file(
    workspace_history: Handle,
    path: FsPath,
) -> Result<FileDescriptor, WorkspaceHistoryOpenFileError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.open_file(path).await
}

pub async fn workspace_history_open_file_by_id(
    workspace_history: Handle,
    entry_id: VlobID,
) -> Result<FileDescriptor, WorkspaceHistoryOpenFileError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.open_file_by_id(entry_id).await
}

pub async fn workspace_history_open_file_and_get_id(
    workspace_history: Handle,
    path: FsPath,
) -> Result<(FileDescriptor, VlobID), WorkspaceHistoryOpenFileError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.open_file_and_get_id(path).await
}

pub fn workspace_history_fd_close(
    workspace_history: Handle,
    fd: FileDescriptor,
) -> Result<(), WorkspaceHistoryFdCloseError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.fd_close(fd)
}

pub async fn workspace_history_fd_read(
    workspace_history: Handle,
    fd: FileDescriptor,
    offset: u64,
    size: u64,
) -> Result<Vec<u8>, WorkspaceHistoryFdReadError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    let mut buf = Vec::with_capacity(size as usize);
    workspace_history
        .fd_read(fd, offset, size, &mut buf)
        .await?;
    Ok(buf)
}

pub async fn workspace_history_fd_stat(
    workspace_history: Handle,
    fd: FileDescriptor,
) -> Result<WorkspaceHistoryFileStat, WorkspaceHistoryFdStatError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.fd_stat(fd).await
}
