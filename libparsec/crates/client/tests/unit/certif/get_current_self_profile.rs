// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::CertifGetCurrentSelfProfileError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    env.customize(|builder| {
        // Need Bob to change Alice's profile
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Admin);
        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder.update_user_profile("alice", UserProfile::Standard);
        // Note at first alice@dev1 is not aware of the profile change !
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops.get_current_self_profile().await.unwrap();
    p_assert_eq!(res, UserProfile::Admin);

    // Now add the change and ensure the cache gets updated as expected !

    let new_certif = env.template.certificates_rev().next().unwrap().signed;
    ops.add_certificates_batch(&[new_certif], &[], &[], &HashMap::new())
        .await
        .unwrap();

    let res = ops.get_current_self_profile().await.unwrap();
    p_assert_eq!(res, UserProfile::Standard);
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops.get_current_self_profile().await.unwrap_err();

    p_assert_matches!(err, CertifGetCurrentSelfProfileError::Stopped);
}
