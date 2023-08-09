// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::{Arc, Mutex, OnceLock};

pub type Handle = u32;

pub(crate) enum RegisteredHandleItem {
    Running(HandleItem),
    Closed,
}

pub(crate) enum HandleItem {
    Client((Arc<libparsec_client::Client>, crate::OnEventCallbackPlugged)),

    UserGreetInitial(libparsec_client::UserGreetInitialCtx),
    DeviceGreetInitial(libparsec_client::DeviceGreetInitialCtx),
    UserGreetInProgress1(libparsec_client::UserGreetInProgress1Ctx),
    DeviceGreetInProgress1(libparsec_client::DeviceGreetInProgress1Ctx),
    UserGreetInProgress2(libparsec_client::UserGreetInProgress2Ctx),
    DeviceGreetInProgress2(libparsec_client::DeviceGreetInProgress2Ctx),
    UserGreetInProgress3(libparsec_client::UserGreetInProgress3Ctx),
    DeviceGreetInProgress3(libparsec_client::DeviceGreetInProgress3Ctx),
    UserGreetInProgress4(libparsec_client::UserGreetInProgress4Ctx),
    DeviceGreetInProgress4(libparsec_client::DeviceGreetInProgress4Ctx),

    UserClaimInitial(libparsec_client::UserClaimInitialCtx),
    DeviceClaimInitial(libparsec_client::DeviceClaimInitialCtx),
    UserClaimInProgress1(libparsec_client::UserClaimInProgress1Ctx),
    DeviceClaimInProgress1(libparsec_client::DeviceClaimInProgress1Ctx),
    UserClaimInProgress2(libparsec_client::UserClaimInProgress2Ctx),
    DeviceClaimInProgress2(libparsec_client::DeviceClaimInProgress2Ctx),
    UserClaimInProgress3(libparsec_client::UserClaimInProgress3Ctx),
    DeviceClaimInProgress3(libparsec_client::DeviceClaimInProgress3Ctx),
    UserClaimFinalize(libparsec_client::UserClaimFinalizeCtx),
    DeviceClaimFinalize(libparsec_client::DeviceClaimFinalizeCtx),
}

static HANDLES: OnceLock<Mutex<Vec<RegisteredHandleItem>>> = OnceLock::new();

pub(crate) fn register_handle(item: HandleItem) -> Handle {
    let mut guard = HANDLES
        .get_or_init(Default::default)
        .lock()
        .expect("Mutex is poisoned");
    let handle = guard.len();
    guard.push(RegisteredHandleItem::Running(item));
    handle as u32
}

pub(crate) fn take_and_close_handle<T>(
    handle: Handle,
    mapper: impl Fn(HandleItem) -> Option<T>,
) -> Option<T> {
    let mut guard = HANDLES
        .get_or_init(Default::default)
        .lock()
        .expect("Mutex is poisoned");

    guard.get_mut(handle as usize).and_then(|maybe_running| {
        match maybe_running {
            RegisteredHandleItem::Closed => None,
            RegisteredHandleItem::Running(_) => {
                let item = std::mem::replace(maybe_running, RegisteredHandleItem::Closed);
                match item {
                    RegisteredHandleItem::Running(item) => mapper(item),
                    _ => unreachable!(), // Already checked
                }
            }
        }
    })
}

pub(crate) fn borrow_from_handle<T>(
    handle: Handle,
    mapper: impl Fn(&mut HandleItem) -> Option<T>,
) -> Option<T> {
    let mut guard = HANDLES
        .get_or_init(Default::default)
        .lock()
        .expect("Mutex is poisoned");

    guard
        .get_mut(handle as usize)
        .and_then(|maybe_running| match maybe_running {
            RegisteredHandleItem::Closed => None,
            RegisteredHandleItem::Running(item) => mapper(item),
        })
}
