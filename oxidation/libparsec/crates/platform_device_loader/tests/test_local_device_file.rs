// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use rstest::rstest;
use std::collections::HashSet;

use libparsec_client_types::{
    AvailableDevice, DeviceFile, DeviceFilePassword, DeviceFileType, LocalDevice,
};
use libparsec_platform_device_loader::*;
use tests_fixtures::{alice, bob, mallory, tmp_path, Device, TmpPath};

fn device_file_factory(device: LocalDevice) -> DeviceFile {
    DeviceFile::Password(DeviceFilePassword {
        salt: b"salt".to_vec(),
        ciphertext: b"ciphertext".to_vec(),

        slug: device.slug(),
        organization_id: device.organization_id().clone(),
        device_id: device.device_id,

        human_handle: device.human_handle,
        device_label: device.device_label,
    })
}

#[rstest]
#[case(false)]
#[case(true)]
#[tokio::test]
async fn test_list_no_devices(tmp_path: TmpPath, #[case] path_exists: bool) {
    if path_exists {
        std::fs::create_dir(tmp_path.join("devices")).unwrap();
    }

    let devices = list_available_devices(&tmp_path).await;
    assert_eq!(devices, []);
}

#[rstest]
#[tokio::test]
async fn test_list_devices(tmp_path: TmpPath, alice: &Device, bob: &Device, mallory: &Device) {
    let alice = alice.local_device();
    let bob = bob.local_device();
    let mallory = mallory.local_device();

    let alice_device = device_file_factory(alice.clone());
    let bob_device = device_file_factory(bob.clone());
    let mallory_device = device_file_factory(mallory.clone());

    let alice_file_path = get_default_key_file(&tmp_path, &alice);
    // Device must have a .keys extension, but can be in nested directories with a random name
    let bob_file_path = tmp_path.join("devices/foo/whatever.keys");
    let mallory_file_path = tmp_path.join("devices/foo/bar/spam/whatever.keys");

    save_device_file(&alice_file_path, &alice_device).unwrap();
    save_device_file(&bob_file_path, &bob_device).unwrap();
    save_device_file(&mallory_file_path, &mallory_device).unwrap();

    // Also add dummy stuff that should be ignored
    let devices_dir = tmp_path.join("devices");
    std::fs::File::create(devices_dir.join("bad1")).unwrap();
    std::fs::create_dir(devices_dir.join("373955f566#corp#bob@laptop")).unwrap();
    let dummy_slug = String::from("a54ed6df3a#corp#alice@laptop");
    std::fs::create_dir(devices_dir.join(&dummy_slug)).unwrap();
    std::fs::write(
        devices_dir.join(&dummy_slug).join(dummy_slug + ".keys"),
        b"dummy",
    )
    .unwrap();

    let devices = list_available_devices(&tmp_path).await;

    let expected_devices = HashSet::from([
        AvailableDevice {
            key_file_path: alice_file_path,

            organization_id: alice.organization_id().clone(),
            device_id: alice.device_id.clone(),
            slug: alice.slug(),
            human_handle: alice.human_handle.clone(),
            device_label: alice.device_label.clone(),
            ty: DeviceFileType::Password,
        },
        AvailableDevice {
            key_file_path: bob_file_path,

            organization_id: bob.organization_id().clone(),
            device_id: bob.device_id.clone(),
            slug: bob.slug(),
            human_handle: bob.human_handle.clone(),
            device_label: bob.device_label.clone(),
            ty: DeviceFileType::Password,
        },
        AvailableDevice {
            key_file_path: mallory_file_path,

            organization_id: mallory.organization_id().clone(),
            device_id: mallory.device_id.clone(),
            slug: mallory.slug(),
            human_handle: mallory.human_handle.clone(),
            device_label: mallory.device_label.clone(),
            ty: DeviceFileType::Password,
        },
    ]);

    assert_eq!(HashSet::from_iter(devices), expected_devices);
}

#[rstest]
#[tokio::test]
async fn test_list_devices_support_legacy_file_without_labels(tmp_path: TmpPath) {
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

    assert_eq!(devices, [expected_device]);
}

#[rstest]
#[tokio::test]
async fn test_renew_legacy_file(tmp_path: TmpPath) {
    let legacy_device = hex!(
        "85a474797065a870617373776f7264a473616c74c40473616c74aa63697068657274657874"
        "c40a63697068657274657874ac68756d616e5f68616e646c65c0ac6465766963655f6c6162"
        "656cc0"
    );
    let slug = "9d84fbd57a#Org#Zack@PC1".to_string();
    let key_file_path = tmp_path.join("devices").join(slug.clone() + ".keys");

    std::fs::create_dir_all(tmp_path.join("devices")).unwrap();
    std::fs::write(&key_file_path, legacy_device).unwrap();

    let device = load_device_file(&key_file_path).unwrap();

    assert_eq!(
        device,
        DeviceFile::Password(DeviceFilePassword {
            ciphertext: hex!("63697068657274657874").to_vec(),
            salt: hex!("73616c74").to_vec(),
            organization_id: "Org".parse().unwrap(),
            device_id: "Zack@PC1".parse().unwrap(),
            human_handle: None,
            device_label: None,
            slug,
        })
    )
}
