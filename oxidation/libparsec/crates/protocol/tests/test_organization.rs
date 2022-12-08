// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use libparsec_types::{Maybe, UserProfile};
use rstest::rstest;

use libparsec_protocol::{
    anonymous_cmds::v2 as anonymous_cmds, authenticated_cmds::v2 as authenticated_cmds,
};
use libparsec_types::UsersPerProfileDetailItem;
use tests_fixtures::{alice, Device};

#[rstest]
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

#[rstest]
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

#[rstest]
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

#[rstest]
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
        active_users_limit: Some(1),
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
        active_users_limit: None,
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
        active_users_limit: Some(1),
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

#[rstest]
#[case::absent_sequester_authority(
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   bootstrap_token: "foo"
    //   cmd: "organization_bootstrap"
    //   device_certificate: hex!("666f6f")
    //   redacted_device_certificate: hex!("666f6f")
    //   redacted_user_certificate: hex!("666f6f")
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    //   user_certificate: hex!("666f6f")
    //
    &hex!(
        "87af626f6f7473747261705f746f6b656ea3666f6fa3636d64b66f7267616e697a6174696f"
        "6e5f626f6f747374726170b26465766963655f6365727469666963617465c403666f6fbb72"
        "656461637465645f6465766963655f6365727469666963617465c403666f6fb97265646163"
        "7465645f757365725f6365727469666963617465c403666f6faf726f6f745f766572696679"
        "5f6b6579c420be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9b"
        "bdb0757365725f6365727469666963617465c403666f6f"
    )[..],
    Box::new(|alice: &Device| {
        anonymous_cmds::AnyCmdReq::OrganizationBootstrap(anonymous_cmds::organization_bootstrap::Req {
            bootstrap_token: "foo".into(),
            root_verify_key: alice.root_verify_key().clone(),
            user_certificate: b"foo".to_vec(),
            device_certificate: b"foo".to_vec(),
            redacted_user_certificate: b"foo".to_vec(),
            redacted_device_certificate: b"foo".to_vec(),
            sequester_authority_certificate: Maybe::Absent,
        })
    })
)]
#[case::none_sequester_authority(
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   bootstrap_token: "foo"
    //   cmd: "organization_bootstrap"
    //   device_certificate: hex!("666f6f")
    //   redacted_device_certificate: hex!("666f6f")
    //   redacted_user_certificate: hex!("666f6f")
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    //   sequester_authority_certificate: None
    //   user_certificate: hex!("666f6f")
    //
    &hex!(
        "88af626f6f7473747261705f746f6b656ea3666f6fa3636d64b66f7267616e697a6174696f"
        "6e5f626f6f747374726170b26465766963655f6365727469666963617465c403666f6fbb72"
        "656461637465645f6465766963655f6365727469666963617465c403666f6fb97265646163"
        "7465645f757365725f6365727469666963617465c403666f6faf726f6f745f766572696679"
        "5f6b6579c420be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9b"
        "bdbf7365717565737465725f617574686f726974795f6365727469666963617465c0b07573"
        "65725f6365727469666963617465c403666f6f"
    )[..],
    Box::new(|alice: &Device| {
        anonymous_cmds::AnyCmdReq::OrganizationBootstrap(anonymous_cmds::organization_bootstrap::Req {
            bootstrap_token: "foo".into(),
            root_verify_key: alice.root_verify_key().clone(),
            user_certificate: b"foo".to_vec(),
            device_certificate: b"foo".to_vec(),
            redacted_user_certificate: b"foo".to_vec(),
            redacted_device_certificate: b"foo".to_vec(),
            sequester_authority_certificate: Maybe::Present(None),
        })
    })
)]
#[case::with_sequester_authority(
    // Generated from Python implementation (Parsec v2.14.1+dev)
    // Content:
    //   bootstrap_token: "foo"
    //   cmd: "organization_bootstrap"
    //   device_certificate: hex!("666f6f")
    //   redacted_device_certificate: hex!("666f6f")
    //   redacted_user_certificate: hex!("666f6f")
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    //   sequester_authority_certificate: hex!("666f6f")
    //   user_certificate: hex!("666f6f")
    //
    &hex!(
        "88af626f6f7473747261705f746f6b656ea3666f6fa3636d64b66f7267616e697a6174696f"
        "6e5f626f6f747374726170b26465766963655f6365727469666963617465c403666f6fbb72"
        "656461637465645f6465766963655f6365727469666963617465c403666f6fb97265646163"
        "7465645f757365725f6365727469666963617465c403666f6faf726f6f745f766572696679"
        "5f6b6579c420be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9b"
        "bdbf7365717565737465725f617574686f726974795f6365727469666963617465c403666f"
        "6fb0757365725f6365727469666963617465c403666f6f"
    )[..],
    Box::new(|alice: &Device| {
        anonymous_cmds::AnyCmdReq::OrganizationBootstrap(anonymous_cmds::organization_bootstrap::Req {
            bootstrap_token: "foo".into(),
            root_verify_key: alice.root_verify_key().clone(),
            user_certificate: b"foo".to_vec(),
            device_certificate: b"foo".to_vec(),
            redacted_user_certificate: b"foo".to_vec(),
            redacted_device_certificate: b"foo".to_vec(),
            sequester_authority_certificate: Maybe::Present(Some(b"foo".to_vec())),
        })
    })
)]
fn serde_organization_bootstrap_req(
    alice: &Device,
    #[case] raw: &[u8],
    #[case] generate_expected: Box<dyn FnOnce(&Device) -> anonymous_cmds::AnyCmdReq>,
) {
    let expected = generate_expected(alice);

    let data = anonymous_cmds::AnyCmdReq::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
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
    // Generated from Rust implementation (Parsec v2.14.1+dev)
    // Content:
    //   status: "already_bootstrapped"
    //
    &hex!("81a6737461747573b4616c72656164795f626f6f747374726170706564")[..],
    anonymous_cmds::organization_bootstrap::Rep::AlreadyBootstrapped
)]
#[case::not_found(
    // Generated from Rust implementation (Parsec v2.14.1+dev)
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
