// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(clippy::unwrap_used)]

use crate::{dump_recovery_device, load_recovery_device, LoadRecoveryDeviceError};
use libparsec_tests_lite::p_assert_matches;
use libparsec_tests_lite::parsec_test;
use libparsec_types::HumanHandle;
use libparsec_types::LocalDevice;
use libparsec_types::ParsecOrganizationAddr;
use libparsec_types::UserProfile;

#[parsec_test]
async fn test_ok_recovery() {
    let url = ParsecOrganizationAddr::from_any(
        // cspell:disable-next-line
        "parsec3://test.invalid/Org?no_ssl=true&p=xCD7SjlysFv3d4mTkRu-ZddRjIZPGraSjUnoOHT9s8rmLA",
    )
    .unwrap();
    let local_device = LocalDevice::generate_new_device(
        url,
        UserProfile::Admin,
        HumanHandle::from_raw("alice@dev1", "alice").unwrap(),
        "alice label".parse().unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );
    let (secret_passphrase, data) = dump_recovery_device(&local_device);
    let imported_recovery_device = load_recovery_device(&data, secret_passphrase.clone()).unwrap();

    assert_eq!(*imported_recovery_device, local_device);

    // It's convenient to also test the import bad cases here

    // Invalid data
    p_assert_matches!(
        load_recovery_device(&b"dummy"[..], secret_passphrase.clone()),
        Err(LoadRecoveryDeviceError::InvalidData)
    );

    // Invalid passphrase

    p_assert_matches!(
        load_recovery_device(&data, "dummy".into()),
        Err(LoadRecoveryDeviceError::InvalidPassphrase)
    );

    // Decryption failed

    p_assert_matches!(
        load_recovery_device(
            &data,
            libparsec_types::SecretKey::generate_recovery_passphrase().0
        ),
        Err(LoadRecoveryDeviceError::DecryptionFailed)
    );
}
