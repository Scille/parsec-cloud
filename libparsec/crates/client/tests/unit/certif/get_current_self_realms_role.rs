// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::CertifGetCurrentSelfRealmsRoleError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let (env, (realm_id, timestamp)) = env.customize_with_map(|builder| {
        let (realm_id, timestamp) = builder
            .new_realm("alice")
            .map(|event| (event.realm_id, event.timestamp));

        builder.certificates_storage_fetch_certificates("alice@dev1");

        (realm_id, timestamp)
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let res = ops.get_current_self_realms_role().await.unwrap();

    p_assert_eq!(res, [(realm_id, Some(RealmRole::Owner), timestamp)]);
}

#[parsec_test(testbed = "minimal")]
async fn multiple_realm(env: &TestbedEnv) {
    let (env, (realm_id1, t1, realm_id2, t2)) = env.customize_with_map(|builder| {
        let (realm_id1, t1) = builder
            .new_realm("alice")
            .map(|event| (event.realm_id, event.timestamp));

        let (realm_id2, t2) = builder
            .new_realm("alice")
            .map(|event| (event.realm_id, event.timestamp));

        builder.certificates_storage_fetch_certificates("alice@dev1");

        (realm_id1, t1, realm_id2, t2)
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let res = ops.get_current_self_realms_role().await.unwrap();

    p_assert_eq!(
        res,
        [
            (realm_id1, Some(RealmRole::Owner), t1),
            (realm_id2, Some(RealmRole::Owner), t2)
        ]
    );
}

#[parsec_test(testbed = "minimal")]
async fn duplicate_realm_id(env: &TestbedEnv) {
    let (env, (realm_id, timestamp)) = env.customize_with_map(|builder| {
        builder.new_user("bob");
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

        builder.share_realm(realm_id, "bob", Some(RealmRole::Contributor));
        let timestamp = builder
            .share_realm(realm_id, "bob", Some(RealmRole::Manager))
            .map(|event| event.timestamp);

        builder.certificates_storage_fetch_certificates("bob@dev1");

        (realm_id, timestamp)
    });
    let bob = env.local_device("bob@dev1");
    let ops = certificates_ops_factory(&env, &bob).await;

    let res = ops.get_current_self_realms_role().await.unwrap();

    p_assert_eq!(res, [(realm_id, Some(RealmRole::Manager), timestamp)]);
}

#[parsec_test(testbed = "minimal")]
async fn empty(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let res = ops.get_current_self_realms_role().await.unwrap();

    p_assert_eq!(res, []);
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops.get_current_self_realms_role().await.unwrap_err();

    p_assert_matches!(err, CertifGetCurrentSelfRealmsRoleError::Stopped);
}
