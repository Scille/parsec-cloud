// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::{
    CertifAddCertificatesBatchError, InvalidCertificateError, MaybeRedactedSwitch,
};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(
    #[values(
        RealmArchivingConfiguration::Available,
        RealmArchivingConfiguration::Archived,
        RealmArchivingConfiguration::DeletionPlanned {
            deletion_date: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap()
        },
    )]
    conf: RealmArchivingConfiguration,
    env: &TestbedEnv,
) {
    let env = env.customize(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

        builder.archive_realm(realm_id, conf);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch);
}

#[parsec_test(testbed = "minimal")]
async fn multiple(
    #[values(
        RealmArchivingConfiguration::Available,
        RealmArchivingConfiguration::Archived,
        RealmArchivingConfiguration::DeletionPlanned {
            deletion_date: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap()
        },
    )]
    first_conf: RealmArchivingConfiguration,
    #[values(
        RealmArchivingConfiguration::Available,
        RealmArchivingConfiguration::Archived,
        RealmArchivingConfiguration::DeletionPlanned {
            deletion_date: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap()
        },
    )]
    second_conf: RealmArchivingConfiguration,
    env: &TestbedEnv,
) {
    let env = env.customize(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

        builder.archive_realm(realm_id, first_conf);
        builder.archive_realm(realm_id, second_conf);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch);
}

#[parsec_test(testbed = "minimal")]
async fn older_than_author(
    #[values(
        RealmArchivingConfiguration::Available,
        RealmArchivingConfiguration::Archived,
        RealmArchivingConfiguration::DeletionPlanned {
            deletion_date: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap()
        },
    )]
    conf: RealmArchivingConfiguration,
    env: &TestbedEnv,
) {
    let env = env.customize(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

        builder.archive_realm(realm_id, conf).customize(|event| {
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
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::OlderThanAuthor {
                author_created_on,
                ..
            }
            if author_created_on == DateTime::from_ymd_hms_us(2000, 1, 2, 0, 0, 0, 0).unwrap()
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_timestamp(
    #[values(
        RealmArchivingConfiguration::Available,
        RealmArchivingConfiguration::Archived,
        RealmArchivingConfiguration::DeletionPlanned {
            deletion_date: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap()
        },
    )]
    conf: RealmArchivingConfiguration,
    env: &TestbedEnv,
) {
    let (env, timestamp) = env.customize_with_map(|builder| {
        let (realm_id, timestamp) = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| (event.realm, event.timestamp));

        builder
            .archive_realm(realm_id, conf)
            .customize(|event| event.timestamp = timestamp);

        timestamp
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::InvalidTimestamp {
                last_certificate_timestamp,
                ..
            }
            if last_certificate_timestamp == timestamp
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn not_member(
    #[values(
        RealmArchivingConfiguration::Available,
        RealmArchivingConfiguration::Archived,
        RealmArchivingConfiguration::DeletionPlanned {
            deletion_date: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap()
        },
    )]
    conf: RealmArchivingConfiguration,
    env: &TestbedEnv,
) {
    let env = env.customize(|builder| {
        let bob = builder.new_user("bob").map(|event| event.device_id.clone());
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

        builder
            .archive_realm(realm_id, conf)
            .customize(|event| event.author = bob);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(*boxed, InvalidCertificateError::RealmAuthorHasNoRole { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn author_not_owner(
    #[values(
        RealmArchivingConfiguration::Available,
        RealmArchivingConfiguration::Archived,
        RealmArchivingConfiguration::DeletionPlanned {
            deletion_date: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap()
        },
    )]
    conf: RealmArchivingConfiguration,
    env: &TestbedEnv,
) {
    let env = env.customize(|builder| {
        let bob = builder.new_user("bob").map(|event| event.device_id.clone());
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

        builder.share_realm(realm_id, "bob", Some(RealmRole::Contributor));

        builder
            .archive_realm(realm_id, conf)
            .customize(|event| event.author = bob);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(*boxed, InvalidCertificateError::RealmAuthorNotOwner { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn revoked(
    #[values(
        RealmArchivingConfiguration::Available,
        RealmArchivingConfiguration::Archived,
        RealmArchivingConfiguration::DeletionPlanned {
            deletion_date: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap()
        },
    )]
    conf: RealmArchivingConfiguration,
    env: &TestbedEnv,
) {
    let env = env.customize(|builder| {
        let bob = builder.new_user("bob").map(|event| event.device_id.clone());
        let realm_id = builder
            .new_realm("bob")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

        builder
            .share_realm(realm_id, "alice", Some(RealmRole::Owner))
            .customize(|event| event.author = bob.clone());

        builder.revoke_user("bob");

        builder
            .archive_realm(realm_id, conf)
            .customize(|event| event.author = bob);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(*boxed, InvalidCertificateError::RevokedAuthor { .. })
    );
}
