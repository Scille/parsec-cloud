// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::future::Future;

use libparsec_platform_async::{spawn, JoinHandle};

use crate::event_bus::EventBus;

/// A monitor is a background task reacting to external events
///
/// Typical monitor runs a single coroutine that listens on the event bus
/// and calls methods on a ops component
pub(crate) struct Monitor {
    task: JoinHandle<()>,
    // The task can itself start additional sub-tasks (e.g. to sync files in parallel),
    // this callback is used to notify about the stop so that the sub-tasks are closed.
    stop_cb: Option<Box<dyn FnOnce() + Send + 'static>>,
    #[allow(unused)]
    event_bus: EventBus,
}

impl Monitor {
    #[allow(unused)]
    pub async fn start<Fut>(
        event_bus: EventBus,
        task: Fut,
        stop_cb: Option<Box<dyn FnOnce() + Send + 'static>>,
    ) -> Self
    where
        Fut: Future<Output = ()> + Send + 'static,
    {
        let task = spawn(task);

        Self {
            task,
            event_bus,
            stop_cb,
        }
    }

    /// Abort the monitor task and wait until the task has actually finished
    #[allow(unused)]
    pub async fn stop(&mut self) {
        if let Some(stop_cb) = self.stop_cb.take() {
            stop_cb();
        } else {
            self.task.abort();
        }
        let m = &mut self.task;
        // TODO: do we care about the error ?
        let _ = m.await;
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
