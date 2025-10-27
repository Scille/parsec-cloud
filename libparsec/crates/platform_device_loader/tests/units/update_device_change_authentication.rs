// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use crate::{load_device, save_device, update_device_change_authentication};
use libparsec_testbed::TestbedEnv;
use libparsec_tests_fixtures::{tmp_path, TmpPath};
use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use crate::tests::utils::key_present_in_system;

#[parsec_test]
async fn same_key_file(tmp_path: TmpPath) {
    let key_file = tmp_path.join("devices/alice@dev1.keys");
    let save_strategy = DeviceSaveStrategy::Password {
        password: "P@ssw0rd.".to_owned().into(),
    };
    let access_strategy = save_strategy.clone().into_access(key_file.clone());
    let url = ParsecOrganizationAddr::from_any(
        // cspell:disable-next-line
        "parsec3://test.invalid/Org?no_ssl=true&p=xCD7SjlysFv3d4mTkRu-ZddRjIZPGraSjUnoOHT9s8rmLA",
    )
    .unwrap();
    let device = LocalDevice::generate_new_device(
        url,
        UserProfile::Admin,
        HumanHandle::from_raw("alice@dev1", "alice").unwrap(),
        "alice label".parse().unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    // Sanity check
    assert!(!key_present_in_system(&key_file).await);

    TimeProvider::root_provider().mock_time_frozen("2000-01-01T00:00:00Z".parse().unwrap());
    let available1 = save_device(Path::new(""), &save_strategy, &device, key_file.clone())
        .await
        .unwrap();
    TimeProvider::root_provider().unmock_time();

    // Sanity check
    assert!(key_present_in_system(&key_file).await);
    p_assert_eq!(
        available1.created_on,
        "2000-01-01T00:00:00Z".parse().unwrap()
    );
    p_assert_eq!(
        available1.protected_on,
        "2000-01-01T00:00:00Z".parse().unwrap()
    );

    let new_save_strategy = DeviceSaveStrategy::Password {
        password: "P@ssw0rd1.".to_owned().into(),
    };
    let new_access_strategy = new_save_strategy.clone().into_access(key_file.clone());
    TimeProvider::root_provider().mock_time_frozen("2000-01-02T00:00:00Z".parse().unwrap());
    let available2 = update_device_change_authentication(
        Path::new(""),
        &access_strategy,
        &new_save_strategy,
        &key_file,
    )
    .await
    .unwrap();
    TimeProvider::root_provider().unmock_time();

    // Sanity check
    assert!(key_present_in_system(&key_file).await);
    let expected_available2 = {
        let mut expected = available1.clone();
        expected.protected_on = "2000-01-02T00:00:00Z".parse().unwrap();
        expected
    };
    p_assert_eq!(available2, expected_available2);

    p_assert_eq!(
        *load_device(Path::new(""), &new_access_strategy)
            .await
            .unwrap(),
        device
    );

    load_device(Path::new(""), &access_strategy)
        .await
        .unwrap_err();
}

#[parsec_test]
async fn different_key_file(tmp_path: TmpPath) {
    let key_file = tmp_path.join("devices/alice@dev1.keys");

    let save_strategy = DeviceSaveStrategy::Password {
        password: "P@ssw0rd.".to_owned().into(),
    };
    let access_strategy = save_strategy.clone().into_access(key_file.clone());
    let url = ParsecOrganizationAddr::from_any(
        // cspell:disable-next-line
        "parsec3://test.invalid/Org?no_ssl=true&p=xCD7SjlysFv3d4mTkRu-ZddRjIZPGraSjUnoOHT9s8rmLA",
    )
    .unwrap();
    let device = LocalDevice::generate_new_device(
        url,
        UserProfile::Admin,
        HumanHandle::from_raw("alice@dev1", "alice").unwrap(),
        "alice label".parse().unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    // Sanity check
    assert!(!key_present_in_system(&key_file).await);

    save_device(Path::new(""), &save_strategy, &device, key_file.clone())
        .await
        .unwrap();

    // Sanity check
    assert!(key_present_in_system(&key_file).await);

    let new_key_file = tmp_path.join("devices/alice@dev2.keys");
    let new_save_strategy = DeviceSaveStrategy::Password {
        password: "P@ssw0rd.".to_owned().into(),
    };
    let new_access_strategy = new_save_strategy.clone().into_access(new_key_file.clone());

    update_device_change_authentication(
        Path::new(""),
        &access_strategy,
        &new_save_strategy,
        &new_key_file,
    )
    .await
    .unwrap();

    // Sanity check
    assert!(!key_present_in_system(&key_file).await);
    assert!(key_present_in_system(&new_key_file).await);

    p_assert_eq!(
        *load_device(Path::new(""), &new_access_strategy)
            .await
            .unwrap(),
        device
    );

    load_device(Path::new(""), &access_strategy)
        .await
        .unwrap_err();
}

#[parsec_test(testbed = "empty")]
async fn testbed_same_key_file(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.bootstrap_organization("alice"); // alice@dev1
    })
    .await;
    let key_file = env.discriminant_dir.join("devices/alice@dev1.keys");

    let current_access = DeviceAccessStrategy::Password {
        key_file: key_file.clone(),
        password: "P@ssw0rd.".to_owned().into(),
    };

    let new_save_strategy = DeviceSaveStrategy::Password {
        password: "P@ssw0rd1.".to_owned().into(),
    };
    let new_access_strategy = new_save_strategy.clone().into_access(key_file.clone());
    update_device_change_authentication(
        &env.discriminant_dir,
        &current_access,
        &new_save_strategy,
        &key_file,
    )
    .await
    .unwrap();

    load_device(&env.discriminant_dir, &new_access_strategy)
        .await
        .unwrap();
    load_device(&env.discriminant_dir, &current_access)
        .await
        .unwrap_err();
}

#[parsec_test(testbed = "empty")]
async fn testbed_different_key_file(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.bootstrap_organization("alice"); // alice@dev1
    })
    .await;
    let key_file = env.discriminant_dir.join("devices/alice@dev1.keys");
    let new_key_file = env.discriminant_dir.join("devices/alice@dev2.keys");

    let current_access = DeviceAccessStrategy::Password {
        key_file,
        password: "P@ssw0rd.".to_owned().into(),
    };

    let new_save_strategy = DeviceSaveStrategy::Password {
        password: "P@ssw0rd.".to_owned().into(),
    };
    let new_access_strategy = new_save_strategy.clone().into_access(new_key_file.clone());

    update_device_change_authentication(
        &env.discriminant_dir,
        &current_access,
        &new_save_strategy,
        &new_key_file,
    )
    .await
    .unwrap();

    load_device(&env.discriminant_dir, &new_access_strategy)
        .await
        .unwrap();
    load_device(&env.discriminant_dir, &current_access)
        .await
        .unwrap_err();
}
