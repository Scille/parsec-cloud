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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: user_certificate
    //   author: ext(2, de10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   user_id: ext(2, 808c0010000000000000000000000000)
    //   human_handle: ['bob@example.com', 'Boby McBobFace']
    //   public_key: 7c999e9980bef37707068b07d975591efc56335be9634ceef7c932a09c891e25
    //   algorithm: X25519_XSALSA20_POLY1305
    //   profile: STANDARD
    let data = Bytes::from_static(&hex!(
        "47450a97d5b15191dd26828a285a76c657d3e62d0079ce598924099039355a6e8ee566"
        "5b7cfac1764e8406cbd8605f119c4266d746cfdf0ae501ec312152df0cff88a4747970"
        "65b0757365725f6365727469666963617465a6617574686f72d802de10a11cec001000"
        "0000000000000000a974696d657374616d70d7010005d250a2269a75a7757365725f69"
        "64d802808c0010000000000000000000000000ac68756d616e5f68616e646c6592af62"
        "6f62406578616d706c652e636f6dae426f6279204d63426f6246616365aa7075626c69"
        "635f6b6579c4207c999e9980bef37707068b07d975591efc56335be9634ceef7c932a0"
        "9c891e25a9616c676f726974686db85832353531395f5853414c534132305f504f4c59"
        "31333035a770726f66696c65a85354414e44415244"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: user_certificate
    //   author: ext(2, de10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   user_id: ext(2, 808c0010000000000000000000000000)
    //   human_handle: None
    //   public_key: 7c999e9980bef37707068b07d975591efc56335be9634ceef7c932a09c891e25
    //   algorithm: X25519_XSALSA20_POLY1305
    //   profile: STANDARD
    let data = Bytes::from_static(&hex!(
        "66adf9f0d950658ee38d4154b4957c59cd2804bb491a783fb5c60d4b2e033138f390fc"
        "0c6658ab0b406ad6369544559527168adeb61a7fa5b4598a65e4cce20eff88a4747970"
        "65b0757365725f6365727469666963617465a6617574686f72d802de10a11cec001000"
        "0000000000000000a974696d657374616d70d7010005d250a2269a75a7757365725f69"
        "64d802808c0010000000000000000000000000ac68756d616e5f68616e646c65c0aa70"
        "75626c69635f6b6579c4207c999e9980bef37707068b07d975591efc56335be9634cee"
        "f7c932a09c891e25a9616c676f726974686db85832353531395f5853414c534132305f"
        "504f4c5931333035a770726f66696c65a85354414e44415244"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: device_certificate
    //   author: ext(2, de10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   user_id: ext(2, a11cec00100000000000000000000000)
    //   device_id: ext(2, de10808c001000000000000000000000)
    //   device_label: My dev1 machine
    //   verify_key: 840d872f4252da2d1c9f81a77db5f0a5b9b60a5cde1eeabf40388ef6bca64909
    //   algorithm: ED25519
    let data = Bytes::from_static(&hex!(
        "dc217cebca09b8b3ed3c3a4a77ea10f183b9949799f6fa59ba2815b8d6a476d02f117b"
        "6182c056dd0c9e2551831f700aff6bc0543c2c77ec5d12d11edd12880cff88a4747970"
        "65b26465766963655f6365727469666963617465a6617574686f72d802de10a11cec00"
        "10000000000000000000a974696d657374616d70d7010005d250a2269a75a775736572"
        "5f6964d802a11cec00100000000000000000000000a96465766963655f6964d802de10"
        "808c001000000000000000000000ac6465766963655f6c6162656caf4d792064657631"
        "206d616368696e65aa7665726966795f6b6579c420840d872f4252da2d1c9f81a77db5"
        "f0a5b9b60a5cde1eeabf40388ef6bca64909a9616c676f726974686da7454432353531"
        "39"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: device_certificate
    //   author: ext(2, de10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   user_id: ext(2, a11cec00100000000000000000000000)
    //   device_id: ext(2, de10808c001000000000000000000000)
    //   device_label: None
    //   verify_key: 840d872f4252da2d1c9f81a77db5f0a5b9b60a5cde1eeabf40388ef6bca64909
    //   algorithm: ED25519
    let data = Bytes::from_static(&hex!(
        "6372a56378037182e782b7846be62e06218cee7b107e30403e95bd9db449c8e4e5ba45"
        "9bf6d80f04540bc127600d09839ae6595d0e445a89b87c0949ec15fc04ff88a4747970"
        "65b26465766963655f6365727469666963617465a6617574686f72d802de10a11cec00"
        "10000000000000000000a974696d657374616d70d7010005d250a2269a75a775736572"
        "5f6964d802a11cec00100000000000000000000000a96465766963655f6964d802de10"
        "808c001000000000000000000000ac6465766963655f6c6162656cc0aa766572696679"
        "5f6b6579c420840d872f4252da2d1c9f81a77db5f0a5b9b60a5cde1eeabf40388ef6bc"
        "a64909a9616c676f726974686da745443235353139"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: revoked_user_certificate
    //   author: ext(2, de10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   user_id: ext(2, 808c0010000000000000000000000000)
    let data = Bytes::from_static(&hex!(
        "f03f69d6098a9b13a2b39624cab76f151740d5f7c1ea875a7cb5bf74579218a53f0962"
        "d35dab1c682c3f7d45f35c93d4fc8035382df2053aac759556428bdb00ff84a4747970"
        "65b87265766f6b65645f757365725f6365727469666963617465a6617574686f72d802"
        "de10a11cec0010000000000000000000a974696d657374616d70d7010005d250a2269a"
        "75a7757365725f6964d802808c0010000000000000000000000000"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: user_update_certificate
    //   author: ext(2, de10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   user_id: ext(2, 808c0010000000000000000000000000)
    //   new_profile: OUTSIDER
    let data = Bytes::from_static(&hex!(
        "ad195fd99016f8aa88f544fd072c8b188cece24338eda7f2b5e4d87f900b53b7335d53"
        "1049d4184413e52d115bad9e3e7a30bbba83b8f742edfd3a9e472ca900ff85a4747970"
        "65b7757365725f7570646174655f6365727469666963617465a6617574686f72d802de"
        "10a11cec0010000000000000000000a974696d657374616d70d7010005d250a2269a75"
        "a7757365725f6964d802808c0010000000000000000000000000ab6e65775f70726f66"
        "696c65a84f55545349444552"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: realm_role_certificate
    //   author: ext(2, de10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   realm_id: ext(2, 4486e7cf02d747bd9126679ba58e0474)
    //   user_id: ext(2, 808c0010000000000000000000000000)
    //   role: OWNER
    let data = Bytes::from_static(&hex!(
        "4f85b51dfb534eb55ebf7a107fda12d38c17e7404e8632eef16e2183a79beb92ad5f76"
        "0188b2bb9dbde1da48604f7231a688037bd702df47f9877de23bd11a0cff86a4747970"
        "65b67265616c6d5f726f6c655f6365727469666963617465a6617574686f72d802de10"
        "a11cec0010000000000000000000a974696d657374616d70d7010005d250a2269a75a8"
        "7265616c6d5f6964d8024486e7cf02d747bd9126679ba58e0474a7757365725f6964d8"
        "02808c0010000000000000000000000000a4726f6c65a54f574e4552"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: realm_role_certificate
    //   author: ext(2, de10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   realm_id: ext(2, 4486e7cf02d747bd9126679ba58e0474)
    //   user_id: ext(2, 808c0010000000000000000000000000)
    //   role: None
    let data = Bytes::from_static(&hex!(
        "47342b9ff8476003fbaa2d00989a481236197d695cafa134a4fb4626425008f6866686"
        "a478883d311a8c1b31ff2930a14da8b390199a4d190cefa14f06517f04ff86a4747970"
        "65b67265616c6d5f726f6c655f6365727469666963617465a6617574686f72d802de10"
        "a11cec0010000000000000000000a974696d657374616d70d7010005d250a2269a75a8"
        "7265616c6d5f6964d8024486e7cf02d747bd9126679ba58e0474a7757365725f6964d8"
        "02808c0010000000000000000000000000a4726f6c65c0"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: realm_archiving_certificate
    //   author: ext(2, de10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1577836800000000) i.e. 2020-01-01T01:00:00Z
    //   realm_id: ext(2, 4486e7cf02d747bd9126679ba58e0474)
    //   configuration: {'type': 'AVAILABLE'}
    let data = Bytes::from_static(&hex!(
        "4a0f7c4399ac2aced8833bdf1c32871374536e82fbd5b3a39f3253e2b7da5da9158d2e"
        "455b874237079bdd5ad0501e2985d3dc2da96266169a58f16f432d2707ff85a4747970"
        "65bb7265616c6d5f617263686976696e675f6365727469666963617465a6617574686f"
        "72d802de10a11cec0010000000000000000000a974696d657374616d70d70100059b08"
        "c1fa4000a87265616c6d5f6964d8024486e7cf02d747bd9126679ba58e0474ad636f6e"
        "66696775726174696f6e81a474797065a9415641494c41424c45"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: realm_archiving_certificate
    //   author: ext(2, de10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1577836800000000) i.e. 2020-01-01T01:00:00Z
    //   realm_id: ext(2, 4486e7cf02d747bd9126679ba58e0474)
    //   configuration: {'type': 'ARCHIVED'}
    let data = Bytes::from_static(&hex!(
        "abdeb92c23c13f0f9c60d037f8ef8d9421309b67e74545d66df37a99b4a181cda6164e"
        "5125b70ac4add2b9e5850a644268c519b5cbc0a8def68a0d43df28ca06ff85a4747970"
        "65bb7265616c6d5f617263686976696e675f6365727469666963617465a6617574686f"
        "72d802de10a11cec0010000000000000000000a974696d657374616d70d70100059b08"
        "c1fa4000a87265616c6d5f6964d8024486e7cf02d747bd9126679ba58e0474ad636f6e"
        "66696775726174696f6e81a474797065a84152434849564544"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: realm_archiving_certificate
    //   author: ext(2, de10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1577836800000000) i.e. 2020-01-01T01:00:00Z
    //   realm_id: ext(2, 4486e7cf02d747bd9126679ba58e0474)
    //   configuration: {'type': 'DELETION_PLANNED', 'deletion_date': ExtType(code=1, data=b'\x00\x05\x9ddA7\x80\x00')}
    let data = Bytes::from_static(&hex!(
        "39c3377a657a32aaf46213497d65276d57391a143ac6f15bd3462ab746cea879a12f6a"
        "1fcdcb9efdd0d334c472b4086a3b0650569516f369089c8aa4f0d9a805ff85a4747970"
        "65bb7265616c6d5f617263686976696e675f6365727469666963617465a6617574686f"
        "72d802de10a11cec0010000000000000000000a974696d657374616d70d70100059b08"
        "c1fa4000a87265616c6d5f6964d8024486e7cf02d747bd9126679ba58e0474ad636f6e"
        "66696775726174696f6e82a474797065b044454c4554494f4e5f504c414e4e4544ad64"
        "656c6574696f6e5f64617465d70100059d6441378000"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: realm_name_certificate
    //   author: ext(2, de10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   realm_id: ext(2, 4486e7cf02d747bd9126679ba58e0474)
    //   key_index: 42
    //   encrypted_name: 3132333435
    let data = Bytes::from_static(&hex!(
        "59af6356a7348835175d43bfec635376963136b486d4c7812d4c44968e510e5788b9f1"
        "90f1fb7372233178821dbf211d2a46ba928943d2bdeb58a44afad1ec0cff86a4747970"
        "65b67265616c6d5f6e616d655f6365727469666963617465a6617574686f72d802de10"
        "a11cec0010000000000000000000a974696d657374616d70d7010005d250a2269a75a8"
        "7265616c6d5f6964d8024486e7cf02d747bd9126679ba58e0474a96b65795f696e6465"
        "782aae656e637279707465645f6e616d65c4053132333435"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: realm_key_rotation_certificate
    //   author: ext(2, de10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   realm_id: ext(2, 4486e7cf02d747bd9126679ba58e0474)
    //   key_index: 42
    //   encryption_algorithm: BLAKE2_XSALSA20_POLY1305
    //   hash_algorithm: SHA256
    //   key_canary: 3132333435
    let data = Bytes::from_static(&hex!(
        "30ef7b46c6c6c875485a67f3873986fc1b0e4fbed17c54edf45250da169b755c376c69"
        "567a7e91268559227e21b0ff14083b3efe9af5b7ed3d0e02d05acb1909ff88a4747970"
        "65be7265616c6d5f6b65795f726f746174696f6e5f6365727469666963617465a66175"
        "74686f72d802de10a11cec0010000000000000000000a974696d657374616d70d70100"
        "05d250a2269a75a87265616c6d5f6964d8024486e7cf02d747bd9126679ba58e0474a9"
        "6b65795f696e6465782ab4656e6372797074696f6e5f616c676f726974686db8424c41"
        "4b45325f5853414c534132305f504f4c5931333035ae686173685f616c676f72697468"
        "6da6534841323536aa6b65795f63616e617279c4053132333435"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: shamir_recovery_share_certificate
    //   author: ext(2, de10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1577836800000000) i.e. 2020-01-01T01:00:00Z
    //   user_id: ext(2, a11cec00100000000000000000000000)
    //   recipient: ext(2, 808c0010000000000000000000000000)
    //   ciphered_share: 61626364
    let data = Bytes::from_static(&hex!(
        "b11d8ecf86af5c8c5ed215d63f985ad4d264cf985d4c4d25eeb3aa5059142eaa106f85"
        "6ef586a5f36bae19a547c850032583558b433396598f39b46aba6a3c00ff86a4747970"
        "65d9217368616d69725f7265636f766572795f73686172655f63657274696669636174"
        "65a6617574686f72d802de10a11cec0010000000000000000000a974696d657374616d"
        "70d70100059b08c1fa4000a7757365725f6964d802a11cec0010000000000000000000"
        "0000a9726563697069656e74d802808c0010000000000000000000000000ae63697068"
        "657265645f7368617265c40461626364"
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
    // Generated from Rust implementation (Parsec v2.16.1+dev)
    // Content:
    //   type: "shamir_recovery_brief_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1577836800.0)
    //   per_recipient_shares: {"bob": 2, "carl": 1, "diana": 1}
    //   threshold: 3
    //
    let data = hex!(
        "1e9d752322f4185a97a3328325c043ffb4f3ebee4e9aac4e9078af5e6c047a2be3be61"
        "996fc6ed1ffa4643b75c3655537c45ff2b083306f2d9a3ae17e5818b050028b52ffd00"
        "58cd04009208212a9045e90f3e85633d86a08cced60feb85ed10043f91a559d250d981"
        "2412bb5c5b85ff212076c370db5b06ec3160da6c0f469b4c550f97361bf2fca074adf8"
        "c67dac04a20405e519f6a5da66b48e2f2b8a3e01960144a02c13a7d7f2d3f6a58e0210"
        "22cea64da660576ff27ed04adb7cc8495f46388bbfa0a0b3e39d1e9405cda3d5124706"
        "00e02836920b31063416804fc9ac2fd803"

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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: sequester_authority_certificate
    //   timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    //   verify_key_der: 30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62cb38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195f3bcc72ab57207ebfd0203010001
    let data = Bytes::from_static(&hex!(
        "b73d3afa988dbe02cf9b2f2f3f6914a29127fb9ffafa6490087b8f617cbbd8ecc20f81"
        "f00482e9d240894277a8c7c9a16a8f8beba0d140597a9dd73111310d03ff83a4747970"
        "65bf7365717565737465725f617574686f726974795f6365727469666963617465a974"
        "696d657374616d70d70100035d162fa2e400ae7665726966795f6b65795f646572c4a2"
        "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5"
        "c689b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d508"
        "4782738a3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083"
        "662269b7b62cb38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5a"
        "ff66f7ee18303195f3bcc72ab57207ebfd0203010001"
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
#[ignore = "TODO: scheme has changed, must regenerate the dump"]
fn serde_sequester_service_certificate() {
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: sequester_service_certificate
    //   timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    //   service_id: ext(2, b5eb565343c442b3a26be44573813ff0)
    //   service_label: foo
    //   encryption_key_der: 30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62cb38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195f3bcc72ab57207ebfd0203010001
    let data = Bytes::from_static(&hex!(
        "ff85a474797065bd7365717565737465725f736572766963655f636572746966696361"
        "7465a974696d657374616d70d70100035d162fa2e400aa736572766963655f6964d802"
        "b5eb565343c442b3a26be44573813ff0ad736572766963655f6c6162656ca3666f6fb2"
        "656e6372797074696f6e5f6b65795f646572c4a230819f300d06092a864886f70d0101"
        "01050003818d0030818902818100b2dc00a3c3b5c689b069f3f40c494d2a5be313b103"
        "4fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a3045d75b584c1943bc04c7"
        "123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62cb38582f8a30219047b"
        "087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195f3bcc72ab57207"
        "ebfd0203010001"
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
