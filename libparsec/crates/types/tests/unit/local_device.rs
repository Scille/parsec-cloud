// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use crate::prelude::*;

#[rstest]
#[case::admin(
    // Generated from v3.0.0-b.6+dev
    // Content:
    //   server_url: http://alice_dev1.example.com:9999
    //   organization_id: "CoolOrg"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    //   device_id: "alice@dev1"
    //   device_label: "My dev1 machine"
    //   human_handle: ("alice@example.com", "Alicey McAliceFace")
    //   signing_key: <alice.signing_key as bytes>
    //   private_key: <alice.private_key as bytes>
    //   profile: "ADMIN"
    //   user_realm_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_realm_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!(
        "a4eeb71ec7b4dccf55fd715b0686d650d22990201076acfc67dc9a506a87ec9e652665"
        "148f745a5f27eb3c4d6f22e5bd23c3bb02eb64a11c1072f6003317bc7826dbcda57af7"
        "8f412b4cfaef049c7cc358038f928783db21c5f9ec21017b37232c290c2872db6efc2a"
        "ff88b530ce5e90d16dfe5ee9b2d6f537f919d306918d9d1224d4574c466b786cc2b932"
        "f88219a5273b2f016244f04c9bec93f23514295bc7ccbe64a79ef9b00ab6daaabb5e5f"
        "ad26fc53bca5adb90f865fe1d514f072095878d3b16f4579288626b4963adfb3ac5139"
        "765fb8154b95188201688cf0356f5e4f3a6f5b9f1f3419906f5cd1af8bb985dd40afc3"
        "3bcc5ef4a9c7eed5a03fa0133153a1be6bccd652b42e1eb937d79a4347fbdef3dd51bf"
        "55ca3ee7762f88708143c63d0bff2e0558a8b9948038b1126a6970a68a471ad427c71f"
        "d0ef0288f5dbbcd7e45d661fff667da685e98250aa76d312b26e7d1de6aa578333b7ab"
        "90c33edfdbf6e2a22a4680ef7a7058d14db29f47866eec7ecb338f0415e4a13b4c05da"
        "c6999b9477862ef83ee1aef9c20b8ca6c9fd135ff291cf730b53f23c19aafc4d58d342"
        "24b330a3786a13dee70c4e304fd1237d5d9a9999ed2c86fc9fb3eb84e92dd92591dacd"
        "d504c2a63a1acc527351fa1720f507c6d0535ac29d5747c8ce86e700a952c4fe463308"
        "719f8dfc923d999e789c645b7b43dfc0"
    ),
    UserProfile::Admin,
)]
#[case::standard(
    // Generated from v3.0.0-b.6+dev
    // Content:
    //   server_url: http://alice_dev1.example.com:9999
    //   organization_id: "CoolOrg"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    //   device_id: "alice@dev1"
    //   device_label: "My dev1 machine"
    //   human_handle: ("alice@example.com", "Alicey McAliceFace")
    //   signing_key: <alice.signing_key as bytes>
    //   private_key: <alice.private_key as bytes>
    //   profile: "STANDARD"
    //   user_realm_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_realm_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!(
        "e2cca1b6fc5cb43dabb33a2d77556feb873ac5396a602b4b8547ba6d338d3ecda5506c"
        "e54242d90920d80c7059ff1082ee9f672978320688329ec7a59bf6abf9eb58b7ac35ba"
        "e55b6c94549c7544117c8262da043dce90889d5426781a917364e4a27829e39e0270e5"
        "83dc28426ae8c5100dd3878a7ad7b83aba39b44d74e85f841b795bd21157a1b0035a88"
        "3e84955e36835a95f56898935b3969ed7153feed771184bb80af5bd6231361de5af61e"
        "2704e9b1118950c72808e757fd3188b87d6c31443f75c39ca066fbcfda391af749c7d3"
        "b989742b4e0770ec70545b4ef55ce07a3009a162fa3fba333b51139c9d094f2b8d6438"
        "d174b27c416e64ce9e39a0bbe30819f47c5e83cf96c1688f24b317573b9a5c379dce3c"
        "07531f636ef4fc4be8e1811d6d0365230c17d02649e70772117914df06f48b236ab035"
        "c6d898b413b17ada185387cb0fa40249a1a82f5172e68f4795a419b75a355cdc6ea7d6"
        "66b9dbd26a2757381a750657dd9e1ee3927d303d9cb64c9aec1ab6d5f17245712469e7"
        "42e057411716d8cd51cefb4bc8903577e2ea1d8dcfb641c9de86a84470f6a162a71c4d"
        "874fcd34af9b9fc9ff62ac9b109635cb5544645204ff32effc9b26fd0021e8cdf51947"
        "a2ca36d5fde110d3565e52e02d84d8590a1fb725a60ffaa80beccdedfd87e80cf9e096"
        "2e23763d9eaaefef2781e9288bb3e1944bec66"
    ),
    UserProfile::Standard,
)]
#[case::outsider(
    // Generated from v3.0.0-b.6+dev
    // Content:
    //   server_url: http://alice_dev1.example.com:9999
    //   organization_id: "CoolOrg"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    //   device_id: "alice@dev1"
    //   device_label: "My dev1 machine"
    //   human_handle: ("alice@example.com", "Alicey McAliceFace")
    //   signing_key: <alice.signing_key as bytes>
    //   private_key: <alice.private_key as bytes>
    //   profile: "OUTSIDER"
    //   user_realm_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_realm_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!(
        "21db4301d0d64f2ab7697fcfde71deeea8acb8b94187bfd985680d8ba6a794340db0cc"
        "9c9da49114b3f29fdab29b00d13afd4e59c1661dceaa928b7e6777397a8c0695a27c71"
        "1948f49a339495ea95796059518d8633a214868b01a8f95af6b7784d7a81a7dd01e42a"
        "1403374c943426b499a4efd2d9082715451dc99886089dfe125c47d24db4029ed544e2"
        "2cd7be5cd8170b4b165dadcaa28063c18942dc3ea6b002b4dbb8f1c2d4ed2371ae77f7"
        "1ea6c3e814fba3cc89e490ec117fc6d308643c5de52a6bf337b1070a2fb8b405ae8c20"
        "41a30d0c0ed16e4981e253bff6d02ce8428fcfbf206d6dcab3f05af1ef0ddefbf9ca5a"
        "e54d359efb754192eddd5631b22b128a081724bf80845e54c65143d46b64c5474ed345"
        "b578abcb51087aa895366dfe16726ae7f479594899d344a8e51dba754f08565d9799f3"
        "751b478bd43c9de56e48a2c18f503229537e604c06517aeadd4bb355eadcf6eea00e67"
        "e6c5bab563715488dff15006cdfc36ad5a66eed5120fad68f0ae0130d5f15f387528db"
        "ccd247b2a5505df5eec9290d256b5705dd62eedf9ae1cd1550ff2389988a7ddee8143f"
        "e5cf047a544571a7ff610f42c278602d5bcd8daa171a8c914fff1f2a1035a2d0355ab0"
        "d83786afa829e65b3e36c4c99852d6d20ca57ee74156f07c6a2ed0439a3b710b7a6afd"
        "1e20fbe53b7a6ce1ccb831d8d9ac8680fe7bc7"
    ),
    UserProfile::Outsider,
)]
#[ignore = "TODO: scheme has changed, must regenerate the dump"]
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
        human_handle: HumanHandle::new("alice@example.com", "Alicey McAliceFace").unwrap(),
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
