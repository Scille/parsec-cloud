// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::super::utils::client_factory;
use libparsec_client_connection::{protocol, test_register_send_hook};
use libparsec_tests_fixtures::prelude::*;

use crate::ClientListAsyncEnrollmentsError;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: protocol::authenticated_cmds::latest::async_enrollment_list::Req| {
            protocol::authenticated_cmds::latest::async_enrollment_list::Rep::Ok {
                enrollments: Vec::new(),
            }
        },
    );

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    p_assert_eq!(client.list_async_enrollments().await.unwrap(), vec![]);
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    p_assert_matches!(
        client.list_async_enrollments().await,
        Err(ClientListAsyncEnrollmentsError::Offline(_))
    )
}

#[parsec_test(testbed = "minimal")]
async fn author_not_allowed(env: &TestbedEnv) {
    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: protocol::authenticated_cmds::latest::async_enrollment_list::Req| {
            protocol::authenticated_cmds::latest::async_enrollment_list::Rep::AuthorNotAllowed
        },
    );

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    p_assert_matches!(
        client.list_async_enrollments().await,
        Err(ClientListAsyncEnrollmentsError::AuthorNotAllowed)
    );
}
