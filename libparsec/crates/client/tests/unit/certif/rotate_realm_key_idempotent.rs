// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::test_register_sequence_of_send_hooks;
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{CertifRotateRealmKeyError, CertificateBasedActionOutcome};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let outcome = ops.rotate_realm_key_idempotent(wksp1_id, 2).await.unwrap();
    p_assert_matches!(outcome, CertificateBasedActionOutcome::Uploaded { .. });
}

#[parsec_test(testbed = "sequestered", with_server)]
async fn sequestered_ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let outcome = ops.rotate_realm_key_idempotent(wksp1_id, 4).await.unwrap();
    p_assert_matches!(outcome, CertificateBasedActionOutcome::Uploaded { .. });
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn noop(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let key_index_2_timestamp = env
        .customize(|builder| {
            let key_index_2_timestamp = builder.rotate_key_realm(wksp1_id).map(|e| e.timestamp);
            key_index_2_timestamp
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    // Noop (key index already in local storage)

    let outcome = ops.rotate_realm_key_idempotent(wksp1_id, 1).await.unwrap();
    p_assert_matches!(outcome, CertificateBasedActionOutcome::LocalIdempotent);

    // Noop (key index not in local storage, but exists on server)

    let outcome = ops.rotate_realm_key_idempotent(wksp1_id, 2).await.unwrap();
    p_assert_matches!(
        outcome,
        CertificateBasedActionOutcome::RemoteIdempotent { certificate_timestamp }
        if certificate_timestamp == key_index_2_timestamp
    );

    // Key index to far for rotation

    let outcome = ops.rotate_realm_key_idempotent(wksp1_id, 5).await.unwrap();
    p_assert_matches!(outcome, CertificateBasedActionOutcome::LocalIdempotent);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn author_not_allowed(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let bob = env.local_device("bob@dev1");
    let ops = certificates_ops_factory(env, &bob).await;

    let err = ops
        .rotate_realm_key_idempotent(wksp1_id, 2)
        .await
        .unwrap_err();
    p_assert_matches!(err, CertifRotateRealmKeyError::AuthorNotAllowed,);
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn unknown_realm(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .rotate_realm_key_idempotent(VlobID::default(), 1)
        .await
        .unwrap_err();
    p_assert_matches!(err, CertifRotateRealmKeyError::UnknownRealm,);
}

#[parsec_test(testbed = "coolorg")]
async fn offline(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .rotate_realm_key_idempotent(wksp1_id, 2)
        .await
        .unwrap_err();
    p_assert_matches!(err, CertifRotateRealmKeyError::Offline(_));
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stopped(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops
        .rotate_realm_key_idempotent(wksp1_id, 2)
        .await
        .unwrap_err();
    p_assert_matches!(err, CertifRotateRealmKeyError::Stopped,);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn current_keys_bundle_corrupted(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let keys_bundle = Bytes::from_static(b"<dummy>");
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(wksp1_id, alice.user_id);
    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            p_assert_eq!(req.key_index, 1);
            p_assert_eq!(req.realm_id, wksp1_id);

            authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                keys_bundle,
                keys_bundle_access,
            }
        }
    });

    let err = ops
        .rotate_realm_key_idempotent(wksp1_id, 2)
        .await
        .unwrap_err();
    p_assert_matches!(
        err,
        CertifRotateRealmKeyError::CurrentKeysBundleCorrupted { .. },
    );
}
