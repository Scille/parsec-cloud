// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use hex_literal::hex;

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

// Ignore the test on MacOS given it is based on creating a non-utf8 folder,
// which is not possible there (it causes an "Illegal byte sequence" error)
#[cfg(not(target_os = "macos"))]
// Also ignore the test on Windows: win32's `CreateDirectoryW` prevent from
// creating files with an invalid UTF-16 character.
#[cfg(not(target_family = "windows"))]
#[parsec_test]
async fn ignore_invalid_items(tmp_path: TmpPath) {
    let devices_dir = tmp_path.join("devices");
    std::fs::create_dir(&devices_dir).unwrap();

    // Also add dummy stuff that should be ignored

    // Empty folder
    std::fs::File::create(devices_dir.join("bad1")).unwrap();
    // Valid legacy key file dir, but not containing an actual key file
    std::fs::create_dir(devices_dir.join("373955f566#corp#bob@laptop")).unwrap();
    // Valid legacy key file, but with invalid content
    {
        let dummy_slug = String::from("a54ed6df3a#corp#alice@laptop");
        std::fs::create_dir(devices_dir.join(&dummy_slug)).unwrap();
        std::fs::write(
            devices_dir.join(&dummy_slug).join(dummy_slug + ".keys"),
            b"dummy",
        )
        .unwrap();
    }
    // Non-utf8 folder shouldn't break filename-to-slug parsing for legacy file
    let patch_into_non_utf8 = |x: &str| {
        assert!(x.is_ascii()); // Sanity check
        #[cfg(target_family = "unix")]
        {
            let mut buf = x.as_bytes().to_owned();
            buf[0] = 0xff; // 0xff is never valid in UTF-8
            use std::os::unix::ffi::OsStrExt;
            std::path::PathBuf::from(std::ffi::OsStr::from_bytes(&buf))
        }

        #[cfg(target_family = "windows")]
        {
            use std::os::windows::ffi::OsStringExt;
            // Convert UTF-8 into UTF-16 is easy as long as it is only composed of ASCII
            let mut buf: Vec<u16> = x.as_bytes().iter().map(|c| *c as u16).collect();
            buf[0] = 0xd800; // 0xd800 is high surrogate, but here it is lacking it pair
            let os_string = std::ffi::OsString::from_wide(&buf);
            std::path::PathBuf::from(os_string.as_os_str())
        }
    };
    {
        let non_utf8_folder_path =
            devices_dir.join(patch_into_non_utf8("b7619d20a5#corp#mallory@laptop"));
        std::fs::create_dir(non_utf8_folder_path).unwrap();
    }
    // Non-utf8 key file name shouldn't break filename-to-slug parsing for legacy file
    {
        // Generated from Rust implementation (Parsec v3.0.0+dev)
        // Content:
        //   type: "password"
        //   salt: b"salt"
        //   ciphertext: b"ciphertext"
        //   human_handle: None
        //   device_label: None
        let legacy_file_content = hex!(
            "85a474797065a870617373776f7264a473616c74c40473616c74aa63697068657274657874"
            "c40a63697068657274657874ac68756d616e5f68616e646c65c0ac6465766963655f6c6162"
            "656cc0"
        );

        let parent = devices_dir.join("c17fc4c8bf#corp#adam@laptop");
        std::fs::create_dir(&parent).unwrap();
        let non_utf8_file_path =
            parent.join(patch_into_non_utf8("c17fc4c8bf#corp#adam@laptop.keys"));
        std::fs::write(non_utf8_file_path, legacy_file_content).unwrap();
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

    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   type: "password"
    //   ciphertext: b"ciphertext"
    //   human_handle: ["alice@example.com", "Alicey McAliceFace"]
    //   device_label: "My dev1 machine"
    //   device_id: "alice@dev1"
    //   organization_id: "CoolOrg"
    //   slug: "f78292422e#CoolOrg#alice@dev1"
    //   salt: b"salt"
    let alice_file_content = hex!(
        "88a474797065a870617373776f7264aa63697068657274657874c40a63697068657274657874ac"
        "68756d616e5f68616e646c6592b1616c696365406578616d706c652e636f6db2416c6963657920"
        "4d63416c69636546616365ac6465766963655f6c6162656caf4d792064657631206d616368696e"
        "65a96465766963655f6964aa616c6963654064657631af6f7267616e697a6174696f6e5f6964a7"
        "436f6f6c4f7267a4736c7567bd6637383239323432326523436f6f6c4f726723616c6963654064"
        "657631a473616c74c40473616c74"
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
    //   human_handle: None
    //   device_label: None
    //   device_id: "cc2578be6a174c9590a98b7d0d2c7e4f@6b92acd628294292a4b126b3e875c686"
    //   organization_id: "CoolOrg"
    //   slug: "f78292422e#CoolOrg#cc2578be6a174c9590a98b7d0d2c7e4f@6b92acd628294292a4b126b3e875c686"
    let bob_file_content = hex!(
        "8aa474797065a9736d61727463617264ad656e637279707465645f6b6579c44064653563353963"
        "666363306335326266393937353934653066646432633234666665653934363562366632356533"
        "306261633932333863326638336664313961ae63657274696669636174655f6964b1426f622773"
        "206365727469666963617465b063657274696669636174655f73686131c4283436383265303162"
        "6333653232666466666631633333623535316466616438653439323935303035aa636970686572"
        "74657874c40a63697068657274657874ac68756d616e5f68616e646c65c0ac6465766963655f6c"
        "6162656cc0a96465766963655f6964d94163633235373862653661313734633935393061393862"
        "376430643263376534664036623932616364363238323934323932613462313236623365383735"
        "63363836af6f7267616e697a6174696f6e5f6964a7436f6f6c4f7267a4736c7567d95466373832"
        "39323432326523436f6f6c4f726723636332353738626536613137346339353930613938623764"
        "306432633765346640366239326163643632383239343239326134623132366233653837356336"
        "3836"
    );
    std::fs::create_dir_all(bob_file_path.parent().unwrap()).unwrap();
    std::fs::write(&bob_file_path, bob_file_content).unwrap();

    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   type: "password"
    //   ciphertext: b"ciphertext"
    //   human_handle: ["mallory@example.com", "Mallory McMalloryFace"]
    //   device_label: None
    //   device_id: "mallory@dev1"
    //   organization_id: "CoolOrg"
    //   slug: "f78292422e#CoolOrg#mallory@dev1"
    //   salt: b"salt"
    let mallory_file_content = hex!(
        "88a474797065a870617373776f7264aa63697068657274657874c40a63697068657274657874ac"
        "68756d616e5f68616e646c6592b36d616c6c6f7279406578616d706c652e636f6db54d616c6c6f"
        "7279204d634d616c6c6f727946616365ac6465766963655f6c6162656cc0a96465766963655f69"
        "64ac6d616c6c6f72794064657631af6f7267616e697a6174696f6e5f6964a7436f6f6c4f7267a4"
        "736c7567bf6637383239323432326523436f6f6c4f7267236d616c6c6f72794064657631a47361"
        "6c74c40473616c74"
    );
    std::fs::create_dir_all(mallory_file_path.parent().unwrap()).unwrap();
    std::fs::write(&mallory_file_path, mallory_file_content).unwrap();

    let devices = list_available_devices(&tmp_path).await;

    let expected_devices = Vec::from([
        AvailableDevice {
            key_file_path: alice_file_path,
            organization_id: "CoolOrg".parse().unwrap(),
            device_id: "alice@dev1".parse().unwrap(),
            slug: "f78292422e#CoolOrg#alice@dev1".to_owned(),
            human_handle: Some("Alicey McAliceFace <alice@example.com>".parse().unwrap()),
            device_label: Some("My dev1 machine".parse().unwrap()),
            ty: DeviceFileType::Password,
        },
        AvailableDevice {
            key_file_path: bob_file_path,
            organization_id: "CoolOrg".parse().unwrap(),
            device_id: "cc2578be6a174c9590a98b7d0d2c7e4f@6b92acd628294292a4b126b3e875c686".parse().unwrap(),
            slug: "f78292422e#CoolOrg#cc2578be6a174c9590a98b7d0d2c7e4f@6b92acd628294292a4b126b3e875c686".to_owned(),
            human_handle: None,
            device_label: None,
            ty: DeviceFileType::Smartcard,
        },
        AvailableDevice {
            key_file_path: mallory_file_path,
            organization_id: "CoolOrg".parse().unwrap(),
            device_id: "mallory@dev1".parse().unwrap(),
            slug: "f78292422e#CoolOrg#mallory@dev1".to_owned(),
            human_handle: Some("Mallory McMalloryFace <mallory@example.com>".parse().unwrap()),
            device_label: None,
            ty: DeviceFileType::Password,
        },
    ]);

    p_assert_eq!(devices, expected_devices);
}

#[parsec_test]
async fn list_devices_support_legacy_file_without_labels(tmp_path: TmpPath) {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   type: "password"
    //   salt: b"salt"
    //   ciphertext: b"ciphertext"
    //   human_handle: None
    //   device_label: None
    let legacy_device = hex!(
        "85a474797065a870617373776f7264a473616c74c40473616c74aa63697068657274657874"
        "c40a63697068657274657874ac68756d616e5f68616e646c65c0ac6465766963655f6c6162"
        "656cc0"
    );
    let slug = "9d84fbd57a#Org#Zack@PC1".to_string();
    let key_file_path = tmp_path.join("devices").join(slug.clone() + ".keys");

    std::fs::create_dir_all(tmp_path.join("devices")).unwrap();
    std::fs::write(&key_file_path, legacy_device).unwrap();

    let devices = list_available_devices(&tmp_path).await;
    let expected_device = AvailableDevice {
        key_file_path,
        organization_id: "Org".parse().unwrap(),
        device_id: "Zack@PC1".parse().unwrap(),
        human_handle: None,
        device_label: None,
        slug,
        ty: DeviceFileType::Password,
    };

    p_assert_eq!(devices, [expected_device]);
}

#[parsec_test(testbed = "empty")]
async fn testbed(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.bootstrap_organization("alice"); // alice@dev1
        builder.new_user("bob"); // bob@dev1
        builder.new_device("alice"); // alice@dev2
        builder.new_device("bob"); // bob@dev2
        builder.new_user("mallory"); // mallory@dev1
    });
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
