// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::{
    CertifAddCertificatesBatchError, InvalidCertificateError, MaybeRedactedSwitch,
};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "empty")]
async fn ok(env: &TestbedEnv) {
    env.customize(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &env.get_sequester_certificates_signed(),
            &[],
            &Default::default(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch { .. });
}

#[parsec_test(testbed = "empty")]
async fn duplicate_authority_certificate(env: &TestbedEnv) {
    env.customize(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            // Corrupted because only the first certificate is allowed to be authority
            InvalidCertificateError::Corrupted { .. }
        )
    )
}

#[parsec_test(testbed = "empty")]
async fn root_signature_timestamp_mismatch(env: &TestbedEnv) {
    env.customize(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let (authority_certif, _) = env.get_sequester_authority_certificate();

    let mut old_authority_certif = (*authority_certif).clone();
    old_authority_certif.timestamp = DateTime::from_ymd_hms_us(1999, 1, 1, 0, 0, 0, 0).unwrap();
    let old_authority_signed =
        Bytes::from(old_authority_certif.dump_and_sign(env.template.root_signing_key()));

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
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::RootSignatureTimestampMismatch {
                last_root_signature_timestamp,
                ..
            }
            if last_root_signature_timestamp == DateTime::from_ymd_hms_us(1999, 1, 1, 0, 0, 0, 0).unwrap()
        )
    );
}
