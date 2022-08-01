// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashSet;

use libparsec_client_types::LocalDevice;
use rstest::rstest;

use libparsec_core::{
    get_default_key_file, list_available_devices, AvailableDevice, DeviceFile, DeviceFilePassword,
    DeviceFileType, LocalDeviceError,
};
use tests_fixtures::{alice, bob, mallory, tmp_path, Device, TmpPath};

fn device_file_factory(device: LocalDevice) -> DeviceFile {
    DeviceFile::Password(DeviceFilePassword {
        salt: b"salt".to_vec(),
        ciphertext: b"ciphertext".to_vec(),

        slug: Some(device.slug()),
        organization_id: device.organization_id().clone(),
        device_id: device.device_id,

        human_handle: device.human_handle,
        device_label: device.device_label,
    })
}

#[rstest]
#[case(false)]
#[case(true)]
fn test_list_no_devices(tmp_path: TmpPath, #[case] path_exists: bool) {
    if path_exists {
        std::fs::create_dir(tmp_path.join("devices")).unwrap();
    }

    match list_available_devices(&tmp_path) {
        Ok(devices) => assert_eq!(devices, []),
        Err(e) => assert_eq!(e, LocalDeviceError::Access(tmp_path.join("devices"))),
    }
}

#[rstest]
fn test_list_devices(tmp_path: TmpPath, alice: &Device, bob: &Device, mallory: &Device) {
    let alice = alice.local_device();
    let bob = bob.local_device();
    let mallory = mallory.local_device();

    let alice_device = device_file_factory(alice.clone());
    let bob_device = device_file_factory(bob.clone());
    let mallory_device = device_file_factory(mallory.clone());

    let alice_file_path = get_default_key_file(&tmp_path, &alice);
    let bob_file_path = get_default_key_file(&tmp_path, &bob);
    let mallory_file_path = get_default_key_file(&tmp_path, &mallory);

    alice_device.save(&alice_file_path).unwrap();
    bob_device.save(&bob_file_path).unwrap();
    mallory_device.save(&mallory_file_path).unwrap();

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

    let devices = list_available_devices(tmp_path.as_path()).unwrap();

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
fn test_list_devices_support_legacy_file_without_labels(tmp_path: TmpPath, alice: &Device) {
    let device = alice.local_device();

    let device = DeviceFile::Password(DeviceFilePassword {
        salt: b"salt".to_vec(),
        ciphertext: b"ciphertext".to_vec(),

        slug: None,
        organization_id: device.organization_id().clone(),
        device_id: device.device_id,

        human_handle: None,
        device_label: None,
    });
    let slug = "9d84fbd57a#Org#Zack@PC1".to_string();
    let key_file_path = tmp_path.join("devices").join(slug.clone() + ".keys");
    device.save(&key_file_path).unwrap();

    let devices = list_available_devices(&tmp_path).unwrap();
    let expected_device = AvailableDevice {
        key_file_path,
        organization_id: alice.organization_id().clone(),
        device_id: alice.device_id.clone(),
        human_handle: None,
        device_label: None,
        slug,
        ty: DeviceFileType::Password,
    };

    assert_eq!(devices, [expected_device]);
}

#[rstest]
fn test_available_device_display(tmp_path: TmpPath, alice: &Device) {
    let alice = alice.local_device();

    let without_labels = AvailableDevice {
        key_file_path: get_default_key_file(&tmp_path, &alice),
        organization_id: alice.organization_id().clone(),
        device_id: alice.device_id.clone(),
        human_handle: None,
        device_label: None,
        slug: alice.slug(),
        ty: DeviceFileType::Password,
    };

    let with_labels = AvailableDevice {
        key_file_path: get_default_key_file(&tmp_path, &alice),
        organization_id: alice.organization_id().clone(),
        device_id: alice.device_id.clone(),
        human_handle: alice.human_handle.clone(),
        device_label: alice.device_label.clone(),
        slug: alice.slug(),
        ty: DeviceFileType::Password,
    };

    assert_eq!(
        without_labels.device_display(),
        alice.device_name().to_string()
    );
    assert_eq!(without_labels.user_display(), alice.user_id().to_string());

    assert_eq!(
        with_labels.device_display(),
        alice.device_label.unwrap().to_string()
    );
    assert_eq!(
        with_labels.user_display(),
        alice.human_handle.unwrap().to_string()
    );
}
