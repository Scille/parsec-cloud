// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_platform_device_loader::{load_device, LoadDeviceError};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

// TODO: Additional tests to write !
// - load ok
// - load ok (relative path in access, hence config_dir is used)
// - load ok (legacy format)
// - bad password

enum BadPathKind {
    UnknownPath,
    ExistingParent,
    NotAFile,
}

#[parsec_test]
#[case::unknown_path(BadPathKind::UnknownPath)]
#[case::existing_parent(BadPathKind::ExistingParent)]
#[case::not_a_file(BadPathKind::NotAFile)]
async fn bad_path(tmp_path: TmpPath, #[case] kind: BadPathKind) {
    let key_file = tmp_path.join("devices/my_device.keys");
    match kind {
        BadPathKind::UnknownPath => (),
        BadPathKind::ExistingParent => {
            std::fs::create_dir_all(key_file.parent().unwrap()).unwrap();
        }
        BadPathKind::NotAFile => {
            std::fs::create_dir_all(&key_file).unwrap();
        }
    };

    let access = DeviceAccessStrategy::Password {
        key_file,
        password: "P@ssw0rd.".to_owned().into(),
    };
    let outcome = load_device(&tmp_path, &access).await;
    p_assert_matches!(outcome, Err(LoadDeviceError::InvalidPath(_)));
}

#[parsec_test]
async fn bad_file_content(tmp_path: TmpPath) {
    let key_file = tmp_path.join("devices/my_device.keys");
    std::fs::create_dir_all(key_file.parent().unwrap()).unwrap();
    std::fs::write(&key_file, b"<dummy>").unwrap();

    let access = DeviceAccessStrategy::Password {
        key_file,
        password: "P@ssw0rd.".to_owned().into(),
    };
    let outcome = load_device(&tmp_path, &access).await;
    p_assert_matches!(outcome, Err(LoadDeviceError::InvalidData));
}

#[parsec_test]
#[case::new_format(
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   type: "password"
    //   salt: b"salt"  <== Salt is too small
    //   ciphertext: b"ciphertext"  <== Dummy data never considered given salt is invalid
    //   human_handle: ["alice@example.com", "Alicey McAliceFace"]
    //   device_label: "My dev1 machine"
    //   device_id: "alice@dev1"
    //   organization_id: "CoolOrg"
    //   slug: "f78292422e#CoolOrg#alice@dev1"
    &hex!(
        "88a474797065a870617373776f7264aa63697068657274657874c40a63697068657274657874ac"
        "68756d616e5f68616e646c6592b1616c696365406578616d706c652e636f6db2416c6963657920"
        "4d63416c69636546616365ac6465766963655f6c6162656caf4d792064657631206d616368696e"
        "65a96465766963655f6964aa616c6963654064657631af6f7267616e697a6174696f6e5f6964a7"
        "436f6f6c4f7267a4736c7567bd6637383239323432326523436f6f6c4f726723616c6963654064"
        "657631a473616c74c40473616c74"
    )
)]
// TODO: human handle / device label legacy default not implemented yet !
#[case::legacy_format(
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   type: "password"
    //   salt: b"salt"  <== Salt is too small
    //   ciphertext: b"ciphertext"  <== Dummy data never considered given salt is invalid
    //   human_handle: None
    //   device_label: None
    &hex!(
        "85a474797065a870617373776f7264a473616c74c40473616c74aa63697068657274657874"
        "c40a63697068657274657874ac68756d616e5f68616e646c65c0ac6465766963655f6c6162"
        "656cc0"
    )
)]
async fn invalid_salt_size(tmp_path: TmpPath, #[case] content: &[u8]) {
    // Store it in a path compatible with the legacy format
    let key_file =
        tmp_path.join("devices/c17fc4c8bf#corp#alice@laptop/c17fc4c8bf#corp#alice@laptop.keys");
    std::fs::create_dir_all(key_file.parent().unwrap()).unwrap();
    std::fs::write(&key_file, content).unwrap();

    let access = DeviceAccessStrategy::Password {
        key_file,
        password: "P@ssw0rd.".to_owned().into(),
    };
    let outcome = load_device(&tmp_path, &access).await;
    p_assert_matches!(outcome, Err(LoadDeviceError::InvalidData));
}

#[parsec_test(testbed = "empty")]
async fn testbed(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.bootstrap_organization("alice"); // alice@dev1
    });
    let key_file = env.discriminant_dir.join("devices/alice@dev1.keys");
    let access = DeviceAccessStrategy::Password {
        key_file,
        password: "P@ssw0rd.".to_owned().into(),
    };
    let device = load_device(&env.discriminant_dir, &access).await.unwrap();
    p_assert_eq!(device.device_id, "alice@dev1".parse().unwrap());
}
