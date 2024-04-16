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
        author: CertificateSignerOwned::User(alice.device_id.clone()),
        timestamp,
        user_id: bob.user_id().clone(),
        human_handle: MaybeRedacted::Real(bob.human_handle.clone()),
        public_key: bob.public_key(),
        algorithm: PrivateKeyAlgorithm::X25519XSalsa20Poly1305,
        profile: UserProfile::Standard,
    };
    p_assert_eq!(
        format!("{:?}", user_certificate),
        concat!(
            "UserCertificate {",
            " author: User(DeviceID(\"alice@dev1\")),",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " user_id: UserID(\"bob\"),",
            " human_handle: Real(HumanHandle(\"Boby McBobFace <bob@example.com>\")),",
            " public_key: PublicKey(****),",
            " algorithm: X25519XSalsa20Poly1305,",
            " profile: Standard",
            " }",
        )
    );

    let device_certificate = DeviceCertificate {
        author: CertificateSignerOwned::User(alice.device_id.clone()),
        timestamp,
        device_id: bob.device_id.clone(),
        device_label: MaybeRedacted::Real(bob.device_label.clone()),
        verify_key: bob.verify_key(),
        algorithm: SigningKeyAlgorithm::Ed25519,
    };
    p_assert_eq!(
        format!("{:?}", device_certificate),
        concat!(
            "DeviceCertificate {",
            " author: User(DeviceID(\"alice@dev1\")),",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " device_id: DeviceID(\"bob@dev1\"),",
            " device_label: Real(DeviceLabel(\"My dev1 machine\")),",
            " verify_key: VerifyKey(****),",
            " algorithm: Ed25519",
            " }",
        )
    );

    let revoked_user_certificate = RevokedUserCertificate {
        author: alice.device_id.clone(),
        timestamp,
        user_id: bob.user_id().to_owned(),
    };
    p_assert_eq!(
        format!("{:?}", revoked_user_certificate),
        concat!(
            "RevokedUserCertificate {",
            " author: DeviceID(\"alice@dev1\"),",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " user_id: UserID(\"bob\")",
            " }",
        )
    );

    let user_update_certificate = UserUpdateCertificate {
        author: alice.device_id.clone(),
        timestamp,
        user_id: bob.user_id().to_owned(),
        new_profile: UserProfile::Outsider,
    };
    p_assert_eq!(
        format!("{:?}", user_update_certificate),
        concat!(
            "UserUpdateCertificate {",
            " author: DeviceID(\"alice@dev1\"),",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " user_id: UserID(\"bob\"),",
            " new_profile: Outsider",
            " }",
        )
    );

    let realm_role_certificate = RealmRoleCertificate {
        author: alice.device_id.clone(),
        timestamp,
        user_id: bob.user_id().to_owned(),
        realm_id: VlobID::from_hex("604784450642426b91eb89242f54fa52").unwrap(),
        role: Some(RealmRole::Owner),
    };
    p_assert_eq!(
        format!("{:?}", realm_role_certificate),
        concat!(
            "RealmRoleCertificate {",
            " author: DeviceID(\"alice@dev1\"),",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " realm_id: VlobID(60478445-0642-426b-91eb-89242f54fa52),",
            " user_id: UserID(\"bob\"),",
            " role: Some(Owner)",
            " }",
        )
    );

    let realm_name = RealmNameCertificate {
        author: alice.device_id.clone(),
        timestamp,
        realm_id: VlobID::from_hex("604784450642426b91eb89242f54fa52").unwrap(),
        key_index: 42,
        encrypted_name: b"012345".to_vec(),
    };
    p_assert_eq!(
        format!("{:?}", realm_name),
        concat!(
            "RealmNameCertificate {",
            " author: DeviceID(\"alice@dev1\"),",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " realm_id: VlobID(60478445-0642-426b-91eb-89242f54fa52),",
            " key_index: 42,",
            " encrypted_name: [48, 49, 50, 51, 52, 53]",
            " }",
        )
    );

    let realm_key_rotation = RealmKeyRotationCertificate {
        author: alice.device_id.clone(),
        timestamp,
        realm_id: VlobID::from_hex("604784450642426b91eb89242f54fa52").unwrap(),
        encryption_algorithm: SecretKeyAlgorithm::Xsalsa20Poly1305,
        hash_algorithm: HashAlgorithm::Sha256,
        key_index: 42,
        key_canary: b"012345".to_vec(),
    };
    p_assert_eq!(
        format!("{:?}", realm_key_rotation),
        concat!(
            "RealmKeyRotationCertificate {",
            " author: DeviceID(\"alice@dev1\"),",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " realm_id: VlobID(60478445-0642-426b-91eb-89242f54fa52),",
            " key_index: 42,",
            " encryption_algorithm: Xsalsa20Poly1305,",
            " hash_algorithm: Sha256,",
            " key_canary: [48, 49, 50, 51, 52, 53]",
            " }",
        )
    );

    let realm_archiving_certificate = RealmArchivingCertificate {
        author: alice.device_id.clone(),
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
            " author: DeviceID(\"alice@dev1\"),",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " realm_id: VlobID(60478445-0642-426b-91eb-89242f54fa52),",
            " configuration: DeletionPlanned { deletion_date: DateTime(\"2020-01-01T00:00:00Z\") }",
            " }",
        )
    );

    let shamir_recovery_brief_certificate = ShamirRecoveryBriefCertificate {
        author: alice.device_id.clone(),
        timestamp: "2020-01-01T00:00:00Z".parse().unwrap(),
        threshold: 3.try_into().unwrap(),
        per_recipient_shares: HashMap::from([
            ((bob.user_id().to_owned()), 2.try_into().unwrap()),
            ("carl".parse().unwrap(), 1.try_into().unwrap()),
            ("diana".parse().unwrap(), 1.try_into().unwrap()),
        ]),
    };
    assert!(
        format!("{:?}", shamir_recovery_brief_certificate).starts_with(
            // Ignore `per_recipient_shares` as, as a HashMap, it output is not stable across runs
            concat!(
                "ShamirRecoveryBriefCertificate {",
                " author: DeviceID(\"alice@dev1\"),",
                " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
                " threshold: 3,",
                " per_recipient_shares: "
            )
        )
    );

    let shamir_recovery_share_certificate = ShamirRecoveryShareCertificate {
        author: alice.device_id.clone(),
        timestamp: "2020-01-01T00:00:00Z".parse().unwrap(),
        recipient: bob.user_id().to_owned(),
        ciphered_share: b"abcd".to_vec(),
    };
    p_assert_eq!(
        format!("{:?}", shamir_recovery_share_certificate),
        concat!(
            "ShamirRecoveryShareCertificate {",
            " author: DeviceID(\"alice@dev1\"),",
            " timestamp: DateTime(\"2020-01-01T00:00:00Z\"),",
            " recipient: UserID(\"bob\"),",
            " ciphered_share: [97, 98, 99, 100]",
            " }",
        )
    );

    let sequester_authority_certificate = SequesterAuthorityCertificate {
        timestamp,
        verify_key_der: SequesterVerifyKeyDer::try_from(
            &hex!(
                "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689"
                "b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a"
                "3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62c"
                "b38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195"
                "f3bcc72ab57207ebfd0203010001"
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
                "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689"
                "b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a"
                "3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62c"
                "b38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195"
                "f3bcc72ab57207ebfd0203010001"
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
    // Generated from Parsec v3.0.0-b.6+dev
    // Content:
    //   type: "user_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   user_id: "bob"
    //   public_key: <bob.public_key as bytes>
    //   algorithm: X25519_XSALSA20_POLY1305
    //   profile: "STANDARD"
    let data = Bytes::from_static(&hex!(
        "e0b24615251070b6ec25811ae05479d2674368eb299896c8425bbe3a3f5d3818910334"
        "29f7e39638f8636c7e920268b886279e37f3f2ddae7bdd0e9d3c718704789c01e1001e"
        "ff89a474797065b0757365725f6365727469666963617465a6617574686f72aa616c69"
        "63654064657631a974696d657374616d70d70141d86ad584cd5d53a7757365725f6964"
        "a3626f62ac68756d616e5f68616e646c6592af626f62406578616d706c652e636f6dae"
        "426f6279204d63426f6246616365aa7075626c69635f6b6579c4207c999e9980bef377"
        "07068b07d975591efc56335be9634ceef7c932a09c891e25a9616c676f726974686db8"
        "5832353531395f5853414c534132305f504f4c5931333035a869735f61646d696ec2a7"
        "70726f66696c65a85354414e444152447ae35f3c"
    ));

    let expected = UserCertificate {
        author: CertificateSignerOwned::User(alice.device_id.to_owned()),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        user_id: bob.user_id().to_owned(),
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
        &CertificateSignerOwned::User(alice.device_id.clone())
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
    // Generated from Parsec v3.0.0-b.6+dev
    // Content:
    //   type: "user_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   human_handle: None
    //   user_id: "bob"
    //   public_key: <bob.public_key as bytes>
    //   algorithm: X25519_XSALSA20_POLY1305
    //   profile: "STANDARD"
    let data = hex!(
        "b5bb4b44bfc97e104671044387932a0c60e0d7207c047925828c3063c8e718e4b47197"
        "373cc97bd3a3edb54d00434bd6b84b357e6f43b214c764239f84392104789c01c2003d"
        "ff89a474797065b0757365725f6365727469666963617465a6617574686f72aa616c69"
        "63654064657631a974696d657374616d70d70141d86ad584cd5d53a7757365725f6964"
        "a3626f62ac68756d616e5f68616e646c65c0aa7075626c69635f6b6579c4207c999e99"
        "80bef37707068b07d975591efc56335be9634ceef7c932a09c891e25a9616c676f7269"
        "74686db85832353531395f5853414c534132305f504f4c5931333035a869735f61646d"
        "696ec2a770726f66696c65a85354414e44415244040d5363"
    );
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = UserCertificate {
        author: CertificateSignerOwned::User(alice.device_id.to_owned()),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        user_id: bob.user_id().to_owned(),
        human_handle: MaybeRedacted::Redacted(HumanHandle::new_redacted(bob.user_id())),
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
        &CertificateSignerOwned::User(alice.device_id.clone())
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

// TODO: Parsec v2 legacy no longer has to be support, remove me asap !
#[ignore]
#[rstest]
fn serde_user_certificate_legacy_format(alice: &Device, bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "user_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   user_id: "bob"
    //   public_key: <bob.public_key as bytes>
    //   is_admin: true
    let data_is_admin_true = hex!(
        "f1328bb29b9c55ed016bcb89756fd8b7733996637e0a1b7e2175d2c56dd1b7e65f319cb8e7e0c1"
        "9b795d52185e86f87c938de5f564907d6ab0d960cff02a9e08789c6b5b52525990baa1b438b528"
        "3e39b5a824332d3339b1247559626949467ed1aac49ccce4548794d432c3952599b9a9c52589b9"
        "05d7191d6f645d6d391b1bbc1cac2d336571527ed2aa82d224a0e2f8ecd4ca230a3533e7cd6cd8"
        "f7b99c9dad9bfd6669a4dc9f30e3e897c93eefbe9f345a30a7534e754566717c624a6e66de6100"
        "a6953b01"
    );
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "user_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   user_id: "bob"
    //   public_key: <bob.public_key as bytes>
    //   is_admin: false
    let data_is_admin_false = hex!(
        "15dd73f6ddd5af3b46806437194083c80921d51029393b21cb3da620dc42ec7056625b5b5dae3d"
        "16f822a505d609581979c62d093827fd418cc0d0cd99039707789c6b5b52525990baa1b438b528"
        "3e39b5a824332d3339b1247559626949467ed1aac49ccce4548794d432c3952599b9a9c52589b9"
        "05d7191d6f645d6d391b1bbc1cac2d336571527ed2aa82d224a0e2f8ecd4ca230a3533e7cd6cd8"
        "f7b99c9dad9bfd6669a4dc9f30e3e897c93eefbe9f345a30a7534e754566717c624a6e66de2100"
        "a6943b00"
    );

    let expected_is_admin_true = UserCertificate {
        author: CertificateSignerOwned::User(alice.device_id.to_owned()),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        user_id: bob.user_id().to_owned(),
        human_handle: MaybeRedacted::Redacted(HumanHandle::new_redacted(bob.user_id())),
        public_key: bob.public_key(),
        algorithm: PrivateKeyAlgorithm::X25519XSalsa20Poly1305,
        profile: UserProfile::Admin,
    };
    let expected_is_admin_false = {
        let mut obj = expected_is_admin_true.clone();
        obj.profile = UserProfile::Standard;
        obj
    };

    let check = |data: &[u8], expected: &UserCertificate| {
        let certif = UserCertificate::verify_and_load(
            data,
            &alice.verify_key(),
            CertificateSignerRef::User(&alice.device_id),
            None,
            None,
        )
        .unwrap();
        p_assert_eq!(&certif, expected);

        let data = Bytes::from(data.as_ref().to_vec());

        let unsecure_certif = UserCertificate::unsecure_load(data.clone()).unwrap();
        p_assert_eq!(
            unsecure_certif.author(),
            &CertificateSignerOwned::User(alice.device_id.clone())
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
            *expected
        );
    };

    check(&data_is_admin_true, &expected_is_admin_true);
    check(&data_is_admin_false, &expected_is_admin_false);
}

#[rstest]
fn serde_device_certificate(alice: &Device, bob: &Device) {
    // Generated from Parsec v3.0.0-b.6+dev
    // Content:
    //   type: "device_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   device_id: "bob@dev1"
    //   device_label: "My dev1 machine"
    //   verify_key: <bob.verify_key>
    //   algorithm: ED25519
    let data = hex!(
        "4a3f0aa00d5c4b5b61ccd03c50579e23e6c3f5dae5d0ac783eca2d4d95dd775aea593b"
        "13d7a22da2591aced48aae8b283edd439afc49f84080a9ea38adde1300789c01ae0051"
        "ff87a474797065b26465766963655f6365727469666963617465a6617574686f72aa61"
        "6c6963654064657631a974696d657374616d70d70141d86ad584cd5d53a96465766963"
        "655f6964a8626f624064657631ac6465766963655f6c6162656caf4d79206465763120"
        "6d616368696e65aa7665726966795f6b6579c420840d872f4252da2d1c9f81a77db5f0"
        "a5b9b60a5cde1eeabf40388ef6bca64909a9616c676f726974686da745443235353139"
        "03864b9d"
    );
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = DeviceCertificate {
        author: CertificateSignerOwned::User(alice.device_id.to_owned()),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
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
        &CertificateSignerOwned::User(alice.device_id.clone())
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
    // Generated from Parsec v3.0.0-b.6+dev
    // Content:
    //   type: "device_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   device_id: "bob@dev1"
    //   device_label: None
    //   verify_key: <bob.verify_key>
    //   algorithm: ED25519
    let data = hex!(
        "40da6782dad5bf04d76980b2e1d69a3810ad19c5512d76dc16958fd442c350ee499a40"
        "b092ab300570d020dcc7b792cfe4f882688e98d80ba830f9e0d9d46603789c019f0060"
        "ff87a474797065b26465766963655f6365727469666963617465a6617574686f72aa61"
        "6c6963654064657631a974696d657374616d70d70141d86ad584cd5d53a96465766963"
        "655f6964a8626f624064657631ac6465766963655f6c6162656cc0aa7665726966795f"
        "6b6579c420840d872f4252da2d1c9f81a77db5f0a5b9b60a5cde1eeabf40388ef6bca6"
        "4909a9616c676f726974686da74544323535313919274663"
    );
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = DeviceCertificate {
        author: CertificateSignerOwned::User(alice.device_id.to_owned()),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        device_id: bob.device_id.to_owned(),
        device_label: MaybeRedacted::Redacted(DeviceLabel::new_redacted(
            bob.device_id.device_name(),
        )),
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
        &CertificateSignerOwned::User(alice.device_id.clone())
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

// TODO: Parsec v2 legacy no longer has to be support, remove me asap !
#[ignore]
#[rstest]
fn serde_device_certificate_legacy_format(alice: &Device, bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "device_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   device_id: "bob@dev1"
    //   verify_key: <bob.verify_key>
    let data = hex!(
        "7068b77b57567dff9c201f0897f3020076420019884651f7fe604b364f88496dee3674294b02a6"
        "45d65b0b055add67403235038230045c98df01b0d17080cb00789c6b5d52525990ba2925b52c33"
        "39353e39b5a824332d3339b1247559626949467ed1aac41ca0840350de706549666e6a7149626e"
        "c17546c71b59575bcec606af846acc4c5991949f0456b6aa2cb52833ad323e3bb5f288420b6fbb"
        "be53d02d5d99f98dcb6bb77e58ba731b57cc3db957fb1d2cfabeed59e6c90900151e3980"
    );
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = DeviceCertificate {
        author: CertificateSignerOwned::User(alice.device_id.to_owned()),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        device_id: bob.device_id.to_owned(),
        device_label: MaybeRedacted::Redacted(DeviceLabel::new_redacted(
            bob.device_id.device_name(),
        )),
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
        &CertificateSignerOwned::User(alice.device_id.clone())
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
}

#[rstest]
fn serde_revoked_user_certificate(alice: &Device, bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "revoked_user_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   user_id: "bob"
    let data = hex!(
        "d3bb83c6366f6f232d6ad0e61ec4e45cb818d219aec21c116756ac9b5240f4b50c8658b0429e8b"
        "e08dff45f8a9a9eb8ad7ca1c9c23fe23435d845fd6c9e69605789c6b5996585a92915fb42a3127"
        "3339d52125b5cc7049496541ea8ea2d4b2fcecd494f8d2e2d4a2f8e4d4a292ccb4cce4c492d495"
        "2599b9a9c52589b905d7191d6f645d6d391b1bbc1cac28336571527e1200bd0d243a"
    );
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = RevokedUserCertificate {
        author: alice.device_id.to_owned(),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        user_id: bob.user_id().to_owned(),
    };

    let unsecure_certif = RevokedUserCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), &alice.device_id);
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
        RevokedUserCertificate::verify_and_load(&data, &alice.verify_key(), &alice.device_id, None)
            .unwrap();
    p_assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = RevokedUserCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        &alice.device_id,
        None,
    )
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
            &alice.device_id,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_user_update_certificate(alice: &Device, bob: &Device) {
    // Generated from Rust implementation (Parsec v3.0.x)
    // Content:
    //   type: "user_update_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   user_id: "bob"
    //   new_profile: 'OUTSIDER'
    let data = hex!(
        "9fcf8d19225f29cb3252f792bdb8c183aadf7904e5d2f167b0ac39aeb4370c3b3193c291d14720"
        "bd302ade3ae338bd0f6654c9270696653c4d8b91cdf9fcdc02789c0165009aff85a474797065b7"
        "757365725f7570646174655f6365727469666963617465a6617574686f72aa616c696365406465"
        "7631a974696d657374616d70d70141d86ad584cd5d53a7757365725f6964a3626f62ab6e65775f"
        "70726f66696c65a84f55545349444552f6972c29"
    );
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = UserUpdateCertificate {
        author: alice.device_id.to_owned(),
        new_profile: UserProfile::Outsider,
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        user_id: bob.user_id().to_owned(),
    };

    let unsecure_certif = UserUpdateCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), &alice.device_id);
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
        UserUpdateCertificate::verify_and_load(&data, &alice.verify_key(), &alice.device_id, None)
            .unwrap();
    p_assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 =
        UserUpdateCertificate::verify_and_load(&data2, &alice.verify_key(), &alice.device_id, None)
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
            &alice.device_id,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_realm_role_certificate(alice: &Device, bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "realm_role_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   realm_id: ext(2, b"4486e7cf02d747bd9126679ba58e0474")
    //   user_id: "bob",
    //   role: "OWNER",
    let data = hex!(
        "842251fd775c0cb6cdf19b7d00195713361856192cdea53efdbc79b63d40b1437fad4e991c0d00"
        "a658fce3d32254ff613c49383fbbc0abd828ab211fc49d090b789c6b5b52525990baad28353127"
        "37be283f27353e39b5a824332d3339b1247505443833e506934bdbf3f34cd7ddf74e544b9fbdb4"
        "8fa5647969716a11506671527ed21290bea5fee17eae412b4b3273538b4b12730bae333adec8ba"
        "da7236367859626949467ed1aac49ccce4548794d432430020cc3454"
    );
    let data = Bytes::from(data.as_ref().to_vec());

    let certif = RealmRoleCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        &alice.device_id,
        None,
        None,
    )
    .unwrap();

    let expected = RealmRoleCertificate {
        author: alice.device_id.clone(),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        realm_id: VlobID::from_hex("4486e7cf02d747bd9126679ba58e0474").unwrap(),
        user_id: bob.user_id().to_owned(),
        role: Some(RealmRole::Owner),
    };
    p_assert_eq!(certif, expected);

    let unsecure_certif = RealmRoleCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), &alice.device_id,);
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
        &alice.device_id,
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
            &alice.device_id,
            None,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_realm_role_certificate_no_role(alice: &Device, bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "realm_role_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   realm_id: ext(2, b"4486e7cf02d747bd9126679ba58e0474")
    //   user_id: "bob",
    //   role: None,
    let data = hex!(
        "0241a345e65a7271487e7d0145660c7dce22b51ac8c4da05d617226b382b38efcf2d1fa5366b8e"
        "75abf8689e358de2564d1e02113537d2dcf5cdeb2db800a304789c6b5b5992999b5a5c92985b70"
        "9dd1f146d6d596b3b1c1cb124b4b32f28b5625e66426a73aa4a496192e29cacf493db0bcb438b5"
        "283e336571527ed29292ca82d46d45a98939b9f120c9f8e4d4a292ccb4cce4c492d41510e1cc94"
        "1b4c2e6dcfcf335d77df3b512d7df6d23e9612005ac632e4"
    );
    let certif = RealmRoleCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        &alice.device_id,
        None,
        None,
    )
    .unwrap();

    let expected = RealmRoleCertificate {
        author: alice.device_id.clone(),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        realm_id: VlobID::from_hex("4486e7cf02d747bd9126679ba58e0474").unwrap(),
        user_id: bob.user_id().to_owned(),
        role: None,
        // role: Some(RealmRole::Owner),
    };
    p_assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = RealmRoleCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        &alice.device_id,
        None,
        None,
    )
    .unwrap();
    p_assert_eq!(certif2, expected);
}

#[rstest]
fn serde_realm_archiving_certificate_available(alice: &Device) {
    // Generated from Rust implementation (Parsec v2.16.0-rc.4+dev)
    // Content:
    //   type: "realm_archiving_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1577836800.0)
    //   configuration: {type:"AVAILABLE"}
    //   realm_id: ext(2, hex!("4486e7cf02d747bd9126679ba58e0474"))
    //
    let data = hex!(
        "5ed2a9a35096161dd741299427e56d5bf56de9a54cfbb6b0e754de9f2cfc699cf25bb92686"
        "fe38f1e2ad5a14130852d51a1ee4b74aaaa6c90e914a0011a2e000789c0181007eff85a474"
        "797065bb7265616c6d5f617263686976696e675f6365727469666963617465a6617574686f"
        "72aa616c6963654064657631a974696d657374616d70d70141d782f840000000a87265616c"
        "6d5f6964d8024486e7cf02d747bd9126679ba58e0474ad636f6e66696775726174696f6e81"
        "a474797065a9415641494c41424c4539583722"
    );
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = RealmArchivingCertificate {
        author: alice.device_id.clone(),
        timestamp: "2020-01-01T00:00:00Z".parse().unwrap(),
        realm_id: VlobID::from_hex("4486e7cf02d747bd9126679ba58e0474").unwrap(),
        configuration: RealmArchivingConfiguration::Available,
    };

    let unsecure_certif = RealmArchivingCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), &alice.device_id);
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
        &alice.device_id,
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
        &alice.device_id,
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
            &alice.device_id,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_realm_archiving_certificate_archived(alice: &Device) {
    // Generated from Rust implementation (Parsec v2.16.0-rc.4+dev)
    // Content:
    //   type: "realm_archiving_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1577836800.0)
    //   configuration: {type:"ARCHIVED"}
    //   realm_id: ext(2, hex!("4486e7cf02d747bd9126679ba58e0474"))
    //
    let data = hex!(
        "ca7aaea973705fae5737d382667e6ae535963d470bb6a1e1e073999e5a2ad35d2bb68b1181"
        "f821e6e0f462062ce9c48bb7e8e3c76ff880ea6cf6afaf0ac13306789c0180007fff85a474"
        "797065bb7265616c6d5f617263686976696e675f6365727469666963617465a6617574686f"
        "72aa616c6963654064657631a974696d657374616d70d70141d782f840000000a87265616c"
        "6d5f6964d8024486e7cf02d747bd9126679ba58e0474ad636f6e66696775726174696f6e81"
        "a474797065a84152434849564544024936e6"
    );
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = RealmArchivingCertificate {
        author: alice.device_id.clone(),
        timestamp: "2020-01-01T00:00:00Z".parse().unwrap(),
        realm_id: VlobID::from_hex("4486e7cf02d747bd9126679ba58e0474").unwrap(),
        configuration: RealmArchivingConfiguration::Archived,
    };

    let unsecure_certif = RealmArchivingCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), &alice.device_id);
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
        &alice.device_id,
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
        &alice.device_id,
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
            &alice.device_id,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_realm_archiving_certificate_deletion_planned(alice: &Device) {
    // Generated from Rust implementation (Parsec v2.16.0-rc.4+dev)
    // Content:
    //   type: "realm_archiving_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1577836800.0)
    //   configuration: {type:"DELETION_PLANNED", deletion_date:ext(1, 1580428800.0)}
    //   realm_id: ext(2, hex!("4486e7cf02d747bd9126679ba58e0474"))
    //
    let data = hex!(
        "ff3f4ed5765c70230f55bfce6c051f00be174cf8bd36b57224ca048c67d063a3f9357315c9"
        "993d869b79f713325535c8dbe9a341198205af21dad8056489a200789c01a0005fff85a474"
        "797065bb7265616c6d5f617263686976696e675f6365727469666963617465a6617574686f"
        "72aa616c6963654064657631a974696d657374616d70d70141d782f840000000a87265616c"
        "6d5f6964d8024486e7cf02d747bd9126679ba58e0474ad636f6e66696775726174696f6e82"
        "a474797065b044454c4554494f4e5f504c414e4e4544ad64656c6574696f6e5f64617465d7"
        "0141d78cdb80000000ab704333"
    );
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = RealmArchivingCertificate {
        author: alice.device_id.clone(),
        timestamp: "2020-01-01T00:00:00Z".parse().unwrap(),
        realm_id: VlobID::from_hex("4486e7cf02d747bd9126679ba58e0474").unwrap(),
        configuration: RealmArchivingConfiguration::DeletionPlanned {
            deletion_date: "2020-01-31T00:00:00Z".parse().unwrap(),
        },
    };

    let unsecure_certif = RealmArchivingCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), &alice.device_id);
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
        &alice.device_id,
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
        &alice.device_id,
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
            &alice.device_id,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_realm_name_certificate(alice: &Device) {
    // Generated from Rust implementation (Parsec v3.0.x)
    // Content:
    //   type: "realm_name_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   realm_id: ext(2, hex!("4486e7cf02d747bd9126679ba58e0474"))
    //   key_index: 42
    //   encrypted_name: b"12345"
    let data = hex!(
        "39ff6b6a3b84921598adc04b9a45cfea198ceb2820740ceab3d6f10172905f530c086b5032"
        "dfbab0046927917d43a1b5027b9e287ef8e5525fdefcf02851e801789c017f0080ff86a474"
        "797065b67265616c6d5f6e616d655f6365727469666963617465a6617574686f72aa616c69"
        "63654064657631a974696d657374616d70d70141d86ad584cd5d53a87265616c6d5f6964d8"
        "024486e7cf02d747bd9126679ba58e0474a96b65795f696e6465782aae656e637279707465"
        "645f6e616d65c405313233343524f3372c"
    );
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = RealmNameCertificate {
        author: alice.device_id.to_owned(),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        realm_id: VlobID::from_hex("4486e7cf02d747bd9126679ba58e0474").unwrap(),
        key_index: 42,
        encrypted_name: b"12345".to_vec(),
    };

    let unsecure_certif = RealmNameCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), &alice.device_id);
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
        RealmNameCertificate::verify_and_load(&data, &alice.verify_key(), &alice.device_id, None)
            .unwrap();
    p_assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 =
        RealmNameCertificate::verify_and_load(&data2, &alice.verify_key(), &alice.device_id, None)
            .unwrap();
    p_assert_eq!(certif2, expected);

    // Test invalid data
    p_assert_matches!(
        RealmNameCertificate::unsecure_load(b"dummy".to_vec().into()),
        Err(DataError::Signature)
    );
    p_assert_matches!(
        RealmNameCertificate::verify_and_load(
            b"dummy",
            &alice.verify_key(),
            &alice.device_id,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_realm_key_rotation_certificate(alice: &Device) {
    // Generated from Rust implementation (Parsec v3.0.x)
    // Content:
    //   type: "realm_name_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   realm_id: ext(2, hex!("4486e7cf02d747bd9126679ba58e0474"))
    //   key_index: 42
    //   encryption_algorithm: "XSALSA20_POLY1305"
    //   hash_algorithm: "SHA256"
    //   encrypted_name: b"12345"
    let data = hex!(
        "876755790e8e5b0a21808df04ba92eab8c8a72ad0f37f4c9a670020ca69411f9f1de0bf058"
        "47a7c40c522ddd1be6652f1caac7ab0fa29498782b69fec7e3c10b789c45cdbf0ec1501480"
        "f1105ec560d252b326120609491706694e6e0fbdf45f6e0fd15dc424315b69b483446265b2"
        "8a49da37f01a5206fb97dfb7de53e0e14d2058b63ec540172e0171d7d1190ae223ce803084"
        "1999ae88c1e20c1b06cea588b88d3e81ed2539359dbc968fa176fc21dc48f3cdd5fb994f5a"
        "d76d69bc3b6c0a146532770c5c942fe8301178df055863577032ed735f533b9a2a57f45eb7"
        "3390aa15e564826ffe83506babb2528f3387810322b81725b95a533e97c65008"
    );
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = RealmKeyRotationCertificate {
        author: alice.device_id.to_owned(),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        realm_id: VlobID::from_hex("4486e7cf02d747bd9126679ba58e0474").unwrap(),
        key_index: 42,
        encryption_algorithm: SecretKeyAlgorithm::Xsalsa20Poly1305,
        hash_algorithm: HashAlgorithm::Sha256,
        key_canary: b"12345".to_vec(),
    };

    let unsecure_certif = RealmKeyRotationCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), &alice.device_id);
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
        &alice.device_id,
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
        &alice.device_id,
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
            &alice.device_id,
            None
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_shamir_recovery_share_certificate(alice: &Device, bob: &Device) {
    // Generated from Rust implementation (Parsec v2.16.1+dev)
    // Content:
    //   type: "shamir_recovery_share_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1577836800.0)
    //   ciphered_share: hex!("61626364")
    //   recipient: "bob"
    //
    let data = hex!(
        "59008168d41356fb52c874bc1524c9f06e1bc0f2624ab7f3e61d94a03094bfe63ada1a3cfa"
        "abbfd861acd84a67a1a3a816567b1aecfdffc9697ad0bb007d9806789c25cc410ac2301046"
        "e1085ec42378837a923099fc920163c2742c742b781245a80b4fe11d8a37e9b281eededb7c"
        "8f978d15ff439f288b7a0597013afaf60acf5093b33019de74b354f44317617411c37132c9"
        "e88d729d77a7f9be74ceb9a9015205577b8612bead131471e37e7b0a1c57ffa9303c"
    );
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = ShamirRecoveryShareCertificate {
        author: alice.device_id.clone(),
        timestamp: "2020-01-01T00:00:00Z".parse().unwrap(),
        recipient: bob.user_id().to_owned(),
        ciphered_share: b"abcd".to_vec(),
    };

    let unsecure_certif = ShamirRecoveryShareCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), &alice.device_id);
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
        &alice.device_id,
        Some(bob.user_id()),
    )
    .unwrap();
    p_assert_eq!(certif, expected);

    // Test bad recipient

    let err = ShamirRecoveryShareCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        &alice.device_id,
        Some(alice.user_id()),
    )
    .unwrap_err();
    p_assert_matches!(err, DataError::UnexpectedUserID { .. });

    // Test bad author

    let err = ShamirRecoveryShareCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        &bob.device_id,
        Some(bob.user_id()),
    )
    .unwrap_err();
    p_assert_matches!(err, DataError::UnexpectedAuthor { .. });

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = ShamirRecoveryShareCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        &alice.device_id,
        Some(bob.user_id()),
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
            &alice.device_id,
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
        "3ea699e81d7c26fe7ee8b128ab80dfb9c3f11886fa78f0aa24bcb24c4b1efad23a6d4f75fb"
        "1376dbcc5954bb31af23dd7150e2a7a649410abc482d6e86088607789c1dcd410ac2301046"
        "e1564fe211bc413d4998247fc940d284c958e856f12456a1aebd4bf1262e2dae1fbcef36eb"
        "54f039d44089c5085c1e2193b1c2e88d8328f7ec48f1a4b3862c2f8aecd0798cc74539a12a"
        "a5b2b6a7f5f2ed9aa65934086ac8d1efdf05ff1f17c6a06603b672bddb6c770fcf34503b3b"
        "92d8fe0028ca35f9"
    );
    let data = Bytes::from(data.as_ref().to_vec());

    let expected = ShamirRecoveryBriefCertificate {
        author: alice.device_id.clone(),
        timestamp: "2020-01-01T00:00:00Z".parse().unwrap(),
        threshold: 3.try_into().unwrap(),
        per_recipient_shares: HashMap::from([
            ("bob".parse().unwrap(), 2.try_into().unwrap()),
            ("carl".parse().unwrap(), 1.try_into().unwrap()),
            ("diana".parse().unwrap(), 1.try_into().unwrap()),
        ]),
    };

    let unsecure_certif = ShamirRecoveryBriefCertificate::unsecure_load(data.clone()).unwrap();
    p_assert_eq!(unsecure_certif.author(), &alice.device_id);
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
        &alice.device_id,
    )
    .unwrap();
    p_assert_eq!(certif, expected);

    // Test bad author

    let err = ShamirRecoveryBriefCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        &"dummy@dummy".parse().unwrap(),
    )
    .unwrap_err();
    p_assert_matches!(err, DataError::UnexpectedAuthor { .. });

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = ShamirRecoveryBriefCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        &alice.device_id,
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
            &alice.device_id,
        ),
        Err(DataError::Signature)
    );
}

#[rstest]
fn serde_sequester_authority_certificate(alice: &Device) {
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   type: "sequester_authority_certificate"
    //   author: None
    //   timestamp: ext(1, 946774800.0)
    //   verify_key_der: hex!(
    //     "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689b069f3"
    //     "f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a3045d75b584c"
    //     "1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62cb38582f8a30219047b"
    //     "087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195f3bcc72ab57207ebfd020301"
    //     "0001"
    //   )
    //
    let data = hex!(
        "5955d05b6175d010dcb653dd0b56ea4096d8f26f3991a3a4cb22489a8b80c12631ae1d041d"
        "94cc7f4ae4440cf476ee9c3e5b0539953bba7b46015b4b02765304789c01f5000aff84a474"
        "797065bf7365717565737465725f617574686f726974795f6365727469666963617465a974"
        "696d657374616d70d70141cc375188000000ae7665726966795f6b65795f646572c4a23081"
        "9f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689b069"
        "f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a3045"
        "d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62cb385"
        "82f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195f3bc"
        "c72ab57207ebfd0203010001a6617574686f72c0b8006a3d"
    );
    let data = Bytes::from(data.as_ref().to_vec());
    let certif =
        SequesterAuthorityCertificate::verify_and_load(&data, &alice.verify_key()).unwrap();

    let expected = SequesterAuthorityCertificate {
        timestamp: "2000-01-02T01:00:00Z".parse().unwrap(),
        verify_key_der: SequesterVerifyKeyDer::try_from(
            &hex!(
                "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689"
                "b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a"
                "3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62c"
                "b38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195"
                "f3bcc72ab57207ebfd0203010001"
            )[..],
        )
        .unwrap(),
    };
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
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   type: "sequester_service_certificate"
    //   timestamp: ext(1, 946774800.0)
    //   encryption_key_der: hex!(
    //     "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689b069f3"
    //     "f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a3045d75b584c"
    //     "1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62cb38582f8a30219047b"
    //     "087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195f3bcc72ab57207ebfd020301"
    //     "0001"
    //   )
    //   service_id: ext(2, hex!("b5eb565343c442b3a26be44573813ff0"))
    //   service_label: "foo"
    //
    let data = hex!(
        "789c6b5d559c5a5496999c1a9f99728369ebebb060e7234e9b17653f712d6eb4ffb0b22433"
        "37b5b82431b7e03aa3e319f3c00e060686b5300d398949a9398bd3f2f397945416a4ee2d4e"
        "2d2c05aa4d2d8a8729484e2d2ac94ccb4c4e2c49dd949a975c54595092999f179f9d5a199f"
        "925a74649141e37c035e364ead368fb6efbc8c8c8cac0ccc8dbd0c068d9d4c8d8d0c9bee30"
        "2c3ebcf558e786cccf5f783c7db5a21f0b6f64f6df2ffb8fefdd83cf39bfcf27b930a8e61c"
        "4bb8cae1de54dc65e07a3d3ac247d2790fcb7121db8c353cef55adfd5e5c9f7d61eec28949"
        "770e34a729656edfa6b3b9b5e9c7622649966a8e1ac16d8c2d1b3ced7864ba65057e5e7fb6"
        "e875d4ffb4efef240c0ca77ede735c6b6b11fbebbf4ccc8c0c8c00ac417d28"
    );

    let certif = SequesterServiceCertificate::load(&data).unwrap();

    let expected = SequesterServiceCertificate {
        timestamp: "2000-01-02T01:00:00Z".parse().unwrap(),
        service_id: SequesterServiceID::from_hex("b5eb565343c442b3a26be44573813ff0").unwrap(),
        service_label: "foo".into(),
        encryption_key_der: SequesterPublicKeyDer::try_from(
            &hex!(
                "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689"
                "b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a"
                "3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62c"
                "b38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195"
                "f3bcc72ab57207ebfd0203010001"
            )[..],
        )
        .unwrap(),
    };
    p_assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump();
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = SequesterServiceCertificate::load(&data2).unwrap();
    p_assert_eq!(certif2, expected);

    // Test invalid data
    p_assert_matches!(
        SequesterServiceCertificate::load(b"dummy"),
        Err(DataError::Compression)
    );
}

// TODO: check sequester service certificate signed with actual DER key
