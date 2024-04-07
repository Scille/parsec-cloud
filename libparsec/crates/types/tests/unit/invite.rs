// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use crate::prelude::*;

#[rstest]
fn serde_invite_0_wait_peer() {
    // Generated from Parsec 3.0.0-b.6+dev
    // Content:
    //   type: "invite_0_wait_peer"
    //   public_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    let encrypted = &hex!(
        "bc265bf0cb189d316b3c587b2020578087628277d2fbc897127e6df27c475d389776159bd8"
        "33281f98f8d230a4d82823b4248c1d773ff9f60a0f64fb580d2232cffcb4dcff79a2a4a85d"
        "57ef2d31dcfd8f133fc1e368904564168ce8847e0d60b5ad8c64abc2a04348a9d4f2bb4c2d"
        "578eee41a08ab44b12ad"
    );

    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let expected = Invite0WaitPeer {
        ty: Invite0WaitPeerDataType::default(),
        public_key: PublicKey::from(hex!(
            "be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd"
        )),
    };

    let data = Invite0WaitPeer::decrypt_and_load(encrypted, &key).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let encrypted2 = data.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to signature and keys order
    let data2 = Invite0WaitPeer::decrypt_and_load(&encrypted2, &key).unwrap();
    p_assert_eq!(data2, expected);
}

// TODO: add tests for the invite_{0,7}_* schemas !
