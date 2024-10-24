// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(clippy::unwrap_used)]

use libparsec_platform_device_loader::{
    inner_export_recovery_device, inner_import_recovery_device,
};
use libparsec_tests_lite::parsec_test;
use libparsec_types::HumanHandle;
use libparsec_types::ParsecOrganizationAddr;
use libparsec_types::UserProfile;
use libparsec_types::{DeviceLabel, LocalDevice};

#[parsec_test]
async fn test_ok_recovery() {
    let url = ParsecOrganizationAddr::from_any(
        // cspell:disable-next-line
        "parsec3://127.0.0.1:6770/Org?no_ssl=true&p=xCD7SjlysFv3d4mTkRu-ZddRjIZPGraSjUnoOHT9s8rmLA",
    )
    .unwrap();
    let local_device = LocalDevice::generate_new_device(
        url,
        UserProfile::Admin,
        HumanHandle::new("alice@dev1", "alice").unwrap(),
        "alice label".parse().unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );
    let recovery_device_label = DeviceLabel::default();
    let (secret_passphrase, data, exported_recovery_device) =
        inner_export_recovery_device(&local_device, recovery_device_label.clone())
            .await
            .unwrap();
    let imported_recovery_device = inner_import_recovery_device(data, secret_passphrase)
        .await
        .unwrap();

    assert_eq!(exported_recovery_device, imported_recovery_device);
    assert_eq!(imported_recovery_device.device_label, recovery_device_label);
}
