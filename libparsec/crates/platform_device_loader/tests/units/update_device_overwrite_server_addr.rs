// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use crate::{load_device, save_device, update_device_overwrite_server_addr, DeviceAccessStrategy};
use libparsec_testbed::TestbedEnv;
use libparsec_tests_fixtures::{tmp_path, TmpPath};
use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test]
async fn ok(tmp_path: TmpPath) {
    let key_file = tmp_path.join("devices/alice@dev1.keys");

    let initial_organization_addr = ParsecOrganizationAddr::from_any(
        // cspell:disable-next-line
        "parsec3://old.invalid/Org?no_ssl=true&p=xCD7SjlysFv3d4mTkRu-ZddRjIZPGraSjUnoOHT9s8rmLA",
    )
    .unwrap();
    let device = LocalDevice::generate_new_device(
        initial_organization_addr,
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
    let access_strategy =
        DeviceAccessStrategy::new_password(key_file.clone(), "P@ssw0rd.".to_owned().into());
    let save_strategy = access_strategy.clone().into();
    save_device(Path::new(""), &save_strategy, &device, key_file)
        .await
        .unwrap();

    let new_server_addr: ParsecAddr = "parsec3://new.invalid".parse().unwrap();

    update_device_overwrite_server_addr(Path::new(""), &access_strategy, new_server_addr)
        .await
        .unwrap();

    let expected_device = {
        let mut expected_device = device.clone();
        expected_device.organization_addr =
            "parsec3://new.invalid/Org?p=xCD7SjlysFv3d4mTkRu-ZddRjIZPGraSjUnoOHT9s8rmLA"
                .parse()
                .unwrap();
        expected_device
    };

    p_assert_eq!(
        *load_device(Path::new(""), &access_strategy).await.unwrap(),
        expected_device
    );

    let new_server_addr: ParsecAddr = "parsec3://new.invalid?no_ssl=true".parse().unwrap();

    update_device_overwrite_server_addr(Path::new(""), &access_strategy, new_server_addr)
        .await
        .unwrap();

    let expected_device = {
        let mut expected_device = device.clone();
        // cspell:disable-next-line
        expected_device.organization_addr = "parsec3://new.invalid/Org?no_ssl=true&p=xCD7SjlysFv3d4mTkRu-ZddRjIZPGraSjUnoOHT9s8rmLA".parse().unwrap();
        expected_device
    };

    p_assert_eq!(
        *load_device(Path::new(""), &access_strategy).await.unwrap(),
        expected_device
    );
}

#[parsec_test(testbed = "empty")]
async fn testbed(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.bootstrap_organization("alice"); // alice@dev1
    })
    .await;
    let alice = env.local_device("alice@dev1");

    let key_file = env
        .discriminant_dir
        .join("devices/de10a11cec0010000000000000000000.keys");

    let new_server_addr: ParsecAddr = "parsec3://new.invalid".parse().unwrap();
    let expected_new_organization_addr = ParsecOrganizationAddr::new(
        new_server_addr.clone(),
        alice.organization_addr.organization_id().clone(),
        alice.organization_addr.root_verify_key().clone(),
    );

    let access =
        DeviceAccessStrategy::new_password(key_file.clone(), "P@ssw0rd.".to_owned().into());

    update_device_overwrite_server_addr(&env.discriminant_dir, &access, new_server_addr)
        .await
        .unwrap();

    let device = load_device(&env.discriminant_dir, &access).await.unwrap();
    p_assert_eq!(device.organization_addr, expected_new_organization_addr,);
}
