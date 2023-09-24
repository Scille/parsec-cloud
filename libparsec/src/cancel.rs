// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Unlike `libparsec_client::Client` that is totally concurrency proof, the
// greeter&claimer should be used one call a at time, and on top of that
// said call may take a while (e.g. peer not responding to a wait_peer)
// so it must be possible to cancel the call altogether at any time.
//
// Note we don't try to go fancy here and only implement this with a one-way
// signal, hence there is no confirmation the call has actually stopped.
// The idea is the event should be polled along the actual work of the call
// and (given the coroutine should be IO-bound) hence should close right away.

use std::sync::{Mutex, OnceLock};

use libparsec_types::prelude::*;

use crate::handle::Handle;

pub(crate) enum Canceller {
    // Canceller has been created
    Free(libparsec_platform_async::event::Event),
    // Canceller is now listened by a running task
    Bound(libparsec_platform_async::event::Event),
    // The task has finished, the canceller's mission is done
    Terminated,
}

static CANCELLERS: OnceLock<Mutex<Vec<Canceller>>> = OnceLock::new();

pub fn new_canceller() -> Handle {
    let mut guard = CANCELLERS
        .get_or_init(Default::default)
        .lock()
        .expect("Mutex is poisoned");
    let handle = guard.len();
    guard.push(Canceller::Free(
        libparsec_platform_async::event::Event::new(),
    ));
    handle as Handle
}

pub(crate) struct CancelListenGuard(Handle);

impl Drop for CancelListenGuard {
    fn drop(&mut self) {
        let mut guard = CANCELLERS
            .get_or_init(Default::default)
            .lock()
            .expect("Mutex is poisoned");

        match guard.get_mut(self.0 as usize) {
            Some(c) => {
                *c = Canceller::Terminated;
            }
            // Handles are never removed from `CANCELLERS`
            None => unreachable!(),
        }
    }
}

pub(crate) fn listen_canceller(
    canceller: Handle,
) -> Result<
    (
        libparsec_platform_async::event::EventListener,
        CancelListenGuard,
    ),
    anyhow::Error,
> {
    let mut guard = CANCELLERS
        .get_or_init(Default::default)
        .lock()
        .expect("Mutex is poisoned");

    match guard.get_mut(canceller as usize) {
        None => Err(anyhow::anyhow!("Invalid canceller handle")),
        Some(c) => match c {
            Canceller::Free(event) => {
                let listener = event.listen();

                // Dummy swap to extract the event object
                let event = match std::mem::replace(c, Canceller::Terminated) {
                    Canceller::Free(event) => event,
                    _ => unreachable!(),
                };
                // Actual update
                *c = Canceller::Bound(event);

                Ok((listener, CancelListenGuard(canceller)))
            }
            _ => Err(anyhow::anyhow!(
                "Canceller already connected to another task"
            )),
        },
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CancelError {
    #[error("Canceller is not connected to any task")]
    NotBound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub fn cancel(canceller: Handle) -> Result<(), CancelError> {
    let mut guard = CANCELLERS
        .get_or_init(Default::default)
        .lock()
        .expect("Mutex is poisoned");

    match guard.get_mut(canceller as usize) {
        Some(x) => match x {
            Canceller::Free(_) => Err(CancelError::NotBound),
            Canceller::Terminated => Ok(()), // Idempotent
            Canceller::Bound(event) => {
                event.notify(usize::MAX);
                Ok(())
            }
        },
        None => Err(anyhow::anyhow!("Invalid handle").into()),
    }
}
