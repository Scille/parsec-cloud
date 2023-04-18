// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;

use libparsec_protocol::{
    anonymous_cmds::v2 as anonymous_cmds, authenticated_cmds::v2 as authenticated_cmds,
};
use libparsec_tests_fixtures::{
    alice, device_certificate, parsec_test, redacted_device_certificate, redacted_user_certificate,
    user_certificate, Device,
};
use libparsec_types::prelude::*;

type OrganizationBootstrapGenerator =
    Box<dyn FnOnce(&Device, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) -> anonymous_cmds::AnyCmdReq>;

#[parsec_test]
fn serde_organization_stats_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "organization_stats"
    let raw = hex!("81a3636d64b26f7267616e697a6174696f6e5f7374617473");

    let req = authenticated_cmds::organization_stats::Req;

    let expected = authenticated_cmds::AnyCmdReq::OrganizationStats(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   active_users: 1
    //   raw_size: 8
    //   metaraw_size: 8
    //   realms: 1
    //   status: "ok"
    //   users: 1
    //   users_per_profile_detail: [{active:1, profile:"ADMIN", revoked:0}]
    &hex!(
        "87ac6163746976655f757365727301a9646174615f73697a6508ad6d657461646174615f73"
        "697a6508a67265616c6d7301a6737461747573a26f6ba5757365727301b875736572735f70"
        "65725f70726f66696c655f64657461696c9183a770726f66696c65a541444d494ea6616374"
        "69766501a77265766f6b656400"
    )[..],
    authenticated_cmds::organization_stats::Rep::Ok {
        data_size: 8,
        metadata_size: 8,
        realms: 1,
        users: 1,
        active_users: 1,
        users_per_profile_detail: vec![UsersPerProfileDetailItem {
            profile: UserProfile::Admin,
            active: 1,
            revoked: 0,
        }],
    }
)]
#[case::not_allowed(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_allowed"
    &hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573ab6e6f745f616c6c6f776564"
    )[..],
    authenticated_cmds::organization_stats::Rep::NotAllowed {
        reason: Some("foobar".to_owned())
    }
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::organization_stats::Rep::NotFound
)]
fn serde_organization_stats_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::organization_stats::Rep,
) {
    let data = authenticated_cmds::organization_stats::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::organization_stats::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_organization_config_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "organization_config"
    let raw = hex!("81a3636d64b36f7267616e697a6174696f6e5f636f6e666967");

    let req = authenticated_cmds::organization_config::Req;

    let expected = authenticated_cmds::AnyCmdReq::OrganizationConfig(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok_with_absent_sequester(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   active_users_limit: 1
    //   status: "ok"
    //   user_profile_outsider_allowed: false
    &hex!(
        "83b26163746976655f75736572735f6c696d697401a6737461747573a26f6bbd757365725f"
        "70726f66696c655f6f757473696465725f616c6c6f776564c2"
    )[..],
    authenticated_cmds::organization_config::Rep::Ok {
        user_profile_outsider_allowed: false,
        active_users_limit: ActiveUsersLimit::LimitedTo(1),
        sequester_authority_certificate: Maybe::Absent,
        sequester_services_certificates: Maybe::Absent,
    }
)]
#[case::ok_with_none_sequester(
    // Generated from Rust implementation (Parsec v2.12.1+dev)
    // Content:
    //   active_users_limit: None
    //   sequester_authority_certificate: None
    //   sequester_services_certificates: None
    //   status: "ok"
    //   user_profile_outsider_allowed: false
    //
    &hex!(
        "85a6737461747573a26f6bbd757365725f70726f66696c655f6f757473696465725f616c6c"
        "6f776564c2b26163746976655f75736572735f6c696d6974c0bf7365717565737465725f61"
        "7574686f726974795f6365727469666963617465c0bf7365717565737465725f7365727669"
        "6365735f636572746966696361746573c0"
    )[..],
    authenticated_cmds::organization_config::Rep::Ok {
        user_profile_outsider_allowed: false,
        active_users_limit: ActiveUsersLimit::NoLimit,
        sequester_authority_certificate: Maybe::Present(None),
        sequester_services_certificates: Maybe::Present(None),
    }
)]
#[case::ok_with_sequester(
    // Generated from Python implementation (Parsec v2.11.1+dev)
    // Content:
    //   active_users_limit: 1
    //   sequester_authority_certificate: hex!("666f6f626172")
    //   sequester_services_certificates: [hex!("666f6f"), hex!("626172")]
    //   status: "ok"
    //   user_profile_outsider_allowed: false
    //
    &hex!(
    "85b26163746976655f75736572735f6c696d697401bf7365717565737465725f617574686f"
    "726974795f6365727469666963617465c406666f6f626172bf7365717565737465725f7365"
    "7276696365735f63657274696669636174657392c403666f6fc403626172a6737461747573"
    "a26f6bbd757365725f70726f66696c655f6f757473696465725f616c6c6f776564c2"
    )[..],
    authenticated_cmds::organization_config::Rep::Ok {
        user_profile_outsider_allowed: false,
        active_users_limit: ActiveUsersLimit::LimitedTo(1),
        sequester_authority_certificate: Maybe::Present(Some(b"foobar".to_vec())),
        sequester_services_certificates: Maybe::Present(Some(vec![b"foo".to_vec(), b"bar".to_vec()])),
    }
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::organization_config::Rep::NotFound
)]
fn serde_organization_config_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::organization_config::Rep,
) {
    let data = authenticated_cmds::organization_config::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::organization_config::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::absent_sequester_authority(
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
        "87a3636d64b66f7267616e697a6174696f6e5f626f6f747374726170af626f6f7473747261"
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
        "be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbdb075736572"
        "5f6365727469666963617465c50109fee16e1a7aff6fbe84eb9f1b3008e37cc8ca7cb27b4f"
        "d49b8c787085d2451fd09e4e1cf471f23c916a5a8524ff27dfa1ce3de3568949c83140ecd9"
        "250d2f0702789c01be0041ff88a474797065b0757365725f6365727469666963617465a661"
        "7574686f72aa616c6963654064657631a974696d657374616d70d70141d782f840000000a7"
        "757365725f6964a3626f62ac68756d616e5f68616e646c6592af626f62406578616d706c65"
        "2e636f6dae426f6279204d63426f6246616365aa7075626c69635f6b6579c4207c999e9980"
        "bef37707068b07d975591efc56335be9634ceef7c932a09c891e25a869735f61646d696ec2"
        "a770726f66696c65a85354414e44415244a7225235"
    )[..],
    Box::new(|
        alice: &Device,
        user_certificate: Vec<u8>,
        redacted_user_certificate: Vec<u8>,
        device_certificate: Vec<u8>,
        redacted_device_certificate: Vec<u8>
        | {
        anonymous_cmds::AnyCmdReq::OrganizationBootstrap(anonymous_cmds::organization_bootstrap::Req {
            bootstrap_token: "0db537dee3ff9a3c2f76e337a4461f41fb3d738f35eb48f3759046dfbedb2e79".into(),
            root_verify_key: alice.root_verify_key().clone(),
            user_certificate,
            device_certificate,
            redacted_user_certificate,
            redacted_device_certificate,
            sequester_authority_certificate: Maybe::Absent,
        })
    })
)]
#[case::none_sequester_authority(
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
    Box::new(|
        alice: &Device,
        user_certificate: Vec<u8>,
        redacted_user_certificate: Vec<u8>,
        device_certificate: Vec<u8>,
        redacted_device_certificate: Vec<u8>
        | {
        anonymous_cmds::AnyCmdReq::OrganizationBootstrap(anonymous_cmds::organization_bootstrap::Req {
            bootstrap_token: "0db537dee3ff9a3c2f76e337a4461f41fb3d738f35eb48f3759046dfbedb2e79".into(),
            root_verify_key: alice.root_verify_key().clone(),
            user_certificate,
            device_certificate,
            redacted_user_certificate,
            redacted_device_certificate,
            sequester_authority_certificate: Maybe::Present(None),
        })
    })
)]
#[case::with_sequester_authority(
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
    Box::new(|
        alice: &Device,
        user_certificate: Vec<u8>,
        redacted_user_certificate: Vec<u8>,
        device_certificate: Vec<u8>,
        redacted_device_certificate: Vec<u8>
        | {
        anonymous_cmds::AnyCmdReq::OrganizationBootstrap(anonymous_cmds::organization_bootstrap::Req {
            bootstrap_token: "0db537dee3ff9a3c2f76e337a4461f41fb3d738f35eb48f3759046dfbedb2e79".into(),
            root_verify_key: alice.root_verify_key().clone(),
            user_certificate,
            device_certificate,
            redacted_user_certificate,
            redacted_device_certificate,
            sequester_authority_certificate: Maybe::Present(Some(b"foo".to_vec())),
        })
    })
)]
fn serde_organization_bootstrap_req(
    alice: &Device,
    user_certificate: &[u8],
    redacted_user_certificate: &[u8],
    device_certificate: &[u8],
    redacted_device_certificate: &[u8],
    #[case] raw: &[u8],
    #[case] generate_expected: OrganizationBootstrapGenerator,
) {
    let expected = generate_expected(
        alice,
        user_certificate.to_vec(),
        redacted_user_certificate.to_vec(),
        device_certificate.to_vec(),
        redacted_device_certificate.to_vec(),
    );

    let data = anonymous_cmds::AnyCmdReq::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   status: "ok"
    //
    &hex!("81a6737461747573a26f6b")[..],
    anonymous_cmds::organization_bootstrap::Rep::Ok
)]
#[case::invalid_certification(
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   reason: "foobar"
    //   status: "invalid_certification"
    //
    &hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573b5696e76616c69645f636572746966"
        "69636174696f6e"
    )[..],
    anonymous_cmds::organization_bootstrap::Rep::InvalidCertification {
        reason: Some("foobar".into()),
    }
)]
#[case::invalid_data(
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   reason: "foobar"
    //   status: "invalid_data"
    //
    &hex!("82a6726561736f6ea6666f6f626172a6737461747573ac696e76616c69645f64617461")[..],
    anonymous_cmds::organization_bootstrap::Rep::InvalidData {
        reason: Some("foobar".into()),
    }
)]
#[case::bad_timestamp(
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   backend_timestamp: ext(1, 946774800.0)
    //   ballpark_client_early_offset: 300.0
    //   ballpark_client_late_offset: 320.0
    //   client_timestamp: ext(1, 946774800.0)
    //   status: "bad_timestamp"
    //
    &hex!(
        "85b16261636b656e645f74696d657374616d70d70141cc375188000000bc62616c6c706172"
        "6b5f636c69656e745f6561726c795f6f6666736574cb4072c00000000000bb62616c6c7061"
        "726b5f636c69656e745f6c6174655f6f6666736574cb4074000000000000b0636c69656e74"
        "5f74696d657374616d70d70141cc375188000000a6737461747573ad6261645f74696d6573"
        "74616d70"
    )[..],
    anonymous_cmds::organization_bootstrap::Rep::BadTimestamp {
        reason: None,
        ballpark_client_early_offset: Maybe::Present(300.0),
        ballpark_client_late_offset: Maybe::Present(320.0),
        backend_timestamp: Maybe::Present("2000-01-02T01:00:00Z".parse().unwrap()),
        client_timestamp: Maybe::Present("2000-01-02T01:00:00Z".parse().unwrap()),
    }
)]
#[case::already_bootstrapped(
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   status: "already_bootstrapped"
    //
    &hex!("81a6737461747573b4616c72656164795f626f6f747374726170706564")[..],
    anonymous_cmds::organization_bootstrap::Rep::AlreadyBootstrapped
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   status: "not_found"
    //
    &hex!("81a6737461747573a96e6f745f666f756e64")[..],
    anonymous_cmds::organization_bootstrap::Rep::NotFound
)]
fn serde_organization_bootstrap_rep(
    #[case] raw: &[u8],
    #[case] expected: anonymous_cmds::organization_bootstrap::Rep,
) {
    let data = anonymous_cmds::organization_bootstrap::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::organization_bootstrap::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}
