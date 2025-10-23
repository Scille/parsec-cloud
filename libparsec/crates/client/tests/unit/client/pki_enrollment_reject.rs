// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::client::{
    pki_enrollment_reject::ClientPkiEnrollmentRejectError, tests::utils::client_factory,
};
use libparsec_client_connection::test_register_send_hook;
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::TestbedEnv;
use libparsec_tests_lite::{p_assert_eq, parsec_test};
use libparsec_types::EnrollmentID;

#[parsec_test(testbed = "coolorg")]
async fn ok(env: &TestbedEnv) {
    let alice_device = env.local_device("alice@dev1");
    let alice_client = client_factory(&env.discriminant_dir, alice_device).await;

    let id = EnrollmentID::from_hex("c4d69794b01811f084d6bf6c412bee7c").unwrap();
    test_register_send_hook(&env.discriminant_dir, {
        move |req: authenticated_cmds::latest::pki_enrollment_reject::Req| {
            p_assert_eq!(req.enrollment_id, id);
            authenticated_cmds::latest::pki_enrollment_reject::Rep::Ok
        }
    });
    alice_client.pki_enrollment_reject(id).await.unwrap()
}

#[parsec_test(testbed = "coolorg")]
async fn author_not_allowed(env: &TestbedEnv) {
    let alice_device = env.local_device("alice@dev1");
    let alice_client = client_factory(&env.discriminant_dir, alice_device).await;

    let id = EnrollmentID::from_hex("c4d69794b01811f084d6bf6c412bee7c").unwrap();
    test_register_send_hook(&env.discriminant_dir, {
        move |req: authenticated_cmds::latest::pki_enrollment_reject::Req| {
            p_assert_eq!(req.enrollment_id, id);
            authenticated_cmds::latest::pki_enrollment_reject::Rep::AuthorNotAllowed
        }
    });
    assert!(matches!(
        alice_client.pki_enrollment_reject(id).await.unwrap_err(),
        ClientPkiEnrollmentRejectError::AuthorNotAllowed
    ))
}

#[parsec_test(testbed = "coolorg")]
async fn enrollment_not_found(env: &TestbedEnv) {
    let alice_device = env.local_device("alice@dev1");
    let alice_client = client_factory(&env.discriminant_dir, alice_device).await;

    let id = EnrollmentID::from_hex("c4d69794b01811f084d6bf6c412bee7c").unwrap();
    test_register_send_hook(&env.discriminant_dir, {
        move |req: authenticated_cmds::latest::pki_enrollment_reject::Req| {
            p_assert_eq!(req.enrollment_id, id);
            authenticated_cmds::latest::pki_enrollment_reject::Rep::EnrollmentNotFound
        }
    });
    assert!(matches!(
        alice_client.pki_enrollment_reject(id).await.unwrap_err(),
        ClientPkiEnrollmentRejectError::EnrollmentNotFound
    ))
}
#[parsec_test(testbed = "coolorg")]
async fn enrollment_no_longer_available(env: &TestbedEnv) {
    let alice_device = env.local_device("alice@dev1");
    let alice_client = client_factory(&env.discriminant_dir, alice_device).await;

    let id = EnrollmentID::from_hex("c4d69794b01811f084d6bf6c412bee7c").unwrap();
    test_register_send_hook(&env.discriminant_dir, {
        move |req: authenticated_cmds::latest::pki_enrollment_reject::Req| {
            p_assert_eq!(req.enrollment_id, id);
            authenticated_cmds::latest::pki_enrollment_reject::Rep::EnrollmentNoLongerAvailable
        }
    });
    assert!(matches!(
        alice_client.pki_enrollment_reject(id).await.unwrap_err(),
        ClientPkiEnrollmentRejectError::EnrollmentNoLongerAvailable
    ))
}
