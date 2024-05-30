// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::CertifGetCurrentSelfRealmRoleError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder.new_realm("alice").map(|event| event.realm_id);
            builder.certificates_storage_fetch_certificates("alice@dev1");
            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops.get_current_self_realm_role(realm_id).await.unwrap();

    p_assert_eq!(res, Some(Some(RealmRole::Owner)));
}

#[parsec_test(testbed = "minimal")]
async fn last_share_role_is_used(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            builder.new_user("bob");
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);

            builder.share_realm(realm_id, "bob", Some(RealmRole::Contributor));
            builder.share_realm(realm_id, "bob", Some(RealmRole::Manager));
            builder.certificates_storage_fetch_certificates("bob@dev1");

            realm_id
        })
        .await;
    let bob = env.local_device("bob@dev1");
    let ops = certificates_ops_factory(env, &bob).await;

    let res = ops.get_current_self_realm_role(realm_id).await.unwrap();

    p_assert_eq!(res, Some(Some(RealmRole::Manager)));
}

#[parsec_test(testbed = "minimal")]
async fn empty(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops
        .get_current_self_realm_role(VlobID::default())
        .await
        .unwrap();

    p_assert_eq!(res, None);
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops
        .get_current_self_realm_role(VlobID::default())
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifGetCurrentSelfRealmRoleError::Stopped);
}
