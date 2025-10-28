// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::DeviceInfo;

use libparsec_platform_device_loader::{get_default_key_file, load_device, DeviceSaveStrategy};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;

fn assert_device(info: &DeviceInfo, expected_device: impl TryInto<DeviceID>, env: &TestbedEnv) {
    let (certif, _) = env.get_device_certificate(expected_device);

    let DeviceInfo {
        id,
        device_label,
        purpose,
        created_on,
        created_by,
    } = info;
    p_assert_eq!(id, &certif.device_id);
    let expected_device_label = match &certif.device_label {
        MaybeRedacted::Real(device_label) => device_label,
        MaybeRedacted::Redacted(_) => unreachable!(),
    };
    p_assert_eq!(device_label, expected_device_label);
    p_assert_eq!(purpose, &certif.purpose);
    p_assert_eq!(created_on, &certif.timestamp);
    let expected_created_by = match &certif.author {
        CertificateSigner::User(author) => Some(author),
        CertificateSigner::Root => None,
    };
    p_assert_eq!(created_by.as_ref(), expected_created_by);
}

fn devices_belongs_to_same_user(dev1: &LocalDevice, dev2: &LocalDevice) -> bool {
    dev1.organization_addr == dev2.organization_addr
        && dev1.user_id == dev2.user_id
        && dev1.human_handle == dev2.human_handle
        && dev1.private_key == dev2.private_key
        && dev1.initial_profile == dev2.initial_profile
        && dev1.user_realm_id == dev2.user_realm_id
        && dev1.user_realm_key == dev2.user_realm_key
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok(env: &TestbedEnv) {
    // Part 1: create recovery device
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    let config = client.config.clone();
    let config_dir = config.config_dir.clone();
    let recovery_device_label = DeviceLabel::try_from("recovery").unwrap();
    let (passphrase, data) = client
        .export_recovery_device(recovery_device_label.clone())
        .await
        .unwrap();
    client.refresh_workspaces_list().await.unwrap();

    // but why do you test workspace count ?
    // it was the symptom of the keys not being
    // properly transmitted to the new devices
    let workspaces = client.list_workspaces().await;
    p_assert_eq!(workspaces.len(), 1, "{:?}", workspaces);

    let alice_devices = client
        .list_user_devices("alice".parse().unwrap())
        .await
        .unwrap();
    p_assert_eq!(alice_devices.len(), 3);
    assert_device(&alice_devices[0], "alice@dev1", env);
    assert_device(&alice_devices[1], "alice@dev2", env);
    assert_eq!(alice_devices[2].device_label, recovery_device_label);

    let devices = client.list_user_devices(client.user_id()).await.unwrap();
    p_assert_eq!(devices.len(), 3);
    p_assert_eq!(devices[2].purpose, DevicePurpose::PassphraseRecovery);

    //  we lose access
    client.stop().await;

    // Part 2: import recovery device to get new device
    let new_device_label = DeviceLabel::try_from("new_device").unwrap();
    let save_strategy = DeviceSaveStrategy::Keyring;

    let saved_device = libparsec_client::import_recovery_device(
        &config_dir,
        &data,
        passphrase.to_string(),
        new_device_label.clone(),
        save_strategy.clone(),
    )
    .await
    .unwrap();

    // Part 3: connect with new device

    let access = {
        let key_file = get_default_key_file(&env.discriminant_dir, saved_device.device_id);
        save_strategy.into_access(key_file)
    };
    let new_device = load_device(&env.discriminant_dir, &access).await.unwrap();
    let client = client_factory(&env.discriminant_dir, new_device.clone()).await;

    client.poll_server_for_new_certificates().await.unwrap();

    let devices = client.list_user_devices(client.user_id()).await.unwrap();
    p_assert_eq!(devices.len(), 4);
    p_assert_eq!(devices[3].purpose, DevicePurpose::Standard);

    let alice_devices = client.list_user_devices(client.user_id()).await.unwrap();
    p_assert_eq!(alice_devices.len(), 4);

    assert_device(&alice_devices[0], "alice@dev1", env);
    assert_device(&alice_devices[1], "alice@dev2", env);
    assert_eq!(alice_devices[2].device_label, recovery_device_label);
    assert_eq!(alice_devices[3].device_label, new_device_label);

    assert!(devices_belongs_to_same_user(&alice, &new_device));

    client.refresh_workspaces_list().await.unwrap();
    client.ensure_workspaces_bootstrapped().await.unwrap();

    let workspaces = client.list_workspaces().await;
    p_assert_eq!(workspaces.len(), 1, "{:?}", workspaces);
}
