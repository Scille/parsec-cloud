// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;
use crate::{certif::DeviceInfo, client::ClientListUserDevicesError};

#[parsec_test(testbed = "coolorg")]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let assert_device = |info: &DeviceInfo, expected_device: &str| {
        let (certif, _) = env.get_device_certificate(expected_device);

        let DeviceInfo {
            id,
            device_label,
            created_on,
            created_by,
        } = info;
        p_assert_eq!(id, &certif.device_id);
        let expected_device_label = match &certif.device_label {
            MaybeRedacted::Real(device_label) => device_label,
            MaybeRedacted::Redacted(_) => unreachable!(),
        };
        p_assert_eq!(device_label, expected_device_label);
        p_assert_eq!(created_on, &certif.timestamp);
        let expected_created_by = match &certif.author {
            CertificateSignerOwned::User(author) => Some(author),
            CertificateSignerOwned::Root => None,
        };
        p_assert_eq!(created_by.as_ref(), expected_created_by);
    };

    let alice_devices = client
        .list_user_devices("alice".parse().unwrap())
        .await
        .unwrap();
    p_assert_eq!(alice_devices.len(), 2);
    assert_device(&alice_devices[0], "alice@dev1");
    assert_device(&alice_devices[1], "alice@dev2");

    let mallory_devices = client
        .list_user_devices("mallory".parse().unwrap())
        .await
        .unwrap();
    p_assert_eq!(mallory_devices.len(), 1);
    assert_device(&mallory_devices[0], "mallory@dev1");
}

#[parsec_test(testbed = "minimal")]
async fn unknown_user(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let unknown_user_devices = client
        .list_user_devices("unknown".parse().unwrap())
        .await
        .unwrap();
    p_assert_eq!(unknown_user_devices.len(), 0);
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    client.certificates_ops.stop().await.unwrap();

    let err = client
        .list_user_devices("alice".parse().unwrap())
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientListUserDevicesError::Stopped);
}
