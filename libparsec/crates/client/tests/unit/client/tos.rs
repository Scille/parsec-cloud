// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_client_connection::test_register_send_hook;
use libparsec_protocol::tos_cmds;
use libparsec_tests_fixtures::prelude::*;

use super::utils::client_factory;
use crate::{ClientAcceptTosError, ClientGetTosError, EventShouldRetryConnectionNow, Tos};

#[parsec_test(testbed = "minimal")]
async fn get_tos_ok(env: &TestbedEnv) {
    let tos_per_locale_urls = HashMap::from_iter([
        (
            "fr_CA".to_string(),
            "https://parsec.invalid/tos_fr.pdf".to_string(),
        ),
        (
            "en_CA".to_string(),
            "https://parsec.invalid/tos_en.pdf".to_string(),
        ),
    ]);
    let tos_updated_on: DateTime = "2000-01-01T00:00:00Z".parse().unwrap();

    test_register_send_hook(&env.discriminant_dir, {
        let tos_per_locale_urls = tos_per_locale_urls.clone();
        move |_req: tos_cmds::latest::tos_get::Req| tos_cmds::latest::tos_get::Rep::Ok {
            updated_on: tos_updated_on,
            per_locale_urls: tos_per_locale_urls,
        }
    });

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let tos = client.get_tos().await.unwrap();
    p_assert_eq!(
        tos,
        Tos {
            per_locale_urls: tos_per_locale_urls,
            updated_on: tos_updated_on
        }
    )
}

#[parsec_test(testbed = "minimal")]
async fn get_tos_no_tos(env: &TestbedEnv) {
    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: tos_cmds::latest::tos_get::Req| tos_cmds::latest::tos_get::Rep::NoTos,
    );

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let outcome = client.get_tos().await;
    p_assert_matches!(outcome, Err(ClientGetTosError::NoTos));
}

#[parsec_test(testbed = "minimal")]
async fn get_tos_offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let outcome = client.get_tos().await;
    p_assert_matches!(outcome, Err(ClientGetTosError::Offline(_)));
}

#[parsec_test(testbed = "minimal")]
async fn get_tos_stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let outcome = client.get_tos().await;
    p_assert_matches!(outcome, Err(ClientGetTosError::Offline(_)));
}

#[parsec_test(testbed = "minimal")]
async fn accept_tos_ok(env: &TestbedEnv) {
    let tos_updated_on: DateTime = "2000-01-01T00:00:00Z".parse().unwrap();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: tos_cmds::latest::tos_accept::Req| {
            p_assert_eq!(req.tos_updated_on, tos_updated_on);
            tos_cmds::latest::tos_accept::Rep::Ok
        },
    );

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let mut spy = client.event_bus.spy.start_expecting();

    let outcome = client.accept_tos(tos_updated_on).await;
    p_assert_matches!(outcome, Ok(()));

    spy.wait_and_assert_next(|_: &EventShouldRetryConnectionNow| {})
        .await;
}

#[parsec_test(testbed = "minimal")]
async fn accept_tos_no_tos(env: &TestbedEnv) {
    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: tos_cmds::latest::tos_accept::Req| tos_cmds::latest::tos_accept::Rep::NoTos,
    );

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let spy = client.event_bus.spy.start_expecting();

    let outcome = client
        .accept_tos("2000-01-01T00:00:00Z".parse().unwrap())
        .await;
    p_assert_matches!(outcome, Err(ClientAcceptTosError::NoTos));

    spy.assert_no_events();
}

#[parsec_test(testbed = "minimal")]
async fn accept_tos_tos_mismatch(env: &TestbedEnv) {
    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: tos_cmds::latest::tos_accept::Req| {
            tos_cmds::latest::tos_accept::Rep::TosMismatch
        },
    );

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let spy = client.event_bus.spy.start_expecting();

    let outcome = client
        .accept_tos("2000-01-01T00:00:00Z".parse().unwrap())
        .await;
    p_assert_matches!(outcome, Err(ClientAcceptTosError::TosMismatch));

    spy.assert_no_events();
}

#[parsec_test(testbed = "minimal")]
async fn accept_tos_offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let spy = client.event_bus.spy.start_expecting();

    let outcome = client
        .accept_tos("2000-01-01T00:00:00Z".parse().unwrap())
        .await;
    p_assert_matches!(outcome, Err(ClientAcceptTosError::Offline(_)));

    spy.assert_no_events();
}
