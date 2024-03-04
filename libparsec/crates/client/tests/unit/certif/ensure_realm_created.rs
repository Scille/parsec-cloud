// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::{CertifEnsureRealmCreatedError, CertificateBasedActionOutcome};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder.new_realm("alice").map(|event| event.realm_id);
        builder.certificates_storage_fetch_certificates("alice@dev1");
        realm_id
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let res = ops.ensure_realm_created(realm_id).await.unwrap();

    p_assert_matches!(res, CertificateBasedActionOutcome::LocalIdempotent);
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .ensure_realm_created(VlobID::default())
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifEnsureRealmCreatedError::Offline);
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops
        .ensure_realm_created(VlobID::default())
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifEnsureRealmCreatedError::Stopped);
}
