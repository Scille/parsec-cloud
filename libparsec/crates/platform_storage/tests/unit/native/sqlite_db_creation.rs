// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::{tmp_path, TmpPath};
use libparsec_tests_lite::parsec_test;
use libparsec_types::*;

use crate::platform::certificates::PlatformCertificatesStorage;

#[parsec_test]
async fn should_create_db_file_and_reusable(tmp_path: TmpPath) {
    let user_id = "alice".parse().unwrap();
    let device_id = "alice@dev1".parse().unwrap();
    let device = LocalDevice::generate_new_device(
        // cspell:disable-next-line
        "parsec3://127.0.0.1:6770/Org?no_ssl=true&p=xCBs8zpdIwovR8EdliVVo2vUOmtumnfsI6Fdndjm0WconA"
            .parse()
            .unwrap(),
        UserProfile::Admin,
        HumanHandle::new_redacted(user_id),
        DeviceLabel::new_redacted(device_id),
        Some(user_id),
        Some(device_id),
        None,
        None,
        None,
    );

    let sqlite_path = tmp_path
        .join(device.device_id.hex())
        .join("certificates-v1.sqlite");

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
