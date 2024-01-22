// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

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
        builder.new_sequester_service();
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
async fn duplicate_authority_certificate(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let mut sequester_signed = env.get_sequester_certificates_signed();
    sequester_signed.push(sequester_signed[0].clone());

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &sequester_signed,
            &[],
            &Default::default(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(
            // Corrupted because only the first certificate is allowed to be authority
            InvalidCertificateError::Corrupted { .. }
        )
    )
}

#[parsec_test(testbed = "empty")]
async fn content_already_exists(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
        let service_id = builder.new_sequester_service().map(|e| e.id);
        builder.new_sequester_service().customize(|e| {
            e.id = service_id;
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
        CertifAddCertificatesBatchError::InvalidCertificate(
            InvalidCertificateError::ContentAlreadyExists { .. }
        )
    )
}

#[parsec_test(testbed = "empty")]
async fn missing_authority_certificate(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
        builder.new_sequester_service();
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let mut sequester_signed = env.get_sequester_certificates_signed();
    // Remove the authority certificate
    sequester_signed.remove(0);

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &sequester_signed,
            &[],
            &Default::default(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(
            // Corrupted because only the first certificate can only be an authority
            InvalidCertificateError::Corrupted { .. }
        )
    )
}

#[parsec_test(testbed = "empty")]
async fn invalid_timestamp(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
        builder.new_sequester_service().customize(|e| {
            e.timestamp = DateTime::from_ymd_hms_us(1999, 1, 1, 0, 0, 0, 0).unwrap();
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
        CertifAddCertificatesBatchError::InvalidCertificate(InvalidCertificateError::InvalidTimestamp {
            last_certificate_timestamp,
            ..
        })
        if last_certificate_timestamp == DateTime::from_ymd_hms_us(2000, 1, 2, 0, 0, 0, 0).unwrap()
    );
}

#[parsec_test(testbed = "empty")]
async fn root_signature_timestamp_mismatch(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let (authority_certif, _) = env.get_sequester_authority_certificate();

    let mut old_authority_certif = (*authority_certif).clone();
    old_authority_certif.timestamp = DateTime::from_ymd_hms_us(1999, 1, 1, 0, 0, 0, 0).unwrap();
    let old_authority_signed = Bytes::from(old_authority_certif.dump_and_sign(&alice.signing_key));

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[old_authority_signed],
            &[],
            &Default::default(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(InvalidCertificateError::RootSignatureTimestampMismatch {
            last_root_signature_timestamp,
            ..
        })
        if last_root_signature_timestamp == DateTime::from_ymd_hms_us(1999, 1, 1, 0, 0, 0, 0).unwrap()
    );
}
