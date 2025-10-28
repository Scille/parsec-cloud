// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use crate::{
    archive_device, list_available_devices, load_available_device, load_device, remove_device,
    save_device, tests::utils::MockedAccountVaultOperations, update_device_change_authentication,
    update_device_overwrite_server_addr, AvailableDevice, AvailableDeviceType,
    DeviceAccessStrategy, DeviceSaveStrategy,
};
use libparsec_testbed::TestbedEnv;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test]
#[case::unknown_path(false)]
#[cfg_attr(not(target_arch = "wasm32"), case::existing_path(true))]
async fn list_no_devices(
    tmp_path: TmpPath,
    #[case]
    #[cfg_attr(target_arch = "wasm32", expect(unused_variables))]
    path_exists: bool,
) {
    #[cfg(not(target_arch = "wasm32"))]
    if path_exists {
        std::fs::create_dir(tmp_path.join("devices")).unwrap();
    }

    let devices = list_available_devices(&tmp_path).await.unwrap();
    p_assert_eq!(devices, []);
}

#[parsec_test]
async fn ignore_invalid_items(tmp_path: TmpPath) {
    let devices_dir = tmp_path.join("devices");

    // Also add dummy stuff that should be ignored

    // Empty file
    crate::tests::utils::create_device_file(&devices_dir.join("empty.keys"), b"").await;
    // Dummy file
    crate::tests::utils::create_device_file(&devices_dir.join("dummy.keys"), b"dummy").await;
    // Folder with dummy file
    crate::tests::utils::create_device_file(&devices_dir.join("dir/a_file.keys"), b"dummy").await;

    log::trace!("Listing available devices");
    let devices = list_available_devices(&tmp_path).await.unwrap();
    p_assert_eq!(devices, []);
}

#[parsec_test]
async fn list_devices(tmp_path: TmpPath) {
    // 1. Generate raw data

    // Keyring

    let keyring_expected = DeviceFileKeyring {
        created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
        protected_on: "2000-01-01T00:00:01Z".parse().unwrap(),
        server_url: "https://parsec.invalid".to_string(),
        organization_id: "CoolOrg".parse().unwrap(),
        user_id: "alice".parse().unwrap(),
        device_id: "alice@dev1".parse().unwrap(),
        human_handle: "Alicey McAliceFace <alice@parsec.invalid>".parse().unwrap(),
        device_label: "My dev1 machine".parse().unwrap(),
        keyring_service: "keyring_service".to_string(),
        keyring_user: "keyring_user".to_string(),
        ciphertext: b"<ciphertext>".as_ref().into(),
    };
    println!(
        "***expected: {:?}",
        DeviceFile::Keyring(keyring_expected.clone()).dump()
    );

    // Generated from Parsec 3.3.0-rc.12+dev
    // Content:
    //   type: 'keyring'
    //   created_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
    //   protected_on: ext(1, 946684801000000) i.e. 2000-01-01T01:00:01Z
    //   server_url: 'https://parsec.invalid'
    //   organization_id: 'CoolOrg'
    //   user_id: ext(2, 0xa11cec00100000000000000000000000)
    //   device_id: ext(2, 0xde10a11cec0010000000000000000000)
    //   human_handle: [ 'alice@parsec.invalid', 'Alicey McAliceFace', ]
    //   device_label: 'My dev1 machine'
    //   keyring_service: 'keyring_service'
    //   keyring_user: 'keyring_user'
    //   ciphertext: 0x3c636970686572746578743e
    let keyring_raw: &[u8] = hex!(
        "8ca474797065a76b657972696e67aa637265617465645f6f6ed70100035d013b37e000"
        "ac70726f7465637465645f6f6ed70100035d013b472240aa7365727665725f75726cb6"
        "68747470733a2f2f7061727365632e696e76616c6964af6f7267616e697a6174696f6e"
        "5f6964a7436f6f6c4f7267a7757365725f6964d802a11cec0010000000000000000000"
        "0000a96465766963655f6964d802de10a11cec0010000000000000000000ac68756d61"
        "6e5f68616e646c6592b4616c696365407061727365632e696e76616c6964b2416c6963"
        "6579204d63416c69636546616365ac6465766963655f6c6162656caf4d792064657631"
        "206d616368696e65af6b657972696e675f73657276696365af6b657972696e675f7365"
        "7276696365ac6b657972696e675f75736572ac6b657972696e675f75736572aa636970"
        "68657274657874c40c3c636970686572746578743e"
    )
    .as_ref();

    // Password

    let password_expected = DeviceFilePassword {
        created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
        protected_on: "2000-01-01T00:00:01Z".parse().unwrap(),
        server_url: "https://parsec.invalid".to_string(),
        organization_id: "CoolOrg".parse().unwrap(),
        user_id: "bob".parse().unwrap(),
        device_id: "bob@dev1".parse().unwrap(),
        human_handle: "Boby McBobFace <bob@parsec.invalid>".parse().unwrap(),
        device_label: "My dev2 machine".parse().unwrap(),
        algorithm: PasswordAlgorithm::Argon2id {
            salt: *b"1234567890123456",
            opslimit: 1,
            memlimit_kb: 8,
            parallelism: 1,
        },
        ciphertext: b"<ciphertext>".as_ref().into(),
    };
    println!(
        "***expected: {:?}",
        DeviceFile::Password(password_expected.clone()).dump()
    );

    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   type: 'password'
    //   created_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
    //   protected_on: ext(1, 946684801000000) i.e. 2000-01-01T01:00:01Z
    //   server_url: 'https://parsec.invalid'
    //   organization_id: 'CoolOrg'
    //   user_id: ext(2, 0x808c0010000000000000000000000000)
    //   device_id: ext(2, 0xde10808c001000000000000000000000)
    //   human_handle: [ 'bob@parsec.invalid', 'Boby McBobFace', ]
    //   device_label: 'My dev2 machine'
    //   algorithm: { type: 'ARGON2ID', memlimit_kb: 8, opslimit: 1, parallelism: 1, salt: 0x31323334353637383930313233343536, }
    //   ciphertext: 0x3c636970686572746578743e
    let password_raw: &[u8] = hex!(
        "8ba474797065a870617373776f7264aa637265617465645f6f6ed70100035d013b37e0"
        "00ac70726f7465637465645f6f6ed70100035d013b472240aa7365727665725f75726c"
        "b668747470733a2f2f7061727365632e696e76616c6964af6f7267616e697a6174696f"
        "6e5f6964a7436f6f6c4f7267a7757365725f6964d802808c0010000000000000000000"
        "000000a96465766963655f6964d802de10808c001000000000000000000000ac68756d"
        "616e5f68616e646c6592b2626f62407061727365632e696e76616c6964ae426f627920"
        "4d63426f6246616365ac6465766963655f6c6162656caf4d792064657632206d616368"
        "696e65a9616c676f726974686d85a474797065a84152474f4e324944ab6d656d6c696d"
        "69745f6b6208a86f70736c696d697401ab706172616c6c656c69736d01a473616c74c4"
        "1031323334353637383930313233343536aa63697068657274657874c40c3c63697068"
        "6572746578743e"
    )
    .as_ref();

    // Smartcard

    let smartcard_expected = DeviceFileSmartcard {
        created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
        protected_on: "2000-01-01T00:00:01Z".parse().unwrap(),
        server_url: "https://parsec.invalid".to_string(),
        organization_id: "CoolOrg".parse().unwrap(),
        user_id: "mallory".parse().unwrap(),
        device_id: "mallory@dev1".parse().unwrap(),
        human_handle: "Mallory McMalloryFace <mallory@parsec.invalid>"
            .parse()
            .unwrap(),
        device_label: "PC1".parse().unwrap(),
        certificate_ref: X509CertificateReference::from(X509CertificateHash::fake_sha256())
            .add_or_replace_uri(X509WindowsCngURI::from(Bytes::from_static(
                b"Mallory's certificate",
            ))),
        algorithm_for_encrypted_key: EncryptionAlgorithm::RsaesOaepSha256,
        encrypted_key: hex!("de5c59cfcc0c52bf997594e0fdd2c24ffee9465b6f25e30bac9238c2f83fd19a")
            .as_ref()
            .into(),
        ciphertext: b"<ciphertext>".as_ref().into(),
    };
    println!(
        "***expected: {:?}",
        DeviceFile::Smartcard(smartcard_expected.clone()).dump()
    );
    // Generated from Parsec 3.5.1-a.0+dev
    // Content:
    //   type: 'smartcard'
    //   created_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
    //   protected_on: ext(1, 946684801000000) i.e. 2000-01-01T01:00:01Z
    //   server_url: 'https://parsec.invalid'
    //   organization_id: 'CoolOrg'
    //   user_id: ext(2, 0x3a11031c001000000000000000000000)
    //   device_id: ext(2, 0xde103a11031c00100000000000000000)
    //   human_handle: [ 'mallory@parsec.invalid', 'Mallory McMalloryFace', ]
    //   device_label: 'PC1'
    //   certificate_ref: {
    //     uri: 0x4d616c6c6f72792773206365727469666963617465,
    //     hash: 'sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=',
    //   }
    //   algorithm_for_encrypted_key: 'RSAES-OAEP-SHA256'
    //   encrypted_key: 0xde5c59cfcc0c52bf997594e0fdd2c24ffee9465b6f25e30bac9238c2f83fd19a
    //   ciphertext: 0x3c636970686572746578743e
    let smartcard_raw: &[u8] = hex!(
        "8da474797065a9736d61727463617264aa637265617465645f6f6ed70100035d013b37"
        "e000ac70726f7465637465645f6f6ed70100035d013b472240aa7365727665725f7572"
        "6cb668747470733a2f2f7061727365632e696e76616c6964af6f7267616e697a617469"
        "6f6e5f6964a7436f6f6c4f7267a7757365725f6964d8023a11031c0010000000000000"
        "00000000a96465766963655f6964d802de103a11031c00100000000000000000ac6875"
        "6d616e5f68616e646c6592b66d616c6c6f7279407061727365632e696e76616c6964b5"
        "4d616c6c6f7279204d634d616c6c6f727946616365ac6465766963655f6c6162656ca3"
        "504331af63657274696669636174655f72656682a47572697391d92877696e646f7773"
        "2d636e673a54574673624739796553647a49474e6c636e52705a6d6c6a5958526ca468"
        "617368d9337368613235362d4141414141414141414141414141414141414141414141"
        "41414141414141414141414141414141414141413dbb616c676f726974686d5f666f72"
        "5f656e637279707465645f6b6579b152534145532d4f4145502d534841323536ad656e"
        "637279707465645f6b6579c420de5c59cfcc0c52bf997594e0fdd2c24ffee9465b6f25"
        "e30bac9238c2f83fd19aaa63697068657274657874c40c3c636970686572746578743e"
    )
    .as_ref();

    // Recovery

    let recovery_expected = DeviceFileRecovery {
        created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
        protected_on: "2000-01-01T00:00:01Z".parse().unwrap(),
        server_url: "https://parsec.invalid".to_string(),
        organization_id: "CoolOrg".parse().unwrap(),
        user_id: "mike".parse().unwrap(),
        device_id: "mike@dev1".parse().unwrap(),
        human_handle: "Mike <mike@parsec.invalid>".parse().unwrap(),
        device_label: "My dev1 machine".parse().unwrap(),
        ciphertext: b"<ciphertext>".as_ref().into(),
    };
    println!(
        "***expected: {:?}",
        DeviceFile::Recovery(recovery_expected.clone()).dump()
    );

    // Generated from Parsec 3.3.0-rc.12+dev
    // Content:
    //   type: 'recovery'
    //   created_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
    //   protected_on: ext(1, 946684801000000) i.e. 2000-01-01T01:00:01Z
    //   server_url: 'https://parsec.invalid'
    //   organization_id: 'CoolOrg'
    //   user_id: ext(2, 0x31cec001000000000000000000000000)
    //   device_id: ext(2, 0xde1031cec00100000000000000000000)
    //   human_handle: [ 'mike@parsec.invalid', 'Mike', ]
    //   device_label: 'My dev1 machine'
    //   ciphertext: 0x3c636970686572746578743e
    let recovery_raw: &[u8] = hex!(
        "8aa474797065a87265636f76657279aa637265617465645f6f6ed70100035d013b37e0"
        "00ac70726f7465637465645f6f6ed70100035d013b472240aa7365727665725f75726c"
        "b668747470733a2f2f7061727365632e696e76616c6964af6f7267616e697a6174696f"
        "6e5f6964a7436f6f6c4f7267a7757365725f6964d80231cec001000000000000000000"
        "000000a96465766963655f6964d802de1031cec00100000000000000000000ac68756d"
        "616e5f68616e646c6592b36d696b65407061727365632e696e76616c6964a44d696b65"
        "ac6465766963655f6c6162656caf4d792064657631206d616368696e65aa6369706865"
        "7274657874c40c3c636970686572746578743e"
    )
    .as_ref();

    // AccountVault

    let account_vault_expected = DeviceFileAccountVault {
        created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
        protected_on: "2000-01-01T00:00:01Z".parse().unwrap(),
        server_url: "https://parsec.invalid".to_string(),
        organization_id: "CoolOrg".parse().unwrap(),
        user_id: "philip".parse().unwrap(),
        device_id: "philip@dev1".parse().unwrap(),
        human_handle: "Philip <philip@parsec.invalid>".parse().unwrap(),
        device_label: "My dev1 machine".parse().unwrap(),
        ciphertext_key_id: AccountVaultItemOpaqueKeyID::from_hex(
            "098dfa03df464cd6a580b151d7d3bb30",
        )
        .unwrap(),
        ciphertext: b"<ciphertext>".as_ref().into(),
    };
    println!(
        "***expected: {:?}",
        DeviceFile::AccountVault(account_vault_expected.clone()).dump()
    );

    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   type: 'account_vault'
    //   created_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
    //   protected_on: ext(1, 946684801000000) i.e. 2000-01-01T01:00:01Z
    //   server_url: 'https://parsec.invalid'
    //   organization_id: 'CoolOrg'
    //   user_id: ext(2, 0x91119ec0010000000000000000000000)
    //   device_id: ext(2, 0xde1091119ec001000000000000000000)
    //   human_handle: [ 'philip@parsec.invalid', 'Philip', ]
    //   device_label: 'My dev1 machine'
    //   ciphertext_key_id: ext(2, 0x098dfa03df464cd6a580b151d7d3bb30)
    //   ciphertext: 0x3c636970686572746578743e
    let account_vault_raw: &[u8] = hex!(
        "8ba474797065ad6163636f756e745f7661756c74aa637265617465645f6f6ed7010003"
        "5d013b37e000ac70726f7465637465645f6f6ed70100035d013b472240aa7365727665"
        "725f75726cb668747470733a2f2f7061727365632e696e76616c6964af6f7267616e69"
        "7a6174696f6e5f6964a7436f6f6c4f7267a7757365725f6964d80291119ec001000000"
        "0000000000000000a96465766963655f6964d802de1091119ec0010000000000000000"
        "00ac68756d616e5f68616e646c6592b57068696c6970407061727365632e696e76616c"
        "6964a65068696c6970ac6465766963655f6c6162656caf4d792064657631206d616368"
        "696e65b1636970686572746578745f6b65795f6964d802098dfa03df464cd6a580b151"
        "d7d3bb30aa63697068657274657874c40c3c636970686572746578743e"
    )
    .as_ref();

    // 2. Store the raws in files

    let keyring_path = tmp_path.join("devices/94a8691e9765497984d63aad3c7df9e0.keys");
    // Device must have a .keys extension, but can be in nested directories with a random name
    let password_path = tmp_path.join("devices/foo/bar/spam/whatever.keys");
    let smartcard_path = tmp_path.join("devices/foo/bar/spam/whatever2.keys");
    let recovery_path = tmp_path.join("devices/foo/whatever.keys");
    let account_vault_path = tmp_path.join("devices/whatever.keys");

    for (path, raw) in [
        (&keyring_path, keyring_raw),
        (&password_path, password_raw),
        (&smartcard_path, smartcard_raw),
        (&recovery_path, recovery_raw),
        (&account_vault_path, account_vault_raw),
    ] {
        crate::tests::utils::create_device_file(path, raw).await;
    }

    // 3. Actual test !

    let devices = list_available_devices(&tmp_path).await.unwrap();

    let expected_devices = Vec::from([
        AvailableDevice {
            key_file_path: keyring_path,
            created_on: keyring_expected.created_on,
            protected_on: keyring_expected.protected_on,
            server_url: keyring_expected.server_url,
            organization_id: keyring_expected.organization_id,
            user_id: keyring_expected.user_id,
            device_id: keyring_expected.device_id,
            human_handle: keyring_expected.human_handle,
            device_label: keyring_expected.device_label,
            ty: AvailableDeviceType::Keyring,
        },
        AvailableDevice {
            key_file_path: password_path,
            created_on: password_expected.created_on,
            protected_on: password_expected.protected_on,
            server_url: password_expected.server_url,
            organization_id: password_expected.organization_id,
            user_id: password_expected.user_id,
            device_id: password_expected.device_id,
            human_handle: password_expected.human_handle,
            device_label: password_expected.device_label,
            ty: AvailableDeviceType::Password,
        },
        AvailableDevice {
            key_file_path: smartcard_path,
            created_on: smartcard_expected.created_on,
            protected_on: smartcard_expected.protected_on,
            server_url: smartcard_expected.server_url,
            organization_id: smartcard_expected.organization_id,
            user_id: smartcard_expected.user_id,
            device_id: smartcard_expected.device_id,
            human_handle: smartcard_expected.human_handle,
            device_label: smartcard_expected.device_label,
            ty: AvailableDeviceType::Smartcard,
        },
        AvailableDevice {
            key_file_path: recovery_path,
            created_on: recovery_expected.created_on,
            protected_on: recovery_expected.protected_on,
            server_url: recovery_expected.server_url,
            organization_id: recovery_expected.organization_id,
            user_id: recovery_expected.user_id,
            device_id: recovery_expected.device_id,
            human_handle: recovery_expected.human_handle,
            device_label: recovery_expected.device_label,
            ty: AvailableDeviceType::Recovery,
        },
        AvailableDevice {
            key_file_path: account_vault_path,
            created_on: account_vault_expected.created_on,
            protected_on: account_vault_expected.protected_on,
            server_url: account_vault_expected.server_url,
            organization_id: account_vault_expected.organization_id,
            user_id: account_vault_expected.user_id,
            device_id: account_vault_expected.device_id,
            human_handle: account_vault_expected.human_handle,
            device_label: account_vault_expected.device_label,
            ty: AvailableDeviceType::AccountVault,
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
    let organization_addr = env.local_device("alice@dev1").organization_addr.clone();
    let devices = list_available_devices(&env.discriminant_dir).await.unwrap();
    p_assert_eq!(
        devices
            .iter()
            .map(|a| a.device_id)
            .collect::<Vec<DeviceID>>(),
        [
            "mallory@dev1".parse().unwrap(),
            "bob@dev1".parse().unwrap(),
            "alice@dev1".parse().unwrap(),
            "bob@dev2".parse().unwrap(),
            "alice@dev2".parse().unwrap(),
        ]
    );

    let alice1 = devices
        .iter()
        .find(|x| x.device_id == "alice@dev1".parse().unwrap())
        .unwrap();
    let alice2 = devices
        .iter()
        .find(|x| x.device_id == "alice@dev2".parse().unwrap())
        .unwrap();
    let bob1 = devices
        .iter()
        .find(|x| x.device_id == "bob@dev1".parse().unwrap())
        .unwrap();
    let bob2 = devices
        .iter()
        .find(|x| x.device_id == "bob@dev2".parse().unwrap())
        .unwrap();
    let mallory = devices
        .iter()
        .find(|x| x.device_id == "mallory@dev1".parse().unwrap())
        .unwrap();

    // Ensure loading a device doesn't change it

    p_assert_eq!(
        load_available_device(&env.discriminant_dir, alice1.key_file_path.clone())
            .await
            .unwrap(),
        *alice1,
    );
    load_device(
        &env.discriminant_dir,
        &DeviceAccessStrategy::Password {
            key_file: alice2.key_file_path.clone(),
            password: "P@ssw0rd.".to_string().into(),
        },
    )
    .await
    .unwrap();
    p_assert_eq!(
        list_available_devices(&env.discriminant_dir).await.unwrap(),
        devices
    );

    // Check list alteration

    remove_device(&env.discriminant_dir, &alice1.key_file_path)
        .await
        .unwrap();

    archive_device(&env.discriminant_dir, &alice2.key_file_path)
        .await
        .unwrap();

    let zack_key_file = env.discriminant_dir.join("zack_new_device.keys");
    let zack_human_handle = HumanHandle::from_raw("zack@example.invalid", "Zack").unwrap();
    let zack = save_device(
        &env.discriminant_dir,
        &DeviceSaveStrategy::AccountVault {
            operations: Arc::new(MockedAccountVaultOperations::new(
                zack_human_handle.email().to_owned(),
            )),
        },
        &LocalDevice::generate_new_device(
            organization_addr.clone(),
            UserProfile::Admin,
            zack_human_handle,
            "PC1".parse().unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        zack_key_file,
    )
    .await
    .unwrap();

    update_device_overwrite_server_addr(
        &env.discriminant_dir,
        &DeviceAccessStrategy::Password {
            key_file: bob1.key_file_path.clone(),
            password: "P@ssw0rd.".to_string().into(),
        },
        ParsecAddr::new("newhost.example.com".to_string(), None, true),
    )
    .await
    .unwrap();

    let bob2_new_key_file = env.discriminant_dir.join("bob2_new_device.keys");
    update_device_change_authentication(
        &env.discriminant_dir,
        &DeviceAccessStrategy::Password {
            key_file: bob2.key_file_path.clone(),
            password: "P@ssw0rd.".to_string().into(),
        },
        &DeviceSaveStrategy::AccountVault {
            operations: Arc::new(MockedAccountVaultOperations::new(
                bob2.human_handle.email().to_owned(),
            )),
        },
        &bob2_new_key_file,
    )
    .await
    .unwrap();

    let devices = list_available_devices(&env.discriminant_dir).await.unwrap();
    p_assert_eq!(
        devices
            .into_iter()
            .map(|a| (a.device_id, a.server_url, a.ty))
            .collect::<Vec<(DeviceID, String, AvailableDeviceType)>>(),
        [
            (
                bob2.device_id,
                "https://noserver.example.com/".to_string(),
                AvailableDeviceType::AccountVault,
            ),
            (
                mallory.device_id,
                "https://noserver.example.com/".to_string(),
                AvailableDeviceType::Password,
            ),
            (
                bob1.device_id,
                "https://newhost.example.com/".to_string(),
                AvailableDeviceType::Password,
            ),
            (
                zack.device_id,
                "https://noserver.example.com/".to_string(),
                AvailableDeviceType::AccountVault,
            ),
        ]
    );
}
