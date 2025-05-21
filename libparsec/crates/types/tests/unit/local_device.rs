// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use crate::prelude::*;

#[rstest]
#[case::admin(
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   server_url: http://alice_dev1.example.com:9999
    //   organization_id: "CoolOrg"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    //   user_id: ext(2, a11cec00100000000000000000000000)
    //   device_id: ext(2, de10a11cec0010000000000000000000)
    //   device_label: "My dev1 machine"
    //   human_handle: ("alice@example.com", "Alicey McAliceFace")
    //   signing_key: <alice.signing_key as bytes>
    //   private_key: <alice.private_key as bytes>
    //   initial_profile: "ADMIN"
    //   user_realm_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_realm_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!(
        "7183aed06c4aca4a681cac2d0005a333f1710b7e0f68b7cb3d283b7470bc78fff39519"
        "375f7edb40a3e8060b07382f141534cf087ae8e9c545e75acdf2dbadd2aaa8a56cd68d"
        "72f9e98f645c84a4cc7499ef1a1190c43a8673fc2521c3addc1ac7434962f92286798b"
        "3783c6de3eb3efde083514911c844b0f2843f51b452b9742737065fff0d11450e8757d"
        "6c04c0111690eaef205b30a3cf6cc072129d93d41fd4724f3f734fea3e28519550c4cc"
        "7aee4bdbcc69b92b2aef705043a8ff921b3b8b129270cc5bcd50e0a808535d796f5851"
        "9766a3899810c451a018b4aaf786e075b4247678a408a996181069f77c8eb0774f5da2"
        "194fc6b771f7792755a2f21f53d6f8a2bbf1ac1e47424b6f7b57006bb5735c27c9e64e"
        "9507d5cf167907fc5fa1ddbae9d99db22239a99e674936e4236d96d55596b746601055"
        "7ad0164a5ffa206934f5e813f9fb4f14a9a1715660a6b07e69253f8efda8bde2bb1d3d"
        "d54a291f3a31ffae3664884b26b8ef30882abfbd3b4d5ebcc959b83f894d6e70a8d083"
        "caf59ed69fc4264ec4a6c5cee2039acaf2200444ee923cf01d9b73aceb34ddea057072"
        "1ad654b8e555608d65c91b311c8ad2806c8178aab75ae00942a755d2a018f88c3bc22f"
        "ba8768e75d7cb55950114f5ff70161f29eac8c8490d492165a442d7ece078eeb632a71"
        "aff4a5acbb15ccd586a9fa57ce24a158f1bdeb6990e743bdf734303d5c484e9d8cadd4"
        "122e23a1cee9a87ebf268c6360068db44398e2aaf26382606bb7e1bee4321a91"
    ),
    UserProfile::Admin,
)]
#[case::standard(
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   server_url: http://alice_dev1.example.com:9999
    //   organization_id: "CoolOrg"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    //   user_id: ext(2, a11cec00100000000000000000000000)
    //   device_id: ext(2, de10a11cec0010000000000000000000)
    //   device_label: "My dev1 machine"
    //   human_handle: ("alice@example.com", "Alicey McAliceFace")
    //   signing_key: <alice.signing_key as bytes>
    //   private_key: <alice.private_key as bytes>
    //   initial_profile: "STANDARD"
    //   user_realm_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_realm_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!(
        "bfd29c0e21f862421c4171ca3e1c5b4bbe3e580fc7c33b341343efbd671a73aa3a8627"
        "666685478204f5a7a1f92f6ed96ee8f1473535581e01d1a7c570660df75c68115bd88c"
        "f91952d59268cd334e59ab25707082ed062a163f3681630c58fcebf655a3ab30eb5c76"
        "a320ab8fb8d1d91d64b38d13d48cb9812acb822fc0bea3015436c9928435cf98a94f0a"
        "6fcdbf187390f32ab483c079c6bb7de16578350dea09a3927f08c6da41cf902aa19d5d"
        "328c7ccc00eb24e99fa799f21f84f20090e248bb1dfd2939b2271304794d872a64fb73"
        "4ba0df9627834a5ed31a26790aa95f4e84b738dfce0952ec8c3a3e4ed21d0cf5480b4f"
        "c17750b4ce3a094f4401d04d86c6f6499bb66c99a86882a864ae755673049e0c034bb7"
        "c35fe7bc2f1f7d5c3baebc2ae1d7012bb93db6fd5fe6432cc566fa081cc9d5a4da33c5"
        "427c843c4f4883548c70d0c06d775105621eaa0ae993120f6032bf345eb3060e4f484d"
        "e5376460c56057bcfa554b652456af6e2f199d7345470963370af42d51a80fd0ff09de"
        "12ac98c14d702f8f48fe0b5bea102316e808c7cd066e2e458a50f53e52b12e48383f6d"
        "2239f3c7b0d5e2ceeb42e07abbbf0b4eb236904259235163f3bfa90e3ecc77dc7d5e80"
        "e00ed493073c4062f84e72d796c8df83c8132092eac6bcafa394fc52f182df32d50567"
        "dae8a9923a8c36bda8fe793b2672de8165b6446baf2bc2d4ce051b56a0cfe984f82e4e"
        "bb4509bc79de71abae79cfa37139de4c393b41674c2729ec169d0092b8d590fd336dbb"
    ),
    UserProfile::Standard,
)]
#[case::outsider(
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   server_url: http://alice_dev1.example.com:9999
    //   organization_id: "CoolOrg"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    //   user_id: ext(2, a11cec00100000000000000000000000)
    //   device_id: ext(2, de10a11cec0010000000000000000000)
    //   device_label: "My dev1 machine"
    //   human_handle: ("alice@example.com", "Alicey McAliceFace")
    //   signing_key: <alice.signing_key as bytes>
    //   private_key: <alice.private_key as bytes>
    //   initial_profile: "OUTSIDER"
    //   user_realm_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_realm_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!(
        "cb74876c6136006164a1fb8ba211a0447205edc26469c6834e1332cec1b43680046550"
        "1c86bbb734908c63c04d369b6d0e1d8eb8de78324cb13c53d219c94c0e6cedae660852"
        "1a94673f34fbb2f5ea10c1a9630d12c3557bc0cab0e71420e6afdc875be52def609fa5"
        "2cd18522b947b38638b9aacda8517235a5c1ed3730beb5482c91ebd5c73d75fb3e7d1a"
        "c1deae0f04e9c6597fcd2b299c6c1503795395ee275a12affad6c816f3e1b7d8375bd6"
        "bd36f9635237c1963008e9f31d656e83a650f3039582a8df873fa8c7b45116db89d664"
        "12fc64bb3ebb6411da83de5db0992df311154edc3f1497a9367d947f1a70f58f8230a8"
        "d8af9b8aa48d767d85478dbbf8ff950ecc7c2bf993fd8e0357f1a401d257ce7aaebbff"
        "6c622a58fc197d533e0682d1d43798a349b802d385f581455a742defe5171430b9253a"
        "8a7e8cbf1838953ea9885d02510bd800be3834e16b2b1f30bdc6fa4cffff5c911a2d52"
        "df15c0b461616fb755ef62b05d9c98c9ef2d3b059062230e7cfa9b82b28584393a656c"
        "be3700553866b93293e9dc2ce7dbc4eff760ecbeaf59c6a67501cf7e966029601d8a49"
        "a5a424cfaa440d802fc12f5eed0e4f6e2a76e80ce399a9bbf3d14d5ee97a45369473c4"
        "e631c154a907112f5d18fc151e832ace8684bbf5530a4e43a2a54b896e7022f6bc9e42"
        "25f795551d2c414e6560bde08e1c34176cd45a19c16c0ab8d3f665242eb45acda90965"
        "82fcb754a9268ca37e6719040a3a655bbd899651123118f8568ed00ef1a196f74048aa"
    ),
    UserProfile::Outsider,
)]
fn serde_local_device_data_x(#[case] data: &[u8], #[case] initial_profile: UserProfile) {
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let expected = LocalDevice {
        initial_profile,
        // cspell:disable-next-line
        organization_addr: "parsec3://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&p=xCC-KXZzLOyMqU7tzwqv1BPNFZNj4PrcnmhXLHeh4X2bvQ".parse().unwrap(),
        user_id: "alice".parse().unwrap(),
        device_id: "alice@dev1".parse().unwrap(),
        device_label: "My dev1 machine".parse().unwrap(),
        human_handle: HumanHandle::from_raw("alice@example.com", "Alicey McAliceFace").unwrap(),
        signing_key: SigningKey::from(hex!(
            "d544f66ece9c85d5b80275db9124b5f04bb038081622bed139c1e789c5217400"
        )),
        private_key: PrivateKey::from(hex!(
            "74e860967fd90d063ebd64fb1ba6824c4c010099dd37508b7f2875a5db2ef8c9"
        )),
        user_realm_id: VlobID::from_hex("a4031e8bcdd84df8ae12bd3d05e6e20f").unwrap(),
        user_realm_key: SecretKey::from(hex!(
            "26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a"
        )),
        local_symkey: SecretKey::from(hex!(
            "125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4"
        )),
        time_provider: TimeProvider::default(),
    };

    let manifest = LocalDevice::decrypt_and_load(data, &key).unwrap();

    p_assert_eq!(manifest, expected);

    // Also test serialization round trip
    let data2 = manifest.dump_and_encrypt(&key);

    // Note we cannot just compare with `data` due to signature and keys order
    let manifest2 = LocalDevice::decrypt_and_load(&data2, &key).unwrap();
    p_assert_eq!(manifest2, expected);
}
