// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_async::event::{Event, EventListener};
use libparsec_types::prelude::*;

/// This structure provide a way to lock independently each manifest for update.
/// Once taken, a `ManifestUpdateLockGuard` is returned to the caller, which must
/// be in turn provided to `release` once the lock should be released.
#[derive(Debug)]
pub(super) struct PerManifestUpdateLock {
    per_manifest_lock: Vec<(VlobID, EntryLockState)>,
}

/// We cannot just use a simple async mutex to protect update of manifests.
///
/// This is because the manifests cache is itself behind a sync mutex (which
/// must be released before waiting on the per manifest async mutex !).
/// The natural solution to this is to wrap the per manifest async mutex in an
/// Arc, but this creates a new issue: the `ManifestUpdateLockGuard` would then
/// have to keep both the `Arc<AsyncMutex>` and the corresponding `AsyncMutexGuard`.
/// However this is not possible given the guard takes a reference on the mutex !
///
/// Hence we basically have to re-implement a custom async mutex using the lower-level
/// event system.
#[derive(Debug)]
enum EntryLockState {
    /// A single coroutine has locked the manifest
    Taken,
    /// Multiple coroutines want to lock the manifest: the first has the lock
    /// and the subsequent ones are listening on the event (which will be fired
    /// when the first coroutine releases the manifest).
    /// Note this means all coroutines waiting for the event will fight with no
    /// fairness to take the lock (it's basically up to the tokio scheduler).
    TakenWithConcurrency(Event),
}

pub(super) enum ManifestUpdateLockTakeOutcome {
    // The updater object is responsible to release the update lock on drop.
    // Hence this object must be created as soon as possible otherwise a deadlock
    // could occur (typically in case of an early return due to error handling)
    Taken(ManifestUpdateLockGuard),
    NeedWait(EventListener),
}

/// ⚠️ This lock doesn't release on drop ⚠️
///
/// Instead you must manually call `PerManifestUpdateLock::release` once you are done.
#[derive(Debug)] // Note this should not be made `Clone` !
pub(super) struct ManifestUpdateLockGuard {
    entry_id: VlobID,
}

// We use this drop to detect misuse of the guard (i.e. not releasing it properly).
//
// Note we only log an error instead of panicking, this is because this
// "drop without release" check will be triggered any time another error in the
// tests makes us skip the release part. Worst, in this case the only outputted
// error is this panic message instead of the original error :/
impl Drop for ManifestUpdateLockGuard {
    fn drop(&mut self) {
        // We use `std::mem::forget` to avoid running this guard drop when
        // the release is done properly.
        log::error!(
            "Manifest `{}` guard dropped without being released !",
            self.entry_id
        );
    }
}

impl PerManifestUpdateLock {
    pub(super) fn new() -> Self {
        Self {
            per_manifest_lock: Vec::new(),
        }
    }

    pub(super) fn is_taken(&self, entry_id: VlobID) -> bool {
        self.per_manifest_lock.iter().any(|(id, _)| *id == entry_id)
    }

    pub(super) fn take(&mut self, entry_id: VlobID) -> ManifestUpdateLockTakeOutcome {
        let found = self
            .per_manifest_lock
            .iter_mut()
            .find(|(id, _)| *id == entry_id);

        match found {
            // Entry is missing: the manifest is not currently under update
            None => {
                self.per_manifest_lock
                    .push((entry_id, EntryLockState::Taken));
                let guard = ManifestUpdateLockGuard { entry_id };
                // It's official: we are now the one and only updating this manifest !
                ManifestUpdateLockTakeOutcome::Taken(guard)
            }

            // The entry is present: the manifest is already taken for update !
            Some((_, state)) => match state {
                // Apart from the coroutine currently updating the manifest, nobody
                // else is waiting for it... except us!
                // So it's our job to setup the event to get notified when the manifest
                // is again available for update.
                EntryLockState::Taken => {
                    let event = Event::new();
                    let listener = event.listen();
                    *state = EntryLockState::TakenWithConcurrency(event);
                    ManifestUpdateLockTakeOutcome::NeedWait(listener)
                }
                // Multiple coroutines are already waiting to get their hands on
                // this manifest, we are just the n+1 :)
                EntryLockState::TakenWithConcurrency(event) => {
                    ManifestUpdateLockTakeOutcome::NeedWait(event.listen())
                }
            },
        }
    }

    pub(super) fn try_take(&mut self, entry_id: VlobID) -> Option<ManifestUpdateLockGuard> {
        let found = self
            .per_manifest_lock
            .iter_mut()
            .find(|(id, _)| *id == entry_id);

        match found {
            // Entry is missing: the manifest is not currently under update
            None => {
                self.per_manifest_lock
                    .push((entry_id, EntryLockState::Taken));
                let guard = ManifestUpdateLockGuard { entry_id };
                // It's official: we are now the one and only updating this manifest !
                Some(guard)
            }

            // The entry is present: the manifest is already taken for update !
            Some(_) => None,
        }
    }

    pub(super) fn release(&mut self, guard: ManifestUpdateLockGuard) {
        let (index, (_, state)) = self
            .per_manifest_lock
            .iter()
            .enumerate()
            .find(|(_, (id, _))| *id == guard.entry_id)
            // `ManifestUpdateLockGuard` constructor is private, so it can only be
            // created in `take`, and is only removed once here (since we take ownership).
            .expect("Must be present");

        // If other coroutines are waiting for the manifest, now is the time to notify them !
        if let EntryLockState::TakenWithConcurrency(event) = state {
            event.notify(usize::MAX);
        }

        // We remove the entry altogether, this is the way to signify it is free to take now
        self.per_manifest_lock.swap_remove(index);

        // Finally forget about the guard to avoid running its drop (since it
        // is expected to be called only when the guard is improperly dropped).
        std::mem::forget(guard);
    }

    // See `WorkspaceDataStorageChildManifestUpdater`'s drop for the release part
}
