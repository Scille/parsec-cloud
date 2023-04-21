// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;

use libparsec_types::prelude::*;

use crate::{TestbedDeviceData, TestbedDeviceFileData, TestbedTemplate, TestbedUserData};

pub(crate) fn generate() -> TestbedTemplate {
    let mut devices = vec![];
    let mut users = vec![];

    let root_signing_key = SigningKey::from(hex!(
        "b62e7d2a9ed95187975294a1afb1ba345a79e4beb873389366d6c836d20ec5bc"
    ));

    // 2000-01-01: Org is created by alice@dev1

    let dt: DateTime = "2000-01-01T00:00:00Z".parse().unwrap();
    let alice1_id: DeviceID = "alice@dev1".parse().unwrap();
    let alice1_signkey = SigningKey::from(hex!(
        "d544f66ece9c85d5b80275db9124b5f04bb038081622bed139c1e789c5217400"
    ));
    users.push(TestbedUserData::new(
        alice1_id.user_id(),
        Some(HumanHandle::new("alice@example.com", "Alicey McAliceFace").unwrap()),
        PrivateKey::from(hex!(
            "74e860967fd90d063ebd64fb1ba6824c4c010099dd37508b7f2875a5db2ef8c9"
        )),
        UserProfile::Admin,
        EntryID::from_hex("a4031e8bcdd84df8ae12bd3d05e6e20f").unwrap(),
        SecretKey::from(hex!(
            "26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a"
        )),
        CertificateSignerOwned::Root,
        &root_signing_key,
        dt,
    ));
    devices.push(TestbedDeviceData::new(
        &alice1_id,
        Some("My dev1 machine".parse().unwrap()),
        alice1_signkey.clone(),
        SecretKey::from(hex!(
            "323614fc6bd2d300f42d6731059f542f89534cdca11b2cb13d5a9a5a6b19b6ac"
        )),
        CertificateSignerOwned::Root,
        &root_signing_key,
        dt,
    ));

    // 2000-01-02: alice@dev1 creates bob@dev1 (standard profile)

    let dt: DateTime = "2000-01-02T00:00:00Z".parse().unwrap();
    let bob1_id: DeviceID = "bob@dev1".parse().unwrap();
    let bob1_signkey = SigningKey::from(hex!(
        "85f47472a2c0f30f01b769617db248f3ec8d96a490602a9262f95e9e43432b30"
    ));
    users.push(TestbedUserData::new(
        bob1_id.user_id(),
        Some(HumanHandle::new("bob@example.com", "Boby McBobFace").unwrap()),
        PrivateKey::from(hex!(
            "16767ec446f2611f971c36f19c2dc11614d853475ac395d6c1d70ba46d07dd49"
        )),
        UserProfile::Standard,
        EntryID::from_hex("71568d41afcb4e2380b3d164ace4fb85").unwrap(),
        SecretKey::from(hex!(
            "65de53d2c6cd965aa53a1ba5cc7e54b331419e6103466121996fa99a97197a48"
        )),
        CertificateSignerOwned::User(alice1_id.clone()),
        &alice1_signkey,
        dt,
    ));
    devices.push(TestbedDeviceData::new(
        &bob1_id,
        Some("My dev1 machine".parse().unwrap()),
        bob1_signkey.clone(),
        SecretKey::from(hex!(
            "93f25b18491016f20b10dcf4eb7986716d914653d6ab4e778701c13435e6bdf0"
        )),
        CertificateSignerOwned::User(alice1_id.clone()),
        &alice1_signkey,
        dt,
    ));

    // 2000-01-03: alice@dev1 creates alice@dev2

    let dt: DateTime = "2000-01-03T00:00:00Z".parse().unwrap();
    let alice2_id: DeviceID = "alice@dev2".parse().unwrap();
    devices.push(TestbedDeviceData::new(
        &alice2_id,
        Some("My dev2 machine".parse().unwrap()),
        SigningKey::from(hex!(
            "571d726cc5586b4bfd5b20d8af2365cf8bb8c881b4925794e6e38cdcc5ec82ef"
        )),
        SecretKey::from(hex!(
            "fd64fba009dc635af6662f45753b8c9772ff6e214e203e9d61ce029550e250f7"
        )),
        CertificateSignerOwned::User(alice1_id.clone()),
        &alice1_signkey,
        dt,
    ));

    // 2000-01-04: bob@dev1 creates bob@dev2

    let dt: DateTime = "2000-01-04T00:00:00Z".parse().unwrap();
    let bob2_id: DeviceID = "bob@dev2".parse().unwrap();
    devices.push(TestbedDeviceData::new(
        &bob2_id,
        Some("My dev2 machine".parse().unwrap()),
        SigningKey::from(hex!(
            "0d00fbdeef1cd8b12b6bf40ce88452e9190ed03f2130394930524e3edde192f0"
        )),
        SecretKey::from(hex!(
            "e4f37fc4a62c7d775c703d23a95b91820dde82ce923a694a1c131f66bc952474"
        )),
        CertificateSignerOwned::User(bob1_id.clone()),
        &bob1_signkey,
        dt,
    ));

    // 2000-01-05: alice@dev1 creates mallory@dev1 (outsider profile with no label/human handle)

    let dt: DateTime = "2000-01-05T00:00:00Z".parse().unwrap();
    let mallory1_id: DeviceID = "mallory@dev1".parse().unwrap();
    let mallory1_signkey = SigningKey::from(hex!(
        "f73862c4d73c9a6b6d5123fdfadcbd547cc8e9674c2a6f5a32f67e228d56eb9d"
    ));
    users.push(TestbedUserData::new(
        mallory1_id.user_id(),
        None,
        PrivateKey::from(hex!(
            "ef97499baaf2f5ee729b8fc7b6a89ec00d149e3cfb86c52a5db5cb3db5c0d521"
        )),
        UserProfile::Outsider,
        EntryID::from_hex("bfd597b825ac4a0aaeb8a31b08f4f377").unwrap(),
        SecretKey::from(hex!(
            "64d129049f977bc12d285b9520469603766161ef793c19a486bd0ee26eb56142"
        )),
        CertificateSignerOwned::User(alice1_id.clone()),
        &alice1_signkey,
        dt,
    ));
    devices.push(TestbedDeviceData::new(
        &mallory1_id,
        None,
        mallory1_signkey,
        SecretKey::from(hex!(
            "dd718a6d8eecc42e2cb7a158960d38ff153b43668286e0bae994134db8723b3f"
        )),
        CertificateSignerOwned::User(alice1_id.clone()),
        &alice1_signkey,
        dt,
    ));

    // Local devices
    let device_files = vec![
        TestbedDeviceFileData::new(
            alice1_id,
            "P@ssw0rd.".to_owned(),
            SecretKey::from(hex!(
                "323614fc6bd2d300f42d6731059f542f89534cdca11b2cb13d5a9a5a6b19b6ac"
            )),
        ),
        TestbedDeviceFileData::new(
            alice2_id,
            "P@ssw0rd.".to_owned(),
            SecretKey::from(hex!(
                "898e1f4e5825ff9c039603f53653caee0eb3ccf18f419b4cb4dafe5ab5bd590f"
            )),
        ),
        TestbedDeviceFileData::new(
            bob1_id,
            "P@ssw0rd.".to_owned(),
            SecretKey::from(hex!(
                "bab3f8ba8d5e3f313472208dad9bb41497116ef0e5a391a5fc659bf5eb7fdbda"
            )),
        ),
    ];

    // TODO: create revoked users
    // TODO: creates manifests, blocks

    TestbedTemplate::new("coolorg", root_signing_key, devices, users, device_files)
}
