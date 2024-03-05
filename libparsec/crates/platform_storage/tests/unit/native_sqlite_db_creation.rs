// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::{tmp_path, TmpPath};
use libparsec_tests_lite::parsec_test;
use libparsec_types::*;

use crate::platform::certificates::PlatformCertificatesStorage;

#[parsec_test]
async fn should_create_db_file_and_reusable(tmp_path: TmpPath) {
    let device = LocalDevice::generate_new_device(
        ParsecOrganizationAddr::from_any("parsec3://127.0.0.1:6770/Org?no_ssl=true&rvk=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss").unwrap(),
        UserProfile::Admin,
        HumanHandle::new_redacted(&"alice".parse().unwrap()),
        DeviceLabel::new_redacted(&"device".parse().unwrap()),
        None,
        None,
        None,
    );

    let sqlite_path = tmp_path.join(device.slug()).join("certificates-v1.sqlite");

    assert!(!sqlite_path.exists());

    let storage = PlatformCertificatesStorage::no_populate_start(&tmp_path, &device)
        .await
        .unwrap();

    storage.stop().await.unwrap();

    assert!(sqlite_path.exists());

    let storage = PlatformCertificatesStorage::no_populate_start(&tmp_path, &device)
        .await
        .unwrap();

    storage.stop().await.unwrap();

    assert!(sqlite_path.exists());
}
