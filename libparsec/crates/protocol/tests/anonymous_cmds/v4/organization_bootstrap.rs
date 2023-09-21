// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::anonymous_cmds;

// Request

pub fn req() {
    let raw_expected = [
        (
            // Generated from Rust implementation (Parsec v2.14.1+dev)
            // Content:
            //   bootstrap_token: "0db537dee3ff9a3c2f76e337a4461f41fb3d738f35eb48f3759046dfbedb2e79"
            //   cmd: "organization_bootstrap"
            //   device_certificate: hex!(
            //     "1e600d29590d2e005e19cdc7b2a75b54a549c15e9eb57213aa5bdd3696d0a7c68cb88f3209d1e682"
            //     "cfa5754229165924900a5e56ef571dc53fa2ba4ba51e0c04789c019c0063ff86a474797065b26465"
            //     "766963655f6365727469666963617465a6617574686f72aa616c6963654064657631a974696d6573"
            //     "74616d70d70141d782f840000000a96465766963655f6964a8626f624064657631ac646576696365"
            //     "5f6c6162656caf4d792064657631206d616368696e65aa7665726966795f6b6579c420840d872f42"
            //     "52da2d1c9f81a77db5f0a5b9b60a5cde1eeabf40388ef6bca649094f5b436f"
            //   )
            //   redacted_device_certificate: hex!(
            //     "2e44c56592b613a94ae151cc3b754945e300f457eb71d9f6af18f026f943bd0c1b3a029c1fbc504a"
            //     "a793f7937067864cb4fc0c7eeba7e1f9d83dfec5fb9b3d01789c018d0072ff86a474797065b26465"
            //     "766963655f6365727469666963617465a6617574686f72aa616c6963654064657631a974696d6573"
            //     "74616d70d70141d782f840000000a96465766963655f6964a8626f624064657631ac646576696365"
            //     "5f6c6162656cc0aa7665726966795f6b6579c420840d872f4252da2d1c9f81a77db5f0a5b9b60a5c"
            //     "de1eeabf40388ef6bca64909da083e35"
            //   )
            //   redacted_user_certificate: hex!(
            //     "e04ff4746d51000f68eb3dcfde36c53fe60f919c5eb8e76bbe2e4748b71553484cf5e4a73fde1650"
            //     "736538d5f70f83205307b216d3109ca1b9ca66f453089009789c019f0060ff88a474797065b07573"
            //     "65725f6365727469666963617465a6617574686f72aa616c6963654064657631a974696d65737461"
            //     "6d70d70141d782f840000000a7757365725f6964a3626f62ac68756d616e5f68616e646c65c0aa70"
            //     "75626c69635f6b6579c4207c999e9980bef37707068b07d975591efc56335be9634ceef7c932a09c"
            //     "891e25a869735f61646d696ec2a770726f66696c65a85354414e44415244fe7e465c"
            //   )
            //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
            //   sequester_authority_certificate: None
            //   user_certificate: hex!(
            //     "fee16e1a7aff6fbe84eb9f1b3008e37cc8ca7cb27b4fd49b8c787085d2451fd09e4e1cf471f23c91"
            //     "6a5a8524ff27dfa1ce3de3568949c83140ecd9250d2f0702789c01be0041ff88a474797065b07573"
            //     "65725f6365727469666963617465a6617574686f72aa616c6963654064657631a974696d65737461"
            //     "6d70d70141d782f840000000a7757365725f6964a3626f62ac68756d616e5f68616e646c6592af62"
            //     "6f62406578616d706c652e636f6dae426f6279204d63426f6246616365aa7075626c69635f6b6579"
            //     "c4207c999e9980bef37707068b07d975591efc56335be9634ceef7c932a09c891e25a869735f6164"
            //     "6d696ec2a770726f66696c65a85354414e44415244a7225235"
            //   )
            //
            &hex!(
                "88a3636d64b66f7267616e697a6174696f6e5f626f6f747374726170af626f6f7473747261"
                "705f746f6b656ed94030646235333764656533666639613363326637366533333761343436"
                "316634316662336437333866333565623438663337353930343664666265646232653739b2"
                "6465766963655f6365727469666963617465c4e71e600d29590d2e005e19cdc7b2a75b54a5"
                "49c15e9eb57213aa5bdd3696d0a7c68cb88f3209d1e682cfa5754229165924900a5e56ef57"
                "1dc53fa2ba4ba51e0c04789c019c0063ff86a474797065b26465766963655f636572746966"
                "6963617465a6617574686f72aa616c6963654064657631a974696d657374616d70d70141d7"
                "82f840000000a96465766963655f6964a8626f624064657631ac6465766963655f6c616265"
                "6caf4d792064657631206d616368696e65aa7665726966795f6b6579c420840d872f4252da"
                "2d1c9f81a77db5f0a5b9b60a5cde1eeabf40388ef6bca649094f5b436fbb72656461637465"
                "645f6465766963655f6365727469666963617465c4d82e44c56592b613a94ae151cc3b7549"
                "45e300f457eb71d9f6af18f026f943bd0c1b3a029c1fbc504aa793f7937067864cb4fc0c7e"
                "eba7e1f9d83dfec5fb9b3d01789c018d0072ff86a474797065b26465766963655f63657274"
                "69666963617465a6617574686f72aa616c6963654064657631a974696d657374616d70d701"
                "41d782f840000000a96465766963655f6964a8626f624064657631ac6465766963655f6c61"
                "62656cc0aa7665726966795f6b6579c420840d872f4252da2d1c9f81a77db5f0a5b9b60a5c"
                "de1eeabf40388ef6bca64909da083e35b972656461637465645f757365725f636572746966"
                "6963617465c4eae04ff4746d51000f68eb3dcfde36c53fe60f919c5eb8e76bbe2e4748b715"
                "53484cf5e4a73fde1650736538d5f70f83205307b216d3109ca1b9ca66f453089009789c01"
                "9f0060ff88a474797065b0757365725f6365727469666963617465a6617574686f72aa616c"
                "6963654064657631a974696d657374616d70d70141d782f840000000a7757365725f6964a3"
                "626f62ac68756d616e5f68616e646c65c0aa7075626c69635f6b6579c4207c999e9980bef3"
                "7707068b07d975591efc56335be9634ceef7c932a09c891e25a869735f61646d696ec2a770"
                "726f66696c65a85354414e44415244fe7e465caf726f6f745f7665726966795f6b6579c420"
                "be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbdbf73657175"
                "65737465725f617574686f726974795f6365727469666963617465c0b0757365725f636572"
                "7469666963617465c50109fee16e1a7aff6fbe84eb9f1b3008e37cc8ca7cb27b4fd49b8c78"
                "7085d2451fd09e4e1cf471f23c916a5a8524ff27dfa1ce3de3568949c83140ecd9250d2f07"
                "02789c01be0041ff88a474797065b0757365725f6365727469666963617465a6617574686f"
                "72aa616c6963654064657631a974696d657374616d70d70141d782f840000000a775736572"
                "5f6964a3626f62ac68756d616e5f68616e646c6592af626f62406578616d706c652e636f6d"
                "ae426f6279204d63426f6246616365aa7075626c69635f6b6579c4207c999e9980bef37707"
                "068b07d975591efc56335be9634ceef7c932a09c891e25a869735f61646d696ec2a770726f"
                "66696c65a85354414e44415244a7225235"
            )[..],
            anonymous_cmds::AnyCmdReq::OrganizationBootstrap(
                anonymous_cmds::organization_bootstrap::Req {
                    bootstrap_token:
                        "0db537dee3ff9a3c2f76e337a4461f41fb3d738f35eb48f3759046dfbedb2e79".into(),
                    root_verify_key: VerifyKey::try_from(hex!(
                        "be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd"
                    ))
                    .unwrap(),
                    user_certificate: Bytes::from_static(&USER_CERTIF),
                    device_certificate: Bytes::from_static(&DEVICE_CERTIF),
                    redacted_user_certificate: Bytes::from_static(&REDACTED_USER_CERTIF),
                    redacted_device_certificate: Bytes::from_static(&REDACTED_DEVICE_CERTIF),
                    sequester_authority_certificate: None,
                },
            ),
        ),
        (
            // Generated from Rust implementation (Parsec v2.14.1+dev)
            // Content:
            //   bootstrap_token: "0db537dee3ff9a3c2f76e337a4461f41fb3d738f35eb48f3759046dfbedb2e79"
            //   cmd: "organization_bootstrap"
            //   device_certificate: hex!(
            //     "1e600d29590d2e005e19cdc7b2a75b54a549c15e9eb57213aa5bdd3696d0a7c68cb88f3209d1e682"
            //     "cfa5754229165924900a5e56ef571dc53fa2ba4ba51e0c04789c019c0063ff86a474797065b26465"
            //     "766963655f6365727469666963617465a6617574686f72aa616c6963654064657631a974696d6573"
            //     "74616d70d70141d782f840000000a96465766963655f6964a8626f624064657631ac646576696365"
            //     "5f6c6162656caf4d792064657631206d616368696e65aa7665726966795f6b6579c420840d872f42"
            //     "52da2d1c9f81a77db5f0a5b9b60a5cde1eeabf40388ef6bca649094f5b436f"
            //   )
            //   redacted_device_certificate: hex!(
            //     "2e44c56592b613a94ae151cc3b754945e300f457eb71d9f6af18f026f943bd0c1b3a029c1fbc504a"
            //     "a793f7937067864cb4fc0c7eeba7e1f9d83dfec5fb9b3d01789c018d0072ff86a474797065b26465"
            //     "766963655f6365727469666963617465a6617574686f72aa616c6963654064657631a974696d6573"
            //     "74616d70d70141d782f840000000a96465766963655f6964a8626f624064657631ac646576696365"
            //     "5f6c6162656cc0aa7665726966795f6b6579c420840d872f4252da2d1c9f81a77db5f0a5b9b60a5c"
            //     "de1eeabf40388ef6bca64909da083e35"
            //   )
            //   redacted_user_certificate: hex!(
            //     "e04ff4746d51000f68eb3dcfde36c53fe60f919c5eb8e76bbe2e4748b71553484cf5e4a73fde1650"
            //     "736538d5f70f83205307b216d3109ca1b9ca66f453089009789c019f0060ff88a474797065b07573"
            //     "65725f6365727469666963617465a6617574686f72aa616c6963654064657631a974696d65737461"
            //     "6d70d70141d782f840000000a7757365725f6964a3626f62ac68756d616e5f68616e646c65c0aa70"
            //     "75626c69635f6b6579c4207c999e9980bef37707068b07d975591efc56335be9634ceef7c932a09c"
            //     "891e25a869735f61646d696ec2a770726f66696c65a85354414e44415244fe7e465c"
            //   )
            //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
            //   sequester_authority_certificate: hex!("666f6f")
            //   user_certificate: hex!(
            //     "fee16e1a7aff6fbe84eb9f1b3008e37cc8ca7cb27b4fd49b8c787085d2451fd09e4e1cf471f23c91"
            //     "6a5a8524ff27dfa1ce3de3568949c83140ecd9250d2f0702789c01be0041ff88a474797065b07573"
            //     "65725f6365727469666963617465a6617574686f72aa616c6963654064657631a974696d65737461"
            //     "6d70d70141d782f840000000a7757365725f6964a3626f62ac68756d616e5f68616e646c6592af62"
            //     "6f62406578616d706c652e636f6dae426f6279204d63426f6246616365aa7075626c69635f6b6579"
            //     "c4207c999e9980bef37707068b07d975591efc56335be9634ceef7c932a09c891e25a869735f6164"
            //     "6d696ec2a770726f66696c65a85354414e44415244a7225235"
            //   )
            //
            &hex!(
                "88a3636d64b66f7267616e697a6174696f6e5f626f6f747374726170af626f6f7473747261"
                "705f746f6b656ed94030646235333764656533666639613363326637366533333761343436"
                "316634316662336437333866333565623438663337353930343664666265646232653739b2"
                "6465766963655f6365727469666963617465c4e71e600d29590d2e005e19cdc7b2a75b54a5"
                "49c15e9eb57213aa5bdd3696d0a7c68cb88f3209d1e682cfa5754229165924900a5e56ef57"
                "1dc53fa2ba4ba51e0c04789c019c0063ff86a474797065b26465766963655f636572746966"
                "6963617465a6617574686f72aa616c6963654064657631a974696d657374616d70d70141d7"
                "82f840000000a96465766963655f6964a8626f624064657631ac6465766963655f6c616265"
                "6caf4d792064657631206d616368696e65aa7665726966795f6b6579c420840d872f4252da"
                "2d1c9f81a77db5f0a5b9b60a5cde1eeabf40388ef6bca649094f5b436fbb72656461637465"
                "645f6465766963655f6365727469666963617465c4d82e44c56592b613a94ae151cc3b7549"
                "45e300f457eb71d9f6af18f026f943bd0c1b3a029c1fbc504aa793f7937067864cb4fc0c7e"
                "eba7e1f9d83dfec5fb9b3d01789c018d0072ff86a474797065b26465766963655f63657274"
                "69666963617465a6617574686f72aa616c6963654064657631a974696d657374616d70d701"
                "41d782f840000000a96465766963655f6964a8626f624064657631ac6465766963655f6c61"
                "62656cc0aa7665726966795f6b6579c420840d872f4252da2d1c9f81a77db5f0a5b9b60a5c"
                "de1eeabf40388ef6bca64909da083e35b972656461637465645f757365725f636572746966"
                "6963617465c4eae04ff4746d51000f68eb3dcfde36c53fe60f919c5eb8e76bbe2e4748b715"
                "53484cf5e4a73fde1650736538d5f70f83205307b216d3109ca1b9ca66f453089009789c01"
                "9f0060ff88a474797065b0757365725f6365727469666963617465a6617574686f72aa616c"
                "6963654064657631a974696d657374616d70d70141d782f840000000a7757365725f6964a3"
                "626f62ac68756d616e5f68616e646c65c0aa7075626c69635f6b6579c4207c999e9980bef3"
                "7707068b07d975591efc56335be9634ceef7c932a09c891e25a869735f61646d696ec2a770"
                "726f66696c65a85354414e44415244fe7e465caf726f6f745f7665726966795f6b6579c420"
                "be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbdbf73657175"
                "65737465725f617574686f726974795f6365727469666963617465c403666f6fb075736572"
                "5f6365727469666963617465c50109fee16e1a7aff6fbe84eb9f1b3008e37cc8ca7cb27b4f"
                "d49b8c787085d2451fd09e4e1cf471f23c916a5a8524ff27dfa1ce3de3568949c83140ecd9"
                "250d2f0702789c01be0041ff88a474797065b0757365725f6365727469666963617465a661"
                "7574686f72aa616c6963654064657631a974696d657374616d70d70141d782f840000000a7"
                "757365725f6964a3626f62ac68756d616e5f68616e646c6592af626f62406578616d706c65"
                "2e636f6dae426f6279204d63426f6246616365aa7075626c69635f6b6579c4207c999e9980"
                "bef37707068b07d975591efc56335be9634ceef7c932a09c891e25a869735f61646d696ec2"
                "a770726f66696c65a85354414e44415244a7225235"
            )[..],
            anonymous_cmds::AnyCmdReq::OrganizationBootstrap(
                anonymous_cmds::organization_bootstrap::Req {
                    bootstrap_token:
                        "0db537dee3ff9a3c2f76e337a4461f41fb3d738f35eb48f3759046dfbedb2e79".into(),
                    root_verify_key: VerifyKey::try_from(hex!(
                        "be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd"
                    ))
                    .unwrap(),
                    user_certificate: Bytes::from_static(&USER_CERTIF),
                    device_certificate: Bytes::from_static(&DEVICE_CERTIF),
                    redacted_user_certificate: Bytes::from_static(&REDACTED_USER_CERTIF),
                    redacted_device_certificate: Bytes::from_static(&REDACTED_DEVICE_CERTIF),
                    sequester_authority_certificate: Some(Bytes::from_static(b"foo")),
                },
            ),
        ),
    ];

    for (raw, expected) in raw_expected {
        let data = anonymous_cmds::AnyCmdReq::load(raw).unwrap();

        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let anonymous_cmds::AnyCmdReq::OrganizationBootstrap(req2) = data else {
            unreachable!()
        };

        let raw2 = req2.dump().unwrap();

        let data2 = anonymous_cmds::AnyCmdReq::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}

// Responses

pub fn rep_ok() {
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   status: "ok"
    //
    let raw = hex!("81a6737461747573a26f6b");

    let expected = anonymous_cmds::organization_bootstrap::Rep::Ok;

    let data = anonymous_cmds::organization_bootstrap::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::organization_bootstrap::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_certification() {
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   reason: "foobar"
    //   status: "invalid_certification"
    //
    let raw = hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573b5696e76616c69645f636572746966"
        "69636174696f6e"
    );

    let expected = anonymous_cmds::organization_bootstrap::Rep::InvalidCertification {
        reason: Some("foobar".into()),
    };

    let data = anonymous_cmds::organization_bootstrap::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::organization_bootstrap::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_data() {
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   reason: "foobar"
    //   status: "invalid_data"
    //
    let raw = hex!("82a6726561736f6ea6666f6f626172a6737461747573ac696e76616c69645f64617461");

    let expected = anonymous_cmds::organization_bootstrap::Rep::InvalidData {
        reason: Some("foobar".into()),
    };

    let data = anonymous_cmds::organization_bootstrap::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::organization_bootstrap::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_bad_timestamp() {
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   backend_timestamp: ext(1, 946774800.0)
    //   ballpark_client_early_offset: 300.0
    //   ballpark_client_late_offset: 320.0
    //   client_timestamp: ext(1, 946774800.0)
    //   status: "bad_timestamp"
    //
    let raw = hex!(
        "85b16261636b656e645f74696d657374616d70d70141cc375188000000bc62616c6c706172"
        "6b5f636c69656e745f6561726c795f6f6666736574cb4072c00000000000bb62616c6c7061"
        "726b5f636c69656e745f6c6174655f6f6666736574cb4074000000000000b0636c69656e74"
        "5f74696d657374616d70d70141cc375188000000a6737461747573ad6261645f74696d6573"
        "74616d70"
    );

    let expected = anonymous_cmds::organization_bootstrap::Rep::BadTimestamp {
        reason: None,
        ballpark_client_early_offset: 300.0,
        ballpark_client_late_offset: 320.0,
        backend_timestamp: "2000-01-02T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-01-02T01:00:00Z".parse().unwrap(),
    };

    let data = anonymous_cmds::organization_bootstrap::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::organization_bootstrap::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_already_bootstrapped() {
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   status: "already_bootstrapped"
    //
    let raw = hex!("81a6737461747573b4616c72656164795f626f6f747374726170706564");

    let expected = anonymous_cmds::organization_bootstrap::Rep::AlreadyBootstrapped;

    let data = anonymous_cmds::organization_bootstrap::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::organization_bootstrap::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_found() {
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   status: "not_found"
    //
    let raw = hex!("81a6737461747573a96e6f745f666f756e64");

    let expected = anonymous_cmds::organization_bootstrap::Rep::NotFound;

    let data = anonymous_cmds::organization_bootstrap::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::organization_bootstrap::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
