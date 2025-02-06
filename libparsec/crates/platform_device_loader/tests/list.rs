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
#[ignore = "TODO: scheme has changed, must regenerate the dump"]
async fn list_devices(tmp_path: TmpPath) {
    let alice_file_path = tmp_path
        .join("devices/52e905dc5a505f068cbec94298768f877054016080c0a0d09992730385966db6.keys");
    // Device must have a .keys extension, but can be in nested directories with a random name
    let bob_file_path = tmp_path.join("devices/foo/bar/spam/whatever.keys");
    let mallory_file_path = tmp_path.join("devices/foo/whatever.keys");

    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   type: "password"
    //   ciphertext: b"ciphertext"
    //   human_handle: ["alice@example.com", "Alicey McAliceFace"]
    //   device_label: "My dev1 machine"
    //   device_id: "alice@dev1"
    //   organization_id: "CoolOrg"
    //   slug: "f78292422e#CoolOrg#alice@dev1"
    //   algorithm:
    //     type: "ARGON2ID"
    //     salt: b"salt"
    //     opslimit: 1
    //     memlimit_kb: 8
    //     parallelism: 1
    let alice_file_content = hex!(
        "88a474797065a870617373776f7264aa63697068657274657874c40a63697068657274"
        "657874ac68756d616e5f68616e646c6592b1616c696365406578616d706c652e636f6d"
        "b2416c69636579204d63416c69636546616365ac6465766963655f6c6162656caf4d79"
        "2064657631206d616368696e65a96465766963655f6964aa616c6963654064657631af"
        "6f7267616e697a6174696f6e5f6964a7436f6f6c4f7267a4736c7567bd663738323932"
        "3432326523436f6f6c4f726723616c6963654064657631a9616c676f726974686d85a4"
        "74797065a84152474f4e324944a473616c74c40473616c74a86f70736c696d697401ab"
        "6d656d6c696d69745f6b6208ab706172616c6c656c69736d01"
    );
    std::fs::create_dir_all(alice_file_path.parent().unwrap()).unwrap();
    std::fs::write(&alice_file_path, alice_file_content).unwrap();

    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   type: "smartcard"
    //   encrypted_key: hex!("de5c59cfcc0c52bf997594e0fdd2c24ffee9465b6f25e30bac9238c2f83fd19a")
    //   certificate_id: "Bob's certificate"
    //   certificate_sha1: hex!("4682e01bc3e22fdfff1c33b551dfad8e49295005")
    //   ciphertext: b"ciphertext"
    //   human_handle: ["bob@example.com", "Boby McBobFace"]
    //   device_label: "My dev1 machine"
    //   device_id: "cc2578be6a174c9590a98b7d0d2c7e4f@6b92acd628294292a4b126b3e875c686"
    //   organization_id: "CoolOrg"
    //   slug: "f78292422e#CoolOrg#cc2578be6a174c9590a98b7d0d2c7e4f@6b92acd628294292a4b126b3e875c686"
    let bob_file_content = hex!(
        "8aa474797065a9736d61727463617264ad656e637279707465645f6b6579c440646535"
        "6335396366636330633532626639393735393465306664643263323466666565393436"
        "3562366632356533306261633932333863326638336664313961ae6365727469666963"
        "6174655f6964b1426f622773206365727469666963617465b063657274696669636174"
        "655f73686131c428343638326530316263336532326664666666316333336235353164"
        "66616438653439323935303035aa63697068657274657874c40a636970686572746578"
        "74ac68756d616e5f68616e646c6592af626f62406578616d706c652e636f6dae426f62"
        "79204d63426f6246616365ac6465766963655f6c6162656caf4d792064657632206d61"
        "6368696e65a96465766963655f6964d941636332353738626536613137346339353930"
        "6139386237643064326337653466403662393261636436323832393432393261346231"
        "323662336538373563363836af6f7267616e697a6174696f6e5f6964a7436f6f6c4f72"
        "67a4736c7567d9546637383239323432326523436f6f6c4f7267236363323537386265"
        "3661313734633935393061393862376430643263376534664036623932616364363238"
        "32393432393261346231323662336538373563363836"
    );
    std::fs::create_dir_all(bob_file_path.parent().unwrap()).unwrap();
    std::fs::write(&bob_file_path, bob_file_content).unwrap();

    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   type: "password"
    //   ciphertext: b"ciphertext"
    //   human_handle: ["mallory@example.com", "Mallory McMalloryFace"]
    //   device_label: "My dev3 machine"
    //   device_id: "mallory@dev3"
    //   organization_id: "CoolOrg"
    //   slug: "f78292422e#CoolOrg#mallory@dev3"
    //   algorithm:
    //     type: "ARGON2ID"
    //     salt: b"salt"
    //     opslimit: 1
    //     memlimit_kb: 8
    //     parallelism: 1
    let mallory_file_content = hex!(
        "88a474797065a870617373776f7264aa63697068657274657874c40a63697068657274"
        "657874ac68756d616e5f68616e646c6592b36d616c6c6f7279406578616d706c652e63"
        "6f6db54d616c6c6f7279204d634d616c6c6f727946616365ac6465766963655f6c6162"
        "656caf4d792064657633206d616368696e65a96465766963655f6964ac6d616c6c6f72"
        "794064657633af6f7267616e697a6174696f6e5f6964a7436f6f6c4f7267a4736c7567"
        "bf6637383239323432326523436f6f6c4f7267236d616c6c6f72794064657633a9616c"
        "676f726974686d85a474797065a84152474f4e324944a473616c74c40473616c74a86f"
        "70736c696d697401ab6d656d6c696d69745f6b6208ab706172616c6c656c69736d01"
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
            user_id: "cc2578be6a174c9590a98b7d0d2c7e4f".parse().unwrap(),
            device_id: "cc2578be6a174c9590a98b7d0d2c7e4f@6b92acd628294292a4b126b3e875c686"
                .parse()
                .unwrap(),
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
            device_id: "mallory@dev3".parse().unwrap(),
            human_handle: "Mallory McMalloryFace <mallory@example.com>"
                .parse()
                .unwrap(),
            device_label: "My dev3 machine".parse().unwrap(),
            ty: DeviceFileType::Password,
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
