// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use libparsec_platform_storage2::{user_storage_non_speculative_init, UserStorage};

#[parsec_test(testbed = "minimal")]
async fn operations(timestamp: DateTime, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1".parse().unwrap());

    let user_storage = UserStorage::start(&env.discriminant_dir, alice.clone())
        .await
        .unwrap();

    // See unit tests for bad start

    // 1) realm checkpoint

    p_assert_eq!(user_storage.get_realm_checkpoint().await.unwrap(), 0);
    user_storage.update_realm_checkpoint(1, None).await.unwrap();
    p_assert_eq!(user_storage.get_realm_checkpoint().await.unwrap(), 1);

    // 2) user manifest

    let initial_user_manifest = user_storage.get_user_manifest();
    let expected = LocalUserManifest {
        base: UserManifest {
            author: alice.device_id.clone(),
            timestamp: initial_user_manifest.updated,
            id: alice.user_manifest_id,
            version: 0,
            created: initial_user_manifest.updated,
            updated: initial_user_manifest.updated,
            last_processed_message: 0,
            workspaces: vec![],
        },
        need_sync: true,
        updated: initial_user_manifest.updated,
        last_processed_message: 0,
        workspaces: vec![],
        speculative: true,
    };
    p_assert_eq!(*initial_user_manifest, expected);

    let new_user_manifest = Arc::new(LocalUserManifest {
        base: UserManifest {
            author: alice.device_id.clone(),
            timestamp,
            id: alice.user_manifest_id,
            version: 1,
            created: timestamp,
            updated: timestamp,
            last_processed_message: 1,
            workspaces: vec![],
        },
        need_sync: false,
        updated: timestamp,
        last_processed_message: 0,
        workspaces: vec![],
        speculative: false,
    });
    user_storage
        .set_user_manifest(new_user_manifest.clone())
        .await
        .unwrap();

    p_assert_eq!(user_storage.get_user_manifest(), new_user_manifest);

    // 3) Re-starting the database and check data are still there

    user_storage.stop().await;
    let user_storage = UserStorage::start(&env.discriminant_dir, alice.clone())
        .await
        .unwrap();

    p_assert_eq!(user_storage.get_realm_checkpoint().await.unwrap(), 1);
    p_assert_eq!(user_storage.get_user_manifest(), new_user_manifest);
}

#[parsec_test(testbed = "minimal")]
async fn non_speculative_init(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1".parse().unwrap());

    // 1) Initialize the database

    user_storage_non_speculative_init(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    // See unit tests for bad init

    // 2) Check the database content

    let user_storage = UserStorage::start(&env.discriminant_dir, alice.clone())
        .await
        .unwrap();

    p_assert_eq!(user_storage.get_realm_checkpoint().await.unwrap(), 0);

    let user_manifest = user_storage.get_user_manifest();
    let expected = LocalUserManifest {
        base: UserManifest {
            author: alice.device_id.clone(),
            timestamp: user_manifest.updated,
            id: alice.user_manifest_id,
            version: 0,
            created: user_manifest.updated,
            updated: user_manifest.updated,
            last_processed_message: 0,
            workspaces: vec![],
        },
        need_sync: true,
        updated: user_manifest.updated,
        last_processed_message: 0,
        workspaces: vec![],
        speculative: false,
    };
    p_assert_eq!(*user_manifest, expected);
}
