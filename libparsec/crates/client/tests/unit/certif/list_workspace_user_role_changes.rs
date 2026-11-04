// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::CertifListWorkspaceUserRoleChangesError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let (realm_id, bob_id, expected) = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);

            let bob_id = builder.new_user("bob").map(|event| event.user_id);
            let expected: Vec<(DeviceID, DateTime, Option<RealmRole>)> = vec![
                builder
                    .share_realm(realm_id, bob_id, RealmRole::Contributor)
                    .map(|event| (event.author, event.timestamp, event.role)),
                builder
                    .share_realm(realm_id, bob_id, RealmRole::Manager)
                    .map(|event| (event.author, event.timestamp, event.role)),
                builder
                    .share_realm(realm_id, bob_id, None)
                    .map(|event| (event.author, event.timestamp, event.role)),
            ];
            builder.certificates_storage_fetch_certificates("alice@dev1");

            (realm_id, bob_id, expected)
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops
        .list_workspace_user_role_changes(realm_id, bob_id)
        .await
        .unwrap()
        .into_iter()
        .map(|change| (change.granted_by, change.granted_on, change.role))
        .collect::<Vec<_>>();

    p_assert_eq!(res, expected);
}

#[parsec_test(testbed = "minimal")]
async fn never_shared(env: &TestbedEnv) {
    let (realm_id, bob_id) = env
        .customize(|builder| {
            let realm_id = builder.new_realm("alice").map(|event| event.realm_id);
            let bob_id = builder.new_user("bob").map(|event| event.user_id);

            builder.certificates_storage_fetch_certificates("alice@dev1");

            (realm_id, bob_id)
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops
        .list_workspace_user_role_changes(realm_id, bob_id)
        .await
        .unwrap()
        .into_iter()
        .map(|change| (change.granted_by, change.granted_on, change.role))
        .collect::<Vec<_>>();

    assert!(res.is_empty());
}

#[parsec_test(testbed = "minimal")]
async fn empty(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops
        .list_workspace_user_role_changes(VlobID::default(), alice.user_id)
        .await
        .unwrap();

    assert!(res.is_empty());
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops
        .list_workspace_user_role_changes(VlobID::default(), alice.user_id)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifListWorkspaceUserRoleChangesError::Stopped);
}
