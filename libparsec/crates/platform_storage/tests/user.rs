// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use libparsec_platform_storage::user::{user_storage_non_speculative_init, UserStorage};

#[allow(clippy::enum_variant_names)]
enum FetchStrategy {
    No,
    Single,
    Multiple,
}

#[parsec_test(testbed = "minimal")]
#[case::no_fetch(FetchStrategy::No)]
#[case::single_fetch(FetchStrategy::Single)]
#[case::multiple_fetch(FetchStrategy::Multiple)]
async fn testbed_support(#[case] fetch_strategy: FetchStrategy, env: &TestbedEnv) {
    let mut expected_version: VersionInt = 0;

    env.customize(|builder| {
        builder.new_user_realm("alice");

        if matches!(fetch_strategy, FetchStrategy::No) {
            builder.user_storage_local_update("alice@dev1");
        } else {
            builder.create_or_update_user_manifest_vlob("alice");
            expected_version = 1;
            builder.user_storage_fetch_user_vlob("alice@dev1");

            if matches!(fetch_strategy, FetchStrategy::Multiple) {
                builder.create_or_update_user_manifest_vlob("alice");
                expected_version = 2;
                builder.user_storage_fetch_user_vlob("alice@dev1");
            }
        }

        // Stuff the our storage is not aware of
        let actual_version = builder
            .create_or_update_user_manifest_vlob("alice")
            .map(|e| e.manifest.version);

        // Sanity check to ensure additional (and to be ignored) manifest have been added
        p_assert_ne!(expected_version, actual_version);
    });

    let alice = env.local_device("alice@dev1");

    let user_storage = UserStorage::start(&env.discriminant_dir, alice.clone())
        .await
        .unwrap();

    p_assert_eq!(
        user_storage.get_realm_checkpoint().await.unwrap(),
        expected_version as IndexInt
    );

    let user_manifest = user_storage.get_user_manifest();
    p_assert_eq!(user_manifest.base.version, expected_version);
    let expected_need_sync = match fetch_strategy {
        FetchStrategy::No => true,
        FetchStrategy::Single | FetchStrategy::Multiple => false,
    };
    p_assert_eq!(user_manifest.need_sync, expected_need_sync);
}

#[parsec_test(testbed = "minimal")]
async fn operations(timestamp: DateTime, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

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
    let (updater, current_user_manifest) = user_storage.for_update().await;
    p_assert_eq!(*current_user_manifest, expected);
    updater
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
    let alice = env.local_device("alice@dev1");

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
