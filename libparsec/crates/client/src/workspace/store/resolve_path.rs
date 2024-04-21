// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_platform_async::event::EventListener;
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::store::per_manifest_update_lock::ManifestUpdateLockTakeOutcome,
};

use super::{
    cache::{
        populate_cache_from_local_storage_or_server, PopulateCacheFromLocalStorageOrServerError,
    },
    per_manifest_update_lock::ManifestUpdateLockGuard,
};

/// The confinement point corresponds to the entry ID of the folder manifest that
/// contains a child with a confined name when resolving a path.
/// Also note that if multiple entries in the path are confined, it is the outermost
/// that takes precedence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PathConfinementPoint {
    None,
    Confined(VlobID),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ResolvePathError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

/// Be careful with the returned `ManifestUpdateLockGuard` as it must be released manually !
///
/// This function is for internal use (i.e. not to be exposed by `WorkspaceStore`) for
/// implementing file & folder updater. The reason for this is we need to have a way
/// to atomically resolve the path and lock the target manifest for update.
pub(super) async fn resolve_path_and_lock_for_update(
    store: &super::WorkspaceStore,
    path: &FsPath,
) -> Result<
    (
        ArcLocalChildManifest,
        PathConfinementPoint,
        ManifestUpdateLockGuard,
    ),
    ResolvePathError,
> {
    resolve_path_maybe_lock_for_update(store, path, true)
        .await
        .map(|(manifest, confinement_point, lock_guard)| {
            (
                manifest,
                confinement_point,
                lock_guard.expect("always present"),
            )
        })
}

pub(super) async fn resolve_path(
    store: &super::WorkspaceStore,
    path: &FsPath,
) -> Result<(ArcLocalChildManifest, PathConfinementPoint), ResolvePathError> {
    resolve_path_maybe_lock_for_update(store, path, false)
        .await
        .map(|(manifest, confinement_point, _)| (manifest, confinement_point))
}

async fn resolve_path_maybe_lock_for_update(
    store: &super::WorkspaceStore,
    path: &FsPath,
    lock_for_update: bool,
) -> Result<
    (
        ArcLocalChildManifest,
        PathConfinementPoint,
        Option<ManifestUpdateLockGuard>,
    ),
    ResolvePathError,
> {
    // A word about circular path detection:
    // - The path is resolved from the root, which itself is guaranteed to have its
    //   `parent` field pointing on itself.
    // - Each time we resolve a child, we ensure the parent and child agree on the
    //   parenting relationship.
    //
    // This in practice means our valid data form a direct acyclic graph (DAG) and henc
    // we cannot end up in a circular path.
    //
    // Note this is only valid because we start the resolution from the root, if we
    // would be to start from elsewhere (typically to determine the path of a manifest
    // by following its parent backward), we could end up in a circular path with
    // manifests involved forming a "island" disconnect from the root.

    let path_parts = path.parts();

    loop {
        // Most of the time we should have each entry in the path already in the cache,
        // so we want to lock the cache once and only release it in the unlikely case
        // we need to fetch from the local storage or server.
        let cache_only_outcome = {
            let mut cache = store.current_view_cache.lock().expect("Mutex is poisoned");
            cache_only_path_resolution(store, &mut cache, path_parts, lock_for_update)
        };
        match cache_only_outcome {
            CacheOnlyPathResolutionOutcome::Done {
                manifest,
                confinement,
                maybe_update_lock_guard,
            } => return Ok((manifest, confinement, maybe_update_lock_guard)),
            CacheOnlyPathResolutionOutcome::EntryNotFound => {
                return Err(ResolvePathError::EntryNotFound)
            }
            // We got a cache miss
            CacheOnlyPathResolutionOutcome::NeedPopulateCache(cache_miss_entry_id) => {
                populate_cache_from_local_storage_or_server(store, cache_miss_entry_id)
                    .await
                    .map_err(|err| match err {
                        PopulateCacheFromLocalStorageOrServerError::Offline => {
                            ResolvePathError::Offline
                        }
                        PopulateCacheFromLocalStorageOrServerError::Stopped => {
                            ResolvePathError::Stopped
                        }
                        PopulateCacheFromLocalStorageOrServerError::EntryNotFound => {
                            ResolvePathError::EntryNotFound
                        }
                        PopulateCacheFromLocalStorageOrServerError::NoRealmAccess => {
                            ResolvePathError::NoRealmAccess
                        }
                        PopulateCacheFromLocalStorageOrServerError::InvalidKeysBundle(err) => {
                            ResolvePathError::InvalidKeysBundle(err)
                        }
                        PopulateCacheFromLocalStorageOrServerError::InvalidCertificate(err) => {
                            ResolvePathError::InvalidCertificate(err)
                        }
                        PopulateCacheFromLocalStorageOrServerError::InvalidManifest(err) => {
                            ResolvePathError::InvalidManifest(err)
                        }
                        PopulateCacheFromLocalStorageOrServerError::Internal(err) => {
                            err.context("cannot fetch manifest").into()
                        }
                    })?;
            }
            // The path resolution is done, but we must wait for the target entry to be available
            // before being able to lock it for update ourself
            CacheOnlyPathResolutionOutcome::NeedWaitForTakenUpdateLock(need_wait) => {
                need_wait.await;
            }
        }
    }
}

enum CacheOnlyPathResolutionOutcome {
    Done {
        manifest: ArcLocalChildManifest,
        confinement: PathConfinementPoint,
        maybe_update_lock_guard: Option<ManifestUpdateLockGuard>,
    },
    EntryNotFound,
    NeedPopulateCache(VlobID),
    NeedWaitForTakenUpdateLock(EventListener),
}

fn cache_only_path_resolution(
    store: &super::WorkspaceStore,
    cache: &mut super::CurrentViewCache,
    path_parts: &[EntryName],
    lock_for_update: bool,
) -> CacheOnlyPathResolutionOutcome {
    enum StepKind {
        Root,
        Child {
            manifest: ArcLocalChildManifest,
            confinement: PathConfinementPoint,
        },
    }

    let mut last_step = StepKind::Root;
    let mut parts_index = 0;
    loop {
        let child_name = match path_parts.get(parts_index) {
            Some(part) => part,
            // The path is entirely resolved !
            None => {
                // Note it is *vital* to return `maybe_update_lock_guard` to the caller
                // given it may contains a lock that won't be released on drop !
                let maybe_update_lock_guard = if lock_for_update {
                    let id = match &last_step {
                        StepKind::Root => store.realm_id,
                        StepKind::Child { manifest, .. } => manifest.id(),
                    };
                    match cache.lock_update_manifests.take(id) {
                        ManifestUpdateLockTakeOutcome::Taken(lock) => Some(lock),
                        ManifestUpdateLockTakeOutcome::NeedWait(need_wait) => {
                            return CacheOnlyPathResolutionOutcome::NeedWaitForTakenUpdateLock(
                                need_wait,
                            );
                        }
                    }
                } else {
                    None
                };

                match last_step {
                    StepKind::Child {
                        manifest,
                        confinement,
                    } => {
                        return CacheOnlyPathResolutionOutcome::Done {
                            manifest,
                            confinement,
                            maybe_update_lock_guard,
                        };
                    }
                    StepKind::Root => {
                        let manifest = ArcLocalChildManifest::Folder(cache.root_manifest.clone());
                        let confinement = PathConfinementPoint::None;
                        return CacheOnlyPathResolutionOutcome::Done {
                            manifest,
                            confinement,
                            maybe_update_lock_guard,
                        };
                    }
                }
            }
        };

        let (parent_manifest, parent_confinement) = match &last_step {
            StepKind::Root => (&cache.root_manifest, PathConfinementPoint::None),
            StepKind::Child {
                manifest,
                confinement,
            } => match &manifest {
                ArcLocalChildManifest::Folder(manifest) => (manifest, *confinement),
                ArcLocalChildManifest::File(_) => {
                    // Cannot continue to resolve the path !
                    return CacheOnlyPathResolutionOutcome::EntryNotFound;
                }
            },
        };
        let parent_id = parent_manifest.base.id;

        let child_id = match parent_manifest.children.get(child_name) {
            Some(id) => *id,
            None => return CacheOnlyPathResolutionOutcome::EntryNotFound,
        };

        let child_manifest = match cache.child_manifests.get(&child_id) {
            Some(manifest) => manifest.to_owned(),
            // Cache miss !
            // `part_index` is not incremented here, so we are going to
            // leave the second loop, populate the cache, loop into first
            // loop and finally re-enter the second loop with the same
            // part in the path to resolve.
            None => return CacheOnlyPathResolutionOutcome::NeedPopulateCache(child_id),
        };

        // Ensure the child agrees with the parent on the parenting relationship
        if child_manifest.parent() != parent_id {
            return CacheOnlyPathResolutionOutcome::EntryNotFound;
        }

        // Top-most confinement point shadows child ones if any
        let confinement = match parent_confinement {
            confinement @ PathConfinementPoint::Confined(_) => confinement,
            PathConfinementPoint::None => {
                if parent_manifest.local_confinement_points.contains(&child_id) {
                    PathConfinementPoint::Confined(parent_id)
                } else {
                    PathConfinementPoint::None
                }
            }
        };

        last_step = StepKind::Child {
            manifest: child_manifest,
            confinement,
        };
        parts_index += 1;
    }
}

pub(crate) struct ResolvePathForReparenting {
    pub src_parent_manifest: Arc<LocalFolderManifest>,
    pub src_parent_update_lock_guard: ManifestUpdateLockGuard,
    pub src_child_manifest: ArcLocalChildManifest,
    pub src_child_update_lock_guard: ManifestUpdateLockGuard,
    pub dst_parent_manifest: Arc<LocalFolderManifest>,
    pub dst_parent_update_lock_guard: ManifestUpdateLockGuard,
    // In most reparenting we don't even care if the destination exists given
    // it is simply overwritten by the source.
    // However this is not the case when doing an exchange between source and
    // destination, given in this case the destination's `parent` field must
    // also be modified.
    pub dst_child_manifest: Option<ArcLocalChildManifest>,
    pub dst_child_update_lock_guard: Option<ManifestUpdateLockGuard>,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ResolvePathForReparentingError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Source doesn't exist")]
    SourceNotFound,
    #[error("Destination doesn't exist")]
    DestinationNotFound,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(crate) async fn resolve_path_for_reparenting(
    store: &super::WorkspaceStore,
    src_parent_path: &FsPath,
    src_child_name: &EntryName,
    dst_parent_path: &FsPath,
    // In most reparenting we don't even care if the destination exists given
    // it is simply overwritten by the source.
    // However this is not the case when doing an exchange between source and
    // destination, given in this case the destination's `parent` field must
    // also be modified.
    dst_child_name: Option<&EntryName>,
) -> Result<ResolvePathForReparenting, ResolvePathForReparentingError> {
    // A word about circular path detection:
    // - The path is resolved from the root, which itself is guaranteed to have its
    //   `parent` field pointing on itself.
    // - Each time we resolve a child, we ensure the parent and child agree on the
    //   parenting relationship.
    //
    // This in practice means our valid data form a direct acyclic graph (DAG) and henc
    // we cannot end up in a circular path.
    //
    // Note this is only valid because we start the resolution from the root, if we
    // would be to start from elsewhere (typically to determine the path of a manifest
    // by following its parent backward), we could end up in a circular path with
    // manifests involved forming a "island" disconnect from the root.

    let src_parent_path_parts = src_parent_path.parts();
    let dst_parent_path_parts = dst_parent_path.parts();

    loop {
        // On top of path resolution, the key part of this function is to atomically
        // lock three update guards.
        // However it is easy to mess up the logic that release what has been locked
        // so far in case a subsequent lock cannot be achieved for now (e.g. because
        // of cache miss or lock already held by another coroutine).
        // Hence this `AutoReleaseGuards` structure whose drop makes sure the locks
        // are released no matter what.

        struct AutoReleaseGuards<'a> {
            cache: &'a mut super::CurrentViewCache,
            src_parent_guard: Option<ManifestUpdateLockGuard>,
            src_child_guard: Option<ManifestUpdateLockGuard>,
            dst_parent_guard: Option<ManifestUpdateLockGuard>,
            dst_child_guard: Option<ManifestUpdateLockGuard>,
        }
        impl Drop for AutoReleaseGuards<'_> {
            fn drop(&mut self) {
                if let Some(guard) = self.src_parent_guard.take() {
                    self.cache.lock_update_manifests.release(guard);
                }
                if let Some(guard) = self.src_child_guard.take() {
                    self.cache.lock_update_manifests.release(guard);
                }
                if let Some(guard) = self.dst_parent_guard.take() {
                    self.cache.lock_update_manifests.release(guard);
                }
            }
        }

        enum WhoIsInNeed {
            SrcParent,
            SrcChild,
            DstParent,
            DstChild,
        }

        let (needed_before_resolution, who_need) = 'needed_before_resolution: {
            let mut cache = store.current_view_cache.lock().expect("Mutex is poisoned");
            let mut auto_release_guards = AutoReleaseGuards {
                cache: &mut cache,
                src_parent_guard: None,
                src_child_guard: None,
                dst_parent_guard: None,
                dst_child_guard: None,
            };

            // 1) Resolve the destination parent path and lock for update

            let dst_parent_resolution_outcome = cache_only_path_resolution(
                store,
                auto_release_guards.cache,
                dst_parent_path_parts,
                true,
            );
            let dst_parent_manifest = match dst_parent_resolution_outcome {
                CacheOnlyPathResolutionOutcome::Done {
                    manifest,
                    maybe_update_lock_guard,
                    ..
                } => {
                    auto_release_guards.dst_parent_guard =
                        Some(maybe_update_lock_guard.expect("always present"));
                    match manifest {
                        ArcLocalChildManifest::File(_) => {
                            return Err(ResolvePathForReparentingError::DestinationNotFound)
                        }
                        ArcLocalChildManifest::Folder(manifest) => manifest,
                    }
                }

                CacheOnlyPathResolutionOutcome::EntryNotFound => {
                    return Err(ResolvePathForReparentingError::DestinationNotFound)
                }

                other_outcome => {
                    break 'needed_before_resolution (other_outcome, WhoIsInNeed::DstParent)
                }
            };

            // 2) Resolve the source parent path and lock for update

            let src_parent_resolution_outcome = cache_only_path_resolution(
                store,
                &mut auto_release_guards.cache,
                src_parent_path_parts,
                true,
            );
            let src_parent_manifest = match src_parent_resolution_outcome {
                CacheOnlyPathResolutionOutcome::Done {
                    manifest,
                    maybe_update_lock_guard,
                    ..
                } => {
                    auto_release_guards.src_parent_guard =
                        Some(maybe_update_lock_guard.expect("always present"));
                    match manifest {
                        ArcLocalChildManifest::File(_) => {
                            return Err(ResolvePathForReparentingError::SourceNotFound)
                        }
                        ArcLocalChildManifest::Folder(manifest) => manifest,
                    }
                }

                CacheOnlyPathResolutionOutcome::EntryNotFound => {
                    return Err(ResolvePathForReparentingError::SourceNotFound)
                }

                other_outcome => {
                    break 'needed_before_resolution (other_outcome, WhoIsInNeed::SrcParent)
                }
            };

            // 3) Resolve the source child path and lock for update

            let src_child_manifest = {
                let src_child_id = match src_parent_manifest.children.get(&src_child_name) {
                    Some(src_child_id) => *src_child_id,
                    None => return Err(ResolvePathForReparentingError::SourceNotFound),
                };

                let src_child_manifest =
                    match auto_release_guards.cache.child_manifests.get(&src_child_id) {
                        Some(manifest) => manifest.to_owned(),
                        // Cache miss !
                        None => {
                            break 'needed_before_resolution (
                                CacheOnlyPathResolutionOutcome::NeedPopulateCache(src_child_id),
                                WhoIsInNeed::SrcChild,
                            )
                        }
                    };

                // Ensure the child agrees with the parent on the parenting relationship
                if src_child_manifest.parent() != src_parent_manifest.base.id {
                    return Err(ResolvePathForReparentingError::SourceNotFound);
                }

                let maybe_update_lock_guard = match auto_release_guards
                    .cache
                    .lock_update_manifests
                    .take(src_child_id)
                {
                    ManifestUpdateLockTakeOutcome::Taken(lock) => Some(lock),
                    ManifestUpdateLockTakeOutcome::NeedWait(need_wait) => {
                        break 'needed_before_resolution (
                            CacheOnlyPathResolutionOutcome::NeedWaitForTakenUpdateLock(need_wait),
                            WhoIsInNeed::SrcChild,
                        );
                    }
                };
                auto_release_guards.src_child_guard =
                    Some(maybe_update_lock_guard.expect("always present"));

                src_child_manifest
            };

            // 4) Resolve the destination child path and lock for update

            let dst_child_manifest = if let Some(dst_child_name) = dst_child_name {
                let dst_child_id = match dst_parent_manifest.children.get(&dst_child_name) {
                    Some(dst_child_id) => *dst_child_id,
                    None => return Err(ResolvePathForReparentingError::DestinationNotFound),
                };

                let dst_child_manifest =
                    match auto_release_guards.cache.child_manifests.get(&dst_child_id) {
                        Some(manifest) => manifest.to_owned(),
                        // Cache miss !
                        None => {
                            break 'needed_before_resolution (
                                CacheOnlyPathResolutionOutcome::NeedPopulateCache(dst_child_id),
                                WhoIsInNeed::DstChild,
                            )
                        }
                    };

                // Ensure the child agrees with the parent on the parenting relationship
                if dst_child_manifest.parent() != dst_parent_manifest.base.id {
                    return Err(ResolvePathForReparentingError::DestinationNotFound);
                }

                let maybe_update_lock_guard = match auto_release_guards
                    .cache
                    .lock_update_manifests
                    .take(dst_child_id)
                {
                    ManifestUpdateLockTakeOutcome::Taken(lock) => Some(lock),
                    ManifestUpdateLockTakeOutcome::NeedWait(need_wait) => {
                        break 'needed_before_resolution (
                            CacheOnlyPathResolutionOutcome::NeedWaitForTakenUpdateLock(need_wait),
                            WhoIsInNeed::DstChild,
                        );
                    }
                };
                auto_release_guards.dst_child_guard =
                    Some(maybe_update_lock_guard.expect("always present"));

                Some(dst_child_manifest)
            } else {
                None
            };

            // 5) All done, we defuse the auto release system to return it guards

            return Ok(ResolvePathForReparenting {
                src_parent_update_lock_guard: auto_release_guards
                    .src_parent_guard
                    .take()
                    .expect("always present"),
                src_child_update_lock_guard: auto_release_guards
                    .src_child_guard
                    .take()
                    .expect("always present"),
                dst_parent_update_lock_guard: auto_release_guards
                    .dst_parent_guard
                    .take()
                    .expect("always present"),
                dst_child_update_lock_guard: auto_release_guards.dst_parent_guard.take(),
                src_parent_manifest,
                src_child_manifest,
                dst_parent_manifest,
                dst_child_manifest,
            });
        };

        match needed_before_resolution {
            // We got a cache miss
            CacheOnlyPathResolutionOutcome::NeedPopulateCache(cache_miss_entry_id) => {
                populate_cache_from_local_storage_or_server(store, cache_miss_entry_id)
                    .await
                    .map_err(|err| match err {
                        PopulateCacheFromLocalStorageOrServerError::Offline => {
                            ResolvePathForReparentingError::Offline
                        }
                        PopulateCacheFromLocalStorageOrServerError::Stopped => {
                            ResolvePathForReparentingError::Stopped
                        }
                        PopulateCacheFromLocalStorageOrServerError::EntryNotFound => match who_need
                        {
                            WhoIsInNeed::DstParent | WhoIsInNeed::DstChild => {
                                ResolvePathForReparentingError::DestinationNotFound
                            }
                            WhoIsInNeed::SrcParent | WhoIsInNeed::SrcChild => {
                                ResolvePathForReparentingError::SourceNotFound
                            }
                        },
                        PopulateCacheFromLocalStorageOrServerError::NoRealmAccess => {
                            ResolvePathForReparentingError::NoRealmAccess
                        }
                        PopulateCacheFromLocalStorageOrServerError::InvalidKeysBundle(err) => {
                            ResolvePathForReparentingError::InvalidKeysBundle(err)
                        }
                        PopulateCacheFromLocalStorageOrServerError::InvalidCertificate(err) => {
                            ResolvePathForReparentingError::InvalidCertificate(err)
                        }
                        PopulateCacheFromLocalStorageOrServerError::InvalidManifest(err) => {
                            ResolvePathForReparentingError::InvalidManifest(err)
                        }
                        PopulateCacheFromLocalStorageOrServerError::Internal(err) => {
                            err.context("cannot fetch manifest").into()
                        }
                    })?;
            }

            // The path resolution is done, but we must wait for the target entry to be available
            // before being able to lock it for update ourself
            CacheOnlyPathResolutionOutcome::NeedWaitForTakenUpdateLock(need_wait) => {
                need_wait.await;
            }

            // Already handled in the part above
            CacheOnlyPathResolutionOutcome::EntryNotFound
            | CacheOnlyPathResolutionOutcome::Done { .. } => unreachable!(),
        }
    }
}
