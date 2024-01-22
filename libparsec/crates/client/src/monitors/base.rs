// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::future::Future;

use libparsec_platform_async::{spawn, JoinHandle};
use libparsec_types::prelude::*;

use crate::event_bus::EventBus;

/// A monitor is a background task reacting to the events.
///
/// Typical monitor runs a single coroutine that listens on the event bus
/// and calls methods on an ops component.
pub(crate) struct Monitor {
    pub name: &'static str,
    /// Not `None` if the monitor is related to a specific workspace
    pub workspace_id: Option<VlobID>,
    task: JoinHandle<()>,
    // The task can itself start additional sub-tasks (e.g. to sync files in parallel),
    // this callback is used to notify about the stop so that the sub-tasks are closed.
    stop_cb: Option<Box<dyn FnOnce() + Send + 'static>>,
}

impl Monitor {
    #[allow(unused)]
    pub async fn start<Fut>(
        _event_bus: EventBus,
        name: &'static str,
        workspace_id: Option<VlobID>,
        future: Fut,
        stop_cb: Option<Box<dyn FnOnce() + Send + 'static>>,
    ) -> Self
    where
        Fut: Future<Output = ()> + Send + 'static,
    {
        // TODO: should wrap the task to send an event if it panics
        let task = spawn(future);

        Self {
            name,
            workspace_id,
            task,
            stop_cb,
        }
    }

    /// Abort the monitor task and wait until the task has actually finished
    #[allow(unused)]
    pub async fn stop(mut self) -> anyhow::Result<()> {
        if let Some(stop_cb) = self.stop_cb.take() {
            stop_cb();
        } else {
            self.task.abort();
        }
        let m = &mut self.task;
        m.await.map_err(|e| e.into())
    }
}

impl Drop for Monitor {
    fn drop(&mut self) {
        if !self.task.is_finished() {
            // This is unexpected: the task is still running !
            //
            // In theory `Monitor::stop` is supposed to be called to ensure the
            // monitor is not processing something while the ops component it is
            // based on is stopped.
            //
            // If that's not the case multiple things can go wrong:
            // - The ops component is used while stopping/stopped. This will
            //   likely trigger warning events and inconsistent state in memory
            //   (the database is closed, so on-disk data are not affected)
            // - If any, the sub tasks started by the monitor won't be aborted.
            //   They should eventually crash while trying to use the ops component
            //   once it is closed.
            //
            // In any way, there is a bug in the implementation, so we log an error
            // about it and we abort the task as last-ditch effort to reach a stable state.
            self.task.abort();
            // TODO: send an error event about this unexpected behavior !
        }
    }
}
