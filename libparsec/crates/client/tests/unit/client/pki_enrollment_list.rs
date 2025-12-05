// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{InvalidityReason, PkiEnrollmentListError, PkiEnrollmentListItem};
use libparsec_client_connection::test_register_send_hook;
use libparsec_platform_pki::test_fixture::{test_pki, TestPKI};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;

#[parsec_test(testbed = "minimal")]
async fn ok_untrusted(test_pki: &TestPKI, env: &TestbedEnv) {
    let alice_device = env.local_device("alice@dev1");
    let alice_client = client_factory(&env.discriminant_dir, alice_device).await;

    let payload = PkiEnrollmentSubmitPayload {
        // For this test, we do not need to keep the private part
        verify_key: SigningKey::generate().verify_key(),
        public_key: PrivateKey::generate().public_key(),
        // We reuse alice device label and human handle for simplicity
        device_label: alice_client.device_label().clone(),
    };

    let raw_payload = payload.dump();
    let valid_cert = &test_pki.cert["bob"];

    // Test data from libparsec/crates/protocol/tests/authenticated_cmds/v5/pki_enrollment_list.rs
    let valid_pki_enrollment_item =
        authenticated_cmds::latest::pki_enrollment_list::PkiEnrollmentListItem {
            der_x509_certificate: valid_cert.der_certificate.clone(),
            intermediate_der_x509_certificates: [b"deadbeef".as_ref().into()].to_vec(),
            enrollment_id: PKIEnrollmentID::from_hex("e1fe88bd0f054261887a6c8039710b40").unwrap(),
            payload: raw_payload.clone().into(),
            payload_signature: hex!("3c7369676e61747572653e").as_ref().into(),
            payload_signature_algorithm: PkiSignatureAlgorithm::RsassaPssSha256,
            submitted_on: DateTime::from_timestamp_micros(1668594983390001).unwrap(),
        };
    let invalid_pki_enrollment_item =
        authenticated_cmds::latest::pki_enrollment_list::PkiEnrollmentListItem {
            der_x509_certificate: hex!("3c78353039206365727469663e").as_ref().into(),
            intermediate_der_x509_certificates: [b"deadbeef".as_ref().into()].to_vec(),
            enrollment_id: PKIEnrollmentID::from_hex("e1fe88bd0f054261887a6c8039710b40").unwrap(),
            payload: raw_payload.into(),
            payload_signature: hex!("1234").as_ref().into(),
            payload_signature_algorithm: PkiSignatureAlgorithm::RsassaPssSha256,
            submitted_on: DateTime::from_timestamp_micros(1668594983390001).unwrap(),
        };
    let valid_pki_enrollment_item_clone = valid_pki_enrollment_item.clone();
    let invalid_pki_enrollment_item_clone = invalid_pki_enrollment_item.clone();

    test_register_send_hook(&env.discriminant_dir, {
        move |_req: authenticated_cmds::latest::pki_enrollment_list::Req| {
            authenticated_cmds::latest::pki_enrollment_list::Rep::Ok {
                enrollments: vec![valid_pki_enrollment_item, invalid_pki_enrollment_item],
            }
        }
    });

    let enrollments = alice_client.pki_list_enrollments_untrusted().await.unwrap();
    p_assert_eq!(
        enrollments,
        vec![
            valid_pki_enrollment_item_clone,
            invalid_pki_enrollment_item_clone,
        ]
    )
}

#[ignore = "TODO: (#11269) Need to generate a valid payload & signature with a cert that is trusted by it's root certificate"]
#[parsec_test(testbed = "minimal")]
async fn ok(test_pki: &TestPKI, env: &TestbedEnv) {
    let alice_device = env.local_device("alice@dev1");
    let alice_client = client_factory(&env.discriminant_dir, alice_device).await;

    let payload = PkiEnrollmentSubmitPayload {
        // For this test, we do not need to keep the private part
        verify_key: SigningKey::generate().verify_key(),
        public_key: PrivateKey::generate().public_key(),
        // We reuse alice device label and human handle for simplicity
        device_label: alice_client.device_label().clone(),
    };

    let raw_payload = payload.dump();
    let valid_cert = &test_pki.cert["bob"];

    // Test data from libparsec/crates/protocol/tests/authenticated_cmds/v5/pki_enrollment_list.rs
    let valid_pki_enrollment_item =
        authenticated_cmds::latest::pki_enrollment_list::PkiEnrollmentListItem {
            der_x509_certificate: valid_cert.der_certificate.clone(),
            intermediate_der_x509_certificates: [b"deadbeef".as_ref().into()].to_vec(),
            enrollment_id: PKIEnrollmentID::from_hex("e1fe88bd0f054261887a6c8039710b40").unwrap(),
            payload: raw_payload.clone().into(),
            payload_signature: hex!("3c7369676e61747572653e").as_ref().into(),
            payload_signature_algorithm: PkiSignatureAlgorithm::RsassaPssSha256,
            submitted_on: DateTime::from_timestamp_micros(1668594983390001).unwrap(),
        };
    let invalid_pki_enrollment_item =
        authenticated_cmds::latest::pki_enrollment_list::PkiEnrollmentListItem {
            der_x509_certificate: hex!("3c78353039206365727469663e").as_ref().into(),
            intermediate_der_x509_certificates: [b"deadbeef".as_ref().into()].to_vec(),
            enrollment_id: PKIEnrollmentID::from_hex("e1fe88bd0f054261887a6c8039710b40").unwrap(),
            payload: raw_payload.into(),
            payload_signature: hex!("1234").as_ref().into(),
            payload_signature_algorithm: PkiSignatureAlgorithm::RsassaPssSha256,
            submitted_on: DateTime::from_timestamp_micros(1668594983390001).unwrap(),
        };
    let valid_pki_enrollment_item_clone = valid_pki_enrollment_item.clone();
    let invalid_pki_enrollment_item_clone = invalid_pki_enrollment_item.clone();

    test_register_send_hook(&env.discriminant_dir, {
        move |_req: authenticated_cmds::latest::pki_enrollment_list::Req| {
            authenticated_cmds::latest::pki_enrollment_list::Rep::Ok {
                enrollments: vec![valid_pki_enrollment_item, invalid_pki_enrollment_item],
            }
        }
    });

    let enrollments = alice_client.pki_list_enrollments(X509CertificateHash::fake_sha256().into());
    p_assert_eq!(
        enrollments.await.unwrap(),
        vec![
            PkiEnrollmentListItem::Valid {
                human_handle: valid_cert.cert_info.human_handle().unwrap(),
                enrollment_id: valid_pki_enrollment_item_clone.enrollment_id,
                submitted_on: valid_pki_enrollment_item_clone.submitted_on,
                submitter_der_cert: valid_pki_enrollment_item_clone.der_x509_certificate,
                payload
            },
            PkiEnrollmentListItem::Invalid {
                human_handle: None,
                enrollment_id: invalid_pki_enrollment_item_clone.enrollment_id,
                submitted_on: invalid_pki_enrollment_item_clone.submitted_on,
                details: "error".to_string(), // Update when real payload is generated
                reason: InvalidityReason::InvalidSignature
            }
        ]
    )
}

#[parsec_test(testbed = "minimal")]
async fn ok_empty(env: &TestbedEnv) {
    let alice_device = env.local_device("alice@dev1");
    let alice_client = client_factory(&env.discriminant_dir, alice_device).await;

    test_register_send_hook(&env.discriminant_dir, {
        move |_req: authenticated_cmds::latest::pki_enrollment_list::Req| {
            authenticated_cmds::latest::pki_enrollment_list::Rep::Ok {
                enrollments: vec![],
            }
        }
    });

    let enrollments = alice_client.pki_list_enrollments(X509CertificateHash::fake_sha256().into());
    p_assert_eq!(enrollments.await.unwrap(), vec![])
}

#[parsec_test(testbed = "coolorg")]
async fn author_not_allowed(env: &TestbedEnv) {
    let mallory_device = env.local_device("mallory@dev1");
    let mallory_client = client_factory(&env.discriminant_dir, mallory_device).await;

    matches!(
        mallory_client
            .pki_list_enrollments(X509CertificateHash::fake_sha256().into())
            .await
            .unwrap_err(),
        PkiEnrollmentListError::AuthorNotAllowed
    );
}
