// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::anonymous_cmds;

// Request

pub fn req() {
    let bootstrap_token = BootstrapToken::from_hex("672bc6ba9c43455da28344e975dc72b7").unwrap();

    let raw_expected = [
        (
            // Generated from Rust implementation (Parsec v3.0.0+dev)
            // Content:
            //   bootstrap_token: "672bc6ba9c43455da28344e975dc72b7"
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
                "705f746f6b656ed802672bc6ba9c43455da28344e975dc72b7af726f6f745f766572696679"
                "5f6b6579c420be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9b"
                "bdb0757365725f6365727469666963617465c50109fee16e1a7aff6fbe84eb9f1b3008e37c"
                "c8ca7cb27b4fd49b8c787085d2451fd09e4e1cf471f23c916a5a8524ff27dfa1ce3de35689"
                "49c83140ecd9250d2f0702789c01be0041ff88a474797065b0757365725f63657274696669"
                "63617465a6617574686f72aa616c6963654064657631a974696d657374616d70d70141d782"
                "f840000000a7757365725f6964a3626f62ac68756d616e5f68616e646c6592af626f624065"
                "78616d706c652e636f6dae426f6279204d63426f6246616365aa7075626c69635f6b6579c4"
                "207c999e9980bef37707068b07d975591efc56335be9634ceef7c932a09c891e25a869735f"
                "61646d696ec2a770726f66696c65a85354414e44415244a7225235b26465766963655f6365"
                "727469666963617465c4e71e600d29590d2e005e19cdc7b2a75b54a549c15e9eb57213aa5b"
                "dd3696d0a7c68cb88f3209d1e682cfa5754229165924900a5e56ef571dc53fa2ba4ba51e0c"
                "04789c019c0063ff86a474797065b26465766963655f6365727469666963617465a6617574"
                "686f72aa616c6963654064657631a974696d657374616d70d70141d782f840000000a96465"
                "766963655f6964a8626f624064657631ac6465766963655f6c6162656caf4d792064657631"
                "206d616368696e65aa7665726966795f6b6579c420840d872f4252da2d1c9f81a77db5f0a5"
                "b9b60a5cde1eeabf40388ef6bca649094f5b436fb972656461637465645f757365725f6365"
                "727469666963617465c4eae04ff4746d51000f68eb3dcfde36c53fe60f919c5eb8e76bbe2e"
                "4748b71553484cf5e4a73fde1650736538d5f70f83205307b216d3109ca1b9ca66f4530890"
                "09789c019f0060ff88a474797065b0757365725f6365727469666963617465a6617574686f"
                "72aa616c6963654064657631a974696d657374616d70d70141d782f840000000a775736572"
                "5f6964a3626f62ac68756d616e5f68616e646c65c0aa7075626c69635f6b6579c4207c999e"
                "9980bef37707068b07d975591efc56335be9634ceef7c932a09c891e25a869735f61646d69"
                "6ec2a770726f66696c65a85354414e44415244fe7e465cbb72656461637465645f64657669"
                "63655f6365727469666963617465c4d82e44c56592b613a94ae151cc3b754945e300f457eb"
                "71d9f6af18f026f943bd0c1b3a029c1fbc504aa793f7937067864cb4fc0c7eeba7e1f9d83d"
                "fec5fb9b3d01789c018d0072ff86a474797065b26465766963655f63657274696669636174"
                "65a6617574686f72aa616c6963654064657631a974696d657374616d70d70141d782f84000"
                "0000a96465766963655f6964a8626f624064657631ac6465766963655f6c6162656cc0aa76"
                "65726966795f6b6579c420840d872f4252da2d1c9f81a77db5f0a5b9b60a5cde1eeabf4038"
                "8ef6bca64909da083e35bf7365717565737465725f617574686f726974795f636572746966"
                "6963617465c0"
            )[..],
            anonymous_cmds::AnyCmdReq::OrganizationBootstrap(
                anonymous_cmds::organization_bootstrap::Req {
                    bootstrap_token: Some(bootstrap_token),
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
            // Generated from Rust implementation (Parsec v3.0.0+dev)
            // Content:
            //   bootstrap_token: "672bc6ba9c43455da28344e975dc72b7"
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
                "705f746f6b656ed802672bc6ba9c43455da28344e975dc72b7af726f6f745f766572696679"
                "5f6b6579c420be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9b"
                "bdb0757365725f6365727469666963617465c50109fee16e1a7aff6fbe84eb9f1b3008e37c"
                "c8ca7cb27b4fd49b8c787085d2451fd09e4e1cf471f23c916a5a8524ff27dfa1ce3de35689"
                "49c83140ecd9250d2f0702789c01be0041ff88a474797065b0757365725f63657274696669"
                "63617465a6617574686f72aa616c6963654064657631a974696d657374616d70d70141d782"
                "f840000000a7757365725f6964a3626f62ac68756d616e5f68616e646c6592af626f624065"
                "78616d706c652e636f6dae426f6279204d63426f6246616365aa7075626c69635f6b6579c4"
                "207c999e9980bef37707068b07d975591efc56335be9634ceef7c932a09c891e25a869735f"
                "61646d696ec2a770726f66696c65a85354414e44415244a7225235b26465766963655f6365"
                "727469666963617465c4e71e600d29590d2e005e19cdc7b2a75b54a549c15e9eb57213aa5b"
                "dd3696d0a7c68cb88f3209d1e682cfa5754229165924900a5e56ef571dc53fa2ba4ba51e0c"
                "04789c019c0063ff86a474797065b26465766963655f6365727469666963617465a6617574"
                "686f72aa616c6963654064657631a974696d657374616d70d70141d782f840000000a96465"
                "766963655f6964a8626f624064657631ac6465766963655f6c6162656caf4d792064657631"
                "206d616368696e65aa7665726966795f6b6579c420840d872f4252da2d1c9f81a77db5f0a5"
                "b9b60a5cde1eeabf40388ef6bca649094f5b436fb972656461637465645f757365725f6365"
                "727469666963617465c4eae04ff4746d51000f68eb3dcfde36c53fe60f919c5eb8e76bbe2e"
                "4748b71553484cf5e4a73fde1650736538d5f70f83205307b216d3109ca1b9ca66f4530890"
                "09789c019f0060ff88a474797065b0757365725f6365727469666963617465a6617574686f"
                "72aa616c6963654064657631a974696d657374616d70d70141d782f840000000a775736572"
                "5f6964a3626f62ac68756d616e5f68616e646c65c0aa7075626c69635f6b6579c4207c999e"
                "9980bef37707068b07d975591efc56335be9634ceef7c932a09c891e25a869735f61646d69"
                "6ec2a770726f66696c65a85354414e44415244fe7e465cbb72656461637465645f64657669"
                "63655f6365727469666963617465c4d82e44c56592b613a94ae151cc3b754945e300f457eb"
                "71d9f6af18f026f943bd0c1b3a029c1fbc504aa793f7937067864cb4fc0c7eeba7e1f9d83d"
                "fec5fb9b3d01789c018d0072ff86a474797065b26465766963655f63657274696669636174"
                "65a6617574686f72aa616c6963654064657631a974696d657374616d70d70141d782f84000"
                "0000a96465766963655f6964a8626f624064657631ac6465766963655f6c6162656cc0aa76"
                "65726966795f6b6579c420840d872f4252da2d1c9f81a77db5f0a5b9b60a5cde1eeabf4038"
                "8ef6bca64909da083e35bf7365717565737465725f617574686f726974795f636572746966"
                "6963617465c403666f6f"
            )[..],
            anonymous_cmds::AnyCmdReq::OrganizationBootstrap(
                anonymous_cmds::organization_bootstrap::Req {
                    bootstrap_token: Some(bootstrap_token),
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

pub fn rep_invalid_certificate() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "invalid_certificate"
    //
    let raw = hex!("81a6737461747573b3696e76616c69645f6365727469666963617465");

    let expected = anonymous_cmds::organization_bootstrap::Rep::InvalidCertificate;

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

    let expected = anonymous_cmds::organization_bootstrap::Rep::InvalidCertificate;

    let data = anonymous_cmds::organization_bootstrap::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::organization_bootstrap::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_timestamp_out_of_ballpark() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   ballpark_client_early_offset: 300.0
    //   ballpark_client_late_offset: 320.0
    //   client_timestamp: ext(1, 946774800.0)
    //   server_timestamp: ext(1, 946774800.0)
    //   status: "timestamp_out_of_ballpark"
    //
    let raw = hex!(
        "85a6737461747573b974696d657374616d705f6f75745f6f665f62616c6c7061726bbc6261"
        "6c6c7061726b5f636c69656e745f6561726c795f6f6666736574cb4072c00000000000bb62"
        "616c6c7061726b5f636c69656e745f6c6174655f6f6666736574cb4074000000000000b063"
        "6c69656e745f74696d657374616d70d70141cc375188000000b07365727665725f74696d65"
        "7374616d70d70141cc375188000000"
    );

    let expected = anonymous_cmds::organization_bootstrap::Rep::TimestampOutOfBallpark {
        ballpark_client_early_offset: 300.0,
        ballpark_client_late_offset: 320.0,
        server_timestamp: "2000-01-02T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-01-02T01:00:00Z".parse().unwrap(),
    };

    let data = anonymous_cmds::organization_bootstrap::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::organization_bootstrap::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_organization_already_bootstrapped() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "organization_already_bootstrapped"
    //
    let raw = hex!(
        "81a6737461747573d9216f7267616e697a6174696f6e5f616c72656164795f626f6f747374"
        "726170706564"
    );

    let expected = anonymous_cmds::organization_bootstrap::Rep::OrganizationAlreadyBootstrapped;

    let data = anonymous_cmds::organization_bootstrap::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::organization_bootstrap::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_bootstrap_token() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "invalid_bootstrap_token"
    //
    let raw = hex!("81a6737461747573b7696e76616c69645f626f6f7473747261705f746f6b656e");

    let expected = anonymous_cmds::organization_bootstrap::Rep::InvalidBootstrapToken;

    let data = anonymous_cmds::organization_bootstrap::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::organization_bootstrap::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
