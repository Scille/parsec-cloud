// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::{CertifAddCertificatesBatchError, InvalidCertificateError};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn corrupted_signature(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let signed = Bytes::from_static(b"corrupted");

    let err = ops
        .add_certificates_batch(&[signed], &[], &[], &HashMap::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(*boxed, InvalidCertificateError::Corrupted {
            error: DataError::Signature,
            ..
        })
    );
}

#[parsec_test(testbed = "minimal")]
async fn corrupted_compression(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let signed = Bytes::from(alice.signing_key.sign(b"corrupted"));

    let err = ops
        .add_certificates_batch(&[signed], &[], &[], &HashMap::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed) if matches!(*boxed, InvalidCertificateError::Corrupted {
            error: DataError::Compression,
            ..
        })
    );
}

#[parsec_test(testbed = "minimal")]
async fn corrupted_serialization(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;
    let now = DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap();

    let manifest = UserManifest {
        author: alice.device_id.to_owned(),
        timestamp: now,
        id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        version: 0,
        created: now,
        updated: now,
        workspaces_legacy_initial_info: vec![],
    };

    let signed = Bytes::from(manifest.dump_and_sign(&alice.signing_key));

    let err = ops
        .add_certificates_batch(&[signed], &[], &[], &HashMap::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed) if matches!(*boxed, InvalidCertificateError::Corrupted {
            error: DataError::Serialization,
            ..
        })
    );
}
