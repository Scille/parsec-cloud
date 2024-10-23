// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::DeviceInfo;

use libparsec_platform_device_loader::{get_default_key_file, load_device};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;

fn assert_device(info: &DeviceInfo, expected_device: impl TryInto<DeviceID>, env: &TestbedEnv) {
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
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(dbg!(&env.discriminant_dir), alice).await;

    let config = client.config.clone();

    let recovery_device_label = dbg!(DeviceLabel::try_from("recovery").unwrap());
    let (passphrase, data) = client
        .export_recovery_device(recovery_device_label.clone())
        .await
        .unwrap();
    client.refresh_workspaces_list().await.unwrap();

    let workspaces = client.list_workspaces().await;
    p_assert_eq!(dbg!(&workspaces).len(), 1, "{:?}", workspaces);

    let alice_devices = client
        .list_user_devices("alice".parse().unwrap())
        .await
        .unwrap();
    p_assert_eq!(alice_devices.len(), 3);
    assert_device(&alice_devices[0], "alice@dev1", env);
    assert_device(&alice_devices[1], "alice@dev2", env);
    // assert_device(&alice_devices[2], dbg!(alice_devices[2].id), env);

    assert_eq!(dbg!(&alice_devices[2]).device_label, recovery_device_label);

    // // we lose access
    client.stop().await;
    let new_device_label = dbg!(DeviceLabel::try_from("new_device").unwrap());

    let recovery_device =
        libparsec_platform_device_loader::inner_import_recovery_device(data, passphrase)
            .await
            .unwrap();

    let client = client_factory(&env.discriminant_dir, recovery_device.clone().into()).await;

    assert_eq!(client.user_id(), "alice".parse().unwrap());

    // let alice_devices = client.list_user_devices(client.user_id()).await.unwrap();
    // p_assert_eq!(alice_devices.len(), 3);
    // assert_device(&alice_devices[0], "alice@dev1", env);
    // assert_device(&alice_devices[1], "alice@dev2", env);
    // assert_eq!(dbg!(&alice_devices[2]).device_label, recovery_device_label);

    let save_strategy = DeviceSaveStrategy::Keyring;

    let saved_device = client
        .create_device_from_recovery(&recovery_device, &new_device_label, save_strategy.clone())
        .await
        .unwrap();

    client.stop().await;
    let access = {
        let key_file = dbg!(get_default_key_file(
            &config.config_dir,
            &saved_device.device_id
        ));
        save_strategy.into_access(key_file)
    };
    let new_device = load_device(&config.config_dir, &access).await.unwrap();
    let client = client_factory(&env.discriminant_dir, new_device).await;
    // check from avavble device

    client.poll_server_for_new_certificates().await.unwrap();

    let alice_devices = client.list_user_devices(client.user_id()).await.unwrap();
    p_assert_eq!(alice_devices.len(), 4);
    assert_device(&alice_devices[0], "alice@dev1", env);
    assert_device(&alice_devices[1], "alice@dev2", env);
    assert_eq!(dbg!(&alice_devices[2]).device_label, recovery_device_label);
    assert_eq!(dbg!(&alice_devices[3]).device_label, new_device_label);

    client.refresh_workspaces_list().await.unwrap();
    client.ensure_workspaces_bootstrapped().await.unwrap();

    let workspaces = client.list_workspaces().await;
    p_assert_eq!(dbg!(&workspaces).len(), 1, "{:?}", workspaces);
    // assert_eq!(new_common_certificates.lock().unwrap().len(), 2);
}
