// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::{
    CertifAddCertificatesBatchError, InvalidCertificateError, MaybeRedactedSwitch,
};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.update_user_profile("bob", UserProfile::Admin);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &Default::default(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch);
}

#[parsec_test(testbed = "minimal")]
async fn ok_outsider_switch(
    #[values(true, false)] switched_is_self: bool,
    #[values(true, false)] switch_from_outsider: bool,
    #[values(true, false)] storage_initially_empty: bool,
    env: &TestbedEnv,
) {
    let (initial_profile, updated_profile) = if switch_from_outsider {
        (UserProfile::Outsider, UserProfile::Standard)
    } else {
        (UserProfile::Standard, UserProfile::Outsider)
    };
    let env = env.customize(|builder| {
        builder
            .new_user("bob")
            .with_initial_profile(initial_profile);
        builder.update_user_profile("bob", updated_profile);
    });
    let device = if switched_is_self {
        env.local_device("bob@dev1")
    } else {
        env.local_device("alice@dev1")
    };
    let ops = certificates_ops_factory(&env, &device).await;
    let mut certificates = env.get_common_certificates_signed();

    if !storage_initially_empty {
        let first_certificate = certificates.remove(0);
        ops.add_certificates_batch(&[first_certificate], &[], &[], &Default::default())
            .await
            .unwrap();
    }

    let switch = ops
        .add_certificates_batch(&certificates, &[], &[], &Default::default())
        .await
        .unwrap();

    if switched_is_self && !storage_initially_empty {
        p_assert_matches!(switch, MaybeRedactedSwitch::Switched);
    } else {
        p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch);
    }
}

#[parsec_test(testbed = "minimal")]
async fn content_already_exists(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.update_user_profile("bob", UserProfile::Admin);
        builder.with_check_consistency_disabled(|builder| {
            builder.update_user_profile("bob", UserProfile::Admin);
        });
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &Default::default(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed) if matches!(*boxed,
            InvalidCertificateError::ContentAlreadyExists { .. }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn older_than_author(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder
            .update_user_profile("bob", UserProfile::Admin)
            .customize(|event| {
                event.timestamp = DateTime::from_ymd_hms_us(1999, 1, 1, 0, 0, 0, 0).unwrap()
            });
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &Default::default(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed) if matches!(*boxed, InvalidCertificateError::OlderThanAuthor {
            author_created_on,
            ..
        }
        if author_created_on == DateTime::from_ymd_hms_us(2000, 1, 2, 0, 0, 0, 0).unwrap()
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_timestamp(env: &TestbedEnv) {
    let (env, timestamp) = env.customize_with_map(|builder| {
        let timestamp = builder.new_user("bob").map(|e| e.timestamp);
        builder
            .update_user_profile("bob", UserProfile::Admin)
            .customize(|event| {
                event.timestamp = timestamp;
            });
        timestamp
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &Default::default(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed) if matches!(*boxed, InvalidCertificateError::InvalidTimestamp {
            last_certificate_timestamp,
            ..
        }
        if last_certificate_timestamp == timestamp
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn non_existing_author(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.update_user_profile("bob", UserProfile::Standard);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let certificates = vec![
        env.get_user_certificate("alice").1,
        // Alice's device certificate is missing !
        env.get_user_certificate("bob").1,
        env.get_device_certificate("bob@dev1").1,
        env.get_last_user_update_certificate("bob").1,
    ];

    let err = ops
        .add_certificates_batch(&certificates, &[], &[], &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed) if matches!(*boxed,
            InvalidCertificateError::NonExistingAuthor { .. }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn same_profile(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.update_user_profile("bob", UserProfile::Standard);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &Default::default(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed) if matches!(*boxed,
            InvalidCertificateError::ContentAlreadyExists { .. }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn owner_of_realm_not_shared_to_outsider_is_ok(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user_realm("bob");
        builder.update_user_profile("bob", UserProfile::Outsider);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &Default::default(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch);
}

#[parsec_test(testbed = "minimal")]
// Outsider is not allowed to be Owner and Manager of a realm,
// however switching to outsider is considered a corner case so
// we allow this exotic behavior to simplify code (typically admin
// may not have access to the realm certificates that cause the issue)
#[case(RealmRole::Owner)]
#[case(RealmRole::Manager)]
#[case(RealmRole::Contributor)]
#[case(RealmRole::Reader)]
async fn realm_member_switching_to_outsider_is_ok(#[case] role: RealmRole, env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder.share_realm(realm_id, "bob", Some(role));
        builder.update_user_profile("bob", UserProfile::Outsider);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &Default::default(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch);
}

#[parsec_test(testbed = "minimal")]
async fn revoked(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Admin);
        builder.new_user("mallory");
        builder.revoke_user("bob");
        builder
            .update_user_profile("mallory", UserProfile::Admin)
            .with_author("bob@dev1".try_into().unwrap());
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &Default::default(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed) if matches!(*boxed,
            InvalidCertificateError::RevokedAuthor { author_revoked_on, .. }
        if author_revoked_on == DateTime::from_ymd_hms_us(2000, 1, 5, 0, 0, 0, 0).unwrap()
        )
    );
}

#[parsec_test(testbed = "minimal")]
#[case(UserProfile::Standard)]
#[case(UserProfile::Outsider)]
async fn not_admin(#[case] profile: UserProfile, env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob").with_initial_profile(profile);
        builder.new_user("mallory");
        builder
            .update_user_profile("mallory", UserProfile::Admin)
            .with_author("bob@dev1".try_into().unwrap());
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &Default::default(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed) if matches!(*boxed, InvalidCertificateError::AuthorNotAdmin { author_profile, .. }
        if author_profile == profile
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn self_author(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder
            .update_user_profile("bob", UserProfile::Admin)
            .with_author("bob@dev1".try_into().unwrap());
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &Default::default(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed) if matches!(*boxed,
            InvalidCertificateError::SelfSigned { .. }
        )
    );
}
