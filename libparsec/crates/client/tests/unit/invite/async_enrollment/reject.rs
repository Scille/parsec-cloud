// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::super::utils::client_factory;
use libparsec_client_connection::{protocol, test_register_send_hook};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::ClientRejectAsyncEnrollmentError;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let enrollment_id = AsyncEnrollmentID::default();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::authenticated_cmds::latest::async_enrollment_reject::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::authenticated_cmds::latest::async_enrollment_reject::Rep::Ok
        },
    );

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    p_assert_matches!(client.reject_async_enrollment(enrollment_id).await, Ok(()));
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let enrollment_id = AsyncEnrollmentID::default();

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    p_assert_matches!(
        client.reject_async_enrollment(enrollment_id).await,
        Err(ClientRejectAsyncEnrollmentError::Offline(_))
    )
}

#[parsec_test(testbed = "minimal")]
async fn author_not_allowed(env: &TestbedEnv) {
    let enrollment_id = AsyncEnrollmentID::default();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::authenticated_cmds::latest::async_enrollment_reject::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::authenticated_cmds::latest::async_enrollment_reject::Rep::AuthorNotAllowed
        },
    );

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    p_assert_matches!(
        client.reject_async_enrollment(enrollment_id).await,
        Err(ClientRejectAsyncEnrollmentError::AuthorNotAllowed)
    );
}

#[parsec_test(testbed = "minimal")]
async fn enrollment_not_found(env: &TestbedEnv) {
    let enrollment_id = AsyncEnrollmentID::default();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::authenticated_cmds::latest::async_enrollment_reject::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::authenticated_cmds::latest::async_enrollment_reject::Rep::EnrollmentNotFound
        },
    );

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    p_assert_matches!(
        client.reject_async_enrollment(enrollment_id).await,
        Err(ClientRejectAsyncEnrollmentError::EnrollmentNotFound)
    );
}

#[parsec_test(testbed = "minimal")]
async fn enrollment_no_longer_available(env: &TestbedEnv) {
    let enrollment_id = AsyncEnrollmentID::default();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::authenticated_cmds::latest::async_enrollment_reject::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::authenticated_cmds::latest::async_enrollment_reject::Rep::EnrollmentNoLongerAvailable
        },
    );

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    p_assert_matches!(
        client.reject_async_enrollment(enrollment_id).await,
        Err(ClientRejectAsyncEnrollmentError::EnrollmentNoLongerAvailable)
    );
}
