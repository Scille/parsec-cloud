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
async fn ok_as_only_recipient(
    #[values("initial_setup", "after_removed_setup")] kind: &str,
    env: &TestbedEnv,
) {
    env.customize(|builder| {
        builder.new_user("bob");
        let recovery_device_id = builder.new_device("bob").map(|e| e.device_id);

        match kind {
            "initial_setup" => (),
            "after_removed_setup" => {
                builder.new_shamir_recovery(
                    "bob",
                    1,
                    [("alice".parse().unwrap(), 1.try_into().unwrap())],
                    recovery_device_id,
                );
                builder.delete_shamir_recovery("bob");
            }
            unknown => panic!("Unknown kind: {}", unknown),
        }

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
async fn ok_as_recipient_among_others(env: &TestbedEnv) {
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
    // Must remove Mallory's share certificate since it not part of our shamir topic
    shamir_recovery_certificates.pop();
    p_assert_eq!(shamir_recovery_certificates.len(), 2); // Sanity check: expect brief + our share

    let switch = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch { .. });
}

#[parsec_test(testbed = "minimal")]
async fn ok_as_author(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        let recovery_device_id = builder.new_device("alice").map(|e| e.device_id);

        builder.certificates_storage_fetch_certificates("alice@dev1");

        builder.new_shamir_recovery(
            "alice",
            1,
            [
                ("bob".parse().unwrap(), 1.try_into().unwrap()),
                ("mallory".parse().unwrap(), 1.try_into().unwrap()),
            ],
            recovery_device_id,
        );
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let mut shamir_recovery_certificates: Vec<_> = certifs_from_last_shamir_recovery(env);
    // Must only keep the brief certificate since we are not among the recipients
    shamir_recovery_certificates.pop();
    shamir_recovery_certificates.pop();
    p_assert_eq!(shamir_recovery_certificates.len(), 1); // Sanity check: expect brief only

    let switch = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch { .. });
}

#[parsec_test(testbed = "minimal")]
async fn already_setup(env: &TestbedEnv) {
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

        builder.with_check_consistency_disabled(|builder| {
            builder.new_shamir_recovery(
                "bob",
                1,
                [("alice".parse().unwrap(), 1.try_into().unwrap())],
                recovery_device_id,
            );
        })
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let mut shamir_recovery_certificates: Vec<_> = certifs_from_last_shamir_recovery(env);
    // Only keep the brief certificate: the error should be triggered before
    // we need the share certificate.
    shamir_recovery_certificates.pop();

    let err = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::ShamirRecoveryAlreadySetup { .. }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn as_recipient_with_missing_related_share(env: &TestbedEnv) {
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

    let mut shamir_recovery_certificates: Vec<_> = certifs_from_last_shamir_recovery(env);
    // Given our user is among the recipient, the brief certificate must be
    // provided together with the share certificate for us.
    // Here we remove such share certificate...
    shamir_recovery_certificates.pop();
    p_assert_eq!(shamir_recovery_certificates.len(), 1); // Sanity check, only brief certificate remains

    let err = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            &*boxed,
            InvalidCertificateError::ShamirRecoveryMissingShare { hint, brief_hint, allowed_recipient }
            if hint == "<no more certificates>" && brief_hint.starts_with("ShamirRecoveryBriefCertificate") && *allowed_recipient == alice.user_id
        )
    )
}

#[parsec_test(testbed = "minimal")]
async fn older_than_author(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
        let recovery_device_id = builder.new_device("bob").map(|e| e.device_id);

        builder.certificates_storage_fetch_certificates("alice@dev1");

        builder
            .new_shamir_recovery(
                "bob",
                1,
                [("alice".parse().unwrap(), 1.try_into().unwrap())],
                recovery_device_id,
            )
            .customize(|event| {
                event.timestamp = DateTime::from_ymd_hms_us(1999, 1, 1, 0, 0, 0, 0).unwrap()
            });
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let mut shamir_recovery_certificates: Vec<_> = certifs_from_last_shamir_recovery(env);
    // Only keep the brief certificate: the error should be triggered before
    // we need the share certificate.
    shamir_recovery_certificates.pop();

    let err = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
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
            if author_created_on == DateTime::from_ymd_hms_us(2000, 1, 3, 0, 0, 0, 0).unwrap()
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_timestamp(env: &TestbedEnv) {
    let timestamp = env
        .customize(|builder| {
            builder.new_user("bob");
            let recovery_device_id = builder.new_device("bob").map(|e| e.device_id);

            builder.new_shamir_recovery(
                "bob",
                1,
                [("alice".parse().unwrap(), 1.try_into().unwrap())],
                recovery_device_id,
            );
            let timestamp = builder
                .delete_shamir_recovery("bob")
                .map(|event| event.timestamp);

            builder.certificates_storage_fetch_certificates("alice@dev1");

            builder
                .new_shamir_recovery(
                    "bob",
                    1,
                    [("alice".parse().unwrap(), 1.try_into().unwrap())],
                    recovery_device_id,
                )
                .customize(|event| event.timestamp = timestamp);

            timestamp
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let mut shamir_recovery_certificates: Vec<_> = certifs_from_last_shamir_recovery(env);
    // Only keep the brief certificate: the error should be triggered before
    // we need the share certificate.
    shamir_recovery_certificates.pop();

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
async fn revoked(env: &TestbedEnv) {
    let timestamp = env
        .customize(|builder| {
            builder.new_user("bob");
            let recovery_device_id = builder.new_device("bob").map(|e| e.device_id);
            let timestamp = builder.revoke_user("bob").map(|event| event.timestamp);

            builder.certificates_storage_fetch_certificates("alice@dev1");

            builder.with_check_consistency_disabled(|builder| {
                builder.new_shamir_recovery(
                    "bob",
                    1,
                    [("alice".parse().unwrap(), 1.try_into().unwrap())],
                    recovery_device_id,
                );
            });

            timestamp
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let mut shamir_recovery_certificates: Vec<_> = certifs_from_last_shamir_recovery(env);
    // Only keep the brief certificate: the error should be triggered before
    // we need the share certificate.
    shamir_recovery_certificates.pop();

    let err = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::RevokedAuthor { author_revoked_on, .. }
            if author_revoked_on == timestamp
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn self_recipient(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.certificates_storage_fetch_certificates("alice@dev1");

        builder.new_shamir_recovery(
            "alice",
            1,
            [("alice".parse().unwrap(), 1.try_into().unwrap())],
            "alice@dev1",
        );
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let shamir_recovery_certificates: Vec<_> = certifs_from_last_shamir_recovery(env);

    let err = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::ShamirRecoverySelfAmongRecipients { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
async fn recipient_revoked(env: &TestbedEnv) {
    let timestamp = env
        .customize(|builder| {
            builder.new_user("bob");
            let recovery_device_id = builder.new_device("bob").map(|e| e.device_id);

            let timestamp = builder.revoke_user("bob").map(|event| event.timestamp);

            builder.certificates_storage_fetch_certificates("alice@dev1");

            builder.with_check_consistency_disabled(|builder| {
                builder.new_shamir_recovery(
                    "alice",
                    1,
                    [("bob".parse().unwrap(), 1.try_into().unwrap())],
                    recovery_device_id,
                );
            });

            timestamp
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let shamir_recovery_certificates: Vec<_> = certifs_from_last_shamir_recovery(env);

    let err = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::RelatedUserAlreadyRevoked { user_revoked_on, .. }
            if user_revoked_on == timestamp
        )
    )
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

            let brief_certificate = match brief.certificate {
                libparsec_types::AnyArcCertificate::ShamirRecoveryBrief(mut c) => {
                    let c_mut = Arc::make_mut(&mut c);
                    c_mut.user_id = alice.user_id;
                    c_mut.dump_and_sign(&bob.signing_key)
                }
                _ => unreachable!(),
            };
            vec![brief_certificate.into(), share.signed]
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
async fn brief_not_for_us(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        let recovery_device_id = builder.new_device("bob").map(|e| e.device_id);

        builder.certificates_storage_fetch_certificates("alice@dev1");

        // Alice is not related to this shamir recovery, hence she shouldn't receive it
        builder.new_shamir_recovery(
            "bob",
            1,
            [("mallory".parse().unwrap(), 1.try_into().unwrap())],
            recovery_device_id,
        );
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let mut shamir_recovery_certificates = certifs_from_last_shamir_recovery(env);
    // Certificates are [<brief>, <share for Mallory>]
    shamir_recovery_certificates.pop(); // Remove Mallory share
    p_assert_eq!(shamir_recovery_certificates.len(), 1); // Sanity check

    let err = ops
        .add_certificates_batch(&[], &[], &shamir_recovery_certificates, &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::ShamirRecoveryUnrelatedToUs { .. }
        )
    );
}
