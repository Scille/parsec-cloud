// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]
// TODO: Web support is not implemented
#![cfg(not(target_arch = "wasm32"))]

use libparsec_platform_device_loader::list_available_devices;
use libparsec_testbed::TestbedEnv;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test]
#[case::unknown_path(false)]
#[case::existing_path(true)]
async fn list_no_devices(tmp_path: TmpPath, #[case] path_exists: bool) {
    if path_exists {
        std::fs::create_dir(tmp_path.join("devices")).unwrap();
    }

    let devices = list_available_devices(&tmp_path).await;
    p_assert_eq!(devices, []);
}

#[parsec_test]
async fn ignore_invalid_items(tmp_path: TmpPath) {
    let devices_dir = tmp_path.join("devices");
    std::fs::create_dir(&devices_dir).unwrap();

    // Also add dummy stuff that should be ignored

    // Empty folder
    std::fs::File::create(devices_dir.join("bad1")).unwrap();
    // Dummy file
    std::fs::write(
        devices_dir.join("1797e0d4507cf1b7d0706876840d8b3105edecd61066c3c6a9c3c194eeadea29.key"),
        b"dummy",
    )
    .unwrap();
    // Folder with dummy file
    {
        let dummy_dir =
            devices_dir.join("1797e0d4507cf1b7d0706876840d8b3105edecd61066c3c6a9c3c194eeadea29");
        std::fs::create_dir(&dummy_dir).unwrap();
        std::fs::write(
            dummy_dir.join("1797e0d4507cf1b7d0706876840d8b3105edecd61066c3c6a9c3c194eeadea29.key"),
            b"dummy",
        )
        .unwrap();
    }

    let devices = list_available_devices(&tmp_path).await;
    p_assert_eq!(devices, []);
}

#[parsec_test]
async fn list_devices(tmp_path: TmpPath) {
    let alice_file_path = tmp_path
        .join("devices/52e905dc5a505f068cbec94298768f877054016080c0a0d09992730385966db6.keys");
    // Device must have a .keys extension, but can be in nested directories with a random name
    let bob_file_path = tmp_path.join("devices/foo/bar/spam/whatever.keys");
    let mallory_file_path = tmp_path.join("devices/foo/whatever.keys");

    // Generated from Rust implementation (Parsec v3.2.5-a.0+dev)
    // let alice_device = DeviceFile::Password(DeviceFilePassword {
    //     created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
    //     protected_on: "2000-01-10T00:00:00Z".parse().unwrap(),
    //     server_url: "https://parsec.invalid".into(),
    //     organization_id: "CoolOrg".parse().unwrap(),
    //     user_id: "alice".parse().unwrap(),
    //     device_id: "alice@dev1".parse().unwrap(),
    //     human_handle: "Alicey McAliceFace <alice@example.com>".parse().unwrap(),
    //     device_label: "My dev1 machine".parse().unwrap(),
    //     algorithm: DeviceFilePasswordAlgorithm::Argon2id {
    //         salt: Bytes::from_static(b"salt"),
    //         opslimit: 1,
    //         memlimit_kb: 8,
    //         parallelism: 1,
    //     },
    //     ciphertext: vec![].into(),
    // });
    // eprintln!("Alice blob: {}", hex::encode(alice_device.dump()));
    let alice_file_content = hex!(
        "8ba474797065a870617373776f7264aa637265617465645f6f6ed70100035d013b37e0"
        "00ac70726f7465637465645f6f6ed70100035db647ca4000aa7365727665725f75726c"
        "b668747470733a2f2f7061727365632e696e76616c6964af6f7267616e697a6174696f"
        "6e5f6964a7436f6f6c4f7267a7757365725f6964d802a11cec00100000000000000000"
        "000000a96465766963655f6964d802de10a11cec0010000000000000000000ac68756d"
        "616e5f68616e646c6592b1616c696365406578616d706c652e636f6db2416c69636579"
        "204d63416c69636546616365ac6465766963655f6c6162656caf4d792064657631206d"
        "616368696e65a9616c676f726974686d85a474797065a84152474f4e324944ab6d656d"
        "6c696d69745f6b6208a86f70736c696d697401ab706172616c6c656c69736d01a47361"
        "6c74c40473616c74aa63697068657274657874c400"
    );
    std::fs::create_dir_all(alice_file_path.parent().unwrap()).unwrap();
    std::fs::write(&alice_file_path, alice_file_content).unwrap();

    // Generated from Rust implementation (Parsec v3.2.5-a.0+dev)
    // let bob_device = DeviceFile::Smartcard(DeviceFileSmartcard {
    //     created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
    //     protected_on: "2000-01-10T00:00:00Z".parse().unwrap(),
    //     server_url: "https://parsec.invalid".into(),
    //     organization_id: "CoolOrg".parse().unwrap(),
    //     user_id: "bob".parse().unwrap(),
    //     device_id: "bob@dev2".parse().unwrap(),
    //     human_handle: "Boby McBobFace <bob@example.com>".parse().unwrap(),
    //     device_label: "My dev2 machine".parse().unwrap(),
    //     certificate_id: "".into(),
    //     certificate_sha1: None,
    //     encrypted_key: vec![].into(),
    //     ciphertext: vec![].into(),
    // });
    // eprintln!("Bob device: {}", hex::encode(bob_device.dump()));
    let bob_file_content = hex!(
        "8da474797065a9736d61727463617264aa637265617465645f6f6ed70100035d013b37"
        "e000ac70726f7465637465645f6f6ed70100035db647ca4000aa7365727665725f7572"
        "6cb668747470733a2f2f7061727365632e696e76616c6964af6f7267616e697a617469"
        "6f6e5f6964a7436f6f6c4f7267a7757365725f6964d802808c00100000000000000000"
        "00000000a96465766963655f6964d802de20808c001000000000000000000000ac6875"
        "6d616e5f68616e646c6592af626f62406578616d706c652e636f6dae426f6279204d63"
        "426f6246616365ac6465766963655f6c6162656caf4d792064657632206d616368696e"
        "65ae63657274696669636174655f6964a0b063657274696669636174655f73686131c0"
        "ad656e637279707465645f6b6579c400aa63697068657274657874c400"
    );
    std::fs::create_dir_all(bob_file_path.parent().unwrap()).unwrap();
    std::fs::write(&bob_file_path, bob_file_content).unwrap();

    // Generated from Rust implementation (Parsec v3.2.5-a.0+dev)
    // let mallory_device = DeviceFile::Keyring(DeviceFileKeyring {
    //     created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
    //     protected_on: "2000-01-10T00:00:00Z".parse().unwrap(),
    //     server_url: "https://parsec.invalid".into(),
    //     organization_id: "CoolOrg".parse().unwrap(),
    //     user_id: "mallory".parse().unwrap(),
    //     device_id: "mallory@dev2".parse().unwrap(),
    //     human_handle: "Mallory McMalloryFace <mallory@example.com>"
    //         .parse()
    //         .unwrap(),
    //     device_label: "My dev2 machine".parse().unwrap(),
    //     keyring_service: "parsec".into(),
    //     keyring_user: "mallory".into(),
    //     ciphertext: vec![].into(),
    // });
    // eprintln!("Mallory device: {}", hex::encode(mallory_device.dump()));
    let mallory_file_content = hex!(
        "8ca474797065a76b657972696e67aa637265617465645f6f6ed70100035d013b37e000"
        "ac70726f7465637465645f6f6ed70100035db647ca4000aa7365727665725f75726cb6"
        "68747470733a2f2f7061727365632e696e76616c6964af6f7267616e697a6174696f6e"
        "5f6964a7436f6f6c4f7267a7757365725f6964d8023a11031c00100000000000000000"
        "0000a96465766963655f6964d802de203a11031c00100000000000000000ac68756d61"
        "6e5f68616e646c6592b36d616c6c6f7279406578616d706c652e636f6db54d616c6c6f"
        "7279204d634d616c6c6f727946616365ac6465766963655f6c6162656caf4d79206465"
        "7632206d616368696e65af6b657972696e675f73657276696365a6706172736563ac6b"
        "657972696e675f75736572a76d616c6c6f7279aa63697068657274657874c400"
    );
    std::fs::create_dir_all(mallory_file_path.parent().unwrap()).unwrap();
    std::fs::write(&mallory_file_path, mallory_file_content).unwrap();

    let devices = list_available_devices(&tmp_path).await;

    let expected_devices = Vec::from([
        AvailableDevice {
            key_file_path: alice_file_path,
            created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
            protected_on: "2000-01-10T00:00:00Z".parse().unwrap(),
            server_url: "https://parsec.invalid".to_string(),
            organization_id: "CoolOrg".parse().unwrap(),
            user_id: "alice".parse().unwrap(),
            device_id: "alice@dev1".parse().unwrap(),
            human_handle: "Alicey McAliceFace <alice@example.com>".parse().unwrap(),
            device_label: "My dev1 machine".parse().unwrap(),
            ty: DeviceFileType::Password,
        },
        AvailableDevice {
            key_file_path: bob_file_path,
            created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
            protected_on: "2000-01-10T00:00:00Z".parse().unwrap(),
            server_url: "https://parsec.invalid".to_string(),
            organization_id: "CoolOrg".parse().unwrap(),
            user_id: "bob".parse().unwrap(),
            device_id: "bob@dev2".parse().unwrap(),
            human_handle: "Boby McBobFace <bob@example.com>".parse().unwrap(),
            device_label: "My dev2 machine".parse().unwrap(),
            ty: DeviceFileType::Smartcard,
        },
        AvailableDevice {
            key_file_path: mallory_file_path,
            created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
            protected_on: "2000-01-10T00:00:00Z".parse().unwrap(),
            server_url: "https://parsec.invalid".to_string(),
            organization_id: "CoolOrg".parse().unwrap(),
            user_id: "mallory".parse().unwrap(),
            device_id: "mallory@dev2".parse().unwrap(),
            human_handle: "Mallory McMalloryFace <mallory@example.com>"
                .parse()
                .unwrap(),
            device_label: "My dev2 machine".parse().unwrap(),
            ty: DeviceFileType::Keyring,
        },
    ]);

    p_assert_eq!(devices, expected_devices);
}

#[parsec_test(testbed = "empty")]
async fn testbed(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.bootstrap_organization("alice"); // alice@dev1
        builder.new_user("bob"); // bob@dev1
        builder.new_device("alice"); // alice@dev2
        builder.new_device("bob"); // bob@dev2
        builder.new_user("mallory"); // mallory@dev1
    })
    .await;
    let devices = list_available_devices(&env.discriminant_dir).await;
    p_assert_eq!(
        devices
            .into_iter()
            .map(|a| a.device_id)
            .collect::<Vec<DeviceID>>(),
        [
            "alice@dev1".parse().unwrap(),
            "bob@dev1".parse().unwrap(),
            "alice@dev2".parse().unwrap(),
            "bob@dev2".parse().unwrap(),
            "mallory@dev1".parse().unwrap(),
        ]
    );
}
