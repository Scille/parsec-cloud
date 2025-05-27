// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use crate::fixtures::{bob, Device};
use crate::prelude::*;

#[rstest]
fn serde_realm_keys_bundle(bob: &Device) {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   type: 'realm_keys_bundle'
    //   author: ext(2, 0xde10808c001000000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   realm_id: ext(2, 0xac42ef607148434a94cc502fd5e61bad)
    //   keys: [
    //     0x8bc8137eb3a96329fdf66e6a83fe744bfb88fd6a5a877252dfb8aebf74a177aa,
    //     0xac11adfb0f439a70d982b4f2f31b038fc0cc3d19a1e551f9acb4da5647a071d1,
    //   ]
    let raw: &[u8] = hex!(
        "155cf5b62ecd244a1999a5ffafc7e82bc364928331c14e8fd780c5bc64d443dd9ad3e8"
        "c3b3a04ee0186469b709ca652b294787603dd72bde184a2301cea40a040028b52ffd00"
        "582d0500b40985a474797065b17265616c6d5f6b6579735f62756e646c65a661757468"
        "6f72d802de10808c001000a974696d657374616d70d7010005d250a2269a75a86964d8"
        "02ac42ef607148434a94cc502fd5e61bada46b65797392c4208bc8137eb3a96329fdf6"
        "6e6a83fe744bfb88fd6a5a877252dfb8aebf74a177aac420ac11adfb0f439a70d982b4"
        "f2f31b038fc0cc3d19a1e551f9acb4da5647a071d102008530180a5006"
    )
    .as_ref();

    let k1 = KeyDerivation::from(hex!(
        "8bc8137eb3a96329fdf66e6a83fe744bfb88fd6a5a877252dfb8aebf74a177aa"
    ));
    let k2 = KeyDerivation::from(hex!(
        "ac11adfb0f439a70d982b4f2f31b038fc0cc3d19a1e551f9acb4da5647a071d1"
    ));
    let expected = RealmKeysBundle {
        author: bob.device_id,
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        realm_id: VlobID::from_hex("ac42ef607148434a94cc502fd5e61bad").unwrap(),
        keys: vec![k1.clone(), k2.clone()],
    };
    println!(
        "***expected: {:?}",
        expected.dump_and_sign(&bob.signing_key)
    );

    p_assert_eq!(expected.key_index(), 2);
    p_assert_eq!(expected.last_key(), &k2);
    p_assert_eq!(expected.keys(), [k1, k2]);

    let unsecure = RealmKeysBundle::unsecure_load(Bytes::from_static(raw)).unwrap();
    let (data, _) = unsecure.verify_signature(&bob.verify_key()).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump_and_sign(&bob.signing_key);
    // Note we cannot just compare with `data` due to signature and keys order
    let unsecure2 = RealmKeysBundle::unsecure_load(raw2.into()).unwrap();
    let (data2, _) = unsecure2.verify_signature(&bob.verify_key()).unwrap();
    p_assert_eq!(data2, expected);
}

#[rstest]
fn serde_realm_keys_bundle_access() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   type: 'realm_keys_bundle_access'
    //   keys_bundle_key: 0xac11adfb0f439a70d982b4f2f31b038fc0cc3d19a1e551f9acb4da5647a071d1
    let raw: &[u8] = hex!(
        "0028b52ffd00586d0200540482a474797065b87265616c6d5f6b6579735f62756e646c"
        "655f616363657373af6b6579c420ac11adfb0f439a70d982b4f2f31b038fc0cc3d19a1"
        "e551f9acb4da5647a071d10100306e3e01"
    )
    .as_ref();

    let expected = RealmKeysBundleAccess {
        keys_bundle_key: SecretKey::from(hex!(
            "ac11adfb0f439a70d982b4f2f31b038fc0cc3d19a1e551f9acb4da5647a071d1"
        )),
    };
    println!("***expected: {:?}", expected.dump());

    let data = RealmKeysBundleAccess::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump();
    let data2 = RealmKeysBundleAccess::load(&raw2).unwrap();
    p_assert_eq!(data2, expected);
}
