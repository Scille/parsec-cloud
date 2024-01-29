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
            // Generated from Rust implementation (Parsec v3.0.0+dev)
            // Content:
            //   event: "SERVER_CONFIG"
            //   active_users_limit: 8
            //   user_profile_outsider_allowed: true
            //   status: "ok"
            &hex!(
                "84a6737461747573a26f6ba56576656e74ad5345525645525f434f4e464947b26163746976"
                "655f75736572735f6c696d697408bd757365725f70726f66696c655f6f757473696465725f"
                "616c6c6f776564c3"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::ServerConfig {
                    active_users_limit: ActiveUsersLimit::LimitedTo(8),
                    user_profile_outsider_allowed: true,
                },
            ),
        ),
        (
            // Generated from Rust implementation (Parsec v3.0.0+dev)
            // Content:
            //   event: "SERVER_CONFIG"
            //   user_profile_outsider_allowed: false
            //   status: "ok"
            &hex!(
                "84a6737461747573a26f6ba56576656e74ad5345525645525f434f4e464947b26163746976"
                "655f75736572735f6c696d6974c0bd757365725f70726f66696c655f6f757473696465725f"
                "616c6c6f776564c2"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::ServerConfig {
                    active_users_limit: ActiveUsersLimit::NoLimit,
                    user_profile_outsider_allowed: false,
                },
            ),
        ),
        (
            // Generated from Rust implementation (Parsec v3.0.0+dev)
            // Content:
            //   event: "INVITATION"
            //   invitation_status: "IDLE"
            //   status: "ok"
            //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
            &hex!(
                "84a6737461747573a26f6ba56576656e74aa494e5649544154494f4eb1696e766974617469"
                "6f6e5f737461747573a449444c45a5746f6b656ed802d864b93ded264aae9ae583fd3d40c4"
                "5a"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::Invitation {
                    invitation_status: InvitationStatus::Idle,
                    token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
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
            // Generated from Rust implementation (Parsec v3.0.0+dev)
            // Content:
            //   event: "COMMON_CERTIFICATE"
            //   status: "ok"
            //   timestamp: ext(1, 946774800.0)
            &hex!(
                "83a6737461747573a26f6ba56576656e74b2434f4d4d4f4e5f4345525449464943415445a9"
                "74696d657374616d70d70141cc375188000000"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::CommonCertificate {
                    timestamp: "2000-01-02T01:00:00Z".parse().unwrap(),
                },
            ),
        ),
        (
            // Generated from Rust implementation (Parsec v3.0.0+dev)
            // Content:
            //   event: "SEQUESTER_CERTIFICATE"
            //   status: "ok"
            //   timestamp: ext(1, 946774800.0)
            &hex!(
                "83a6737461747573a26f6ba56576656e74b55345515545535445525f434552544946494341"
                "5445a974696d657374616d70d70141cc375188000000"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::SequesterCertificate {
                    timestamp: "2000-01-02T01:00:00Z".parse().unwrap(),
                },
            ),
        ),
        (
            // Generated from Rust implementation (Parsec v3.0.0+dev)
            // Content:
            //   event: "SHAMIR_RECOVERY_CERTIFICATE"
            //   status: "ok"
            //   timestamp: ext(1, 946774800.0)
            &hex!(
                "83a6737461747573a26f6ba56576656e74bb5348414d49525f5245434f564552595f434552"
                "5449464943415445a974696d657374616d70d70141cc375188000000"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::ShamirRecoveryCertificate {
                    timestamp: "2000-01-02T01:00:00Z".parse().unwrap(),
                },
            ),
        ),
    ];

    for (raw, expected) in raw_expected {
        let data = authenticated_cmds::events_listen::Rep::load(raw).unwrap();

        assert_eq!(data, expected);

        // Also test serialization round trip
        println!("{expected:?}");
        let raw2 = data.dump().unwrap();
        println!("{raw2:02x?}");

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
