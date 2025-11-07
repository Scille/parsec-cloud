// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{PkiEnrollmentListError, PkiEnrollmentListItem};
use libparsec_client_connection::test_register_send_hook;
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let alice_device = env.local_device("alice@dev1");
    let alice_client = client_factory(&env.discriminant_dir, alice_device).await;

    // Test data from libparsec/crates/protocol/tests/authenticated_cmds/v5/pki_enrollment_list.rs
    let pki_enrollment_item = PkiEnrollmentListItem {
        der_x509_certificate: hex!("3c78353039206365727469663e").as_ref().into(),
        enrollment_id: PKIEnrollmentID::from_hex("e1fe88bd0f054261887a6c8039710b40").unwrap(),
        payload: hex!("3c64756d6d793e").as_ref().into(),
        payload_signature: hex!("3c7369676e61747572653e").as_ref().into(),
        submitted_on: DateTime::from_timestamp_micros(1668594983390001).unwrap(),
    };
    let pki_enrollment_item_clone = pki_enrollment_item.clone();

    test_register_send_hook(&env.discriminant_dir, {
        move |_req: authenticated_cmds::latest::pki_enrollment_list::Req| {
            authenticated_cmds::latest::pki_enrollment_list::Rep::Ok {
                enrollments: vec![pki_enrollment_item],
            }
        }
    });

    let enrollments = alice_client.pki_list_enrollments();
    p_assert_eq!(enrollments.await.unwrap(), vec![pki_enrollment_item_clone])
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

    let enrollments = alice_client.pki_list_enrollments();
    p_assert_eq!(enrollments.await.unwrap(), vec![])
}

#[parsec_test(testbed = "coolorg")]
async fn author_not_allowed(env: &TestbedEnv) {
    let mallory_device = env.local_device("mallory@dev1");
    let mallory_client = client_factory(&env.discriminant_dir, mallory_device).await;

    matches!(
        mallory_client.pki_list_enrollments().await.unwrap_err(),
        PkiEnrollmentListError::AuthorNotAllowed
    );
}
