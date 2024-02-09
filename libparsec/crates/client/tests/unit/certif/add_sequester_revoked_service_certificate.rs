// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;

use crate::certif::{
    CertifAddCertificatesBatchError, InvalidCertificateError, MaybeRedactedSwitch,
};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "empty")]
async fn ok(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
        let service_id = builder.new_sequester_service().map(|event| event.id);
        builder.revoke_sequester_service(service_id);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &env.get_sequester_certificates_signed(),
            &[],
            &Default::default(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch);
}

#[parsec_test(testbed = "empty")]
async fn already_revoked(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
        let service_id = builder.new_sequester_service().map(|event| event.id);

        builder.revoke_sequester_service(service_id);
        builder.revoke_sequester_service(service_id);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &env.get_sequester_certificates_signed(),
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
    )
}

#[parsec_test(testbed = "empty")]
async fn invalid_timestamp(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
        let service_id = builder.new_sequester_service().map(|event| event.id);

        builder
            .revoke_sequester_service(service_id)
            .customize(|event| {
                event.timestamp = DateTime::from_ymd_hms_us(1999, 1, 1, 0, 0, 0, 0).unwrap();
            });
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &env.get_sequester_certificates_signed(),
            &[],
            &Default::default(),
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
            if last_certificate_timestamp == DateTime::from_ymd_hms_us(2000, 1, 2, 0, 0, 0, 0).unwrap()
        )
    );
}
