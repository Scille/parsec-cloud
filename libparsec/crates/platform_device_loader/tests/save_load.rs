// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// TODO: implement web
#![cfg(not(target_arch = "wasm32"))]

use std::path::{Path, PathBuf};

use libparsec_platform_device_loader::{load_device, save_device};
use libparsec_tests_fixtures::{tmp_path, TmpPath};
use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test]
#[case::keyring(|key_file| DeviceAccessStrategy::Keyring { key_file })]
#[case::password(|key_file| DeviceAccessStrategy::Password {
    key_file,
    password: "P@ssw0rd.".to_string().into()
})]
async fn save_load(
    tmp_path: TmpPath,
    #[case] key_file_to_access: impl FnOnce(PathBuf) -> DeviceAccessStrategy,
) {
    let key_file = tmp_path.join("keyring_file");

    let device = LocalDevice::generate_new_device(
        ParsecOrganizationAddr::from_any(
            "parsec3://127.0.0.1:6770/Org?no_ssl=true&rvk=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss"
        ).unwrap(),
        UserProfile::Admin,
        HumanHandle::new("alice@dev1", "alice").unwrap(),
        "alice label".parse().unwrap(),
        None,
        None,
        None,
    );

    let access = key_file_to_access(key_file.clone());

    assert!(!key_file.exists());

    save_device(&tmp_path, &access, &device).await.unwrap();

    assert!(key_file.exists());

    let res = load_device(Path::new(""), &access).await.unwrap();

    p_assert_eq!(*res, device);
}
