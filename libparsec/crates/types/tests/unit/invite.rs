// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use crate::fixtures::{bob, Device};
use crate::prelude::*;

#[rstest]
fn serde_invite_user_data(bob: &Device) {
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: "invite_user_data"
    //   requested_device_label: "My dev1 machine"
    //   requested_human_handle: ("bob@example.com", "Boby McBobFace")
    //   public_key: <bob.public_key as bytes>
    //   verify_key: <bob.verify_key as bytes>
    let encrypted = &hex!(
        "e4f5ba3abc4d45355a5977e0bfd07bfdeff13ffe2a82f7079f214250a479d9c0084fb1"
        "600b3de09b8064419fd14fd36b4ed3acf1778804695d21030b78c903f033e0d8193e93"
        "88368b386a3939c442c2eb081eb72682c441d3a92a62ceeb99c6d7f7c182bc4e8b4994"
        "806a49a41ffd8e20b30205517ee33012b9360f52dc20b6f725963ba2cda55a611e1add"
        "8bf76b4d7b2906a62e0b89871a37f3a267bc0ee21460334b212e1c39ad7b3a3054e15c"
        "d12b1b2c1bc4a43bd693711f37acc5b2f774b478c8d2a2457b3f24ff3ca84564a6b223"
        "625c5507cd2710eace856e8c4d9084b294c3cc1147cec211036d4f8c570fd2958f378a"
        "44b318"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: "invite_user_confirmation"
    //   user_id: ext(2, 808c0010000000000000000000000000)
    //   device_id: ext(2, de10808c001000000000000000000000)
    //   device_label: "My dev1 machine"
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   profil: "STANDARD"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    let encrypted = &hex!(
        "da506f3f3431ec1f18849c94f4e4e76ce792c147d31a195ed831036ce1ab803930ce29"
        "ae62ecae189601befa743ec9b73a0a76118bdb5e26a01360bd337ab3cb3b3298932250"
        "7b6c2c3528b7a26c6eb9ad11fd5057fc0a72b2c93f7b412d76bbce5d992a7cf0c8025f"
        "b32f2d12c92872fb44e3306fc16f57400f6b303cce8aa7063fa064eb50dfdd5ae3898e"
        "5e1d2804ebba18a1c6558eb2a92e3ca2088659001963b5aa844fa5de61ea7a06e71598"
        "8c993ed10a42ab2aa6d5e40455d4aeb6365473c486f1ebb6318e9d8ef047e0407a7736"
        "d6d3607196afebec66c920c99a3dd17656085d787d6b0f7303ec374cf3275745bfe799"
        "77160c6001054dd6af082a62a358f72252fd57f5ae83"
    );

    let expected = InviteUserConfirmation {
        user_id: bob.user_id,
        device_id: bob.device_id,
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: "invite_device_data"
    //   requested_device_label: "My dev1 machine"
    //   verify_key: <bob.verify_key as bytes>
    let encrypted = &hex!(
        "04040f06a106396d6d53e03abb5cfb77088cda9683bb54e434ca2fad0187c0a78d00ac"
        "99727b980ba0eba6586a4db4b6575ee37b37178ff9f2b8623d045cf8fa8981a8cf7c76"
        "0fccf4c324e6da00142bc382c6365fc4255bf57026e83d463b7241d89175edc90d8726"
        "7a6839b5c867a7b630f30bb13e0f0a01b02c2c0ff107cf98212f0cc87ab903cef8380f"
        "ad96a704e1907946ed48"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: "invite_device_confirmation"
    //   user_id: ext(2, 808c0010000000000000000000000000)
    //   device_id: ext(2, de10808c001000000000000000000000)
    //   device_label: "My dev1 machine"
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   profil: "STANDARD"
    //   private_key: <bob.private_key as bytes>
    //   user_manifest_id: ext(2, hex!("71568d41afcb4e2380b3d164ace4fb85"))
    //   user_manifest_key: <bob.user_manifest_key as bytes>
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    let encrypted = &hex!(
        "06733269843fadca366b5f1a4328eeebfbd9ee0c33b11435ddbba1721c317ebcded96b"
        "e030e5cc768faebd91b09c4370f4153f7bc134b803253b5911377f5558d81c79c173d2"
        "859f1d52b5baa7ceafd03e8259c45eca8522f05b35c7c5c297ee7303b12b19bd1c9a87"
        "d76d103098baa2b423c314749416a95b4381f40d9899666f10f4866f618ec91a711ff4"
        "55738e009854013f4b2772e4d9c69eac81a6c0df27bf0c3e0c98965e29ee4272d23471"
        "a1537485fe6c03dc29e8dbe7c81903cf54aae45bc9994d1028f4de9e8f57abdedecd16"
        "30aec2414f714944f864484d87d3fd93c8ae2af9dab9695c60936043452e679d92c55e"
        "c2b39a5c720ae04579b419d3be360936c535824c7d7f109ba4c72340fda07435b28891"
        "412a9b2a1164dac3b607fb236c715896ea029ed64aa1c77afab39c2447bc2eba85415e"
        "26860e463e0b6fde8a77e709d5991db925f6f6c2051388228f5c3a17797fb9000c7502"
        "80026b4f239c2a4ec112eaf29aaedbf6f9fba0be3af96aa119a4ebbb031d81d880069d"
        "2210ca4999db6ecf2f5ecc"
    );

    let expected = InviteDeviceConfirmation {
        user_id: bob.user_id,
        device_id: bob.device_id,
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
