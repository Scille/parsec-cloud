// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use crate::workspace::store::resolve_path::ResolvePathForReparentingError;
use libparsec_platform_storage::workspace::UpdateManifestData;

use super::per_manifest_update_lock::ManifestUpdateLockGuard;

pub(super) type UpdateManifestsForReparentingError = super::WorkspaceStoreOperationError;
pub(crate) type ForUpdateReparentingError = ResolvePathForReparentingError;

pub(super) async fn resolve_path_for_update_reparenting<'a>(
    store: &'a super::WorkspaceStore,
    src_parent_path: &FsPath,
    src_child_name: &EntryName,
    dst_parent_path: &FsPath,
    // In most reparenting we don't even care if the destination exists given
    // it is simply overwritten by the source.
    // However this is not the case when doing an exchange between source and
    // destination, given in this case the destination's `parent` field must
    // also be modified.
    dst_child_name: Option<&EntryName>,
) -> Result<ReparentingUpdater<'a>, ForUpdateReparentingError> {
    let resolution = super::resolve_path::resolve_path_for_reparenting(
        store,
        src_parent_path,
        src_child_name,
        dst_parent_path,
        dst_child_name,
    )
    .await?;

    Ok(ReparentingUpdater {
        src_parent_manifest: resolution.src_parent_manifest,
        src_child_manifest: resolution.src_child_manifest,
        dst_parent_manifest: resolution.dst_parent_manifest,
        dst_child_manifest: resolution.dst_child_manifest,
        guards: ReparentingUpdaterGuards {
            store,
            update_guards: Some((
                resolution.src_parent_update_lock_guard,
                resolution.src_child_update_lock_guard,
                resolution.dst_parent_update_lock_guard,
                resolution.dst_child_update_lock_guard,
            )),
        },
    })
}

pub(crate) struct ReparentingUpdater<'a> {
    pub src_parent_manifest: Arc<LocalFolderManifest>,
    pub src_child_manifest: ArcLocalChildManifest,
    pub dst_parent_manifest: Arc<LocalFolderManifest>,
    pub dst_child_manifest: Option<ArcLocalChildManifest>,
    guards: ReparentingUpdaterGuards<'a>,
}

/// Destructuring and `impl Drop` are not compatible, hence this structure that is
/// responsible to drop the update guards, while `ReparentingUpdater` can still be
/// destructured (which is done in `update_manifests` to populate the cache with
/// it manifests).
struct ReparentingUpdaterGuards<'a> {
    store: &'a super::WorkspaceStore,
    update_guards: Option<(
        ManifestUpdateLockGuard,
        ManifestUpdateLockGuard,
        ManifestUpdateLockGuard,
        Option<ManifestUpdateLockGuard>,
    )>,
}

impl Drop for ReparentingUpdaterGuards<'_> {
    fn drop(&mut self) {
        if let Some((guard1, guard2, guard3, guard4)) = self.update_guards.take() {
            let mut cache = self
                .store
                .current_view_cache
                .lock()
                .expect("Mutex is poisoned");
            cache.lock_update_manifests.release(guard1);
            cache.lock_update_manifests.release(guard2);
            cache.lock_update_manifests.release(guard3);
            if let Some(guard4) = guard4 {
                cache.lock_update_manifests.release(guard4);
            }
        }
    }
}

impl ReparentingUpdater<'_> {
    pub(crate) async fn update_manifests(self) -> Result<(), UpdateManifestsForReparentingError> {
        // Note `guards` is only dropped at the end of the function, hence protecting
        // it against any concurrent operation.
        let ReparentingUpdater {
            src_parent_manifest,
            src_child_manifest,
            dst_parent_manifest,
            dst_child_manifest,
            guards,
        } = self;
        let store = guards.store;

        let mut storage_guard = store.storage.lock().await;
        let storage = storage_guard
            .as_mut()
            .ok_or_else(|| UpdateManifestsForReparentingError::Stopped)?;

        // 1) Insert in database

        let src_parent_update_data = UpdateManifestData {
            entry_id: src_parent_manifest.base.id,
            base_version: src_parent_manifest.base.version,
            need_sync: src_parent_manifest.need_sync,
            encrypted: src_parent_manifest.dump_and_encrypt(&store.device.local_symkey),
        };
        let src_child_update_data = match &src_child_manifest {
            ArcLocalChildManifest::File(src_child_manifest) => UpdateManifestData {
                entry_id: src_child_manifest.base.id,
                base_version: src_child_manifest.base.version,
                need_sync: src_child_manifest.need_sync,
                encrypted: src_child_manifest.dump_and_encrypt(&store.device.local_symkey),
            },
            ArcLocalChildManifest::Folder(src_child_manifest) => UpdateManifestData {
                entry_id: src_child_manifest.base.id,
                base_version: src_child_manifest.base.version,
                need_sync: src_child_manifest.need_sync,
                encrypted: src_child_manifest.dump_and_encrypt(&store.device.local_symkey),
            },
        };
        let dst_parent_update_data = UpdateManifestData {
            entry_id: dst_parent_manifest.base.id,
            base_version: dst_parent_manifest.base.version,
            need_sync: dst_parent_manifest.need_sync,
            encrypted: dst_parent_manifest.dump_and_encrypt(&store.device.local_symkey),
        };

        let src_child_id = src_child_update_data.entry_id;

        match &dst_child_manifest {
            // 3 manifests to update
            None => {
                storage
                    .update_manifests(
                        [
                            src_parent_update_data,
                            src_child_update_data,
                            dst_parent_update_data,
                        ]
                        .into_iter(),
                    )
                    .await?;
            }

            // 4 manifests to update (including destination child manifest)
            Some(dst_child_manifest) => {
                let dst_child_update_data = match &dst_child_manifest {
                    ArcLocalChildManifest::File(dst_child_manifest) => UpdateManifestData {
                        entry_id: dst_child_manifest.base.id,
                        base_version: dst_child_manifest.base.version,
                        need_sync: dst_child_manifest.need_sync,
                        encrypted: dst_child_manifest.dump_and_encrypt(&store.device.local_symkey),
                    },
                    ArcLocalChildManifest::Folder(dst_child_manifest) => UpdateManifestData {
                        entry_id: dst_child_manifest.base.id,
                        base_version: dst_child_manifest.base.version,
                        need_sync: dst_child_manifest.need_sync,
                        encrypted: dst_child_manifest.dump_and_encrypt(&store.device.local_symkey),
                    },
                };

                storage
                    .update_manifests(
                        [
                            src_parent_update_data,
                            src_child_update_data,
                            dst_parent_update_data,
                            dst_child_update_data,
                        ]
                        .into_iter(),
                    )
                    .await?;
            }
        }

        // 2) Update the cache

        let mut cache = store.current_view_cache.lock().expect("Mutex is poisoned");

        if src_parent_manifest.base.id == store.realm_id {
            cache.root_manifest = src_parent_manifest;
        } else {
            cache.child_manifests.insert(
                src_parent_manifest.base.id,
                ArcLocalChildManifest::Folder(src_parent_manifest),
            );
        }
        if dst_parent_manifest.base.id == store.realm_id {
            cache.root_manifest = dst_parent_manifest;
        } else {
            cache.child_manifests.insert(
                dst_parent_manifest.base.id,
                ArcLocalChildManifest::Folder(dst_parent_manifest),
            );
        }
        cache
            .child_manifests
            .insert(src_child_id, src_child_manifest);
        if let Some(dst_child_manifest) = dst_child_manifest {
            cache
                .child_manifests
                .insert(dst_child_manifest.id(), dst_child_manifest);
        }

        Ok(())
    }
}
