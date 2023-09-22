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
            // Generated from Python implementation (Parsec v2.6.0+dev)
            // Content:
            //   event: "pinged"
            //   ping: "foobar"
            //   status: "ok"
            &hex!("83a56576656e74a670696e676564a470696e67a6666f6f626172a6737461747573a26f6b")[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::Pinged {
                    ping: "foobar".to_owned(),
                },
            ),
        ),
        (
            // Generated from Rust implementation (Parsec v2.15.0+dev)
            // Content:
            //   event: "message_received"
            //   index: 0
            //   status: "ok"
            &hex!(
                "83a6737461747573a26f6ba56576656e74b06d6573736167655f7265636569766564a5696e"
                "64657800"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::MessageReceived { index: 0 },
            ),
        ),
        (
            // Generated from Rust implementation (Parsec v2.15.0+dev)
            // Content:
            //   event: "invite_status_changed"
            //   invitation_status: "IDLE"
            //   status: "ok"
            //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
            &hex!(
                "84a6737461747573a26f6ba56576656e74b5696e766974655f7374617475735f6368616e67"
                "6564b1696e7669746174696f6e5f737461747573a449444c45a5746f6b656ed802d864b93d"
                "ed264aae9ae583fd3d40c45a"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::InviteStatusChanged {
                    invitation_status: InvitationStatus::Idle,
                    token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
                },
            ),
        ),
        (
            // Generated from Rust implementation (Parsec v2.15.0+dev)
            // Content:
            //   encryption_revision: 0
            //   event: "realm_maintenance_finished"
            //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
            //   status: "ok"
            &hex!(
                "84a6737461747573a26f6ba56576656e74ba7265616c6d5f6d61696e74656e616e63655f66"
                "696e6973686564b3656e6372797074696f6e5f7265766973696f6e00a87265616c6d5f6964"
                "d8021d3353157d7d4e95ad2fdea7b3bd19c5"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::RealmMaintenanceFinished {
                    realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
                    encryption_revision: 0,
                },
            ),
        ),
        (
            // Generated from Rust implementation (Parsec v2.15.0+dev)
            // Content:
            //   encryption_revision: 0
            //   event: "realm_maintenance_started"
            //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
            //   status: "ok"
            &hex!(
                "84a6737461747573a26f6ba56576656e74b97265616c6d5f6d61696e74656e616e63655f73"
                "746172746564b3656e6372797074696f6e5f7265766973696f6e00a87265616c6d5f6964d8"
                "021d3353157d7d4e95ad2fdea7b3bd19c5"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::RealmMaintenanceStarted {
                    realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
                    encryption_revision: 0,
                },
            ),
        ),
        (
            // Generated from Rust implementation (Parsec v2.15.0+dev)
            // Content:
            //   checkpoint: 0
            //   event: "realm_vlobs_updated"
            //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
            //   src_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
            //   src_version: 0
            //   status: "ok"
            &hex!(
                "86a6737461747573a26f6ba56576656e74b37265616c6d5f766c6f62735f75706461746564"
                "aa636865636b706f696e7400a87265616c6d5f6964d8021d3353157d7d4e95ad2fdea7b3bd"
                "19c5a67372635f6964d8022b5f314728134a12863da1ce49c112f6ab7372635f7665727369"
                "6f6e00"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::RealmVlobsUpdated {
                    realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
                    checkpoint: 0,
                    src_id: VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
                    src_version: 0,
                },
            ),
        ),
        (
            // Generated from Rust implementation (Parsec v2.15.0+dev)
            // Content:
            //   event: "certificates_updated"
            //   index: 0
            //   status: "ok"
            &hex!(
                "83a6737461747573a26f6ba56576656e74b46365727469666963617465735f757064617465"
                "64a5696e64657800"
            )[..],
            authenticated_cmds::events_listen::Rep::Ok(
                authenticated_cmds::events_listen::APIEvent::CertificatesUpdated { index: 0 },
            ),
        ),
    ];

    for (raw, expected) in raw_expected {
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
