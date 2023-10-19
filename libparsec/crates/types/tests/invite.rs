// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;
use libparsec_types::fixtures::{bob, Device};
use libparsec_types::prelude::*;

#[rstest]
fn serde_invite_user_data(bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_user_data"
    //   requested_device_label: "My dev1 machine"
    //   requested_human_handle: ("bob@example.com", "Boby McBobFace")
    //   public_key: <bob.public_key as bytes>
    //   verify_key: <bob.verify_key as bytes>
    let encrypted = &hex!(
        "2802b8a233ad8ec183bb7060b81432fab2f7786ce1a9f774798f3c793bed75fc9ac654056ba0"
        "c42786dd9f95b4eeaeb9ba51fe3e799d93f26f8de11f47bfec8067d844d9d3cc1a4665ad94c0189060"
        "0c1a29b2c058ede68f50c872a3762fb843a5516283d70565bb5f089eac347da2524af11775efa56189"
        "eba5bdbc3bffca52a8e948a3502419f0694afa481f7f1b8beb149b09182eb258d4ce95dedb6bed0c90"
        "0dacd1cd3fec67637721351cd449e7093477e05e3510fa01cefce2d31af8df499e54d9e15e136b4546"
        "57b3bf104fb7bc3e0d1bd170a1c9cba023bd19693e4ae3c115bfbf01d8ce6fbb414e5db5e752bf05c2"
        "0a"
    );

    let expected = InviteUserData {
        requested_device_label: "My dev1 machine".parse().unwrap(),
        requested_human_handle: "Boby McBobFace <bob@example.com>".parse().unwrap(),
        public_key: bob.public_key(),
        verify_key: bob.verify_key(),
    };

    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let data = InviteUserData::decrypt_and_load(encrypted, &key).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let encrypted2 = data.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to signature and keys order
    let data2 = InviteUserData::decrypt_and_load(&encrypted2, &key).unwrap();
    p_assert_eq!(data2, expected);
}

#[rstest]
fn serde_invite_user_confirmation(bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_user_confirmation"
    //   device_id: "bob@dev1"
    //   device_label: "My dev1 machine"
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   profil: "STANDARD"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    let encrypted = &hex!(
        "d49ccfc53ca9eeda99cf7218c7b2cd60996885f7b16059f444530a6da146be4cf7a95c638e86"
        "ac42ac446bb03da4dd77aba5b7df397d2584cd07741168094bb0f4927aa4fff893a3580ad289c60e3a"
        "ad326eb7534339fb000553b1f37b52b99d9636f78344ef2f7b5afd621a40f9bca39c4db8fbedfc0936"
        "28eace16c81714710273c772a0ee4c236a7056c688ad49d69d5a6661d98c6dbc1677d8c4a6dff65c01"
        "d5d1a91580e5a45bd638dcf23beffc690cc70382690e9ae76be763ef7c8ef077be06a6d2672aa24537"
        "a59545918ad5dd952394d0f9371271ef8b6d71af2dd087725be430f96b"
    );

    let expected = InviteUserConfirmation {
        device_id: bob.device_id.to_owned(),
        device_label: "My dev1 machine".parse().unwrap(),
        human_handle: "Boby McBobFace <bob@example.com>".parse().unwrap(),
        profile: UserProfile::Standard,
        root_verify_key: bob.root_verify_key().to_owned(),
    };

    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let data = InviteUserConfirmation::decrypt_and_load(encrypted, &key).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let encrypted2 = data.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to signature and keys order
    let data2 = InviteUserConfirmation::decrypt_and_load(&encrypted2, &key).unwrap();
    p_assert_eq!(data2, expected);
}

#[rstest]
fn serde_invite_device_data(bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_device_data"
    //   requested_device_label: "My dev1 machine"
    //   verify_key: <bob.verify_key as bytes>
    let encrypted = &hex!(
        "6ffae1baf9f4eec1ef7b29ec88dbe4e006672a30a48871d04a0f5d63965653a8d69a0fbf3fa7"
        "c6ed1351863080232acc245748e5c970c0fda93b192c6b81e1d8c5126e6960f1e0470deded2f42eaca"
        "16a793198818b85c03914ef3fd780785278feed85a7243e8274a0a98d2fbed573d69b6df31a8d06ac5"
        "7ca2529ad112cb0d214ab9c1886073b3d81a7e5da402fcea57696dbb25c860"
    );

    let expected = InviteDeviceData {
        requested_device_label: "My dev1 machine".parse().unwrap(),
        verify_key: bob.verify_key(),
    };

    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let data = InviteDeviceData::decrypt_and_load(encrypted, &key).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let encrypted2 = data.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to signature and keys order
    let data2 = InviteDeviceData::decrypt_and_load(&encrypted2, &key).unwrap();
    p_assert_eq!(data2, expected);
}

#[rstest]
fn serde_invite_device_confirmation(bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_device_confirmation"
    //   device_id: "bob@dev1"
    //   device_label: "My dev1 machine"
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   profil: "STANDARD"
    //   private_key: <bob.private_key as bytes>
    //   user_manifest_id: ext(2, hex!("71568d41afcb4e2380b3d164ace4fb85"))
    //   user_manifest_key: <bob.user_manifest_key as bytes>
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    let encrypted = &hex!(
        "b703ea4cfb8aed8c7ba7a434316f0acd43389159bbf66c3a2bedd34d943ed97ed22302c76d2b"
        "91790f19ca7a87c4dd5526ea493cf388ff5a492c7343409175f17542f29ccf251bf8bb4b503a78c361"
        "4535225ffcf974069bd036576f31f2ec1cb498e8716f08a56339eb1c784183bf34874963e0f1656d6a"
        "f6859ffcb58e57ab3e73ce6285de981d3b636f7b4dd37070f3f68947ff463d1c9668f30f4e30adc898"
        "741c57871d97975be07da4639abad4249e461fed36635bd9240b1bbca9317d239164426eed3d03f861"
        "20ecfef81d5c03e318988311db64628e01560546bec8a48633b6024cd417d49dfff774de638d1a14fb"
        "133d54a69d6c21238417dea12b4f8f8db2fc8d99b20e62c49a6b7efc22b9e4cd2d00dacd66e0cf57c1"
        "2ca375a2e2853794f294af7e7858832b66ec128713ee3c2310fd25d3b1e2c14622d103c9ce636025b4"
        "a61ed7ee1d5ac9817b14955b99f5b9b93c65d300"
    );

    let expected = InviteDeviceConfirmation {
        device_id: bob.device_id.to_owned(),
        device_label: "My dev1 machine".parse().unwrap(),
        human_handle: "Boby McBobFace <bob@example.com>".parse().unwrap(),
        profile: UserProfile::Standard,
        private_key: bob.private_key.to_owned(),
        user_realm_id: bob.user_realm_id.to_owned(),
        user_realm_key: bob.user_realm_key.to_owned(),
        root_verify_key: bob.root_verify_key().to_owned(),
    };

    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let data = InviteDeviceConfirmation::decrypt_and_load(encrypted, &key).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let encrypted2 = data.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to signature and keys order
    let data2 = InviteDeviceConfirmation::decrypt_and_load(&encrypted2, &key).unwrap();
    p_assert_eq!(data2, expected);
}
