// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// This module provide helpers that are used for testing purpose.
// To simplify the writing of those helpers, we use the same rule for when writing tests.
#![allow(clippy::unwrap_used)]

use std::{any::Any, path::Path, sync::Arc};

use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_testbed::{
    test_get_testbed, test_get_testbed_component_store, TestbedEnv, TestbedEvent,
};
use libparsec_types::prelude::*;

use crate::{certificates::CertificatesStorage, user::UserStorage};

const STORE_ENTRY_KEY: &str = "platform_storage";

enum StorageKind {
    Certificates,
    #[allow(dead_code)]
    User,
    #[allow(dead_code)]
    Workspace(RealmID),
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
            let up_to_index = env.template.events.iter().rev().find_map(|e| match e {
                TestbedEvent::CertificatesStorageFetchCertificates(x)
                    if x.device == device.device_id =>
                {
                    Some(x.up_to_index)
                }
                _ => None,
            });

            // 2) Actually do the initialization
            if let Some(up_to_index) = up_to_index {
                let need_redacted = matches!(
                    env.template.user_profile_at(device.user_id(), up_to_index),
                    UserProfile::Outsider
                );

                let storage = CertificatesStorage::no_populate_start(data_base_dir, device)
                    .await
                    .unwrap();

                for certif in env.template.certificates().take(up_to_index as usize) {
                    let to_encrypt = if need_redacted {
                        &certif.raw_redacted
                    } else {
                        &certif.raw
                    };
                    match certif.certificate {
                        AnyArcCertificate::User(c) => {
                            let encrypted = device.local_symkey.encrypt(to_encrypt);
                            storage
                                .add_user_certificate(
                                    certif.certificate_index,
                                    c.timestamp,
                                    c.user_id.clone(),
                                    encrypted,
                                )
                                .await
                                .unwrap();
                        }
                        AnyArcCertificate::RevokedUser(c) => {
                            let encrypted = device.local_symkey.encrypt(to_encrypt);
                            storage
                                .add_revoked_user_certificate(
                                    certif.certificate_index,
                                    c.timestamp,
                                    c.user_id.clone(),
                                    encrypted,
                                )
                                .await
                                .unwrap();
                        }
                        AnyArcCertificate::UserUpdate(c) => {
                            let encrypted = device.local_symkey.encrypt(to_encrypt);
                            storage
                                .add_user_update_certificate(
                                    certif.certificate_index,
                                    c.timestamp,
                                    c.user_id.clone(),
                                    encrypted,
                                )
                                .await
                                .unwrap();
                        }
                        AnyArcCertificate::Device(c) => {
                            let encrypted = device.local_symkey.encrypt(to_encrypt);
                            storage
                                .add_device_certificate(
                                    certif.certificate_index,
                                    c.timestamp,
                                    c.device_id.clone(),
                                    encrypted,
                                )
                                .await
                                .unwrap();
                        }
                        AnyArcCertificate::RealmRole(c) => {
                            let encrypted = device.local_symkey.encrypt(to_encrypt);
                            storage
                                .add_realm_role_certificate(
                                    certif.certificate_index,
                                    c.timestamp,
                                    c.realm_id,
                                    c.user_id.clone(),
                                    encrypted,
                                )
                                .await
                                .unwrap();
                        }
                        AnyArcCertificate::SequesterAuthority(c) => {
                            let encrypted = device.local_symkey.encrypt(to_encrypt);
                            storage
                                .add_sequester_authority_certificate(
                                    certif.certificate_index,
                                    c.timestamp,
                                    encrypted,
                                )
                                .await
                                .unwrap();
                        }
                        AnyArcCertificate::SequesterService(c) => {
                            let encrypted = device.local_symkey.encrypt(to_encrypt);
                            storage
                                .add_sequester_service_certificate(
                                    certif.certificate_index,
                                    c.timestamp,
                                    c.service_id,
                                    encrypted,
                                )
                                .await
                                .unwrap();
                        }
                    };
                }

                storage.stop().await;
            }

            // Mark as populated
            guard.push((device.device_id.clone(), StorageKind::Certificates));
        }
    }
}

#[allow(unused)]
pub(crate) async fn maybe_populate_user_storage(data_base_dir: &Path, device: Arc<LocalDevice>) {
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

            let mut lazy_storage: Option<UserStorage> = None;

            for event in &env.template.events {
                let (manifest, maybe_checkpoint) = match event {
                    TestbedEvent::UserStorageFetchUserVlob(x) if x.device == device.device_id => {
                        (x.local_user_manifest.clone(), Some(x.realm_checkpoint))
                    }
                    TestbedEvent::UserStorageLocalUpdate(x) if x.device == device.device_id => {
                        (x.local_user_manifest.clone(), None)
                    }
                    _ => continue,
                };

                if lazy_storage.is_none() {
                    lazy_storage = Some(
                        UserStorage::no_populate_start(data_base_dir, device.clone())
                            .await
                            .unwrap(),
                    );
                }

                if let Some(checkpoint) = maybe_checkpoint {
                    lazy_storage
                        .as_ref()
                        .unwrap()
                        .update_realm_checkpoint(checkpoint, Some(manifest.base.version))
                        .await
                        .unwrap();
                }

                lazy_storage
                    .as_ref()
                    .unwrap()
                    .for_update()
                    .await
                    .0
                    .set_user_manifest(manifest)
                    .await
                    .unwrap();
            }

            if let Some(storage) = lazy_storage {
                storage.stop().await;
            }

            // Mark as populated
            guard.push((device.device_id.clone(), StorageKind::User));
        }
    }
}
