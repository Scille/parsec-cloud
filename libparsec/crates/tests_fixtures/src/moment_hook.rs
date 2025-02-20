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
    /// `WorkspaceOps::inbound_sync` first check if the entry is already locked, then
    /// fetches its corresponding remote manifest, and only then locks the entry to
    /// merge local and remote contents.
    /// This moment occurs right after the remote manifest is fetched.
    InboundSyncRemoteFetched,
    /// Before actual stop of the workspace ops, all currently opened files must be
    /// closed so that they change get flushed to database.
    /// This moment occurs in `WorkspaceOps::stop` right after the files has been closed.
    WorkspaceOpsStopAllFdsClosed,
    /// Populating the cache from local or server is done in three steps:
    /// 1. Local DB lookup
    /// 2. If cache miss, server lookup
    /// 3. If no cache miss, store the result from the server into the local DB
    ///
    /// This moment occurs right after the local DB lookup, which allows us to
    /// modify the content of the local DB while the server lookup is still pending.
    WorkspaceStorePopulateCacheFetchRemote,
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
