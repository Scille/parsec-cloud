// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashSet, sync::Arc};

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::{
    CertifAddCertificatesBatchError, InvalidCertificateError, MaybeRedactedSwitch,
};

use super::utils::certificates_ops_factory;

fn certifs_from_last_shamir_recovery(env: &TestbedEnv) -> Vec<Bytes> {
    match env.template.events.last().unwrap() {
        TestbedEvent::DeleteShamirRecovery(e) => {
            e.certificates(&env.template).map(|c| c.signed).collect()
        }
        _ => unreachable!(),
    }
}

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
        let recovery_device_id = builder.new_device("bob").map(|e| e.device_id);

        builder.new_shamir_recovery(
            "bob",
            1,
            [("alice".parse().unwrap(), 1.try_into().unwrap())],
            recovery_device_id,
        );

        builder.certificates_storage_fetch_certificates("alice@dev1");

        builder.delete_shamir_recovery("bob");
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let shamir_recovery_certificates = certifs_from_last_shamir_recovery(env);

    let switch = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch { .. });
}

#[parsec_test(testbed = "minimal")]
async fn already_deleted(env: &TestbedEnv) {
    let delete_timestamp = env
        .customize(|builder| {
            builder.new_user("bob");
            let recovery_device_id = builder.new_device("bob").map(|e| e.device_id);

            builder.new_shamir_recovery(
                "bob",
                1,
                [("alice".parse().unwrap(), 1.try_into().unwrap())],
                recovery_device_id,
            );
            let (setup_timestamp, delete_timestamp) = builder
                .delete_shamir_recovery("bob")
                .map(|e| (e.setup_to_delete_timestamp, e.timestamp));

            builder.certificates_storage_fetch_certificates("alice@dev1");

            builder.with_check_consistency_disabled(|builder| {
                builder
                    .delete_shamir_recovery("bob")
                    .with_setup_to_delete_timestamp(setup_timestamp);
            });

            delete_timestamp
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let shamir_recovery_certificates = certifs_from_last_shamir_recovery(env);

    let err = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(*boxed, InvalidCertificateError::ShamirRecoveryAlreadyDeleted { deleted_on, .. } if deleted_on == delete_timestamp)
    )
}

#[parsec_test(testbed = "minimal")]
async fn missing_brief(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
        let recovery_device_id = builder.new_device("bob").map(|e| e.device_id);

        builder.certificates_storage_fetch_certificates("alice@dev1");

        builder.new_shamir_recovery(
            "bob",
            1,
            [("alice".parse().unwrap(), 1.try_into().unwrap())],
            recovery_device_id,
        );
        builder.delete_shamir_recovery("bob");
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    // We get the delete certificate, but not the brief & share
    let shamir_recovery_certificates = certifs_from_last_shamir_recovery(env);

    let err = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::ShamirRecoveryDeletionMustReferenceLastSetup { .. }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn setup_timestamp_mismatch_with_last_valid_brief(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
        let recovery_device_id = builder.new_device("bob").map(|e| e.device_id);

        let older_shamir_timestamp = builder
            .new_shamir_recovery(
                "bob",
                1,
                [("alice".parse().unwrap(), 1.try_into().unwrap())],
                recovery_device_id,
            )
            .map(|event| event.timestamp);

        builder.delete_shamir_recovery("bob");

        builder.new_shamir_recovery(
            "bob",
            1,
            [("alice".parse().unwrap(), 1.try_into().unwrap())],
            recovery_device_id,
        );

        builder.certificates_storage_fetch_certificates("alice@dev1");

        builder
            .delete_shamir_recovery("bob")
            .with_setup_to_delete_timestamp(older_shamir_timestamp);
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let shamir_recovery_certificates = certifs_from_last_shamir_recovery(env);

    let err = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::ShamirRecoveryDeletionMustReferenceLastSetup {
                ..
            }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn recipients_mismatch(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
        let recovery_device_id = builder.new_device("bob").map(|e| e.device_id);

        builder.new_shamir_recovery(
            "bob",
            1,
            [("alice".parse().unwrap(), 1.try_into().unwrap())],
            recovery_device_id,
        );

        builder.certificates_storage_fetch_certificates("alice@dev1");

        builder
            .delete_shamir_recovery("bob")
            .with_share_recipients(HashSet::from_iter(["mallory".parse().unwrap()]));
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let shamir_recovery_certificates = certifs_from_last_shamir_recovery(env);

    let err = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::ShamirRecoveryDeletionRecipientsMismatch {
                ref setup_recipients,
                ..
            } if *setup_recipients == HashSet::from_iter(["alice".parse().unwrap()])
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_timestamp(env: &TestbedEnv) {
    let timestamp = env
        .customize(|builder| {
            builder.new_user("bob");
            let recovery_device_id = builder.new_device("bob").map(|e| e.device_id);

            let timestamp = builder
                .new_shamir_recovery(
                    "bob",
                    1,
                    [("alice".parse().unwrap(), 1.try_into().unwrap())],
                    recovery_device_id,
                )
                .map(|event| event.timestamp);

            builder.certificates_storage_fetch_certificates("alice@dev1");

            builder
                .delete_shamir_recovery("bob")
                .customize(|e| e.timestamp = timestamp);

            timestamp
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let shamir_recovery_certificates = certifs_from_last_shamir_recovery(env);

    let err = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
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
async fn invalid_user_id(env: &TestbedEnv) {
    let bob_user_id = env
        .customize(|builder| {
            let bob_user_id = builder.new_user("bob").map(|u| u.user_id);
            let recovery_device_id = builder.new_device("bob").map(|e| e.device_id);

            builder.new_shamir_recovery(
                "bob",
                1,
                [("alice".parse().unwrap(), 1.try_into().unwrap())],
                recovery_device_id,
            );

            builder.certificates_storage_fetch_certificates("alice@dev1");

            builder.delete_shamir_recovery("bob");

            bob_user_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    // Patch the certificate to have an invalid user id

    let shamir_recovery_certificates = match env.template.events.last().unwrap() {
        TestbedEvent::DeleteShamirRecovery(e) => {
            let mut certifs = e.certificates(&env.template);
            let delete = certifs.next().unwrap();
            assert!(certifs.next().is_none()); // Sanity check

            let delete_certificate = match delete.certificate {
                libparsec_types::AnyArcCertificate::ShamirRecoveryDeletion(mut c) => {
                    let c_mut = Arc::make_mut(&mut c);
                    c_mut.setup_to_delete_user_id = alice.user_id;
                    c_mut.dump_and_sign(&bob.signing_key)
                }
                _ => unreachable!(),
            };
            vec![delete_certificate.into()]
        }
        _ => unreachable!(),
    };

    let err = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::ShamirRecoveryNotAboutSelf { user_id, author_user_id, .. }
            if user_id == alice.user_id && author_user_id == bob_user_id
        )
    );
}
