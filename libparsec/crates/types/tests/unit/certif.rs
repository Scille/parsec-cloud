// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_tests_lite::prelude::*;

use crate::fixtures::{alice, bob, timestamp, Device};
use crate::prelude::*;

// TODO: check serde output to ensure handling of Option<T> depending of
// default/missing policy

#[rstest]
fn debug_format(alice: &Device, bob: &Device, timestamp: DateTime) {
    let user_certificate = UserCertificate {
        author: CertificateSignerOwned::User(alice.device_id),
        timestamp,
        user_id: bob.user_id,
        human_handle: MaybeRedacted::Real(bob.human_handle.clone()),
        public_key: bob.public_key(),
        algorithm: PrivateKeyAlgorithm::X25519XSalsa20Poly1305,
        profile: UserProfile::Standard,
    };
    p_assert_eq!(
        format!("{:?}", user_certificate),
        concat!(
            "UserCertificate {",
            " author: User(DeviceID { nickname: \"alice@dev1\", id: \"de10a11c-ec00-1000-0000-000000000000\" }),",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " user_id: UserID { nickname: \"bob\", id: \"808c0010-0000-0000-0000-000000000000\" },",
            " human_handle: Real(HumanHandle(\"Boby McBobFace <bob@example.com>\")),",
            " public_key: PublicKey(****),",
            " algorithm: X25519XSalsa20Poly1305,",
            " profile: Standard",
            " }",
        )
    );

    let device_certificate = DeviceCertificate {
        author: CertificateSignerOwned::User(alice.device_id),
        timestamp,
        user_id: bob.user_id,
        device_id: bob.device_id,
        device_label: MaybeRedacted::Real(bob.device_label.clone()),
        verify_key: bob.verify_key(),
        algorithm: SigningKeyAlgorithm::Ed25519,
    };
    p_assert_eq!(
        format!("{:?}", device_certificate),
        concat!(
            "DeviceCertificate {",
            " author: User(DeviceID { nickname: \"alice@dev1\", id: \"de10a11c-ec00-1000-0000-000000000000\" }),",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " user_id: UserID { nickname: \"bob\", id: \"808c0010-0000-0000-0000-000000000000\" },",
            " device_id: DeviceID { nickname: \"bob@dev1\", id: \"de10808c-0010-0000-0000-000000000000\" },",
            " device_label: Real(DeviceLabel(\"My dev1 machine\")),",
            " verify_key: VerifyKey(****),",
            " algorithm: Ed25519",
            " }",
        )
    );

    let revoked_user_certificate = RevokedUserCertificate {
        author: alice.device_id,
        timestamp,
        user_id: bob.user_id,
    };
    p_assert_eq!(
        format!("{:?}", revoked_user_certificate),
        concat!(
            "RevokedUserCertificate {",
            " author: DeviceID { nickname: \"alice@dev1\", id: \"de10a11c-ec00-1000-0000-000000000000\" },",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " user_id: UserID { nickname: \"bob\", id: \"808c0010-0000-0000-0000-000000000000\" }",
            " }",
        )
    );

    let user_update_certificate = UserUpdateCertificate {
        author: alice.device_id,
        timestamp,
        user_id: bob.user_id,
        new_profile: UserProfile::Outsider,
    };
    p_assert_eq!(
        format!("{:?}", user_update_certificate),
        concat!(
            "UserUpdateCertificate {",
            " author: DeviceID { nickname: \"alice@dev1\", id: \"de10a11c-ec00-1000-0000-000000000000\" },",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " user_id: UserID { nickname: \"bob\", id: \"808c0010-0000-0000-0000-000000000000\" },",
            " new_profile: Outsider",
            " }",
        )
    );

    let realm_role_certificate = RealmRoleCertificate {
        author: alice.device_id,
        timestamp,
        user_id: bob.user_id,
        realm_id: VlobID::from_hex("604784450642426b91eb89242f54fa52").unwrap(),
        role: Some(RealmRole::Owner),
    };
    p_assert_eq!(
        format!("{:?}", realm_role_certificate),
        concat!(
            "RealmRoleCertificate {",
            " author: DeviceID { nickname: \"alice@dev1\", id: \"de10a11c-ec00-1000-0000-000000000000\" },",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " realm_id: VlobID(60478445-0642-426b-91eb-89242f54fa52),",
            " user_id: UserID { nickname: \"bob\", id: \"808c0010-0000-0000-0000-000000000000\" },",
            " role: Some(Owner)",
            " }",
        )
    );

    let realm_name = RealmNameCertificate {
        author: alice.device_id,
        timestamp,
        realm_id: VlobID::from_hex("604784450642426b91eb89242f54fa52").unwrap(),
        key_index: 42,
        encrypted_name: b"012345".to_vec(),
    };
    p_assert_eq!(
        format!("{:?}", realm_name),
        concat!(
            "RealmNameCertificate {",
            " author: DeviceID { nickname: \"alice@dev1\", id: \"de10a11c-ec00-1000-0000-000000000000\" },",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " realm_id: VlobID(60478445-0642-426b-91eb-89242f54fa52),",
            " key_index: 42,",
            " encrypted_name: [48, 49, 50, 51, 52, 53]",
            " }",
        )
    );

    let realm_key_rotation = RealmKeyRotationCertificate {
        author: alice.device_id,
        timestamp,
        realm_id: VlobID::from_hex("604784450642426b91eb89242f54fa52").unwrap(),
        encryption_algorithm: SecretKeyAlgorithm::Blake2bXsalsa20Poly1305,
        hash_algorithm: HashAlgorithm::Sha256,
        key_index: 42,
        key_canary: b"012345".to_vec(),
    };
    p_assert_eq!(
        format!("{:?}", realm_key_rotation),
        concat!(
            "RealmKeyRotationCertificate {",
            " author: DeviceID { nickname: \"alice@dev1\", id: \"de10a11c-ec00-1000-0000-000000000000\" },",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " realm_id: VlobID(60478445-0642-426b-91eb-89242f54fa52),",
            " key_index: 42,",
            " encryption_algorithm: Blake2bXsalsa20Poly1305,",
            " hash_algorithm: Sha256,",
            " key_canary: [48, 49, 50, 51, 52, 53]",
            " }",
        )
    );

    let realm_archiving_certificate = RealmArchivingCertificate {
        author: alice.device_id,
        timestamp,
        realm_id: VlobID::from_hex("604784450642426b91eb89242f54fa52").unwrap(),
        configuration: RealmArchivingConfiguration::DeletionPlanned {
            deletion_date: timestamp,
        },
    };
    p_assert_eq!(
        format!("{:?}", realm_archiving_certificate),
        concat!(
            "RealmArchivingCertificate {",
            " author: DeviceID { nickname: \"alice@dev1\", id: \"de10a11c-ec00-1000-0000-000000000000\" },",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " realm_id: VlobID(60478445-0642-426b-91eb-89242f54fa52),",
            " configuration: DeletionPlanned { deletion_date: DateTime(\"2020-01-01T00:00:00Z\") }",
            " }",
        )
    );

    let shamir_recovery_brief_certificate = ShamirRecoveryBriefCertificate {
        author: alice.device_id,
        timestamp: "2020-01-01T00:00:00Z".parse().unwrap(),
        user_id: alice.user_id,
        threshold: 3.try_into().unwrap(),
        per_recipient_shares: HashMap::from([
            ((bob.user_id), 2.try_into().unwrap()),
            ("carl".parse().unwrap(), 1.try_into().unwrap()),
            ("diana".parse().unwrap(), 1.try_into().unwrap()),
        ]),
    };
    assert!(
        format!("{:?}", shamir_recovery_brief_certificate).starts_with(
            // Ignore `per_recipient_shares` as, as a HashMap, it output is not stable across runs
            concat!(
                "ShamirRecoveryBriefCertificate {",
                " author: DeviceID { nickname: \"alice@dev1\", id: \"de10a11c-ec00-1000-0000-000000000000\" },",
                " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
                " user_id: UserID { nickname: \"alice\", id: \"a11cec00-1000-0000-0000-000000000000\" },",
                " threshold: 3,",
                " per_recipient_shares: "
            )
        )
    );

    let shamir_recovery_share_certificate = ShamirRecoveryShareCertificate {
        author: alice.device_id,
        timestamp: "2020-01-01T00:00:00Z".parse().unwrap(),
        user_id: alice.user_id,
        recipient: bob.user_id,
        ciphered_share: b"abcd".to_vec(),
    };
    p_assert_eq!(
        format!("{:?}", shamir_recovery_share_certificate),
        concat!(
            "ShamirRecoveryShareCertificate {",
            " author: DeviceID { nickname: \"alice@dev1\", id: \"de10a11c-ec00-1000-0000-000000000000\" },",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " user_id: UserID { nickname: \"alice\", id: \"a11cec00-1000-0000-0000-000000000000\" },",
            " recipient: UserID { nickname: \"bob\", id: \"808c0010-0000-0000-0000-000000000000\" },",
            " ciphered_share: [97, 98, 99, 100]",
            " }",
        )
    );

    let sequester_authority_certificate = SequesterAuthorityCertificate {
        timestamp,
        verify_key_der: SequesterVerifyKeyDer::try_from(
            &hex!(
                "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5"
                "c689b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d508"
                "4782738a3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083"
                "662269b7b62cb38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5a"
                "ff66f7ee18303195f3bcc72ab57207ebfd0203010001"
            )[..],
        )
        .unwrap(),
    };
    p_assert_eq!(
        format!("{:?}", sequester_authority_certificate),
        concat!(
            "SequesterAuthorityCertificate {",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " verify_key_der: SequesterVerifyKeyDer(****)",
            " }",
        )
    );

    let sequester_service_certificate = SequesterServiceCertificate {
        timestamp,
        service_id: SequesterServiceID::from_hex("b5eb565343c442b3a26be44573813ff0").unwrap(),
        service_label: "foo".into(),
        encryption_key_der: SequesterPublicKeyDer::try_from(
            &hex!(
                "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5"
                "c689b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d508"
                "4782738a3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083"
                "662269b7b62cb38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5a"
                "ff66f7ee18303195f3bcc72ab57207ebfd0203010001"
            )[..],
        )
        .unwrap(),
    };
    p_assert_eq!(
        format!("{:?}", sequester_service_certificate),
        concat!(
            "SequesterServiceCertificate {",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " service_id: SequesterServiceID(b5eb5653-43c4-42b3-a26b-e44573813ff0),",
            " service_label: \"foo\",",
            " encryption_key_der: SequesterPublicKeyDer(****)",
            " }",
        )
    );
}

#[rstest]
fn serde_user_certificate(alice: &Device, bob: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'user_certificate'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   user_id: ext(2, 0x808c0010000000000000000000000000)
    //   human_handle: ['bob@example.com', 'Boby McBobFace']
    //   public_key: 0x7c999e9980bef37707068b07d975591efc56335be9634ceef7c932a09c891e25
    //   algorithm: 'X25519_XSALSA20_POLY1305'
    //   profile: 'STANDARD'
    let data = Bytes::from_static(&hex!(
    "62bb973ba1120ba57aeab608d2de055785be4faf3b9a54be6f409f801b6f3afc1a4102"
    "e41cfea6e7247a0126844b2a076feadd8be5caf903ffea9abe135ea3030028b52ffd00"
    "58050700440d88a474797065b0757365725f6365727469666963617465a6617574686f"
    "72d802de10a11cec001000a974696d657374616d70d7010005d250a2269a75a76964d8"
    "02808c000000ac68756d616e5f68616e646c6592af626f62406578616d706c652e636f"
    "6dae426f6279204d63426f6246616365aa7075626c69635f6b6579c4207c999e9980be"
    "f37707068b07d975591efc56335be9634ceef7c932a09c891e25a9616c676f72697468"
    "6db85832353531395f5853414c534132305f504f4c5931333035a770726f66696c65a8"
    "5354414e4441524403004ead0cc265204019"
    ));

    let expected = UserCertificate {
        author: CertificateSignerOwned::User(alice.device_id),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        user_id: bob.user_id,
        human_handle: MaybeRedacted::Real(bob.human_handle.to_owned()),
        public_key: bob.public_key(),
        algorithm: PrivateKeyAlgorithm::X25519XSalsa20Poly1305,
        profile: bob.profile,
    };

    let certif = UserCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        CertificateSignerRef::User(&alice.device_id),
        None,
        None,
    )
    .unwrap();
    p_assert_eq!(certif, expected);

    let unsecure_certif = UserCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(
        unsecure_certif.author(),
        CertificateSignerOwned::User(alice.device_id)
    );
    p_assert_eq!(
        unsecure_certif
            .verify_signature(&alice.verify_key())
            .unwrap(),
        (expected.clone(), data.clone())
    );

    let unsecure_certif = UserCertificate::unsecure_load(data).unwrap();
    p_assert_eq!(
        unsecure_certif.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage),
        expected
    );

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = UserCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        CertificateSignerRef::User(&alice.device_id),
        None,
        None,
    )
    .unwrap();
    p_assert_eq!(certif2, expected);

    // Test invalid data
    p_assert_matches!(
        UserCertificate::unsecure_load(b"dummy".to_vec().into()),
        Err(DataError::Signature)
    );
    p_assert_matches!(
        UserCertificate::verify_and_load(
            b"dummy",
            &alice.verify_key(),
            CertificateSignerRef::Root,
            None,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_user_certificate_redacted(alice: &Device, bob: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'user_certificate'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   user_id: ext(2, 0x808c0010000000000000000000000000)
    //   human_handle: None
    //   public_key: 0x7c999e9980bef37707068b07d975591efc56335be9634ceef7c932a09c891e25
    //   algorithm: 'X25519_XSALSA20_POLY1305'
    //   profile: 'STANDARD'
    let data = Bytes::from_static(&hex!(
    "780c2456266bf6c7c9401cefa43455beb83fc64707513df2a4f36cc65a75522dc7142e"
    "5968ee979803a7a9d8a84b9332e45a934b88e57267aeea1b9bc797a10a0028b52ffd00"
    "580d0600540b88a474797065b0757365725f6365727469666963617465a6617574686f"
    "72d802de10a11cec001000a974696d657374616d70d7010005d250a2269a75a76964d8"
    "02808c000000ac68756d616e5f68616e646c65c0aa7075626c69635f6b6579c4207c99"
    "9e9980bef37707068b07d975591efc56335be9634ceef7c932a09c891e25a9616c676f"
    "726974686db85832353531395f5853414c534132305f504f4c5931333035a770726f66"
    "696c65a85354414e4441524403004ead0cc265204019"
    ));
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = UserCertificate {
        author: CertificateSignerOwned::User(alice.device_id),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        user_id: bob.user_id,
        human_handle: MaybeRedacted::Redacted(HumanHandle::new_redacted(bob.user_id)),
        public_key: bob.public_key(),
        algorithm: PrivateKeyAlgorithm::X25519XSalsa20Poly1305,
        profile: bob.profile,
    };

    let certif = UserCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        CertificateSignerRef::User(&alice.device_id),
        None,
        None,
    )
    .unwrap();
    p_assert_eq!(certif, expected);

    let unsecure_certif = UserCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(
        unsecure_certif.author(),
        CertificateSignerOwned::User(alice.device_id)
    );
    p_assert_eq!(
        unsecure_certif
            .verify_signature(&alice.verify_key())
            .unwrap(),
        (expected.clone(), data.clone())
    );

    let unsecure_certif = UserCertificate::unsecure_load(data).unwrap();
    p_assert_eq!(
        unsecure_certif.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage),
        expected
    );

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = UserCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        CertificateSignerRef::User(&alice.device_id),
        None,
        None,
    )
    .unwrap();
    p_assert_eq!(certif2, expected);
}

#[rstest]
fn serde_device_certificate(alice: &Device, bob: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'device_certificate'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   user_id: ext(2, 0xa11cec00100000000000000000000000)
    //   device_id: ext(2, 0xde10808c001000000000000000000000)
    //   device_label: 'My dev1 machine'
    //   verify_key: 0x840d872f4252da2d1c9f81a77db5f0a5b9b60a5cde1eeabf40388ef6bca64909
    //   algorithm: 'ED25519'
    let data = Bytes::from_static(&hex!(
    "7594b5baab012bc81b0ddbe668cfcfa25cd6beffc7ea42a1a0dcbeeb9da4f51e861a2c"
    "b15148b8f456e56f3f92ba570ade7da5ec670edc079479e754901e5b020028b52ffd00"
    "58d50500940a88a474797065b26465766963655f6365727469666963617465a6617574"
    "686f72d802de10a11cec001000a974696d657374616d70d7010005d250a2269a75a775"
    "7365725f6964d8020000a96964d802de10808c00ac6c6162656caf4d79206465763120"
    "6d616368696e65aa7665726966795f6b6579c420840d872f4252da2d1c9f81a77db5f0"
    "a5b9b60a5cde1eeabf40388ef6bca64909a9616c676f726974686da745443235353139"
    "05003f1933687887b3e153d2e90565"
    ));
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = DeviceCertificate {
        author: CertificateSignerOwned::User(alice.device_id),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        user_id: alice.user_id,
        device_id: bob.device_id.to_owned(),
        device_label: MaybeRedacted::Real(bob.device_label.to_owned()),
        verify_key: bob.verify_key(),
        algorithm: SigningKeyAlgorithm::Ed25519,
    };

    let certif = DeviceCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        CertificateSignerRef::User(&alice.device_id),
        None,
    )
    .unwrap();
    p_assert_eq!(certif, expected);

    let unsecure_certif = DeviceCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(
        unsecure_certif.author(),
        CertificateSignerOwned::User(alice.device_id)
    );
    p_assert_eq!(
        unsecure_certif
            .verify_signature(&alice.verify_key())
            .unwrap(),
        (expected.clone(), data.clone())
    );

    let unsecure_certif = DeviceCertificate::unsecure_load(data).unwrap();
    p_assert_eq!(
        unsecure_certif.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage),
        expected
    );

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = DeviceCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        CertificateSignerRef::User(&alice.device_id),
        None,
    )
    .unwrap();
    p_assert_eq!(certif2, expected);

    // Test invalid data
    p_assert_matches!(
        DeviceCertificate::unsecure_load(b"dummy".to_vec().into()),
        Err(DataError::Signature)
    );
    p_assert_matches!(
        DeviceCertificate::verify_and_load(
            b"dummy",
            &alice.verify_key(),
            CertificateSignerRef::Root,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_device_certificate_redacted(alice: &Device, bob: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'device_certificate'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   user_id: ext(2, 0xa11cec00100000000000000000000000)
    //   device_id: ext(2, 0xde10808c001000000000000000000000)
    //   device_label: None
    //   verify_key: 0x840d872f4252da2d1c9f81a77db5f0a5b9b60a5cde1eeabf40388ef6bca64909
    //   algorithm: 'ED25519'
    let data = Bytes::from_static(&hex!(
    "8d90b720e56cff7706388e064c24ab4b6ea8dc0f62e07f93b9f8bcc5409b30caac42ee"
    "b6bd77c01ccf32bf6c0e5db51ed55b21b93fe219f9c5d813d5f25ed90a0028b52ffd00"
    "585d0500a40988a474797065b26465766963655f6365727469666963617465a6617574"
    "686f72d802de10a11cec001000a974696d657374616d70d7010005d250a2269a75a775"
    "7365725f6964d8020000a96964d802de10808c00ac6c6162656cc0aa7665726966795f"
    "6b6579c420840d872f4252da2d1c9f81a77db5f0a5b9b60a5cde1eeabf40388ef6bca6"
    "4909a9616c676f726974686da74544323535313905003f1933687887b3e153d2e90565"
    ));
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = DeviceCertificate {
        author: CertificateSignerOwned::User(alice.device_id),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        user_id: alice.user_id,
        device_id: bob.device_id.to_owned(),
        device_label: MaybeRedacted::Redacted(DeviceLabel::new_redacted(bob.device_id)),
        verify_key: bob.verify_key(),
        algorithm: SigningKeyAlgorithm::Ed25519,
    };

    let certif = DeviceCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        CertificateSignerRef::User(&alice.device_id),
        None,
    )
    .unwrap();
    p_assert_eq!(certif, expected);

    let unsecure_certif = DeviceCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(
        unsecure_certif.author(),
        CertificateSignerOwned::User(alice.device_id)
    );
    p_assert_eq!(
        unsecure_certif
            .verify_signature(&alice.verify_key())
            .unwrap(),
        (expected.clone(), data.clone())
    );

    let unsecure_certif = DeviceCertificate::unsecure_load(data).unwrap();
    p_assert_eq!(
        unsecure_certif.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage),
        expected
    );

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = DeviceCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        CertificateSignerRef::User(&alice.device_id),
        None,
    )
    .unwrap();
    p_assert_eq!(certif2, expected);
}

#[rstest]
fn serde_revoked_user_certificate(alice: &Device, bob: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'revoked_user_certificate'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   user_id: ext(2, 0x808c0010000000000000000000000000)
    let data = Bytes::from_static(&hex!(
    "9714eac8455369a26e197bccea7b9e44fe5a664c53926f7b494dbdc9829b9adbcf57a4"
    "818a98b4c8c2b6c4d214cc9bcc07e73a892d7751ccfa29e7d66d5b2e060028b52ffd00"
    "58d50200e40484a474797065b87265766f6b65645f757365725f636572746966696361"
    "7465a6617574686f72d802de10a11cec001000a974696d657374616d70d7010005d250"
    "a2269a75a76964d802808c00000003004ead0cc265400028"
    ));
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = RevokedUserCertificate {
        author: alice.device_id,
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        user_id: bob.user_id,
    };

    let unsecure_certif = RevokedUserCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), alice.device_id);
    p_assert_eq!(
        unsecure_certif
            .verify_signature(&alice.verify_key())
            .unwrap(),
        (expected.clone(), data.clone())
    );

    let unsecure_certif = RevokedUserCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(
        unsecure_certif.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage),
        expected
    );

    let certif =
        RevokedUserCertificate::verify_and_load(&data, &alice.verify_key(), alice.device_id, None)
            .unwrap();
    p_assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 =
        RevokedUserCertificate::verify_and_load(&data2, &alice.verify_key(), alice.device_id, None)
            .unwrap();
    p_assert_eq!(certif2, expected);

    // Test invalid data
    p_assert_matches!(
        RevokedUserCertificate::unsecure_load(b"dummy".to_vec().into()),
        Err(DataError::Signature)
    );
    p_assert_matches!(
        RevokedUserCertificate::verify_and_load(
            b"dummy",
            &alice.verify_key(),
            alice.device_id,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_user_update_certificate(alice: &Device, bob: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'user_update_certificate'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   user_id: ext(2, 0x808c0010000000000000000000000000)
    //   new_profile: 'OUTSIDER'
    let data = Bytes::from_static(&hex!(
    "158f3218b45387c0701b811a35d313fa5db1d359750f1a33646181bc7c008ea0887ed4"
    "ef624637e6deb66795972a4d622805762cd4fb3f5ce29c838cd05374040028b52ffd00"
    "58750300240685a474797065b7757365725f7570646174655f63657274696669636174"
    "65a6617574686f72d802de10a11cec001000a974696d657374616d70d7010005d250a2"
    "269a75a76964d802808c000000ab6e65775f70726f66696c65a84f5554534944455203"
    "004ead44c265274019"
    ));
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = UserUpdateCertificate {
        author: alice.device_id,
        new_profile: UserProfile::Outsider,
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        user_id: bob.user_id,
    };

    let unsecure_certif = UserUpdateCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), alice.device_id);
    p_assert_eq!(
        unsecure_certif
            .verify_signature(&alice.verify_key())
            .unwrap(),
        (expected.clone(), data.clone())
    );

    let unsecure_certif = UserUpdateCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(
        unsecure_certif.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage),
        expected
    );

    let certif =
        UserUpdateCertificate::verify_and_load(&data, &alice.verify_key(), alice.device_id, None)
            .unwrap();
    p_assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 =
        UserUpdateCertificate::verify_and_load(&data2, &alice.verify_key(), alice.device_id, None)
            .unwrap();
    p_assert_eq!(certif2, expected);

    // Test invalid data
    p_assert_matches!(
        UserUpdateCertificate::unsecure_load(b"dummy".to_vec().into()),
        Err(DataError::Signature)
    );
    p_assert_matches!(
        UserUpdateCertificate::verify_and_load(
            b"dummy",
            &alice.verify_key(),
            alice.device_id,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_realm_role_certificate(alice: &Device, bob: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'realm_role_certificate'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   realm_id: ext(2, 0x4486e7cf02d747bd9126679ba58e0474)
    //   user_id: ext(2, 0x808c0010000000000000000000000000)
    //   role: 'OWNER'
    let data = Bytes::from_static(&hex!(
    "7545823c4d29cb38e353b057fb9b28ed2f21ab954ca9d155c521fa48969896d920ecc9"
    "0272741981a350093330f1fb9021271af1f7ddad331e557146736407070028b52ffd00"
    "58dd0300c40686a474797065b67265616c6d5f726f6c655f6365727469666963617465"
    "a6617574686f72d802de10a11cec001000a974696d657374616d70d7010005d250a226"
    "9a75a86964d8024486e7cf02d747bd9126679ba58e0474a775736572808c000000a472"
    "6f6c65a54f574e4552040049305ccf1d0f3cae09a007"
    ));
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = RealmRoleCertificate {
        author: alice.device_id,
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        realm_id: VlobID::from_hex("4486e7cf02d747bd9126679ba58e0474").unwrap(),
        user_id: bob.user_id,
        role: Some(RealmRole::Owner),
    };
    let certif = RealmRoleCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        alice.device_id,
        None,
        None,
    )
    .unwrap();
    p_assert_eq!(certif, expected);

    let unsecure_certif = RealmRoleCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), alice.device_id,);
    p_assert_eq!(
        unsecure_certif
            .verify_signature(&alice.verify_key())
            .unwrap(),
        (expected.clone(), data.clone())
    );

    let unsecure_certif = RealmRoleCertificate::unsecure_load(data).unwrap();
    p_assert_eq!(
        unsecure_certif.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage),
        expected
    );

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = RealmRoleCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        alice.device_id,
        None,
        None,
    )
    .unwrap();
    p_assert_eq!(certif2, expected);

    // Test invalid data
    p_assert_matches!(
        RealmRoleCertificate::unsecure_load(b"dummy".to_vec().into()),
        Err(DataError::Signature)
    );
    p_assert_matches!(
        RealmRoleCertificate::verify_and_load(
            b"dummy",
            &alice.verify_key(),
            alice.device_id,
            None,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_realm_role_certificate_no_role(alice: &Device, bob: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'realm_role_certificate'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   realm_id: ext(2, 0x4486e7cf02d747bd9126679ba58e0474)
    //   user_id: ext(2, 0x808c0010000000000000000000000000)
    //   role: None
    let data = Bytes::from_static(&hex!(
    "08c8deac362d90d18496441b0afac872b24f1908d881c3345da59981539b19a24b09e8"
    "8b777f2002d790b55237e8640c89ee54a28c29bcb8a4c9eed4453dd60c0028b52ffd00"
    "58b50300740686a474797065b67265616c6d5f726f6c655f6365727469666963617465"
    "a6617574686f72d802de10a11cec001000a974696d657374616d70d7010005d250a226"
    "9a75a86964d8024486e7cf02d747bd9126679ba58e0474a775736572808c000000a472"
    "6f6c65c0040049305ccf1d0f3cae09a007"
    ));

    let expected = RealmRoleCertificate {
        author: alice.device_id,
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        realm_id: VlobID::from_hex("4486e7cf02d747bd9126679ba58e0474").unwrap(),
        user_id: bob.user_id,
        role: None,
        // role: Some(RealmRole::Owner),
    };
    let certif = RealmRoleCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        alice.device_id,
        None,
        None,
    )
    .unwrap();
    p_assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = RealmRoleCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        alice.device_id,
        None,
        None,
    )
    .unwrap();
    p_assert_eq!(certif2, expected);
}

#[rstest]
fn serde_realm_archiving_certificate_available(alice: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'realm_archiving_certificate'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1577836800000000) i.e. 2020-01-01T01:00:00Z
    //   realm_id: ext(2, 0x4486e7cf02d747bd9126679ba58e0474)
    //   configuration: {
    //     type: 'AVAILABLE'
    //   }
    let data = Bytes::from_static(&hex!(
    "2bb15b3dca504c44c452e1ab606036846b89584e64b6ecea4083507c3d1f2d91a39243"
    "8a0998ceab603e156d2f2458ee91c4479fec7129e17172c84bc61c28040028b52ffd00"
    "58150400540785a474797065bb7265616c6d5f617263686976696e675f636572746966"
    "6963617465a6617574686f72d802de10a11cec001000a974696d657374616d70d70100"
    "059b08c1fa4000a86964d8024486e7cf02d747bd9126679ba58e0474ad636f6e666967"
    "75726174696f6e81a9415641494c41424c450300db43bc4c186c080005"
    ));
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = RealmArchivingCertificate {
        author: alice.device_id,
        timestamp: "2020-01-01T00:00:00Z".parse().unwrap(),
        realm_id: VlobID::from_hex("4486e7cf02d747bd9126679ba58e0474").unwrap(),
        configuration: RealmArchivingConfiguration::Available,
    };

    let unsecure_certif = RealmArchivingCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), alice.device_id);
    p_assert_eq!(
        unsecure_certif
            .verify_signature(&alice.verify_key())
            .unwrap(),
        (expected.clone(), data.clone())
    );

    let unsecure_certif = RealmArchivingCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(
        unsecure_certif.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage),
        expected
    );

    let certif = RealmArchivingCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        alice.device_id,
        None,
    )
    .unwrap();
    p_assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = RealmArchivingCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        alice.device_id,
        None,
    )
    .unwrap();
    p_assert_eq!(certif2, expected);

    // Test invalid data
    p_assert_matches!(
        RealmArchivingCertificate::unsecure_load(b"dummy".to_vec().into()),
        Err(DataError::Signature)
    );
    p_assert_matches!(
        RealmArchivingCertificate::verify_and_load(
            b"dummy",
            &alice.verify_key(),
            alice.device_id,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_realm_archiving_certificate_archived(alice: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'realm_archiving_certificate'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1577836800000000) i.e. 2020-01-01T01:00:00Z
    //   realm_id: ext(2, 0x4486e7cf02d747bd9126679ba58e0474)
    //   configuration: {
    //     type: 'ARCHIVED'
    //   }
    let data = Bytes::from_static(&hex!(
    "451cf7de92f4a8825d264c6249919bbe0a9bf191d3fd842f58a95b8a5894b23f413124"
    "6cb2c4934bdcc68d809f3b31a4971e71ecf21b9f612961c5128d13df030028b52ffd00"
    "580d0400440785a474797065bb7265616c6d5f617263686976696e675f636572746966"
    "6963617465a6617574686f72d802de10a11cec001000a974696d657374616d70d70100"
    "059b08c1fa4000a86964d8024486e7cf02d747bd9126679ba58e0474ad636f6e666967"
    "75726174696f6e81a841524348495645440300db43bc4c186c080005"
    ));
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = RealmArchivingCertificate {
        author: alice.device_id,
        timestamp: "2020-01-01T00:00:00Z".parse().unwrap(),
        realm_id: VlobID::from_hex("4486e7cf02d747bd9126679ba58e0474").unwrap(),
        configuration: RealmArchivingConfiguration::Archived,
    };

    let unsecure_certif = RealmArchivingCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), alice.device_id);
    p_assert_eq!(
        unsecure_certif
            .verify_signature(&alice.verify_key())
            .unwrap(),
        (expected.clone(), data.clone())
    );

    let unsecure_certif = RealmArchivingCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(
        unsecure_certif.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage),
        expected
    );

    let certif = RealmArchivingCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        alice.device_id,
        None,
    )
    .unwrap();
    p_assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = RealmArchivingCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        alice.device_id,
        None,
    )
    .unwrap();
    p_assert_eq!(certif2, expected);

    // Test invalid data
    p_assert_matches!(
        RealmArchivingCertificate::unsecure_load(b"dummy".to_vec().into()),
        Err(DataError::Signature)
    );
    p_assert_matches!(
        RealmArchivingCertificate::verify_and_load(
            b"dummy",
            &alice.verify_key(),
            alice.device_id,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_realm_archiving_certificate_deletion_planned(alice: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'realm_archiving_certificate'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1577836800000000) i.e. 2020-01-01T01:00:00Z
    //   realm_id: ext(2, 0x4486e7cf02d747bd9126679ba58e0474)
    //   configuration: {
    //     type: 'DELETION_PLANNED',
    //     deletion_date: ext(1, 1580428800000000) i.e. 2020-01-31T00:00:00Z
    //   }
    let data = Bytes::from_static(&hex!(
    "408e7c6d0df49360ae42550e6d820cddf58e6398b7081228f6544ade4f8595bc38dfb3"
    "ed9e8386599fa528f5867a0938a8c32b6ce4d147983bdcb085a41bb3070028b52ffd00"
    "580d0500440985a474797065bb7265616c6d5f617263686976696e675f636572746966"
    "6963617465a6617574686f72d802de10a11cec001000a974696d657374616d70d70100"
    "059b08c1fa4000a86964d8024486e7cf02d747bd9126679ba58e0474ad636f6e666967"
    "75726174696f6e82b044454c4554494f4e5f504c414e4e4544ad64656c6574696f6e5f"
    "64617465d70100059d64413780000300db43bc4c186c080005"
    ));
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = RealmArchivingCertificate {
        author: alice.device_id,
        timestamp: "2020-01-01T00:00:00Z".parse().unwrap(),
        realm_id: VlobID::from_hex("4486e7cf02d747bd9126679ba58e0474").unwrap(),
        configuration: RealmArchivingConfiguration::DeletionPlanned {
            deletion_date: "2020-01-31T00:00:00Z".parse().unwrap(),
        },
    };

    let unsecure_certif = RealmArchivingCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), alice.device_id);
    p_assert_eq!(
        unsecure_certif
            .verify_signature(&alice.verify_key())
            .unwrap(),
        (expected.clone(), data.clone())
    );

    let unsecure_certif = RealmArchivingCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(
        unsecure_certif.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage),
        expected
    );

    let certif = RealmArchivingCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        alice.device_id,
        None,
    )
    .unwrap();
    p_assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = RealmArchivingCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        alice.device_id,
        None,
    )
    .unwrap();
    p_assert_eq!(certif2, expected);

    // Test invalid data
    p_assert_matches!(
        RealmArchivingCertificate::unsecure_load(b"dummy".to_vec().into()),
        Err(DataError::Signature)
    );
    p_assert_matches!(
        RealmArchivingCertificate::verify_and_load(
            b"dummy",
            &alice.verify_key(),
            alice.device_id,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_realm_name_certificate(alice: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'realm_name_certificate'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   realm_id: ext(2, 0x4486e7cf02d747bd9126679ba58e0474)
    //   key_index: 42
    //   encrypted_name: 0x3132333435
    let data = Bytes::from_static(&hex!(
    "c81a75b58276ebd43e7d483a466490e08bcfa4e095b02d49129a24a6e6fe5ed144033b"
    "08a52bc0f793553fa7e25ae93a6f7b2eb60bbb98307f434cd2fef0cb0e0028b52ffd00"
    "58050400340786a474797065b67265616c6d5f6e616d655f6365727469666963617465"
    "a6617574686f72d802de10a11cec001000a974696d657374616d70d7010005d250a226"
    "9a75a86964d8024486e7cf02d747bd9126679ba58e0474a96b65795f696e6465782aae"
    "656e63727970746564c405313233343503008943944718d704d003"
    ));
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = RealmNameCertificate {
        author: alice.device_id,
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        realm_id: VlobID::from_hex("4486e7cf02d747bd9126679ba58e0474").unwrap(),
        key_index: 42,
        encrypted_name: b"12345".to_vec(),
    };

    let unsecure_certif = RealmNameCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), alice.device_id);
    p_assert_eq!(
        unsecure_certif
            .verify_signature(&alice.verify_key())
            .unwrap(),
        (expected.clone(), data.clone())
    );

    let unsecure_certif = RealmNameCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(
        unsecure_certif.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage),
        expected
    );

    let certif =
        RealmNameCertificate::verify_and_load(&data, &alice.verify_key(), alice.device_id, None)
            .unwrap();
    p_assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 =
        RealmNameCertificate::verify_and_load(&data2, &alice.verify_key(), alice.device_id, None)
            .unwrap();
    p_assert_eq!(certif2, expected);

    // Test invalid data
    p_assert_matches!(
        RealmNameCertificate::unsecure_load(b"dummy".to_vec().into()),
        Err(DataError::Signature)
    );
    p_assert_matches!(
        RealmNameCertificate::verify_and_load(b"dummy", &alice.verify_key(), alice.device_id, None),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_realm_key_rotation_certificate(alice: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'realm_key_rotation_certificate'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   realm_id: ext(2, 0x4486e7cf02d747bd9126679ba58e0474)
    //   key_index: 42
    //   encryption_algorithm: 'BLAKE2_XSALSA20_POLY1305'
    //   hash_algorithm: 'SHA256'
    //   key_canary: 0x3132333435
    let data = Bytes::from_static(&hex!(
    "d7f22813f50d7e9c388e022ccc8809c6b7c39958b67afbfd2a73a83166d828938699f8"
    "6f14d40cb369a5a7e201bc288a276ea5a32a861b234eb6f844da14ba050028b52ffd00"
    "58050600140b88a474797065be7265616c6d5f6b65795f726f746174696f6e5f636572"
    "7469666963617465a6617574686f72d802de10a11cec001000a974696d657374616d70"
    "d7010005d250a2269a75a86964d8024486e7cf02d747bd9126679ba58e0474a96b6579"
    "5f696e6465782ab4656e63727970616c676f726974686db8424c414b45325f5853414c"
    "534132305f504f4c5931333035ae68617368a6534841323536aa6b65795f63616e6172"
    "79c405313233343504005fbacf6db4f709981901a0"
    ));
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = RealmKeyRotationCertificate {
        author: alice.device_id,
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        realm_id: VlobID::from_hex("4486e7cf02d747bd9126679ba58e0474").unwrap(),
        key_index: 42,
        encryption_algorithm: SecretKeyAlgorithm::Blake2bXsalsa20Poly1305,
        hash_algorithm: HashAlgorithm::Sha256,
        key_canary: b"12345".to_vec(),
    };

    let unsecure_certif = RealmKeyRotationCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), alice.device_id);
    p_assert_eq!(
        unsecure_certif
            .verify_signature(&alice.verify_key())
            .unwrap(),
        (expected.clone(), data.clone())
    );

    let unsecure_certif = RealmKeyRotationCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(
        unsecure_certif.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage),
        expected
    );

    let certif = RealmKeyRotationCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        alice.device_id,
        None,
    )
    .unwrap();
    p_assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = RealmKeyRotationCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        alice.device_id,
        None,
    )
    .unwrap();
    p_assert_eq!(certif2, expected);

    // Test invalid data
    p_assert_matches!(
        RealmKeyRotationCertificate::unsecure_load(b"dummy".to_vec().into()),
        Err(DataError::Signature)
    );
    p_assert_matches!(
        RealmKeyRotationCertificate::verify_and_load(
            b"dummy",
            &alice.verify_key(),
            alice.device_id,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_shamir_recovery_share_certificate(alice: &Device, bob: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'shamir_recovery_share_certificate'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1577836800000000) i.e. 2020-01-01T01:00:00Z
    //   user_id: ext(2, 0xa11cec00100000000000000000000000)
    //   recipient: ext(2, 0x808c0010000000000000000000000000)
    //   ciphered_share: 0x61626364
    let data = Bytes::from_static(&hex!(
    "bce7135eb518046ae92574cc54509c45b2c423cc3e31d9e136df6dae6a8dc86c796e78"
    "3649c6687f9fc38700d69af3fde311fc379eba242f615fdcf6cf4624040028b52ffd00"
    "581d0400a2c71c25a0291d9fe588161aae62767d2c44631d7796c8b22ce986052dad94"
    "a3e255ed6851289bbbc94c0f19b0b0f39b5ba5206263a1a64e7cddea7982a826b68bff"
    "22091a3695814380808b7bb2df9fed093aa8213c9c8c9a186a1d65e49dba886dbeae73"
    "b6b724be02d2f3e27db2bd02e64e6a08230400011b89128ce553d2e90ba0"
    ));
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = ShamirRecoveryShareCertificate {
        author: alice.device_id,
        timestamp: "2020-01-01T00:00:00Z".parse().unwrap(),
        user_id: alice.user_id,
        recipient: bob.user_id,
        ciphered_share: b"abcd".to_vec(),
    };

    let unsecure_certif = ShamirRecoveryShareCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), alice.device_id);
    p_assert_eq!(
        unsecure_certif
            .verify_signature(&alice.verify_key())
            .unwrap(),
        (expected.clone(), data.clone())
    );

    let unsecure_certif = ShamirRecoveryShareCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(
        unsecure_certif.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage),
        expected
    );

    let certif = ShamirRecoveryShareCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        alice.device_id,
        Some(bob.user_id),
    )
    .unwrap();
    p_assert_eq!(certif, expected);

    // Test bad recipient

    let err = ShamirRecoveryShareCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        alice.device_id,
        Some(alice.user_id),
    )
    .unwrap_err();
    p_assert_matches!(err, DataError::UnexpectedUserID { .. });

    // Test bad author

    let err = ShamirRecoveryShareCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        bob.device_id,
        Some(bob.user_id),
    )
    .unwrap_err();
    p_assert_matches!(err, DataError::UnexpectedAuthor { .. });

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = ShamirRecoveryShareCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        alice.device_id,
        Some(bob.user_id),
    )
    .unwrap();
    p_assert_eq!(certif2, expected);

    // Test invalid data
    p_assert_matches!(
        ShamirRecoveryShareCertificate::unsecure_load(b"dummy".to_vec().into()),
        Err(DataError::Signature)
    );
    p_assert_matches!(
        ShamirRecoveryShareCertificate::verify_and_load(
            b"dummy",
            &alice.verify_key(),
            alice.device_id,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_shamir_recovery_brief_certificate(alice: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'shamir_recovery_brief_certificate'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1577836800000000) i.e. 2020-01-01T01:00:00Z
    //   user_id: ext(2, 0xa11cec00100000000000000000000000)
    //   threshold: 3
    //   per_recipient_shares: {
    //     ext(2, 0xd1444010000000000000000000000000): 1,
    //     ext(2, 0x808c0010000000000000000000000000): 2,
    //     ext(2, 0xca470010000000000000000000000000): 1,
    //   }
    let data = &hex!(
    "0c2439269a6f413a2aecc26e33c91873b5e35b02bc2ca580afabf38e44a91e0d7088f7"
    "c21d36b0843ea2179ba2c683ab2f3c478d14e3eae44548cbe5a1b797000028b52ffd00"
    "58dd0400c288212a9045e90f3e85633d86a08cced60feb85ed10043f91a559d250d981"
    "4812bb5c5b85ff212076c370db3b05ec7069b3fd7f30da64ff1f03a6cd86547e50ba56"
    "e8c6d5580944090a4a65d8d7df66b48e5556943f01960144709689d36ba9d3f675e700"
    "8488b3699329d8d59b683f68a56d56c849ba8c7016eb8282941deff4a02c68345a2d71"
    "0406003692d318f91061406301f894ccfa823d"
    );
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = ShamirRecoveryBriefCertificate {
        author: alice.device_id,
        timestamp: "2020-01-01T00:00:00Z".parse().unwrap(),
        user_id: alice.user_id,
        threshold: 3.try_into().unwrap(),
        per_recipient_shares: HashMap::from([
            ("bob".parse().unwrap(), 2.try_into().unwrap()),
            ("carl".parse().unwrap(), 1.try_into().unwrap()),
            ("diana".parse().unwrap(), 1.try_into().unwrap()),
        ]),
    };

    let unsecure_certif = ShamirRecoveryBriefCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), alice.device_id);
    p_assert_eq!(
        unsecure_certif
            .verify_signature(&alice.verify_key())
            .unwrap(),
        (expected.clone(), data.clone())
    );

    let unsecure_certif = ShamirRecoveryBriefCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(
        unsecure_certif.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage),
        expected
    );

    let certif = ShamirRecoveryBriefCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        alice.device_id,
    )
    .unwrap();
    p_assert_eq!(certif, expected);

    // Test bad author

    let err = ShamirRecoveryBriefCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        DeviceID::default(),
    )
    .unwrap_err();
    p_assert_matches!(err, DataError::UnexpectedAuthor { .. });

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = ShamirRecoveryBriefCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        alice.device_id,
    )
    .unwrap();
    p_assert_eq!(certif2, expected);

    // Test invalid data
    p_assert_matches!(
        ShamirRecoveryBriefCertificate::unsecure_load(b"dummy".to_vec().into()),
        Err(DataError::Signature)
    );
    p_assert_matches!(
        ShamirRecoveryBriefCertificate::verify_and_load(
            b"dummy",
            &alice.verify_key(),
            alice.device_id,
        ),
        Err(DataError::Signature)
    );

    // Test threshold greater than shares
    let data = hex!(
        "d524e93060fbdcd68b0d48903a669742839704df18fd5ba753aaf5ac343a70ca4fae30"
        "19b36351a50bbc2cfb6fb14bd69a58a593d64a153abd66ecdf54cf50070028b52ffd00"
        "58c5040082c8202a9045e90f3e85633d86a08cced60feb85ed10043f91a559d250d981"
        "246ebb5c5b85ff212076030c37910110c1c26653a8b0c954b5a1607321cf0ecad6886f"
        "dcc789131d2828cfb02f5536a36d7c5949140aa0041c8e369738b9969fb62f7d1440f0"
        "78326c3205bb7291b77b2b6df35f93be6c68163f41bd73e39d1c9403cca3d510460600"
        "402e36920b31063416804fc9ac2fd803"

    );
    let data = Bytes::from(data.as_ref().to_vec());

    p_assert_matches!(
        ShamirRecoveryBriefCertificate::unsecure_load(data.clone()),
        Err(DataError::DataIntegrity {
            data_type: "libparsec_types::certif::ShamirRecoveryBriefCertificate",
            invariant: "threshold <= total_shares"
        })
    );
    p_assert_matches!(
        ShamirRecoveryBriefCertificate::verify_and_load(
            &data,
            &alice.verify_key(),
            alice.device_id,
        ),
        Err(DataError::DataIntegrity {
            data_type: "libparsec_types::certif::ShamirRecoveryBriefCertificate",
            invariant: "threshold <= total_shares"
        })
    );
}

#[rstest]
fn serde_sequester_authority_certificate(alice: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'sequester_authority_certificate'
    //   timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    //   verify_key_der: 0x30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62cb38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195f3bcc72ab57207ebfd0203010001
    let data = Bytes::from_static(&hex!(
    "60c6e02a3e10edd18845cbd9f1c1bca44905b9066a52717c1aec420add59c644649bea"
    "af363a23baaeb1fbbb2801aa4812651d985fe0387b9296786f9a0eb0040028b52ffd00"
    "5869070083a474797065bf7365717565737465725f617574686f726974795f63657274"
    "69666963617465a974696d657374616d70d70100035d162fa2e400ae7665726966795f"
    "6b65795f646572c4a230819f300d06092a864886f70d010101050003818d0030818902"
    "818100b2dc00a3c3b5c689b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf"
    "624400256cc660d5084782738a3045d75b584c1943bc04c7123d68ac0cef253b4ee8d7"
    "9bd09da19162dcc083662269b7b62cb38582f8a30219047b087c11b60184b0493e0c1c"
    "8b1d10f9d7e6a2eb5aff66f7ee18303195f3bcc72ab57207ebfd0203010001"
    ));
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = SequesterAuthorityCertificate {
        timestamp: "2000-01-02T01:00:00Z".parse().unwrap(),
        verify_key_der: SequesterVerifyKeyDer::try_from(
            &hex!(
                "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5"
                "c689b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d508"
                "4782738a3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083"
                "662269b7b62cb38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5a"
                "ff66f7ee18303195f3bcc72ab57207ebfd0203010001"
            )[..],
        )
        .unwrap(),
    };
    let certif =
        SequesterAuthorityCertificate::verify_and_load(&data, &alice.verify_key()).unwrap();

    p_assert_eq!(certif, expected);

    let unsecure_certif = SequesterAuthorityCertificate::unsecure_load(data).unwrap();
    p_assert_eq!(
        unsecure_certif.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage),
        expected
    );

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 =
        SequesterAuthorityCertificate::verify_and_load(&data2, &alice.verify_key()).unwrap();
    p_assert_eq!(certif2, expected);

    // Test invalid data
    p_assert_matches!(
        SequesterAuthorityCertificate::unsecure_load(b"dummy".to_vec().into()),
        Err(DataError::Signature)
    );
    p_assert_matches!(
        SequesterAuthorityCertificate::verify_and_load(b"dummy", &alice.verify_key()),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_sequester_service_certificate() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'sequester_service_certificate'
    //   timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    //   service_id: ext(2, 0xb5eb565343c442b3a26be44573813ff0)
    //   service_label: 'foo'
    //   encryption_key_der: 0x30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62cb38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195f3bcc72ab57207ebfd0203010001
    let data = Bytes::from_static(&hex!(
    "0028b52ffd0058f1080085a474797065bd7365717565737465725f736572766963655f"
    "6365727469666963617465a974696d657374616d70d70100035d162fa2e400aa736572"
    "766963655f6964d802b5eb565343c442b3a26be44573813ff0ad736572766963655f6c"
    "6162656ca3666f6fb2656e6372797074696f6e5f6b65795f646572c4a230819f300d06"
    "092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689b069f3f4"
    "0c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a3045"
    "d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62c"
    "b38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee1830"
    "3195f3bcc72ab57207ebfd0203010001"
    ));

    let expected = SequesterServiceCertificate {
        timestamp: "2000-01-02T01:00:00Z".parse().unwrap(),
        service_id: SequesterServiceID::from_hex("b5eb565343c442b3a26be44573813ff0").unwrap(),
        service_label: "foo".into(),
        encryption_key_der: SequesterPublicKeyDer::try_from(
            &hex!(
                "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5"
                "c689b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d508"
                "4782738a3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083"
                "662269b7b62cb38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5a"
                "ff66f7ee18303195f3bcc72ab57207ebfd0203010001"
            )[..],
        )
        .unwrap(),
    };
    let certif = SequesterServiceCertificate::load(&data).unwrap();

    p_assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump();
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = SequesterServiceCertificate::load(&data2).unwrap();
    p_assert_eq!(certif2, expected);

    // Test invalid data
    let outcome = SequesterServiceCertificate::load(b"dummy");
    assert_eq!(
        outcome,
        Err(DataError::BadSerialization {
            format: None,
            step: "format detection"
        })
    );
}

// TODO: check sequester service certificate signed with actual DER key
