// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// This module provide helpers that are used for testing purpose.
// To simplify the writing of those helpers, we use the same rule than when writing tests.
#![allow(clippy::unwrap_used)]

use std::{any::Any, ops::Deref, path::Path, sync::Arc};

use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_testbed::{
    test_get_testbed, test_get_testbed_component_store, TestbedEnv, TestbedEvent,
};
use libparsec_types::prelude::*;

use crate::{
    certificates::CertificatesStorage,
    user::UserStorage,
    workspace::{UpdateManifestData, WorkspaceStorage},
};

const STORE_ENTRY_KEY: &str = "platform_storage";

enum StorageKind {
    Certificates,
    User,
    Workspace(VlobID),
}

struct ComponentStore {
    populated: AsyncMutex<Vec<(DeviceID, StorageKind)>>,
}

fn store_factory(_env: &TestbedEnv) -> Arc<dyn Any + Send + Sync> {
    Arc::new(ComponentStore {
        populated: AsyncMutex::new(vec![]),
    })
}

#[allow(unused)]
pub(crate) async fn maybe_populate_certificate_storage(data_base_dir: &Path, device: &LocalDevice) {
    let disabled = std::env::var("TESTBED_DISABLE_POPULATE_CERTIFICATE_STORAGE").is_ok();
    if disabled {
        return;
    }

    if let Some(store) = test_get_testbed_component_store::<ComponentStore>(
        data_base_dir,
        STORE_ENTRY_KEY,
        store_factory,
    ) {
        let mut guard = store.populated.lock().await;
        let already_populated = guard.iter().any(|(candidate, kind)| {
            matches!(kind, StorageKind::Certificates) && *candidate == device.device_id
        });

        if !already_populated {
            let env = test_get_testbed(data_base_dir).expect("Testbed existence already checked");

            // 1) Do we need to be initialized ? and by fetching up what certificate index ?
            let up_to = env.template.events.iter().rev().find_map(|e| match e {
                TestbedEvent::CertificatesStorageFetchCertificates(x)
                    if x.device == device.device_id =>
                {
                    Some(x.up_to)
                }
                _ => None,
            });

            // 2) Actually do the initialization
            if let Some(up_to) = up_to {
                let need_redacted = matches!(
                    env.template.user_profile_at(device.user_id, Some(up_to)),
                    UserProfile::Outsider
                );

                let mut storage = CertificatesStorage::no_populate_start(data_base_dir, device)
                    .await
                    .unwrap();

                let certifs = env.template.certificates().take_while(|c| {
                    let certif_timestamp = match &c.certificate {
                        AnyArcCertificate::User(c) => c.timestamp,
                        AnyArcCertificate::Device(c) => c.timestamp,
                        AnyArcCertificate::UserUpdate(c) => c.timestamp,
                        AnyArcCertificate::RevokedUser(c) => c.timestamp,
                        AnyArcCertificate::RealmRole(c) => c.timestamp,
                        AnyArcCertificate::RealmName(c) => c.timestamp,
                        AnyArcCertificate::RealmArchiving(c) => c.timestamp,
                        AnyArcCertificate::RealmKeyRotation(c) => c.timestamp,
                        AnyArcCertificate::ShamirRecoveryBrief(c) => c.timestamp,
                        AnyArcCertificate::ShamirRecoveryShare(c) => c.timestamp,
                        AnyArcCertificate::ShamirRecoveryDeletion(c) => c.timestamp,
                        AnyArcCertificate::SequesterAuthority(c) => c.timestamp,
                        AnyArcCertificate::SequesterService(c) => c.timestamp,
                        AnyArcCertificate::SequesterRevokedService(c) => c.timestamp,
                    };
                    certif_timestamp <= up_to
                });

                storage
                    .for_update(async |mut updater| {
                        for (offset, certif) in certifs.enumerate() {
                            let signed = if need_redacted {
                                &certif.signed_redacted
                            } else {
                                &certif.signed
                            };
                            let encrypted = device.local_symkey.encrypt(signed);
                            match &certif.certificate {
                                AnyArcCertificate::User(certif) => updater
                                    .add_certificate(certif.deref(), encrypted)
                                    .await
                                    .unwrap(),
                                AnyArcCertificate::Device(certif) => updater
                                    .add_certificate(certif.deref(), encrypted)
                                    .await
                                    .unwrap(),
                                AnyArcCertificate::UserUpdate(certif) => updater
                                    .add_certificate(certif.deref(), encrypted)
                                    .await
                                    .unwrap(),
                                AnyArcCertificate::RevokedUser(certif) => updater
                                    .add_certificate(certif.deref(), encrypted)
                                    .await
                                    .unwrap(),

                                // In theory we should skip the certificates related to realm we are not part of,
                                // but in practice it has no impact since we don't have any operation that crawls
                                // all the realm in the certificate storage (i.e. we always provide a realm ID when
                                // querying for realm-related certificates).
                                AnyArcCertificate::RealmRole(certif) => updater
                                    .add_certificate(certif.deref(), encrypted)
                                    .await
                                    .unwrap(),
                                AnyArcCertificate::RealmName(certif) => updater
                                    .add_certificate(certif.deref(), encrypted)
                                    .await
                                    .unwrap(),
                                AnyArcCertificate::RealmKeyRotation(certif) => updater
                                    .add_certificate(certif.deref(), encrypted)
                                    .await
                                    .unwrap(),
                                AnyArcCertificate::RealmArchiving(certif) => updater
                                    .add_certificate(certif.deref(), encrypted)
                                    .await
                                    .unwrap(),

                                // Just like for realm, we should in theory skip the shamir certificates not meant to us.
                                // We do have operations that crawl the whole shamir certificates, but they are aware of
                                // this shortcoming and deal with them accordingly.
                                AnyArcCertificate::ShamirRecoveryBrief(certif) => updater
                                    .add_certificate(certif.deref(), encrypted)
                                    .await
                                    .unwrap(),
                                AnyArcCertificate::ShamirRecoveryShare(certif) => updater
                                    .add_certificate(certif.deref(), encrypted)
                                    .await
                                    .unwrap(),
                                AnyArcCertificate::ShamirRecoveryDeletion(certif) => updater
                                    .add_certificate(certif.deref(), encrypted)
                                    .await
                                    .unwrap(),

                                AnyArcCertificate::SequesterAuthority(certif) => updater
                                    .add_certificate(certif.deref(), encrypted)
                                    .await
                                    .unwrap(),
                                AnyArcCertificate::SequesterService(certif) => updater
                                    .add_certificate(certif.deref(), encrypted)
                                    .await
                                    .unwrap(),
                                AnyArcCertificate::SequesterRevokedService(certif) => updater
                                    .add_certificate(certif.deref(), encrypted)
                                    .await
                                    .unwrap(),
                            }
                        }

                        updater.commit().await.unwrap();
                    })
                    .await
                    .unwrap();

                storage.stop().await;
            }

            // Mark as populated
            guard.push((device.device_id, StorageKind::Certificates));
        }
    }
}

#[allow(unused)]
pub(crate) async fn maybe_populate_user_storage(data_base_dir: &Path, device: &LocalDevice) {
    if let Some(store) = test_get_testbed_component_store::<ComponentStore>(
        data_base_dir,
        STORE_ENTRY_KEY,
        store_factory,
    ) {
        let mut guard = store.populated.lock().await;
        let already_populated = guard.iter().any(|(candidate, kind)| {
            matches!(kind, StorageKind::User) && *candidate == device.device_id
        });

        if !already_populated {
            let env = test_get_testbed(data_base_dir).expect("Testbed existence already checked");

            // Only start the storage if we need to do some initialization work
            let mut lazy_storage: Option<UserStorage> = None;

            for event in &env.template.events {
                let (maybe_manifest, maybe_checkpoint) = match event {
                    TestbedEvent::UserStorageFetchUserVlob(x) if x.device == device.device_id => {
                        (Some(x.local_manifest.clone()), None)
                    }
                    TestbedEvent::UserStorageLocalUpdate(x) if x.device == device.device_id => {
                        (Some(x.local_manifest.clone()), None)
                    }
                    TestbedEvent::UserStorageFetchRealmCheckpoint(x)
                        if x.device == device.device_id =>
                    {
                        (None, Some((x.checkpoint, x.remote_user_manifest_version)))
                    }
                    _ => continue,
                };

                if lazy_storage.is_none() {
                    lazy_storage = Some(
                        UserStorage::no_populate_start(data_base_dir, device)
                            .await
                            .unwrap(),
                    );
                }

                if let Some((checkpoint, remote_user_manifest_version)) = maybe_checkpoint {
                    lazy_storage
                        .as_mut()
                        .unwrap()
                        .update_realm_checkpoint(checkpoint, remote_user_manifest_version)
                        .await
                        .unwrap();
                }

                if let Some(manifest) = maybe_manifest {
                    let encrypted = manifest.dump_and_encrypt(&device.local_symkey);
                    lazy_storage
                        .as_mut()
                        .unwrap()
                        .update_user_manifest(&encrypted, manifest.need_sync, manifest.base.version)
                        .await
                        .unwrap();
                }
            }

            if let Some(storage) = lazy_storage {
                storage.stop().await;
            }

            // Mark as populated
            guard.push((device.device_id, StorageKind::User));
        }
    }
}

#[allow(unused)]
pub(crate) async fn maybe_populate_workspace_storage(
    data_base_dir: &Path,
    device: &LocalDevice,
    realm_id: VlobID,
) {
    if let Some(store) = test_get_testbed_component_store::<ComponentStore>(
        data_base_dir,
        STORE_ENTRY_KEY,
        store_factory,
    ) {
        let mut guard = store.populated.lock().await;
        let already_populated = guard.iter().any(|(candidate, kind)| {
            matches!(kind, StorageKind::Workspace(candidate_realm_id) if *candidate_realm_id == realm_id) && *candidate == device.device_id
        });

        if !already_populated {
            let env = test_get_testbed(data_base_dir).expect("Testbed existence already checked");

            // Only start the storage if we need to do some initialization work
            let mut lazy_storage: Option<WorkspaceStorage> = None;
            macro_rules! lazy_storage {
                () => {
                    match &mut lazy_storage {
                        Some(storage) => storage,
                        None => {
                            lazy_storage = Some(
                                WorkspaceStorage::no_populate_start(
                                    data_base_dir,
                                    device,
                                    realm_id,
                                    u64::MAX,
                                )
                                .await
                                .unwrap(),
                            );
                            lazy_storage.as_mut().unwrap()
                        }
                    }
                };
            }

            for event in &env.template.events {
                match event {
                    TestbedEvent::WorkspaceDataStorageFetchFileVlob(x)
                        if x.realm == realm_id && x.device == device.device_id =>
                    {
                        lazy_storage!()
                            .update_manifest(&UpdateManifestData {
                                entry_id: x.local_manifest.base.id,
                                need_sync: x.local_manifest.need_sync,
                                base_version: x.local_manifest.base.version,
                                encrypted: x.local_manifest.dump_and_encrypt(&device.local_symkey),
                            })
                            .await
                            .unwrap();
                    }
                    TestbedEvent::WorkspaceDataStorageFetchFolderVlob(x)
                        if x.realm == realm_id && x.device == device.device_id =>
                    {
                        lazy_storage!()
                            .update_manifest(&UpdateManifestData {
                                entry_id: x.local_manifest.base.id,
                                need_sync: x.local_manifest.need_sync,
                                base_version: x.local_manifest.base.version,
                                encrypted: x.local_manifest.dump_and_encrypt(&device.local_symkey),
                            })
                            .await
                            .unwrap();
                    }
                    TestbedEvent::WorkspaceCacheStorageFetchBlock(x)
                        if x.realm == realm_id && x.device == device.device_id =>
                    {
                        let encrypted = device.local_symkey.encrypt(&x.cleartext);
                        lazy_storage!()
                            .set_block(x.block_id, &encrypted, device.now())
                            .await
                            .unwrap();
                    }
                    TestbedEvent::WorkspaceDataStorageFetchRealmCheckpoint(x)
                        if x.realm == realm_id && x.device == device.device_id =>
                    {
                        lazy_storage!()
                            .update_realm_checkpoint(x.checkpoint, &x.changed_vlobs)
                            .await
                            .unwrap();
                    }
                    TestbedEvent::WorkspaceDataStorageLocalFolderManifestCreateOrUpdate(x)
                        if x.realm == realm_id && x.device == device.device_id =>
                    {
                        lazy_storage!()
                            .update_manifest(&UpdateManifestData {
                                entry_id: x.local_manifest.base.id,
                                need_sync: x.local_manifest.need_sync,
                                base_version: x.local_manifest.base.version,
                                encrypted: x.local_manifest.dump_and_encrypt(&device.local_symkey),
                            })
                            .await
                            .unwrap();
                    }
                    TestbedEvent::WorkspaceDataStorageLocalFileManifestCreateOrUpdate(x)
                        if x.realm == realm_id && x.device == device.device_id =>
                    {
                        lazy_storage!()
                            .update_manifest(&UpdateManifestData {
                                entry_id: x.local_manifest.base.id,
                                need_sync: x.local_manifest.need_sync,
                                base_version: x.local_manifest.base.version,
                                encrypted: x.local_manifest.dump_and_encrypt(&device.local_symkey),
                            })
                            .await
                            .unwrap();
                    }
                    TestbedEvent::WorkspaceDataStorageChunkCreate(x)
                        if x.realm == realm_id && x.device == device.device_id =>
                    {
                        let encrypted = device.local_symkey.encrypt(&x.chunk);
                        lazy_storage!()
                            .set_chunk(x.chunk_id, &encrypted)
                            .await
                            .unwrap();
                    }
                    _ => continue,
                };
            }

            if let Some(storage) = lazy_storage {
                storage.stop().await;
            }

            // Mark as populated
            guard.push((device.device_id, StorageKind::Workspace(realm_id)));
        }
    }
}
