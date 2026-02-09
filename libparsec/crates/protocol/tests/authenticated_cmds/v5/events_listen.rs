// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   cmd: "events_listen"
    let raw = hex!("81a3636d64ad6576656e74735f6c697374656e");

    let req = authenticated_cmds::events_listen::Req;

    let expected = authenticated_cmds::AnyCmdReq::EventsListen(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::EventsListen(req2) = data else {
        unreachable!();
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    let raw_expected = [
        (
            // Generated from Rust implementation (Parsec v3.0.0+dev)
            // Content:
            //   event: "PINGED"
            //   ping: "foobar"
            //   status: "ok"
            &hex!("83a6737461747573a26f6ba56576656e74a650494e474544a470696e67a6666f6f626172")[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::Pinged {
                    ping: "foobar".to_owned(),
                },
            ),
        ),
        (
            // Generated from Parsec 3.2.5-a.0+dev
            // Content:
            //   status: 'ok'
            //   event: 'ORGANIZATION_CONFIG'
            //   active_users_limit: 8
            //   sse_keepalive_seconds: 30
            //   user_profile_outsider_allowed: True
            &hex!(
                "85a6737461747573a26f6ba56576656e74b34f5247414e495a4154494f4e5f434f4e46"
                "4947b26163746976655f75736572735f6c696d697408b57373655f6b656570616c6976"
                "655f7365636f6e64731ebd757365725f70726f66696c655f6f757473696465725f616c"
                "6c6f776564c3"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::OrganizationConfig {
                    active_users_limit: ActiveUsersLimit::LimitedTo(8),
                    user_profile_outsider_allowed: true,
                    sse_keepalive_seconds: Some(30.try_into().unwrap()),
                },
            ),
        ),
        (
            // Generated from Parsec 3.2.5-a.0+dev
            // Content:
            //   status: 'ok'
            //   event: 'ORGANIZATION_CONFIG'
            //   active_users_limit: None
            //   sse_keepalive_seconds: None
            //   user_profile_outsider_allowed: False
            &hex!(
                "85a6737461747573a26f6ba56576656e74b34f5247414e495a4154494f4e5f434f4e46"
                "4947b26163746976655f75736572735f6c696d6974c0b57373655f6b656570616c6976"
                "655f7365636f6e64731ebd757365725f70726f66696c655f6f757473696465725f616c"
                "6c6f776564c2"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::OrganizationConfig {
                    active_users_limit: ActiveUsersLimit::NoLimit,
                    user_profile_outsider_allowed: false,
                    sse_keepalive_seconds: Some(30.try_into().unwrap()),
                },
            ),
        ),
        (
            // Legacy 5.1>=API<5.3 format (with `allowed_client_agent/account_vault_strategy` fields`)
            // Generated from Parsec 3.4.0-a.7+dev
            // Content:
            //   status: 'ok'
            //   event: 'ORGANIZATION_CONFIG'
            //   account_vault_strategy: 'ALLOWED'
            //   active_users_limit: 8
            //   allowed_client_agent: 'NATIVE_OR_WEB'
            //   sse_keepalive_seconds: 30
            //   user_profile_outsider_allowed: True
            &hex!(
            "87a6737461747573a26f6ba56576656e74b34f5247414e495a4154494f4e5f434f4e46"
            "4947b66163636f756e745f7661756c745f7374726174656779a7414c4c4f574544b261"
            "63746976655f75736572735f6c696d697408b4616c6c6f7765645f636c69656e745f61"
            "67656e74ad4e41544956455f4f525f574542b57373655f6b656570616c6976655f7365"
            "636f6e64731ebd757365725f70726f66696c655f6f757473696465725f616c6c6f7765"
            "64c3"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::OrganizationConfig {
                    active_users_limit: ActiveUsersLimit::LimitedTo(8),
                    user_profile_outsider_allowed: true,
                    sse_keepalive_seconds: Some(30.try_into().unwrap()),
                },
            ),
        ),
        (
            // Legacy 5.1>=API<5.3 format (with `allowed_client_agent/account_vault_strategy` fields`)
            // Generated from Parsec 3.4.0-a.7+dev
            // Content:
            //   status: 'ok'
            //   event: 'ORGANIZATION_CONFIG'
            //   account_vault_strategy: 'FORBIDDEN'
            //   active_users_limit: None
            //   allowed_client_agent: 'NATIVE_ONLY'
            //   sse_keepalive_seconds: 30
            //   user_profile_outsider_allowed: False
            &hex!(
            "87a6737461747573a26f6ba56576656e74b34f5247414e495a4154494f4e5f434f4e46"
            "4947b66163636f756e745f7661756c745f7374726174656779a9464f5242494444454e"
            "b26163746976655f75736572735f6c696d6974c0b4616c6c6f7765645f636c69656e74"
            "5f6167656e74ab4e41544956455f4f4e4c59b57373655f6b656570616c6976655f7365"
            "636f6e64731ebd757365725f70726f66696c655f6f757473696465725f616c6c6f7765"
            "64c2"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::OrganizationConfig {
                    active_users_limit: ActiveUsersLimit::NoLimit,
                    user_profile_outsider_allowed: false,
                    sse_keepalive_seconds: Some(30.try_into().unwrap()),
                },
            ),
        ),
        (
            // Generated from Parsec 3.2.5-a.0+dev
            // Content:
            //   status: 'ok'
            //   event: 'INVITATION'
            //   invitation_status: 'PENDING'
            //   token: 0xd864b93ded264aae9ae583fd3d40c45a
            &hex!(
                "84a6737461747573a26f6ba56576656e74aa494e5649544154494f4eb1696e76697461"
                "74696f6e5f737461747573a750454e44494e47a5746f6b656ec410d864b93ded264aae"
                "9ae583fd3d40c45a"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::Invitation {
                    invitation_status: InvitationStatus::Pending,
                    token: AccessToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
                },
            ),
        ),
        (
            // Generated from Rust implementation (Parsec v3.0.0+dev)
            // Content:
            //   event: "PKI_ENROLLMENT"
            //   status: "ok"
            &hex!("82a6737461747573a26f6ba56576656e74ae504b495f454e524f4c4c4d454e54")[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::PkiEnrollment,
            ),
        ),
        (
            // Generated from Parsec v3.0.0-b.11+dev
            // Content:
            //   event: "COMMON_CERTIFICATE"
            //   status: "ok"
            //   timestamp: ext(1, 946774800.0)
            &hex!(
                "83a6737461747573a26f6ba56576656e74b2434f4d4d4f4e5f43455254494649434154"
                "45a974696d657374616d70d70100035d162fa2e400"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::CommonCertificate {
                    timestamp: "2000-01-02T01:00:00Z".parse().unwrap(),
                },
            ),
        ),
        (
            // Generated from Parsec v3.0.0-b.11+dev
            // Content:
            //   event: "SEQUESTER_CERTIFICATE"
            //   status: "ok"
            //   timestamp: ext(1, 946774800.0)
            &hex!(
                "83a6737461747573a26f6ba56576656e74b55345515545535445525f43455254494649"
                "43415445a974696d657374616d70d70100035d162fa2e400"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::SequesterCertificate {
                    timestamp: "2000-01-02T01:00:00Z".parse().unwrap(),
                },
            ),
        ),
        (
            // Generated from Parsec v3.0.0-b.11+dev
            // Content:
            //   event: "SHAMIR_RECOVERY_CERTIFICATE"
            //   status: "ok"
            //   timestamp: ext(1, 946774800.0)
            &hex!(
            "83a6737461747573a26f6ba56576656e74bb5348414d49525f5245434f564552595f43"
            "45525449464943415445a974696d657374616d70d70100035d162fa2e400"
                )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::ShamirRecoveryCertificate {
                    timestamp: "2000-01-02T01:00:00Z".parse().unwrap(),
                },
            ),
        ),
    ];

    for (raw, expected) in raw_expected {
        println!("***expected: {:?}", expected.dump().unwrap());
        let data = authenticated_cmds::events_listen::Rep::load(raw).unwrap();

        assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = data.dump().unwrap();

        let data2 = authenticated_cmds::events_listen::Rep::load(&raw2).unwrap();

        assert_eq!(data2, expected);
    }
}

pub fn rep_not_available() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   status: "not_available"
    let raw = hex!("81a6737461747573ad6e6f745f617661696c61626c65");

    let expected = authenticated_cmds::events_listen::Rep::NotAvailable;

    let data = authenticated_cmds::events_listen::Rep::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::events_listen::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}
