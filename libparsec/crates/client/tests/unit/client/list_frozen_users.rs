// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;

use super::utils::client_factory;
use crate::ClientListFrozenUsersError;

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    assert_eq!(client.list_frozen_users().await.unwrap(), vec![]);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok_with_frozen(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.freeze_user(BOB_USER_ID);
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    assert_eq!(client.list_frozen_users().await.unwrap(), vec![BOB_USER_ID]);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn author_not_allowed(env: &TestbedEnv) {
    let mallory = env.local_device("mallory@dev1");
    let client = client_factory(&env.discriminant_dir, mallory).await;

    matches!(
        client.list_frozen_users().await.unwrap_err(),
        ClientListFrozenUsersError::AuthorNotAllowed
    );
}
