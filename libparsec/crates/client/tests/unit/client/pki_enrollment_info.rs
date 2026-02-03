// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    client::pki_enrollment_info::{PKIInfoItem, PkiEnrollmentInfoError},
    pki_enrollment_info,
};

use super::utils::client_factory;
use libparsec_client_connection::test_register_send_hook;
use libparsec_platform_pki::sign_message;
use libparsec_protocol::anonymous_cmds::{self, v5::pki_enrollment_info::PkiEnrollmentInfoStatus};

#[parsec_test(testbed = "minimal")]
#[case("submitted")]
#[case("rejected")]
#[case("cancelled")]
// #[case("accepted")] // TODO #11269 when pki set up in testbed
async fn ok(#[case] status: &str, env: &TestbedEnv) {
    let alice_device = env.local_device("alice@dev1");

    let alice_client = client_factory(&env.discriminant_dir, alice_device.clone()).await;

    let expected_submitted_on = DateTime::from_timestamp_micros(1668594983390001).unwrap();
    let enrollment_id = PKIEnrollmentID::from_hex("e1fe88bd0f054261887a6c8039710b40").unwrap();

    let expected_accepted_on = DateTime::from_timestamp_micros(1668594983390002).unwrap();
    let expected_cancelled_on = DateTime::from_timestamp_micros(1668594983390002).unwrap();
    let expected_rejected_on = DateTime::from_timestamp_micros(1668594983390002).unwrap();

    let pki_enrollment_item = match status {
        "accepted" => {
            let expected_accepted_on = DateTime::from_timestamp_micros(1668594983390002).unwrap();

            let expected_answer = PkiEnrollmentAnswerPayload {
                user_id: UserID::from_hex("9268b5acc07711f0ae7c2394da79527f").unwrap(),
                device_id: DeviceID::from_hex("a46105b6c07711f09e41f70f2e4e5650").unwrap(),
                device_label: DeviceLabel::try_from("new pki device").unwrap(),
                profile: UserProfile::Standard,
                root_verify_key: alice_device.root_verify_key().clone(),
            };

            let cert_ref = X509CertificateHash::fake_sha256().into();
            let (accept_payload_signature_algorithm, accept_payload_signature) =
                sign_message(&expected_answer.dump(), &cert_ref)
                    .await
                    .context("Failed to sign message")
                    .unwrap();

            PkiEnrollmentInfoStatus::Accepted {
                accepter_der_x509_certificate: b"a cert".as_ref().into(),
                accepter_intermediate_der_x509_certificates: [b"deadbeef".as_ref().into()].to_vec(),
                accept_payload: expected_answer.dump().into(),
                accept_payload_signature,
                accept_payload_signature_algorithm,
                submitted_on: expected_submitted_on,
                accepted_on: expected_accepted_on,
            }
        }
        "cancelled" => PkiEnrollmentInfoStatus::Cancelled {
            cancelled_on: expected_cancelled_on,
            submitted_on: expected_submitted_on,
        },
        "rejected" => PkiEnrollmentInfoStatus::Rejected {
            rejected_on: expected_rejected_on,
            submitted_on: expected_submitted_on,
        },
        "submitted" => PkiEnrollmentInfoStatus::Submitted {
            submitted_on: expected_submitted_on,
        },
        _ => unimplemented!(),
    };

    test_register_send_hook(&env.discriminant_dir, {
        move |_req: anonymous_cmds::latest::pki_enrollment_info::Req| {
            anonymous_cmds::latest::pki_enrollment_info::Rep::Ok(pki_enrollment_item)
        }
    });

    let enrollment_info = pki_enrollment_info(
        alice_client.config.clone(),
        ParsecPkiEnrollmentAddr::new(
            alice_client.organization_addr(),
            alice_client.organization_id().clone(),
        ),
        X509CertificateHash::fake_sha256().into(),
        enrollment_id,
    )
    .await
    .unwrap();
    match (enrollment_info, status) {
        (
            PKIInfoItem::Accepted {
                accepted_on,
                submitted_on,
                ..
            },
            "accepted",
        ) => {
            // TODO check answer
            assert_eq!(expected_accepted_on, accepted_on);
            assert_eq!(expected_submitted_on, submitted_on);
        }
        (
            PKIInfoItem::Cancelled {
                submitted_on,
                cancelled_on,
            },
            "cancelled",
        ) => {
            assert_eq!(expected_cancelled_on, cancelled_on);
            assert_eq!(expected_submitted_on, submitted_on);
        }
        (PKIInfoItem::Submitted { submitted_on }, "submitted") => {
            assert_eq!(expected_submitted_on, submitted_on);
        }
        (
            PKIInfoItem::Rejected {
                rejected_on,
                submitted_on,
            },
            "rejected",
        ) => {
            assert_eq!(expected_rejected_on, rejected_on);
            assert_eq!(expected_submitted_on, submitted_on);
        }
        _ => panic!("unexpected answer"),
    }
}

#[parsec_test(testbed = "coolorg")]
async fn enrollment_not_found(env: &TestbedEnv) {
    let mallory_device = env.local_device("mallory@dev1");
    let mallory_client = client_factory(&env.discriminant_dir, mallory_device).await;
    test_register_send_hook(&env.discriminant_dir, {
        move |_req: anonymous_cmds::latest::pki_enrollment_info::Req| {
            anonymous_cmds::latest::pki_enrollment_info::Rep::EnrollmentNotFound
        }
    });

    let rep = pki_enrollment_info(
        mallory_client.config.clone(),
        ParsecPkiEnrollmentAddr::new(
            mallory_client.organization_addr(),
            mallory_client.organization_id().clone(),
        ),
        X509CertificateHash::fake_sha256().into(),
        PKIEnrollmentID::default(),
    )
    .await;
    p_assert_matches!(rep.unwrap_err(), PkiEnrollmentInfoError::EnrollmentNotFound)
}
