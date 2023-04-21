// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;

use libparsec_types::prelude::*;

use crate::{TestbedDeviceData, TestbedTemplate, TestbedUserData};

pub(crate) fn generate() -> TestbedTemplate {
    let root_signing_key = SigningKey::from(hex!(
        "b62e7d2a9ed95187975294a1afb1ba345a79e4beb873389366d6c836d20ec5bc"
    ));

    let dt: DateTime = "2000-01-01T00:00:00Z".parse().unwrap();
    let alice1_id: DeviceID = "alice@dev1".parse().unwrap();
    let alice1_signkey = SigningKey::from(hex!(
        "d544f66ece9c85d5b80275db9124b5f04bb038081622bed139c1e789c5217400"
    ));
    let user = TestbedUserData::new(
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
    );
    let device = TestbedDeviceData::new(
        &alice1_id,
        Some("My dev1 machine".parse().unwrap()),
        alice1_signkey,
        SecretKey::from(hex!(
            "323614fc6bd2d300f42d6731059f542f89534cdca11b2cb13d5a9a5a6b19b6ac"
        )),
        CertificateSignerOwned::Root,
        &root_signing_key,
        dt,
    );

    TestbedTemplate::new(
        "minimal",
        root_signing_key,
        vec![device],
        vec![user],
        vec![],
    )
}
