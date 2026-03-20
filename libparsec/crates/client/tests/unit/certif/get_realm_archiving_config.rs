// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::CertifGetRealmArchivingConfigurationError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn no_archiving_certificate(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);

            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops
        .get_realm_archiving_configuration(realm_id)
        .await
        .unwrap();

    p_assert_eq!(res, RealmArchivingConfiguration::Available);
}

#[parsec_test(testbed = "minimal")]
async fn archived(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);

            builder.archive_realm(realm_id, RealmArchivingConfiguration::Archived);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops
        .get_realm_archiving_configuration(realm_id)
        .await
        .unwrap();

    p_assert_eq!(res, RealmArchivingConfiguration::Archived);
}

#[parsec_test(testbed = "minimal")]
async fn deletion_planned(env: &TestbedEnv) {
    let deletion_date = DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap();
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);

            builder.archive_realm(
                realm_id,
                RealmArchivingConfiguration::DeletionPlanned { deletion_date },
            );
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops
        .get_realm_archiving_configuration(realm_id)
        .await
        .unwrap();

    p_assert_eq!(
        res,
        RealmArchivingConfiguration::DeletionPlanned { deletion_date }
    );
}

#[parsec_test(testbed = "minimal")]
async fn multiple_archiving_certificates(env: &TestbedEnv) {
    let deletion_date = DateTime::from_ymd_hms_us(2010, 1, 1, 0, 0, 0, 0).unwrap();
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);

            builder.archive_realm(realm_id, RealmArchivingConfiguration::Archived);
            builder.archive_realm(
                realm_id,
                RealmArchivingConfiguration::DeletionPlanned { deletion_date },
            );
            builder.archive_realm(realm_id, RealmArchivingConfiguration::Available);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops
        .get_realm_archiving_configuration(realm_id)
        .await
        .unwrap();

    // The last certificate should win
    p_assert_eq!(res, RealmArchivingConfiguration::Available);
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops
        .get_realm_archiving_configuration(VlobID::default())
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifGetRealmArchivingConfigurationError::Stopped);
}
