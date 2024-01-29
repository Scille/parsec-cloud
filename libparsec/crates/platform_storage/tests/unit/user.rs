// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::{user_storage_non_speculative_init, UserStorage};

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
            builder.user_storage_fetch_realm_checkpoint("alice@dev1");
            builder.user_storage_fetch_user_vlob("alice@dev1");

            if matches!(fetch_strategy, FetchStrategy::Multiple) {
                builder.create_or_update_user_manifest_vlob("alice");
                expected_version = 2;
                builder.user_storage_fetch_realm_checkpoint("alice@dev1");
                builder.user_storage_fetch_user_vlob("alice@dev1");
            }
        }

        // Stuff our storage is not aware of
        let actual_version = builder
            .create_or_update_user_manifest_vlob("alice")
            .map(|e| e.manifest.version);

        // Sanity check to ensure additional (and to be ignored) manifest have been added
        p_assert_ne!(expected_version, actual_version);
    });

    let alice = env.local_device("alice@dev1");

    let mut user_storage = UserStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    p_assert_eq!(
        user_storage.get_realm_checkpoint().await.unwrap(),
        expected_version as IndexInt
    );

    let encrypted = user_storage.get_user_manifest().await.unwrap().unwrap();
    let user_manifest =
        LocalUserManifest::decrypt_and_load(&encrypted, &alice.local_symkey).unwrap();
    p_assert_eq!(user_manifest.base.version, expected_version);
    let expected_need_sync = match fetch_strategy {
        FetchStrategy::No => true,
        FetchStrategy::Single | FetchStrategy::Multiple => false,
    };
    p_assert_eq!(user_manifest.need_sync, expected_need_sync);
}

#[parsec_test(testbed = "minimal")]
async fn user_manifest(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let mut user_storage = UserStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    // 1) Storage starts empty
    p_assert_eq!(user_storage.get_user_manifest().await.unwrap(), None);

    // 2) Update

    user_storage
        .update_user_manifest(b"<user_manifest_v1>", false, 1)
        .await
        .unwrap();
    p_assert_eq!(
        user_storage.get_user_manifest().await.unwrap().unwrap(),
        b"<user_manifest_v1>"
    );

    // 3) Re-starting the database and check data are still there

    user_storage.stop().await.unwrap();
    let mut user_storage = UserStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    p_assert_eq!(
        user_storage.get_user_manifest().await.unwrap().unwrap(),
        b"<user_manifest_v1>"
    );
}

#[parsec_test(testbed = "minimal")]
async fn checkpoint(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let mut user_storage = UserStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    // 1) Initial value

    p_assert_eq!(user_storage.get_realm_checkpoint().await.unwrap(), 0);

    // 2) Update

    user_storage.update_realm_checkpoint(1, None).await.unwrap();
    p_assert_eq!(user_storage.get_realm_checkpoint().await.unwrap(), 1);

    // 3) Re-starting the database and check data are still there

    user_storage.stop().await.unwrap();
    let mut user_storage = UserStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    p_assert_eq!(user_storage.get_realm_checkpoint().await.unwrap(), 1);
}

#[parsec_test(testbed = "minimal")]
async fn non_speculative_init(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    // 1) Initialize the database

    user_storage_non_speculative_init(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    // 2) Check the database content

    let mut user_storage = UserStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    p_assert_eq!(user_storage.get_realm_checkpoint().await.unwrap(), 0);

    let encrypted = user_storage.get_user_manifest().await.unwrap().unwrap();
    let user_manifest =
        LocalUserManifest::decrypt_and_load(&encrypted, &alice.local_symkey).unwrap();

    let expected = LocalUserManifest {
        base: UserManifest {
            author: alice.device_id.clone(),
            timestamp: user_manifest.updated,
            id: alice.user_realm_id,
            version: 0,
            created: user_manifest.updated,
            updated: user_manifest.updated,
            workspaces_legacy_initial_info: vec![],
        },
        need_sync: true,
        updated: user_manifest.updated,
        local_workspaces: vec![],
        speculative: false,
    };
    p_assert_eq!(user_manifest, expected);
}

#[cfg(not(target_arch = "wasm32"))]
#[parsec_test]
async fn bad_start(tmp_path: TmpPath, alice: &Device) {
    // 1) Bad path

    let not_a_dir_path = tmp_path.join("foo.txt");
    std::fs::File::create(&not_a_dir_path).unwrap();

    p_assert_matches!(
        UserStorage::start(&not_a_dir_path, &alice.local_device()).await,
        Err(_)
    );

    // TODO: create a valid database, then modify the user manifest's vlob to
    // turn it into something invalid:
    // - invalid schema
    // - invalid encryption

    // TODO: modify the database to make its schema invalid

    // TODO: drop the database so that it exists but it is empty, this shouldn't cause any issue

    // TODO: remove user manifest's vlob from the database, this shouldn't cause any issue
}
