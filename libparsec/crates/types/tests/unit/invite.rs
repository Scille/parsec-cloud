// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use crate::fixtures::{Device, bob};
use crate::prelude::*;

#[rstest]
fn serde_invite_user_data(bob: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'invite_user_data'
    //   requested_device_label: 'My dev1 machine'
    //   requested_human_handle: ['bob@example.com', 'Boby McBobFace']
    //   public_key: 0x7c999e9980bef37707068b07d975591efc56335be9634ceef7c932a09c891e25
    //   verify_key: 0x840d872f4252da2d1c9f81a77db5f0a5b9b60a5cde1eeabf40388ef6bca64909
    let encrypted = &hex!(
    "8a3e68b9894a1a1b2aaa1cbd60b78e8a6dbbb552ee103d4154afd04e9e519c8ff20430"
    "b11bd542dc9e67652d1e40b90dda660b56c6c2b602b3bfdcf23b810073af6ae58515df"
    "a395bdbed5cd1509402f23e8582fa60139b3f99acbebacb1d0b50091248fd066ed77a9"
    "c7145350550ed0dd897fb3cc68238239f0e531b61e18d746334d4b161b14f503ecc69a"
    "c2c36b327a5d74fc21ea37868e18c42429d84a5540eba2c3eb2739efb5a6fe2ab0c263"
    "d5ca907f8ffc202094e7aff3a86919553936f584b50290a33136133a9847d8ca51f482"
    "3be23632e678f980bc758287d1388e42b2916ee8aa5fa081d39235845bdae35a7db4dd"
    "8a91923ed640"
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
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'invite_user_confirmation'
    //   user_id: ext(2, 0x808c0010000000000000000000000000)
    //   device_id: ext(2, 0xde10808c001000000000000000000000)
    //   device_label: 'My dev1 machine'
    //   human_handle: ['bob@example.com', 'Boby McBobFace']
    //   profile: 'STANDARD'
    //   root_verify_key: 0xbe2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd
    let encrypted = &hex!(
    "edce3bf9e7829de37eb47bfe7f1e7214dd0853fb0c4e37c3f175dfc7e67439355328dc"
    "cf6998494e2410b87a70f1c9170f808cc22b37c636f3ec67e16d6545adae87dc393dd0"
    "041ed9d05a4013a0d391fad3c0061934ca3bc24ac3b912c2bc4187215a9bd24126c9aa"
    "021f77f1fef817ba007681f191f59916b7e04bbcbcd8bb74fa043784eab84716a74cf6"
    "a3a9e2b84390a81594caf1969f17f7e4a615317372f6d48faa762c7033407e3d38a5f7"
    "a988c62e8d00b8d9f78f350f1b5c0810dcdccfff0f843a76131013a58a0f4bc69a2f6f"
    "9352ab9a1563891fbbb8a8184d21c6a301f40bb7228fae27978eb75a197a0eb693e7ef"
    "745b63f4e955"
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
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'invite_device_data'
    //   requested_device_label: 'My dev1 machine'
    //   verify_key: 0x840d872f4252da2d1c9f81a77db5f0a5b9b60a5cde1eeabf40388ef6bca64909
    let encrypted = &hex!(
    "71df358a1c069d7c7532282cfc2fbb3ac9c3a303d5e51cd78665150ddaf11ff794c4ba"
    "b74edb34ff5fbfc7e18f7398a0a204b63411a1c4ec50824d49f5916621bd048ce98cb5"
    "f93c14e7ebdede85dc90fc9deae51c261cb971b462fd5564f416b9ed5da4147c5a0142"
    "7a2d94b9ca5bb02b9ffd54e5c1ae3c32e20a65ea70d1999bf4af3b3fb6a92ab979541e"
    "4ca45c3e070fbe2e40d4868a9c29bf9973cb2b"
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
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'invite_device_confirmation'
    //   user_id: ext(2, 0x808c0010000000000000000000000000)
    //   device_id: ext(2, 0xde10808c001000000000000000000000)
    //   device_label: 'My dev1 machine'
    //   human_handle: ['bob@example.com', 'Boby McBobFace']
    //   profile: 'STANDARD'
    //   private_key: 0x16767ec446f2611f971c36f19c2dc11614d853475ac395d6c1d70ba46d07dd49
    //   user_realm_id: ext(2, 0x71568d41afcb4e2380b3d164ace4fb85)
    //   user_realm_key: 0x65de53d2c6cd965aa53a1ba5cc7e54b331419e6103466121996fa99a97197a48
    //   root_verify_key: 0xbe2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd
    let encrypted = &hex!(
    "9ce694da6243270f4b33931ef3c82d7f7b17c8710463f4cbefcf98fad5277ffb09c7b0"
    "82a2e6cbfba3735c953053ea330a1bb5132d515b9a08e33f662d13b8942f778cae2d19"
    "5e43a20da5b7567f0f70d9f5e8b450b4f129e3380fe2068190f12c216d9f619c938ea4"
    "623fad23c7f94807c77748515b8c56473d117a4649c3b907d61e1ba2200ad532ceea03"
    "f37c5975b05c2c803ed597d28aa29877dd0d0ccc0f1a2bb6f73c22c611631143684023"
    "942e5ed8cc6c9193e282cfc4edd675b91c37bcd772e0270ee425647e304903b7e3455b"
    "177a90016a9ad3e01cab26c5351c455fb3b67894da89d5b68c43fe7a5ff36c83404b9d"
    "7e3d87e2b3af1ffeb7707695219fdf5c9a4bda33daca60658a705168c0aea797e64991"
    "9ed4e66b93e19c7ec28a5380c66278aea167cec45eeed22f376a338a6a84018b750eec"
    "4fd6ba983c0a5bd3b75e1cf9a118b7b92308e939b3ce00aa6310e60862b7d3bc2dc7df"
    "7f55abbb6c8cc501db70d3ca"
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
