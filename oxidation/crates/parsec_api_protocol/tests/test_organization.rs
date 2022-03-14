// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use hex_literal::hex;
use parsec_api_types::UserProfile;
use rstest::rstest;

use parsec_api_crypto::VerifyKey;
use parsec_api_protocol::*;

#[rstest]
fn serde_api_v1_organization_bootstrap_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   bootstrap_token: "foobar"
    //   cmd: "api_v1_organization_bootstrap"
    //   device_certificate: hex!("666f6f626172")
    //   redacted_device_certificate: hex!("666f6f626172")
    //   redacted_user_certificate: hex!("666f6f626172")
    //   root_verify_key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //   user_certificate: hex!("666f6f626172")
    let data = hex!(
        "87af626f6f7473747261705f746f6b656ea6666f6f626172a3636d64bd6170695f76315f6f"
        "7267616e697a6174696f6e5f626f6f747374726170b26465766963655f6365727469666963"
        "617465c406666f6f626172bb72656461637465645f6465766963655f636572746966696361"
        "7465c406666f6f626172b972656461637465645f757365725f6365727469666963617465c4"
        "06666f6f626172af726f6f745f7665726966795f6b6579c4206507907d33bae6b5980b32fa"
        "03f3ebac56141b126e44f352ea46c5f22cd5ac57b0757365725f6365727469666963617465"
        "c406666f6f626172"
    );

    let expected = APIV1OrganizationBootstrapReq {
        cmd: "api_v1_organization_bootstrap".to_owned(),
        bootstrap_token: "foobar".to_owned(),
        root_verify_key: VerifyKey::from(hex!(
            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
        )),
        user_certificate: b"foobar".to_vec(),
        device_certificate: b"foobar".to_vec(),
        redacted_user_certificate: Some(b"foobar".to_vec()),
        redacted_device_certificate: Some(b"foobar".to_vec()),
    };

    let schema = APIV1OrganizationBootstrapReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = APIV1OrganizationBootstrapReq::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        &hex!(
            "81a6737461747573a26f6b"
        )[..],
        APIV1OrganizationBootstrapRep::Ok
    )
)]
#[case::invalid_certification(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "invalid_certification"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573b5696e76616c69645f636572746966"
            "69636174696f6e"
        )[..],
        APIV1OrganizationBootstrapRep::InvalidCertification {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::invalid_data(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "invalid_data"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573ac696e76616c69645f64617461"
        )[..],
        APIV1OrganizationBootstrapRep::InvalidData {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::already_bootstrapped(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "already_bootstrapped"
        &hex!(
            "81a6737461747573b4616c72656164795f626f6f747374726170706564"
        )[..],
        APIV1OrganizationBootstrapRep::AlreadyBootstrapped
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        APIV1OrganizationBootstrapRep::NotFound
    )
)]
fn serde_api_v1_organization_bootstrap_rep(
    #[case] data_expected: (&[u8], APIV1OrganizationBootstrapRep),
) {
    let (data, expected) = data_expected;

    let schema = APIV1OrganizationBootstrapRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = APIV1OrganizationBootstrapRep::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_organization_stats_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "organization_stats"
    let data = hex!("81a3636d64b26f7267616e697a6174696f6e5f7374617473");

    let expected = OrganizationStatsReq {
        cmd: "organization_stats".to_owned(),
    };

    let schema = OrganizationStatsReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = OrganizationStatsReq::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   active_users: 1
        //   data_size: 8
        //   metadata_size: 8
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
        OrganizationStatsRep::Ok {
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
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_allowed"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        OrganizationStatsRep::NotAllowed {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        OrganizationStatsRep::NotFound
    )
)]
fn serde_organization_stats_rep(#[case] data_expected: (&[u8], OrganizationStatsRep)) {
    let (data, expected) = data_expected;

    let schema = OrganizationStatsRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = OrganizationStatsRep::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_organization_config_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "organization_config"
    let data = hex!("81a3636d64b36f7267616e697a6174696f6e5f636f6e666967");

    let expected = OrganizationConfigReq {
        cmd: "organization_config".to_owned(),
    };

    let schema = OrganizationConfigReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = OrganizationConfigReq::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   active_users_limit: 1
        //   status: "ok"
        //   user_profile_outsider_allowed: false
        &hex!(
            "83b26163746976655f75736572735f6c696d697401a6737461747573a26f6bbd757365725f"
            "70726f66696c655f6f757473696465725f616c6c6f776564c2"
        )[..],
        OrganizationConfigRep::Ok {
            user_profile_outsider_allowed: false,
            active_users_limit: Some(1),
        }
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        OrganizationConfigRep::NotFound
    )
)]
fn serde_organization_config_rep(#[case] data_expected: (&[u8], OrganizationConfigRep)) {
    let (data, expected) = data_expected;

    let schema = OrganizationConfigRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = OrganizationConfigRep::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}
