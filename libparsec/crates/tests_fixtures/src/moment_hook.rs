// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Mutex;

struct Hook {
    moment: Moment,
    fut: std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>,
}

static HOOKS: Mutex<Vec<Hook>> = Mutex::new(vec![]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Moment {
    /// `WorkspaceOps::outbound_sync` first take a lock to retrieve the to-be-synced
    /// entry's local manifest, then release it to do server upload, before taking
    /// it again to mark the local manifest as synced.
    /// This moment occurs right after the lock is released.
    OutboundSyncLocalRetrieved,
    /// In case of file, `WorkspaceOps::outbound_sync` first reshapes and uploads the
    // blocks, then uploads the manifest.
    /// During the reshape, the file's lock is taken multiple time to update the
    /// manifest with the reshaped block.
    /// This moment occurs right after the blocks has been uploaded.
    OutboundSyncFileReshapedAndBlockUploaded,
}

/// Used by the test code to control the execution of the tested code.
/// This is typically useful to simulate a concurrent operation occurring at
/// a specific time.
pub fn moment_inject_hook(
    moment: Moment,
    fut: impl std::future::Future<Output = ()> + Send + 'static,
) {
    let mut hooks = HOOKS.lock().expect("Mutex is poisoned");
    hooks.push(Hook {
        moment,
        fut: Box::pin(fut),
    });
}

/// Used within the tested code to specify a key moment that should wait until the
/// test decides to go further.
pub async fn moment_define_inject_point(moment: Moment) {
    let hook = {
        let mut hooks = HOOKS.lock().expect("Mutex is poisoned");
        if let Some(pos) = hooks
            .iter()
            .position(|candidate| candidate.moment == moment)
        {
            hooks.swap_remove(pos)
        } else {
            // No hook has been inject for this moment, nothing to do then.
            return;
        }
    };
    hook.fut.await;
}
