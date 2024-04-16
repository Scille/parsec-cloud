// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_platform_device_loader::{change_authentication, load_device, save_device};
use libparsec_testbed::TestbedEnv;
use libparsec_tests_fixtures::{tmp_path, TmpPath};
use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test]
async fn same_key_file(tmp_path: TmpPath) {
    let key_file = tmp_path.join("devices/alice@dev1.keys");
    let access = DeviceAccessStrategy::Password {
        key_file: key_file.clone(),
        password: "P@ssw0rd.".to_owned().into(),
    };
    let device = LocalDevice::generate_new_device(
        ParsecOrganizationAddr::from_any(
            // cspell:disable-next-line
            "parsec3://127.0.0.1:6770/Org?no_ssl=true&p=xCD7SjlysFv3d4mTkRu-ZddRjIZPGraSjUnoOHT9s8rmLA"
        ).unwrap(),
        UserProfile::Admin,
        HumanHandle::new("alice@dev1", "alice").unwrap(),
        "alice label".parse().unwrap(),
        None,
        None,
        None,
    );

    // Sanity check
    assert!(!key_file.exists());

    save_device(Path::new(""), &access, &device).await.unwrap();

    // Sanity check
    assert!(key_file.exists());

    let new_access = DeviceAccessStrategy::Password {
        key_file: key_file.clone(),
        password: "P@ssw0rd1.".to_owned().into(),
    };
    change_authentication(Path::new(""), &access, &new_access)
        .await
        .unwrap();

    // Sanity check
    assert!(key_file.exists());

    p_assert_eq!(
        *load_device(Path::new(""), &new_access).await.unwrap(),
        device
    );

    load_device(Path::new(""), &access).await.unwrap_err();
}

#[parsec_test]
async fn different_key_file(tmp_path: TmpPath) {
    let key_file = tmp_path.join("devices/alice@dev1.keys");
    let access = DeviceAccessStrategy::Password {
        key_file: key_file.clone(),
        password: "P@ssw0rd.".to_owned().into(),
    };
    let device = LocalDevice::generate_new_device(
        ParsecOrganizationAddr::from_any(
            // cspell:disable-next-line
            "parsec3://127.0.0.1:6770/Org?no_ssl=true&p=xCD7SjlysFv3d4mTkRu-ZddRjIZPGraSjUnoOHT9s8rmLA"
        ).unwrap(),
        UserProfile::Admin,
        HumanHandle::new("alice@dev1", "alice").unwrap(),
        "alice label".parse().unwrap(),
        None,
        None,
        None,
    );

    // Sanity check
    assert!(!key_file.exists());

    save_device(Path::new(""), &access, &device).await.unwrap();

    // Sanity check
    assert!(key_file.exists());

    let new_key_file = tmp_path.join("devices/alice@dev2.keys");
    let new_access = DeviceAccessStrategy::Password {
        key_file: new_key_file.clone(),
        password: "P@ssw0rd.".to_owned().into(),
    };

    change_authentication(Path::new(""), &access, &new_access)
        .await
        .unwrap();

    // Sanity check
    assert!(!key_file.exists());
    assert!(new_key_file.exists());

    p_assert_eq!(
        *load_device(Path::new(""), &new_access).await.unwrap(),
        device
    );

    load_device(Path::new(""), &access).await.unwrap_err();
}

#[parsec_test(testbed = "empty")]
async fn testbed_same_key_file(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.bootstrap_organization("alice"); // alice@dev1
    });
    let key_file = env.discriminant_dir.join("devices/alice@dev1.keys");

    let current_access = DeviceAccessStrategy::Password {
        key_file: key_file.clone(),
        password: "P@ssw0rd.".to_owned().into(),
    };

    let new_access = DeviceAccessStrategy::Password {
        key_file,
        password: "P@ssw0rd1.".to_owned().into(),
    };
    change_authentication(&env.discriminant_dir, &current_access, &new_access)
        .await
        .unwrap();

    load_device(&env.discriminant_dir, &new_access)
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
    });
    let key_file = env.discriminant_dir.join("devices/alice@dev1.keys");
    let new_key_file = env.discriminant_dir.join("devices/alice@dev2.keys");

    let current_access = DeviceAccessStrategy::Password {
        key_file,
        password: "P@ssw0rd.".to_owned().into(),
    };

    let new_access = DeviceAccessStrategy::Password {
        key_file: new_key_file,
        password: "P@ssw0rd.".to_owned().into(),
    };
    change_authentication(&env.discriminant_dir, &current_access, &new_access)
        .await
        .unwrap();

    load_device(&env.discriminant_dir, &new_access)
        .await
        .unwrap();
    load_device(&env.discriminant_dir, &current_access)
        .await
        .unwrap_err();
}
