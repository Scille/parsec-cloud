// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashSet, sync::Arc};

use libparsec_client_connection::ConnectionError;
use libparsec_platform_async::event::EventListener;
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::store::per_manifest_update_lock::ManifestUpdateLockTakeOutcome,
};

use super::{
    cache::{
        populate_cache_from_local_storage, populate_cache_from_local_storage_or_server,
        PopulateCacheFromLocalStorageError, PopulateCacheFromLocalStorageOrServerError,
    },
    per_manifest_update_lock::ManifestUpdateLockGuard,
};

/// The confinement point corresponds to the entry ID of the folder manifest that
/// contains a child with a confined name when resolving a path.
/// Also note that if multiple entries in the path are confined, it is the outermost
/// that takes precedence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PathConfinementPoint {
    NotConfined,
    Confined(VlobID),
}

impl From<PathConfinementPoint> for Option<VlobID> {
    fn from(val: PathConfinementPoint) -> Self {
        match val {
            PathConfinementPoint::NotConfined => None,
            PathConfinementPoint::Confined(id) => Some(id),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ResolvePathError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
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
        .map(|(manifest, confinement_point, lock_guard)| {
            debug_assert!(lock_guard.is_none());
            (manifest, confinement_point)
        })
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
    // This in practice means our valid data form a direct acyclic graph (DAG) and hence
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
        let cache_only_outcome = store.data.with_current_view_cache(|cache| {
            cache_only_path_resolution(store.realm_id, cache, path_parts, lock_for_update)
        });
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
                        PopulateCacheFromLocalStorageOrServerError::Offline(e) => {
                            ResolvePathError::Offline(e)
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
    realm_id: VlobID,
    cache: &mut super::CurrentViewCache,
    path_parts: &[EntryName],
    lock_for_update: bool,
) -> CacheOnlyPathResolutionOutcome {
    enum StepKind<'a> {
        Root,
        Child {
            manifest: &'a ArcLocalChildManifest,
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
                        StepKind::Root => realm_id,
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
                            manifest: manifest.to_owned(),
                            confinement,
                            maybe_update_lock_guard,
                        };
                    }
                    StepKind::Root => {
                        let manifest =
                            ArcLocalChildManifest::Folder(cache.manifests.root_manifest().clone());
                        let confinement = PathConfinementPoint::NotConfined;
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
            StepKind::Root => (
                cache.manifests.root_manifest(),
                PathConfinementPoint::NotConfined,
            ),
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

        let child_manifest = match cache.manifests.get(&child_id) {
            Some(manifest) => manifest,
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

        // Root-most confinement point shadows child ones if any
        let confinement = match parent_confinement {
            confinement @ PathConfinementPoint::Confined(_) => confinement,
            PathConfinementPoint::NotConfined => {
                if parent_manifest.local_confinement_points.contains(&child_id) {
                    PathConfinementPoint::Confined(parent_id)
                } else {
                    PathConfinementPoint::NotConfined
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
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
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
    // This in practice means our valid data form a direct acyclic graph (DAG) and hence
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
        // However it is easy to mess up the logic that releases what has been locked
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
                if let Some(guard) = self.dst_child_guard.take() {
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

        enum CacheOnlyResolutionOutcome {
            Resolved(ResolvePathForReparenting),
            Need((CacheOnlyPathResolutionOutcome, WhoIsInNeed)),
        }

        let outcome = store.data.with_current_view_cache(|cache| {
            let mut auto_release_guards = AutoReleaseGuards {
                cache,
                src_parent_guard: None,
                src_child_guard: None,
                dst_parent_guard: None,
                dst_child_guard: None,
            };

            // 1) Resolve the destination parent path and lock for update

            let dst_parent_resolution_outcome = cache_only_path_resolution(
                store.realm_id,
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
                    return Ok(CacheOnlyResolutionOutcome::Need((
                        other_outcome,
                        WhoIsInNeed::DstParent,
                    )));
                }
            };

            // 2) Resolve the source parent path and lock for update

            let src_parent_resolution_outcome = cache_only_path_resolution(
                store.realm_id,
                auto_release_guards.cache,
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
                    return Ok(CacheOnlyResolutionOutcome::Need((
                        other_outcome,
                        WhoIsInNeed::SrcParent,
                    )));
                }
            };

            // 3) Resolve the source child path and lock for update

            let src_child_manifest = {
                let src_child_id = match src_parent_manifest.children.get(src_child_name) {
                    Some(src_child_id) => *src_child_id,
                    None => return Err(ResolvePathForReparentingError::SourceNotFound),
                };

                let src_child_manifest =
                    match auto_release_guards.cache.manifests.get(&src_child_id) {
                        Some(manifest) => manifest.to_owned(),
                        // Cache miss !
                        None => {
                            return Ok(CacheOnlyResolutionOutcome::Need((
                                CacheOnlyPathResolutionOutcome::NeedPopulateCache(src_child_id),
                                WhoIsInNeed::SrcChild,
                            )));
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
                        return Ok(CacheOnlyResolutionOutcome::Need((
                            CacheOnlyPathResolutionOutcome::NeedWaitForTakenUpdateLock(need_wait),
                            WhoIsInNeed::SrcChild,
                        )));
                    }
                };
                auto_release_guards.src_child_guard =
                    Some(maybe_update_lock_guard.expect("always present"));

                src_child_manifest
            };

            // 4) Resolve the destination child path and lock for update

            let dst_child_manifest = if let Some(dst_child_name) = dst_child_name {
                let dst_child_id = match dst_parent_manifest.children.get(dst_child_name) {
                    Some(dst_child_id) => *dst_child_id,
                    None => return Err(ResolvePathForReparentingError::DestinationNotFound),
                };

                let dst_child_manifest =
                    match auto_release_guards.cache.manifests.get(&dst_child_id) {
                        Some(manifest) => manifest.to_owned(),
                        // Cache miss !
                        None => {
                            return Ok(CacheOnlyResolutionOutcome::Need((
                                CacheOnlyPathResolutionOutcome::NeedPopulateCache(dst_child_id),
                                WhoIsInNeed::DstChild,
                            )));
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
                        return Ok(CacheOnlyResolutionOutcome::Need((
                            CacheOnlyPathResolutionOutcome::NeedWaitForTakenUpdateLock(need_wait),
                            WhoIsInNeed::DstChild,
                        )));
                    }
                };
                auto_release_guards.dst_child_guard =
                    Some(maybe_update_lock_guard.expect("always present"));

                Some(dst_child_manifest)
            } else {
                None
            };

            // 5) All done, we defuse the auto release system to return its guards

            Ok(CacheOnlyResolutionOutcome::Resolved(
                ResolvePathForReparenting {
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
                },
            ))
        })?;

        let (needed_before_resolution, who_need) = match outcome {
            CacheOnlyResolutionOutcome::Resolved(resolve_path_for_reparenting) => {
                return Ok(resolve_path_for_reparenting)
            }
            CacheOnlyResolutionOutcome::Need((needed_before_resolution, who_need)) => {
                (needed_before_resolution, who_need)
            }
        };

        match needed_before_resolution {
            // We got a cache miss
            CacheOnlyPathResolutionOutcome::NeedPopulateCache(cache_miss_entry_id) => {
                populate_cache_from_local_storage_or_server(store, cache_miss_entry_id)
                    .await
                    .map_err(|err| match err {
                        PopulateCacheFromLocalStorageOrServerError::Offline(e) => {
                            ResolvePathForReparentingError::Offline(e)
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

enum CacheOnlyPathRetrievalOutcome {
    Done {
        entry: RetrievePathFromIDEntry,
        maybe_update_lock_guard: Option<ManifestUpdateLockGuard>,
    },
    NeedPopulateCache(VlobID),
    NeedWaitForTakenUpdateLock(EventListener),
}

fn cache_only_retrieve_path_from_id(
    cache: &mut super::CurrentViewCache,
    entry_id: VlobID,
    lock_for_update: bool,
    last_not_found_during_populate: Option<VlobID>,
) -> CacheOnlyPathRetrievalOutcome {
    // Note it is *vital* to return the update lock guard to the caller given
    // it contains a lock that won't be released on drop !
    macro_rules! maybe_take_update_lock_guard_for_entry {
        () => {
            if lock_for_update {
                match cache.lock_update_manifests.take(entry_id) {
                    ManifestUpdateLockTakeOutcome::Taken(lock) => Some(lock),
                    ManifestUpdateLockTakeOutcome::NeedWait(need_wait) => {
                        return CacheOnlyPathRetrievalOutcome::NeedWaitForTakenUpdateLock(need_wait)
                    }
                }
            } else {
                None
            }
        };
    }

    // Initialize the results
    let mut parts = Vec::new();
    let mut confinement = PathConfinementPoint::NotConfined;

    // Get the root ID
    let root_entry_id = cache.manifests.root_manifest().base.id;

    let entry_manifest = match cache.manifests.get(&entry_id) {
        Some(manifest) => manifest,
        // Cache miss !
        None => {
            // If we already failed to populate this entry, we are done...
            if last_not_found_during_populate == Some(entry_id) {
                return CacheOnlyPathRetrievalOutcome::Done {
                    entry: RetrievePathFromIDEntry::Missing,
                    maybe_update_lock_guard: maybe_take_update_lock_guard_for_entry!(),
                };
            }

            // ...otherwise tell the caller we need this entry to be populated !
            return CacheOnlyPathRetrievalOutcome::NeedPopulateCache(entry_id);
        }
    };

    // Initialize the loop state
    let mut current_entry_id = entry_id;
    let mut current_parent_id = entry_manifest.parent();
    let mut seen: HashSet<VlobID> = HashSet::from_iter([current_entry_id]);
    let mut entry_chain = vec![current_entry_id];

    // Loop until we reach the root
    while current_entry_id != root_entry_id {
        // Get the parent manifest
        let parent_manifest = match cache.manifests.get(&current_parent_id) {
            // Happy case :)
            Some(ArcLocalChildManifest::Folder(manifest)) => manifest,
            // A file cannot be the parent of another entry, the path is broken !
            Some(ArcLocalChildManifest::File(_)) => {
                return CacheOnlyPathRetrievalOutcome::Done {
                    entry: RetrievePathFromIDEntry::Unreachable {
                        manifest: entry_manifest.to_owned(),
                    },
                    maybe_update_lock_guard: maybe_take_update_lock_guard_for_entry!(),
                };
            }
            // Cache miss !
            None => {
                // If we already failed to populate this parent, we are done...
                if last_not_found_during_populate == Some(entry_id) {
                    return CacheOnlyPathRetrievalOutcome::Done {
                        entry: RetrievePathFromIDEntry::Unreachable {
                            manifest: entry_manifest.to_owned(),
                        },
                        maybe_update_lock_guard: maybe_take_update_lock_guard_for_entry!(),
                    };
                }

                // ...otherwise tell the caller we need this parent to be populated !
                return CacheOnlyPathRetrievalOutcome::NeedPopulateCache(current_parent_id);
            }
        };

        // Update the path result
        let child_name = parent_manifest.children.iter().find_map(|(name, id)| {
            if *id == current_entry_id {
                Some(name)
            } else {
                None
            }
        });
        match child_name {
            // Parent and child agree on the parenting relationship
            Some(child_name) => parts.push(child_name.clone()),
            // The path is broken !
            None => {
                return CacheOnlyPathRetrievalOutcome::Done {
                    entry: RetrievePathFromIDEntry::Unreachable {
                        manifest: entry_manifest.to_owned(),
                    },
                    maybe_update_lock_guard: maybe_take_update_lock_guard_for_entry!(),
                };
            }
        }

        // Update the confinement point result
        // Give priority to the root-most confinement point
        confinement = if parent_manifest
            .local_confinement_points
            .contains(&current_entry_id)
        {
            PathConfinementPoint::Confined(current_parent_id)
        } else {
            confinement
        };

        // Update the loop state
        current_entry_id = current_parent_id;
        current_parent_id = parent_manifest.parent;
        entry_chain.push(current_entry_id);

        // Protect against circular paths
        if !seen.insert(current_entry_id) {
            return CacheOnlyPathRetrievalOutcome::Done {
                entry: RetrievePathFromIDEntry::Unreachable {
                    manifest: entry_manifest.to_owned(),
                },
                maybe_update_lock_guard: maybe_take_update_lock_guard_for_entry!(),
            };
        }
    }

    // Reverse the parts to get the path
    parts.reverse();
    entry_chain.reverse();
    CacheOnlyPathRetrievalOutcome::Done {
        entry: RetrievePathFromIDEntry::Reachable {
            manifest: entry_manifest.to_owned(),
            path: FsPath::from_parts(parts),
            confinement_point: confinement,
            entry_chain,
        },
        maybe_update_lock_guard: maybe_take_update_lock_guard_for_entry!(),
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum RetrievePathFromIDError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
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

#[derive(Debug, thiserror::Error)]
pub(crate) enum RetrievePathFromIdAndLockForUpdateError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("Entry is already being updated")]
    WouldBlock,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub(crate) enum RetrievePathFromIDEntry {
    /// The entry is not present in the local storage
    Missing,
    /// The entry is present in the local storage, however its path is broken.
    Unreachable { manifest: ArcLocalChildManifest },
    /// The entry is present in the local storage and has a valid path to reach it.
    Reachable {
        manifest: ArcLocalChildManifest,
        #[allow(unused)]
        path: FsPath,
        confinement_point: PathConfinementPoint,
        // The chain of entry IDs from the root to the provided entry
        // (both the root and the provided entry are included in the chain).
        entry_chain: Vec<VlobID>,
    },
}

/// Get back the local entry, resolve its path, then lock it for update.
///
/// This method as a peculiar behavior:
/// - Any missing manifest needed to resolve the path will be downloaded from the server.
/// - The entry itself will only be fetched from local data.
///
/// This is because this method is expected to be used to synchronize this very
/// entry from/to the server.
pub(crate) async fn retrieve_path_from_id_and_lock_for_update(
    store: &super::WorkspaceStore,
    entry_id: VlobID,
    wait: bool,
) -> Result<
    (RetrievePathFromIDEntry, ManifestUpdateLockGuard),
    RetrievePathFromIdAndLockForUpdateError,
> {
    // In this function, not all cache misses are going to be successfully populated:
    // - If the entry is not present in local obviously.
    // - If a parent in the path has an invalid ID.
    //
    // However event in those cases we need to take the update lock guard for the entry.
    //
    // Hence this `last_not_found_during_populate` variable that gets provided to
    // the cache-only retrieval function so that it knows if it should ask for a
    // populate or just abort and return a missing/unreachable outcome.
    let mut last_not_found_during_populate = None;
    loop {
        // Most of the time we should have each entry in the path already in the cache,
        // so we want to lock the cache once and only release it in the unlikely case
        // we need to fetch from the local storage or server.
        let cache_only_outcome = store.data.with_current_view_cache(|cache| {
            cache_only_retrieve_path_from_id(cache, entry_id, true, last_not_found_during_populate)
        });
        match cache_only_outcome {
            CacheOnlyPathRetrievalOutcome::Done {
                entry,
                maybe_update_lock_guard,
            } => {
                let lock_guard = maybe_update_lock_guard.expect("always present");
                return Ok((entry, lock_guard));
            }

            // We got a cache miss (entry is missing)
            CacheOnlyPathRetrievalOutcome::NeedPopulateCache(cache_miss_entry_id)
                if cache_miss_entry_id == entry_id =>
            {
                match populate_cache_from_local_storage(store, cache_miss_entry_id).await {
                    // Happy case :)
                    Ok(_) => (),
                    // The entry is not in local, the idea here is to retry
                    // `cache_only_retrieve_path_from_id` knowing this information.
                    Err(PopulateCacheFromLocalStorageError::EntryNotFound) => {
                        last_not_found_during_populate = Some(cache_miss_entry_id);
                    }
                    // Other errors
                    Err(PopulateCacheFromLocalStorageError::Stopped) => {
                        return Err(RetrievePathFromIdAndLockForUpdateError::Stopped)
                    }
                    Err(PopulateCacheFromLocalStorageError::Internal(err)) => {
                        return Err(err.context("cannot fetch manifest").into())
                    }
                }
            }

            // We got a cache miss (parents in the path are missing)
            CacheOnlyPathRetrievalOutcome::NeedPopulateCache(cache_miss_entry_id) => {
                match populate_cache_from_local_storage_or_server(store, cache_miss_entry_id).await
                {
                    // Happy case :)
                    Ok(_) => (),
                    // The entry seems to just not exist, the idea here is to retry
                    // `cache_only_retrieve_path_from_id` knowing this information.
                    Err(PopulateCacheFromLocalStorageOrServerError::EntryNotFound) => {
                        last_not_found_during_populate = Some(cache_miss_entry_id);
                    }
                    // Other errors
                    Err(PopulateCacheFromLocalStorageOrServerError::Offline(e)) => {
                        return Err(RetrievePathFromIdAndLockForUpdateError::Offline(e))
                    }
                    Err(PopulateCacheFromLocalStorageOrServerError::Stopped) => {
                        return Err(RetrievePathFromIdAndLockForUpdateError::Stopped)
                    }
                    Err(PopulateCacheFromLocalStorageOrServerError::NoRealmAccess) => {
                        return Err(RetrievePathFromIdAndLockForUpdateError::NoRealmAccess)
                    }
                    Err(PopulateCacheFromLocalStorageOrServerError::InvalidKeysBundle(err)) => {
                        return Err(RetrievePathFromIdAndLockForUpdateError::InvalidKeysBundle(
                            err,
                        ))
                    }
                    Err(PopulateCacheFromLocalStorageOrServerError::InvalidCertificate(err)) => {
                        return Err(RetrievePathFromIdAndLockForUpdateError::InvalidCertificate(
                            err,
                        ))
                    }
                    Err(PopulateCacheFromLocalStorageOrServerError::InvalidManifest(err)) => {
                        return Err(RetrievePathFromIdAndLockForUpdateError::InvalidManifest(
                            err,
                        ))
                    }
                    Err(PopulateCacheFromLocalStorageOrServerError::Internal(err)) => {
                        return Err(err.context("cannot fetch manifest").into())
                    }
                }
            }

            // The path resolution is done, but we must wait for the target entry to be available
            // before being able to lock it for update ourself
            CacheOnlyPathRetrievalOutcome::NeedWaitForTakenUpdateLock(need_wait) => {
                if !wait {
                    return Err(RetrievePathFromIdAndLockForUpdateError::WouldBlock);
                }
                need_wait.await;
            }
        }
    }
}

pub(crate) async fn retrieve_path_from_id(
    store: &super::WorkspaceStore,
    entry_id: VlobID,
) -> Result<RetrievePathFromIDEntry, RetrievePathFromIDError> {
    // In this function, not all cache misses are going to be successfully populated:
    // - If the entry is not present in local obviously.
    // - If a parent in the path has an invalid ID.
    //
    // Hence this `last_not_found_during_populate` variable that gets provided to
    // the cache-only retrieval function so that it knows if it should ask for a
    // populate or just abort and return a missing/unreachable outcome.
    let mut last_not_found_during_populate = None;
    loop {
        // Most of the time we should have each entry in the path already in the cache,
        // so we want to lock the cache once and only release it in the unlikely case
        // we need to fetch from the local storage or server.
        let cache_only_outcome = store.data.with_current_view_cache(|cache| {
            cache_only_retrieve_path_from_id(cache, entry_id, false, last_not_found_during_populate)
        });
        match cache_only_outcome {
            CacheOnlyPathRetrievalOutcome::Done {
                entry,
                maybe_update_lock_guard,
            } => {
                debug_assert!(maybe_update_lock_guard.is_none());
                return Ok(entry);
            }

            // We got a cache miss
            CacheOnlyPathRetrievalOutcome::NeedPopulateCache(cache_miss_entry_id) => {
                match populate_cache_from_local_storage_or_server(store, cache_miss_entry_id).await
                {
                    // Happy case :)
                    Ok(_) => (),
                    // The entry seems to just not exist, the idea here is to retry
                    // `cache_only_retrieve_path_from_id` knowing this information.
                    Err(PopulateCacheFromLocalStorageOrServerError::EntryNotFound) => {
                        last_not_found_during_populate = Some(cache_miss_entry_id);
                    }
                    // Other errors
                    Err(PopulateCacheFromLocalStorageOrServerError::Offline(e)) => {
                        return Err(RetrievePathFromIDError::Offline(e))
                    }
                    Err(PopulateCacheFromLocalStorageOrServerError::Stopped) => {
                        return Err(RetrievePathFromIDError::Stopped)
                    }
                    Err(PopulateCacheFromLocalStorageOrServerError::NoRealmAccess) => {
                        return Err(RetrievePathFromIDError::NoRealmAccess)
                    }
                    Err(PopulateCacheFromLocalStorageOrServerError::InvalidKeysBundle(err)) => {
                        return Err(RetrievePathFromIDError::InvalidKeysBundle(err))
                    }
                    Err(PopulateCacheFromLocalStorageOrServerError::InvalidCertificate(err)) => {
                        return Err(RetrievePathFromIDError::InvalidCertificate(err))
                    }
                    Err(PopulateCacheFromLocalStorageOrServerError::InvalidManifest(err)) => {
                        return Err(RetrievePathFromIDError::InvalidManifest(err))
                    }
                    Err(PopulateCacheFromLocalStorageOrServerError::Internal(err)) => {
                        return Err(err.context("cannot fetch manifest").into())
                    }
                }
            }

            // Unreachable since we are not locking for update
            CacheOnlyPathRetrievalOutcome::NeedWaitForTakenUpdateLock(_) => {
                unreachable!()
            }
        }
    }
}
