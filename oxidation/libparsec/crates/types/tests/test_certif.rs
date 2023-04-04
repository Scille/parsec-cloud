// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use pretty_assertions::assert_eq;

use libparsec_crypto::{SequesterPublicKeyDer, SequesterVerifyKeyDer};
use libparsec_tests_types::{alice, bob, rstest, Device};
use libparsec_types::*;

// TODO: check serde output to ensure handling of Option<T> depending of
// default/missing policy

#[rstest::rstest]
fn serde_user_certificate(alice: &Device, bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "user_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   user_id: "bob"
    //   public_key: <bob.public_key as bytes>
    //   profile: "STANDARD"
    let data = hex!(
        "2610244ce2508516dd5eb7fd93d2962de2897028bb0b0a2bb5cd3fae6563c06f0a02171cbe51e9"
        "5678c5e9e18b90489bfd1df5b9df945f982605c9345b272006789ceb585992999b5a5c92985b70"
        "9dd1f146d6d596b3b1c16b324a7313f3e23312f352725227ad4fca4f7248ad00aac849d54bcecf"
        "5de7949f54a9e09b0ca4dc12935397945416a46e282d4e2d8a4f4e2d2ac94ccb4c4e2c495d5e50"
        "949f969993ba2238c4d1cfc531c865554169524e66727c766ae511859a99f36636ecfb5cceced6"
        "cd7eb33452ee4f9871f4cb649f77df4f1a2d98d329a7ba1c6c5c66ca62a0d5cb124b4b32f28b56"
        "250275a73aa4a49619aec82c8e4f4cc9cdcc3b040023ce53bc"
    );

    let expected = UserCertificate {
        author: CertificateSignerOwned::User(alice.device_id.to_owned()),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        user_id: bob.user_id().to_owned(),
        human_handle: bob.human_handle.to_owned(),
        public_key: bob.public_key(),
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
    assert_eq!(certif, expected);

    let unsecure_certif = UserCertificate::unsecure_load(&data).unwrap();
    assert_eq!(unsecure_certif, expected);

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
    assert_eq!(certif2, expected);
}

#[rstest::rstest]
fn serde_user_certificate_redacted(alice: &Device, bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "user_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   human_handle: None
    //   user_id: "bob"
    //   public_key: <bob.public_key as bytes>
    //   profile: "STANDARD"
    let data = hex!(
        "c432c916e7595e27490828514e362b2bea679024421935288a61d715e9ddaa9316d9474ad344ac"
        "224628f3c431f4ad50a01bdcaa745f254a668336d47b87d103789ceb5852525990baa1b438b528"
        "3e39b5a824332d3339b124754d46696e625e7c46625e4a4eea81952599b9a9c52589b905d7191d"
        "6f645d6d391b1bbc1cac25336571527ed28acce2f8c494dcccbc43ab0a4a93723293e3b3532b8f"
        "28d4cc9c37b361dfe77276b66ef69ba591727fc28ca35f26fbbcfb7ed268c19c4e39d56589a525"
        "19f945ab12813a521d5252cb0c971714e5a765e6a4ae080e71f473710c720100e58447e3"
    );

    let expected = UserCertificate {
        author: CertificateSignerOwned::User(alice.device_id.to_owned()),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        user_id: bob.user_id().to_owned(),
        human_handle: None,
        public_key: bob.public_key(),
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
    assert_eq!(certif, expected);

    let unsecure_certif = UserCertificate::unsecure_load(&data).unwrap();
    assert_eq!(unsecure_certif, expected);

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
    assert_eq!(certif2, expected);
}

#[rstest::rstest]
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
        human_handle: None,
        public_key: bob.public_key(),
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
        assert_eq!(&certif, expected);

        let unsecure_certif = UserCertificate::unsecure_load(data).unwrap();
        assert_eq!(&unsecure_certif, expected);
    };

    check(&data_is_admin_true, &expected_is_admin_true);
    check(&data_is_admin_false, &expected_is_admin_false);
}

#[rstest::rstest]
fn serde_device_certificate(alice: &Device, bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "device_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   device_id: "bob@dev1"
    //   device_label: "My dev1 machine"
    //   verify_key: <bob.verify_key>
    let data = hex!(
        "8f2e9e0170e147b04e8687de5dae9b5bfb61f1d626c50a9b464182935f9676a308ff7432133bba"
        "e80bc312a762351f65c60207f404afaf347b26dfb308f8c702789c6b5b55965a949956199f9d5a"
        "7944a185b75ddf29e896aeccfcc6e5b55b3f2cddb98d2be69edcabfd0e167ddff62cf3e45c9392"
        "5a96999c1a9f9398949ab3deb75201c83754c84d4ccec8cc4b5d96585a92915fb42a3107a8c401"
        "24b312aa3c336545527e12586849496541ea26a878726a5149665a66726249eaca92ccdcd4e292"
        "c4dc82eb8c8e37b2aeb69c8d0d06004dc544f6"
    );

    let expected = DeviceCertificate {
        author: CertificateSignerOwned::User(alice.device_id.to_owned()),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        device_id: bob.device_id.to_owned(),
        device_label: bob.device_label.to_owned(),
        verify_key: bob.verify_key(),
    };

    let certif = DeviceCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        CertificateSignerRef::User(&alice.device_id),
        None,
    )
    .unwrap();
    assert_eq!(certif, expected);

    let unsecure_certif = DeviceCertificate::unsecure_load(&data).unwrap();
    assert_eq!(unsecure_certif, expected);

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
    assert_eq!(certif2, expected);
}

#[rstest::rstest]
fn serde_device_certificate_redacted(alice: &Device, bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "device_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   device_id: "bob@dev1"
    //   device_label: None
    //   verify_key: <bob.verify_key>
    let data = hex!(
        "05b65b45823c5d02aec29de4862ae8a9528c8c39b1c9536e13612c66bfd3c12e8b2755e200cf7c"
        "263e8c9415e0b0c141688f6c9ca7b0d33fc2caf33115683d00789c6b5b52525990ba2925b52c33"
        "39353e39b5a824332d3339b124756549666e6a7149626ec17546c71b59575bcec606af812acb49"
        "4c4acd39b02cb1b42423bf6855620e50cc012865b8aa2cb52833ad323e3bb5f288420b6fbbbe53"
        "d02d5d99f98dcb6bb77e58ba731b57cc3db957fb1d2cfabeed59e6c9b9126a5866ca8aa4fc24b0"
        "7e00b3a93fbc"
    );

    let expected = DeviceCertificate {
        author: CertificateSignerOwned::User(alice.device_id.to_owned()),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        device_id: bob.device_id.to_owned(),
        device_label: None,
        verify_key: bob.verify_key(),
    };

    let certif = DeviceCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        CertificateSignerRef::User(&alice.device_id),
        None,
    )
    .unwrap();
    assert_eq!(certif, expected);

    let unsecure_certif = DeviceCertificate::unsecure_load(&data).unwrap();
    assert_eq!(unsecure_certif, expected);

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
    assert_eq!(certif2, expected);
}

#[rstest::rstest]
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

    let expected = DeviceCertificate {
        author: CertificateSignerOwned::User(alice.device_id.to_owned()),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        device_id: bob.device_id.to_owned(),
        device_label: None,
        verify_key: bob.verify_key(),
    };

    let certif = DeviceCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        CertificateSignerRef::User(&alice.device_id),
        None,
    )
    .unwrap();
    assert_eq!(certif, expected);

    let unsecure_certif = DeviceCertificate::unsecure_load(&data).unwrap();
    assert_eq!(unsecure_certif, expected);
}

#[rstest::rstest]
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

    let expected = RevokedUserCertificate {
        author: alice.device_id.to_owned(),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        user_id: bob.user_id().to_owned(),
    };

    let unsecure_certif = RevokedUserCertificate::unsecure_load(&data).unwrap();
    assert_eq!(unsecure_certif, expected);

    let certif =
        RevokedUserCertificate::verify_and_load(&data, &alice.verify_key(), &alice.device_id, None)
            .unwrap();
    assert_eq!(certif, expected);

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
    assert_eq!(certif2, expected);
}

#[rstest::rstest]
fn serde_realm_role_certificate(alice: &Device, bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "realm_role_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   realm_id: "4486e7cf02d747bd9126679ba58e0474",
    //   user_id: "bob",
    //   role: OWNER,
    let data = hex!(
        "842251fd775c0cb6cdf19b7d00195713361856192cdea53efdbc79b63d40b1437fad4e991c0d00"
        "a658fce3d32254ff613c49383fbbc0abd828ab211fc49d090b789c6b5b52525990baad28353127"
        "37be283f27353e39b5a824332d3339b1247505443833e506934bdbf3f34cd7ddf74e544b9fbdb4"
        "8fa5647969716a11506671527ed21290bea5fee17eae412b4b3273538b4b12730bae333adec8ba"
        "da7236367859626949467ed1aac49ccce4548794d432430020cc3454"
    );
    let certif = RealmRoleCertificate::verify_and_load(
        &data,
        &alice.verify_key(),
        CertificateSignerRef::User(&alice.device_id),
        None,
        None,
    )
    .unwrap();

    let expected = RealmRoleCertificate {
        author: CertificateSignerOwned::User(alice.device_id.to_owned()),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        realm_id: RealmID::from_hex("4486e7cf02d747bd9126679ba58e0474").unwrap(),
        user_id: bob.user_id().to_owned(),
        role: Some(RealmRole::Owner),
    };
    assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = RealmRoleCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        CertificateSignerRef::User(&alice.device_id),
        None,
        None,
    )
    .unwrap();
    assert_eq!(certif2, expected);
}

#[rstest::rstest]
fn serde_realm_role_certificate_no_role(alice: &Device, bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "realm_role_certificate"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   realm_id: "4486e7cf02d747bd9126679ba58e0474",
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
        CertificateSignerRef::User(&alice.device_id),
        None,
        None,
    )
    .unwrap();

    let expected = RealmRoleCertificate {
        author: CertificateSignerOwned::User(alice.device_id.to_owned()),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        realm_id: RealmID::from_hex("4486e7cf02d747bd9126679ba58e0474").unwrap(),
        user_id: bob.user_id().to_owned(),
        role: None,
        // role: Some(RealmRole::Owner),
    };
    assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = RealmRoleCertificate::verify_and_load(
        &data2,
        &alice.verify_key(),
        CertificateSignerRef::User(&alice.device_id),
        None,
        None,
    )
    .unwrap();
    assert_eq!(certif2, expected);
}

#[rstest::rstest]
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
        "f3bcc72ab57207ebfd0203010001")[..],
        )
        .unwrap(),
    };
    assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump_and_sign(&alice.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 =
        SequesterAuthorityCertificate::verify_and_load(&data2, &alice.verify_key()).unwrap();
    assert_eq!(certif2, expected);
}

#[rstest::rstest]
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
        "f3bcc72ab57207ebfd0203010001")[..],
        )
        .unwrap(),
    };
    assert_eq!(certif, expected);

    // Also test serialization round trip
    let data2 = expected.dump();
    // Note we cannot just compare with `data` due to signature and keys order
    let certif2 = SequesterServiceCertificate::load(&data2).unwrap();
    assert_eq!(certif2, expected);
}
