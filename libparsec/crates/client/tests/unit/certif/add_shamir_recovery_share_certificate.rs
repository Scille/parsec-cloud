// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::{
    CertifAddCertificatesBatchError, InvalidCertificateError, MaybeRedactedSwitch,
};

use super::utils::certificates_ops_factory;

fn certifs_from_last_shamir_recovery(env: &TestbedEnv) -> Vec<Bytes> {
    match env.template.events.last().unwrap() {
        TestbedEvent::NewShamirRecovery(e) => {
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

        builder.certificates_storage_fetch_certificates("alice@dev1");

        builder.new_shamir_recovery(
            "bob",
            1,
            [("alice".parse().unwrap(), 1.try_into().unwrap())],
            recovery_device_id,
        );
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
async fn duplicated(env: &TestbedEnv) {
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
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let mut shamir_recovery_certificates = certifs_from_last_shamir_recovery(env);
    shamir_recovery_certificates.push(shamir_recovery_certificates[1].clone()); // Duplicate alice share certificate
    p_assert_eq!(shamir_recovery_certificates.len(), 3); // Sanity check

    let err = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(*boxed, InvalidCertificateError::ContentAlreadyExists { .. })
    )
}

#[parsec_test(testbed = "minimal")]
async fn missing_brief(
    #[values(
        "no_brief_exists",
        "removed_brief_exists",
        "different_user_brief_exists"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let setup_timestamp = env
        .customize(|builder| {
            builder.new_user("bob");
            let recovery_device_id = builder.new_device("bob").map(|e| e.device_id);

            let setup_timestamp = match kind {
                "no_brief_exists" => builder.counters.next_timestamp(),
                "removed_brief_exists" => {
                    builder.new_shamir_recovery(
                        "bob",
                        1,
                        [("alice".parse().unwrap(), 1.try_into().unwrap())],
                        recovery_device_id,
                    );
                    builder.delete_shamir_recovery("bob").map(|e| e.timestamp)
                }
                "different_user_brief_exists" => {
                    builder.new_user("mallory");
                    let recovery_device_id = builder.new_device("mallory").map(|e| e.device_id);
                    builder
                        .new_shamir_recovery(
                            "mallory",
                            1,
                            [("alice".parse().unwrap(), 1.try_into().unwrap())],
                            recovery_device_id,
                        )
                        .map(|e| e.timestamp)
                }
                unknown => panic!("Unknown kind: {unknown}"),
            };

            builder.certificates_storage_fetch_certificates("alice@dev1");

            setup_timestamp
        })
        .await;
    let bob = env.local_device("bob@dev1");
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let shamir_recovery_share = ShamirRecoveryShareCertificate {
        author: bob.device_id,
        timestamp: setup_timestamp,
        user_id: bob.user_id,
        recipient: alice.user_id,
        ciphered_share: b"<share>".to_vec(),
    }
    .dump_and_sign(&bob.signing_key)
    .into();

    let err = ops
        .add_certificates_batch(&[], &[], &[shamir_recovery_share], &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::ShamirRecoveryMissingBriefCertificate { .. }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn timestamp_mismatch_with_brief(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
        let recovery_device_id = builder.new_device("bob").map(|e| e.device_id);

        builder.certificates_storage_fetch_certificates("alice@dev1");

        // First shamir, we will use its brief certificate
        builder
            .new_shamir_recovery(
                "bob",
                1,
                [("alice".parse().unwrap(), 1.try_into().unwrap())],
                recovery_device_id,
            )
            .map(|event| event.timestamp);

        builder.with_check_consistency_disabled(|builder| {
            // Second shamir, we will use its share certificate (so that timestamp differs)
            builder.new_shamir_recovery(
                "bob",
                1,
                [("alice".parse().unwrap(), 1.try_into().unwrap())],
                recovery_device_id,
            );
        });
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let mut shamir_recovery_certificates = env.get_shamir_recovery_certificates_signed();
    shamir_recovery_certificates.remove(2); // Remove second recovery's brief
    shamir_recovery_certificates.remove(1); // Remove first recovery's alice share
    p_assert_eq!(shamir_recovery_certificates.len(), 2); // Sanity check: expect first recovery brief + second recovery share

    let err = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::ShamirRecoveryMissingBriefCertificate { .. }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_timestamp(env: &TestbedEnv) {
    let timestamp = env
        .customize(|builder| {
            builder.new_user("bob");
            let alice_recovery_device_id = builder.new_device("alice").map(|e| e.device_id);
            builder.new_shamir_recovery(
                "alice",
                1,
                [("bob".parse().unwrap(), 1.try_into().unwrap())],
                alice_recovery_device_id,
            );

            let bob_recovery_device_id = builder.new_device("bob").map(|e| e.device_id);

            builder.certificates_storage_fetch_certificates("alice@dev1");

            // Good shamir, we will use its brief certificate
            let timestamp = builder
                .new_shamir_recovery(
                    "bob",
                    1,
                    [("alice".parse().unwrap(), 1.try_into().unwrap())],
                    bob_recovery_device_id,
                )
                .map(|e| e.timestamp);

            builder.with_check_consistency_disabled(|builder| {
                // Bad shamir, we will use its share certificate
                builder
                    .new_shamir_recovery(
                        "bob",
                        1,
                        [("alice".parse().unwrap(), 1.try_into().unwrap())],
                        bob_recovery_device_id,
                    )
                    .customize(|event| event.timestamp = timestamp.add_us(-1));
            });

            timestamp
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let mut shamir_recoveries = env.template.events.iter().filter_map(|e| match e {
        TestbedEvent::NewShamirRecovery(e) => Some(
            e.certificates(&env.template)
                .map(|c| c.signed)
                .collect::<Vec<_>>(),
        ),
        _ => None,
    });
    shamir_recoveries.next().unwrap(); // Ignore first that has already been fetched
    let good_brief = shamir_recoveries.next().unwrap()[0].clone(); // Take brief from the good shamir
    let bad_share = shamir_recoveries.next().unwrap()[1].clone(); // Take share from the bad shamir
    let shamir_recovery_certificates = vec![good_brief, bad_share];

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

            builder.certificates_storage_fetch_certificates("alice@dev1");

            builder.new_shamir_recovery(
                "bob",
                1,
                [("alice".parse().unwrap(), 1.try_into().unwrap())],
                recovery_device_id,
            );

            bob_user_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    // Patch the certificate to have an invalid user id

    let shamir_recovery_certificates = match env.template.events.last().unwrap() {
        TestbedEvent::NewShamirRecovery(e) => {
            let mut certifs = e.certificates(&env.template);
            let brief = certifs.next().unwrap();
            let share = certifs.next().unwrap();
            assert!(certifs.next().is_none()); // Sanity check

            let share_certificate = match share.certificate {
                libparsec_types::AnyArcCertificate::ShamirRecoveryShare(mut c) => {
                    let c_mut = Arc::make_mut(&mut c);
                    c_mut.user_id = alice.user_id;
                    c_mut.dump_and_sign(&bob.signing_key)
                }
                _ => unreachable!(),
            };
            vec![brief.signed, share_certificate.into()]
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

#[parsec_test(testbed = "minimal")]
async fn share_not_for_us(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        let recovery_device_id = builder.new_device("bob").map(|e| e.device_id);

        builder.certificates_storage_fetch_certificates("alice@dev1");

        builder.new_shamir_recovery(
            "bob",
            3,
            [
                ("alice".parse().unwrap(), 2.try_into().unwrap()),
                ("mallory".parse().unwrap(), 1.try_into().unwrap()),
            ],
            recovery_device_id,
        );
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let mut shamir_recovery_certificates: Vec<_> = certifs_from_last_shamir_recovery(env);
    // Keep Mallory share instead of our own
    shamir_recovery_certificates.remove(1);
    p_assert_eq!(shamir_recovery_certificates.len(), 2); // Sanity check: expect brief + Mallory's share

    let err = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::ShamirRecoveryUnrelatedToUs {
                ..
            }
        )
    );
}
