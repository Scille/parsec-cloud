// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

use libparsec_platform_async::event::Event;
use libparsec_types::prelude::*;

/// Used to identify [HandleItem]
pub type Handle = u32;
const INVALID_HANDLE_ERROR_MSG: &str = "Invalid Handle";

enum RegisteredHandleItem {
    Open(Box<HandleItem>),
    Closed,
}

pub(crate) enum HandleItem {
    StartingClient {
        device_id: DeviceID,
        // Concurrent start for the same client will register itself here
        to_wake_on_done: Vec<Event>,
    },
    Client {
        client: Arc<libparsec_client::Client>,
        on_event: crate::OnEventCallbackPlugged,
        #[cfg(not(target_arch = "wasm32"))]
        #[allow(unused)]
        device_in_use_guard: libparsec_platform_ipc::InUseDeviceLockGuard,
    },

    StartingWorkspace {
        client: Handle,
        realm_id: VlobID,
        // Concurrent start for the same workspace will register itself here
        to_wake_on_done: Vec<Event>,
    },
    Workspace {
        client: Handle,
        workspace_ops: Arc<libparsec_client::WorkspaceOps>,
    },
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    StartingMountpoint {
        client: Handle,
        workspace: Handle,
        // Concurrent start for the same mountpoint will register itself here
        to_wake_on_done: Vec<Event>,
    },
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    Mountpoint {
        client: Handle,
        workspace: Handle,
        #[cfg(not(target_arch = "wasm32"))]
        mountpoint: libparsec_platform_mountpoint::Mountpoint,
    },
    /// Workspace history is a read-only stuff, so it has no trouble being started
    /// multiple times concurrently.
    ///
    /// However starting a workspace history impacts its related client at shutdown
    /// time (i.e. a client stop must wait for the completion of a concurrent workspace
    /// history start in order to close its handle right away).
    ///
    /// Also note this is only needed for workspace history relying on a client (i.e.
    /// the server-based ones), not the ones relying instead on a realm export database.
    StartingClientWorkspaceHistory {
        client: Handle,
        to_wake_on_done: Vec<Event>,
    },
    WorkspaceHistory {
        /// `None` if the workspace history uses a realm export database
        client: Option<Handle>,
        workspace_history_ops: Arc<libparsec_client::WorkspaceHistoryOps>,
    },

    Account(Arc<libparsec_account::Account>),

    UserGreetInitial(libparsec_client::UserGreetInitialCtx),
    DeviceGreetInitial(libparsec_client::DeviceGreetInitialCtx),
    ShamirRecoveryGreetInitial(libparsec_client::ShamirRecoveryGreetInitialCtx),

    UserGreetInProgress1(libparsec_client::UserGreetInProgress1Ctx),
    DeviceGreetInProgress1(libparsec_client::DeviceGreetInProgress1Ctx),
    ShamirRecoveryGreetInProgress1(libparsec_client::ShamirRecoveryGreetInProgress1Ctx),

    UserGreetInProgress2(libparsec_client::UserGreetInProgress2Ctx),
    DeviceGreetInProgress2(libparsec_client::DeviceGreetInProgress2Ctx),
    ShamirRecoveryGreetInProgress2(libparsec_client::ShamirRecoveryGreetInProgress2Ctx),

    UserGreetInProgress3(libparsec_client::UserGreetInProgress3Ctx),
    DeviceGreetInProgress3(libparsec_client::DeviceGreetInProgress3Ctx),
    ShamirRecoveryGreetInProgress3(libparsec_client::ShamirRecoveryGreetInProgress3Ctx),

    UserGreetInProgress4(libparsec_client::UserGreetInProgress4Ctx),
    DeviceGreetInProgress4(libparsec_client::DeviceGreetInProgress4Ctx),

    ShamirRecoveryClaimPickRecipient(libparsec_client::ShamirRecoveryClaimPickRecipientCtx),
    ShamirRecoveryClaimShare(libparsec_client::ShamirRecoveryClaimShare),
    ShamirRecoveryClaimRecoverDevice(libparsec_client::ShamirRecoveryClaimRecoverDeviceCtx),

    UserClaimListAdministrators(libparsec_client::UserClaimListAdministratorsCtx),

    UserClaimInitial(libparsec_client::UserClaimInitialCtx),
    DeviceClaimInitial(libparsec_client::DeviceClaimInitialCtx),
    ShamirRecoveryClaimInitial(libparsec_client::ShamirRecoveryClaimInitialCtx),

    UserClaimInProgress1(libparsec_client::UserClaimInProgress1Ctx),
    DeviceClaimInProgress1(libparsec_client::DeviceClaimInProgress1Ctx),
    ShamirRecoveryClaimInProgress1(libparsec_client::ShamirRecoveryClaimInProgress1Ctx),

    UserClaimInProgress2(libparsec_client::UserClaimInProgress2Ctx),
    DeviceClaimInProgress2(libparsec_client::DeviceClaimInProgress2Ctx),
    ShamirRecoveryClaimInProgress2(libparsec_client::ShamirRecoveryClaimInProgress2Ctx),

    UserClaimInProgress3(libparsec_client::UserClaimInProgress3Ctx),
    DeviceClaimInProgress3(libparsec_client::DeviceClaimInProgress3Ctx),
    ShamirRecoveryClaimInProgress3(libparsec_client::ShamirRecoveryClaimInProgress3Ctx),

    UserClaimFinalize(libparsec_client::UserClaimFinalizeCtx),
    DeviceClaimFinalize(libparsec_client::DeviceClaimFinalizeCtx),
    ShamirRecoveryClaimFinalize(libparsec_client::ShamirRecoveryClaimFinalizeCtx),
}

/// Store the resources in a vector with index as handle, as it is simple and fast.
/// Note we never re-use a handle once it corresponding resource is closed (given
/// it prevents hard to track bugs). However the downside is the vector is always
/// growing, but this is not a big deal given the number of resources opened
/// in the application lifetime is limited (e.g. the most demanding is greeter/claimer
/// invitation which uses around 20 resources to complete).
static HANDLES: OnceLock<Mutex<Vec<RegisteredHandleItem>>> = OnceLock::new();

fn get_handles<'a>() -> MutexGuard<'a, Vec<RegisteredHandleItem>> {
    HANDLES
        .get_or_init(|| {
            // Having a handle with zero as value (i.e. the very first one) is error
            // prone, so instead we reserve it with a dummy value
            Mutex::new(vec![RegisteredHandleItem::Closed])
        })
        .lock()
        .expect("Mutex is poisoned")
}

pub(crate) struct InitializingGuard {
    handle: Handle,
    started: bool,
}

impl InitializingGuard {
    pub fn initialized(mut self, mark_initialized_cb: impl FnOnce(&mut HandleItem)) -> Handle {
        let mut guard = get_handles();

        // Replace the handle item by the new one
        if let Some(RegisteredHandleItem::Open(item)) = guard.get_mut(self.handle as usize) {
            mark_initialized_cb(item);
        }

        // Prevent the drop from closing our handle !
        self.started = true;

        self.handle
    }

    pub fn handle(&self) -> Handle {
        self.handle
    }
}

impl Drop for InitializingGuard {
    fn drop(&mut self) {
        if self.started {
            return;
        }

        // The guard has been dropped without finalizing the start, there is nothing to do
        // but to close the handle (otherwise precondition will block any new start attempt)
        let mut guard = get_handles();

        if let Some(maybe_running) = guard.get_mut(self.handle as usize) {
            *maybe_running = RegisteredHandleItem::Closed;
        }
    }
}

pub(crate) fn register_handle_with_init<E>(
    initializing: HandleItem,
    precondition: impl Fn(Handle, &mut HandleItem) -> Result<(), E>,
) -> Result<InitializingGuard, E> {
    let mut guard = get_handles();

    for (handle, maybe_running) in guard.iter_mut().enumerate() {
        if let RegisteredHandleItem::Open(item) = maybe_running {
            precondition(handle as Handle, item)?;
        }
    }

    let handle = guard.len() as Handle;
    guard.push(RegisteredHandleItem::Open(Box::new(initializing)));
    Ok(InitializingGuard {
        handle,
        started: false,
    })
}

pub(crate) fn register_handle(item: HandleItem) -> Handle {
    let mut guard = get_handles();

    let handle = guard.len();
    guard.push(RegisteredHandleItem::Open(Box::new(item)));
    handle as Handle
}

pub(crate) enum FilterCloseHandle {
    Keep,
    Close,
}

pub(crate) fn filter_close_handles(
    start_at: Handle,
    mut filter: impl FnMut(&mut HandleItem) -> FilterCloseHandle,
) {
    let mut guard = get_handles();

    for maybe_running in guard.iter_mut().skip(start_at as usize) {
        if let RegisteredHandleItem::Open(item) = maybe_running {
            if let FilterCloseHandle::Close = filter(item) {
                *maybe_running = RegisteredHandleItem::Closed;
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub(crate) fn iter_opened_handles(mut callback: impl FnMut(Handle, &HandleItem)) {
    let guard = get_handles();
    for (handle, maybe_opened) in guard.iter().enumerate() {
        if let RegisteredHandleItem::Open(item) = maybe_opened {
            callback(handle as u32, item);
        }
    }
}

pub(crate) fn take_and_close_handle<T>(
    handle: Handle,
    mut mapper: impl FnMut(Box<HandleItem>) -> Result<T, Box<HandleItem>>,
) -> anyhow::Result<T> {
    let mut guard = get_handles();

    guard
        .get_mut(handle as usize)
        .and_then(|maybe_running| {
            match maybe_running {
                RegisteredHandleItem::Closed => None,
                RegisteredHandleItem::Open(_) => {
                    let item = std::mem::replace(maybe_running, RegisteredHandleItem::Closed);
                    let outcome = match item {
                        RegisteredHandleItem::Open(item) => mapper(item),
                        _ => unreachable!(), // Already checked
                    };
                    match outcome {
                        // The handle pointed to a valid running item, but which type was
                        // unexpected, the close is hence cancelled !
                        Err(item) => {
                            *maybe_running = RegisteredHandleItem::Open(item);
                            None
                        }
                        Ok(ret) => Some(ret),
                    }
                }
            }
        })
        .ok_or_else(|| anyhow::anyhow!(INVALID_HANDLE_ERROR_MSG))
}

pub(crate) fn borrow_from_handle<T>(
    handle: Handle,
    mapper: impl Fn(&mut HandleItem) -> Option<T>,
) -> anyhow::Result<T> {
    let mut guard = get_handles();

    guard
        .get_mut(handle as usize)
        .and_then(|maybe_running| match maybe_running {
            RegisteredHandleItem::Closed => None,
            RegisteredHandleItem::Open(item) => mapper(item),
        })
        .ok_or_else(|| anyhow::anyhow!(INVALID_HANDLE_ERROR_MSG))
}
